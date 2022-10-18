#![forbid(unsafe_code)]

use std::{path::{Path, PathBuf}, fs::{File}, io::{Read}};

use sha1::Digest;

use crate::{file_factory::encode_from_path, Links, ErrorType};

pub fn get_current_files_dirs (cur_dir: PathBuf, memo: &mut Vec<PathBuf>) {
    let tmp_paths = std::fs::read_dir(cur_dir);
    let paths = tmp_paths.unwrap();
    
    for cur_path in paths {
        let new_path = cur_path.unwrap();

        if !new_path.path().file_name().unwrap().to_str().unwrap().starts_with(".") {
            if new_path.file_type().unwrap().is_file() {
                memo.push(new_path.path());
            } else if new_path.file_type().unwrap().is_dir() {
                get_current_files_dirs(new_path.path(), memo);
            }
        }
    }
}

pub fn get_file_content (path: PathBuf) -> Result<String, ErrorType> {
    let mut res: String = String::new();
    File::open(path).unwrap().read_to_string(&mut res);
    Ok(res)
}

pub fn get_vec_hash (file: &Vec<u8>) -> String {
    let result = hex::encode(sha1::Sha1::digest(file));
    result
}

pub fn get_file_hash (file: PathBuf) -> String {
    let tmp_encoded = encode_from_path(&file);
    let encoded = tmp_encoded.unwrap();
    let hash = get_vec_hash(&encoded);

    hash
}

pub fn find_dir(starting_directory: &Path, dir_name: String) -> Result<PathBuf, ErrorType> {
    let mut path: PathBuf = starting_directory.into();
    let file = Path::new(&dir_name);

    loop {
        path.push(file);

        if path.is_dir() {
            break Ok(path);
        }

        if !(path.pop() && path.pop()) { // remove file && remove parent
            break Err(ErrorType::NoSuchDir);
        }
    }

}

pub fn get_link(link: Links, starting_dir:PathBuf) -> Result<PathBuf, ErrorType> {

    match link {
        Links::Vcs => {
            find_dir(&starting_dir, ".vcs".to_owned())
        },
        Links::Head => {
            let mut vcs = find_dir(&starting_dir, ".vcs".to_owned())?;
            vcs.push("HEAD");
            return Ok(vcs);
        },
        Links::Master => {
            let mut vcs = find_dir(&starting_dir, ".vcs".to_owned())?;
            vcs.push("refs");
            vcs.push("heads");
            vcs.push("master");
            return Ok(vcs);
        },
    }
}

pub fn get_delta_files_paths (main_paths: &Vec<PathBuf>, main_hashes: &Vec<String>, side_paths: &Vec<PathBuf>, side_hashes: &Vec<String>) -> (Vec<String>, Vec<PathBuf>, Vec<String>) {

    let mut res_meta: Vec<String> = vec![];
    let mut res_path: Vec<PathBuf> = vec![];
    let mut res_hash: Vec<String> = vec![];

    for path in side_paths.iter().enumerate() {
        let mut main_iter = main_paths.iter();
        let index = main_iter.position(|x| *x == *path.1);
        if index.is_some() {
            if main_hashes[index.unwrap()] != side_hashes[path.0] {
                res_meta.push("moddified".to_owned());
                res_path.push((*path.1.clone()).to_path_buf());
                res_hash.push(side_hashes[path.0].clone());
            } else {
                res_meta.push("nothing".to_owned());
                res_path.push((*path.1.clone()).to_path_buf());
                res_hash.push(side_hashes[path.0].clone());
            }
        } else {
            res_meta.push("added".to_owned());
            res_path.push((*path.1.clone()).to_path_buf());
            res_hash.push(side_hashes[path.0].clone());
        }
    }

    (res_meta, res_path, res_hash)
}
