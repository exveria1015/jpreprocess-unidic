use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞
pub enum Meishi {
    /// 普通名詞
    FutsuMeishi(FutsuMeishi),
    /// 固有名詞
    KoyuMeishi(KoyuMeishi),
    /// 数詞
    Suushi,
    /// 助動詞語幹
    JodoushiGokan,

    /// \*
    None,
}

impl Meishi {
    pub fn from_strs(g1: &str, g2: &str, g3: &str) -> Result<Self, POSParseError> {
        match g1 {
            "普通名詞" => FutsuMeishi::from_strs(g2).map(Self::FutsuMeishi),
            // "サ変接続" => Ok(Self::SahenSetsuzoku),
            // "ナイ形容詞語幹" => Ok(Self::NaiKeiyoushiGokan),
            // "一般" => Ok(Self::General),
            // "引用文字列" => Ok(Self::QuoteStr),
            // "形容動詞語幹" => Ok(Self::KeiyoudoushiGokan),
            "固有名詞" => KoyuMeishi::from_strs(g2, g3).map(Self::KoyuMeishi),
            "数詞" => Ok(Self::Suushi),
            "助動詞語幹" => Ok(Self::JodoushiGokan),
            // "接続詞的" => Ok(Self::Setsuzokushiteki),
            // "接尾" => Setsubi::from_str(g2).map(Self::Setsubi),
            // "代名詞" => Daimeishi::from_str(g2).map(Self::Daimeishi),
            // "動詞非自立的" => Ok(Self::DoushiHijiritsuteki),
            // "特殊" => Ok(Self::Special),
            // "非自立" => MeishiHijiritsu::from_str(g2).map(Self::Hijiritsu),
            // "副詞可能" => Ok(Self::FukushiKanou),
            "*" => Ok(Self::None),

            _ => Err(POSParseError::new(1, g1.to_string(), POSKind::Meishi)),
        }
    }
}

impl Display for Meishi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var_name = match &self {
            Self::FutsuMeishi(futsumeishi) => format!("普通名詞,{}", futsumeishi),
            // Self::SahenSetsuzoku => "サ変接続,*,*".to_string(),
            // Self::NaiKeiyoushiGokan => "ナイ形容詞語幹,*,*".to_string(),
            // Self::General => "一般,*,*".to_string(),
            // Self::QuoteStr => "引用文字列,*,*".to_string(),
            // Self::KeiyoudoushiGokan => "形容動詞語幹,*,*".to_string(),
            Self::KoyuMeishi(koyumeishi) => format!("固有名詞,{}", koyumeishi),
            Self::Suushi => "数詞,*,*".to_string(),
            Self::JodoushiGokan => "助動詞語幹,*,*".to_string(),
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
/// 普通名詞
pub enum FutsuMeishi {

    /// サ変可能
    SahenKanou,
    /// サ変形状詞可能
    SahenKeijoushiKanou,
    /// 一般
    General,
    /// 副詞可能
    FukushiKanou,
    /// 助数詞可能
    JosuushiKanou,
    /// 形状詞可能
    KeijoushiKanou,
}

impl FutsuMeishi {
    pub fn from_strs(g2: &str) -> Result<Self, POSParseError> {
        match g2 {
            "サ変可能" => Ok(Self::SahenKanou),
            "サ変形状詞可能" => Ok(Self::SahenKeijoushiKanou),
            "一般" => Ok(Self::General),
            "副詞可能" => Ok(Self::FukushiKanou),
            "助数詞可能" => Ok(Self::JosuushiKanou),
            "形状詞可能" => Ok(Self::KeijoushiKanou),

            _ => Err(POSParseError::new(2, g2.to_string(), POSKind::FutsuMeishi)),
        }
    }
}

impl Display for FutsuMeishi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::SahenKanou => "サ変可能,*",
            Self::SahenKeijoushiKanou => "サ変形状詞可能,*",
            Self::General => "一般,*",
            Self::FukushiKanou => "副詞可能,*",
            Self::JosuushiKanou => "助数詞可能,*",
            Self::KeijoushiKanou => "形状詞可能,*",
        })
    }
}




#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 固有名詞
pub enum KoyuMeishi {
    /// 一般
    General,
    /// 人名
    Person(Person),
    /// 地名
    Region(Region),
}

