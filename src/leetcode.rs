use crate::codefile::CodeFile;
use colored::Colorize;
use serde::{Deserialize, Serialize};

pub struct Authorized;
pub struct Unauthorized;

pub struct LeetCode<State = Unauthorized> {
    state: std::marker::PhantomData<State>,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Question {
    pub questionId: String,
    pub questionTitle: String,
    pub exampleTestcaseList: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitCorrect {
    pub submission_id: String,
    pub lang: String,
    pub question_id: String,
    pub status_code: u8,
    pub run_success: bool,
    pub status_msg: String,
    pub compare_result: String,
    pub state: String,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub status_runtime: String,
    pub status_memory: String,
    pub runtime_percentile: f64,
    pub memory_percentile: f64,
}

#[derive(Debug, Deserialize)]
pub struct SubmitLimitExceeded {
    pub submission_id: String,
    pub lang: String,
    pub question_id: String,
    pub status_code: u8,
    pub run_success: bool,
    pub status_msg: String,
    pub compare_result: String,
    pub state: String,
    pub total_correct: u8,
    pub total_testcases: u8,
}

#[derive(Debug, Deserialize)]
pub struct SubmitWrong {
    pub status_code: u8,
    pub lang: String,
    pub run_success: bool,
    pub status_runtime: String,
    pub memory: u64,
    pub question_id: String,
    pub elapsed_time: u64,
    pub compare_result: String,
    pub code_output: String,
    pub std_output: String,
    pub last_testcase: String,
    pub expected_output: String,
    pub task_finish_time: u64,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub pretty_lang: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
    pub input: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitRuntimeError {
    pub status_code: u8,
    pub lang: String,
    pub run_success: bool,
    pub runtime_error: String,
    pub full_runtime_error: String,
    pub memory: u64,
    pub question_id: String,
    pub elapsed_time: u64,
    pub compare_result: String,
    pub code_output: String,
    pub std_output: String,
    pub last_testcase: String,
    pub expected_output: String,
    pub task_finish_time: u64,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub pretty_lang: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitCompileError {
    pub status_code: u8,
    pub lang: String,
    pub run_success: bool,
    pub compile_error: String,
    pub full_compile_error: String,
    pub memory: u64,
    pub question_id: String,
    pub elapsed_time: u64,
    pub compare_result: String,
    pub code_output: String,
    pub std_output: String,
    pub last_testcase: String,
    pub expected_output: String,
    pub task_finish_time: u64,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub pretty_lang: String,
    pub submission_id: String,
    pub status_msg: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]

pub enum SubmissionResult {
    Success(SubmitCorrect),
    CompileError(SubmitCompileError),
    RuntimeError(SubmitRuntimeError),
    Wrong(SubmitWrong),
    LimitExceeded(SubmitLimitExceeded),
    PendingResult(PendingResult),
    Unknown(Unknown),
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Variables {
    titleSlug: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LimitExceeded {
    status_code: u8,
    lang: String,
    run_success: bool,
    pub status_runtime: String,
    pub memory: u64,
    pub code_answer: Vec<String>,
    pub code_output: Vec<String>,
    pub std_output: Vec<String>,
    pub elapsed_time: u64,
    pub task_finish_time: u64,
    pub total_correct: Option<u8>,
    pub total_testcases: Option<u8>,
    pub status_memory: String,
    pretty_lang: String,
    submission_id: String,
    pub status_msg: String,
    pub state: String,
}
#[derive(Debug, Deserialize)]
pub struct UserMetadata {
    pub user_name: String,
    pub num_total: u16,
    pub ac_easy: u16,
    pub ac_medium: u16,
    pub ac_hard: u16,
}

#[derive(Debug, Serialize)]
struct QuestionIdQuery {
    query: String,
    variables: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Success {
    status_code: u8,
    lang: String,
    run_success: bool,
    pub status_runtime: String,
    pub memory: u64,
    pub code_answer: Vec<String>,
    pub code_output: Vec<String>,
    pub std_output: Vec<String>,
    pub elapsed_time: u64,
    task_finish_time: u64,
    expected_status_code: u8,
    expected_lang: String,
    expected_run_success: bool,
    pub expected_status_runtime: String,
    pub expected_memory: u64,
    pub expected_code_answer: Vec<String>,
    pub expected_code_output: Vec<String>,
    pub expected_std_output: Vec<String>,
    pub expected_elapsed_time: u64,
    expected_task_finish_time: u64,
    correct_answer: bool,
    pub compare_result: String,
    pub total_correct: u8,
    pub total_testcases: u8,
    pub status_memory: String,
    pretty_lang: String,
    submission_id: String,
    pub status_msg: String,
    state: String,
}

#[derive(Debug, Deserialize)]
pub struct CompileError {
    pub compile_error: String,
    pub full_compile_error: String,
    pub std_output: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeError {
    pub runtime_error: String,
    pub full_runtime_error: String,
    pub std_output: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PendingResult {
    state: String,
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ExecutionResult {
    Success(Success),
    CompileError(CompileError),
    RuntimeError(RuntimeError),
    LimitExceeded(LimitExceeded),
    PendingResult(PendingResult),
    Unknown(Unknown),
}

#[derive(Debug, Deserialize)]
pub struct Unknown {}

#[derive(Debug, Serialize)]
struct TestCaseExec {
    lang: String,
    question_id: String,
    question_title: String,
    typed_code: String,
    data_input: String,
}

#[derive(Debug, Serialize)]
struct SubmitCode {
    lang: String,
    question_id: String,
    typed_code: String,
}

#[derive(Debug, Deserialize)]
struct InterpretID {
    interpret_id: String,
}
#[derive(Debug)]
pub enum PendingState {
    Pending,
    Started,
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct LeetcodeQuestion {
    pub content: String,
}

impl UserMetadata {
    pub fn display(&self) {
        println!(
            "User name        :\t{}\nQuestions Solved :\n\t- Easy   :\t{}\n\t- Medium :\t{}\n\t- Hard   :\t{}",
            self.user_name, self.ac_easy, self.ac_medium, self.ac_hard
        );
    }
}

impl PendingResult {
    pub fn state(&self) -> PendingState {
        match self.state.as_str() {
            "PENDING" => PendingState::Pending,
            "STARTED" => PendingState::Started,
            unknown_state => {
                println!(
                    "Unknown state : {}\nKindly inform about this to the developer",
                    unknown_state
                );
                PendingState::Unknown
            }
        }
    }
}

impl std::fmt::Display for SubmitCorrect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "\n{seperator}\n{}\n{seperator}\nStatus   : {}\nLanguage : {}\nRuntime  : {}\t( beats {:.2}% )\nMemory   : {}\t( beats {:.2}% )\n",
            "Submission Correct!".green().bold(),
            self.status_msg, self.lang, self.status_runtime.cyan(),self.runtime_percentile, self.status_memory.cyan(), self.memory_percentile
        )
    }
}

impl std::fmt::Display for SubmitLimitExceeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "\n{seperator}\n{}\n{seperator}\nStatus : {}\nTestcase {}/{} failed\n\nResult interpretation :\n{}\n",
            "Submission Wrong!".red().bold(),
            self.status_msg,
             format!("{}",self.total_correct).green(),
              format!("{}",self.total_testcases).green(), 
              self.compare_result
        )
    }
}

impl std::fmt::Display for SubmitWrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seperator = "-------------------------------";
        write!(
            f,
            "\n{seperator}\n{}\n{seperator}\nStatus : {}\nTestcase {}/{} failed\n\nTestcase failed :\n{}\n",
            "Submission Wrong!".red().bold(),
            self.status_msg,
             format!("{}",self.total_correct).green(),
              format!("{}",self.total_testcases).green(), 
              self.last_testcase.cyan()
        )
    }
}

impl SubmitWrong {
    pub fn display(&self) {
        println!("{}", self);
    }
}

impl SubmitLimitExceeded {
    pub fn display(&self) {
        println!("{}", self);
    }
}

impl SubmitCorrect {
    pub fn display(&self) {
        println!("{}", self);
    }
}

impl Success {
    pub fn is_correct(&self) -> bool {
        self.correct_answer
    }
    pub fn display(&self) {
        let seperator = "-------------------------------";

        println!(
            "\n{}\n\nOutput   : {:?}\nExpected : {:?}\n",
            if self.correct_answer {
                "Testcase execution success".green().bold()
            } else {
                format!(
                    "Testcase {}/{} testcase passed",
                    self.total_correct, self.total_testcases
                )
                .red()
                .bold()
            },
            self.code_answer,
            self.expected_code_answer
        );

        for i in 0..self.code_answer.len() {
            let is_correct = self.code_answer[i] == self.expected_code_answer[i];
            println!(
                "{}\n{}\n{}\nOutput   : {:?}\nExpected : {:?}\n{}",
                seperator.yellow(),
                if is_correct {
                    format!("Testcase {} execution success", i + 1).green()
                } else {
                    format!("Testcase {} execution failed", i + 1).red()
                },
                seperator.yellow(),
                self.code_answer[i],
                self.expected_code_answer[i],
                if !self.std_output[i].is_empty() {
                    format!("\nStd Output :\n{}\n", self.std_output[i])
                } else {
                    format!("")
                }
            );
        }

        println!(
            "{}\nRuntime  : {}\nOutput   : {}\nExpected : {}\n",
            seperator.yellow(),
            self.status_runtime.cyan(),
            self.elapsed_time,
            self.expected_status_runtime
        );

        println!(
            "Memory   : {}\nOutput   : {}\nExpected : {}\n",
            self.status_memory.cyan(),
            self.memory,
            self.expected_memory
        );
    }
}

impl LeetCode {
    pub fn new() -> LeetCode<Unauthorized> {
        LeetCode {
            state: std::marker::PhantomData::<Unauthorized>,
            client: Default::default(),
        }
    }
}

impl LeetCode<Unauthorized> {
    /// # Authenticate with cookie
    /// Builds a new reqwest client with the cookie
    pub fn authenticate(&mut self, cookie: &str) -> Result<LeetCode<Authorized>, String> {
        let mut headers = reqwest::header::HeaderMap::with_capacity(5);
        let Some(csrf_token) = cookie
            .split(';')
            .find(|s| s.contains("csrftoken"))
            else{ Err("No csrf token found".to_string())? };
        let Some(csrf_token) = csrf_token.split('=').last() else{ Err("No csrf token found".to_string())? };
        let csrf_token = csrf_token.to_string();
        headers.insert(
            reqwest::header::COOKIE,
            reqwest::header::HeaderValue::from_str(&cookie).unwrap(),
        );
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36").unwrap(),
        );
        headers.insert(
            reqwest::header::REFERER,
            reqwest::header::HeaderValue::from_str("https://leetcode.com/").unwrap(),
        );
        headers.insert(
            reqwest::header::HeaderName::from_static("x-csrftoken"),
            reqwest::header::HeaderValue::from_str(csrf_token.as_str()).unwrap(),
        );
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers.clone())
            .build()
            .unwrap();
        Ok(LeetCode {
            state: std::marker::PhantomData::<Authorized>,
            client,
        })
    }
}

impl LeetCode<Authorized> {
    pub fn get_metadata(&self) -> Result<UserMetadata, String> {
        let client = &self.client;
        let Ok(data) = client
            .get("https://leetcode.com/api/problems/all/")
            .send() else {
                return Err("Failed to fetch metadata from leetcode".to_string());
            };

        let metadata = data
            .json::<UserMetadata>()
            .map_err(|_| "Failed to parse metadata, try renewing cookie".to_string());
        if let Ok(metadata) = metadata.as_ref() {
            if metadata.user_name == "" {
                return Err(String::from("Cookie invalid. Renew cookies"));
            }
        }
        metadata
    }

