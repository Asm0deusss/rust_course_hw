use crate::{launch_git, repository::repo::Repository};
use anyhow::{bail, Result};
use std::{
    path::Path,
    process,
};

pub fn submit_problem(
    problem_path: &Path,
    message: &str,
) -> Result<()> {
    let repo = Repository::from_path(problem_path)?;
    let problem = repo.problem_from_path(problem_path)?;
    let repo_path = repo.get_path();
    if !launch_git!(repo_path, "add", problem.relative_path()) {
        bail!("git add failed");
    }
    if !launch_git!(repo_path, "commit", "-m", message) {
        bail!("git commit failed: either no changes since the last commit or git failed")
    }
    if !launch_git!(
        repo_path,
        "push",
        "--force",
        "student",
        &format!("HEAD:{}", problem.branch_name())
    ) {
        bail!("git push failed")
    }
    Ok(())
}
