#![forbid(unsafe_code)]

use std::{path::{Path, PathBuf}, fs};

use crate::{ErrorType, repo_file_manager::{get_current_files_dirs, find_dir, get_file_content, get_file_hash, get_link}, vcs_file_manager::{get_files_dir_from_commit}, Links};

pub fn cur_head_status() -> Result<[Vec<String>; 3], ErrorType> {
    let cur_dir = std::fs::canonicalize("./").unwrap();
    let vcs_path = find_dir(Path::new(&cur_dir), ".vcs".to_owned())?;
    let head_path = get_link(Links::Head, vcs_path).unwrap();
    let current_head_hash = get_file_content(head_path).unwrap();

    status(current_head_hash)
}

pub fn status(current_commit_hash: String) -> Result<[Vec<String>; 3], ErrorType> {

    let mut modified: Vec<String> = vec![];
    let mut added: Vec<String> = vec![];
    let mut deleted: Vec<String> = vec![];

    let cur_dir = std::fs::canonicalize("./").unwrap();
    let vcs_path = find_dir(Path::new(&cur_dir), ".vcs".to_owned())?;

    let work_dir = vcs_path.parent().unwrap();

    let mut current_files: Vec<PathBuf> = vec![];
    let mut commit_files: Vec<PathBuf> = vec![];
    let mut commit_hash: Vec<String> = vec![];

    get_current_files_dirs(work_dir.to_path_buf(), &mut current_files);
    get_files_dir_from_commit(&vcs_path, &current_commit_hash, &mut commit_files, &mut commit_hash)?;

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
            added.push(cur_file.to_str().unwrap().to_string());
        } else if flag == 1 {
            modified.push(cur_file.to_str().unwrap().to_string());
        }
    }

    for commit_file in commit_files.iter() {
        let tmp = fs::File::open(commit_file);

        if tmp.is_err() {
            deleted.push(commit_file.to_str().unwrap().to_string());
        }
    }
    
    Ok([added, modified, deleted])
}
