use std::process::{Command, Stdio};

use atopx::run_command;

pub struct Osp {
    exec_dir: String,
}


impl Osp {
    const PS: &'static str = "/bin/ps";
    const LSOF: &'static str = "/usr/lsof";
    const GREP: &'static str = "/usr/bin/grep";

    pub fn new(path: &str) -> Self {
        Self {
            exec_dir: String::from(path),
        }
    }

    pub fn ps_grep(&self, value: &str) {
        let ps_cmd = Command::new(Osp::PS)
            .arg("-ef")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut grep_cmd = Command::new(Osp::GREP);
        grep_cmd.arg("--color=auto");
        grep_cmd.arg(value);
        grep_cmd.stdin(ps_cmd.stdout.unwrap());
        _ = run_command(&mut grep_cmd, &self.exec_dir);
    }

    pub fn lsof_tcp(&self, value: &u16) {
        let mut command = Command::new(Osp::LSOF);
        command.arg("-i");
        command.arg(format!("tcp:{}", value));
        _ = run_command(&mut command, &self.exec_dir);
    }

}
