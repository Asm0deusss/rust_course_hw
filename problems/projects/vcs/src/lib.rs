#![forbid(unsafe_code)]

pub mod commands;
pub mod vcs_file_manager;
pub mod repo_file_manager;
pub mod file_factory;

#[derive(Debug)]
pub enum ErrorType {
    BadInitDir,
    NoSuchFile,
    NoSuchDir,
    NoSuchCommit,
    NoSuchBranch,
    BadCommit,
    DoubleBranch,
    NotInRepo,
    NotMergedChanges,
    NotInMasterForMerge,
    NewBranchNotFromMaster,
    MergeConflict,
}

pub enum Links {
    Vcs,
    Head,
    Master,
}

pub enum Commands {
    Init,
    Status,
    Commit,
    JumpToCommit,
    JumpToBranch,
    NewBranch,
    Merge,
    Log,
}

