/**
 * time.rs
 *
 * ENet time constants and macros
*/

#[macro_export]
macro_rules! ENET_TIME_OVERFLOW {
    () => { 86400000 }
}

#[macro_export]
macro_rules! ENET_TIME_LESS {
    ($a:expr,$b:expr) => {
        (($a) - ($b) >= $crate::ENET_TIME_OVERFLOW!())
    };
}

#[macro_export]
macro_rules! ENET_TIME_GREATER {
    ($a:expr,$b:expr) => {
        (($b) - ($a) >= $crate::ENET_TIME_OVERFLOW!())
    };
}

#[macro_export]
macro_rules! ENET_TIME_LESS_EQUAL {
    ($a:expr,$b:expr) => {
        (!$crate::ENET_TIME_GREATER!($a, $b))
    };
}

#[macro_export]
macro_rules! ENET_TIME_GREATER_EQUAL {
    ($a:expr,$b:expr) => {
        (!$crate::ENET_TIME_LESS!($a, $b))
    };
}

#[macro_export]
macro_rules! ENET_TIME_DIFFERENCE {
    ($a:expr,$b:expr) => {
        if $a - $b >= $crate::ENET_TIME_OVERFLOW!() { $b - $a } else { $a - $b }
    };
}

