use std::net::SocketAddr;

pub fn parse_addr(input: &str) -> std::io::Result<SocketAddr> {
    input.parse().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid socket address")
    })
}

#[cfg(target_os = "linux")]
pub fn linux_net_note() -> &'static str {
    "linux-specific networking helpers will be implemented in exercises"
}
