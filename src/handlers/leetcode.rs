use super::helpers::*;
use super::user::*;
use super::utils::*;
use crate::file_parser::codefile::CodeFile;

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
    pub fn authenticate(&self, cookie: &str) -> Result<LeetCode<Authorized>, &str> {
        let mut headers = reqwest::header::HeaderMap::with_capacity(5);
        let Some(csrf_token) = cookie
   .split(';')
   .find(|s| s.contains("csrftoken"))
   else{return  Err("No csrf token found"); };
        let Some(csrf_token) = csrf_token.split('=').last() else{return Err("No csrf token found"); };
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

impl LeetCode<Authorized> {}
