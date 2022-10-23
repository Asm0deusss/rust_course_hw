#![forbid(unsafe_code)]

use core::fmt;

use clap::Args;

pub mod commands;
pub mod file_factory;
pub mod repo_file_manager;
pub mod vcs_file_manager;

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
    IoError(std::io::Error),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl std::error::Error for ErrorType {}

impl From<std::io::Error> for ErrorType {
    fn from(err: std::io::Error) -> Self {
        ErrorType::IoError(err)
    }
}

pub enum Links {
    Vcs,
    Head,
    Master,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(about = "Initializes repo")]
    Init(Init),

    #[command(about = "Prints current repo status")]
    Status,

    #[command(about = "Commits current changes")]
    Commit(Commit),

    #[command(arg_required_else_help(true))]
    #[command(about = "Jumps into branch or commit")]
    Jump(Jump),

    #[command(name("new_branch"))]
    #[command(about = "Creates new branch")]
    NewBranch(NewBranch),

    #[command(about = "Merges branch into master")]
    Merge(Merge),

    #[command(about = "Prints commits list")]
    Log,
}

#[derive(Args)]
pub struct Init {
    #[arg(long, value_name("DIRECTORY_PATH"))]
    pub path: String,
}

#[derive(Args)]
pub struct Commit {
    #[arg(long)]
    pub message: String,
}

#[derive(Debug, Args)]
pub struct Jump {
    #[arg(long, value_name("COMMIT_HASH"))]
    pub commit: Option<String>,

    #[arg(long, value_name("BRANCH_NAME"), conflicts_with("commit"))]
    pub branch: Option<String>,
}

#[derive(Args)]
pub struct NewBranch {
    #[arg(long, value_name("BRANCH_NAME"))]
    pub name: String,
}

#[derive(Args)]
pub struct Merge {
    #[arg(long, value_name("BRANCH_NAME"))]
    pub branch: String,
}

#[derive(PartialEq)]
pub enum FileStatus {
    Deleted,
    Added,
    Modified,
    NoChange,
}
