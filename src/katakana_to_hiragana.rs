use constants;
use is;

/// Convert [Katakana](https://en.wikipedia.org/wiki/Katakana) to
/// [Hiragana](https://en.wikipedia.org/wiki/Hiragana))
///
/// ```rust
/// # use wanakana::katakana_to_hiragana;
/// assert_eq!(katakana_to_hiragana("カタカナ"), "かたかな");
/// assert_eq!(
///     katakana_to_hiragana("カタカナ is a type of kana"),
///     "かたかな is a type of kana"
/// );
/// ```
pub fn katakana_to_hiragana(input: &str) -> String {
    // TODO: This is mostly a 1-to-1 port, can probably be improved
    let mut hira = String::with_capacity(input.len());

    let mut previous_kana: Option<char> = None;

    for (index, c) in input.chars().enumerate() {
        let is_long_dash = (c as u32) == constants::PROLONGED_SOUND_MARK;

        if (c as u32) == constants::KANA_SLASH_DOT || (is_long_dash && index < 1) {
            hira.push(c);
        } else if previous_kana.is_some() && is_long_dash && index > 0 {
            // Transform long vowels: 'オー' to 'おう'
            // Transform previous_kana back to romaji, and slice off the vowel
            let long_vowel = previous_kana
                .and_then(|kana| {
                    constants::TO_ROMAJI.get::<str>(kana.to_string().as_ref())
                })
                .and_then(|romaji| romaji.chars().rev().next())
                .and_then(|vowel| {
                    constants::LONG_VOWELS.get::<str>(vowel.to_string().as_ref())
                });

            if let Some(vowel) = long_vowel {
                hira.push_str(vowel);
            }
        } else if !is_long_dash && is::char_is_katakana(c) {
            // Shift charcode.
            let code = c as u32 + constants::HIRAGANA_START - constants::KATAKANA_START;

            if let Some(hira_char) = ::std::char::from_u32(code) {
                hira.push(hira_char);
                previous_kana = Some(hira_char);
            }
        } else {
            // Pass non katakana chars through
            hira.push(c);
            previous_kana = None;
        }
    }

    hira
}
