use constants;
use std::ops::Range;

fn is_char_in_range(c: char, range: &Range<u32>) -> bool {
    is_char_between(c, range.start, range.end)
}

fn is_char_between(c: char, lower: u32, upper: u32) -> bool {
    lower <= (c as u32) && (c as u32) <= upper
}

fn is_char_in_ranges(c: char, ranges: &[Range<u32>]) -> bool {
    ranges.iter().any(|range| is_char_in_range(c, range))
}

fn all_in_ranges(input: &str, ranges: &[Range<u32>]) -> bool {
    input.chars().all(|c| is_char_in_ranges(c, ranges))
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
    all_in_ranges(input, constants::ROMAJI_RANGES)
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
    all_in_ranges(input, constants::JAPANESE_RANGES)
}

/// Test if `input` is [Kana](https://en.wikipedia.org/wiki/Kana)
/// ([Katakana](https://en.wikipedia.org/wiki/Katakana) and/or
/// [Hiragana](https://en.wikipedia.org/wiki/Hiragana))
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
    all_in_ranges(input, constants::KANA_RANGES)
}

/// Test if `input` is [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
///
/// ```rust
/// # use wanakana::is_hiragana;
/// assert!(is_hiragana("げーむ"));
/// assert!(!is_hiragana("A"));
/// assert!(!is_hiragana("あア"));
/// ```
pub fn is_hiragana(input: &str) -> bool {
    input.chars().all(|c| {
        (c as u32) == constants::PROLONGED_SOUND_MARK ||
            is_char_between(c, constants::HIRAGANA_START, constants::HIRAGANA_END)
    })
}

/// Test if `input` is [Katakana](https://en.wikipedia.org/wiki/Katakana))
///
/// ```rust
/// # use wanakana::is_katakana;
/// assert!(is_katakana("ゲーム"));
/// assert!(!is_katakana("あ"));
/// assert!(!is_katakana("A"));
/// assert!(!is_katakana("あア"));
/// ```
pub fn is_katakana(input: &str) -> bool {
    input.chars().all(|c| {
        is_char_between(c, constants::KATAKANA_START, constants::KATAKANA_END)
    })
}
