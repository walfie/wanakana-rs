use constants;
use is;

/// Convert [Hiragana](https://en.wikipedia.org/wiki/Hiragana) to
/// [Katakana](https://en.wikipedia.org/wiki/Katakana)
///
/// ```rust
/// # use wanakana::hiragana_to_katakana;
/// assert_eq!(hiragana_to_katakana("ひらがな"), "ヒラガナ");
/// assert_eq!(
///     hiragana_to_katakana("ひらがな is a type of kana"),
///     "ヒラガナ is a type of kana"
/// );
/// ```
pub fn hiragana_to_katakana(input: &str) -> String {
    // TODO: This is mostly a 1-to-1 port, can probably be improved
    let mut kata = String::new();

    for c in input.chars() {
        if (c as u32) == constants::PROLONGED_SOUND_MARK ||
            (c as u32) == constants::KANA_SLASH_DOT
        {
            kata.push(c);
        } else if is::char_is_hiragana(c) {
            // Shift charcode.
            let code = c as u32 + constants::KATAKANA_START - constants::HIRAGANA_START;

            if let Some(kata_char) = ::std::char::from_u32(code) {
                kata.push(kata_char);
            }
        } else {
            // Pass non-hiragana chars through
            kata.push(c)
        }
    }

    kata
}
