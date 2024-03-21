use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use crate::JPreprocessError;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
/// 活用形
pub enum CForm {
    /// ガル接続
    ConjunctionGaru,
    /// 音便基本形
    BasicEuphony,
    /// 仮定形
    Conditional,
    /// 仮定縮約１
    ConditionalContraction1,
    /// 仮定縮約２
    ConditionalContraction2,
    /// 基本形
    Basic,
    /// 基本形-促音便
    BasicDoubledConsonant,
    /// 現代基本形
    BasicModern,
    /// 体言接続
    TaigenConjunction,
    /// 体言接続特殊
    TaigenConjunctionSpecial,
    /// 体言接続特殊２
    TaigenConjunctionSpecial2,
    /// 文語基本形
    BasicOld,
    /// 未然ウ接続
    MizenConjunctionU,
    /// 未然ヌ接続
    MizenConjunctionNu,
    /// 未然レル接続
    MizenConjunctionReru,
    /// 未然形
    Mizen,
    /// 未然特殊
    MizenSpecial,
    /// 命令ｅ
    ImperativeE,
    /// 命令ｉ
    ImperativeI,
    /// 命令ｒｏ
    ImperativeRo,
    /// 命令ｙｏ
    ImperativeYo,
    /// 連用ゴザイ接続
    RenyouConjunctionGozai,
    /// 連用タ接続
    RenyouConjunctionTa,
    /// 連用テ接続
    RenyouConjunctionTe,
    /// 連用デ接続
    RenyouConjunctionDe,
    /// 連用ニ接続
    RenyouConjunctionNi,
    /// 連用形
    Renyou,

    ///UniDic
    /// ク語法
    Kugohou,
    /// 仮定形-一般
    KateikeiGeneral,
    /// 仮定形-融合
    KateikeiFusion,
    /// 命令形
    Meireikei,
    /// 已然形-一般
    IzenkeiGeneral,
    /// 已然形-補助
    IzenkeiHojo,
    /// 意志推量形
    Ishisuiryoukei,
    /// 未然形-サ
    MizenkeiSa,
    /// 未然形-セ
    MizenkeiSe,
    /// 未然形-一般
    MizenkeiGeneral,
    /// 未然形-撥音便
    MizenkeiHatsuonbin,
    /// 未然形-補助
    MizenkeiHojo,
    /// 終止形-一般
    ShuushikeiGeneral,
    /// 終止形-促音便
    ShuushikeiSokuonbin,
    /// 終止形-撥音便
    ShuushikeiHatsuonbin,
    /// 終止形-融合
    ShuushikeiFusion,
    /// 終止形-補助
    ShuushikeiHojo,
    /// 語幹-サ
    GokanSa,
    /// 語幹-一般
    GokanGeneral,
    /// 連体形-イ音便
    RentaikeiIonbin,
    /// 連体形-ウ音便
    RentaikeiUonbin,
    /// 連体形-一般
    RentaikeiGeneral,
    /// 連体形-促音便
    RentaikeiSokuonbin,
    /// 連体形-撥音便
    RentaikeiHatsuonbin,
    /// 連体形-省略
    RentaikeiShouryaku,
    /// 連体形-補助
    RentaikeiHojo,
    /// 連用形-イ音便
    RenyoukeiIonbin,
    /// 連用形-ウ音便
    RenyoukeiUonbin,
    /// 連用形-ト
    RenyoukeiTo,
    /// 連用形-ニ
    RenyoukeiNi,
    /// 連用形-一般
    RenyoukeiGeneral,
    /// 連用形-促音便
    RenyoukeiSokuonbin,
    /// 連用形-撥音便
    RenyoukeiHatsuonbin,
    /// 連用形-省略
    RenyoukeiShouryaku,
    /// 連用形-融合
    RenyoukeiFusion,
    /// 連用形-補助
    RenyoukeiHojo,

    /// \*
    #[default]
    None,
}

impl CForm {
    pub fn is_renyou(&self) -> bool {
        matches!(
            self,
            Self::RenyouConjunctionGozai
                | Self::RenyouConjunctionTa
                | Self::RenyouConjunctionTe
                | Self::RenyouConjunctionDe
                | Self::RenyouConjunctionNi
                | Self::Renyou
        )
    }
}

