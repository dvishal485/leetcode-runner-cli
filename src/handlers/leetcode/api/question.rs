use crate::handlers::leetcode::*;

impl LeetCode<Authorized> {
    pub fn get_daily_challenge(&self) -> Result<DailyChallenge, &str> {
        let url = "https://leetcode.com/graphql";
        let client = &self.client;
        let query = GraphqlRequest {
            query: "\n query questionOfToday {\n  activeDailyCodingChallengeQuestion {\n date\n userStatus\n link\n question {\n   acRate\n   difficulty\n   freqBar\n   frontendQuestionId: questionFrontendId\n   isFavor\n   paidOnly: isPaidOnly\n   status\n   title\n   titleSlug\n   hasVideoSolution\n   hasSolution\n   topicTags {\n  name\n  id\n  slug\n   }\n }\n  }\n}\n ".to_string(),
            variables: "{}".to_string(),
        };
        let Ok(data) = client.post(url).json(&query).send() else {
            return Err("Failed to fetch daily challenge from leetcode!");
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
            .map_err(|_| "Failed to parse daily challenge!")?
            .data
            .activeDailyCodingChallengeQuestion)
    }

    pub fn get_metadata(&self) -> Result<UserMetadata, &str> {
        let client = &self.client;
        let Ok(data) = client
            .get("https://leetcode.com/api/problems/all/")
            .send() else {
                return Err("Failed to fetch metadata from leetcode!");
            };

        let metadata = data
            .json::<UserMetadata>()
            .map_err(|_| "Failed to parse metadata! Try renewing cookie");
        if let Ok(metadata) = metadata.as_ref() {
            if metadata.user_name == "" {
                return Err("Cookie invalid. Renew cookies");
            }
        }
        metadata
    }

    pub fn question_content(&self, title_slug: &str) -> Result<LeetcodeQuestion, &str> {
        let client = &self.client;
        let url = "https://leetcode.com/graphql";
        let query = GraphqlRequest {
            query:  "query questionContent($titleSlug: String!) { question(titleSlug: $titleSlug) { content mysqlSchemas }}".to_string(),
            variables: serde_json::to_string(&Variables { titleSlug: title_slug.to_string() }).unwrap(),
        };

        let Ok(data) = client.post(url).json(&query).send() else {
            return Err("Failed to fetch question id from leetcode!");
        };
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
        })
        .unwrap();
        let Ok(boiler_code) = client
            .post(url)
            .json(&GraphqlRequest {
                query: query.to_string(),
                variables: varibales,
            })
        .send() else {
            return Err("Failed to fetch boiler plate code!");
        };

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

        let boiler_code_vector = boiler_code
            .json::<Wrapper>()
            .map_err(|_| "Failed to parse boiler plate code!")?
            .data
            .question
            .codeSnippets;

        let boiler_code_vector = boiler_code_vector
            .into_iter()
            .filter(|code| code.is_supported())
            .collect::<Vec<_>>();

        // ask user to specify language among these options without using external lib
        let boiler_code = if boiler_code_vector.len() == 1 {
            boiler_code_vector.into_iter().next().unwrap()
        } else if !boiler_code_vector.is_empty() {
            let mut input = String::new();
            println!("\nPlease select a language from the following options :");
            for (i, code) in boiler_code_vector.iter().enumerate() {
                println!("{}: {}", i, code.langSlug);
            }
            println!(
                "\nFor example : Input \"{}\" for {}",
                0, &boiler_code_vector[0].langSlug
            );
            if let Err(_) = std::io::stdin().read_line(&mut input) {
                return Err("Failed to read input!");
            }
            let input = input.trim();
            let Ok(input) = input.parse::<usize>() else {
                return Err("Invalid input!");
            };
            if let Some(code) = boiler_code_vector.into_iter().nth(input) {
                code
            } else {
                return Err("Invalid input!");
            }
        } else {
            return Err("No boiler plate code available in supported language!");
        };
        let mut input = String::new();
        println!("Filename (main.{}) : ", &(boiler_code.extension()));
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            return Err("Failed to read input!");
        }
        let input = input.trim();
        let filename = if input.is_empty() {
            format!("main.{}", boiler_code.extension())
        } else {
            input.to_string()
        };
        boiler_code.save_code(&filename, &title_slug);

        data.json::<Data>()
            .map_err(|_| "Failed to parse question content!")
            .map(|op| op.data.question)
    }

    pub fn question_metadata(&self, title_slug: &str) -> Result<Question, &str> {
        let client = &self.client;
        let url = "https://leetcode.com/graphql";

        let query = GraphqlRequest {
            query: "\n query consolePanelConfig($titleSlug: String!) {\n question(titleSlug: $titleSlug) {\n questionId\n questionFrontendId\n questionTitle\n enableDebugger\n enableRunCode\n enableSubmit\n enableTestMode\n exampleTestcaseList\n metaData\n }\n}\n".to_string(),
            variables: serde_json::to_string(&Variables { titleSlug: title_slug.to_string() }).unwrap(),
        };
        let Ok(data) = client.post(url).json(&query).send() else {
            return Err("Failed to fetch question id from leetcode!");
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
            .map_err(|_| "Failed to parse question id from leetcode!")
            .map(|opt| opt.data.question)
    }
}
