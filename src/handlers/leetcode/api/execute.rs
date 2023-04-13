use crate::handlers::{leetcode::*, utils::ExecutionResult};

impl LeetCode<Authorized> {
    pub fn execute_default(&self, codefile: &CodeFile) -> Result<ExecutionResult, &str> {
        self.execute(codefile, String::new())
    }
    pub fn execute(
        &self,
        codefile: &CodeFile,
        mut data_input: String,
    ) -> Result<ExecutionResult, &str> {
        let question_title = codefile.question_title.clone();
        let ques = self.question_metadata(&question_title)?;
        if data_input == "" {
            data_input = ques.exampleTestcaseList.join("\n");
            // write this to testcase.txt
            if let Ok(mut file) = std::fs::File::create("testcase.txt") {
                if let Ok(_) = std::io::Write::write_all(&mut file, data_input.as_bytes()) {
                    println!("Wrote default testcases to testcase.txt");
                } else {
                    eprintln!("Failed to write default testcases to testcase.txt!");
                }
            } else {
                eprintln!("Failed to create testcase.txt!");
            }
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
    ) -> Result<ExecutionResult, &str> {
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
 return Err("Failed to parse arguments!");
   };
        let Ok(data) = data.json::<InterpretID>() else{
 return Err("Failed to parse JSON from leetcode! Try again after sometime or renew cookie");
   };

        let interpret_id = data.interpret_id;
        println!("Executing testcases...");
        let mut last_state = PendingState::Unknown;
        loop {
            let url = format!("https://leetcode.com/submissions/detail/{interpret_id}/check/");
            // std::thread::sleep(std::time::Duration::from_secs(7));
            let Ok(data) = client.get(&url).send() else {
 return Err("Failed to parse arguments!");
   };

            let Ok(data) = data.json::<ExecutionResult>() else  {
  return Err("Failed to parse JSON from leetcode! Try again after sometime or renew cookie");
  };
            match data {
                ExecutionResult::PendingResult(data) => {
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
