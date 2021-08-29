# enet-rs

[![Documentation](https://docs.rs/enet-rs/badge.svg)](https://docs.rs/enet)
[![Crates.io](https://img.shields.io/crates/v/enet-rs.svg)](https://crates.io/crates/enet)
[![License](https://img.shields.io/crates/l/enet-rs.svg)](https://github.com/futile/enet-rs)

Rust bindings for [ENet](http://enet.bespin.org) library, the reliable UDP networking library.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
enet-rs = "1.3.17"
```

## Examples

`server.rs`:
```rust
fn main() {
    let address = ENetAddress {
        host: ENET_HOST_ANY,
        port: 2555
    };

    unsafe {
        let server = enet_host_create(&address,
            max_peers,
            2,
            0,
            0);
        if server.is_null() {
            println!("An error occurred while trying to create an ENet server host.");
        }
    }
    
    loop {
        /* does nothing */
    }
}
```

`client.rs`:
```rust
fn main() {
    unsafe {
        let client = enet_host_create(null(),
            1,
            2,
            0,
            0);
        if client.is_null() {
            panic!("An error occurred while trying to create an ENet client host.")
        }

        let mut address_ = MaybeUninit::uninit();
        enet_address_set_host(address_.as_mut_ptr(), (&CString::new("127.0.0.1").unwrap()).as_ptr());
        let mut address = address_.assume_init();
        address.port = 2555;

        let peer = enet_host_connect(client, &address, 2, 0);
        if peer.is_null() {
            println!("No available peers for initiating an ENet connection.\n");
        }

        let mut event_ = MaybeUninit::uninit();
        loop {
            if enet_host_service(client, event_.as_mut_ptr(), 5000) > 0 {
                let event = event_.assume_init();
                match event.type_ {
                    ENetEventType::ENET_EVENT_TYPE_CONNECT => {
                        println!("connected to server.")
                    }
                    _ => enet_peer_reset(peer)
                }
            }
        }
    }
}
```

### Full Examples

Full examples, detailing and explaining usage of the basic functionality of the library, can be found in the `examples` directory.

## Documentation

Documentation is available by running `cargo doc` or visit [docs.rs](https://docs.rs/enet-rs/).

## License

enet-rs is licensed under the [ISC](https://github.com/ZTzTopia/enet-rs/blob/main/LICENSE.md) license. See the file `LICENSE.md` for more information.