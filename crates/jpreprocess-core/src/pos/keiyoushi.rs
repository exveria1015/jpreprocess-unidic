use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 形容詞
pub enum Keiyoushi {
    /// 自立
    Jiritsu,
    /// 接尾
    Setsubi,
    /// 非自立
    Hijiritsu,

    //Unidic 3.1.0
    /// 一般
    General,
    /// 非自立可能
    HijiritsuKanou,
}

impl FromStr for Keiyoushi {
    type Err = POSParseError;
    fn from_str(f: &str) -> Result<Self, Self::Err> {
        match f {
            "自立" => Ok(Self::Jiritsu),
            "接尾" => Ok(Self::Setsubi),
            "非自立" => Ok(Self::Hijiritsu),
            //Unidic 3.1.0
            "一般" => Ok(Self::General),
            "非自立可能" => Ok(Self::HijiritsuKanou),

            _ => Err(POSParseError::new(1, f.to_string(), POSKind::Keiyoushi)),
        }
    }
}

impl Display for Keiyoushi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::Jiritsu => "自立",
                Self::Setsubi => "接尾",
                Self::Hijiritsu => "非自立",
                //Unidic 3.1.0
                Self::General => "一般",
                Self::HijiritsuKanou => "非自立可能",
            },
        )
    }
}
