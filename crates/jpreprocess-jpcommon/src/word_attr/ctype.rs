use jpreprocess_core::ctype::CType;

pub fn ctype_to_id(ctype: &CType) -> Option<u8> {
    match ctype {
        // *:xx
        CType::None => None,
        // カ行変格:5
        CType::KaIrregular(_) => Some(5),
        // サ行変格:4
        CType::SaIrregular(_) => Some(4),
        // ラ行変格:6
        CType::RaIrregular => Some(6),
        // 一段:3
        CType::One(_) => Some(3),
        // 形容詞:7
        CType::Keiyoushi(_) => Some(7),
        // 五段:1
        CType::Five(_) => Some(1),
        // 四段:6
        CType::Four(_) => Some(6),
        // 助動詞:7
        CType::Special(_) => Some(7),
        // 二段:6
        CType::LowerTwo(_) => Some(6),
        CType::UpperTwo(_) => Some(6),
        // 不変化:6
        CType::NoConjugation => Some(6),
        // 文語助動詞:6
        CType::Old(_) => Some(6),

        //UniDic
        // カ行変格:5
        CType::KagyouHenkaku => Some(5),
        // サ行変格:4
        CType::SagyouHenkaku => Some(4),
        // 上一段-ア行:3
        CType::KamiichidanAGyou => Some(3),
        // 上一段-カ行:3
        CType::KamiichidanKAGyou => Some(3),
        // 上一段-ガ行:3
        CType::KamiichidanGAGyou => Some(3),
        // 上一段-ザ行:3
        CType::KamiichidanZaGyou => Some(3),
        // 上一段-タ行:3
        CType::KamiichidanTAGyou => Some(3),
        // 上一段-ナ行:3
        CType::KamiichidanNAGyou => Some(3),
        // 上一段-ハ行:3
        CType::KamiichidanHAGyou => Some(3),
        // 上一段-バ行:3
        CType::KamiichidanBAGyou => Some(3),
        // 上一段-マ行:3
        CType::KamiichidanMAGyou => Some(3),
        // 上一段-ラ行:3
        CType::KamiichidanRAGyou => Some(3),
        // 下一段-ア行:6
        CType::ShimoIchidanAGyou => Some(6),
        // 下一段-カ行:6
        CType::ShimoIchidanKAGyou => Some(6),
        // 下一段-ガ行:6
        CType::ShimoIchidanGAGyou => Some(6),
        // 下一段-サ行:6
        CType::ShimoIchidanSAGyou => Some(6),
        // 下一段-ザ行:6
        CType::ShimoIchidanZAGyou => Some(6),
        // 下一段-タ行:6
        CType::ShimoIchidanTAGyou => Some(6),
        // 下一段-ダ行:6
        CType::ShimoIchidanDAGyou => Some(6),
        // 下一段-ナ行:6
        CType::ShimoIchidanNAGyou => Some(6),
        // 下一段-ハ行:6
        CType::ShimoIchidanHAGyou => Some(6),
        // 下一段-バ行:6
        CType::ShimoIchidanBAGyou => Some(6),
        // 下一段-マ行:6
        CType::ShimoIchidanMAGyou => Some(6),
        // 下一段-ラ行:6
        CType::ShimoIchidanRAGyou => Some(6),
        // 五段-カ行:1
        CType::GodanKAGyou => Some(1),
        // 五段-ガ行:1
        CType::GodanGAGyou => Some(1),
        // 五段-サ行:1
        CType::GodanSAGyou => Some(1),
        // 五段-タ行:1
        CType::GodanTAGyou => Some(1),
        // 五段-ナ行:1
        CType::GodanNAGyou => Some(1),
        // 五段-バ行:1
        CType::GodanBAGyou => Some(1),
        // 五段-マ行:1
        CType::GodanMAGyou => Some(1),
        // 五段-ラ行:1
        CType::GodanRAGyou => Some(1),
        // 五段-ワア行:1
        CType::GodanWAGyou => Some(1),
        // 助動詞-ザマス:7
        CType::JodoushiZamasu => Some(7),
        // 助動詞-ジャ:7
        CType::JodoushiJa => Some(7),
        // 助動詞-タ:7
        CType::JodoushiTa => Some(7),
        // 助動詞-タイ:7
        CType::JodoushiTai => Some(7),
        // 助動詞-ダ:7
        CType::JodoushiDa => Some(7),
        // 助動詞-デス:7
        CType::JodoushiDesu => Some(7),
        // 助動詞-ドス:7
        CType::JodoushiDosu => Some(7),
        // 助動詞-ナイ:7
        CType::JodoushiNai => Some(7),
        // 助動詞-ナンダ:7
        CType::JodoushiNanda => Some(7),
        // 助動詞-ヌ:7
        CType::JodoushiNu => Some(7),
        // 助動詞-ヒン:7
        CType::JodoushiHin => Some(7),
        // 助動詞-ヘン:7
        CType::JodoushiHen => Some(7),
        // 助動詞-マイ:7
        CType::JodoushiMai => Some(7),
        // 助動詞-マス:7
        CType::JodoushiMasu => Some(7),
        // 助動詞-ヤ:7
        CType::JodoushiYa => Some(7),
        // 助動詞-ヤス:7
        CType::JodoushiYasu => Some(7),
        // 助動詞-ヤン:7
        CType::JodoushiYan => Some(7),
        // 助動詞-ラシイ:7
        CType::JodoushiRashii => Some(7),
        // 助動詞-レル:7
        CType::JodoushiReru => Some(7),
        // 助動詞-ンス:7
        CType::JodoushiNsu => Some(7),
        // 文語カ行変格:6
        CType::BungoKagyouHenkaku => Some(6),
        // 文語サ行変格:6
        CType::BungoSagyouHenkaku => Some(6),
        // 文語ラ行変格:6
        CType::BungoRagyouHenkaku => Some(6),
        // 文語上一段-マ行:6
        CType::BungoKamiichidanMAGyou => Some(6),
        // 文語上二段-タ行:6
        CType::BungoKamiNidanTAGyou => Some(6),
        // 文語上二段-バ行:6
        CType::BungoKamiNidanBAGyou => Some(6),
        // 文語上二段-ラ行:6
        CType::BungoKamiNidanRAGyou => Some(6),
        // 文語下二段-ア行:6
        CType::BungoShimoNidanAGyou => Some(6),
        // 文語下二段-カ行:6
        CType::BungoShimoNidanKAGyou => Some(6),
        // 文語下二段-ガ行:6
        CType::BungoShimoNidanGAGyou => Some(6),
        // 文語下二段-サ行:6
        CType::BungoShimoNidanSAGyou => Some(6),
        // 文語下二段-タ行:6
        CType::BungoShimoNidanTAGyou => Some(6),
        // 文語下二段-ダ行:6
        CType::BungoShimoNidanDAGyou => Some(6),
        // 文語下二段-ナ行:6
        CType::BungoShimoNidanNAGyou => Some(6),
        // 文語下二段-ハ行:6
        CType::BungoShimoNidanHAGyou => Some(6),
        // 文語下二段-マ行:6
        CType::BungoShimoNidanMAGyou => Some(6),
        // 文語下二段-ヤ行:6
        CType::BungoShimoNidanYAGyou => Some(6),
        // 文語下二段-ラ行:6
        CType::BungoShimoNidanRAGyou => Some(6),
        // 文語下二段-ワ行:6
        CType::BungoShimoNidanWAGyou => Some(6),
        // 文語助動詞-キ:6
        CType::BungoJodoushiKi => Some(6),
        // 文語助動詞-ケリ:6
        CType::BungoJodoushiKeri => Some(6),
        // 文語助動詞-ゴトシ:6
        CType::BungoJodoushiGotoshi => Some(6),
        // 文語助動詞-ズ:6
        CType::BungoJodoushiZu => Some(6),
        // 文語助動詞-タリ-完了
        CType::BungoJodoushiTariKanryou => Some(6),
        // 文語助動詞-タリ-断定
        CType::BungoJodoushiTariDantei => Some(6),
        // 文語助動詞-ナリ-断定
        CType::BungoJodoushiNariDantei => Some(6),
        // 文語助動詞-ベシ:6
        CType::BungoJodoushiBeshi => Some(6),
        // 文語助動詞-マジ:6
        CType::BungoJodoushiMaji => Some(6),
        // 文語助動詞-ム:6
        CType::BungoJodoushiMu => Some(6),
        // 文語助動詞-リ:6
        CType::BungoJodoushiRi => Some(6),
        // 文語文語四段-カ行:6
        CType::BungoYodanKAGyou => Some(6),
        // 文語文語四段-ガ行:6
        CType::BungoYodanGAGyou => Some(6),
        // 文語文語四段-サ行:6
        CType::BungoYodanSAGyou => Some(6),
        // 文語文語四段-タ行:6
        CType::BungoYodanTAGyou => Some(6),
        // 文語文語四段-ハ行:6
        CType::BungoYodanHAGyou => Some(6),
        // 文語文語四段-バ行:6
        CType::BungoYodanBAGyou => Some(6),
        // 文語文語四段-マ行:6
        CType::BungoYodanMAGyou => Some(6),
        // 文語文語四段-ラ行:6
        CType::BungoYodanRAGyou => Some(6),
        // 文語形容詞-ク:6
        CType::BungoKeiyoushiKu => Some(6),
        // 文語形容詞-シク:6
        CType::BungoKeiyoushiShiku => Some(6),
        // 無変化型:6
        CType::Muhenkakei => Some(6),
        // 特殊型:6
        CType::SpecialType => Some(6),















    }
}
