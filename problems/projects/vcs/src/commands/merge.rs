#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::{
    file_factory::{make_file_from_hash, delete_hash_file},
    repo_file_manager::{get_delta_files_paths, get_file_content, get_link},
    vcs_file_manager::{get_branch_hash, get_commits_path, get_files_dir_from_commit},
    ErrorType, FileStatus, Links,
};

pub fn merge(branch_name: String) -> Result<(), ErrorType> {
    let cur_dir = std::fs::canonicalize(".").unwrap();
    let vcs_path = get_link(Links::Vcs, &cur_dir)?;
    let cur_head_commit_hash = get_file_content(&get_link(Links::Head, &vcs_path).unwrap())?;
    let last_master_commit_hash = get_file_content(&get_link(Links::Master, &vcs_path).unwrap())?;

    if cur_head_commit_hash != last_master_commit_hash {
        println!("{} {}", cur_head_commit_hash, last_master_commit_hash);
        return Err(ErrorType::NotInMasterForMerge);
    }

    let last_branch_commit_hash = get_branch_hash(&branch_name)?;

    let mut branch_commits_hashes: Vec<String> = vec![];
    get_commits_path(last_branch_commit_hash.clone(), &mut branch_commits_hashes)?;

    let mut master_commits_hashes: Vec<String> = vec![];
    get_commits_path(last_master_commit_hash.clone(), &mut master_commits_hashes)?;

    let mut last_general_commit_hash: String = last_branch_commit_hash.clone();

    branch_commits_hashes.reverse();
    master_commits_hashes.reverse();

    let mut find_index: usize = 0;
    while find_index < branch_commits_hashes.len() && find_index < master_commits_hashes.len() {
        if branch_commits_hashes[find_index] == master_commits_hashes[find_index] {
            last_general_commit_hash = branch_commits_hashes[find_index].clone();
            find_index += 1;
        } else {
            break;
        }
    }

    let mut general_files_paths: Vec<PathBuf> = vec![];
    let mut general_files_hashes: Vec<String> = vec![];
    get_files_dir_from_commit(
        vcs_path.clone(),
        &last_general_commit_hash,
        &mut general_files_paths,
        &mut general_files_hashes,
    )?;

    let mut branch_files_paths: Vec<PathBuf> = vec![];
    let mut branch_files_hashes: Vec<String> = vec![];
    get_files_dir_from_commit(
        vcs_path.clone(),
        &branch_commits_hashes.last().unwrap(),
        &mut branch_files_paths,
        &mut branch_files_hashes,
    )?;

    let mut master_files_paths: Vec<PathBuf> = vec![];
    let mut master_files_hashes: Vec<String> = vec![];
    get_files_dir_from_commit(
        vcs_path.clone(),
        &master_commits_hashes.last().unwrap(),
        &mut master_files_paths,
        &mut master_files_hashes,
    )?;

    let general_to_branch = get_delta_files_paths(
        &general_files_paths,
        &general_files_hashes,
        &branch_files_paths,
        &branch_files_hashes,
    );
    let general_to_master = get_delta_files_paths(
        &general_files_paths,
        &general_files_hashes,
        &master_files_paths,
        &master_files_hashes,
    );

    let mut both_mod: Vec<PathBuf> = vec![];

    for path in general_to_branch.path.iter().enumerate() {
        if general_to_branch.meta[path.0] == FileStatus::Deleted {
            continue;
        }

        let mut master_iter = general_to_master.path.iter();
        let index = master_iter.position(|x| *x == *path.1);

        if index.is_some() {
            if general_to_branch.hash[path.0] != general_to_master.hash[index.unwrap()] {
                both_mod.push(general_to_branch.path[path.0].clone());
            }
        }
    }

    if both_mod.len() != 0 {
        println!("Merge confilict: file has been changed both in master and branch");
        for bm in both_mod.iter() {
            println!("{}", bm.to_str().unwrap());
        }
        println!("Aborting...");
        return Err(ErrorType::MergeConflict);
    }

    for i in 0..branch_files_paths.len() {
        make_file_from_hash(&branch_files_paths[i], &branch_files_hashes[i])?;
    }

    for i in branch_commits_hashes.len()..find_index {
        delete_hash_file(&branch_commits_hashes[i])?;
    }

    Ok(())
}
