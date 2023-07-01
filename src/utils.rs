use std::path::Path;

use crate::GIT_README;
use crate::file_parser::codefile::CodeFile;
use crate::handlers::leetcode::{Authorized, LeetCode};
use crate::handlers::utils::{ExecutionResult, SubmissionResult};

use eyre::{bail, Result};

/// The first element of the return tuple indicates whether the answer is correct.
pub(crate) fn execute_testcases<P: AsRef<Path>>(
    file_path: Option<P>,
    testcases: Option<&str>,
    lc: &LeetCode<Authorized>,
) -> Result<(bool, CodeFile)> {
    let code_file = if let Some(path) = file_path {
        CodeFile::from_file(path)?
    } else {
        CodeFile::from_dir(".")?
    };

    let eval_res = match testcases {
        Some(testcases) => {
            let data_input = std::fs::read_to_string(testcases)?;
            lc.execute(&code_file, data_input)
        }
        None => lc.execute_default(&code_file),
    }?;
    match eval_res {
        ExecutionResult::Success(result) => {
            println!("{}", result);
            Ok((result.is_correct(), code_file))
        }
        ExecutionResult::LimitExceeded(limit_exceeded) => bail!(limit_exceeded),
        ExecutionResult::CompileError(compile_error) => bail!(compile_error),
        ExecutionResult::RuntimeError(runtime_error) => bail!(runtime_error),
        ExecutionResult::PendingResult(pending) => bail!(pending.state),
        ExecutionResult::WrongTestcase(wrong_testcase) => bail!(wrong_testcase),
        ExecutionResult::Unknown(_) => bail!("Unknown error occured"),
    }
}

pub(crate) fn submit(lc: &LeetCode<Authorized>, code_file: CodeFile) -> Result<()> {
    match lc.submit(&code_file)? {
        SubmissionResult::Success(success) => println!("{}", success),
        SubmissionResult::LimitExceeded(wrong) => bail!(wrong),
        SubmissionResult::PendingResult(state) => bail!(state.state),
        SubmissionResult::CompileError(compile_err) => bail!(compile_err),
        SubmissionResult::RuntimeError(runtime_error) => bail!(runtime_error),
        SubmissionResult::Wrong(wrong) => bail!(wrong),
        SubmissionResult::Unknown(_) => bail!("Unknown error"),
    };

    Ok(())
}

pub(crate) fn pack(lc: &LeetCode<Authorized>, file: Option<std::path::PathBuf>) -> Result<()> {
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

    let mut root_readme_file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(GIT_README)?;
    if root_readme_file.metadata()?.len() == 0 {
        std::io::Write::write_all(&mut root_readme_file, "# LeetCode Solutions\n\n".as_bytes())?;
        std::io::Write::write_all(
            &mut root_readme_file,
            format!(
                "- [{title}]({title}/main.{ex})\n",
                ex = code_file.language.extension(),
                title = code_file.question_title,
            )
            .as_bytes(),
        )?;
    } else {
        let mut contents = String::new();
        std::io::Read::read_to_string(&mut root_readme_file, &mut contents)?;
        if !contents.contains(&code_file.question_title) {
            std::io::Write::write_all(
                &mut root_readme_file,
                format!(
                    "- [{title}]({title}/main.{ex})\n",
                    ex = code_file.language.extension(),
                    title = code_file.question_title,
                )
                .as_bytes(),
            )?;
        }
    }
    Ok(())
}
