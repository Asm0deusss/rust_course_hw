
#![forbid(unsafe_code)]

use std::{path::PathBuf, fs::{File, self}, io::{Read, Write}};

use deflate::{write::ZlibEncoder, Compression};
use inflate::inflate_bytes_zlib;

use crate::{ErrorType, repo_file_manager::get_link, Links};

pub fn encode_from_path(path_from: &PathBuf) -> Result<Vec<u8>, ErrorType> {

    let _input_stream = File::open(path_from);
    if _input_stream.is_err() {
       return Err(ErrorType::NoSuchFile);
    }
    let mut input_stream = _input_stream.unwrap();
    
    let mut file_content = vec![];
    input_stream.read_to_end(&mut file_content);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
    encoder.write_all(&file_content).expect("Write error!");
    let encoded_vector = encoder.finish().expect("Failed to finish compression!");

    Ok(encoded_vector)
}

pub fn encode_from_vec(file_content: &Vec<u8>) -> Result<Vec<u8>, ErrorType> {

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
    encoder.write_all(&file_content).expect("Write error!");
    let encoded_vector = encoder.finish().expect("Failed to finish compression!");

    Ok(encoded_vector)
}

pub fn decode(path_from: &PathBuf) -> Result<Vec<u8>, ErrorType> {
    let _input_stream = File::open(&path_from);
    if _input_stream.is_err() {
       return Err(ErrorType::NoSuchFile);
    }
    let mut input_stream = _input_stream.unwrap();

    let mut encoded = vec![];
    input_stream.read_to_end(&mut encoded);

    let decoded_vector = inflate_bytes_zlib(&encoded).unwrap();
    Ok(decoded_vector)
}

pub fn write_to_file (path: &PathBuf, content: &Vec<u8>) {
    std::fs::create_dir_all(path.parent().unwrap());
    let mut output_stream = File::create(&path).unwrap();
    output_stream.write_all(content);
}

pub fn write_to_hash_file (vcs_path: &PathBuf, hash: &String, content: &Vec<u8>) {

    let mut hash_path = vcs_path.clone();
    let hash_dir_1 = &hash[..2];
    let hash_dir_2 = &hash[2..];

    hash_path.push("objects".to_owned());
    hash_path.push(hash_dir_1);
    hash_path.push(hash_dir_2);

    write_to_file(&vcs_path, content); 
}

pub fn make_file_from_hash (path: &PathBuf, hash: &String) -> Result<(), ErrorType> {
    let cur_dir = std::fs::canonicalize("./").unwrap();
    let mut hash_path = get_link(Links::Vcs, cur_dir)?;
    let hash_dir_1 = &hash[0..2];
    let hash_dir_2 = &hash[2..];

    hash_path.push("objects");
    hash_path.push(hash_dir_1);
    hash_path.push(hash_dir_2);

    let decoded_content = decode(&hash_path)?;
    
    write_to_file(path, &decoded_content);
    Ok(())
}

pub fn delete_hash_file (hash: &String) {
    let cur_dir = std::fs::canonicalize("./").unwrap();
    let mut hash_path = get_link(Links::Vcs, cur_dir).unwrap();
    let hash_dir_1 = &hash[0..2];

    hash_path.push("objects");
    hash_path.push(hash_dir_1);

    fs::remove_dir_all(hash_path);
}
