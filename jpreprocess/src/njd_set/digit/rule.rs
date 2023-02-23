use phf::{phf_map, Map};

pub const HAIHUN1:&str="―"     /* horizontal bar */;
pub const HAIHUN2:&str="−"     /* minus sign */;
pub const HAIHUN3:&str="‐"     /* hyphen */;
pub const HAIHUN4:&str="—"     /* em dash */;
pub const HAIHUN5:&str="－"     /* fullwidth hyphen-minus */;
pub const KAKKO1: &str = "（";
pub const KAKKO2: &str = "）";
pub const BANGOU: &str = "番号";
pub const COMMA: &str = "，";
pub const TEN1: &str = "．";
pub const TEN2: &str = "・";
pub const TEN_FEATURE: &str = "．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,-1";
pub const ZERO1: &str = "〇";
pub const ZERO2: &str = "０";
pub const ZERO_BEFORE_DP: &str = "レー";
pub const ZERO_AFTER_DP: &str = "ゼロ";
pub const TWO: &str = "二";
pub const TWO_BEFORE_DP: &str = "ニー";
pub const TWO_AFTER_DP: &str = "ニー";
pub const FIVE: &str = "五";
pub const FIVE_BEFORE_DP: &str = "ゴー";
pub const FIVE_AFTER_DP: &str = "ゴー";
pub const SIX: &str = "六";
pub const NIN: &str = "人";
pub const GATSU: &str = "月";
pub const NICHI: &str = "日";
pub const NICHIKAN: &str = "日間";
pub const ONE: &str = "一";
pub const TSUITACHI: &str = "一日,名詞,副詞可能,*,*,*,*,一日,ツイタチ,ツイタチ,4/4,*";
pub const FOUR: &str = "四";
pub const TEN: &str = "十";
pub const JUYOKKA: &str = "十四日,名詞,副詞可能,*,*,*,*,十四日,ジュウヨッカ,ジューヨッカ,1/5,*";
pub const JUYOKKAKAN: &str =
    "十四日間,名詞,副詞可能,*,*,*,*,十四日間,ジュウヨッカカン,ジューヨッカカン,5/7,*";
pub const NIJU: &str = "二十,名詞,副詞可能,*,*,*,*,二十,ニジュウ,ニジュー,1/3,*";
pub const YOKKA: &str = "四日,名詞,副詞可能,*,*,*,*,四日,ヨッカ,ヨッカ,0/3,*,0";
pub const YOKKAKAN: &str = "四日間,名詞,副詞可能,*,*,*,*,四日間,ヨッカカン,ヨッカカン,3/5,*,0";
pub const HATSUKA: &str = "二十日,名詞,副詞可能,*,*,*,*,二十日,ハツカ,ハツカ,0/3,*";
pub const HATSUKAKAN: &str = "二十日間,名詞,副詞可能,*,*,*,*,二十日間,ハツカカン,ハツカカン,3/5,*";

pub const NUMERAL_LIST1: Map<&'static str, &'static str> = phf_map! {
   "○" => "〇",
   "１" => "一",
   "２" => "二",
   "３" => "三",
   "４" => "四",
   "５" => "五",
   "６" => "六",
   "７" => "七",
   "８" => "八",
   "９" => "九",
   "一" => "一",
   "二" => "二",
   "三" => "三",
   "四" => "四",
   "五" => "五",
   "六" => "六",
   "七" => "七",
   "八" => "八",
   "九" => "九",
   "いち" => "一",
   "に" => "二",
   "さん" => "三",
   "よん" => "四",
   "ご" => "五",
   "ろく" => "六",
   "なな" => "七",
   "はち" => "八",
   "きゅう" => "九",
   "〇" => "〇",
   "０" => "０",
   "壱" => "一",
   "弐" => "二",
   "貳" => "二",
   "ニ" => "二",
   "参" => "三",
   "し" => "四",
   "しち" => "七",
   "く" => "九"
};

