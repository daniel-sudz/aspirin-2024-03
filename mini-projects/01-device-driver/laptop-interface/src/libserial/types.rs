/// Wrapper rust struct for the libserialport native structs
/// https://sigrok.org/api/libserialport/unstable/a00008.html
use super::ffi::{sp_free_port, sp_port};

pub struct Port {
    pub name: String,
    pub handle: *mut sp_port,
}

/// Frees the port when the Port struct is dropped
impl Drop for Port {
    fn drop(&mut self) {
        unsafe {
            sp_free_port(self.handle);
        }
    }
}
