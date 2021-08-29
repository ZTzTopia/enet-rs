use libc::{c_void, size_t};

/**
 * callbacks.rs
 *
 * ENet callbacks
 */

#[repr(C)]
pub struct ENetCallbacks {
    pub malloc: Option<unsafe extern "C" fn(size: usize) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(memory: *mut c_void)>,
    pub no_memory: Option<unsafe extern "C" fn()>,
}

extern "C" {
    /* callbacks ENet internal callbacks */
    pub fn enet_malloc(arg: size_t) -> *mut c_void;
    pub fn enet_free(arg: *mut c_void);
}
