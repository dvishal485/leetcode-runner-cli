use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct TestCaseExec {
    pub(crate) lang: String,
    pub(crate) question_id: String,
    pub(crate) question_title: String,
    pub(crate) typed_code: String,
    pub(crate) data_input: String,
}
#[derive(Debug, Serialize)]
pub(crate) struct SubmitCode {
    pub(crate) lang: String,
    pub(crate) question_id: String,
    pub(crate) typed_code: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct InterpretID {
    pub(crate) interpret_id: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct Variables {
    pub(crate) titleSlug: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct GraphqlRequest {
    pub(crate) query: String,
    pub(crate) variables: String,
}

#[derive(Debug, Deserialize)]
pub struct LeetcodeQuestion {
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct DailyChallengeQuestion {
    pub acRate: f64,
    pub difficulty: String,
    pub frontendQuestionId: String,
    pub status: String,
    pub title: String,
    pub titleSlug: String,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "{}", "Easy".bright_green()),
            Difficulty::Medium => write!(f, "{}", "Medium".bright_yellow()),
            Difficulty::Hard => write!(f, "{}", "Hard".bright_red()),
        }
    }
}

impl Difficulty {
    pub fn from_str(difficulty: &str) -> Difficulty {
        match difficulty {
            "Easy" => Difficulty::Easy,
            "Medium" => Difficulty::Medium,
            "Hard" => Difficulty::Hard,
            _ => panic!("Invalid difficulty"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct DailyChallenge {
    pub date: String,
    pub userStatus: String,
    pub link: String,
    pub question: DailyChallengeQuestion,
}

impl std::fmt::Display for DailyChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title      : {}\nDifficulty : {}\nDate       : {}\nStatus     : {}\nAC Rate    : {:.2}%",
            self.question.title.bright_cyan(),
            Difficulty::from_str(&self.question.difficulty),
            self.date,
            self.userStatus,
            self.question.acRate
        )
    }
}
