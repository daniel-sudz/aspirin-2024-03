
/// Wrapper rust struct for the libserialport native structs
/// https://sigrok.org/api/libserialport/unstable/a00008.html

use super::ffi::sp_port;

pub struct Port {
    pub name: String,
    pub handle: *mut sp_port,
}