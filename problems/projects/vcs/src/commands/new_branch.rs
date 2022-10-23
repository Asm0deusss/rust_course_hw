#![forbid(unsafe_code)]

use std::{fs::File, path::Path};

use crate::{
    file_factory::{write_to_file, write_to_hash_file},
    repo_file_manager::{find_repository_root, get_file_content, get_link},
    vcs_file_manager::{get_commits_path, get_hash_file_content, make_commit},
    ErrorType, Links,
};

pub fn new_branch(branch_name: String) -> Result<String, ErrorType> {
    let cur_dir = std::fs::canonicalize(".").unwrap();
    let vcs_path = find_repository_root(Path::new(&cur_dir)).unwrap();
    let mut check_dir = vcs_path.clone();
    check_dir.push("refs");
    check_dir.push("heads");
    check_dir.push(branch_name);
    if File::open(&check_dir).is_ok() {
        return Err(ErrorType::DoubleBranch);
    }

    let cur_head = get_link(Links::Head, &vcs_path).unwrap();
    let cur_head_hash = get_file_content(&cur_head).unwrap();
    let mut master_hashes = vec![];
    get_commits_path(cur_head_hash.clone(), &mut master_hashes).unwrap();

    let index = master_hashes.iter().position(|x| *x == cur_head_hash);

    if index.is_none() {
        return Err(ErrorType::NewBranchNotFromMaster);
    }

    File::create(&check_dir)?;

    let cur_master_commit_hash = master_hashes[index.unwrap()].clone();

    let tmp = get_hash_file_content(vcs_path.clone(), &cur_master_commit_hash).unwrap();
    let master_commit_content = tmp.split(' ').collect::<Vec<&str>>();
    let tree = master_commit_content[3].to_owned();

    let (hash, encoded) = make_commit(&cur_master_commit_hash, &tree, &"Merge".to_owned()).unwrap();

    write_to_file(&cur_head, &hash.as_bytes().to_vec())?;
    write_to_file(&check_dir, &hash.as_bytes().to_vec())?;
    write_to_hash_file(&vcs_path, &hash, &encoded)?;

    Ok(cur_master_commit_hash)
}
