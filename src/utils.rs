use crate::file_parser::codefile::CodeFile;
use crate::handlers::leetcode::{Authorized, LeetCode};
use crate::handlers::utils::{ExecutionResult, SubmissionResult};

use colored::Colorize;
use eyre::Result;

use std::process::ExitCode;

pub(crate) fn execute_testcases(
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
        let Ok(data_input) = std::fs::read_to_string(&testcases) else{
            eprintln!("Error opening testcases file!");
            return (false, code_file);
        };
        match lc.execute(&code_file, data_input) {
            Ok(result) => {
                println!();
                is_correct = match result {
                    ExecutionResult::Success(result) => {
                        result.display();
                        result.is_correct()
                    }
                    ExecutionResult::LimitExceeded(limit_exceeded) => {
                        limit_exceeded.display();
                        false
                    }
                    ExecutionResult::CompileError(compile_error) => {
                        compile_error.display();
                        false
                    }
                    ExecutionResult::RuntimeError(runtime_error) => {
                        runtime_error.display();
                        false
                    }
                    ExecutionResult::PendingResult(state) => {
                        println!("{}", state);
                        false
                    }
                    ExecutionResult::Unknown(_) => {
                        eprintln!("Unknown Error!");
                        false
                    }
                }
            }
            Err(e) => {
                eprintln!("Some error occured! {e}");
                is_correct = false;
            }
        }
    } else {
        let result = lc.execute_default(&code_file);
        println!();
        match result {
            Ok(result) => {
                is_correct = match result {
                    ExecutionResult::Success(result) => {
                        result.display();
                        if !result.is_correct() {
                            println!(
                                "{}",
                                "Testcases can be found in testcase.txt".yellow().italic()
                            );
                        }
                        result.is_correct()
                    }
                    ExecutionResult::LimitExceeded(limit_exceeded) => {
                        limit_exceeded.display();
                        false
                    }
                    ExecutionResult::CompileError(compile_error) => {
                        compile_error.display();
                        false
                    }
                    ExecutionResult::RuntimeError(runtime_error) => {
                        runtime_error.display();
                        false
                    }
                    ExecutionResult::PendingResult(state) => {
                        println!("{}", state);
                        false
                    }
                    ExecutionResult::Unknown(_) => {
                        eprintln!("Unknown Error!");
                        false
                    }
                }
            }
            Err(e) => {
                eprintln!("Some error occured! {e}");
                is_correct = false;
            }
        }
    }
    (is_correct, code_file)
}

pub(crate) fn submit(lc: &LeetCode<Authorized>, code_file: CodeFile) -> ExitCode {
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
                println!("{}", state);
                ExitCode::FAILURE
            }
            SubmissionResult::CompileError(compile_err) => {
                compile_err.display();
                ExitCode::FAILURE
            }
            SubmissionResult::RuntimeError(runtime_error) => {
                runtime_error.display();
                ExitCode::FAILURE
            }
            SubmissionResult::Wrong(wrong) => {
                wrong.display();
                ExitCode::FAILURE
            }
            SubmissionResult::Unknown(_) => {
                eprintln!("Unknown Error!");
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Some error occured! {e}");
            return ExitCode::FAILURE;
        }
    }
}
