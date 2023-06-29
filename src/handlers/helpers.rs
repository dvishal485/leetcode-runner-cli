use std::{fmt, path::Path, str::FromStr};

use colored::Colorize;
use eyre::Result;
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
    pub title: String,
    pub titleSlug: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl FromStr for Difficulty {
    type Err = eyre::ErrReport;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            _ => Err(eyre::eyre!("Unknown difficulty")),
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

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct BoilerPlateCode {
    pub(crate) code: String,
    pub(crate) langSlug: String,
}

use super::super::file_parser::language::Language;
impl BoilerPlateCode {
    pub(crate) fn save_code<P: AsRef<Path>>(&self, file_path: P, title_slug: &str) -> Result<()> {
        let language = Language::from_str(&self.langSlug)?;
        let mut file = std::fs::File::create(file_path)?;
        let comment = format!(
            " {} #LCEND https://leetcode.com/problems/{}/",
            language.inline_comment_start(),
            title_slug.to_lowercase().trim().replace(" ", "-")
        );

        // write code into file along with the comment
        std::io::Write::write_all(&mut file, self.code.as_bytes())?;
        std::io::Write::write_all(&mut file, comment.as_bytes())?;

        Ok(())
    }
    pub(crate) fn is_supported(&self) -> bool {
        Language::from_str(&self.langSlug).is_ok()
    }
    pub(crate) fn extension(&self) -> Result<String> {
        let language = Language::from_str(&self.langSlug)?;
        Ok(language.extension().to_owned())
    }
}

impl fmt::Display for DailyChallenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{title_txt:12}: {}\n{diff:12}: {}\n{date:12}: {}\n{status:12}: {}\n{ac_rate:12}: {:.2}%\n{link:12}: https://leetcode.com{}",
            self.question.title.bright_cyan(),
            Difficulty::from_str(&self.question.difficulty).map_err(|_| fmt::Error)?,
            self.date,
            self.userStatus,
            self.question.acRate,
            self.link,
            title_txt = "Title",
            diff = "Difficulty",
            ac_rate = "AC Rate",
            date = "Date",
            status = "Status",
            link = "Link",
        )
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "{}", "Easy".bright_green()),
            Difficulty::Medium => write!(f, "{}", "Medium".bright_yellow()),
            Difficulty::Hard => write!(f, "{}", "Hard".bright_red()),
        }
    }
}
