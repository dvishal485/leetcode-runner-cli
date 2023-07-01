use crate::args::Cli;
use crate::file_parser::codefile::CodeFile;
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
const GIT_README: &str = "README.md";

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
        Some(Commands::DailyChallenge) => {
            let daily_challenge = lc.get_daily_challenge()?;
            println!("Today's Daily Challenge:\n{}", daily_challenge);
            let title = daily_challenge.question.titleSlug;
            let question = lc.question_content(&title)?;
            lc.save_boiler_code(&title)?;

            let filename = "daily_challenge.html";
            std::fs::write(filename, question.content)?;
            println!("Saved question as HTML to {}", filename.cyan());
            open::that(filename)?;
        }
        Some(Commands::Question { question_name }) => {
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
            let question = lc.question_content(question_name)?;
            lc.save_boiler_code(question_name)?;
            let filename = format!("{}.html", question_name);

            // save to filename
            std::fs::write(&filename, question.content)?;
            println!("Saved question as HTML to {}", filename.cyan());
            open::that(filename)?;
        }
        Some(Commands::RunCustom { testcases, file }) => {
            _ = execute_testcases(file, Some(&testcases), &lc)?;
            // bail if `is_correct == false`?
        }
        Some(Commands::Run { file }) => {
            _ = execute_testcases(file, None, &lc)?;
        }
        Some(Commands::FastSubmit { file }) => {
            let code_file = if let Some(path) = file {
                CodeFile::from_file(&path)?
            } else {
                CodeFile::from_dir(".")?
            };

            submit(&lc, code_file)?;
        }
        Some(Commands::Submit { file }) => {
            let (is_correct, code_file) = execute_testcases(file, None, &lc)?;
            if is_correct {
                submit(&lc, code_file)?;
            } else {
                bail!("Aborting submission due to failed testcase(s)".red().bold());
            }
        }
        Some(Commands::Pack { file }) => {
            let code_file = if let Some(path) = file {
                CodeFile::from_file(&path)?
            } else {
                CodeFile::from_dir(".")?
            };
            let question = lc.question_content(&code_file.question_title)?;

            // create a directory if it doesn't exists with name of question
            // create a README.md file with the question description
            // create a file with the code
            std::fs::create_dir_all(&code_file.question_title.replace(' ', ""))?;

            std::fs::write(
                format!(
                    "{}/main.{}",
                    &code_file.question_title,
                    code_file.language.extension()
                ),
                code_file.code,
            )?;

            // dont create readme if it exists
            if let Ok(mut readme_file) = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(format!("{}/{}", &code_file.question_title, GIT_README))
            {
                println!(
                    "You can write your notes about question in {}/README.md",
                    &code_file.question_title
                );
                std::io::Write::write_all(
                    &mut readme_file,
                    format!("# {}\n", code_file.question_title).as_bytes(),
                )?;
                std::io::Write::write_all(&mut readme_file, question.content.as_bytes())?;
            } else {
                println!("{} already exists, skipping creation.", GIT_README);
            };
        }
        None => {}
    };

    Ok(())
}
