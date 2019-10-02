#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub(crate) use self::macos::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use self::linux::*;

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
compile_error!("wifiscan doesn't compile for this platform yet");
