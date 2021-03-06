#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! Rust bindings for [`ENet`] library, the reliable UDP networking library.
//!
//! # Installation
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! enet-rs = "1.3.17"
//! ```
//!
//! # Examples
//!
//! `server.rs`:
//! ```rust
//! use enet_rs::enet::{
//!     ENET_HOST_ANY,
//!     ENetAddress,
//!     enet_host_create,
//! };
//!
//! fn main() {
//!     let address = ENetAddress {
//!         host: ENET_HOST_ANY,
//!         port
//!     };
//!
//!     unsafe {
//!         let server = enet_host_create(&address,
//!             max_peers,
//!             2,
//!             0,
//!             0);
//!         if server.is_null() {
//!             println!("An error occurred while trying to create an ENet server host.");
//!         }
//!     }
//!
//!     loop {
//!         /* does nothing */
//!     }
//! }
//!```
//!
//! `client.rs`:
//! ```rust
//! use std::ptr::null;
//! use std::mem::MaybeUninit;
//! use std::ffi::CString;
//! use enet_rs::enet::{
//!     ENetEventType,
//!     enet_host_create,
//!     enet_address_set_host,
//!     enet_host_connect,
//!     enet_host_service,
//!     enet_peer_reset
//! };
//!
//! fn main() {
//!     unsafe {
//!         let client = enet_host_create(null(),
//!             1,
//!             2,
//!             0,
//!             0);
//!         if client.is_null() {
//!             panic!("An error occurred while trying to create an ENet client host.")
//!         }
//!
//!         let mut address_ = MaybeUninit::uninit();
//!         enet_address_set_host(address_.as_mut_ptr(), (&CString::new("127.0.0.1").unwrap()).as_ptr());
//!         let mut address = address_.assume_init();
//!         address.port = 8080;
//!
//!         let peer = enet_host_connect(client, &address, 2, 0);
//!         if peer.is_null() {
//!             println!("No available peers for initiating an ENet connection.\n");
//!         }
//!
//!         let mut event_ = MaybeUninit::uninit();
//!         loop {
//!             if enet_host_service(client, event_.as_mut_ptr(), 5000) > 0 {
//!                 let event = event_.assume_init();
//!                 match event.type_ {
//!                     ENetEventType::ENET_EVENT_TYPE_CONNECT => {
//!                         println!("connected to server.")
//!                     }
//!                     _ => enet_peer_reset(peer)
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Full Examples
//! Full examples, detailing and explaining usage of the basic functionality of the library, can be found in the [`examples`] directory.
//!
//! [`ENet`]: http://enet.bespin.org/
//! [`examples`]: https://github.com/ZTzTopia/enet-rs

pub mod callbacks;
pub mod enet;
pub mod types;
pub mod list;
pub mod protocol;
pub mod time;
pub mod header;
pub mod utility;