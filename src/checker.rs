use reqwest::StatusCode;
use reqwest::Url;
use std::error::Error;
use std::fmt;

pub enum TestResult {
    Ok,
    Redirect(Url),
    NotFound,
    Error(UrlTestError),
}

pub fn test_url(url: &str) -> TestResult {
    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => {
            return TestResult::Error(UrlTestError {
                url: url.to_string(),
                problem: "Failed to make request".to_string(),
            })
        }
    };

    if resp.status() == StatusCode::NOT_FOUND {
        return TestResult::NotFound;
    }

    if resp.url().as_str() != url {
        return TestResult::Redirect(resp.url().clone());
    }

    TestResult::Ok
}

#[derive(Debug)]
pub struct UrlTestError {
    url: String,
    problem: String,
}

impl fmt::Display for UrlTestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Could not reach url '{}'. Problem: {}",
            self.url, self.problem
        )
    }
}

impl Error for UrlTestError {}
