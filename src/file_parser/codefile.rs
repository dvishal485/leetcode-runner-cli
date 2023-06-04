use eyre::Result;

use super::language::*;
use std::path::PathBuf;

pub struct CodeFile {
    pub language: Language,
    pub path: std::path::PathBuf,
    pub question_title: String,
    pub code: String,
}

impl Default for CodeFile {
    fn default() -> Self {
        Self {
            language: Default::default(),
            path: PathBuf::from("main.rs"),
            question_title: Default::default(),
            code: Default::default(),
        }
    }
}

impl CodeFile {
    pub fn from_file(path: &str) -> Result<Self> {
        let path = PathBuf::from(&path);
        let (_file_name, mut code_file) =
            Self::is_valid_file(&path).ok_or_else(|| eyre::eyre!("Invalid file"))?;
        let code = std::fs::read_to_string(&path)?;

        let (question_title, parsed_code) = Self::parse_code(&code, code_file.language)?;

        code_file.question_title = question_title;
        code_file.code = parsed_code;

        Ok(code_file)
    }

    pub fn from_dir() -> Result<Self> {
        let mut code_file: Option<CodeFile> = None;
        for file in std::fs::read_dir(".")?.filter_map(|f| f.ok()) {
            let path = file.path();
            if let Some((file_name, code_file_)) = Self::is_valid_file(&path) {
                code_file = Some(code_file_);
                if file_name.starts_with("main") {
                    break;
                }
            }
        }
        let mut code_file = code_file.ok_or_else(|| {
            eyre::eyre!("No code file found! Try creating a file named with proper extension")
        })?;
        let code = std::fs::read_to_string(&code_file.path)?;

        let (question_title, parsed_code) = Self::parse_code(&code, code_file.language)?;

        code_file.question_title = question_title;
        code_file.code = parsed_code;

        Ok(code_file)
    }

    fn is_valid_file<'a>(path: &'a std::path::PathBuf) -> Option<(&'a str, Self)> {
        let file_name = path.file_name().and_then(|filename| filename.to_str())?;
        let extension = path.extension().and_then(|ext| ext.to_str())?;
        let language = Language::from_str(extension)?;

        Some((
            file_name,
            CodeFile {
                language,
                path: path.clone(),
                question_title: String::new(),
                code: String::new(),
            },
        ))
    }

    fn parse_code(code: &str, language: Language) -> Result<(String, String)> {
        let start = code
            .find("#LCSTART")
            .map(|idx| idx + code[idx..].find('\n').unwrap_or(0))
            // This returning None means the user
            // wants to submit a practically empty file,
            // but hey we don't judge!
            .unwrap_or(0);

        let end = code.find("#LCEND").unwrap_or(code.len());
        let question_title = code[code.find("leetcode.com/problems/").ok_or_else(|| {
            eyre::eyre!(
                "No leetcode problem found in the code file. \
        Please add the problem link in the code file using comments."
            )
        })?..]
            .split_whitespace()
            .next()
            .expect("Should be Some since the find method succeed")
            .split('/')
            .skip(2)
            .next()
            .ok_or_else(|| eyre::eyre!("Invalid link, expected question identifier"))?
            .to_string();
        let parsed_code = code[start..end]
            .trim()
            .trim_end_matches(language.inline_comment_start())
            .trim_end()
            .to_string();

        Ok((question_title, parsed_code))
    }
}