    pub fn question_content(&self, title_slug: &str) -> Result<LeetcodeQuestion, String> {
        let client = &self.client;
        let url = "https://leetcode.com/graphql";
        let query = QuestionIdQuery {
            query:  "query questionContent($titleSlug: String!) { question(titleSlug: $titleSlug) { content    mysqlSchemas }}".to_string(),
            variables: serde_json::to_string(&Variables { titleSlug: title_slug.to_string() }).unwrap(),
        };
        let Ok(data) = client.post(url).json(&query).send() else {
                return Err("Failed to fetch question id from leetcode".to_string());
            };
        #[derive(Deserialize)]
        struct QuestionWrapper {
            question: LeetcodeQuestion,
        }

        #[derive(Deserialize)]
        struct Data {
            data: QuestionWrapper,
        }

        data.json::<Data>()
            .map_err(|_| "Failed to parse question content".to_string())
            .map(|op| op.data.question)
    }

    pub fn question_metadata(&self, title_slug: &str) -> Result<Question, String> {
        let client = &self.client;
        let url = "https://leetcode.com/graphql";

        let query = QuestionIdQuery {
            query: "\n query consolePanelConfig($titleSlug: String!) {\n question(titleSlug: $titleSlug) {\n questionId\n questionFrontendId\n questionTitle\n enableDebugger\n enableRunCode\n enableSubmit\n enableTestMode\n exampleTestcaseList\n metaData\n }\n}\n".to_string(),
            variables: serde_json::to_string(&Variables { titleSlug: title_slug.to_string() }).unwrap(),
        };
        let Ok(data) = client.post(url).json(&query).send() else {
                return Err("Failed to fetch question id from leetcode".to_string());
            };

        #[derive(Debug, Deserialize)]
        struct QuestionWrapper {
            question: Question,
        }

        #[derive(Debug, Deserialize)]
        struct Data {
            data: QuestionWrapper,
        }

        data.json::<Data>()
            .map_err(|_| "Failed to parse question id from leetcode".to_string())
            .map(|opt| opt.data.question)
    }
    pub fn execute_default(&self, codefile: &CodeFile) -> Result<ExecutionResult, String> {
        self.execute(codefile, String::new())
    }
    pub fn execute(
        &self,
        codefile: &CodeFile,
        mut data_input: String,
    ) -> Result<ExecutionResult, String> {
        // let mut question_title;
        // let mut question_id;
        // if let Ok(config) = std::fs::File::open(".lc_config") {
        //     let config: Question = serde_json::from_reader(config)
        //         .expect("Corrupted config file! Remove .lc_config and try again");
        //     question_title = config.questionTitle;
        //     question_id = config.questionId;
        // } else {
        //     question_title = String::new();
        //     question_id = String::new();
        // }
        // if codefile.question_title != question_title {
        let question_title = codefile.question_title.clone();
        let ques = self.question_metadata(&question_title)?;
        if data_input == "" {
            data_input = ques.exampleTestcaseList.join("\n");
        }
        let question_id = ques.questionId;
        // save to config file
        // let config = serde_json::to_string(&ques).unwrap();
        // std::fs::write(".lc_config", config).unwrap();
        // }
        self._execute(
            codefile.language.to_string(),
            question_id,
            question_title,
            codefile.code.clone(),
            data_input,
        )
    }

