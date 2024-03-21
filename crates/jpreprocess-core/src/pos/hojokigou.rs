use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 記号
pub enum Hojokigou {
    /// \*
    None,
    /// 一般
    General,
    /// 句点
    Kuten,
    /// 括弧開
    KakkoOpen,
    /// 括弧閉
    KakkoClose,
    /// 読点
    Touten,
    /// アスキーアート
    AA,

}

impl FromStr for Hojokigou {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::None),
            "一般" => Ok(Self::General),
            "括弧開" => Ok(Self::KakkoOpen),
            "括弧閉" => Ok(Self::KakkoClose),
            "句点" => Ok(Self::Kuten),
            "読点" => Ok(Self::Touten),
            "ＡＡ" => Ok(Self::AA),
            _ => Err(POSParseError::new(1, s.to_string(), POSKind::Hukukigou)),
        }
    }
}

impl Display for Hojokigou {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::None => "*",
                Self::General => "一般",
                Self::KakkoOpen => "括弧開",
                Self::KakkoClose => "括弧閉",
                Self::Kuten => "句点",
                Self::Touten => "読点",
                Self::AA => "ＡＡ",
            },
        )
    }
}


#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 地名
pub enum AA {
    /// 一般
    General,
    /// 顔文字
    FaceString,
}

impl FromStr for AA {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "顔文字" => Ok(Self::FaceString),

            _ => Err(POSParseError::new(3, s.to_string(), POSKind::AA)),
        }
    }
}
