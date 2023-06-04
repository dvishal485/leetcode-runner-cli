use std::fmt;

use super::execution::*;
use super::submission::*;
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct PendingResult {
    pub(crate) state: String,
}

#[derive(Debug, Deserialize)]
pub struct Unknown {}

impl fmt::Display for SubmissionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubmissionResult::Success(inner) => write!(f, "{}", inner),
            SubmissionResult::CompileError(inner) => write!(f, "{}", inner),
            SubmissionResult::RuntimeError(inner) => write!(f, "{}", inner),
            SubmissionResult::Wrong(inner) => write!(f, "{}", inner),
            SubmissionResult::LimitExceeded(inner) => write!(f, "{}", inner),
            SubmissionResult::PendingResult(inner) => write!(f, "{}", inner),
            SubmissionResult::Unknown(inner) => write!(f, "Unknown"),
        }
    }
}

impl PendingResult {
    pub fn state(&self) -> PendingState {
        match self.state.as_str() {
            "PENDING" => PendingState::Pending,
            "STARTED" => PendingState::Started,
            "SUCCESS" => PendingState::Success,
            other => {
                println!(
                    "Unknown state : {}\nKindly inform about this to the developer",
                    other.cyan().bold()
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
