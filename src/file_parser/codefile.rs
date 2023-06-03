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
    pub fn from_dir() -> Self {
        let mut code_file: Option<CodeFile> = None;
        let Ok(files) = std::fs::read_dir("./") else {
            eprintln!("Error reading the current directory!");
            std::process::exit(1);
        };
        for file in files {
            let Ok(file) = file else {
                // Bad path
                continue
            };
            let path = file.path();
            let Some(valid_file) = Self::is_valid_file(&path) else {continue};
            let file_name = valid_file.0;
            code_file = Some(valid_file.1);

            if file_name.starts_with("main") {
                break;
            }
        }
        let mut code_file = code_file.unwrap_or_else(|| {
            eprintln!("No code file found! Try creating a file named with proper extension",);
            std::process::exit(1);
        });
        let Ok(code) = std::fs::read_to_string(&code_file.path) else { 
            eprintln!("Error reading the code file!");
            std::process::exit(1);
        };

        let parsed_file = Self::parse_code(&code, code_file.language);
        let Ok((question_title, parsed_code)) = parsed_file else{
            eprintln!("Error parsing the code file!\n{}", parsed_file.unwrap_err());
            std::process::exit(1);
                    };
        code_file.question_title = question_title;
        code_file.code = parsed_code;
        code_file
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

    fn parse_code(code: &str, language: Language) -> Result<(String, String), &str> {
        let question_title: String;
        let parsed_code: String;
        let start = code
            .find("#LCSTART")
            .map(|idx| idx + code[idx..].find('\n').unwrap_or(0))
            // This returning None means the user
            // wants to submit a practically empty file,
            // but hey we don't judge!
            .unwrap_or(0);

        let end = code.find("#LCEND").unwrap_or(code.len());
        if let Some(problem) = code.find("leetcode.com/problems/") {
            let problem = (&code[problem..]).split_whitespace().next().unwrap();
            let problem = problem.split('/').skip(2).next().unwrap();
            question_title = problem.to_string();
        } else {
            return Err("No leetcode problem found in the code file. Please add the problem link in the code file using comments.");
        }
        let code = code[start..end].trim();
        let code = code.trim_end_matches(language.inline_comment_start());
        parsed_code = code.to_string();

        Ok((question_title, parsed_code))
    }

    pub fn from_file(path: &str) -> Self {
        let path = PathBuf::from(&path);
        let Some(file) =  Self::is_valid_file(&path) else { 
            eprintln!("Invalid file!");
            std::process::exit(1);
        };
        let (_, mut valid_file) = file;
        let Ok(code) = std::fs::read_to_string(&path) else {
            eprintln!("Error while reading file {}!", path.display());
            std::process::exit(1);
        };
        let parsed_file = Self::parse_code(&code, valid_file.language);
        let Ok((question_title, parsed_code)) = parsed_file else{
            eprintln!("Error parsing the code file!\n{}", parsed_file.unwrap_err());
            std::process::exit(1);
        };
        valid_file.question_title = question_title;
        valid_file.code = parsed_code;
        valid_file
    }
}
