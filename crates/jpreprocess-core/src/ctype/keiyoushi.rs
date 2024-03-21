use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 形容詞
pub enum Keiyoushi {
    /// アウオ段
    Auo,
    /// イ段
    I,
    /// イイ
    Ii,
    /// 形容詞
    KeiyoushiType,
}

impl FromStr for Keiyoushi {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::KeiyoushiType),
            "アウオ段" => Ok(Self::Auo),
            "イ段" => Ok(Self::I),
            "イイ" => Ok(Self::Ii),
            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::Keiyoushi)),
        }
    }
}

impl Display for Keiyoushi {
    fn fmt(&self, s: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        s.write_str(match &self {
            Self::KeiyoushiType => "",
            Self::Auo => "アウオ段",
            Self::I => "イ段",
            Self::Ii => "イイ",
        })
    }
}
