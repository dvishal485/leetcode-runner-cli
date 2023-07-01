use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, arg_required_else_help = true)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate with LeetCode
    #[command(visible_alias = "-a")]
    Auth,
    /// Executes code with testcases
    #[command(visible_alias = "-rt")]
    RunCustom {
        /// Testcases to run
        testcases: String,
        #[arg(short, long)]
        /// File to execute
        file: Option<PathBuf>,
    },
    #[command(visible_alias = "-r")]
    Run {
        #[arg(short, long)]
        /// File to execute with default testcases
        file: Option<PathBuf>,
    },
    /// Submits code to LeetCode
    #[command(visible_alias = "-fs")]
    FastSubmit {
        #[arg(short, long)]
        /// File to submit
        file: Option<PathBuf>,
    },
    #[command(visible_alias = "-s")]
    Submit {
        #[arg(short, long)]
        /// File to submit
        file: Option<PathBuf>,
    },
    /// Save a question as HTML
    #[command(visible_alias = "-q")]
    Question {
        /// Question name
        question_name: String,
    },
    /// Save today's daily challenge as HTML
    #[command(visible_alias = "-d")]
    DailyChallenge,
    /// Packs the solution with the question into a directory
    #[command(visible_alias = "-p")]
    Pack {
        /// File to pack
        file: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum Execute {
    #[command(visible_alias = "-t")]
    Testcases {
        #[arg(short, long)]
        /// File to run
        file: Option<PathBuf>,
        #[arg(short, long)]
        /// Testcases to run
        testcases: Option<String>,
    },
}
