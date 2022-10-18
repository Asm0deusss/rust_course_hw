#![forbid(unsafe_code)]

use std::{path::{PathBuf, Path}};

use crate::{ErrorType, file_factory::{encode_from_path, write_to_hash_file, encode_from_vec, decode}, repo_file_manager::{get_vec_hash, get_link, get_file_content, find_dir}, Links};

fn make_blob (vcs_path: PathBuf, path: PathBuf) -> Result<String, ErrorType> {

    let tmp_encoded = encode_from_path(&path);

    if tmp_encoded.is_err() {
        return Err(tmp_encoded.unwrap_err());
    }

    let encoded = tmp_encoded.unwrap();
    let hash = get_vec_hash(&encoded);

    write_to_hash_file(&vcs_path, &hash, &encoded);
    Ok(hash)
}

pub fn make_tree (vcs_path: PathBuf, path: PathBuf) -> Result<String, ErrorType>{
    let mut content: Vec<u8> = vec![];

    let tmp_paths = std::fs::read_dir(path.clone());

    if tmp_paths.is_err() {
        return Err(ErrorType::NoSuchDir);
    }

    let paths = tmp_paths.unwrap();

    for cur_path in paths {
        let cur_path_object = cur_path.unwrap();

        if !cur_path_object.file_name().to_str().unwrap().starts_with(".") {
            if cur_path_object.path().is_file() {
                let res_blob = make_blob(vcs_path.clone(), cur_path_object.path()).unwrap();
                content.append(&mut ("blob ".to_owned() + &res_blob + &" ".to_owned() + cur_path_object.path().file_name().unwrap().to_str().unwrap()).as_bytes().to_vec());
                content.append(&mut "\n".to_owned().as_bytes().to_vec());
            } else if cur_path_object.path().is_dir(){
                let res_tree = make_tree(vcs_path.clone(), cur_path_object.path()).unwrap();
                content.append(&mut ("tree ".to_owned() + &res_tree + &" ".to_owned() + cur_path_object.path().file_name().unwrap().to_str().unwrap()).as_bytes().to_vec());
                content.append(&mut "\n".to_owned().as_bytes().to_vec());
            }
        }
    }

    let encoded = encode_from_vec(&content).unwrap();
    let hash = get_vec_hash(&encoded);

    write_to_hash_file(&vcs_path, &hash, &encoded);

    Ok(hash)
}

pub fn make_commit (parent: &String, tree: &String, message: &String) -> Result<(String, Vec<u8>), ErrorType> {

    let mut hash = String::new();

    let mut content: Vec<u8> = vec![];
    let mut add_content: String = String::new();

    add_content += "parrent ";
    add_content += &parent;
    add_content += " ";
    add_content += "tree ";
    add_content += &tree;
    add_content += " ";
    add_content += &message;
    content.append(&mut add_content.as_bytes().to_vec());

    let encoded = encode_from_vec(&content).unwrap();
    hash = get_vec_hash(&encoded);

    Ok((hash, encoded))
}

pub fn get_hash_file_content (mut path: PathBuf, hash: &String) -> Result<String, ErrorType> {
    let dir = &hash[..2];
    let sub_dir = &hash[2..];

    path.push("objects".to_owned());
    path.push(dir);
    path.push(sub_dir);

    let decoded = decode(&path).unwrap();
    let res = std::str::from_utf8(&decoded).unwrap();

    Ok(res.to_string())
}

pub fn get_tree_files_dirs (vcs_path: &PathBuf, cur_path: PathBuf, tree_hash: &String, memo_path: &mut Vec<PathBuf>, memo_hash: &mut Vec<String>) -> Result<(), ErrorType>{

    let tmp_content = get_hash_file_content(vcs_path.clone(), tree_hash);
    if tmp_content.is_err() {
        return Err(ErrorType::NoSuchFile);
    }
    let content = tmp_content.unwrap();

    for object in content.split("\n") {
        let obj_content: Vec<&str> = object.split(' ').collect::<Vec<&str>>();
        if obj_content.len() != 3 {
            continue;
        }
        let obj_type = obj_content[0];
        let obj_hash = obj_content[1];
        let obj_name = obj_content[2];

        let mut new_path = cur_path.clone();
        new_path.push(obj_name);

        if !obj_name.starts_with(".") { 
            if obj_type == "blob" {
                memo_path.push(new_path);
                memo_hash.push(obj_hash.to_string());
            } else if obj_type == "tree"{
                get_tree_files_dirs(vcs_path, new_path, &obj_hash.to_owned(), memo_path, memo_hash)?;
            }
        }
    }

    Ok(())
}

pub fn get_files_dir_from_commit(vcs_path: &PathBuf, commit_hash: &String, memo_path: &mut Vec<PathBuf>, memo_hash: &mut Vec<String>) -> Result<(), ErrorType>{
    let tmp_tree_hash = get_hash_file_content(vcs_path.to_path_buf(), commit_hash)?;
    let tree_hash = tmp_tree_hash.split(' ').collect::<Vec<&str>>()[3].to_owned();
    get_tree_files_dirs(vcs_path, vcs_path.parent().unwrap().to_path_buf(), &tree_hash, memo_path, memo_hash)
}

pub fn get_branch_hash (branch_name: &String) -> Result<String, ErrorType> {

    let cur_dir = std::fs::canonicalize("./").unwrap();
    let vcs_path = get_link(Links::Vcs, cur_dir)?;

    let mut heads_path = vcs_path.clone();
    heads_path.push("refs");
    heads_path.push("heads");

    let paths = std::fs::read_dir(heads_path).unwrap();

    let mut cur_commit_hash: String = String::new();

    for branch in paths {
        let cur_branch_name = branch.unwrap();

        if cur_branch_name.file_name().to_str().unwrap() == branch_name {
            cur_commit_hash = get_file_content(cur_branch_name.path())?;
            break;
        }
    }

    if cur_commit_hash.len() == 0 {
        return Err(ErrorType::NoSuchBranch);
    }

    Ok(cur_commit_hash)
}

pub fn get_commits_path (mut commit_hash: String, memo: &mut Vec<String>) -> Result<(), ErrorType> {
    let cur_dir = std::fs::canonicalize("./").unwrap();
    let vcs_path = find_dir(Path::new(&cur_dir), ".vcs".to_owned())?;

    while commit_hash != "#" {
        memo.push(commit_hash.clone());
        let content = get_hash_file_content(vcs_path.to_path_buf(), &commit_hash)?;
        let pars_content: Vec<&str> = content.split(' ').collect::<Vec<&str>>();
        commit_hash = pars_content[1].to_string();
    }

    Ok(())
}
