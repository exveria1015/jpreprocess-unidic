use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 接頭詞
pub enum Setsubizi {

    /// 動詞的
    Doushiteki,
    /// 名詞的
    Meishiteki(Meishiteki),
    /// 形容詞的
    Keiyoushiteki,
    /// 形状詞的
    Keijoushiteki,

    None

}

impl FromStr for Setsubizi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "動詞的" => Ok(Self::Doushiteki),
            "名詞的" => Ok(Self::Meishiteki(Meishiteki::General)),
            "形容詞的" => Ok(Self::Keiyoushiteki),
            "形状詞的" => Ok(Self::Keijoushiteki),
            _ => Err(POSParseError::new(1, s.to_string(), POSKind::Setsubizi)),
        }
    }
}


impl Display for Setsubizi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var_name = match &self {
            Self::Meishiteki(meishiteki) => format!("名詞的,{}", meishiteki),
            Self::Keiyoushiteki => "形容詞的,*,*".to_string(),
            Self::Keijoushiteki => "形状詞的,*,*".to_string(),
            Self::Doushiteki => "動詞的,*,*".to_string(),
            // Self::SahenSetsuzoku => "サ変接続,*,*".to_string(),
            // Self::NaiKeiyoushiGokan => "ナイ形容詞語幹,*,*".to_string(),
            // Self::General => "一般,*,*".to_string(),
            // Self::QuoteStr => "引用文字列,*,*".to_string(),
            // Self::KeiyoudoushiGokan => "形容動詞語幹,*,*".to_string(),
            // Self::Setsuzokushiteki => "接続詞的,*,*".to_string(),
            // Self::Setsubi(setsubi) => format!("接尾,{}", setsubi),
            // Self::Daimeishi(daimeishi) => format!("代名詞,{}", daimeishi),
            // Self::DoushiHijiritsuteki => "動詞非自立的,*,*".to_string(),
            // Self::Special => "特殊,*,*".to_string(),
            // Self::Hijiritsu(meishi_hijiritsu) => format!("非自立,{}", meishi_hijiritsu),
            // Self::FukushiKanou => "副詞可能,*,*".to_string(),

            Self::None => "*".to_string(),
        };
        f.write_str(&var_name)
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞的
pub enum Meishiteki {
    /// サ変可能
    SahenKanou,
    /// 一般
    General,
    /// 副詞可能
    FukushiKanou,
    /// 助数詞
    Josuushi,

}

impl FromStr for Meishiteki {
    type Err = POSParseError;
    fn from_str(g3: &str) -> Result<Self, Self::Err> {
        match g3 {
            "サ変可能" => Ok(Self::SahenKanou),
            "一般" => Ok(Self::General),
            "副詞可能" => Ok(Self::FukushiKanou),
            "助数詞" => Ok(Self::Josuushi),
            _ => Err(POSParseError::new(3, g3.to_string(), POSKind::Meishiteki)),
        }
    }
}




impl Display for Meishiteki {
    fn fmt(&self, g3: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        g3.write_str(match &self {
            Self::General => "一般,*",
            Self::SahenKanou => "サ変可能,*",
            Self::FukushiKanou => "副詞可能,*",
            Self::Josuushi => "助数詞,*",

        })
    }
}
