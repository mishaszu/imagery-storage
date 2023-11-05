pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FileSystem(String),

    ReqwestFailed(reqwest::Error),

    LustResponse(String),
    LustBadStatus(u16),
    LustParsingBytesError(String),

    UrlParseFailed(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestFailed(e)
    }
}

impl Into<async_graphql::Error> for Error {
    fn into(self) -> async_graphql::Error {
        match self {
            Error::FileSystem(s) => async_graphql::Error::new(format!("File system error: {}", s)),
            Error::ReqwestFailed(e) => async_graphql::Error::new(format!("Reqwest error: {}", e)),
            Error::UrlParseFailed(s) => {
                async_graphql::Error::new(format!("Url parse error: {}", s))
            }
            Error::LustResponse(s) => {
                async_graphql::Error::new(format!("Lust response error: {}", s))
            }
            Error::LustParsingBytesError(s) => {
                async_graphql::Error::new(format!("Lust parsing bytes error: {}", s))
            }
            Error::LustBadStatus(s) => {
                async_graphql::Error::new(format!("Lust bad status error: {}", s))
            }
        }
    }
}
