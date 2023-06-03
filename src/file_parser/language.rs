#[derive(Default, Clone, Copy)]
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
    Csharp,
    Ruby,
    Scala,
    PHP,
    Racket,
    Erlang,
    Elixir,
    Dart,
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
            "cs" => Some(Language::Csharp),
            "rb" => Some(Language::Ruby),
            "scala" => Some(Language::Scala),
            "php" => Some(Language::PHP),
            "rkt" => Some(Language::Racket),
            "erl" => Some(Language::Erlang),
            "ex" => Some(Language::Elixir),
            "dart" => Some(Language::Dart),
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
            Language::Csharp => "csharp",
            Language::Ruby => "ruby",
            Language::Scala => "scala",
            Language::PHP => "php",
            Language::Racket => "racket",
            Language::Erlang => "erlang",
            Language::Elixir => "elixir",
            Language::Dart => "dart",
        }
    }
    pub(crate) fn from_slug(input: &str) -> Option<Language> {
        match input.to_lowercase().as_str() {
            "rust" => Some(Language::Rust),
            "python3" => Some(Language::Python3),
            "cpp" => Some(Language::Cpp),
            "java" => Some(Language::Java),
            "c" => Some(Language::C),
            "javascript" => Some(Language::Javascript),
            "golang" => Some(Language::Go),
            "kotlin" => Some(Language::Kotlin),
            "swift" => Some(Language::Swift),
            "typescript" => Some(Language::Typescript),
            "csharp" => Some(Language::Csharp),
            "ruby" => Some(Language::Ruby),
            "scala" => Some(Language::Scala),
            "php" => Some(Language::PHP),
            "racket" => Some(Language::Racket),
            "erlang" => Some(Language::Erlang),
            "elixir" => Some(Language::Elixir),
            "dart" => Some(Language::Dart),
            _ => None,
        }
    }
    pub fn extension(&self) -> &str {
        match self {
            Language::Rust => "rs",
            Language::Python3 => "py",
            Language::Cpp => "cpp",
            Language::Java => "java",
            Language::C => "c",
            Language::Javascript => "js",
            Language::Go => "go",
            Language::Kotlin => "kt",
            Language::Swift => "swift",
            Language::Typescript => "ts",
            Language::Csharp => "cs",
            Language::Ruby => "rb",
            Language::Scala => "scala",
            Language::PHP => "php",
            Language::Racket => "rkt",
            Language::Erlang => "erl",
            Language::Elixir => "ex",
            Language::Dart => "dart",
        }
    }
    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl Language {
    pub(crate) fn inline_comment_start(&self) -> &str {
        use Language::*;
        match self {
            Rust | Cpp | C | Csharp | Javascript | Typescript | Kotlin | Java | Go | Scala
            | Swift | Dart => "//",
            Python3 | Ruby | PHP | Elixir => "#",
            Racket => ";",
            Erlang => "%",
        }
    }
}
