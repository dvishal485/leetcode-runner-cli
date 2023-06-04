use crate::args::Cli;
use crate::file_parser::codefile::CodeFile;
use crate::handlers::html_opener::open_html;
use crate::utils::{execute_testcases, submit};

use args::Commands;
use clap::Parser;
use colored::Colorize;
use handlers::leetcode::LeetCode;

use std::process::ExitCode;

mod args;
mod file_parser;
mod handlers;
mod utils;

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

    match cli.command {
        Some(Commands::Auth) => {
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
        Some(Commands::DailyChallenge) => {
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
        Some(Commands::Question { question_name }) => {
            let question_name = if let Some(idx) = question_name.find("leetcode.com/problems/") {
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
        Some(Commands::RunCustom {
            testcases,
            filename,
        }) => {
            if execute_testcases(filename, Some(testcases), &lc).0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Some(Commands::Run { filename }) => {
            if execute_testcases(filename, None, &lc).0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Some(Commands::FastSubmit { filename }) => {
            let code_file = if filename.is_some() {
                CodeFile::from_file(&filename.unwrap())
            } else {
                CodeFile::from_dir()
            };

            submit(&lc, code_file)
        }
        Some(Commands::Submit { filename }) => {
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
        None => ExitCode::SUCCESS,
    }
}
