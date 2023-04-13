use super::execution::*;
use super::submission::*;
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SubmissionResult {
    Success(SubmitCorrect),
    CompileError(SubmitCompileError),
    RuntimeError(SubmitRuntimeError),
    Wrong(SubmitWrong),
    LimitExceeded(SubmitLimitExceeded),
    PendingResult(PendingResult),
    Unknown(Unknown),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ExecutionResult {
    Success(Success),
    CompileError(CompileError),
    RuntimeError(RuntimeError),
    LimitExceeded(LimitExceeded),
    PendingResult(PendingResult),
    Unknown(Unknown),
}

#[derive(Debug, PartialEq)]
pub enum PendingState {
    Pending,
    Started,
    Success,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Question {
    pub questionId: String,
    pub questionTitle: String,
    pub exampleTestcaseList: Vec<String>,
}

#[derive(Deserialize)]
pub struct PendingResult {
    pub(crate) state: String,
}

#[derive(Deserialize)]
pub struct Unknown {}

impl PendingResult {
    pub fn state(&self) -> PendingState {
        match self.state.as_str() {
            "PENDING" => PendingState::Pending,
            "STARTED" => PendingState::Started,
            "SUCCESS" => PendingState::Success,
            unknown_state => {
                println!(
                    "Unknown state : {}\nKindly inform about this to the developer",
                    unknown_state.cyan().bold()
                );
                PendingState::Unknown
            }
        }
    }
}

impl std::fmt::Display for PendingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\nState : {:?}",
            "Pending Result!".yellow().bold(),
            self.state()
        )
    }
}
