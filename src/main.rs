use crate::args::Cli;
use crate::file_parser::codefile::CodeFile;
use crate::utils::{execute_testcases, pack, submit};

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
const GIT_README: &str = "README.md";
const DAILY_CHALLENGE: &str = "daily_challenge.html";

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cookie = std::env::var_os(LC_COOKIE_ENV_KEY)
        .ok_or_else(|| eyre::eyre!("{} is not set in the environment.", LC_COOKIE_ENV_KEY))?
        .into_string()
        .map_err(|_| eyre::eyre!("Invalid Unicode found"))?;

    let lc = LeetCode::new().authenticate(&cookie)?;

    match cli.command {
        Some(Commands::Auth) => match lc.get_metadata() {
            Ok(metadata) => println!("{}", metadata),
            Err(err) => bail!(err),
        },
        Some(Commands::DailyChallenge { no_code_save }) => {
            let daily_challenge = lc.get_daily_challenge()?;
            println!("Today's Daily Challenge:\n{}", daily_challenge);
            let title = daily_challenge.question.titleSlug;
            if !no_code_save {
                lc.save_boiler_code(&title)?;
            }

            let question = lc.question_content(&title)?;
            std::fs::write(DAILY_CHALLENGE, question.content)?;
            println!("Saved question as HTML to {}", DAILY_CHALLENGE.cyan());
            open::that(DAILY_CHALLENGE)?;
        }
        Some(Commands::Question {
            question_name,
            no_code_save,
        }) => {
            let question_name = if let Some(idx) = question_name.find("leetcode.com/problems/") {
                let question_title = question_name[idx..]
                    .split_whitespace()
                    .next()
                    .expect("Should be Some since the find method succeed")
                    .split('/')
                    .skip(2)
                    .next()
                    .ok_or_else(|| eyre::eyre!("Invalid link, expected question identifier"))?;
                question_title
            } else {
                &question_name
            };
            if !no_code_save {
                lc.save_boiler_code(question_name)?;
            }

            let question = lc.question_content(question_name)?;
            let filename = format!("{}.html", question_name);
            // save to filename
            std::fs::write(&filename, question.content)?;
            println!("Saved question as HTML to {}", filename.cyan());
            open::that(filename)?;
        }
        Some(Commands::Run {
            file,
            testcase_file: testcases,
        }) => {
            execute_testcases(file, testcases, &lc)?;
        }
        Some(Commands::FastSubmit { file }) => {
            let code_file = if let Some(path) = file {
                CodeFile::from_file(&path)?
            } else {
                CodeFile::from_dir(".")?
            };

            submit(&lc, code_file)?;
        }
        Some(Commands::Submit {
            file,
            testcase_file: testcases,
        }) => {
            let (is_correct, code_file) = execute_testcases(file, testcases, &lc)?;
            if is_correct {
                submit(&lc, code_file)?;
            } else {
                bail!("Aborting submission due to failed testcase(s)".red().bold());
            }
        }
        Some(Commands::Pack { file }) => pack(&lc, file)?,

        None => {}
    };

    Ok(())
}
