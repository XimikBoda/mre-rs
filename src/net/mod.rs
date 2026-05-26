pub mod dns;
pub mod tcp;
#[cfg(feature = "tls")]
pub mod tls;
#[cfg(feature = "http")]
pub mod http;

use crate::ffi::net::*;

use core::net::SocketAddrV4;

pub trait IntoMreSockAddr {
    fn into_mre_sockaddr(&self) -> vm_sockaddr_struct;
}

impl IntoMreSockAddr for SocketAddrV4 {
    fn into_mre_sockaddr(&self) -> vm_sockaddr_struct {
        let mut addr_buf = [0u8; VM_MAX_SOCK_ADDR_LEN];
        
        addr_buf[0..4].copy_from_slice(&self.ip().octets());
        
        vm_sockaddr_struct {
            addr_len: 4,
            port: self.port(),
            addr: addr_buf,
        }
    }
}