impl FromStr for CForm {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ガル接続" => Ok(Self::ConjunctionGaru),
            "音便基本形" => Ok(Self::BasicEuphony),
            "仮定形" => Ok(Self::Conditional),
            "仮定縮約１" => Ok(Self::ConditionalContraction1),
            "仮定縮約２" => Ok(Self::ConditionalContraction2),
            "基本形" => Ok(Self::Basic),
            "基本形-促音便" => Ok(Self::BasicDoubledConsonant),
            "現代基本形" => Ok(Self::BasicModern),
            "体言接続" => Ok(Self::TaigenConjunction),
            "体言接続特殊" => Ok(Self::TaigenConjunctionSpecial),
            "体言接続特殊２" => Ok(Self::TaigenConjunctionSpecial2),
            "文語基本形" => Ok(Self::BasicOld),
            "未然ウ接続" => Ok(Self::MizenConjunctionU),
            "未然ヌ接続" => Ok(Self::MizenConjunctionNu),
            "未然レル接続" => Ok(Self::MizenConjunctionReru),
            "未然形" => Ok(Self::Mizen),
            "未然特殊" => Ok(Self::MizenSpecial),
            "命令ｅ" => Ok(Self::ImperativeE),
            "命令ｉ" => Ok(Self::ImperativeI),
            "命令ｒｏ" => Ok(Self::ImperativeRo),
            "命令ｙｏ" => Ok(Self::ImperativeYo),
            "連用ゴザイ接続" => Ok(Self::RenyouConjunctionGozai),
            "連用タ接続" => Ok(Self::RenyouConjunctionTa),
            "連用テ接続" => Ok(Self::RenyouConjunctionTe),
            "連用デ接続" => Ok(Self::RenyouConjunctionDe),
            "連用ニ接続" => Ok(Self::RenyouConjunctionNi),
            "連用形" => Ok(Self::Renyou),

            //UniDic
            "ク語法" => Ok(Self::Kugohou),
            "仮定形-一般" => Ok(Self::KateikeiGeneral),
            "仮定形-融合" => Ok(Self::KateikeiFusion),
            "命令形" => Ok(Self::Meireikei),
            "已然形-一般" => Ok(Self::IzenkeiGeneral),
            "已然形-補助" => Ok(Self::IzenkeiHojo),
            "意志推量形" => Ok(Self::Ishisuiryoukei),
            "未然形-サ" => Ok(Self::MizenkeiSa),
            "未然形-セ" => Ok(Self::MizenkeiSe),
            "未然形-一般" => Ok(Self::MizenkeiGeneral),
            "未然形-撥音便" => Ok(Self::MizenkeiHatsuonbin),
            "未然形-補助" => Ok(Self::MizenkeiHojo),
            "終止形-一般" => Ok(Self::ShuushikeiGeneral),
            "終止形-促音便" => Ok(Self::ShuushikeiSokuonbin),
            "終止形-撥音便" => Ok(Self::ShuushikeiHatsuonbin),
            "終止形-融合" => Ok(Self::ShuushikeiFusion),
            "終止形-補助" => Ok(Self::ShuushikeiHojo),
            "語幹-サ" => Ok(Self::GokanSa),
            "語幹-一般" => Ok(Self::GokanGeneral),
            "連体形-イ音便" => Ok(Self::RentaikeiIonbin),
            "連体形-ウ音便" => Ok(Self::RentaikeiUonbin),
            "連体形-一般" => Ok(Self::RentaikeiGeneral),
            "連体形-促音便" => Ok(Self::RentaikeiSokuonbin),
            "連体形-撥音便" => Ok(Self::RentaikeiHatsuonbin),
            "連体形-省略" => Ok(Self::RentaikeiShouryaku),
            "連体形-補助" => Ok(Self::RentaikeiHojo),
            "連用形-イ音便" => Ok(Self::RenyoukeiIonbin),
            "連用形-ウ音便" => Ok(Self::RenyoukeiUonbin),
            "連用形-ト" => Ok(Self::RenyoukeiTo),
            "連用形-ニ" => Ok(Self::RenyoukeiNi),
            "連用形-一般" => Ok(Self::RenyoukeiGeneral),
            "連用形-促音便" => Ok(Self::RenyoukeiSokuonbin),
            "連用形-撥音便" => Ok(Self::RenyoukeiHatsuonbin),
            "連用形-省略" => Ok(Self::RenyoukeiShouryaku),
            "連用形-融合" => Ok(Self::RenyoukeiFusion),
            "連用形-補助" => Ok(Self::RenyoukeiHojo),
            "*" => Ok(Self::None),

