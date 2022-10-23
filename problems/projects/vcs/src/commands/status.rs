#![forbid(unsafe_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    repo_file_manager::{
        find_repository_root, get_current_files_dirs, get_file_content, get_file_hash, get_link,
    },
    vcs_file_manager::{get_files_dir_from_commit, get_branch_name},
    ErrorType, Links,
};

pub struct VcsSnapshotDiff {
    pub modified: Vec<PathBuf>,
    pub added: Vec<PathBuf>,
    pub deleted: Vec<PathBuf>,
    pub branch_name: String,
}

impl VcsSnapshotDiff {
    fn new() -> VcsSnapshotDiff {
        VcsSnapshotDiff {
            modified: vec![],
            added: vec![],
            deleted: vec![],
            branch_name: String::new(),
        }
    }
}

pub fn print_status(status: &VcsSnapshotDiff) {
    for md in status.modified.iter() {
        println!(" modified: {}", md.to_str().unwrap());
    }
    for add in status.added.iter() {
        println!(" added: {}", add.to_str().unwrap());
    }
    for del in status.deleted.iter() {
        println!(" deleted: {}", del.to_str().unwrap());
    }
}

pub fn cur_head_status() -> Result<VcsSnapshotDiff, ErrorType> {
    let cur_dir = std::fs::canonicalize(".").unwrap();
    let vcs_path = find_repository_root(Path::new(&cur_dir))?;
    let head_path = get_link(Links::Head, &vcs_path).unwrap();
    let current_head_hash = get_file_content(&head_path).unwrap();

    status(&current_head_hash)
}

pub fn status(current_commit_hash: &str) -> Result<VcsSnapshotDiff, ErrorType> {
    let mut shap = VcsSnapshotDiff::new();

    shap.branch_name = get_branch_name()?;

    let cur_dir = std::fs::canonicalize(".").unwrap();
    let vcs_path = find_repository_root(Path::new(&cur_dir))?;

    let work_dir = vcs_path.parent().unwrap();

    let current_files: Vec<PathBuf> = get_current_files_dirs(&work_dir)?;

    let mut commit_files: Vec<PathBuf> = vec![];
    let mut commit_hash: Vec<String> = vec![];

    get_files_dir_from_commit(
        vcs_path,
        current_commit_hash,
        &mut commit_files,
        &mut commit_hash,
    )?;

    for cur_file in current_files.iter() {
        let mut flag = 0;
        for i in 0..commit_files.len() {
            if *cur_file == commit_files[i] {
                if get_file_hash(cur_file.to_path_buf()) != commit_hash[i] {
                    flag = 1;
                    break;
                } else {
                    flag = 2;
                    break;
                }
            }
        }
        if flag == 0 {
            shap.added.push(cur_file.to_path_buf());
        } else if flag == 1 {
            shap.modified.push(cur_file.to_path_buf());
        }
    }

    for commit_file in commit_files.iter() {
        let tmp = fs::File::open(commit_file);

        if tmp.is_err() {
            shap.deleted.push(commit_file.to_path_buf());
        }
    }

    Ok(shap)
}
