extern crate phf;

mod constants;
mod is_romaji;
mod is_japanese;
mod is_kana;
mod util;

pub use is_japanese::is_japanese;
pub use is_kana::is_kana;
pub use is_romaji::is_romaji;
