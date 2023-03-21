use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubmitCorrect {
    pub submission_id: String,
    pub lang: String,
    pub question_id: String,
    pub status_code: u8,
    pub run_success: bool,
    pub status_msg: String,
    pub compare_result: String,
    pub state: String,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub status_runtime: String,
    pub status_memory: String,
    pub runtime_percentile: f64,
    pub memory_percentile: f64,
}

#[derive(Debug, Deserialize)]
pub struct SubmitLimitExceeded {
    pub submission_id: String,
    pub lang: String,
    pub question_id: String,
    pub status_code: u8,
    pub run_success: bool,
    pub status_msg: String,
    pub compare_result: String,
    pub state: String,
    pub total_correct: u8,
    pub total_testcases: u8,
}

#[derive(Debug, Deserialize)]
pub struct SubmitWrong {
    pub status_code: u8,
    pub lang: String,
    pub run_success: bool,
    pub status_runtime: String,
    pub memory: u64,
    pub question_id: String,
    pub elapsed_time: u64,
    pub compare_result: String,
    pub code_output: String,
    pub std_output: String,
    pub last_testcase: String,
    pub expected_output: String,
    pub task_finish_time: u64,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub pretty_lang: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
    pub input: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitRuntimeError {
    pub status_code: u8,
    pub lang: String,
    pub run_success: bool,
    pub runtime_error: String,
    pub full_runtime_error: String,
    pub memory: u64,
    pub question_id: String,
    pub elapsed_time: u64,
    pub compare_result: String,
    pub code_output: String,
    pub std_output: String,
    pub last_testcase: String,
    pub expected_output: String,
    pub task_finish_time: u64,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub pretty_lang: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitCompileError {
    pub status_code: u8,
    pub lang: String,
    pub run_success: bool,
    pub compile_error: String,
    pub full_compile_error: String,
    pub memory: u64,
    pub question_id: String,
    pub elapsed_time: u64,
    pub compare_result: String,
    pub code_output: String,
    pub std_output: String,
    pub last_testcase: String,
    pub expected_output: String,
    pub task_finish_time: u64,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub pretty_lang: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
}

impl std::fmt::Display for SubmitCorrect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "\n{seperator}\n{}\n{seperator}\nStatus   : {}\nLanguage : {}\nRuntime  : {}\t( beats {:.2}% )\nMemory   : {}\t( beats {:.2}% )\n",
            "Submission Correct!".green().bold(),
            self.status_msg, self.lang, self.status_runtime.cyan(),self.runtime_percentile, self.status_memory.cyan(), self.memory_percentile
        )
    }
}

impl std::fmt::Display for SubmitLimitExceeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "\n{seperator}\n{}\n{seperator}\nStatus : {}\nTestcase {}/{} failed\n\nResult interpretation :\n{}\n",
            "Submission Wrong!".red().bold(),
            self.status_msg,
             format!("{}",self.total_correct).green(),
              format!("{}",self.total_testcases).green(), 
              self.compare_result
        )
    }
}

impl std::fmt::Display for SubmitWrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "\n{seperator}\n{}\n{seperator}\nStatus : {}\nTestcase {}/{} failed\n\nTestcase failed :\n{}\n\nExpected Output :\n{}\nYour Output :\n{}\n",
            "Submission Wrong!".red().bold(),
            self.status_msg,
            format!("{}",self.total_correct).green(),
            format!("{}",self.total_testcases).green(),
            self.last_testcase.cyan(),
            self.expected_output.cyan(),
            self.code_output.cyan(),
        )
    }
}

impl SubmitWrong {
    pub fn display(&self) {
        println!("{}", self);
    }
}

impl SubmitLimitExceeded {
    pub fn display(&self) {
        println!("{}", self);
    }
}

impl SubmitCorrect {
    pub fn display(&self) {
        println!("{}", self);
    }
}
