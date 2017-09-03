extern crate phf;

mod constants;
mod is;
mod char_ext;
mod katakana_to_hiragana;
mod hiragana_to_katakana;

pub use hiragana_to_katakana::hiragana_to_katakana;
pub use is::{is_hiragana, is_japanese, is_kana, is_kanji, is_katakana, is_mixed, is_romaji};
pub use katakana_to_hiragana::katakana_to_hiragana;
