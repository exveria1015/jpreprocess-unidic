use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 形状詞
pub enum Kandoushi {
    /// フィラー
    Filler,
    /// 一般
    General,
}

impl FromStr for Kandoushi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "フィラー" => Ok(Self::Filler),
            "一般" => Ok(Self::General),
            _ => Err(POSParseError::new(1, s.to_string(), POSKind::Kandoushi)),
        }
    }
}

impl Display for Kandoushi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::Filler => "フィラー,*",
                Self::General => "一般,*",
            },
        )
    }
}
