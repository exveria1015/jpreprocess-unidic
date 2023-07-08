use std::borrow::Cow;

use jpreprocess_dictionary::{Dictionary, JPreprocessDictionary};

#[cfg(feature = "naist-jdic")]
const WORDS_IDX_DATA: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/jpreprocess.wordsidx"));
#[cfg(not(feature = "naist-jdic"))]
const WORDS_IDX_DATA: &[u8] = &[];

#[cfg(feature = "naist-jdic")]
const WORDS_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/jpreprocess.words"));
#[cfg(not(feature = "naist-jdic"))]
const WORDS_DATA: &[u8] = &[];

pub fn load_dictionary() -> JPreprocessDictionary {
    Dictionary::load_bin(words_data(), words_idx_data()).into()
}

pub fn words_idx_data() -> Cow<'static, [u8]> {
    Cow::Borrowed(WORDS_IDX_DATA)
}

pub fn words_data() -> Cow<'static, [u8]> {
    Cow::Borrowed(WORDS_DATA)
}
