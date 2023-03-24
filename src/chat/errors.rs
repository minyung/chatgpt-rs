use reqwest::header::InvalidHeaderValue;

#[derive(Debug)]
pub enum Error {
    ChatGpt(String),
    Reqwest(reqwest::Error),
    HttpInvalidHeaderValue(InvalidHeaderValue)
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Error::HttpInvalidHeaderValue(value)
    }
}
