#[derive(Default)]
pub enum Language {
    #[default]
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
