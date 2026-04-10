pub fn known_signals() -> &'static [i32] {
    &[libc::SIGINT, libc::SIGTERM]
}

#[cfg(target_os = "linux")]
pub fn linux_signal_note() -> &'static str {
    "linux-specific signal helpers will be implemented in exercises"
}
