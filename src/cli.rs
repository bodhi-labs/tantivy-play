use clap::{value_parser, Arg, ArgAction, Command};
use colored::*;

pub fn build_cli() -> Command {
    Command::new("mindexr")
        .version("0.1.0")
        .author("B.S Indo <37bodhi@gmail.com>")
        .about(format!("{}\n{}",
            "A CLI tool for indexing and searching files".bright_white().bold(),
            "Manage and search through your files efficiently".bright_black()
        ))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Run indexing or search operations")
                .subcommand(
                    Command::new("index")
                        .about("Index files or directories")
                        .arg(
                            Arg::new("file")
                                .short('F')
                                .long("file")
                                .value_name("FILE")
                                .help("File path to index")
                                .required(true)
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(String))
                                .conflicts_with("dir"),
                        )
                        .arg(
                            Arg::new("dir")
                                .short('D')
                                .long("dir")
                                .value_name("DIR")
                                .help("Directory path to index")
                                .required(true)
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(String))
                                .conflicts_with("file"),
                        ),
                )
                .subcommand(
                    Command::new("search")
                        .about("Search for words or phrases")
                        .arg(
                            Arg::new("query")
                                .help("Word or phrase to search for")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::new("file")
                                .short('F')
                                .long("file")
                                .value_name("FILE")
                                .help("File path to search in")
                                .required(true)
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(String))
                                .conflicts_with("dir"),
                        )
                        .arg(
                            Arg::new("dir")
                                .short('D')
                                .long("dir")
                                .value_name("DIR")
                                .help("Directory path to search in")
                                .required(true)
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(String))
                                .conflicts_with("file"),
                        ),
                ),
        )
}