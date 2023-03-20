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
pub(crate) struct QuestionIdQuery {
    pub(crate) query: String,
    pub(crate) variables: String,
}

#[derive(Debug, Deserialize)]
pub struct LeetcodeQuestion {
    pub content: String,
}
