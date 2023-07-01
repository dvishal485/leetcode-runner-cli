use crate::handlers::leetcode::*;

use colored::Colorize;
use eyre::{bail, Context, Result};

const GRAPHQL_URL: &str = "https://leetcode.com/graphql";

impl LeetCode<Authorized> {
    pub fn get_daily_challenge(&self) -> Result<DailyChallenge> {
        let client = &self.client;
        let query = GraphqlRequest {
            query: r#"
            query questionOfToday {
                activeDailyCodingChallengeQuestion {
                    date
                    userStatus
                    link
                    question {
                        acRate
                        difficulty
                        freqBar
                        frontendQuestionId: questionFrontendId
                        isFavor
                        paidOnly: isPaidOnly
                        status
                        title
                        titleSlug
                        hasVideoSolution
                        hasSolution
                        topicTags {
                            name
                            id
                            slug
                        }
                    }
                }
            }
            "#
            .to_string(),
            variables: "{}".to_string(),
        };

        let data = client.post(GRAPHQL_URL).json(&query).send()?;

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
        let query = GraphqlRequest {
            query: r#"
            query questionContent($titleSlug: String!) {
                question(titleSlug: $titleSlug) {
                    content mysqlSchemas
                }
            }
            "#
            .to_string(),
            variables: serde_json::to_string(&Variables {
                titleSlug: title_slug.to_string(),
            })?,
        };

        let data = client.post(GRAPHQL_URL).json(&query).send()?;

        #[derive(Deserialize)]
        struct QuestionWrapper {
            question: LeetcodeQuestion,
        }
        #[derive(Deserialize)]
        struct Data {
            data: QuestionWrapper,
        }

        Ok(data.json::<Data>().map(|op| op.data.question)?)
    }
    pub fn save_boiler_code(&self, title_slug: &str) -> Result<()> {
        let client = &self.client;
        let query = r#"
            query questionEditorData($titleSlug: String!) {
                question(titleSlug: $titleSlug) {
                    questionId
                        questionFrontendId
                        codeSnippets {
                            lang
                                langSlug
                                code
                        }
                    envInfo
                        enableRunCode
                }
            }
        "#;
        let variables = serde_json::to_string(&Variables {
            titleSlug: title_slug.to_string(),
        })?;
        let boiler_code = client
            .post(GRAPHQL_URL)
            .json(&GraphqlRequest {
                query: query.to_string(),
                variables,
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

        // ask user to specify language among these options
        let boiler_code = match boiler_code_vector.len() {
            0 => bail!("No boiler plate code available in supported language!"),
            1 => boiler_code_vector.swap_remove(0),
            _ => {
                let mut input = String::new();
                println!(
                    "{}",
                    "\nPlease select a language from the following options :".yellow()
                );
                for (i, code) in boiler_code_vector.iter().enumerate() {
                    println!("{}: {}", i, code.langSlug);
                }
                println!(
                    "\nFor example : Input \"{}\" for {}",
                    "0".cyan(),
                    &boiler_code_vector[0].langSlug.cyan()
                );
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim().parse::<usize>()?;
                boiler_code_vector.swap_remove(input)
            }
        };

        let mut input = String::new();
        println!("Filename (main.{}) : ", &(boiler_code.extension()?));
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        let filename = if input.is_empty() {
            format!("main.{}", boiler_code.extension()?)
        } else {
            input.to_string()
        };
        boiler_code.save_code(&filename, &title_slug)?;
        Ok(())
    }

    pub fn question_metadata(&self, title_slug: &str) -> Result<Question> {
        let client = &self.client;

        let query = GraphqlRequest {
            query: r#"
            query consolePanelConfig($titleSlug: String!) {
                question(titleSlug: $titleSlug) {
                    questionId
                    questionFrontendId
                    questionTitle
                    enableDebugger
                    enableRunCode
                    enableSubmit
                    enableTestMode
                    exampleTestcaseList
                    metaData
                }
            }
            "#
            .to_string(),
            variables: serde_json::to_string(&Variables {
                titleSlug: title_slug.to_string(),
            })?,
        };
        let data = client
            .post(GRAPHQL_URL)
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
