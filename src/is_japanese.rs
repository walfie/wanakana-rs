use constants::JAPANESE_RANGES;
use util::is_char_in_ranges;

/// Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji),
/// [Kana](https://en.wikipedia.org/wiki/Kana), zenkaku punctuation, japanese symbols and numbers.”
///
/// ```rust
/// # use wanakana::is_japanese;
/// assert!(is_japanese("泣き虫"));
/// assert!(is_japanese("あア"));
/// assert!(is_japanese("２月1日")); // Full and half-width numbers allowed
/// assert!(is_japanese("泣き虫。！〜＄"));
/// assert!(!is_japanese("泣き虫.!~$")); // Half-width / Latin punctuation fails
/// assert!(!is_japanese("A泣き虫"));
/// assert!(!is_japanese("A"));
/// ```
pub fn is_japanese(input: &str) -> bool {
    input.chars().all(|c| is_char_in_ranges(JAPANESE_RANGES, c))
}
