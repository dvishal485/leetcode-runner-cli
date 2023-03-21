use crate::file_parser::codefile::CodeFile;
use crate::leetcode_api::worker::{ExecutionResult, SubmissionResult};
use clap::{Parser, Subcommand};
use colored::Colorize;
use leetcode_api::leetcode::{Authorized, LeetCode};
use std::process::ExitCode;

mod file_parser;
mod leetcode_api;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with LeetCode [ -a ]
    #[command(alias = "-a")]
    Authenticate,
    /// Executes code with testcases [ -r ]
    #[command(alias = "-r")]
    Run {
        /// File to run
        filename: Option<String>,
        /// Testcases to run
        testcases: Option<String>,
    },
    /// Submits code to LeetCode [ -s ]
    #[command(alias = "-s")]
    Submit {
        /// File to submit
        filename: Option<String>,
        /// Run testcases before submitting
        run_testcases: Option<bool>,
    },
    /// Save a question as HTML [ -q ]
    #[command(alias = "-q")]
    Question {
        /// Question name
        question_name: String,
    },
    /// Save today's daily challenge as HTML [ -d ]
    #[command(alias = "-d")]
    DailyChallenge,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let key = "LC_COOKIE";
    let Some(cookie) = std::env::var_os(key) else {
        println!("{} is not set in the environment.", key);
        return ExitCode::FAILURE;
    };
    let cookie = cookie.to_str().expect("Invalid unicode in cookie");

    let mut leetcode = LeetCode::new();
    let lc = leetcode.authenticate(cookie).unwrap();

    if let None = cli.command {
        println!(
            "🪛 {}\nExecute and submit code on LeetCode directly from your terminal!\n\nUsage : {}",
            "Leetcode Runner CLI Tool".bold().yellow(),
            "leetcode-cli -h".cyan().italic()
        );
        ExitCode::SUCCESS
    } else {
        let cli = cli.command.unwrap();
        match cli {
            Commands::Authenticate => {
                let metadata = lc.get_metadata();
                if metadata.is_ok() {
                    println!("Authenticated successfully!\n");
                    metadata.unwrap().display();
                    ExitCode::SUCCESS
                } else {
                    let error = metadata.unwrap_err();
                    println!("Authentication Error : {}", error);
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
                            ExitCode::SUCCESS
                        } else {
                            println!("Error saving question as HTML");
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
                let question = lc.question_content(&question_name);
                if question.is_ok() {
                    let question = question.unwrap();
                    let filename = format!("{}.html", question_name);
                    // save to filename
                    if let Ok(_) = std::fs::write(&filename, question.content) {
                        println!("Saved question as HTML to {}", filename.cyan());
                        ExitCode::SUCCESS
                    } else {
                        println!("Error saving question as HTML");
                        ExitCode::FAILURE
                    }
                } else {
                    eprintln!("Error getting question content : {}", question.unwrap_err());
                    ExitCode::FAILURE
                }
            }
            Commands::Run {
                filename,
                testcases,
            } => {
                if execute_testcases(filename, testcases, &lc).0 {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }

            Commands::Submit {
                filename,
                run_testcases,
            } => {
                let run_testcases = run_testcases.unwrap_or(true);

                let (testcase_result, code_file) = if run_testcases {
                    execute_testcases(filename, None, &lc)
                } else if filename.is_some() {
                    (true, CodeFile::from_file(&filename.unwrap()))
                } else {
                    (true, CodeFile::from_dir())
                };

                if !testcase_result {
                    println!(
                        "{}",
                        "Aborting submission due to failed testcase(s)!"
                            .red()
                            .bold()
                    );
                    ExitCode::FAILURE
                } else {
                    match lc.submit(&code_file) {
                        Ok(result) => match result {
                            SubmissionResult::Success(success) => {
                                success.display();
                                ExitCode::SUCCESS
                            }
                            SubmissionResult::LimitExceeded(wrong) => {
                                wrong.display();
                                ExitCode::FAILURE
                            }
                            SubmissionResult::PendingResult(state) => {
                                println!("Pending Result!");
                                println!("State : {:?}", state.state());
                                ExitCode::FAILURE
                            }
                            SubmissionResult::CompileError(compile_err) => {
                                println!(
                        "\nSubmission failed due to Compile Error!\nError Message :\n{}\n\nFull error message :\n{}",
                        compile_err.compile_error, compile_err.full_compile_error
                    );
                                ExitCode::FAILURE
                            }
                            SubmissionResult::RuntimeError(runtime_error) => {
                                println!(
                        "\nSubmission failed due to Runtime Error!\nError Message :\n{}\n\nFull error message :\n{}",
                        runtime_error.runtime_error, runtime_error.full_runtime_error
                    );
                                ExitCode::FAILURE
                            }
                            SubmissionResult::Wrong(wrong) => {
                                wrong.display();
                                ExitCode::FAILURE
                            }
                            SubmissionResult::Unknown(_) => {
                                println!("Unknown Error!");
                                ExitCode::FAILURE
                            }
                        },
                        Err(e) => {
                            println!("Some error occured! {e}");
                            return ExitCode::FAILURE;
                        }
                    }
                }
            }
        }
    }
}

fn execute_testcases(
    filename: Option<String>,
    testcases: Option<String>,
    lc: &LeetCode<Authorized>,
) -> (bool, CodeFile) {
    let is_correct;
    let code_file: CodeFile;
    if let Some(filename) = filename {
        code_file = CodeFile::from_file(&filename);
    } else {
        code_file = CodeFile::from_dir();
    }
    if let Some(testcases) = testcases {
        if let Ok(mut testcase) = std::fs::File::open(testcases) {
            let mut data_input = String::new();
            std::io::Read::read_to_string(&mut testcase, &mut data_input).unwrap();

            match lc.execute(&code_file, data_input) {
                Ok(result) => match result {
                    ExecutionResult::Success(result) => {
                        result.display();
                        is_correct = result.is_correct();
                    }
                    ExecutionResult::LimitExceeded(limit_exceeded) => {
                        println!("{}", limit_exceeded.status_msg);
                        println!("Time Elapsed : {}", limit_exceeded.elapsed_time);
                        println!("Memory : {}", limit_exceeded.memory);
                        is_correct = false;
                    }
                    ExecutionResult::CompileError(compile_error) => {
                        println!(
                            "Compile Error!\nError Message : {}\n\nFull error message :\n{}",
                            compile_error.compile_error, compile_error.full_compile_error
                        );
                        is_correct = false;
                    }
                    ExecutionResult::RuntimeError(runtime_error) => {
                        println!(
                            "Runtime Error!\nError Message : {}\n\nFull error message :\n{}",
                            runtime_error.runtime_error, runtime_error.full_runtime_error
                        );
                        is_correct = false;
                    }
                    ExecutionResult::PendingResult(state) => {
                        println!("Pending Result!");
                        println!("State : {:?}", state.state());
                        is_correct = false;
                    }
                    ExecutionResult::Unknown(_) => {
                        println!("Unknown Error!");
                        is_correct = false;
                    }
                },
                Err(e) => {
                    println!("Some error occured! {e}");
                    is_correct = false;
                }
            }
        } else {
            println!("Error opening testcases file!");
            is_correct = false;
        }
    } else {
        match lc.execute_default(&code_file) {
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
                    is_correct = false;
                }
                ExecutionResult::CompileError(compile_error) => {
                    println!(
                        "Compile Error!\nError Message : {}\n\nFull error message :\n{}",
                        compile_error.compile_error, compile_error.full_compile_error
                    );
                    is_correct = false;
                }
                ExecutionResult::RuntimeError(runtime_error) => {
                    println!(
                        "Runtime Error!\nError Message : {}\n\nFull error message :\n{}",
                        runtime_error.runtime_error, runtime_error.full_runtime_error
                    );
                    is_correct = false;
                }
                ExecutionResult::PendingResult(state) => {
                    println!("Pending Result!");
                    println!("State : {:?}", state.state());
                    is_correct = false;
                }
                ExecutionResult::Unknown(_) => {
                    println!("Unknown Error!");
                    is_correct = false;
                }
            },
            Err(e) => {
                println!("Some error occured! {e}");
                is_correct = false;
            }
        }
    }
    (is_correct, code_file)
}
