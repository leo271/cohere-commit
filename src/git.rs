use std::process::Command;

use anyhow::{Result, anyhow};

pub fn collect_diff() -> Result<String> {
    // Get staged file statuses (A=added, M=modified, D=deleted)
    let files_status_output = Command::new("git")
        .args(&["diff", "--cached", "--name-status"])
        .output()?;
    let files_status = String::from_utf8(files_status_output.stdout)?
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut diff = String::new();
    for entry in files_status {
        let mut parts = entry.split('\t');
        let status = parts.next().unwrap();
        let file = parts.next().unwrap();
        diff.push_str(&format!("=== {} ===\n", file));
        // For deletions, show a simple notice and skip patch content
        if status == "D" {
            diff.push_str(&format!("deleted file: {}\n", file));
            continue;
        }
        // Existing diff logic for added/modified files
        let file_diff_output = Command::new("git")
            .args(&["diff", "--cached", &file])
            .output()?;
        let file_diff = String::from_utf8(file_diff_output.stdout)?;
        for line in file_diff.lines().take(100) {
            diff.push_str(line);
            diff.push('\n');
        }
    }
    Ok(diff.to_string())
}

pub fn run_pre_commit_hook() -> Result<()> {
    if Command::new("git")
        .args(&["hook", "run", "pre-commit", "--ignore-missing"])
        .status()?
        .success()
    {
        println!("pre-commit hook OK");
        Ok(())
    } else {
        Err(anyhow!("pre-commit hook failed — aborting"))
    }
}

pub fn commit(msg: &str) -> Result<()> {
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&msg)
        .arg("--no-verify")
        .output()?;
    // 実行結果を表示（必要に応じて）
    if commit_output.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "git commit failed: {}{}",
            String::from_utf8_lossy(&commit_output.stdout),
            String::from_utf8_lossy(&commit_output.stderr)
        ))
    }
}
