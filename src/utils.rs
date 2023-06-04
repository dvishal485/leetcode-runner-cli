use crate::file_parser::codefile::CodeFile;
use crate::handlers::leetcode::{Authorized, LeetCode};
use crate::handlers::utils::{ExecutionResult, SubmissionResult};

use eyre::{bail, Result};

use std::process::ExitCode;

pub(crate) fn execute_testcases(
    filename: Option<String>,
    testcases: Option<String>,
    lc: &LeetCode<Authorized>,
) -> Result<(bool, CodeFile)> {
    let code_file = if let Some(path) = filename {
        CodeFile::from_file(&path)
    } else {
        CodeFile::from_dir()
    };

    match testcases {
        Some(testcases) => {
            let data_input = std::fs::read_to_string(&testcases)?;

            match lc.execute(&code_file, data_input)? {
                ExecutionResult::Success(result) => {
                    result.display();
                    return Ok((result.is_correct(), code_file));
                }
                ExecutionResult::LimitExceeded(limit_exceeded) => {
                    bail!(limit_exceeded); // TODO(nozwock): impl Display for these, so you can do `fail_state => bail!(fail_state),`
                }
                ExecutionResult::CompileError(compile_error) => {
                    bail!(compile_error);
                }
                ExecutionResult::RuntimeError(runtime_error) => {
                    bail!(runtime_error);
                }
                ExecutionResult::PendingResult(pending) => {
                    bail!(pending.state);
                }
                ExecutionResult::Unknown(_) => {
                    bail!("Unknown error");
                }
            }
        }
        None => {
            match lc.execute_default(&code_file)? {
                ExecutionResult::Success(result) => {
                    result.display();
                    // if !result.is_correct() {
                    //     println!(
                    //         "{}",
                    //         "Testcases can be found in testcase.txt".yellow().italic()
                    //     );
                    // }
                    return Ok((result.is_correct(), code_file));
                }
                ExecutionResult::LimitExceeded(limit_exceeded) => {
                    bail!(limit_exceeded);
                }
                ExecutionResult::CompileError(compile_error) => {
                    bail!(compile_error);
                }
                ExecutionResult::RuntimeError(runtime_error) => {
                    bail!(runtime_error);
                }
                ExecutionResult::PendingResult(pending) => {
                    bail!(pending.state);
                }
                ExecutionResult::Unknown(_) => {
                    bail!("Unknown error");
                }
            }
        }
    }
}

pub(crate) fn submit(lc: &LeetCode<Authorized>, code_file: CodeFile) -> Result<()> {
    match lc.submit(&code_file)? {
        SubmissionResult::Success(success) => success.display(),
        SubmissionResult::LimitExceeded(wrong) => {
            bail!(wrong)
        }
        SubmissionResult::PendingResult(state) => {
            bail!(state.state)
        }
        SubmissionResult::CompileError(compile_err) => {
            bail!(compile_err)
        }
        SubmissionResult::RuntimeError(runtime_error) => {
            bail!(runtime_error)
        }
        SubmissionResult::Wrong(wrong) => {
            bail!(wrong)
        }
        SubmissionResult::Unknown(_) => {
            bail!("Unknown error")
        }
    };

    Ok(())
}
