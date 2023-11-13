use async_graphql::Enum;
use serde::{Deserialize, Serialize};

pub mod guard;
pub mod scalars;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Deserialize, Serialize)]
pub enum ImageKind {
    Profile,
    Image,
}

impl From<String> for ImageKind {
    fn from(s: String) -> Self {
        match s.as_str() {
            "profile" => Self::Profile,
            "imagery" => Self::Image,
            _ => Self::Image,
        }
    }
}

impl ToString for ImageKind {
    fn to_string(&self) -> String {
        match self {
            Self::Profile => "profile".to_string(),
            Self::Image => "imagery".to_string(),
        }
    }
}
