use atopx::run_command;
use std::io::Error;
use std::process::Command;

pub struct Flow {
    exec_dir: String,
}

impl Flow {
    const GIT_PATH: &'static str = "/opt/homebrew/bin/git";

    pub fn new(path: &str) -> Self {
        Self {
            exec_dir: String::from(path),
        }
    }

    // qa flow
    pub fn qa(&self) {
        if let Some(current_branch) = self.get_current_branch().ok() {
            let branch = current_branch.trim();
            self.checkout_branch("qa", false);
            self.pull();
            self.merge(&branch);
            self.push();
            self.checkout_branch(&branch, false);
        }
    }

    // git branch --show-current
    fn get_current_branch(&self) -> Result<String, Error> {
        let mut command = Command::new(Flow::GIT_PATH);
        command.arg("branch");
        command.arg("--show-current");
        return run_command(&mut command, &self.exec_dir);
    }

    /// git checkout
    fn checkout_branch(&self, branch: &str, force: bool) {
        let mut command = Command::new(Flow::GIT_PATH);
        command.arg("checkout");
        if force {
            command.arg("-b");
        }
        command.arg(branch);
        _ = run_command(&mut command, &self.exec_dir);
    }

    // git merge
    fn merge(&self, branch: &str) {
        let mut command = Command::new(Flow::GIT_PATH);
        command.arg("merge");
        command.arg(branch);
        _ = run_command(&mut command, &self.exec_dir);
    }

    // git pull
    fn pull(&self) {
        let mut command = Command::new(Flow::GIT_PATH);
        command.arg("pull");
        _ = run_command(&mut command, &self.exec_dir);
    }

    // git push
    fn push(&self) {
        let mut command = Command::new(Flow::GIT_PATH);
        command.arg("push");
        _ = run_command(&mut command, &self.exec_dir);
    }
}
