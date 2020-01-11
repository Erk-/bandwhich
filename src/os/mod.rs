#[cfg(target_os = "linux")]
pub(self) mod linux;

#[cfg(all(target_family = "unix", not(target_os = "linux")))]
pub(self) mod bsd;

#[cfg(all(target_family = "unix", not(target_os = "linux")))]
mod lsof_utils;

mod shared;

pub use shared::*;
