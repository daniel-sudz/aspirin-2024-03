use std::os::raw::{c_char, c_int, c_void};

/// FFI bindings for libserial
/// https://sigrok.org/api/libserialport/unstable/a00008.html

#[repr(C)]
pub struct SpPort {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum SpReturn {
    SpOK = 0,
    SpErrARG = -1,
    SpErrFAIL = -2,
    SpErrSUPP = -3,
    SpErrMEM = -4,
}

#[repr(C)]
#[allow(dead_code)]
pub enum SpMODE {
    SpModeREAD = 1,
    SpModeWRITE = 2,
    SpModeReadWrite = 3,
}

#[repr(C)]
#[allow(dead_code)]
pub enum SpPARITY {
    SpParityNONE = 0,
    SpParityODD = 1,
    SpParityEVEN = 2,
}

#[repr(C)]
#[allow(dead_code)]
pub enum SpFLOWCONTROL {
    SpFlowControlNONE = 0,
    SpFlowControlXONXOFF = 1,
    SpFlowControlRTSCTS = 2,
    SpFlowControlDTRDSR = 3,
}

#[allow(dead_code)]
extern "C" {
    pub fn sp_list_ports(port_list: *mut *mut *mut SpPort) -> SpReturn;
    pub fn sp_get_port_name(port: *const SpPort) -> *const c_char;
    pub fn sp_free_port_list(port_list: *mut *mut SpPort);
    pub fn sp_get_port_by_name(portname: *const c_char, port_ptr: *mut *mut SpPort) -> SpReturn;
    pub fn sp_open(port: *mut SpPort, flags: SpMODE) -> SpReturn;
    pub fn sp_free_port(port: *mut SpPort);
    pub fn sp_set_baudrate(port: *mut SpPort, baudrate: c_int) -> SpReturn;
    pub fn sp_set_bits(port: *mut SpPort, bits: c_int) -> SpReturn;
    pub fn sp_set_parity(port: *mut SpPort, parity: SpPARITY) -> SpReturn;
    pub fn sp_set_stopbits(port: *mut SpPort, stopbits: c_int) -> SpReturn;
    pub fn sp_set_flowcontrol(port: *mut SpPort, flowcontrol: SpFLOWCONTROL) -> SpReturn;
    pub fn sp_blocking_write(
        port: *mut SpPort,
        buf: *const c_void,
        count: usize,
        timeout_ms: c_int,
    ) -> SpReturn;
    pub fn sp_blocking_read(
        port: *mut SpPort,
        buf: *mut c_void,
        count: usize,
        timeout_ms: c_int,
    ) -> SpReturn;
    pub fn sp_blocking_read_next(
        port: *mut SpPort,
        buf: *mut c_void,
        count: usize,
        timeout_ms: c_int,
    ) -> SpReturn;
    pub fn sp_last_error_message() -> *mut c_char;
    pub fn sp_free_error_message(message: *mut c_char);
    pub fn sp_drain(port: *mut SpPort) -> SpReturn;
}
