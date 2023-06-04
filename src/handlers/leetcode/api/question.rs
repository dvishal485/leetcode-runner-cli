use crate::handlers::leetcode::*;

use eyre::{bail, Context, Result};

impl LeetCode<Authorized> {
    pub fn get_daily_challenge(&self) -> Result<DailyChallenge> {
        let url = "https://leetcode.com/graphql";
        let client = &self.client;
        let query = GraphqlRequest {
            query: "\n query questionOfToday {\n  activeDailyCodingChallengeQuestion {\n date\n userStatus\n link\n question {\n   acRate\n   difficulty\n   freqBar\n   frontendQuestionId: questionFrontendId\n   isFavor\n   paidOnly: isPaidOnly\n   status\n   title\n   titleSlug\n   hasVideoSolution\n   hasSolution\n   topicTags {\n  name\n  id\n  slug\n   }\n }\n  }\n}\n ".to_string(),
            variables: "{}".to_string(),
        };

        let data = client.post(url).json(&query).send()?;

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
            .json::<Wrapper>()?
            .data
            .activeDailyCodingChallengeQuestion)
    }

    pub fn get_metadata(&self) -> Result<UserMetadata> {
        let client = &self.client;
        let data = client
            .get("https://leetcode.com/api/problems/all/")
            .send()
            .wrap_err("Failed to fetch metadata from LeetCode")?;

        let metadata = data
            .json::<UserMetadata>()
            .wrap_err("Failed to parse metadata, Try renewing cookie")?;
        if metadata.user_name.is_empty() {
            bail!("Cookie invalid. Renew cookies");
        }
        Ok(metadata)
    }

    pub fn question_content(&self, title_slug: &str) -> Result<LeetcodeQuestion> {
        let client = &self.client;
        let url = "https://leetcode.com/graphql";
        let query = GraphqlRequest {
            query:  "query questionContent($titleSlug: String!) { question(titleSlug: $titleSlug) { content mysqlSchemas }}".to_string(),
            variables: serde_json::to_string(&Variables { titleSlug: title_slug.to_string() }).unwrap(),
        };

        let data = client.post(url).json(&query).send()?;

        #[derive(Deserialize)]
        struct QuestionWrapper {
            question: LeetcodeQuestion,
        }
        #[derive(Deserialize)]
        struct Data {
            data: QuestionWrapper,
        }

        let query = "\n query questionEditorData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n questionId\n questionFrontendId\n codeSnippets {\n   lang\n   langSlug\n   code\n }\n envInfo\n enableRunCode\n  }\n}\n ";
        let varibales = serde_json::to_string(&Variables {
            titleSlug: title_slug.to_string(),
        })?;
        let boiler_code = client
            .post(url)
            .json(&GraphqlRequest {
                query: query.to_string(),
                variables: varibales,
            })
            .send()?;

        #[derive(Debug, Deserialize)]
        #[allow(non_snake_case)]
        struct CodeSnippets {
            codeSnippets: Vec<BoilerPlateCode>,
        }
        #[derive(Debug, Deserialize)]
        struct WrapperData {
            question: CodeSnippets,
        }
        #[derive(Debug, Deserialize)]
        struct Wrapper {
            data: WrapperData,
        }

        let boiler_code_vector = boiler_code.json::<Wrapper>()?.data.question.codeSnippets;

        let mut boiler_code_vector = boiler_code_vector
            .into_iter()
            .filter(|code| code.is_supported())
            .collect::<Vec<_>>();

        // ask user to specify language among these options without using external lib
        let boiler_code = match boiler_code_vector.len() {
            0 => bail!("No boiler plate code available in supported language!"),
            1 => boiler_code_vector.swap_remove(0),
            _ => {
                let mut input = String::new();
                println!("\nPlease select a language from the following options :");
                for (i, code) in boiler_code_vector.iter().enumerate() {
                    println!("{}: {}", i, code.langSlug);
                }
                println!(
                    "\nFor example : Input \"{}\" for {}",
                    0, &boiler_code_vector[0].langSlug
                );
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim().parse::<usize>()?;
                boiler_code_vector.swap_remove(input)
            }
        };

        let mut input = String::new();
        println!("Filename (main.{}) : ", &(boiler_code.extension()));
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        let filename = if input.is_empty() {
            format!("main.{}", boiler_code.extension())
        } else {
            input.to_string()
        };
        boiler_code.save_code(&filename, &title_slug);

        Ok(data.json::<Data>().map(|op| op.data.question)?)
    }

    pub fn question_metadata(&self, title_slug: &str) -> Result<Question> {
        let client = &self.client;
        let url = "https://leetcode.com/graphql";

        let query = GraphqlRequest {
            query: "\n query consolePanelConfig($titleSlug: String!) {\n question(titleSlug: $titleSlug) {\n questionId\n questionFrontendId\n questionTitle\n enableDebugger\n enableRunCode\n enableSubmit\n enableTestMode\n exampleTestcaseList\n metaData\n }\n}\n".to_string(),
            variables: serde_json::to_string(&Variables { titleSlug: title_slug.to_string() }).unwrap(),
        };
        let data = client
            .post(url)
            .json(&query)
            .send()
            .wrap_err("Failed to fetch question id from LeetCode")?;

        #[derive(Debug, Deserialize)]
        struct QuestionWrapper {
            question: Question,
        }

        #[derive(Debug, Deserialize)]
        struct Data {
            data: QuestionWrapper,
        }

        data.json::<Data>()
            .wrap_err("Failed to parse question id from LeetCode")
            .map(|opt| opt.data.question)
    }
}