pub const DIGITS: Map<&'static str, u8> = phf_map! {
   "一" => 1,
   "二" => 2,
   "三" => 3,
   "四" => 4,
   "五" => 5,
   "六" => 6,
   "七" => 7,
   "八" => 8,
   "九" => 9,
   "〇" => 0,
   "０" => 0,
};

pub const NUMERAL_LIST2: &[&str] = &[
    "",
    "十,名詞,数,*,*,*,*,十,ジュウ,ジュー,1/2,*",
    "百,名詞,数,*,*,*,*,百,ヒャク,ヒャク,2/2,*",
    "千,名詞,数,*,*,*,*,千,セン,セン,1/2,*",
];

pub const NUMERAL_LIST3: &[&str] = &[
    "",
    "万,名詞,数,*,*,*,*,万,マン,マン,1/2,*",
    "億,名詞,数,*,*,*,*,億,オク,オク,1/2,*",
    "兆,名詞,数,*,*,*,*,兆,チョウ,チョー,1/2,C3",
    "京,名詞,数,*,*,*,*,京,ケイ,ケー,1/2,*",
    "垓,名詞,数,*,*,*,*,垓,ガイ,ガイ,1/2,*",
    "𥝱,名詞,数,*,*,*,*,𥝱,ジョ,ジョ,1/1,*",
    "穣,名詞,数,*,*,*,*,穣,ジョウ,ジョー,1/2,*",
    "溝,名詞,数,*,*,*,*,溝,コウ,コウ,1/2,*",
    "澗,名詞,数,*,*,*,*,澗,カン,カン,1/2,*",
    "正,名詞,数,*,*,*,*,正,セイ,セー,1/2,*",
    "載,名詞,数,*,*,*,*,載,サイ,サイ,1/2,*",
    "極,名詞,数,*,*,*,*,極,ゴク,ゴク,1/2,*",
    "恒河沙,名詞,数,*,*,*,*,恒河沙,ゴウガシャ,ゴウガシャ,1/4,*",
    "阿僧祇,名詞,数,*,*,*,*,阿僧祇,アソウギ,アソーギ,2/4,*",
    "那由他,名詞,数,*,*,*,*,那由他,ナユタ,ナユタ,1/3,*",
    "不可思議,名詞,数,*,*,*,*,不可思議,フカシギ,フカシギ,2/4,*",
    "無量大数,名詞,数,*,*,*,*,無量大数,ムリョウタイスウ,ムリョータイスー,6/7,*",
];

pub const NUMERATIVE_CLASS3: &[(&str, &str)] = &[
    /* from paper */
    ("棟", "ムネ"),
    /* from dictionary */
    ("かけ", "カケ"),
    ("くだり", "クダリ"),
    ("けた", "ケタ"),
    ("すじ", "スジ"),
    ("そろい", "ソロイ"),
    ("たび", "タビ"),
    ("つかみ", "ツカミ"),
    ("つがい", "ツガイ"),
    ("つまみ", "ツマミ"),
    ("とおり", "トオリ"),
    ("ところ", "トコロ"),
    ("とせ", "トセ"),
    ("まわり", "マワリ"),
    ("シーズン", "シーズン"),
    ("セット", "セット"),
    ("握り", "ニギリ"),
    ("回り", "マワリ"),
    ("株", "カブ"),
    ("竿", "サオ"),
    ("筋", "スジ"),
    ("桁", "ケタ"),
    ("ケタ", "ケタ"),
    ("月", "ツキ"),
    ("言", "コト"),
    ("口", "クチ"),
    ("差し", "サシ"),
    ("皿", "サラ"),
    ("山", "ヤマ"),
    ("勺", "シャク"),
    ("尺", "シャク"),
    ("重ね", "カサネ"),
    ("振り", "フリ"),
    ("針", "ハリ"),
    ("切れ", "キレ"),
    ("束", "タバ"),
    ("続き", "ツヅキ"),
    ("揃", "ソロイ"),
    ("袋", "フクロ"),
    ("柱", "ハシラ"),
    ("張り", "ハリ"),
    ("通り", "トオリ"),
    ("掴み", "ツカミ"),
    ("坪", "ツボ"),
    ("箱", "ハコ"),
    ("鉢", "ハチ"),
    ("晩", "バン"),
    ("品", "シナ"),
    ("瓶", "ビン"),
    ("分け", "ワケ"),
    ("幕", "マク"),
    ("夜", "ヤ"),
    ("夜", "ヨ"),
    ("粒", "ツブ"),
    ("枠", "ワク"),
    ("棹", "サオ"),
    ("つ折", "ツオリ"),
    ("つ折り", "ツオリ"),
    ("粒", "ツブ"),
    ("つぶ", "ツブ"),
    ("とき", "トキ"),
    ("重ね", "ガサネ"),
];

