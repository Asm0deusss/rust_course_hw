#![forbid(unsafe_code)]

use crate::{ErrorType, vcs_file_manager::get_branch_hash};

use super::jump_to_commit::jump_to_commit;

pub fn jump_to_branch (branch_name: &str) -> Result<String, ErrorType>{

    let check_cur_commit_hash = get_branch_hash(&branch_name);

    if check_cur_commit_hash.is_err() {
        return Err(ErrorType::NoSuchBranch);
    }

    let cur_commit_hash = check_cur_commit_hash.unwrap();

    jump_to_commit(&cur_commit_hash)?;
    Ok(cur_commit_hash)
}
