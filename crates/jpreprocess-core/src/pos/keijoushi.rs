use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 形状詞
pub enum Keijoushi {
    /// タリ
    Tari,
    /// 一般
    General,
    /// 助動詞語幹
    JodoushiGokan,
}

impl FromStr for Keijoushi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "タリ" => Ok(Self::Tari),
            "一般" => Ok(Self::General),
            "助動詞語幹" => Ok(Self::JodoushiGokan),
            _ => Err(POSParseError::new(1, s.to_string(), POSKind::Keijoushi)),
        }
    }
}

impl Display for Keijoushi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::Tari => "タリ",
                Self::General => "一般",
                Self::JodoushiGokan => "助動詞語幹",
            },
        )
    }
}
