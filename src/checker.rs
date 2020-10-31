use reqwest::StatusCode;
use std::error::Error;
use std::fmt;
use reqwest::Url;

pub enum TestResult {
    Ok,
    Redirect(Url),
    NotFound,
}

pub fn test_url(url: &str) -> Result<TestResult, UrlTestError> {
    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => return Err(UrlTestError{url: url.to_string(), problem: "Failed to make request".to_string()}),
    };

    if resp.status() == StatusCode::NOT_FOUND {
        return Ok(TestResult::NotFound);
    }

    if resp.url().as_str() != url {
        return Ok(TestResult::Redirect(resp.url().clone()))
    }

    Ok(TestResult::Ok)
}

#[derive(Debug)]
pub struct UrlTestError {
    url: String,
    problem: String,
}

impl fmt::Display for UrlTestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not reach url '{}'. Problem: {}", self.url, self.problem)
    }
}

impl Error for UrlTestError {}