pub mod dotnet;
pub mod entry_info;
#[cfg(unix)]
pub mod linux;
#[cfg(windows)]
pub mod windows;
#[macro_use]
#[cfg(windows)]
extern crate lazy_static;
