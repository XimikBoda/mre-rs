use alloc::collections::BTreeMap;
use alloc::ffi::CString;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use embedded_io_async::{Error, ErrorKind, ErrorType};
use embedded_nal_async::TcpConnect;
use core::net::SocketAddr;
use crate::ffi::net::*; 
use crate::mre_callback;

pub use embedded_io_async::{Read, Write};


#[derive(Default)]
struct SocketState {
    connect_waker: Option<Waker>,
    read_waker: Option<Waker>,
    write_waker: Option<Waker>,
    
    is_connected: bool,
    can_read: bool,
    can_write: bool,
    is_closed: bool,
    error: Option<i32>,
}

static mut TCP_MANAGER: Option<BTreeMap<i32, SocketState>> = None;

mre_callback! {
    extern "C" fn mre_tcp_callback(handle: i32, event: i32) {
        unsafe {
            let manager_ptr = core::ptr::addr_of_mut!(TCP_MANAGER);
            
            if let Some(manager) = (*manager_ptr).as_mut() {
                if let Some(state) = manager.get_mut(&handle) {
                    match event {
                        VM_TCP_EVT_CONNECTED => {
                            state.is_connected = true;
                            state.can_write = true;
                            if let Some(w) = state.connect_waker.take() { w.wake(); }
                        }
                        VM_TCP_EVT_CAN_READ => {
                            state.can_read = true;
                            if let Some(w) = state.read_waker.take() { w.wake(); }
                        }
                        VM_TCP_EVT_CAN_WRITE => {
                            state.can_write = true;
                            if let Some(w) = state.write_waker.take() { w.wake(); }
                        }
                        VM_TCP_EVT_PIPE_BROKEN | VM_TCP_EVT_PIPE_CLOSED | VM_TCP_EVT_HOST_NOT_FOUND => {
                            state.is_closed = true;
                            state.error = Some(event);
                            if let Some(w) = state.connect_waker.take() { w.wake(); }
                            if let Some(w) = state.read_waker.take() { w.wake(); }
                            if let Some(w) = state.write_waker.take() { w.wake(); }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MreTcpError(pub i32);

impl core::fmt::Display for MreTcpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MRE TCP Error code: {}", self.0)
    }
}

impl core::error::Error for MreTcpError {}

impl Error for MreTcpError {
    fn kind(&self) -> ErrorKind {
        match self.0 {
            VM_TCP_EVT_HOST_NOT_FOUND => ErrorKind::NotFound,
            VM_TCP_EVT_PIPE_BROKEN | VM_TCP_EVT_PIPE_CLOSED => ErrorKind::ConnectionReset,
            VM_TCP_ERR_NO_ENOUGH_RES => ErrorKind::OutOfMemory,
            _ => ErrorKind::Other,
        }
    }
}

#[derive(Clone)]
pub struct TcpStream {
    handle: i32,
}

impl ErrorType for TcpStream {
    type Error = MreTcpError;
}

impl TcpStream {
    pub async fn connect(host: &str, port: u16, apn: i32) -> Result<Self, MreTcpError> {
        let c_host = CString::new(host).map_err(|_| MreTcpError(-1))?;
        
        let handle = unsafe {
            vm_tcp_connect(
                c_host.as_ptr() as *const u8, 
                port as i32, 
                apn, 
                mre_tcp_callback
            )
        };

        if handle < 0 {
            return Err(MreTcpError(handle));
        }

        unsafe {
            let manager_ptr = core::ptr::addr_of_mut!(TCP_MANAGER);
            if (*manager_ptr).is_none() {
                *manager_ptr = Some(BTreeMap::new());
            }
            (*manager_ptr).as_mut().unwrap().insert(handle, SocketState::default());
        }

        TcpConnectFuture { handle: Some(handle) }.await
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        unsafe {
            vm_tcp_close(self.handle);
            let manager_ptr = core::ptr::addr_of_mut!(TCP_MANAGER);
            if let Some(manager) = (*manager_ptr).as_mut() {
                manager.remove(&self.handle);
            }
        }
    }
}

struct TcpConnectFuture {
    handle: Option<i32>,
}

impl Future for TcpConnectFuture {
    type Output = Result<TcpStream, MreTcpError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let handle = self.handle.unwrap();
        
        unsafe {
            let manager = (*core::ptr::addr_of_mut!(TCP_MANAGER)).as_mut().unwrap();
            if let Some(state) = manager.get_mut(&handle) {
                if state.is_connected {
                    let h = self.handle.take().unwrap();
                    Poll::Ready(Ok(TcpStream { handle: h }))
                } else if state.is_closed {
                    let err = state.error.unwrap_or(-1);
                    Poll::Ready(Err(MreTcpError(err)))
                } else {
                    state.connect_waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            } else {
                Poll::Ready(Err(MreTcpError(-1))) 
            }
        }
    }
}

impl Drop for TcpConnectFuture {
    fn drop(&mut self) {
        if let Some(handle) = self.handle {
            unsafe {
                vm_tcp_close(handle);
                let manager_ptr = core::ptr::addr_of_mut!(TCP_MANAGER);
                if let Some(manager) = (*manager_ptr).as_mut() {
                    manager.remove(&handle);
                }
            }
        }
    }
}

impl Read for TcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        TcpReadFuture {
            handle: self.handle,
            buf,
        }.await
    }
}

impl Write for TcpStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        TcpWriteFuture {
            handle: self.handle,
            buf,
        }.await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct TcpReadFuture<'a> {
    handle: i32,
    buf: &'a mut [u8],
}

impl<'a> Future for TcpReadFuture<'a> {
    type Output = Result<usize, MreTcpError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            let manager = (*core::ptr::addr_of_mut!(TCP_MANAGER)).as_mut().unwrap();
            let state = manager.get_mut(&self.handle).unwrap();

            if state.is_closed {
                return Poll::Ready(Err(MreTcpError(state.error.unwrap_or(-1))));
            }

            if state.can_read {
                let res = vm_tcp_read(
                    self.handle, 
                    self.buf.as_mut_ptr() as *mut core::ffi::c_void, 
                    self.buf.len() as i32
                );

                if res > 0 {
                    return Poll::Ready(Ok(res as usize));
                } else if res == VM_TCP_READ_EOF {
                    state.is_closed = true;
                    return Poll::Ready(Ok(0));
                } else {
                    state.can_read = false; 
                }
            }

            state.read_waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

struct TcpWriteFuture<'a> {
    handle: i32,
    buf: &'a [u8],
}

impl<'a> Future for TcpWriteFuture<'a> {
    type Output = Result<usize, MreTcpError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            let manager = (*core::ptr::addr_of_mut!(TCP_MANAGER)).as_mut().unwrap();
            let state = manager.get_mut(&self.handle).unwrap();

            if state.is_closed {
                return Poll::Ready(Err(MreTcpError(state.error.unwrap_or(-1))));
            }

            if state.can_write {
                let res = vm_tcp_write(
                    self.handle, 
                    self.buf.as_ptr() as *const core::ffi::c_void, 
                    self.buf.len() as i32
                );

                if res >= 0 {
                    return Poll::Ready(Ok(res as usize));
                } else {
                    state.can_write = false; 
                }
            }

            state.write_waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub struct MreTcpStack;

impl TcpConnect for MreTcpStack {
    type Error = MreTcpError;
    type Connection<'a> = TcpStream where Self: 'a;

    async fn connect<'a>(&'a self, remote: SocketAddr) -> Result<Self::Connection<'a>, Self::Error> {
        let host_str = alloc::format!("{}", remote.ip());
        
        TcpStream::connect(&host_str, remote.port(), 1).await
    }
}