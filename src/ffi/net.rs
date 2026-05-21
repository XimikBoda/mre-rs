#![allow(non_snake_case)]
use crate::mre_api;

pub const VM_E_SOC_SUCCESS: i32 = 0;
pub const VM_E_SOC_ERROR: i32 = -1;
pub const VM_E_SOC_WOULDBLOCK: i32 = -2;
pub const VM_E_SOC_LIMIT_RESOURCE: i32 = -3;
pub const VM_E_SOC_INVALID_SOCKET: i32 = -4;
pub const VM_E_SOC_INVALID_ACCOUNT: i32 = -5;
pub const VM_E_SOC_NAMETOOLONG: i32 = -6;
pub const VM_E_SOC_ALREADY: i32 = -7;
pub const VM_E_SOC_OPNOTSUPP: i32 = -8;
pub const VM_E_SOC_CONNABORTED: i32 = -9;
pub const VM_E_SOC_INVAL: i32 = -10;
pub const VM_E_SOC_PIPE: i32 = -11;
pub const VM_E_SOC_NOTCONN: i32 = -12;
pub const VM_E_SOC_MSGSIZE: i32 = -13;
pub const VM_E_SOC_BEARER_FAIL: i32 = -14;
pub const VM_E_SOC_CONNRESET: i32 = -15;
pub const VM_E_SOC_DHCP_ERROR: i32 = -16;
pub const VM_E_SOC_IP_CHANGED: i32 = -17;
pub const VM_E_SOC_ADDRINUSE: i32 = -18;
pub const VM_E_SOC_CANCEL_ACT_BEARER: i32 = -19;

pub const VM_MAX_SOCK_ADDR_LEN: usize = 28;
pub const VM_SOC_DNS_MAX_ADDR: usize = 5;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_sockaddr_struct {
    pub addr_len: i16,
    pub port: u16,
    pub addr: [u8; VM_MAX_SOCK_ADDR_LEN],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_soc_dns_result {
    pub address: [u32; VM_SOC_DNS_MAX_ADDR],
    pub num: i32,
    pub error_cause: i32,
}

pub type VmSocDnsCallback = extern "C" fn(result: *mut vm_soc_dns_result) -> i32;

mre_api!(vm_soc_get_host_by_name(apn: i32, host: *const u8, result: *mut vm_soc_dns_result, callback: VmSocDnsCallback) -> i32 = -1);