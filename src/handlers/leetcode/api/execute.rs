use crate::handlers::{leetcode::*, utils::ExecutionResult};

use eyre::{bail, Context, Result};

impl LeetCode<Authorized> {
    pub fn execute_default(&self, codefile: &CodeFile) -> Result<ExecutionResult> {
        self.execute(codefile, String::new())
    }
    pub fn execute(&self, codefile: &CodeFile, mut data_input: String) -> Result<ExecutionResult> {
        let question_title = codefile.question_title.clone();
        let ques = self.question_metadata(&question_title)?;
        if data_input.is_empty() {
            data_input = ques.exampleTestcaseList.join("\n");

            // write this to testcase.txt
            let mut file = std::fs::File::create("testcase.txt")?;
            std::io::Write::write_all(&mut file, data_input.as_bytes())?;
            println!("Wrote default testcases to testcase.txt");
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

    pub(crate) fn _execute(
        &self,
        lang: String,
        question_id: String,
        question_title: String,
        typed_code: String,
        data_input: String,
    ) -> Result<ExecutionResult> {
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

        let data = client
            .post(&url)
            .json(&testcase)
            .send()?
            .json::<InterpretID>()
            .wrap_err(
                "Failed to parse JSON from LeetCode, Try again after sometime or renew your cookie",
            )?;

        let interpret_id = data.interpret_id;

        println!("Executing testcases...");
        let mut last_state = PendingState::Unknown;
        loop {
            let url = format!("https://leetcode.com/submissions/detail/{interpret_id}/check/");
            // std::thread::sleep(std::time::Duration::from_secs(7));
            let data = client
                .get(&url)
                .send()?
                .json::<ExecutionResult>()
                .wrap_err(
                "Failed to parse JSON from LeetCode, Try again after sometime or renew your cookie",
            )?;

            match data {
                ExecutionResult::PendingResult(data) => {
                    let curr_state = data.state();
                    match curr_state {
                        PendingState::Pending => {
                            if last_state != PendingState::Pending {
                                println!("Status : Evaluation Pending");
                            }
                        }
                        PendingState::Started => {
                            if last_state != PendingState::Started {
                                println!("Status : Execution Started");
                            }
                        }
                        PendingState::Success => {
                            bail!("our code was executed successfully but we failed to parse result\nCheck on leetcode manually");
                        }
                        PendingState::Unknown => {
                            bail!(
                                "Status : {}\nKindly report this state to developer",
                                data.state.as_str()
                            );
                        }
                    };
                    last_state = curr_state;
                }
                not_pending_data => return Ok(not_pending_data),
            };
        }
    }
}
