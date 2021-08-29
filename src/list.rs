use libc::{c_void, size_t};

/**
 * list.rs
 *
 * ENet list management
*/

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ENetListNode
{
    pub next: *mut ENetListNode,
    pub previous: *mut ENetListNode,
}

pub type ENetListIterator = *mut ENetListNode;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ENetList
{
    pub sentinel: ENetListNode,
}

extern "C" {
    pub fn enet_list_clear(list: *mut ENetList);

    pub fn enet_list_insert(position: ENetListIterator, data: *mut c_void) -> ENetListIterator;
    pub fn enet_list_remove(position: ENetListIterator) -> *mut c_void;
    pub fn enet_list_move(position: ENetListIterator, dataFirst: *mut c_void, dataLast: *mut c_void) -> ENetListIterator;

    pub fn enet_list_size(list: *mut ENetList) -> size_t;
}

#[macro_export]
macro_rules! enet_list_begin {
    ($list:expr) => {
        // NOT TESTED!
        ((*$list).sentinel.next)
    };
}

#[macro_export]
macro_rules! enet_list_end {
    ($list:expr) => {
        // NOT TESTED!
        (&(*$list).sentinel)
    };
}

#[macro_export]
macro_rules! enet_list_empty {
    ($list:expr) => {
        // NOT TESTED!
        ($crate::enet_list_begin($list) == $crate::enet_list_end(list))
    };
}

#[macro_export]
macro_rules! enet_list_next {
    ($iterator:expr) => {
        // NOT TESTED!
        ((*$iterator).next)
    };
}

#[macro_export]
macro_rules! enet_list_previous {
    ($iterator:expr) => {
        // NOT TESTED!
        ((*$iterator).previous)
    };
}

#[macro_export]
macro_rules! enet_list_front {
    ($list:expr) => {
        // NOT TESTED!
        ((*mut c_void)(*$list).sentinel.next)
    };
}

#[macro_export]
macro_rules! enet_list_back {
    ($list:expr) => {
        // NOT TESTED!
        ((*mut c_void)(*$list).sentinel.previous)
    };
}
