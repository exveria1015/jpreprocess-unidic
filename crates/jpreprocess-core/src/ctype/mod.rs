use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

mod five;
mod four;
mod ka_irregular;
mod keiyoushi;
mod lower_two;
mod old;
mod one;
mod sa_irregular;
mod special;
mod upper_two;

pub use five::*;
pub use four::*;
pub use ka_irregular::*;
pub use keiyoushi::*;
pub use lower_two::*;
pub use old::*;
pub use one::*;
pub use sa_irregular::*;
pub use special::*;
pub use upper_two::*;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("Tried to parse {string}, but failed in {kind}")]
pub struct CTypeParseError {
    string: String,
    kind: CTypeKind,
}
impl CTypeParseError {
    pub(crate) fn new(string: String, kind: CTypeKind) -> Self {
        Self { string, kind }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CTypeKind {
    CTypeMajor,
    Five,
    Four,
    KaIrregular,
    Keiyoushi,
    LowerTwo,
    Old,
    One,
    SaIrregular,
    Special,
    UpperTwo,
    
    //UniDic 3.1.0

}
impl Display for CTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CTypeMajor => "活用形",
            Self::Five => "五段",
            Self::Four => "四段",
            Self::KaIrregular => "カ変",
            Self::Keiyoushi => "形容詞",
            Self::LowerTwo => "下二",
            Self::Old => "文語",
            Self::One => "一段",
            Self::SaIrregular => "サ変",
            Self::Special => "特殊",
            Self::UpperTwo => "上二",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
/// 活用
pub enum CType {
    /// カ変
    KaIrregular(KaIrregular),
    /// サ変
    SaIrregular(SaIrregular),
    /// ラ変
    RaIrregular,
    /// 一段
    One(One),
    /// 下二
    LowerTwo(LowerTwo),
    /// 形容詞
    Keiyoushi(Keiyoushi),
    /// 五段
    Five(Five),
    /// 四段
    Four(Four),
    /// 上二
    UpperTwo(UpperTwo),
    /// 特殊
    Special(Special),
    /// 不変化型
    NoConjugation,
    /// 文語
    Old(Old),

    //UniDic 3.1.0
// カ行変格
    KagyouHenkaku,
// サ行変格
    SagyouHenkaku,
// 上一段-ア行
    KamiichidanAGyou,
// 上一段-カ行
    KamiichidanKAGyou,
// 上一段-ガ行
    KamiichidanGAGyou,
// 上一段-ザ行
    KamiichidanZaGyou,
// 上一段-タ行
    KamiichidanTAGyou,
// 上一段-ナ行
    KamiichidanNAGyou,
// 上一段-ハ行
    KamiichidanHAGyou,
// 上一段-バ行
    KamiichidanBAGyou,
// 上一段-マ行
    KamiichidanMAGyou,
// 上一段-ラ行
    KamiichidanRAGyou,
// 下一段-ア行
    ShimoIchidanAGyou,
// 下一段-カ行
    ShimoIchidanKAGyou,
// 下一段-ガ行
    ShimoIchidanGAGyou,
// 下一段-サ行
    ShimoIchidanSAGyou,
// 下一段-ザ行
    ShimoIchidanZAGyou,
// 下一段-タ行
    ShimoIchidanTAGyou,
// 下一段-ダ行
    ShimoIchidanDAGyou,
// 下一段-ナ行
    ShimoIchidanNAGyou,
// 下一段-ハ行
    ShimoIchidanHAGyou,
// 下一段-バ行
    ShimoIchidanBAGyou,
// 下一段-マ行
    ShimoIchidanMAGyou,
// 下一段-ラ行
    ShimoIchidanRAGyou,
// 五段-カ行
    GodanKAGyou,
// 五段-ガ行
    GodanGAGyou,
// 五段-サ行
    GodanSAGyou,
// 五段-タ行
    GodanTAGyou,
// 五段-ナ行
    GodanNAGyou,
// 五段-バ行
    GodanBAGyou,
// 五段-マ行
    GodanMAGyou,
// 五段-ラ行
    GodanRAGyou,
// 五段-ワア行
    GodanWAGyou,
// 助動詞-ザマス
    JodoushiZamasu,
// 助動詞-ジャ
    JodoushiJa,
// 助動詞-タ
    JodoushiTa,
// 助動詞-タイ
    JodoushiTai,
// 助動詞-ダ
    JodoushiDa,
// 助動詞-デス
    JodoushiDesu,
// 助動詞-ドス
    JodoushiDosu,
// 助動詞-ナイ
    JodoushiNai,
// 助動詞-ナンダ
    JodoushiNanda,
// 助動詞-ヌ
    JodoushiNu,
// 助動詞-ヒン
    JodoushiHin,
// 助動詞-ヘン
    JodoushiHen,
// 助動詞-マイ
    JodoushiMai,
// 助動詞-マス
    JodoushiMasu,
// 助動詞-ヤ
    JodoushiYa,
// 助動詞-ヤス
    JodoushiYasu,
// 助動詞-ヤン
    JodoushiYan,
// 助動詞-ラシイ
    JodoushiRashii,
// 助動詞-レル
    JodoushiReru,
// 助動詞-ンス
    JodoushiNsu,
// 文語カ行変格
    BungoKagyouHenkaku,
// 文語サ行変格
    BungoSagyouHenkaku,
// 文語ラ行変格
    BungoRagyouHenkaku,
// 文語上一段-マ行
    BungoKamiichidanMAGyou,
// 文語上二段-タ行
    BungoKamiNidanTAGyou,
// 文語上二段-バ行
    BungoKamiNidanBAGyou,
// 文語上二段-ラ行
    BungoKamiNidanRAGyou,
// 文語下二段-ア行
    BungoShimoNidanAGyou,
// 文語下二段-カ行
    BungoShimoNidanKAGyou,
// 文語下二段-ガ行
    BungoShimoNidanGAGyou,
// 文語下二段-サ行
    BungoShimoNidanSAGyou,
// 文語下二段-タ行
    BungoShimoNidanTAGyou,
// 文語下二段-ダ行
    BungoShimoNidanDAGyou,
// 文語下二段-ナ行
    BungoShimoNidanNAGyou,
// 文語下二段-ハ行
    BungoShimoNidanHAGyou,
// 文語下二段-マ行
    BungoShimoNidanMAGyou,
// 文語下二段-ヤ行
    BungoShimoNidanYAGyou,
// 文語下二段-ラ行
    BungoShimoNidanRAGyou,
// 文語下二段-ワ行
    BungoShimoNidanWAGyou,
// 文語助動詞-キ
    BungoJodoushiKi,
// 文語助動詞-ケリ
    BungoJodoushiKeri,
// 文語助動詞-ゴトシ
    BungoJodoushiGotoshi,
// 文語助動詞-ズ
    BungoJodoushiZu,
// 文語助動詞-タリ-完了
    BungoJodoushiTariKanryou,
// 文語助動詞-タリ-断定
    BungoJodoushiTariDantei,
// 文語助動詞-ナリ-断定
    BungoJodoushiNariDantei,
// 文語助動詞-ベシ
    BungoJodoushiBeshi,
    // 文語助動詞-マジ
    BungoJodoushiMaji,
    // 文語助動詞-ム
    BungoJodoushiMu,
    // 文語助動詞-リ
    BungoJodoushiRi,
    // 文語四段-カ行
    BungoYodanKAGyou,
    // 文語四段-ガ行
    BungoYodanGAGyou,
    // 文語四段-サ行
    BungoYodanSAGyou,
    // 文語四段-タ行
    BungoYodanTAGyou,
    // 文語四段-ハ行
    BungoYodanHAGyou,
    // 文語四段-バ行
    BungoYodanBAGyou,
    // 文語四段-マ行
    BungoYodanMAGyou,
    // 文語四段-ラ行
    BungoYodanRAGyou,
    // 文語形容詞-ク
    BungoKeiyoushiKu,
    // 文語形容詞-シク
    BungoKeiyoushiShiku,
    // 無変化型
    Muhenkakei,
    // 特殊型
    SpecialType,

    /// \*
    #[default]
    None,
}

impl FromStr for CType {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, minor) = s.split_once('・').unwrap_or((s, ""));
        match major {
            "カ変" => Ok(Self::KaIrregular(KaIrregular::from_str(minor)?)),
            "サ変" => Ok(Self::SaIrregular(SaIrregular::from_str(minor)?)),
            "ラ変" => Ok(Self::RaIrregular),
            "一段" => Ok(Self::One(One::from_str(minor)?)),
            "下二" => Ok(Self::LowerTwo(LowerTwo::from_str(minor)?)),
            "形容詞" => Ok(Self::Keiyoushi(Keiyoushi::from_str(minor)?)),
            "五段" => Ok(Self::Five(Five::from_str(minor)?)),
            "四段" => Ok(Self::Four(Four::from_str(minor)?)),
            "上二" => Ok(Self::UpperTwo(UpperTwo::from_str(minor)?)),
            "特殊" => Ok(Self::Special(Special::from_str(minor)?)),
            "不変化型" => Ok(Self::NoConjugation),
            "文語" => Ok(Self::Old(Old::from_str(minor)?)),
            "*" => Ok(Self::None),
            //UniDic 3.1.0
            "カ行変格" => Ok(Self::KagyouHenkaku),
            "サ行変格" => Ok(Self::SagyouHenkaku),
            "上一段-ア行" => Ok(Self::KamiichidanAGyou),
            "上一段-カ行" => Ok(Self::KamiichidanKAGyou),
            "上一段-ガ行" => Ok(Self::KamiichidanGAGyou),
            "上一段-ザ行" => Ok(Self::KamiichidanZaGyou),
            "上一段-タ行" => Ok(Self::KamiichidanTAGyou),
            "上一段-ナ行" => Ok(Self::KamiichidanNAGyou),
            "上一段-ハ行" => Ok(Self::KamiichidanHAGyou),
            "上一段-バ行" => Ok(Self::KamiichidanBAGyou),
            "上一段-マ行" => Ok(Self::KamiichidanMAGyou),
            "上一段-ラ行" => Ok(Self::KamiichidanRAGyou),
            "下一段-ア行" => Ok(Self::ShimoIchidanAGyou),
            "下一段-カ行" => Ok(Self::ShimoIchidanKAGyou),
            "下一段-ガ行" => Ok(Self::ShimoIchidanGAGyou),
            "下一段-サ行" => Ok(Self::ShimoIchidanSAGyou),
            "下一段-ザ行" => Ok(Self::ShimoIchidanZAGyou),
            "下一段-タ行" => Ok(Self::ShimoIchidanTAGyou),
            "下一段-ダ行" => Ok(Self::ShimoIchidanDAGyou),
            "下一段-ナ行" => Ok(Self::ShimoIchidanNAGyou),
            "下一段-ハ行" => Ok(Self::ShimoIchidanHAGyou),
            "下一段-バ行" => Ok(Self::ShimoIchidanBAGyou),
            "下一段-マ行" => Ok(Self::ShimoIchidanMAGyou),
            "下一段-ラ行" => Ok(Self::ShimoIchidanRAGyou),
            "五段-カ行" => Ok(Self::GodanKAGyou),
            "五段-ガ行" => Ok(Self::GodanGAGyou),
            "五段-サ行" => Ok(Self::GodanSAGyou),
            "五段-タ行" => Ok(Self::GodanTAGyou),
            "五段-ナ行" => Ok(Self::GodanNAGyou),
            "五段-バ行" => Ok(Self::GodanBAGyou),
            "五段-マ行" => Ok(Self::GodanMAGyou),
            "五段-ラ行" => Ok(Self::GodanRAGyou),
            "五段-ワア行" => Ok(Self::GodanWAGyou),
            "助動詞-ザマス" => Ok(Self::JodoushiZamasu),
            "助動詞-ジャ" => Ok(Self::JodoushiJa),
            "助動詞-タ" => Ok(Self::JodoushiTa),
            "助動詞-タイ" => Ok(Self::JodoushiTai),
            "助動詞-ダ" => Ok(Self::JodoushiDa),
            "助動詞-デス" => Ok(Self::JodoushiDesu),
            "助動詞-ドス" => Ok(Self::JodoushiDosu),
            "助動詞-ナイ" => Ok(Self::JodoushiNai),
            "助動詞-ナンダ" => Ok(Self::JodoushiNanda),
            "助動詞-ヌ" => Ok(Self::JodoushiNu),
            "助動詞-ヒン" => Ok(Self::JodoushiHin),
            "助動詞-ヘン" => Ok(Self::JodoushiHen),
            "助動詞-マイ" => Ok(Self::JodoushiMai),
            "助動詞-マス" => Ok(Self::JodoushiMasu),
            "助動詞-ヤ" => Ok(Self::JodoushiYa),
            "助動詞-ヤス" => Ok(Self::JodoushiYasu),
            "助動詞-ヤン" => Ok(Self::JodoushiYan),
            "助動詞-ラシイ" => Ok(Self::JodoushiRashii),
            "助動詞-レル" => Ok(Self::JodoushiReru),
            "助動詞-ンス" => Ok(Self::JodoushiNsu),
            "文語カ行変格" => Ok(Self::BungoKagyouHenkaku),
            "文語サ行変格" => Ok(Self::BungoSagyouHenkaku),
            "文語ラ行変格" => Ok(Self::BungoRagyouHenkaku),
            "文語上一段-マ行" => Ok(Self::BungoKamiichidanMAGyou),
            "文語上二段-タ行" => Ok(Self::BungoKamiNidanTAGyou),
            "文語上二段-バ行" => Ok(Self::BungoKamiNidanBAGyou),
            "文語上二段-ラ行" => Ok(Self::BungoKamiNidanRAGyou),
            "文語下二段-ア行" => Ok(Self::BungoShimoNidanAGyou),
            "文語下二段-カ行" => Ok(Self::BungoShimoNidanKAGyou),
            "文語下二段-ガ行" => Ok(Self::BungoShimoNidanGAGyou),
            "文語下二段-サ行" => Ok(Self::BungoShimoNidanSAGyou),
            "文語下二段-タ行" => Ok(Self::BungoShimoNidanTAGyou),
            "文語下二段-ダ行" => Ok(Self::BungoShimoNidanDAGyou),
            "文語下二段-ナ行" => Ok(Self::BungoShimoNidanNAGyou),
            "文語下二段-ハ行" => Ok(Self::BungoShimoNidanHAGyou),
            "文語下二段-マ行" => Ok(Self::BungoShimoNidanMAGyou),
            "文語下二段-ヤ行" => Ok(Self::BungoShimoNidanYAGyou),
            "文語下二段-ラ行" => Ok(Self::BungoShimoNidanRAGyou),
            "文語下二段-ワ行" => Ok(Self::BungoShimoNidanWAGyou),
            "文語助動詞-キ" => Ok(Self::BungoJodoushiKi),
            "文語助動詞-ケリ" => Ok(Self::BungoJodoushiKeri),
            "文語助動詞-ゴトシ" => Ok(Self::BungoJodoushiGotoshi),
            "文語助動詞-ズ" => Ok(Self::BungoJodoushiZu),
            "文語助動詞-タリ-完了" => Ok(Self::BungoJodoushiTariKanryou),
            "文語助動詞-タリ-断定" => Ok(Self::BungoJodoushiTariDantei),
            "文語助動詞-ナリ-断定" => Ok(Self::BungoJodoushiNariDantei),
            "文語助動詞-ベシ" => Ok(Self::BungoJodoushiBeshi),
            "文語助動詞-マジ" => Ok(Self::BungoJodoushiMaji),
            "文語助動詞-ム" => Ok(Self::BungoJodoushiMu),
            "文語助動詞-リ" => Ok(Self::BungoJodoushiRi),
            "文語四段-カ行" => Ok(Self::BungoYodanKAGyou),
            "文語四段-ガ行" => Ok(Self::BungoYodanGAGyou),
            "文語四段-サ行" => Ok(Self::BungoYodanSAGyou),
            "文語四段-タ行" => Ok(Self::BungoYodanTAGyou),
            "文語四段-ハ行" => Ok(Self::BungoYodanHAGyou),
            "文語四段-バ行" => Ok(Self::BungoYodanBAGyou),
            "文語四段-マ行" => Ok(Self::BungoYodanMAGyou),
            "文語四段-ラ行" => Ok(Self::BungoYodanRAGyou),
            "文語形容詞-ク" => Ok(Self::BungoKeiyoushiKu),
            "文語形容詞-シク" => Ok(Self::BungoKeiyoushiShiku),
            "無変化型" => Ok(Self::Muhenkakei),
            "特殊型" => Ok(Self::SpecialType),

            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::CTypeMajor)),
        }
    }
}

