use constants::{JAPANESE_RANGES, KANA_RANGES, ROMAJI_RANGES};
use std::ops::Range;

fn is_char_in_ranges(ranges: &[Range<u32>], c: char) -> bool {
    ranges.iter().any(|range| {
        range.start <= (c as u32) && (c as u32) <= range.end
    })
}

fn match_all(ranges: &[Range<u32>], input: &str) -> bool {
    input.chars().all(|c| is_char_in_ranges(ranges, c))
}

/// Test if `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn
/// romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
///
/// ```rust
/// # use wanakana::is_romaji;
/// assert!(is_romaji("Tōkyō and Ōsaka"));
/// assert!(is_romaji("12a*b&c-d"));
/// assert!(!is_romaji("あアA"));
/// assert!(!is_romaji("お願い"));
/// assert!(!is_romaji("a！b&cーd")); // Full-width punctuation fails
/// ```
///
pub fn is_romaji(input: &str) -> bool {
    match_all(ROMAJI_RANGES, input)
}

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
    match_all(JAPANESE_RANGES, input)
}

/// Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji),
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
    match_all(KANA_RANGES, input)
}
