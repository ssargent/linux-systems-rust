use std::os::fd::RawFd;

pub fn is_valid_fd(fd: RawFd) -> bool {
    fd >= 0
}

#[cfg(target_os = "linux")]
pub fn linux_fd_note() -> &'static str {
    "linux-specific fd helpers will be implemented in exercises"
}