impl Display for CType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Current implementation loses the latter half of CType
        let (major, minor) = match &self {
            Self::KaIrregular(minor) => ("カ変", minor.to_string()),
            Self::SaIrregular(minor) => ("サ変", minor.to_string()),
            Self::RaIrregular => ("ラ変", "".to_string()),
            Self::One(minor) => ("一段", minor.to_string()),
            Self::LowerTwo(minor) => ("下二", minor.to_string()),
            Self::Keiyoushi(minor) => ("形容詞", minor.to_string()),
            Self::Five(minor) => ("五段", minor.to_string()),
            Self::Four(minor) => ("四段", minor.to_string()),
            Self::UpperTwo(minor) => ("上二", minor.to_string()),
            Self::Special(minor) => ("特殊", minor.to_string()),
            Self::NoConjugation => ("不変化型", "".to_string()),
            Self::Old(minor) => ("文語", minor.to_string()),
            // UniDic 3.1.0
            Self::KagyouHenkaku => ("カ行変格", "".to_string()),
            Self::SagyouHenkaku => ("サ行変格", "".to_string()),
            Self::KamiichidanAGyou => ("上一段-ア行", "".to_string()),
            Self::KamiichidanKAGyou => ("上一段-カ行", "".to_string()),
            Self::KamiichidanGAGyou => ("上一段-ガ行", "".to_string()),
            Self::KamiichidanZaGyou => ("上一段-ザ行", "".to_string()),
            Self::KamiichidanTAGyou => ("上一段-タ行", "".to_string()),
            Self::KamiichidanNAGyou => ("上一段-ナ行", "".to_string()),
            Self::KamiichidanHAGyou => ("上一段-ハ行", "".to_string()),
            Self::KamiichidanBAGyou => ("上一段-バ行", "".to_string()),
            Self::KamiichidanMAGyou => ("上一段-マ行", "".to_string()),
            Self::KamiichidanRAGyou => ("上一段-ラ行", "".to_string()),
            Self::ShimoIchidanAGyou => ("下一段-ア行", "".to_string()),
            Self::ShimoIchidanKAGyou => ("下一段-カ行", "".to_string()),
            Self::ShimoIchidanGAGyou => ("下一段-ガ行", "".to_string()),
            Self::ShimoIchidanSAGyou => ("下一段-サ行", "".to_string()),
            Self::ShimoIchidanZAGyou => ("下一段-ザ行", "".to_string()),
            Self::ShimoIchidanTAGyou => ("下一段-タ行", "".to_string()),
            Self::ShimoIchidanDAGyou => ("下一段-ダ行", "".to_string()),
            Self::ShimoIchidanNAGyou => ("下一段-ナ行", "".to_string()),
            Self::ShimoIchidanHAGyou => ("下一段-ハ行", "".to_string()),
            Self::ShimoIchidanBAGyou => ("下一段-バ行", "".to_string()),
            Self::ShimoIchidanMAGyou => ("下一段-マ行", "".to_string()),
            Self::ShimoIchidanRAGyou => ("下一段-ラ行", "".to_string()),
            Self::GodanKAGyou => ("五段-カ行", "".to_string()),
            Self::GodanGAGyou => ("五段-ガ行", "".to_string()),
            Self::GodanSAGyou => ("五段-サ行", "".to_string()),
            Self::GodanTAGyou => ("五段-タ行", "".to_string()),
            Self::GodanNAGyou => ("五段-ナ行", "".to_string()),
            Self::GodanBAGyou => ("五段-バ行", "".to_string()),
            Self::GodanMAGyou => ("五段-マ行", "".to_string()),
            Self::GodanRAGyou => ("五段-ラ行", "".to_string()),
            Self::GodanWAGyou => ("五段-ワア行", "".to_string()),
            Self::JodoushiZamasu => ("助動詞-ザマス", "".to_string()),
            Self::JodoushiJa => ("助動詞-ジャ", "".to_string()),
            Self::JodoushiTa => ("助動詞-タ", "".to_string()),
            Self::JodoushiTai => ("助動詞-タイ", "".to_string()),
            Self::JodoushiDa => ("助動詞-ダ", "".to_string()),
            Self::JodoushiDesu => ("助動詞-デス", "".to_string()),
            Self::JodoushiDosu => ("助動詞-ドス", "".to_string()),
            Self::JodoushiNai => ("助動詞-ナイ", "".to_string()),
            Self::JodoushiNanda => ("助動詞-ナンダ", "".to_string()),
            Self::JodoushiNu => ("助動詞-ヌ", "".to_string()),
            Self::JodoushiHin => ("助動詞-ヒン", "".to_string()),
            Self::JodoushiHen => ("助動詞-ヘン", "".to_string()),
            Self::JodoushiMai => ("助動詞-マイ", "".to_string()),
            Self::JodoushiMasu => ("助動詞-マス", "".to_string()),
            Self::JodoushiYa => ("助動詞-ヤ", "".to_string()),
            Self::JodoushiYasu => ("助動詞-ヤス", "".to_string()),
            Self::JodoushiYan => ("助動詞-ヤン", "".to_string()),
            Self::JodoushiRashii => ("助動詞-ラシイ", "".to_string()),
            Self::JodoushiReru => ("助動詞-レル", "".to_string()),
            Self::JodoushiNsu => ("助動詞-ンス", "".to_string()),
            Self::BungoKagyouHenkaku => ("文語カ行変格", "".to_string()),
            Self::BungoSagyouHenkaku => ("文語サ行変格", "".to_string()),
            Self::BungoRagyouHenkaku => ("文語ラ行変格", "".to_string()),
            Self::BungoKamiichidanMAGyou => ("文語上一段-マ行", "".to_string()),
            Self::BungoKamiNidanTAGyou => ("文語上二段-タ行", "".to_string()),
            Self::BungoKamiNidanBAGyou => ("文語上二段-バ行", "".to_string()),
            Self::BungoKamiNidanRAGyou => ("文語上二段-ラ行", "".to_string()),
            Self::BungoShimoNidanAGyou => ("文語下二段-ア行", "".to_string()),
            Self::BungoShimoNidanKAGyou => ("文語下二段-カ行", "".to_string()),
            Self::BungoShimoNidanGAGyou => ("文語下二段-ガ行", "".to_string()),
            Self::BungoShimoNidanSAGyou => ("文語下二段-サ行", "".to_string()),
            Self::BungoShimoNidanTAGyou => ("文語下二段-タ行", "".to_string()),
            Self::BungoShimoNidanDAGyou => ("文語下二段-ダ行", "".to_string()),
            Self::BungoShimoNidanNAGyou => ("文語下二段-ナ行", "".to_string()),
            Self::BungoShimoNidanHAGyou => ("文語下二段-ハ行", "".to_string()),
            Self::BungoShimoNidanMAGyou => ("文語下二段-マ行", "".to_string()),
            Self::BungoShimoNidanYAGyou => ("文語下二段-ヤ行", "".to_string()),
            Self::BungoShimoNidanRAGyou => ("文語下二段-ラ行", "".to_string()),
            Self::BungoShimoNidanWAGyou => ("文語下二段-ワ行", "".to_string()),
            Self::BungoJodoushiKi => ("文語助動詞-キ", "".to_string()),
            Self::BungoJodoushiKeri => ("文語助動詞-ケリ", "".to_string()),
            Self::BungoJodoushiGotoshi => ("文語助動詞-ゴトシ", "".to_string()),
            Self::BungoJodoushiZu => ("文語助動詞-ズ", "".to_string()),
            Self::BungoJodoushiTariKanryou => ("文語助動詞-タリ-完了", "".to_string()),
            Self::BungoJodoushiTariDantei => ("文語助動詞-タリ-断定", "".to_string()),
            Self::BungoJodoushiNariDantei => ("文語助動詞-ナリ-断定", "".to_string()),
            Self::BungoJodoushiBeshi => ("文語助動詞-ベシ", "".to_string()),
            Self::BungoJodoushiMaji => ("文語助動詞-マジ", "".to_string()),
            Self::BungoJodoushiMu => ("文語助動詞-ム", "".to_string()),
            Self::BungoJodoushiRi => ("文語助動詞-リ", "".to_string()),
            Self::BungoYodanKAGyou => ("文語四段-カ行", "".to_string()),
            Self::BungoYodanGAGyou => ("文語四段-ガ行", "".to_string()),
            Self::BungoYodanSAGyou => ("文語四段-サ行", "".to_string()),
            Self::BungoYodanTAGyou => ("文語四段-タ行", "".to_string()),
            Self::BungoYodanHAGyou => ("文語四段-ハ行", "".to_string()),
            Self::BungoYodanBAGyou => ("文語四段-バ行", "".to_string()),
            Self::BungoYodanMAGyou => ("文語四段-マ行", "".to_string()),
            Self::BungoYodanRAGyou => ("文語四段-ラ行", "".to_string()),
            Self::BungoKeiyoushiKu => ("文語形容詞-ク", "".to_string()),
            Self::BungoKeiyoushiShiku => ("文語形容詞-シク", "".to_string()),
            Self::Muhenkakei => ("無変化型", "".to_string()),
            Self::SpecialType => ("特殊型", "".to_string()),


            Self::None => ("*", "".to_string()),
        };

        if minor.is_empty() {
            write!(f, "{}", major)
        } else {
            write!(f, "{}・{}", major, minor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none() {
        let ctype = CType::from_str("*").unwrap();
        assert!(matches!(ctype, CType::None));
        assert_eq!(ctype.to_string(), "*")
    }

    #[test]
    fn ra_irregular() {
        let ctype = CType::from_str("ラ変").unwrap();
        assert!(matches!(ctype, CType::RaIrregular));
        assert_eq!(ctype.to_string(), "ラ変")
    }

    #[test]
    fn lower_two() {
        let ctype = CType::from_str("下二・ア行").unwrap();
        assert!(matches!(ctype, CType::LowerTwo(LowerTwo::A)));
        assert_eq!(ctype.to_string(), "下二・ア行")
    }

    #[test]
    fn one_empty() {
        let ctype = CType::from_str("一段").unwrap();
        assert!(matches!(ctype, CType::One(One::None)));
        assert_eq!(ctype.to_string(), "一段")
    }
}
