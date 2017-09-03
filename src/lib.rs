extern crate phf;

mod constants;
mod is;
mod katakana_to_hiragana;

pub use is::{is_hiragana, is_japanese, is_kana, is_kanji, is_katakana, is_mixed, is_romaji};
pub use katakana_to_hiragana::katakana_to_hiragana;
