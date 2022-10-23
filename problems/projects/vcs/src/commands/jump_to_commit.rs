#![forbid(unsafe_code)]

use std::{fs, path::PathBuf};

use crate::{
    file_factory::write_to_file,
    repo_file_manager::get_link,
    vcs_file_manager::{get_files_dir_from_commit, get_hash_file_content},
    ErrorType, Links,
};

use super::status::cur_head_status;

pub fn jump_to_commit(commit_hash: &str) -> Result<String, ErrorType> {

    let cur_dir = std::fs::canonicalize(".").unwrap();
    let vcs_path = get_link(Links::Vcs, &cur_dir)?;
    let working_dir = vcs_path.parent().unwrap();

    let mut memo_path: Vec<PathBuf> = vec![];
    let mut memo_hash: Vec<String> = vec![];

    get_files_dir_from_commit(vcs_path.clone(), &commit_hash, &mut memo_path, &mut memo_hash)?;

    let status = cur_head_status().unwrap();

    if !status.added.is_empty() || !status.deleted.is_empty() || !status.modified.is_empty() {
        println!("error: Your local changes to the following files should be commited or dropped:");
        for add in status.added.iter() {
            println!("{}", add.to_str().unwrap());
        }
        for md in status.modified.iter() {
            println!("{}", md.to_str().unwrap());
        }
        println!("Please commit your changes or drop them before you jump.");
        println!("Aborting...");
        return Err(ErrorType::NotMergedChanges);
    }

    let tmp_paths = std::fs::read_dir(working_dir);
    if tmp_paths.is_err() {
        return Err(ErrorType::NoSuchDir);
    }
    let paths = tmp_paths.unwrap();

    for _file in paths {
        let file_path = _file.unwrap();

        if !file_path.file_name().to_str().unwrap().starts_with(".") {
            if file_path.file_type().unwrap().is_dir() {
                fs::remove_dir_all(file_path.path()).unwrap();
            } else if file_path.file_type().unwrap().is_file() {
                fs::remove_file(file_path.path()).unwrap();
            }
        }
    }

    for i in 0..memo_hash.len() {
        let cur_file_content = get_hash_file_content(vcs_path.clone(), &memo_hash[i]);
        write_to_file(
            &memo_path[i],
            &cur_file_content.unwrap().as_bytes().to_vec(),
        )?;
    }

    let head = get_link(Links::Head, &vcs_path)?;
    write_to_file(&head, &commit_hash.as_bytes().to_vec())?;

    let branch: String = String::new();

    

    Ok(branch)
}
