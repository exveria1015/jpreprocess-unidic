use crate::njd_set::digit::lut_conversion::*;
use phf::{phf_map, phf_set};

pub const DIGIT_CONVERSION_TABLE: [(Keys, DigitLUT); 2] = [
    (NUMERAL_LIST8, NUMERAL_LIST9),
    (NUMERAL_LIST10, NUMERAL_LIST11),
];
pub const NUMERATIVE_CONVERSION_TABLE: [(Keys, NumerativeLUT); 1] =
    [(NUMERAL_LIST6, NUMERAL_LIST7)];

pub const NUMERAL_LIST4: Keys = phf_set! {
  "一", "二", "三", "四", "五", "六", "七", "八", "九", "何", "幾", "数",
};

pub const NUMERAL_LIST5: Keys = phf_set! {
 "十", "百", "千", "万", "億", "兆", "京", "垓",
 "𥝱",
 "穣", "溝", "澗", "正", "載", "極",
 "恒河沙", "阿僧祇", "那由他", "不可思議", "無量大数",
};

const NUMERAL_LIST6: Keys = phf_set! {"百", "千"};

const NUMERAL_LIST7: NumerativeLUT = phf_map! {
 "三"=>1,
 "六"=>2,
 "八"=>2,
 "何"=>1,
};

const NUMERAL_LIST8: Keys = phf_set! {"百"};

const NUMERAL_LIST9: DigitLUT = phf_map! {
 "六"=> ("ロッ", 0, 2),
 "八"=>("ハッ", 0, 2),
};

const NUMERAL_LIST10: Keys = phf_set! {"千", "兆"};

const NUMERAL_LIST11: DigitLUT = phf_map! {
 "一"=> ("イッ", 0, 2),
 "八"=> ("ハッ", 0, 2),
 "十"=> ("ジュッ", 1, 2),
};