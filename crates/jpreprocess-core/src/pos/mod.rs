use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

mod doushi;
mod fukushi;
mod joshi;
mod keiyoushi;
mod keijoushi;
mod kandoushi;
mod hojokigou;
mod meishi;
mod kigou;
mod setsubizi;

pub use self::{doushi::*, fukushi::*, joshi::*, keiyoushi::*, keijoushi::*, kandoushi::*, hojokigou::*, meishi::*, kigou::*, setsubizi::*};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("Tried to parse {string} (depth: {depth}), but failed in {kind}")]
pub struct POSParseError {
    depth: u8,
    string: String,
    kind: POSKind,
}
impl POSParseError {
    pub(crate) fn new(depth: u8, string: String, kind: POSKind) -> Self {
        Self {
            depth,
            string,
            kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum POSKind {
    POSMajor,
    Kigou,
    Keiyoushi,
    Keijoushi,
    Kandoushi,
    Joshi,
    Settouzi,
    Doushi,
    Fukushi,
    Meishi,
    KakuJoshi,
    KoyuMeishi,
    FutsuMeishi,
    Person,
    Region,
    MeishiSetsubi,
    Daimeishi,
    MeishiHijiritsu,
    AA,
    Hukukigou,
    Setsubizi,
    Meishiteki,
}
impl Display for POSKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::POSMajor => "品詞",
            Self::Kigou => "記号",
            Self::Keiyoushi => "形容詞",
            Self::Keijoushi => "形状詞",
            Self::Kandoushi => "感動詞",
            Self::Joshi => "助詞",
            Self::Settouzi => "接頭辞",
            Self::Doushi => "動詞",
            Self::Fukushi => "副詞",
            Self::Meishi => "名詞",
            Self::KakuJoshi => "格助詞",
            Self::KoyuMeishi => "固有名詞",
            Self::FutsuMeishi => "普通名詞",
            Self::Person => "人名（固有名詞）",
            Self::Region => "地域（固有名詞）",
            Self::MeishiSetsubi => "接尾（名詞）",
            Self::Daimeishi => "代名詞",
            Self::MeishiHijiritsu => "非自立（名詞）",
            Self::AA => "ＡＡ",
            Self::Hukukigou => "補助記号",
            Self::Setsubizi => "接尾辞",
            Self::Meishiteki => "名詞的",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
/// 品詞
pub enum POS {
    /// 代名詞
    Daimeishi,
    /// 副詞
    Fukushi(Fukushi),
    /// 助動詞
    Jodoushi,
    /// 助詞
    Joshi(Joshi),
    /// 動詞
    Doushi(Doushi),
    /// 名詞
    Meishi(Meishi),
    /// 形容詞
    Keiyoushi(Keiyoushi),
    /// 形状詞
    Keijoushi(Keijoushi),
    /// 感動詞
    Kandoushi(Kandoushi),
    /// 接尾辞
    Setsubizi(Setsubizi),
    /// 接頭詞
    Settouzi,
    /// 接続詞
    Setsuzokushi,
    /// 空白
    Space,
    /// 補助記号
    Hojokigou(Hojokigou),
    /// 記号
    Kigou(Kigou),
    /// 連体詞
    Rentaishi,

    /// その他
    Others,


    /// フィラー
    // Filler,
    ///副詞
    // Fukushi(Fukushi),
    /// 助動詞
    // Jodoushi,
    /// 助詞
    // Joshi(Joshi),
    /// 動詞
    // Doushi(Doushi),
    /// 名詞
    // Meishi(Meishi),
    /// 形容詞
    // Keiyoushi(Keiyoushi),
    /// 感動詞
    // Kandoushi,
    /// 接続詞
    // Setsuzokushi,
    /// 接頭詞
    // Settoushi(Settoushi),
    /// 記号
    // Kigou(Kigou),
    /// 連体詞
    // Rentaishi,

    /// 不明
    #[default]
    Unknown,
}

impl POS {
    pub fn from_strs(g0: &str, g1: &str, g2: &str, g3: &str) -> Result<Self, POSParseError> {
        match g0 {
            "代名詞" => Ok(Self::Daimeishi),
            "副詞" => Fukushi::from_str(g1).map(Self::Fukushi),
            "助動詞" => Ok(Self::Jodoushi),
            "助詞" => Joshi::from_strs(g1, g2).map(Self::Joshi),
            "動詞" => Doushi::from_str(g1).map(Self::Doushi),
            "名詞" => Meishi::from_strs(g1, g2, g3).map(Self::Meishi),
            "形容詞" => Keiyoushi::from_str(g1).map(Self::Keiyoushi),
            "形状詞" => Keijoushi::from_str(g1).map(Self::Keijoushi),
            "感動詞" => Kandoushi::from_str(g1).map(Self::Kandoushi),
            "接尾辞" => Setsubizi::from_str(g1).map(Self::Setsubizi),
            "接頭辞" => Ok(Self::Settouzi),
            "接続詞" => Ok(Self::Setsuzokushi),
            "空白" => Ok(Self::Space),
            "補助記号" => Hojokigou::from_str(g1).map(Self::Hojokigou),
            "記号" => Kigou::from_str(g1).map(Self::Kigou),
            "連体詞" => Ok(Self::Rentaishi),

            // "フィラー" => Ok(Self::Filler),
            // "感動詞" => Ok(Self::Kandoushi),
            // "記号" => Kigou::from_str(g1).map(Self::Kigou),
            // "形容詞" => Keiyoushi::from_str(g1).map(Self::Keiyoushi),
            // "助詞" => Joshi::from_strs(g1, g2).map(Self::Joshi),
            // "助動詞" => Ok(Self::Jodoushi),
            // "接続詞" => Ok(Self::Setsuzokushi),
            // "接頭詞" => Settoushi::from_str(g1).map(Self::Settoushi),
            // "動詞" => Doushi::from_str(g1).map(Self::Doushi),
            // "副詞" => Fukushi::from_str(g1).map(Self::Fukushi),
            // "名詞" => Meishi::from_strs(g1, g2, g3).map(Self::Meishi),
            // "連体詞" => Ok(Self::Rentaishi),

            "その他" => Ok(Self::Others),

            "*" => Ok(Self::Unknown),

            _ => Err(POSParseError::new(0, g0.to_string(), POSKind::POSMajor)),
        }
    }

    pub fn is_kazu(&self) -> bool {
        matches!(self, Self::Kigou(Kigou::Char) | Self::Meishi(Meishi::Suushi))
    }

    pub fn convert_to_kigou(&mut self) {
        *self = match self {
            Self::Kigou(kigou) => Self::Kigou(*kigou),
            Self::Meishi(Meishi::Suushi) => Self::Kigou(Kigou::General),
            Self::Fukushi(Fukushi::General) | Self::Meishi(Meishi::FutsuMeishi(FutsuMeishi::General)) => {
                Self::Kigou(Kigou::General)
            }
            _ => Self::Kigou(Kigou::None),
        }
    }
}

impl Display for POS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match &self {
            Self::Daimeishi => "代名詞,*,*,*".to_string(),
            Self::Fukushi(fukushi) => format!("副詞,{}", fukushi),
            Self::Jodoushi => "助動詞,*,*,*".to_string(),
            Self::Joshi(joshi) => format!("助詞,{}", joshi),
            Self::Doushi(doushi) => format!("動詞,{}", doushi),
            Self::Meishi(meishi) => format!("名詞,{}", meishi),
            Self::Keiyoushi(keiyoushi) => format!("形容詞,{}", keiyoushi),
            Self::Keijoushi(keijoushi) => format!("形状詞,{}", keijoushi),
            Self::Kandoushi(kandoushi) => format!("感動詞,{}", kandoushi),
            Self::Setsubizi(setsubizi) => format!("接尾辞,{}", setsubizi),
            Self::Settouzi => "接頭辞,*,*,*".to_string(),
            Self::Setsuzokushi => "接続詞,*,*,*".to_string(),
            Self::Space => "空白,*,*,*".to_string(),
            Self::Hojokigou(hukukigou) => format!("補助記号,{}", hukukigou),
            Self::Kigou(kigou) => format!("記号,{}", kigou),
            Self::Rentaishi => "連体詞,*,*,*".to_string(),




            // Self::Filler => "フィラー,*,*,*".to_string(),
            // Self::Kandoushi => "感動詞,*,*,*".to_string(),
            // Self::Kigou(kigou) => format!("記号,{}", kigou),
            // Self::Keiyoushi(keiyoushi) => format!("形容詞,{}", keiyoushi),
            // Self::Joshi(joshi) => format!("助詞,{}", joshi),
            // Self::Jodoushi => "助動詞,*,*,*".to_string(),
            // Self::Setsuzokushi => "接続詞,*,*,*".to_string(),
            // Self::Settoushi(settoushi) => format!("接頭詞,{}", settoushi),
            // Self::Doushi(doushi) => format!("動詞,{}", doushi),
            // Self::Fukushi(fukushi) => format!("副詞,{}", fukushi),
            // Self::Meishi(meishi) => format!("名詞,{}", meishi),
            // Self::Rentaishi => "連体詞,*,*,*".to_string(),

            Self::Others => "その他,*,*,*".to_string(),

            Self::Unknown => "*,*,*,*".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn filler() {
    //     let pos = POS::from_strs("フィラー", "*", "*", "*").unwrap();
    //     assert!(matches!(pos, POS::Filler));
    //     assert_eq!(pos.to_string(), "フィラー,*,*,*")
    // }

    #[test]
    fn joshi() {
        let pos = POS::from_strs("助詞", "副助詞／並立助詞／終助詞", "*", "*").unwrap();
        assert!(matches!(pos, POS::Joshi(Joshi::FukuHeiritsuShuJoshi)));
        assert_eq!(pos.to_string(), "助詞,副助詞／並立助詞／終助詞,*,*")
    }

    #[test]
    fn meishi() {
        let pos = POS::from_strs("名詞", "固有名詞", "人名", "姓").unwrap();
        assert!(matches!(
            pos,
            POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(Person::Sei)))
        ));
        assert_eq!(pos.to_string(), "名詞,固有名詞,人名,姓")
    }
}
