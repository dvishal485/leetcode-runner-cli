use crate::handlers::leetcode::*;

impl LeetCode<Authorized> {
    pub fn submit(&self, codefile: &CodeFile) -> Result<SubmissionResult, &str> {
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

    pub(crate) fn _submit(
        &self,
        lang: String,
        question_id: String,
        question_title: String,
        typed_code: String,
    ) -> Result<SubmissionResult, &str> {
        let client = &self.client;
        let url = format!("https://leetcode.com/problems/{}/submit/", question_title);
        let submission = SubmitCode {
            lang,
            question_id,
            typed_code,
        };
        let Ok(data)= client.post(&url).json(&submission).send() else {
 return Err("Failed to parse arguments");
   };
        #[derive(Debug, Deserialize)]
        struct SubmissionID {
            submission_id: u32,
        }
        // println!("{}", data.text().unwrap());
        let Ok(data) = data.json::<SubmissionID>() else {
 return Err("Failed to fetch submission id from leetcode! Check your submissions manually on leetcode");
   };
        println!("Evaluating solution...");
        let submission_id = data.submission_id;
        let mut last_state = PendingState::Unknown;

        loop {
            let url = format!("https://leetcode.com/submissions/detail/{submission_id}/check/");
            let Ok(data) = client.get(&url).send() else {
 return Err("Failed to parse arguments!");
   };

            let Ok(data) = data.json::<SubmissionResult>() else  {
  return Err("Failed to fetch from leetcode! Try again after sometime or renew cookie");
  };
            match data {
                SubmissionResult::PendingResult(data) => {
                    let curr_state = data.state();
                    match curr_state {
                        PendingState::Pending => {
                            if last_state != PendingState::Pending {
                                println!("Status : Evalutaion Pending");
                            }
                        }
                        PendingState::Started => {
                            if last_state != PendingState::Started {
                                println!("Status : Execution Started");
                            }
                        }
                        PendingState::Success => {
                            println!("Your code was executed successfully but we failed to parse result\nCheck on leetcode manually");
                            std::process::exit(1);
                        }
                        PendingState::Unknown => {
                            println!(
                                "Status : {}\nKindly report this state to developer",
                                data.state.as_str()
                            );
                            std::process::exit(1);
                        }
                    };
                    last_state = curr_state;
                    continue;
                }
                data => return Ok(data),
            };
        }
    }
}
