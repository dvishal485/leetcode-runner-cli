use super::helpers::*;
use super::user::*;
use super::utils::*;
use crate::file_parser::codefile::CodeFile;

use eyre::Result;
use serde::Deserialize;

mod api;

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
    pub fn authenticate(&self, cookie: &str) -> Result<LeetCode<Authorized>> {
        let mut headers = reqwest::header::HeaderMap::with_capacity(5);
        let csrf_token = cookie
            .split(';')
            .find(|s| s.contains("csrftoken"))
            .map(|s| s.split('=').last())
            .flatten()
            .ok_or_else(|| eyre::eyre!("No csrf token found"))?;

        headers.insert(
            reqwest::header::COOKIE,
            reqwest::header::HeaderValue::from_str(&cookie)?,
        );
        headers.insert(
                reqwest::header::USER_AGENT,
                reqwest::header::HeaderValue::from_str("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")?,
        );
        headers.insert(
            reqwest::header::REFERER,
            reqwest::header::HeaderValue::from_str("https://leetcode.com/")?,
        );
        headers.insert(
            reqwest::header::HeaderName::from_static("x-csrftoken"),
            reqwest::header::HeaderValue::from_str(csrf_token)?,
        );
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers.clone())
            .build()?;
        Ok(LeetCode {
            state: std::marker::PhantomData::<Authorized>,
            client,
        })
    }
}

impl LeetCode<Authorized> {}
