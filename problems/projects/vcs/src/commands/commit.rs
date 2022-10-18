#![forbid(unsafe_code)]

use std::path::{Path};

use crate::{ErrorType, repo_file_manager::{find_dir, get_link, get_file_content}, Links, vcs_file_manager::{make_tree, make_commit}, file_factory::{write_to_file, write_to_hash_file}};

pub fn commit (message: String) -> Result<(), ErrorType>{
    let cur_dir = std::fs::canonicalize("./").unwrap();
    let vcs_path = find_dir(Path::new(&cur_dir), ".vcs".to_owned())?;
    let work_dir = vcs_path.parent().unwrap();
    let cur_head = get_link(Links::Head, work_dir.to_path_buf()).unwrap();
    let last_master_commit = get_link(Links::Master, work_dir.to_path_buf()).unwrap();
    let cur_head_hash = get_file_content(cur_head.clone())?;

    let mut is_last_commit = false;
    let mut last_branch = last_master_commit.clone();

    let mut heads_path = vcs_path.clone();
    heads_path.push("refs");
    heads_path.push("heads");

    let tmp_paths = std::fs::read_dir(heads_path);
    if tmp_paths.is_err() {
        return Err(ErrorType::NoSuchDir);
    }
    let paths = tmp_paths.unwrap();

    for cur_path in paths {
        let cur_branch = cur_path.unwrap();
        let cur_branch_hash = get_file_content(cur_branch.path())?;

        if cur_branch_hash == cur_head_hash {
            last_branch = cur_branch.path();
            is_last_commit = true;
            break;
        }
    }

    if !is_last_commit {
        return Err(ErrorType::BadCommit);
    }

    let cur_tree = make_tree(vcs_path.clone(), work_dir.to_path_buf()).unwrap();

    let parent = get_file_content(last_branch.clone()).unwrap();
    let (hash, encoded) = make_commit(&parent, &cur_tree, &message)?;

    write_to_file(&cur_head, &hash.as_bytes().to_vec());
    write_to_file(&last_branch, &hash.as_bytes().to_vec());

    write_to_hash_file(&vcs_path, &hash, &encoded);

    println!("COMMIT: {} with message {}", hash, message);

    Ok(())
}
