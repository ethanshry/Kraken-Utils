//! Wrappers for git CLI functions

/// Clones a git repository at a specific branch to the dst_dir
pub fn clone_remote_branch(url: &str, branch: &str, dst_dir: &str) -> Result<(), String> {
    if let Ok(mut branch) = std::process::Command::new("git")
    .arg("clone")
    .arg("-b")
    .arg(branch)
    .arg(url)
    .arg(dst_dir)
    .spawn() {
        match branch.wait() {
            Ok(_) => return Ok(()),
            Err(e) => return Err(format!("{:?}", e))
        }
    }
    Err(format!("Could not execute clone command for {}@{} to {}", url, branch, dst_dir))
}
