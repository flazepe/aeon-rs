#[cfg(target_os = "linux")]
include!("linux.rs");

#[cfg(target_os = "windows")]
include!("windows.rs");
