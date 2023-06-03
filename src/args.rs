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
        /// File to execute
        filename: Option<String>,
    },
    #[command(visible_alias = "-r")]
    Run {
        /// File to execute with default testcases
        filename: Option<String>,
    },
    /// Submits code to LeetCode
    #[command(visible_alias = "-fs")]
    FastSubmit {
        /// File to submit
        filename: Option<String>,
    },
    #[command(visible_alias = "-s")]
    Submit {
        /// File to submit
        filename: Option<String>,
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
}

#[derive(Subcommand)]
pub enum Execute {
    #[command(visible_alias = "-t")]
    Testcases {
        /// File to run
        filename: Option<String>,
        /// Testcases to run
        testcases: Option<String>,
    },
}
