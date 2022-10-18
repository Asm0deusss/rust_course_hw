#![forbid(unsafe_code)]

use std::{fs, io::Write};

use crate::ErrorType;

use super::commit::commit;

pub fn init(mut path: String) -> Result<(), ErrorType>{
    path = path + "/.vcs";

    let result = fs::create_dir_all(&path);

    if result.is_err() {
        return Err(ErrorType::BadInitDir);
    }

    let make_dirs = ["/objects", "/refs/heads", "/encoded"];
    for new_dir in make_dirs {
        fs::create_dir_all(&(path.clone() + new_dir));
    }

    let make_files = ["/HEAD", "/log", "/description", "/refs/heads/master"];
    for new_file in make_files {
        let mut res = fs::File::create(&(path.clone() + new_file)).unwrap();
        if new_file == "/refs/heads/master" || new_file == "/HEAD" {
            res.write("#".as_bytes());
        }
    }

    let mut config = fs::File::create(&(path.clone() + "/config")).unwrap();
    let config_content = ["[core]", "repositoryformatversion = 0", "filemode = false", "bare = false"];
    for content in config_content {
        config.write_all((content.to_owned() + "\n").as_bytes()).expect("Unable to write data");
    }
    
    commit("Initial commit".to_owned())?;

    Ok(())
}
