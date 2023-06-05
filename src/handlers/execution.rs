use std::fmt;

use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Success {
    pub status_code: u8,
    pub lang: String,
    pub run_success: bool,
    pub status_runtime: String,
    pub memory: u64,
    pub code_answer: Vec<String>,
    pub code_output: Vec<String>,
    pub std_output: Vec<String>,
    pub elapsed_time: u64,
    pub task_finish_time: u64,
    pub expected_status_code: u8,
    pub expected_lang: String,
    pub expected_run_success: bool,
    pub expected_status_runtime: String,
    pub expected_memory: u64,
    pub expected_code_answer: Vec<String>,
    pub expected_code_output: Vec<String>,
    pub expected_std_output: Vec<String>,
    pub expected_elapsed_time: u64,
    pub expected_task_finish_time: u64,
    pub correct_answer: bool,
    pub compare_result: String,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub status_memory: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct CompileError {
    pub compile_error: String,
    pub full_compile_error: String,
    pub std_output: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeError {
    pub runtime_error: String,
    pub full_runtime_error: String,
    pub std_output: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LimitExceeded {
    pub status_code: u8,
    pub lang: String,
    pub run_success: bool,
    pub status_runtime: String,
    pub memory: u64,
    pub code_answer: Vec<String>,
    pub code_output: Vec<String>,
    pub std_output: Vec<String>,
    pub elapsed_time: u64,
    pub task_finish_time: u64,
    pub total_correct: Option<u8>,
    pub total_testcases: Option<u8>,
    pub status_memory: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
}

impl fmt::Display for LimitExceeded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "{}\n{}\nTime Elapsed : {}\nMemory : {}",
            self.status_msg.red().bold(),
            seperator.yellow(),
            self.elapsed_time,
            self.memory
        )
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "{}\n{}\nError Message : {}\n\nFull error message :\n{}",
            "Compilation Error!".red().bold(),
            seperator.yellow(),
            self.compile_error,
            self.full_compile_error
        )
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "{}\nTestcase {} failed during execution!\n{}\n{}\n {}\n\n{}\n{}\n{}\n{}",
            "Runtime Error!".red().bold(),
            format!("{}", self.std_output.len()).red(),
            seperator.yellow(),
            "Error Message :".yellow(),
            self.runtime_error,
            "Full error message :".yellow(),
            self.full_runtime_error,
            seperator.yellow(),
            format!("{}\n{:?}", "Std Output :".yellow(), self.std_output)
        )
    }
}

impl fmt::Display for Success {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seperator = "-------------------------------";
        let part1 = format!(
            "{}\n\n",
            if self.correct_answer {
                "Testcase execution success".green().bold()
            } else {
                format!(
                    "Testcase {}/{} testcase passed",
                    self.total_correct, self.total_testcases
                )
                .red()
                .bold()
            }
        );
        let mut part2 = Vec::with_capacity(self.code_answer.len());
        for i in 0..self.code_answer.len() {
            let is_correct = self.compare_result.chars().nth(i).unwrap_or('0') == '1';
            part2.push(format!(
                "{1}\n{2}\n{3}\n{0:10}: {4:?}\n{7:10}: {5:?}\n\n{6}",
                "Output",
                seperator.yellow(),
                if is_correct {
                    format!("Testcase {} execution success", i + 1).green()
                } else {
                    format!("Testcase {} execution failed", i + 1).red()
                },
                seperator.yellow(),
                self.code_answer[i],
                self.expected_code_answer[i],
                if !self.std_output[i].is_empty() {
                    format!("Std Output :\n{}\n", self.std_output[i])
                } else {
                    String::new()
                },
                "Expected"
            ));
        }

        let may_tle = {
            // added 200 because leetcode data is not very reliabale
            self.expected_elapsed_time + 200 < self.elapsed_time
        };

        let part3 = format!(
            "{}\n{:10}: {:6} ({}%)\n{}",
            seperator.yellow(),
            "Runtime",
            self.status_runtime.cyan(),
            100 - (self.elapsed_time as u128 * 100)
                .checked_div(self.expected_elapsed_time as u128 + 100)
                .unwrap_or(100)
                .min(100),
            if may_tle {
                "High runtime detected! May lead to TLE\n"
                    .red()
                    .italic()
                    .to_string()
            } else {
                "".to_string()
            },
        );

        let part4 = format!(
            "{2:10}: {:6} ({}%)\n",
            self.status_memory.cyan(),
            100 - (self.memory as u128 * 100)
                .checked_div(self.expected_memory as u128)
                .unwrap_or(100)
                .min(100),
            "Memory",
        );
        write!(f, "{}{}{}{}", part1, part2.join(""), part3, part4)
    }
}

impl Success {
    pub fn is_correct(&self) -> bool {
        self.correct_answer
    }
}
