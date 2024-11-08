use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use anyhow::Result;
use libc::strlen;


/// FFI bindings for libserial
/// https://sigrok.org/api/libserialport/unstable/a00008.html

#[repr(C)]
pub struct sp_port {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SpReturn {
    SP_OK = 0,
    SP_ERR_ARG = -1,
    SP_ERR_FAIL = -2,
    SP_ERR_SUPP = -3,
    SP_ERR_MEM = -4,
}

#[repr(C)]
pub enum SpMode {
    SP_MODE_READ = 1,
    SP_MODE_WRITE = 2,
    SP_MODE_READ_WRITE = 3,
}

#[repr(C)]
pub enum SpParity {
    SP_PARITY_NONE = 0,
    SP_PARITY_ODD = 1,
    SP_PARITY_EVEN = 2,
}

#[repr(C)]
pub enum SpFlowControl {
    SP_FLOWCONTROL_NONE = 0,
    SP_FLOWCONTROL_XONXOFF = 1,
    SP_FLOWCONTROL_RTSCTS = 2,
    SP_FLOWCONTROL_DTRDSR = 3,
}

extern "C" {
    pub fn sp_list_ports(port_list: *mut *mut *mut sp_port) -> SpReturn;
    pub fn sp_get_port_name(port: *const sp_port) -> *const c_char;
    pub fn sp_free_port_list(port_list: *mut *mut sp_port);
    pub fn sp_get_port_by_name(portname: *const c_char, port_ptr: *mut *mut sp_port) -> SpReturn;
    pub fn sp_open(port: *mut sp_port, flags: SpMode) -> SpReturn;
    pub fn sp_close(port: *mut sp_port) -> SpReturn;
    pub fn sp_free_port(port: *mut sp_port);
    pub fn sp_set_baudrate(port: *mut sp_port, baudrate: c_int) -> SpReturn;
    pub fn sp_set_bits(port: *mut sp_port, bits: c_int) -> SpReturn;
    pub fn sp_set_parity(port: *mut sp_port, parity: SpParity) -> SpReturn;
    pub fn sp_set_stopbits(port: *mut sp_port, stopbits: c_int) -> SpReturn;
    pub fn sp_set_flowcontrol(port: *mut sp_port, flowcontrol: SpFlowControl) -> SpReturn;
    pub fn sp_blocking_write(port: *mut sp_port, buf: *const c_void, count: usize, timeout_ms: c_int) -> SpReturn;
    pub fn sp_blocking_read(port: *mut sp_port, buf: *mut c_void, count: usize, timeout_ms: c_int) -> SpReturn;
    pub fn sp_last_error_message() -> *mut c_char;
    pub fn sp_free_error_message(message: *mut c_char);
}