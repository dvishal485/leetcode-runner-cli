use crate::file_parser::codefile::CodeFile;
use crate::handlers::html_opener::open_html;
use crate::utils::{execute_testcases, submit};
use clap::{Parser, Subcommand};
use colored::Colorize;
use handlers::leetcode::LeetCode;
use std::process::ExitCode;

mod file_parser;
mod handlers;
mod utils;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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
enum Execute {
    #[command(visible_alias = "-t")]
    Testcases {
        /// File to run
        filename: Option<String>,
        /// Testcases to run
        testcases: Option<String>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let key = "LC_COOKIE";
    let Some(cookie) = std::env::var_os(key) else {
        eprintln!("{} is not set in the environment.", key);
        return ExitCode::FAILURE;
    };
    let Some( cookie) = cookie.to_str() else {
        eprintln!("Invalid characted found in cookie!");
        return ExitCode::FAILURE;
    };

    let lc = LeetCode::new();
    let lc = lc.authenticate(cookie).unwrap();

    if let None = cli.command {
        println!(
            "🪛 {}\nExecute and submit code on LeetCode directly from your terminal!\n\nUsage : {}",
            "Leetcode Runner CLI Tool".bold().yellow(),
            "leetcode-runner-cli -h".cyan().italic()
        );
        ExitCode::SUCCESS
    } else {
        let cli = cli.command.unwrap();
        match cli {
            Commands::Auth => {
                let metadata = lc.get_metadata();
                if metadata.is_ok() {
                    println!("Authenticated successfully!\n");
                    metadata.unwrap().display();
                    ExitCode::SUCCESS
                } else {
                    let error = metadata.unwrap_err();
                    eprintln!("Authentication Error : {}", error);
                    ExitCode::FAILURE
                }
            }
            Commands::DailyChallenge => {
                let daily_challenge = lc.get_daily_challenge();
                if daily_challenge.is_ok() {
                    let daily_challenge = daily_challenge.unwrap();
                    println!("Today's Daily Challenge :");
                    println!("{}", &daily_challenge);
                    let title = daily_challenge.question.titleSlug;
                    let question = lc.question_content(&title);
                    if question.is_ok() {
                        let question = question.unwrap();
                        // save to filename
                        let filename = "daily_challenge.html";
                        if let Ok(_) = std::fs::write(filename, question.content) {
                            println!("Saved question as HTML to {}", filename.cyan());
                            open_html(filename);
                            ExitCode::SUCCESS
                        } else {
                            eprintln!("Error saving question as HTML");
                            ExitCode::FAILURE
                        }
                    } else {
                        eprintln!("Error getting question content : {}", question.unwrap_err());
                        ExitCode::FAILURE
                    }
                } else {
                    eprintln!(
                        "Error getting daily challenge : {}",
                        daily_challenge.unwrap_err()
                    );
                    ExitCode::FAILURE
                }
            }
            Commands::Question { question_name } => {
                let question_name = if let Some(idx) = question_name.find("leetcode.com/problems/")
                {
                    let problem = (&question_name[idx..]).split_whitespace().next().unwrap();
                    let problem = problem.split('/').skip(2).next().unwrap();
                    problem
                } else {
                    &question_name
                };
                let question = lc.question_content(question_name);
                if question.is_ok() {
                    let question = question.unwrap();
                    let filename = format!("{}.html", question_name);
                    // save to filename
                    if let Ok(_) = std::fs::write(&filename, question.content) {
                        println!("Saved question as HTML to {}", filename.cyan());
                        open_html(&filename);
                        ExitCode::SUCCESS
                    } else {
                        eprintln!("Error saving question as HTML");
                        ExitCode::FAILURE
                    }
                } else {
                    eprintln!("Error getting question content : {}", question.unwrap_err());
                    ExitCode::FAILURE
                }
            }
            Commands::RunCustom {
                testcases,
                filename,
            } => {
                if execute_testcases(filename, Some(testcases), &lc).0 {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Commands::Run { filename } => {
                if execute_testcases(filename, None, &lc).0 {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Commands::FastSubmit { filename } => {
                let code_file = if filename.is_some() {
                    CodeFile::from_file(&filename.unwrap())
                } else {
                    CodeFile::from_dir()
                };

                submit(&lc, code_file)
            }
            Commands::Submit { filename } => {
                let (testcase_result, code_file) = execute_testcases(filename, None, &lc);
                if !testcase_result {
                    println!(
                        "{}",
                        "Aborting submission due to failed testcase(s)!"
                            .red()
                            .bold()
                    );
                    ExitCode::FAILURE
                } else {
                    submit(&lc, code_file)
                }
            }
        }
    }
}
