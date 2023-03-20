use super::language::*;
use std::{
    io::{Read, Write},
    path::PathBuf,
};

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
        let files = std::fs::read_dir("./").unwrap();
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
            let default_code_file: CodeFile = Default::default();
            println!(
                "No code file found. Creating a new file named {}",
                default_code_file.path.display()
            );
            let mut file =
                std::fs::File::create(&default_code_file.path).expect("Error during file creation");
            let two_sum_problem = b"struct Solution;\n\n// https://leetcode.com/problems/two-sum/ #LCSTART\n\nimpl Solution {\n\tpub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n\n\t}\n} // #LCEND\nfn main() {}";
            file.write_all(two_sum_problem).expect("File write failed");

            default_code_file
        });
        let mut file = std::fs::File::open(&code_file.path).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code)
            .expect(&format!("Failed to read file {}", code_file.path.display()));
        let parsed_file = Self::parse_code(&code);
        let Ok((question_title, parsed_code)) = parsed_file else{
            eprintln!("{}", parsed_file.err().unwrap());
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

    fn parse_code(code: &str) -> Result<(String, String), String> {
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
            return Err("No leetcode problem found in the code file. Please add the problem link in the code file using comments.".to_string());
        }
        parsed_code = code[start..end].to_string();

        Ok((question_title, parsed_code))
    }

    pub fn from_file(path: String) -> Self {
        let path = PathBuf::from(path);
        let (_, mut valid_file) =
            Self::is_valid_file(&path).expect("Improper filename or the language is not supported");
        let file = std::fs::File::open(&path);
        let Ok(mut file) = file else {
            eprintln!("Error while opening file {}", path.display());
            std::process::exit(1);
        };
        let mut code = String::new();
        file.read_to_string(&mut code)
            .expect(&format!("Failed to read file {}", path.display()));
        let parsed_file = Self::parse_code(&code);
        let Ok((question_title, parsed_code)) = parsed_file else{
            eprintln!("{}", parsed_file.err().unwrap());
            std::process::exit(1);
        };
        valid_file.question_title = question_title;
        valid_file.code = parsed_code;
        valid_file
    }
}
