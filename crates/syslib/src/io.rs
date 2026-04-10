use std::io::{self, Read};

pub fn read_to_string<R: Read>(mut reader: R) -> io::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

#[cfg(target_os = "linux")]
pub fn linux_io_note() -> &'static str {
    "linux-specific io helpers will be implemented in exercises"
}
