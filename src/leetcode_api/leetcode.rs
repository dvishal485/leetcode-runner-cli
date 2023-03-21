use super::helpers::*;
use super::user::*;
use super::worker::*;
use crate::file_parser::codefile::CodeFile;

use serde::Deserialize;

pub struct Authorized;
pub struct Unauthorized;

pub struct LeetCode<State = Unauthorized> {
    state: std::marker::PhantomData<State>,
    client: reqwest::blocking::Client,
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
    pub fn get_daily_challenge(&self) -> Result<DailyChallenge, String> {
        let url = "https://leetcode.com/graphql";
        let client = &self.client;
        let query = GraphqlRequest {
   query: "\n query questionOfToday {\n  activeDailyCodingChallengeQuestion {\n date\n userStatus\n link\n question {\n   acRate\n   difficulty\n   freqBar\n   frontendQuestionId: questionFrontendId\n   isFavor\n   paidOnly: isPaidOnly\n   status\n   title\n   titleSlug\n   hasVideoSolution\n   hasSolution\n   topicTags {\n  name\n  id\n  slug\n   }\n }\n  }\n}\n ".to_string(),
   variables: "{}".to_string(),
  };
        let Ok(data) = client.post(url).json(&query).send() else {
 return Err("Failed to fetch daily challenge from leetcode".to_string());
   };
        // println!("{:?}", data.text());
        // todo!();
        #[derive(Deserialize)]
        #[allow(non_snake_case)]
        struct DailyChallengeWrapper {
            activeDailyCodingChallengeQuestion: DailyChallenge,
        }
        #[derive(Deserialize)]
        struct Wrapper {
            data: DailyChallengeWrapper,
        }
        Ok(data
            .json::<Wrapper>()
            .map_err(|_| "Failed to parse daily challenge!".to_string())?
            .data
            .activeDailyCodingChallengeQuestion)
    }

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
        let query = GraphqlRequest {
   query:  "query questionContent($titleSlug: String!) { question(titleSlug: $titleSlug) { content mysqlSchemas }}".to_string(),
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

        let query = GraphqlRequest {
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
        let question_title = codefile.question_title.clone();
        let ques = self.question_metadata(&question_title)?;
        if data_input == "" {
            data_input = ques.exampleTestcaseList.join("\n");
        }
        let question_id = ques.questionId;
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