    fn _execute(
        &self,
        lang: String,
        question_id: String,
        question_title: String,
        typed_code: String,
        data_input: String,
    ) -> Result<ExecutionResult, String> {
        let client = &self.client;
        let url = format!(
            "https://leetcode.com/problems/{}/interpret_solution/",
            question_title
        );
        let testcase = TestCaseExec {
            lang,
            question_id,
            question_title,
            typed_code,
            data_input,
        };
        let Ok(data)= client.post(&url).json(&testcase).send() else {
                return Err("Failed to parse arguments".to_string());
            };
        let Ok(data) = data.json::<InterpretID>() else{
                return Err("Failed to parse JSON from leetcode. Try again after sometime or renew cookie".to_string());
            };

        let interpret_id = data.interpret_id;
        println!("Executing testcases...");
        let mut last_state = 0;
        loop {
            let url = format!("https://leetcode.com/submissions/detail/{interpret_id}/check/");
            // std::thread::sleep(std::time::Duration::from_secs(7));
            let Ok(data) = client.get(&url).send() else {
                return Err("Failed to parse arguments".to_string());
            };

            let Ok(data) = data.json::<ExecutionResult>() else  {
                    return Err("Failed to parse JSON from leetcode. Try again after sometime or renew cookie".to_string());
              };
            match data {
                ExecutionResult::PendingResult(data) => {
                    last_state = match data.state.as_str() {
                        "PENDING" => {
                            if last_state == 0 {
                                println!("Status : Pending");
                            }
                            1
                        }
                        "STARTED" => {
                            if last_state == 1 {
                                println!("Status : Execution Started");
                            }
                            2
                        }
                        _ => {
                            if last_state == 2 {
                                println!(
                                    "Status : {}\nKindly report this state to developer",
                                    data.state.as_str()
                                );
                            }
                            3
                        }
                    };
                    continue;
                }
                data => return Ok(data),
            };
        }
    }
    pub fn submit(&self, codefile: &CodeFile) -> Result<SubmissionResult, String> {
        let question_title = codefile.question_title.clone();
        let ques = self.question_metadata(&question_title)?;
        let question_id = ques.questionId;
        self._submit(
            codefile.language.to_string(),
            question_id,
            question_title,
            codefile.code.clone(),
        )
    }

