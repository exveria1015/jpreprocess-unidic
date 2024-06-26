use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 接頭詞
pub enum Kigou {

    /// 一般
    General,
    /// 文字
    Char,
    /// \*
    None,
}

impl FromStr for Kigou {
    type Err = POSParseError;
    fn from_str(f: &str) -> Result<Self, Self::Err> {
        match f {
            "一般" => Ok(Self::General),
            "文字" => Ok(Self::Char),
            "*" => Ok(Self::None),
            _ => Err(POSParseError::new(1, f.to_string(), POSKind::Kigou)),
        }
    }
}

impl Display for Kigou {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::General => "一般",
                Self::Char => "文字",
                Self::None => "*",

            },
        )
    }
}
