mod command_parser;
mod commands;
mod repo_file_manager;
mod report_printer;
mod vcs_state_manager;

use clap::Parser;

fn main() {
    let parser = command_parser::CommandParser::parse();
    println!("{:#?}", parser.command);
}
