use crate::args::Cli;
use crate::file_parser::codefile::CodeFile;
use crate::handlers::html_opener::open_html;
use crate::utils::{execute_testcases, submit};

use args::Commands;
use clap::Parser;
use colored::Colorize;
use eyre::{bail, Result};
use handlers::leetcode::LeetCode;

mod args;
mod file_parser;
mod handlers;
mod utils;

const LC_COOKIE_ENV_KEY: &str = "LC_COOKIE";

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cookie = std::env::var_os(LC_COOKIE_ENV_KEY)
        .ok_or_else(|| eyre::eyre!("{} is not set in the environment.", LC_COOKIE_ENV_KEY))?
        .into_string()
        .map_err(|_| eyre::eyre!("Invalid Unicode found"))?;

    let lc = LeetCode::new().authenticate(&cookie)?;

    match cli.command {
        Some(Commands::Auth) => match lc.get_metadata() {
            Ok(metadata) => metadata.display(),
            Err(err) => bail!(err),
        },
        Some(Commands::DailyChallenge) => {
            let daily_challenge = lc.get_daily_challenge()?;
            println!("Today's Daily Challenge: {}", daily_challenge);
            let title = daily_challenge.question.titleSlug;
            let question = lc.question_content(&title)?;

            let filename = "daily_challenge.html";
            std::fs::write(filename, question.content)?;
            println!("Saved question as HTML to {}", filename.cyan());
            open_html(filename);
        }
        Some(Commands::Question { question_name }) => {
            let question_name = if let Some(idx) = question_name.find("leetcode.com/problems/") {
                let problem = (&question_name[idx..]).split_whitespace().next().unwrap();
                let problem = problem.split('/').skip(2).next().unwrap();
                problem
            } else {
                &question_name
            };
            let question = lc.question_content(question_name)?;
            let filename = format!("{}.html", question_name);

            // save to filename
            std::fs::write(&filename, question.content)?;
            println!("Saved question as HTML to {}", filename.cyan());
            open_html(&filename);
        }
        Some(Commands::RunCustom {
            testcases,
            filename,
        }) => {
            _ = execute_testcases(filename, Some(testcases), &lc)?;
            // bail if `is_correct == false`?
        }
        Some(Commands::Run { filename }) => {
            _ = execute_testcases(filename, None, &lc)?;
        }
        Some(Commands::FastSubmit { filename }) => {
            let code_file = if let Some(path) = filename {
                CodeFile::from_file(&path)?
            } else {
                CodeFile::from_dir()?
            };

            submit(&lc, code_file)?;
        }
        Some(Commands::Submit { filename }) => {
            let (is_correct, code_file) = execute_testcases(filename, None, &lc)?;
            if is_correct {
                submit(&lc, code_file)?;
            } else {
                bail!(
                    "{}",
                    "Aborting submission due to failed testcase(s)".red().bold()
                );
            }
        }
        None => {}
    };

    Ok(())
}
