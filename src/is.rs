use char_ext::{self, CharExt};
use constants;
use std::ops::Range;

fn all_in_ranges(input: &str, ranges: &[Range<u32>]) -> bool {
    input.chars().all(|c| char_ext::is_in_ranges(c, ranges))
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
    input.chars().all(CharExt::is_hiragana)
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
    input.chars().all(CharExt::is_katakana)
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
    input.chars().all(CharExt::is_kanji)
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
        input.chars().any(CharExt::is_kanji)
    };

    input.chars().any(CharExt::is_kana) && input.chars().any(CharExt::is_romaji) && !has_kanji
}
