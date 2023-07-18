use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Clip {
    pub id: String,
    pub content: String,
    pub timestamp: String,
    pub starred: bool,
}

#[derive(Debug, Clone)]
pub enum Error {
    APIError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}
