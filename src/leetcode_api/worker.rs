use super::execution::*;
use super::submission::*;
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

#[derive(Debug)]
pub enum PendingState {
    Pending,
    Started,
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
            unknown_state => {
                println!(
                    "Unknown state : {}\nKindly inform about this to the developer",
                    unknown_state
                );
                PendingState::Unknown
            }
        }
    }
}
