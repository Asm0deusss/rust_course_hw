#![forbid(unsafe_code)]

use std::{path::Path, fs::{File}};

use crate::{ErrorType, repo_file_manager::{find_dir, get_link, get_file_content}, Links, vcs_file_manager::{get_commits_path, make_commit, get_hash_file_content}, file_factory::{write_to_file, write_to_hash_file}};

pub fn new_branch(branch_name: String) -> Result<(), ErrorType> {
    
    let cur_dir = std::fs::canonicalize("./").unwrap();
    let vcs_path = find_dir(Path::new(&cur_dir), ".vcs".to_owned()).unwrap();
    let mut check_dir = vcs_path.clone();
    check_dir.push("refs");
    check_dir.push("heads");
    check_dir.push(branch_name);
    if File::open(&check_dir).is_ok() {
        return Err(ErrorType::DoubleBranch);
    }

    let cur_head = get_link(Links::Head, vcs_path.clone()).unwrap();
    let cur_head_hash = get_file_content(cur_head.clone()).unwrap();
    let mut master_hashes = vec![];
    get_commits_path(cur_head_hash.clone(), &mut master_hashes).unwrap();

    let index = master_hashes.iter().position(|x| *x == cur_head_hash);

    if index.is_none() {
        return Err(ErrorType::NewBranchNotFromMaster);
    }

    println!("new branch {:?}", check_dir);

    File::create(&check_dir);

    let cur_master_commit_hash = master_hashes[index.unwrap()].clone();

    let tmp = get_hash_file_content(vcs_path.clone(), &cur_master_commit_hash).unwrap();
    let master_commit_content = tmp.split(' ').collect::<Vec<&str>>();
    let tree = master_commit_content[3].to_owned();

    let (hash, encoded) = make_commit(&cur_master_commit_hash, &tree, &"Merge".to_owned()).unwrap();

    write_to_file(&cur_head, &hash.as_bytes().to_vec());
    write_to_file(&check_dir, &hash.as_bytes().to_vec());
    write_to_hash_file(&vcs_path, &hash, &encoded);

    Ok(())
}
