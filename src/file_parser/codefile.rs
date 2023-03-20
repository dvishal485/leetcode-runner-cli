use std::io::{Read, Write};
use super::language::*;

pub struct CodeFile {
    pub language: Language,
    pub path: std::path::PathBuf,
    pub question_title: String,
    pub code: String,
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
            let Some(file_name) = path.file_name().and_then(|filename| filename.to_str()) else {
                // Non-UTF-8 name
                continue
            };
            let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
                // A hidden file (like .gitignore), or a file with no '.', or a file with weird non-UTF-8 extension.
                continue
            };
            let Some(language) = Language::from_str(extension) else {
                // Unsupported language
                continue;
            };
            
            code_file = Some(
                CodeFile {
                    language, path: path.clone(), question_title: String::new(), code: String::new() 
                }
            );

            if file_name.starts_with("main") {
                break;
            }
        }
        let mut code_file = code_file.unwrap_or_else(|| {
            let default_code_file = CodeFile {
                language: Language::Rust,
                path: std::path::PathBuf::from("main.rs"),
                question_title: String::new(),
                code: String::new(),
            };
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
        file.read_to_string(&mut code).expect(&format!(
            "Failed to read file {}",
            code_file.path.display()
        ));
        let start = code
            .find("#LCSTART")
            .map(|idx| idx + 
                // This returning None the user
                // wants to submit a practically empty file,
                // but hey we don't judge!
                code[idx..].find('\n').unwrap_or(0))
            .unwrap_or(0);

        let end = code.find("#LCEND").unwrap_or(code.len());
        if let Some(problem) = code.find("leetcode.com/problems/") {
            let problem = (&code[problem..]).split_whitespace().next().unwrap();
            let problem = problem.split('/').skip(2).next().unwrap();
            code_file.question_title = problem.to_string();
        } else {
            println!("No leetcode problem found in the code file. Please add the problem link in the code file using comments.");
            // terminate with error
            std::process::exit(1);
        }
        code_file.code = code[start..end].to_string();
        code_file
    }

    pub fn from_file(path: String) -> Self {
        let extension = path.split('.').last().unwrap();
        let language = Language::from_str(extension).expect("File extension not supported");
        let file = std::fs::File::open(&path);
        let Ok(mut file) = file else{
            println!("Error while opening file {}", &path );
            std::process::exit(1);
        };
        let mut code = String::new();
        file.read_to_string(&mut code)
            .expect(&format!("Failed to read file {}", &path));
        let start = code
            .find("#LCSTART")
            .map(|idx| idx + 
                // This returning None the user
                // wants to submit a practically empty file,
                // but hey we don't judge!
                code[idx..].find('\n').unwrap_or(0))
            .unwrap_or(0);

        let end = code.find("#LCEND").unwrap_or(code.len());
        if let Some(problem) = code.find("leetcode.com/problems/") {
            let problem = (&code[problem..]).split_whitespace().next().unwrap();
            let problem = problem.split('/').into_iter().rev().skip(1).next().unwrap();
            let question_title = problem.to_string();
            Self {
                language,
                path: std::path::PathBuf::from(path),
                question_title,
                code: code[start..end].to_string(),
            }
        } else {
            println!("No leetcode problem found in the code file. Please add the problem link in the code file using comments.");
            // terminate with error
            std::process::exit(1);
        }
    }
}
