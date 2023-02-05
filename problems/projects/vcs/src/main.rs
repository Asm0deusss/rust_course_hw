#![forbid(unsafe_code)]

use clap::{Parser};
use vcs::{
    commands::{
        commit, init,
        merge::merge,
        new_branch,
        status::{self, cur_head_status, print_status}, log::{write_to_log, print_log}, jump_to_branch, jump_to_commit::jump_to_commit,
    },
    vcs_file_manager::get_branch_name,
    Commands, ErrorType,
};

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Init(path) => {
            let result = init::init(path.path.clone()).unwrap();

            println!("Initialized VCS repository in {}", path.path);
            println!("Created commit:");
            println!("[master {}] Initial commit", result);

            let status = cur_head_status().unwrap();

            write_to_log(&result, "Initial commit", &status).unwrap();
        }
        Commands::Status => {
            let result = status::cur_head_status().unwrap();

            println!("On branch {}", result.branch_name);

            if result.added.is_empty() && result.deleted.is_empty() && result.modified.is_empty() {
                println!("No changes to be committed");
            } else {
                println!("Changes to be commited:");
                print_status(&result);
            }
        }
        Commands::Commit(message) => {
            let status = status::cur_head_status().unwrap();

            if status.added.is_empty() && status.deleted.is_empty() && status.modified.is_empty() {
                println!("No changes to be committed");
                return;
            }

            let result = commit::commit(&message.message);

            if result.is_err() {
                println!("You can create a new commit only from last one.");
                println!("Aborting...");
            } else if result.is_ok() {
                let hash = result.unwrap();
                let branch_name = get_branch_name().unwrap();

                println!("[{} {}] Work in progress", branch_name, hash);
                println!(
                    "{} files changed, {} added, {} deleted",
                    status.added.len(),
                    status.modified.len(),
                    status.deleted.len()
                );
                print_status(&status);
                write_to_log(&hash, &message.message, &status).unwrap();
            }
        }
        Commands::Jump(jump) => {
            if jump.branch.is_some() {
                let result = jump_to_branch::jump_to_branch(&jump.branch.as_ref().unwrap());

                match result {
                    Ok(commit_hash) => {
                        println!("Successfully jumped to branch {}. Current commit: {}.", jump.branch.as_ref().unwrap(), commit_hash);
                    },
                    Err(err) => match err {
                        ErrorType::NoSuchBranch => {
                            println!("No branch {} exists.", jump.branch.as_ref().unwrap());
                            println!("Aborting...");
                        },
                        _ => unimplemented!(),
                    },
                }
            } else if jump.commit.is_some() {
                let result = jump_to_commit(&jump.commit.as_ref().unwrap());

                match result {
                    Ok(branch_name) => {
                        println!("Successfully jumped to commit {}. Current branch: {}.", jump.commit.as_ref().unwrap(), branch_name);
                    },
                    Err(err) => match  err {
                        ErrorType::NoSuchDir => {
                            println!("No commit with hash {} exists.", jump.commit.as_ref().unwrap());
                            println!("Aborting...");
                        },
                        _ => unimplemented!(),
                    },
                }
            }
        }
        Commands::NewBranch(name) => {
            let result = new_branch::new_branch(name.name.clone());

            match result {
                Ok(commit_hash) => {
                    println!(
                        "Created a new branch branch_name from master's commit {}",
                        commit_hash
                    );
                }
                Err(err) => match err {
                    ErrorType::DoubleBranch => {
                        println!("Branch {} already exists.", name.name);
                        println!("Aborting...")
                    }
                    ErrorType::NewBranchNotFromMaster => {
                        println!("Creating a new branch is possible only when you are in the master branch.");
                        println!("Aborting...")
                    }
                    _ => unimplemented!(),
                },
            }
        }
        Commands::Merge(branch) => {
            let result = merge(branch.branch.clone());

            match result {
                Ok(()) => {
                    let status = cur_head_status().unwrap();
                    let commit =
                        commit::commit(&("Merge branch ".to_owned() + &branch.branch)).unwrap();
                    println!("Successfully created merge commit:");
                    println!("[master {}] Merged branch {}.", commit, branch.branch);
                    print_status(&status);
                    println!("Deleted branch_with_feature");

                    write_to_log(&commit, &("Merged branch ".to_owned() + &branch.branch), &status).unwrap();

                }
                Err(err) => {
                    match err {
                        ErrorType::NotInMasterForMerge => {
                            println!("The merge is possible only when you are in the last commit in master.");
                            println!("Aborting...");
                        }
                        _ => unimplemented!(),
                    }
                }
            }
        }
        Commands::Log => {
            print_log().unwrap();
        },
    }
}
