pub struct GitProcessAdapter;

impl crate::ports::GitPort for GitProcessAdapter {
    type Error = String;

    fn current_branch(&self) -> Result<String, Self::Error> {
        let output = std::process::Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .map_err(|e| format!("failed to execute git: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn changed_files(&self) -> Result<Vec<String>, Self::Error> {
        let output = std::process::Command::new("git")
            .args(["status", "--short"])
            .output()
            .map_err(|e| format!("failed to execute git: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files = stdout
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return None;
                }
                trimmed.split_whitespace().last().map(ToString::to_string)
            })
            .collect();

        Ok(files)
    }

    fn commit(&self, message: &str) -> Result<(), Self::Error> {
        let status = std::process::Command::new("git")
            .args(["commit", "-m", message])
            .status()
            .map_err(|e| format!("failed to execute git: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err("git commit failed".to_string())
        }
    }
}
