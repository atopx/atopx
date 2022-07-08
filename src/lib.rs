use std::process::{Command, Stdio};
use std::io::Error;

pub fn run_command(command: &mut Command, exec_dir: &str) -> Result<String, Error> {
    command.current_dir(&exec_dir);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    let output = command.output().unwrap();
    let mut result: String = String::new();
    if let Some(status) = output.status.code() {
        if status == 0 {
            result = String::from_utf8(output.stdout).unwrap();
            println!("{}", result);
        } else {
            println!("{}", String::from_utf8(output.stderr).unwrap());
        }
    }
    return Ok(result);
}