use libc::c_uint;

use crate::types::SOCKET;

/**
 * utility.rs
 *
 * ENet utility header
 */

#[macro_export]
macro_rules! ENET_MAX {
    ($x:expr,$y:expr) => {
        std::cmp::max($x, $y)
    };
}

#[macro_export]
macro_rules! ENET_MIN {
    ($x:expr,$y:expr) => {
        std::cmp::min($x, $y)
    };
}

#[macro_export]
macro_rules! ENET_DIFFERENCE {
    ($x:expr,$y:expr) => {
        if $x < $y { $y - $x } else { $x - $y }
    };
}

#[repr(C)]
pub struct fd_set {
    pub fd_count: c_uint,
    pub fd_array: [SOCKET; 64usize],
}

