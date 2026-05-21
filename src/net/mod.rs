pub mod dns;
pub mod tcp;

use crate::ffi::net::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Addr(pub [u8; 4]);

impl Ipv4Addr {
    pub fn from_vmuint(addr: u32) -> Self {
        Self(addr.to_le_bytes())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SocketAddr {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl SocketAddr {
    pub fn into_mre_sockaddr(self) -> vm_sockaddr_struct {
        let mut addr_buf = [0u8; VM_MAX_SOCK_ADDR_LEN];
        addr_buf[0..4].copy_from_slice(&self.ip.0);
        
        vm_sockaddr_struct {
            addr_len: 4,
            port: self.port,
            addr: addr_buf,
        }
    }
}