use libc::{c_char, c_void, size_t};

use crate::{
    types::{SOCKET, INVALID_SOCKET},
    utility::fd_set
};

/**
 * header.rs
 *
 * ENet Unix and Win32 header
*/
pub type ENetSocket = SOCKET;

pub const ENET_SOCKET_NULL: c_char = INVALID_SOCKET;

/** macro that converts host to net byte-order of a 16-bit value */
#[macro_export]
macro_rules! ENET_HOST_TO_NET_16 {
    ($value:expr) => {
        $value.to_be()
    };
}

/** macro that converts host to net byte-order of a 32-bit value */
#[macro_export]
macro_rules! ENET_HOST_TO_NET_32 {
    ($value:expr) => {
        $value.to_be()
    };
}

/** macro that converts net to host byte-order of a 16-bit value */
#[macro_export]
macro_rules! ENET_NET_TO_HOST_16 {
    ($value:expr) => {
        u16::from_be($value)
    };
}

/** macro that converts net to host byte-order of a 32-bit value */
#[macro_export]
macro_rules! ENET_NET_TO_HOST_32 {
    ($value:expr) => {
        u32::from_be($value)
    };
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ENetBuffer
{
    pub dataLength: size_t,
    pub data: *mut c_void,
}

pub type ENetSocketSet = fd_set;

#[macro_export]
macro_rules! ENET_SOCKETSET_EMPTY {
    ($sockset:expr) => {
        // NOT TESTED!
        libc::FD_ZERO(& ($sockset))
    };
}

#[macro_export]
macro_rules! ENET_SOCKETSET_ADD {
    ($sockset:expr,$socket:expr) => {
        // NOT TESTED!
        libc::FD_SET($socket, & ($sockset))
    };
}

#[macro_export]
macro_rules! ENET_SOCKETSET_REMOVE {
    ($sockset:expr,$socket:expr) => {
        // NOT TESTED!
        libc::FD_CLR($socket, & ($sockset))
    };
}

#[macro_export]
macro_rules! ENET_SOCKETSET_CHECK {
    ($sockset:expr,$socket:expr) => {
        // NOT TESTED!
        libc::FD_ISSET($socket, & ($sockset))
    };
}


