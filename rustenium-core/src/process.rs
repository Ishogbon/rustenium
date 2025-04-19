use std::process::Stdio;

use regex::Regex;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::time::{timeout, Duration};

pub struct Process {
    child: Option<Child>,
}

impl Process {
    pub fn create<S, I>(exe_path: S, args: I) -> Process
    where
        S: AsRef<str>,
        I: IntoIterator<Item = String>,
    {
        let child = Command::new(exe_path.as_ref())
            .args(args.into_iter().map(|s| s))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        let child = Some(child);

        return Self { child };
    }

    #[deprecated]
    pub async fn wait_for_pattern(&mut self, pattern: &str, timeout_secs: Option<u64>) -> String {
        let timeout_secs = timeout_secs.unwrap_or(20);
        let regex = Regex::new(pattern).expect("Invalid regex pattern");
        let mut child = self.child.as_mut().unwrap();

        let stdout = child.stdout.as_mut().expect("Failed to access stdout");
        let stderr = child.stderr.as_mut().expect("Failed to access stderr");

        let mut stdout_lines = BufReader::new(stdout).lines();
        let mut stderr_lines = BufReader::new(stderr).lines();

        let mut check_line = |label: &str, line: Result<Option<String>, _>| -> Option<String> {
            if let Ok(Some(line)) = line {
                if let Some(captures) = regex.captures(&line) {
                    if let Some(url) = captures.get(1) {
                        return Some(url.as_str().into());
                    }
                }
            }
            None
        };

        let timeout_duration = Duration::from_secs(timeout_secs);

        let timeout_result = timeout(timeout_duration, async {
            loop {
                tokio::select! {
                    stdout_line = stdout_lines.next_line() => {
                        if let Some(line) = check_line("stdout", stdout_line) {
                            return Some(line);
                        }
                    },
                    stderr_line = stderr_lines.next_line() => {
                        if let Some(line) = check_line("stderr", stderr_line) {
                            return  Some(line);
                        }
                    }
                }
            }
        })
        .await;

        match timeout_result {
            Ok(Some(matched)) => matched,
            Ok(None) => panic!("Found a pattern but None"),
            Err(_) => panic!("Timeout reached without finding pattern"),
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill(); // Ensure cleanup
        }
    }
}
