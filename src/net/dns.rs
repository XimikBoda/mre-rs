use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use alloc::ffi::CString;

use crate::ffi::net::*;
use super::Ipv4Addr;

struct QueuedDnsRequest {
    apn: i32,
    host: String,
    waker: Waker,
}

static mut DNS_IS_BUSY: bool = false;
static mut DNS_WAKER: Option<Waker> = None;
static mut DNS_RESULT: vm_soc_dns_result = vm_soc_dns_result {
    address: [0; 5],
    num: 0,
    error_cause: 0,
};
static mut DNS_QUEUE: Option<VecDeque<QueuedDnsRequest>> = None;

extern "C" fn mre_dns_callback(result_ptr: *mut vm_soc_dns_result) -> i32 {
    unsafe {
        if !result_ptr.is_null() {
            DNS_RESULT = *result_ptr;
        }

        let waker_ptr = core::ptr::addr_of_mut!(DNS_WAKER);

        let waker_opt = core::ptr::replace(waker_ptr, None);

        if let Some(waker) = waker_opt {
            waker.wake();
        }
    }
    0
}

fn parse_dns_result(raw: vm_soc_dns_result) -> Result<Vec<Ipv4Addr>, i32> {
    if raw.num > 0 {
        let mut ips = Vec::with_capacity(raw.num as usize);
        for i in 0..raw.num as usize {
            ips.push(Ipv4Addr::from_vmuint(raw.address[i]));
        }
        Ok(ips)
    } else {
        Err(raw.error_cause)
    }
}

unsafe fn start_mre_dns_request(apn: i32, host: &str, waker: Waker) {
    unsafe {
        DNS_IS_BUSY = true;
        DNS_WAKER = Some(waker);
        let c_host = CString::new(host).unwrap();
        let res =  vm_soc_get_host_by_name(apn, c_host.as_ptr() as *const u8, &raw mut DNS_RESULT, mre_dns_callback);
        
        let waker_ptr = core::ptr::addr_of_mut!(DNS_WAKER);

        let waker_opt = core::ptr::replace(waker_ptr, None);

        if res == 0 || res != -2 {
            if let Some(w) = waker_opt { w.wake(); }
        }
    }
}

pub struct DnsResolver {
    apn: i32,
    host: String,
    state: ResolverState,
}

enum ResolverState {
    New,
    InQueue,
    WaitingForCallback,
}

pub fn resolve_host(apn: i32, host: &str) -> DnsResolver {
    DnsResolver {
        apn,
        host: String::from(host),
        state: ResolverState::New,
    }
}

impl Future for DnsResolver {
    type Output = Result<Vec<Ipv4Addr>, i32>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            let queue_ptr = core::ptr::addr_of_mut!(DNS_QUEUE);
            if (*queue_ptr).is_none() {
                *queue_ptr = Some(VecDeque::new());
            }

            match self.state {
                ResolverState::New => {
                    if DNS_IS_BUSY {
                        let queue = (*queue_ptr).as_mut().unwrap();
                        queue.push_back(QueuedDnsRequest {
                            apn: self.apn,
                            host: self.host.clone(),
                            waker: cx.waker().clone(),
                        });
                        
                        self.state = ResolverState::InQueue;
                        return Poll::Pending;
                    } else {
                        start_mre_dns_request(self.apn, &self.host, cx.waker().clone());
                        self.state = ResolverState::WaitingForCallback;
                        return Poll::Pending;
                    }
                },
                
                ResolverState::InQueue => {
                    self.state = ResolverState::WaitingForCallback;
                    return Poll::Pending;
                },
                
                ResolverState::WaitingForCallback => {
                    let result = parse_dns_result(DNS_RESULT);
                    
                    DNS_IS_BUSY = false;

                    let queue = (*queue_ptr).as_mut().unwrap();
                    if let Some(next_req) = queue.pop_front() {
                        start_mre_dns_request(next_req.apn, &next_req.host, next_req.waker.clone());
                        next_req.waker.wake(); 
                    }
                    // ==========================================

                    return Poll::Ready(result);
                }
            }
        }
    }
}