use constants::KANA_RANGES;
use util::is_char_in_ranges;

/// /// Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji),
/// [Kana](https://en.wikipedia.org/wiki/Kana), zenkaku punctuation, japanese symbols and numbers.”
///
/// ```rust
/// # use wanakana::is_kana;
/// assert!(is_kana("あ"));
/// assert!(is_kana("ア"));
/// assert!(is_kana("あーア"));
/// assert!(!is_kana("A"));
/// assert!(!is_kana("あAア"));
/// ```
pub fn is_kana(input: &str) -> bool {
    input.chars().all(|c| is_char_in_ranges(KANA_RANGES, c))
}
