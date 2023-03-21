use crate::file_parser::codefile::CodeFile;
use crate::leetcode_api::worker::{ExecutionResult, SubmissionResult};

mod file_parser;
mod leetcode_api;
use clap::Parser;
use colored::Colorize;
use leetcode_api::leetcode::LeetCode;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Authenticate with LeetCode
    #[arg(short, long, action)]
    auth: bool,
    /// Code to run or submit
    #[arg(short, long, default_value_t = String::new())]
    file: String,
    /// Executes the testcases from given file
    #[arg(short, long, default_value_t = String::new())]
    testcase: String,
    /// Save question as HTML
    #[arg(short, long, default_value_t = String::new())]
    question: String,
    /// Submit the code after testcase execution
    #[arg(short, long, action)]
    submit: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let key = "LC_COOKIE";
    let Some(cookie) = std::env::var_os(key) else {
        println!("{} is not set in the environment.", key);
        return ExitCode::FAILURE;
    };
    let cookie = cookie.to_str().expect("Invalid unicode in cookie");

    let mut leetcode = LeetCode::new();
    let lc = leetcode.authenticate(cookie).unwrap();

    if args.auth {
        let metadata = lc.get_metadata();
        if metadata.is_ok() {
            println!("Authenticated successfully!\n");
            metadata.unwrap().display();
            return ExitCode::SUCCESS;
        } else {
            let error = metadata.unwrap_err();
            println!("Authentication Error : {}", error);
            return ExitCode::FAILURE;
        }
    }

    if args.question != "" {
        let title: String;
        if args.question == "daily" {
            let daily_challenge = lc.get_daily_challenge();
            if daily_challenge.is_ok() {
                let daily_challenge = daily_challenge.unwrap();
                println!("Today's Daily Challenge :");
                println!("{}", &daily_challenge);
                title = daily_challenge.question.titleSlug;
            } else {
                eprintln!(
                    "Error getting daily challenge : {}",
                    daily_challenge.unwrap_err()
                );
                return ExitCode::FAILURE;
            }
        } else {
            title = args.question.clone();
        }
        let question = lc.question_content(&title);
        if question.is_ok() {
            let question = question.unwrap();
            let filename = format!("{}.html", args.question);
            // save to filename
            if let Ok(_) = std::fs::write(&filename, question.content) {
                println!("Saved question as HTML to {}", filename.cyan());
                return ExitCode::SUCCESS;
            } else {
                println!("Error saving question as HTML");
                return ExitCode::FAILURE;
            }
        };
    }

    let filename = args.file;

    let code: CodeFile;
    if filename != "" {
        code = CodeFile::from_file(filename);
    } else {
        code = CodeFile::from_dir();
    }
    let testcase = args.testcase;
    let is_correct: bool;
    if testcase != "" {
        if let Ok(mut testcase) = std::fs::File::open(testcase) {
            let mut data_input = String::new();
            std::io::Read::read_to_string(&mut testcase, &mut data_input).unwrap();
            match lc.execute(&code, data_input) {
                Ok(result) => match result {
                    ExecutionResult::Success(result) => {
                        is_correct = result.is_correct();
                        result.display();
                    }
                    ExecutionResult::LimitExceeded(limit_exceeded) => {
                        println!("{}", limit_exceeded.status_msg);
                        println!("Time Elapsed : {}", limit_exceeded.elapsed_time);
                        println!("Memory : {}", limit_exceeded.memory);
                        return ExitCode::FAILURE;
                    }
                    ExecutionResult::CompileError(compile_error) => {
                        println!(
                            "Compile Error!\nError Message : {}\n\nFull error message :\n{}",
                            compile_error.compile_error, compile_error.full_compile_error
                        );
                        return ExitCode::FAILURE;
                    }
                    ExecutionResult::RuntimeError(runtime_error) => {
                        println!(
                            "Runtime Error!\nError Message : {}\n\nFull error message :\n{}",
                            runtime_error.runtime_error, runtime_error.full_runtime_error
                        );
                        return ExitCode::FAILURE;
                    }
                    ExecutionResult::PendingResult(state) => {
                        println!("Pending Result!");
                        println!("State : {:?}", state.state());
                        return ExitCode::FAILURE;
                    }
                    ExecutionResult::Unknown(_) => {
                        println!("Unknown Error!");
                        return ExitCode::FAILURE;
                    }
                },
                Err(e) => {
                    println!("Some error occured! {e}");
                    return ExitCode::FAILURE;
                }
            }
        } else {
            println!("Testcase file not found!");
            return ExitCode::FAILURE;
        }
    } else {
        match lc.execute_default(&code) {
            Ok(result) => match result {
                ExecutionResult::Success(result) => {
                    is_correct = result.is_correct();
                    result.display();
                    if !is_correct {
                        println!(
                            "{}",
                            "Testcases can be found in testcase.txt".yellow().italic()
                        );
                    }
                }
                ExecutionResult::LimitExceeded(limit_exceeded) => {
                    println!("{}", limit_exceeded.status_msg);
                    println!("Time Elapsed : {}", limit_exceeded.elapsed_time);
                    println!("Memory : {}", limit_exceeded.memory);
                    return ExitCode::FAILURE;
                }
                ExecutionResult::CompileError(compile_error) => {
                    println!(
                        "Compile Error!\nError Message : {}\n\nFull error message :\n{}",
                        compile_error.compile_error, compile_error.full_compile_error
                    );
                    return ExitCode::FAILURE;
                }
                ExecutionResult::RuntimeError(runtime_error) => {
                    println!(
                        "Runtime Error!\nError Message : {}\n\nFull error message :\n{}",
                        runtime_error.runtime_error, runtime_error.full_runtime_error
                    );
                    return ExitCode::FAILURE;
                }
                ExecutionResult::PendingResult(state) => {
                    println!("Pending Result!");
                    println!("State : {:?}", state.state());
                    return ExitCode::FAILURE;
                }
                ExecutionResult::Unknown(_) => {
                    println!("Unknown Error!");
                    return ExitCode::FAILURE;
                }
            },
            Err(e) => {
                println!("Some error occured! {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    if !is_correct {
        if args.submit {
            println!(
                "{}",
                "Aborting submission due to failed testcase(s)!"
                    .red()
                    .bold()
            );
        }
        return ExitCode::FAILURE;
    }
    if args.submit {
        match lc.submit(&code) {
            Ok(result) => match result {
                SubmissionResult::Success(success) => success.display(),
                SubmissionResult::LimitExceeded(wrong) => {
                    wrong.display();
                    return ExitCode::FAILURE;
                }
                SubmissionResult::PendingResult(state) => {
                    println!("Pending Result!");
                    println!("State : {:?}", state.state());
                    return ExitCode::FAILURE;
                }
                SubmissionResult::CompileError(compile_err) => {
                    println!(
                        "\nSubmission failed due to Compile Error!\nError Message :\n{}\n\nFull error message :\n{}",
                        compile_err.compile_error, compile_err.full_compile_error
                    );
                    return ExitCode::FAILURE;
                }
                SubmissionResult::RuntimeError(runtime_error) => {
                    println!(
                        "\nSubmission failed due to Runtime Error!\nError Message :\n{}\n\nFull error message :\n{}",
                        runtime_error.runtime_error, runtime_error.full_runtime_error
                    );
                    return ExitCode::FAILURE;
                }
                SubmissionResult::Wrong(wrong) => {
                    wrong.display();
                    return ExitCode::FAILURE;
                }
                SubmissionResult::Unknown(_) => {
                    println!("Unknown Error!");
                    return ExitCode::FAILURE;
                }
            },
            Err(e) => {
                println!("Some error occured! {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}