pub const CONV_TABLE3: Map<&'static str, (&'static str, i32, i32)> = phf_map! {
   "一"=>("ヒト", 0, 2),
   "二"=> ("フタ", 0, 2),
   /* "三", "ミ", "1", "1", *//* modified */
};

pub const CONV_TABLE4: Map<&'static str, &'static str> = phf_map! {
   "一"=> "一人,名詞,副詞可能,*,*,*,*,一人,ヒトリ,ヒトリ,2/3,*",
   "二"=> "二人,名詞,副詞可能,*,*,*,*,二人,フタリ,フタリ,3/3,*",
};

pub const CONV_TABLE5: Map<&'static str, &'static str> = phf_map! {
   "一"=> "一日,名詞,副詞可能,*,*,*,*,一日,イチニチ,イチニチ,4/4,*",
   "二"=> "二日,名詞,副詞可能,*,*,*,*,二日,フツカ,フツカ,0/3,*",
   "三"=> "三日,名詞,副詞可能,*,*,*,*,三日,ミッカ,ミッカ,0/3,*",
   "四"=> "四日,名詞,副詞可能,*,*,*,*,四日,ヨッカ,ヨッカ,0/3,*",
   "五"=> "五日,名詞,副詞可能,*,*,*,*,五日,イツカ,イツカ,0/3,*",
   "六"=> "六日,名詞,副詞可能,*,*,*,*,六日,ムイカ,ムイカ,0/3,*",
   "七"=> "七日,名詞,副詞可能,*,*,*,*,七日,ナノカ,ナノカ,0/3,*",
   "八"=> "八日,名詞,副詞可能,*,*,*,*,八日,ヨウカ,ヨーカ,0/3,*",
   "九"=> "九日,名詞,副詞可能,*,*,*,*,九日,ココノカ,ココノカ,0/4,*",
   "十"=> "十日,名詞,副詞可能,*,*,*,*,十日,トウカ,トーカ,0/3,*",
};

pub const CONV_TABLE6: Map<&'static str, &'static str> = phf_map! {
   "一"=> "一日間,名詞,副詞可能,*,*,*,*,一日間,イチニチカン,イチニチカン,4/6,*",
   "二"=> "二日間,名詞,副詞可能,*,*,*,*,二日,フツカカン,フツカカン,3/5,*",
   "三"=> "三日間,名詞,副詞可能,*,*,*,*,三日,ミッカカン,ミッカカン,3/5,*",
   "四"=> "四日間,名詞,副詞可能,*,*,*,*,四日,ヨッカカン,ヨッカカン,3/5,*",
   "五"=> "五日間,名詞,副詞可能,*,*,*,*,五日,イツカカン,イツカカン,3/5,*",
   "六"=> "六日間,名詞,副詞可能,*,*,*,*,六日,ムイカカン,ムイカカン,3/5,*",
   "七"=> "七日間,名詞,副詞可能,*,*,*,*,七日,ナノカカン,ナノカカン,3/5,*",
   "八"=> "八日間,名詞,副詞可能,*,*,*,*,八日,ヨウカカン,ヨーカカン,3/5,*",
   "九"=> "九日間,名詞,副詞可能,*,*,*,*,九日,ココノカカン,ココノカカン,4/6,*",
   "十"=> "十日間,名詞,副詞可能,*,*,*,*,十日,トウカカン,トーカカン,3/5,*",
};