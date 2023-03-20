use std::io::{Read, Write};

pub enum Language {
    Rust,
    Python3,
    Cpp,
    Java,
    C,
    Javascript,
    Go,
    Kotlin,
    Swift,
    Typescript,
}
impl Language {
    pub fn from_str(input: &str) -> Option<Language> {
        match input {
            "rs" => Some(Language::Rust),
            "py" => Some(Language::Python3),
            "cpp" => Some(Language::Cpp),
            "java" => Some(Language::Java),
            "c" => Some(Language::C),
            "js" => Some(Language::Javascript),
            "go" => Some(Language::Go),
            "kt" => Some(Language::Kotlin),
            "swift" => Some(Language::Swift),
            "ts" => Some(Language::Typescript),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Language::Rust => "rust",
            Language::Python3 => "python3",
            Language::Cpp => "cpp",
            Language::Java => "java",
            Language::C => "c",
            Language::Javascript => "javascript",
            Language::Go => "golang",
            Language::Kotlin => "kotlin",
            Language::Swift => "swift",
            Language::Typescript => "typescript",
        }
    }
    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

pub struct CodeFile {
    pub language: Language,
    pub path: std::path::PathBuf,
    pub question_title: String,
    pub code: String,
}

impl CodeFile {
    pub fn from_dir() -> Self {
        let mut file_to_return = Self {
            language: Language::Rust,
            path: std::path::PathBuf::from("main.rs"),
            question_title: String::new(),
            code: String::new(),
        };
        let mut is_set = false;
        let files = std::fs::read_dir("./").unwrap();
        for file in files {
            if let Ok(file) = file {
                if let Some(file_name) = file.file_name().to_str() {
                    let extension = file_name.split('.').last().unwrap();
                    if let Some(language) = Language::from_str(extension) {
                        is_set = true;
                        if file_name.starts_with("main") {
                            file_to_return = Self {
                                language,
                                path: file.path(),
                                question_title: file_to_return.question_title,
                                code: file_to_return.code,
                            };
                            break;
                        }
                        file_to_return = Self {
                            language,
                            path: file.path(),
                            question_title: file_to_return.question_title,
                            code: file_to_return.code,
                        };
                    }
                }
            }
        }
        if !is_set {
            println!(
                "No code file found. Creating a new file named {}",
                file_to_return.path.display()
            );
            let mut file =
                std::fs::File::create(&file_to_return.path).expect("Error during file creation");
            let two_sum_problem = b"struct Solution;\n\n// https://leetcode.com/problems/two-sum/ LCSTART\n\nimpl Solution {\n\tpub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n\n\t}\n} // LCEND\n";
            file.write_all(two_sum_problem).expect("File write failed");
        }
        let mut file = std::fs::File::open(&file_to_return.path).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).expect(&format!(
            "Failed to read file {}",
            file_to_return.path.display()
        ));
        let start = if let Some(s) = code.find("#LCSTART") {
            s + 8
        } else {
            0
        };
        let end = code.find("#LCEND").unwrap_or(code.len());
        if let Some(problem) = code.find("leetcode.com/problems/") {
            let problem = (&code[problem..]).split_whitespace().next().unwrap();
            let problem = problem.split('/').last().unwrap();
            file_to_return.question_title = problem.to_string();
        } else {
            println!("No leetcode problem found in the code file. Please add the problem link in the code file using comments.");
            // terminate with error
            std::process::exit(1);
        }
        file_to_return.code = code[start..end].to_string();
        file_to_return
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
        let start = if let Some(s) = code.find("#LCSTART") {
            s + 8
        } else {
            0
        };
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
