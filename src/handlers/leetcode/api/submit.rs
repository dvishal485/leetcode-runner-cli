use crate::handlers::leetcode::*;

use eyre::{bail, Context, Result};

impl LeetCode<Authorized> {
    pub fn submit(&self, codefile: &CodeFile) -> Result<SubmissionResult> {
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
    ) -> Result<SubmissionResult> {
        #[derive(Debug, Deserialize)]
        struct SubmissionID {
            submission_id: u32,
        }

        let client = &self.client;
        let url = format!("https://leetcode.com/problems/{}/submit/", question_title);
        let submission = SubmitCode {
            lang,
            question_id,
            typed_code,
        };
        let data = client.post(&url).json(&submission).send()?.json::<SubmissionID>().wrap_err(
            "Failed to fetch submission id from LeetCode, Check your submissions manually on leetcode"
        )?;

        println!("Evaluating solution...");
        let submission_id = data.submission_id;
        let mut last_state = PendingState::Unknown;

        loop {
            let url = format!("https://leetcode.com/submissions/detail/{submission_id}/check/");
            let data = client
                .get(&url)
                .send()?
                .json::<SubmissionResult>()
                .wrap_err(
                    "Failed to fetch from leetcode! Try again after sometime or renew cookie",
                )?;

            match data {
                SubmissionResult::PendingResult(data) => {
                    let curr_state = data.state();
                    match curr_state {
                        PendingState::Pending if last_state != PendingState::Pending => {
                            println!("Status : Evaluation Pending");
                        }
                        PendingState::Started if last_state != PendingState::Started => {
                            println!("Status : Execution Started");
                        }
                        PendingState::Success => {
                            bail!("Your code was executed successfully but we failed to parse result\nCheck on leetcode manually");
                        }
                        PendingState::Unknown => {
                            bail!(
                                "Status : {}\nKindly report this state to developer",
                                data.state
                            );
                        }
                        _ => {}
                    };
                    last_state = curr_state;
                }
                not_pending_data => return Ok(not_pending_data),
            };
        }
    }
}
