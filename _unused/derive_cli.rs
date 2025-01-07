use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "tanvy", 
            about = "A CLI for indexing and searching",
            version = "0.1.0",
            author = "B.S Indo <37bodhi@gmail.com>")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}



#[derive(Subcommand)]
pub enum Commands {
    /// Index a file or directory
    Index {
        /// File to index
        #[arg(short, long, conflicts_with = "dir")]
        file: Option<String>,

        /// Directory to index
        #[arg(short, long, conflicts_with = "file")]
        dir: Option<String>,
    },

    /// Search for a word or phrase in a file or directory
    Search {
        /// Word or phrase to search
        query: String,

        /// File to search in
        #[arg(short, long, conflicts_with = "dir")]
        file: Option<String>,

        /// Directory to search in
        #[arg(short, long, conflicts_with = "file")]
        dir: Option<String>,
    },
}

