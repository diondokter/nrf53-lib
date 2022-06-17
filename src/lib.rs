#![no_std]

#[cfg(feature = "app")]
pub use nrf5340_app_pac as pac;
#[cfg(feature = "net")]
pub use nrf5340_net_pac as pac;

pub mod mutex;
pub mod peripheral_id;
pub mod approtect;
#[cfg(feature = "app")]
pub mod spu;
#[cfg(feature = "app")]
pub mod dcnf;