            _ => Err(JPreprocessError::CFormParseError),
        }
    }
}

impl Display for CForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::ConjunctionGaru => "ガル接続",
            Self::BasicEuphony => "音便基本形",
            Self::Conditional => "仮定形",
            Self::ConditionalContraction1 => "仮定縮約１",
            Self::ConditionalContraction2 => "仮定縮約２",
            Self::Basic => "基本形",
            Self::BasicDoubledConsonant => "基本形-促音便",
            Self::BasicModern => "現代基本形",
            Self::TaigenConjunction => "体言接続",
            Self::TaigenConjunctionSpecial => "体言接続特殊",
            Self::TaigenConjunctionSpecial2 => "体言接続特殊２",
            Self::BasicOld => "文語基本形",
            Self::MizenConjunctionU => "未然ウ接続",
            Self::MizenConjunctionNu => "未然ヌ接続",
            Self::MizenConjunctionReru => "未然レル接続",
            Self::Mizen => "未然形",
            Self::MizenSpecial => "未然特殊",
            Self::ImperativeE => "命令ｅ",
            Self::ImperativeI => "命令ｉ",
            Self::ImperativeRo => "命令ｒｏ",
            Self::ImperativeYo => "命令ｙｏ",
            Self::RenyouConjunctionGozai => "連用ゴザイ接続",
            Self::RenyouConjunctionTa => "連用タ接続",
            Self::RenyouConjunctionTe => "連用テ接続",
            Self::RenyouConjunctionDe => "連用デ接続",
            Self::RenyouConjunctionNi => "連用ニ接続",
            Self::Renyou => "連用形",

            //UniDic
            Self::Kugohou => "ク語法",
            Self::KateikeiGeneral => "仮定形-一般",
            Self::KateikeiFusion => "仮定形-融合",
            Self::Meireikei => "命令形",
            Self::IzenkeiGeneral => "已然形-一般",
            Self::IzenkeiHojo => "已然形-補助",
            Self::Ishisuiryoukei => "意志推量形",
            Self::MizenkeiSa => "未然形-サ",
            Self::MizenkeiSe => "未然形-セ",
            Self::MizenkeiGeneral => "未然形-一般",
            Self::MizenkeiHatsuonbin => "未然形-撥音便",
            Self::MizenkeiHojo => "未然形-補助",
            Self::ShuushikeiGeneral => "終止形-一般",
            Self::ShuushikeiSokuonbin => "終止形-促音便",
            Self::ShuushikeiHatsuonbin => "終止形-撥音便",
            Self::ShuushikeiFusion => "終止形-融合",
            Self::ShuushikeiHojo => "終止形-補助",
            Self::GokanSa => "語幹-サ",
            Self::GokanGeneral => "語幹-一般",
            Self::RentaikeiIonbin => "連体形-イ音便",
            Self::RentaikeiUonbin => "連体形-ウ音便",
            Self::RentaikeiGeneral => "連体形-一般",
            Self::RentaikeiSokuonbin => "連体形-促音便",
            Self::RentaikeiHatsuonbin => "連体形-撥音便",
            Self::RentaikeiShouryaku => "連体形-省略",
            Self::RentaikeiHojo => "連体形-補助",
            Self::RenyoukeiIonbin => "連用形-イ音便",
            Self::RenyoukeiUonbin => "連用形-ウ音便",
            Self::RenyoukeiTo => "連用形-ト",
            Self::RenyoukeiNi => "連用形-ニ",
            Self::RenyoukeiGeneral => "連用形-一般",
            Self::RenyoukeiSokuonbin => "連用形-促音便",
            Self::RenyoukeiHatsuonbin => "連用形-撥音便",
            Self::RenyoukeiShouryaku => "連用形-省略",
            Self::RenyoukeiFusion => "連用形-融合",
            Self::RenyoukeiHojo => "連用形-補助",

            Self::None => "*",
        })
    }
}
