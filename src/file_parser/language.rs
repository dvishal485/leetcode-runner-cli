use std::{fmt, str::FromStr};

#[derive(Debug, Default, Clone, Copy)]
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

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self).to_lowercase())
    }
}

impl FromStr for Language {
    type Err = eyre::ErrReport;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "rs" | "rust" => Ok(Language::Rust),
            "py" | "python" | "python3" => Ok(Language::Python3),
            "cpp" => Ok(Language::Cpp),
            "java" => Ok(Language::Java),
            "c" => Ok(Language::C),
            "js" | "javascript" => Ok(Language::Javascript),
            "go" | "golang" => Ok(Language::Go),
            "kt" | "kotlin" => Ok(Language::Kotlin),
            "swift" => Ok(Language::Swift),
            "ts" | "typescript" => Ok(Language::Typescript),
            "cs" | "csharp" => Ok(Language::Csharp),
            "rb" | "ruby" => Ok(Language::Ruby),
            "scala" => Ok(Language::Scala),
            "php" => Ok(Language::PHP),
            "rkt" | "racket" => Ok(Language::Racket),
            "erl" | "erlang" => Ok(Language::Erlang),
            "ex" | "elixer" => Ok(Language::Elixir),
            "dart" => Ok(Language::Dart),
            _ => Err(eyre::eyre!("Unknown language")),
        }
    }
}

impl Language {
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