    fn _submit(
        &self,
        lang: String,
        question_id: String,
        question_title: String,
        typed_code: String,
    ) -> Result<SubmissionResult, String> {
        let client = &self.client;
        let url = format!("https://leetcode.com/problems/{}/submit/", question_title);
        let submission = SubmitCode {
            lang,
            question_id,
            typed_code,
        };
        let Ok(data)= client.post(&url).json(&submission).send() else {
                return Err("Failed to parse arguments".to_string());
            };
        #[derive(Debug, Deserialize)]
        struct SubmissionID {
            submission_id: u32,
        }
        // println!("{}", data.text().unwrap());
        let Ok(data) = data.json::<SubmissionID>() else {
                return Err("Failed to fetch submission id from leetcode. Check your submissions manually on leetcode".to_string());
            };
        println!("Evaluating solution...");
        let submission_id = data.submission_id;
        let mut last_state = 0;

        loop {
            let url = format!("https://leetcode.com/submissions/detail/{submission_id}/check/");
            let Ok(data) = client.get(&url).send() else {
                return Err("Failed to parse arguments".to_string());
            };

            let Ok(data) = data.json::<SubmissionResult>() else  {
                    return Err("Failed to fetch from leetcode. Try again after sometime or renew cookie".to_string());
              };
            match data {
                SubmissionResult::PendingResult(data) => {
                    last_state = match data.state.as_str() {
                        "PENDING" => {
                            if last_state == 0 {
                                println!("Status : Evalutaion Pending");
                            }
                            1
                        }
                        "STARTED" => {
                            if last_state == 1 {
                                println!("Status : Execution Started");
                            }
                            2
                        }
                        _ => {
                            if last_state == 2 {
                                println!(
                                    "Status : {}\nKindly report this state to developer",
                                    data.state.as_str()
                                );
                            }
                            3
                        }
                    };
                    continue;
                }
                data => return Ok(data),
            };
        }
    }
}
