use regex::Regex;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Process {
    child: Arc<Mutex<Option<Child>>>,
}

impl Process {
    pub fn create<S, I>(exe_path: S, args: I) -> Process
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let child = Command::new(exe_path.as_ref())
            .args(args.into_iter().map(|s| s.as_ref().to_string()))
            .spawn()
            .expect("Failed to start process");

        let child = Arc::new(Mutex::new(Some(child)));

        return Self { child };
    }

    pub fn wait_for_pattern(&self, pattern: &str) -> Option<String> {
        let regex = Regex::new(pattern).expect("Invalid regex pattern");
        let mut child = self.child.lock().unwrap();
        let child = child.as_mut().expect("Process is not running");
        let stdout = child.stdout.as_mut().expect("Failed to access stdout");
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
                if let Some(caps) = regex.captures(&line) {
                    if let Some(matched) = caps.get(1) {
                        return Some(matched.as_str().to_string());
                    }
                }
            }
        }
        None
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.lock().unwrap().take() {
            let _ = child.kill(); // Ensure cleanup
        }
    }
}
