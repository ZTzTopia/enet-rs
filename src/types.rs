use libc::{c_char, c_uchar, c_ushort, c_uint, c_ulonglong};

/**
 * types.rs
 *
 * type definitions for ENet
*/

pub type enet_uint8 = c_uchar;
pub type enet_uint16 = c_ushort;
pub type enet_uint32 = c_uint;

pub type SOCKET = c_ulonglong;
pub const INVALID_SOCKET: c_char = -1;