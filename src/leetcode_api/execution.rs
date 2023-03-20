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
    pub pretty_lang: String,
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
    pub pretty_lang: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
}

impl Success {
    pub fn is_correct(&self) -> bool {
        self.correct_answer
    }
    pub fn display(&self) {
        let seperator = "-------------------------------";

        println!(
            "\n{}\n\nOutput   : {:?}\nExpected : {:?}\n",
            if self.correct_answer {
                "Testcase execution success".green().bold()
            } else {
                format!(
                    "Testcase {}/{} testcase passed",
                    self.total_correct, self.total_testcases
                )
                .red()
                .bold()
            },
            self.code_answer,
            self.expected_code_answer
        );

        for i in 0..self.code_answer.len() {
            let is_correct = self.code_answer[i] == self.expected_code_answer[i];
            println!(
                "{}\n{}\n{}\nOutput   : {:?}\nExpected : {:?}\n{}",
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
                    format!("\nStd Output :\n{}\n", self.std_output[i])
                } else {
                    format!("")
                }
            );
        }

        println!(
            "{}\nRuntime  : {}\nOutput   : {}\nExpected : {}\n",
            seperator.yellow(),
            self.status_runtime.cyan(),
            self.elapsed_time,
            self.expected_status_runtime
        );

        println!(
            "Memory   : {}\nOutput   : {}\nExpected : {}\n",
            self.status_memory.cyan(),
            self.memory,
            self.expected_memory
        );
    }
}
