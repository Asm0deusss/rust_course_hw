#![forbid(unsafe_code)]

use std::{fs::{OpenOptions, self}, io::{BufWriter, Write, BufReader, BufRead}};

use chrono::Local;

use crate::{repo_file_manager::get_link, ErrorType, Links};

use super::status::VcsSnapshotDiff;

pub fn write_to_log (commit_hash: &str, message: &str, status: &VcsSnapshotDiff) ->Result<(), ErrorType> {

    let cur_dir = std::fs::canonicalize(".").unwrap();
    let vcs_path = get_link(Links::Vcs, &cur_dir)?;
    
    let log_path = vcs_path.join("log");
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(log_path)
        .expect("unable to open file");
    let mut f = BufWriter::new(f);

    write!(f, "commit {}\n", commit_hash).expect("unable to write");
    
    let date_time: chrono::DateTime<Local> = chrono::Local::now();

    write!(f, "Date: {}\n", date_time.to_rfc2822()).expect("unable to write");
    write!(f, "Message: {}\n", message).expect("unable to write");

    if status.added.is_empty() && status.deleted.is_empty() && status.modified.is_empty() {
        write!(f, " No changes").expect("unable to write");
    } else {
        write!(f, " Changes\n").expect("unable to write");

        for i in status.modified.iter() {
            write!(f, " modified: {}\n", i.to_str().unwrap()).expect("unable to write");
        }
        for i in status.added.iter() {
            write!(f, " added: {}\n", i.to_str().unwrap()).expect("unable to write");
        }
        for i in status.deleted.iter() {
            write!(f, " deleted: {}\n", i.to_str().unwrap()).expect("unable to write");
        }
    }

    write!(f, "\n\n",).expect("unable to write");

    Ok(())
}

pub fn print_log() -> Result<(), ErrorType> {
    let cur_dir = std::fs::canonicalize(".").unwrap();
    let vcs_path = get_link(Links::Vcs, &cur_dir)?;

    let log_path = vcs_path.join("log");
    let log_file = fs::File::open(log_path)?;
    let reader = BufReader::new(log_file);
    for str in reader.lines() {
        println!("{}", str?);
    }

    Ok(())
}
