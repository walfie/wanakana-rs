use constants::ROMAJI_RANGES;
use util::is_char_in_ranges;

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
    input.chars().all(|c| is_char_in_ranges(ROMAJI_RANGES, c))
}
