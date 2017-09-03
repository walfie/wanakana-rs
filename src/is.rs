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

fn char_is_romaji(c: char) -> bool {
    is_char_in_ranges(c, constants::ROMAJI_RANGES)
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

fn char_is_kana(c: char) -> bool {
    is_char_in_ranges(c, constants::KANA_RANGES)
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
    input.chars().all(char_is_hiragana)
}

pub(crate) fn char_is_hiragana(c: char) -> bool {
    (c as u32) == constants::PROLONGED_SOUND_MARK ||
        is_char_between(c, constants::HIRAGANA_START, constants::HIRAGANA_END)
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
    input.chars().all(char_is_katakana)
}

pub(crate) fn char_is_katakana(c: char) -> bool {
    is_char_between(c, constants::KATAKANA_START, constants::KATAKANA_END)
}

/// Tests if `input` is [Kanji](https://en.wikipedia.org/wiki/Kanji) ([Japanese CJK
/// ideographs](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs))
///
/// ```rust
/// # use wanakana::is_kanji;
/// assert!(is_kanji("刀"));
/// assert!(is_kanji("切腹"));
/// assert!(!is_kanji("勢い"));
/// assert!(!is_kanji("あAア"));
/// assert!(!is_kanji("🦀"));
/// ```
pub fn is_kanji(input: &str) -> bool {
    input.chars().all(char_is_kanji)
}

fn char_is_kanji(c: char) -> bool {
    is_char_between(c, constants::KANJI_START, constants::KANJI_END)
}

/// Test if `input` contains a mix of [Romaji](https://en.wikipedia.org/wiki/Romaji) *and*
/// [Kana](https://en.wikipedia.org/wiki/Kana), with an option to pass through
/// [Kanji](https://en.wikipedia.org/wiki/Kanji))
///
/// ```rust
/// # use wanakana::is_mixed;
/// assert!(is_mixed("Abあア", true));
/// assert!(is_mixed("お腹A", true));
/// assert!(!is_mixed("お腹A", false));
/// assert!(!is_mixed("ab", true));
/// assert!(!is_mixed("あア", true));
/// ```
pub fn is_mixed(input: &str, pass_kanji: bool) -> bool {
    // TODO: Could be more optimized. For now, this is using the same technique as the original.
    let has_kanji = if pass_kanji {
        false
    } else {
        input.chars().any(char_is_kanji)
    };

    input.chars().any(char_is_kana) && input.chars().any(char_is_romaji) && !has_kanji
}
