extern crate phf;

mod constants;
mod is_romaji;
mod is_japanese;
mod util;

pub use is_japanese::is_japanese;
pub use is_romaji::is_romaji;
