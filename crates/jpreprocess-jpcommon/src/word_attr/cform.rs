use jpreprocess_core::cform::CForm;

pub fn cform_to_id(cform: &CForm) -> Option<u8> {
    match cform {
        // *:xx
        CForm::None => None,
        // その他:6
        CForm::ConjunctionGaru => Some(6),
        // 仮定形:4
        CForm::Conditional => Some(4),
        CForm::ConditionalContraction1 => Some(4),
        CForm::ConditionalContraction2 => Some(4),
        // 基本形:2
        CForm::Basic => Some(2),
        CForm::BasicDoubledConsonant => Some(2),
        CForm::BasicModern => Some(2),
        CForm::BasicEuphony => Some(2),
        CForm::BasicOld => Some(2),
        // 未然形:0
        CForm::Mizen => Some(0),
        CForm::MizenConjunctionU => Some(0),
        CForm::MizenConjunctionNu => Some(0),
        CForm::MizenConjunctionReru => Some(0),
        CForm::MizenSpecial => Some(0),
        // 命令形:5
        CForm::ImperativeE => Some(5),
        CForm::ImperativeI => Some(5),
        CForm::ImperativeRo => Some(5),
        CForm::ImperativeYo => Some(5),

        // 連体形:3
        CForm::TaigenConjunction => Some(3),
        CForm::TaigenConjunctionSpecial => Some(3),
        CForm::TaigenConjunctionSpecial2 => Some(3),
        // 連用形:1
        CForm::Renyou => Some(1),
        CForm::RenyouConjunctionGozai => Some(1),
        CForm::RenyouConjunctionTa => Some(1),
        CForm::RenyouConjunctionTe => Some(1),
        CForm::RenyouConjunctionDe => Some(1),
        CForm::RenyouConjunctionNi => Some(1),

        //UniDic
        // ク語法:7
        CForm::Kugohou => Some(7),
        // 仮定形:4
        CForm::KateikeiGeneral => Some(4),
        CForm::KateikeiFusion => Some(4),
        // 命令形:5
        CForm::Meireikei => Some(5),
        // 未然形:0
        CForm::MizenkeiGeneral => Some(0),
        CForm::MizenkeiSa => Some(0),
        CForm::MizenkeiSe => Some(0),
        CForm::MizenkeiHatsuonbin => Some(0),
        CForm::MizenkeiHojo => Some(0),
        // 已然形:8
        CForm::IzenkeiGeneral => Some(8),
        CForm::IzenkeiHojo => Some(8),
        // 意志推量形:9
        CForm::Ishisuiryoukei => Some(9),
        // 連体形:3
        CForm::RentaikeiGeneral => Some(3),
        CForm::RentaikeiHojo => Some(3),
        CForm::RentaikeiShouryaku => Some(3),
        CForm::RentaikeiSokuonbin => Some(3),
        CForm::RentaikeiIonbin => Some(3),
        CForm::RentaikeiUonbin => Some(3),
        CForm::RentaikeiHatsuonbin => Some(3),
        // 終止形:10
        CForm::ShuushikeiGeneral => Some(10),
        CForm::ShuushikeiSokuonbin => Some(10),
        CForm::ShuushikeiHatsuonbin => Some(10),
        CForm::ShuushikeiFusion => Some(10),
        CForm::ShuushikeiHojo => Some(10),
        // 語幹:11
        CForm::GokanGeneral => Some(11),
        CForm::GokanSa => Some(11),

        // 連用形:1
        CForm::RenyoukeiGeneral => Some(1),
        CForm::RenyoukeiFusion => Some(1),
        CForm::RenyoukeiTo => Some(1),
        CForm::RenyoukeiNi => Some(1),
        CForm::RenyoukeiHatsuonbin => Some(1),
        CForm::RenyoukeiHojo => Some(1),
        CForm::RenyoukeiShouryaku => Some(1),
        CForm::RenyoukeiUonbin => Some(1),
        CForm::RenyoukeiIonbin => Some(1),
        CForm::RenyoukeiSokuonbin => Some(1),



    }
}
