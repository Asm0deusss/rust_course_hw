#![forbid(unsafe_code)]

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use sha1::Digest;
use walkdir::{DirEntry, WalkDir};

use crate::{file_factory::encode_from_path, ErrorType, FileStatus, Links};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn get_current_files_dirs(cur_dir: &Path) -> Result<Vec<PathBuf>, ErrorType> {
    let mut result: Vec<PathBuf> = vec![];

    for cur_path in WalkDir::new(cur_dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(&e))
    {
        let new_path = cur_path.unwrap();
        if new_path.path() == cur_dir {
            continue;
        }

        if !is_hidden(&new_path)
        {
            if new_path.file_type().is_file() {
                result.push(new_path.path().to_path_buf());
            } else if new_path.file_type().is_dir() {
                let mut add_dir = get_current_files_dirs(&new_path.path())?;
                result.append(&mut add_dir);
            }
        }
    }

    Ok(result)
}

pub fn get_file_content(path: &Path) -> Result<String, ErrorType> {
    let mut res: String = String::new();
    File::open(path).unwrap().read_to_string(&mut res)?;
    Ok(res)
}

pub fn get_vec_hash(file: &Vec<u8>) -> String {
    let result = hex::encode(sha1::Sha1::digest(file));
    result
}

pub fn get_file_hash(file: PathBuf) -> String {
    let tmp_encoded = encode_from_path(&file);
    let encoded = tmp_encoded.unwrap();
    let hash = get_vec_hash(&encoded);

    hash
}

pub fn find_repository_root(starting_directory: &Path) -> Result<PathBuf, ErrorType> {
    let mut path: PathBuf = starting_directory.to_path_buf();

    loop {
        if path.join(".vcs").is_dir() {
            break Ok(path.join(".vcs"));
        } else if !path.pop() {
            break Err(ErrorType::NoSuchDir);
        }
    }
}

pub fn get_link(link: Links, starting_dir: &Path) -> Result<PathBuf, ErrorType> {
    match link {
        Links::Vcs => find_repository_root(&starting_dir),
        Links::Head => {
            let mut vcs = find_repository_root(&starting_dir)?;
            vcs.push("HEAD");
            return Ok(vcs);
        }
        Links::Master => {
            let mut vcs = find_repository_root(&starting_dir)?;
            vcs.push("refs");
            vcs.push("heads");
            vcs.push("master");
            return Ok(vcs);
        }
    }
}

pub struct DeltaFilesSnap {
    pub meta: Vec<FileStatus>,
    pub path: Vec<PathBuf>,
    pub hash: Vec<String>,
}

impl DeltaFilesSnap {
    pub fn new() -> Self {
        Self {
            meta: (vec![]),
            path: (vec![]),
            hash: (vec![]),
        }
    }
}

pub fn get_delta_files_paths(
    main_paths: &Vec<PathBuf>,
    main_hashes: &Vec<String>,
    side_paths: &Vec<PathBuf>,
    side_hashes: &Vec<String>,
) -> DeltaFilesSnap {
    let mut res = DeltaFilesSnap::new();

    for path in side_paths.iter().enumerate() {
        let mut main_iter = main_paths.iter();
        let index = main_iter.position(|x| *x == *path.1);
        if index.is_some() {
            if main_hashes[index.unwrap()] != side_hashes[path.0] {
                res.meta.push(FileStatus::Modified);
                res.path.push((*path.1.clone()).to_path_buf());
                res.hash.push(side_hashes[path.0].clone());
            } else {
                res.meta.push(FileStatus::NoChange);
                res.path.push((*path.1.clone()).to_path_buf());
                res.hash.push(side_hashes[path.0].clone());
            }
        } else {
            res.meta.push(FileStatus::Added);
            res.path.push((*path.1.clone()).to_path_buf());
            res.hash.push(side_hashes[path.0].clone());
        }
    }

    res
}
