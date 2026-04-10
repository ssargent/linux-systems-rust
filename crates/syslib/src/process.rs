use std::process::Command;

pub fn command_status(program: &str) -> std::io::Result<std::process::ExitStatus> {
    Command::new(program).status()
}

#[cfg(target_os = "linux")]
pub fn linux_process_note() -> &'static str {
    "linux-specific process helpers will be implemented in exercises"
}