impl KoyuMeishi {
    pub fn from_strs(g2: &str, g3: &str) -> Result<Self, POSParseError> {
        match g2 {
            "一般" => Ok(Self::General),
            "人名" => Person::from_str(g3).map(Self::Person),
            "地名" => Region::from_str(g3).map(Self::Region),

            _ => Err(POSParseError::new(2, g2.to_string(), POSKind::KoyuMeishi)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 人名
pub enum Person {
    /// 一般
    General,
    /// 姓
    Sei,
    /// 名
    Mei,
}

impl FromStr for Person {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "姓" => Ok(Self::Sei),
            "名" => Ok(Self::Mei),

            _ => Err(POSParseError::new(3, s.to_string(), POSKind::Person)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 地名
pub enum Region {
    /// 一般
    General,
    /// 国
    Country,
}

impl FromStr for Region {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "国" => Ok(Self::Country),

            _ => Err(POSParseError::new(3, s.to_string(), POSKind::Region)),
        }
    }
}

impl Display for KoyuMeishi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::General => "一般,*",
            Self::Person(Person::General) => "人名,一般",
            Self::Person(Person::Sei) => "人名,姓",
            Self::Person(Person::Mei) => "人名,名",
            Self::Region(Region::General) => "地名,一般",
            Self::Region(Region::Country) => "地名,国",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞・接尾
pub enum Setsubi {
    /// サ変接続
    SahenSetsuzoku,
    /// 一般
    General,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 助数詞
    Josuushi,
    /// 助動詞語幹
    JodoushiGokan,
    /// 人名
    Person,
    /// 地名
    Region,
    /// 特殊
    Special,
    /// 副詞可能
    FukushiKanou,
}

impl FromStr for Setsubi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "サ変接続" => Ok(Self::SahenSetsuzoku),
            "一般" => Ok(Self::General),
            "形容動詞語幹" => Ok(Self::KeiyoudoushiGokan),
            "助数詞" => Ok(Self::Josuushi),
            "助動詞語幹" => Ok(Self::JodoushiGokan),
            "人名" => Ok(Self::Person),
            "地名" => Ok(Self::Region),
            "特殊" => Ok(Self::Special),
            "副詞可能" => Ok(Self::FukushiKanou),

            _ => Err(POSParseError::new(2, s.to_string(), POSKind::MeishiSetsubi)),
        }
    }
}

impl Display for Setsubi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*",
            match &self {
                Self::SahenSetsuzoku => "サ変接続",
                Self::General => "一般",
                Self::KeiyoudoushiGokan => "形容動詞語幹",
                Self::Josuushi => "助数詞",
                Self::JodoushiGokan => "助動詞語幹",
                Self::Person => "人名",
                Self::Region => "地名",
                Self::Special => "特殊",
                Self::FukushiKanou => "副詞可能",
            },
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 代名詞
pub enum Daimeishi {
    /// 一般
    General,
    /// 縮約
    Contraction,
}

impl FromStr for Daimeishi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "縮約" => Ok(Self::Contraction),

            _ => Err(POSParseError::new(2, s.to_string(), POSKind::Daimeishi)),
        }
    }
}

impl Display for Daimeishi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*",
            match &self {
                Self::General => "一般",
                Self::Contraction => "縮約",
            },
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞・非自立
pub enum MeishiHijiritsu {
    /// 一般
    General,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 助動詞語幹
    JodoushiGokan,
    /// 副詞可能
    FukushiKanou,
    /// \*
    None,
}

impl FromStr for MeishiHijiritsu {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "形容動詞語幹" => Ok(Self::KeiyoudoushiGokan),
            "助動詞語幹" => Ok(Self::JodoushiGokan),
            "副詞可能" => Ok(Self::FukushiKanou),
            "*" => Ok(Self::None),

            _ => Err(POSParseError::new(
                2,
                s.to_string(),
                POSKind::MeishiHijiritsu,
            )),
        }
    }
}

impl Display for MeishiHijiritsu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*",
            match &self {
                Self::General => "一般",
                Self::KeiyoudoushiGokan => "形容動詞語幹",
                Self::JodoushiGokan => "助動詞語幹",
                Self::FukushiKanou => "副詞可能",
                Self::None => "*",
            },
        )
    }
}
