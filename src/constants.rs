use std::ops::Range;

type Ranges = &'static [Range<u32>];

macro_rules! range {
    ($start:expr, $end:expr) => {
        Range { start: $start, end: ($end + 1), }
    }
}

macro_rules! range_const {
    ($name:ident, $start:expr, $end:expr) => {
        const $name: Range<u32> = range!($start, $end);
    }
}

macro_rules! u32_const {
    ($name:ident, $value:expr) => {
        pub(crate) const $name: u32 = $value;
    }
}

// CharCode References
// http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml
// http://unicode-table.com
range_const!(CJK_SYMBOLS_PUNCTUATION, 0x3000, 0x303F);
range_const!(KATAKANA_PUNCTUATION, 0x30FB, 0x30FC);
range_const!(HIRAGANA_CHARS, 0x3040, 0x309F);
range_const!(KATAKANA_CHARS, 0x30A0, 0x30FF);
range_const!(ZENKAKU_NUMBERS, 0xFF10, 0xFF19);
range_const!(ZENKAKU_PUNCTUATION_1, 0xFF01, 0xFF0F);
range_const!(ZENKAKU_PUNCTUATION_2, 0xFF1A, 0xFF1F);
range_const!(ZENKAKU_PUNCTUATION_3, 0xFF3B, 0xFF3F);
range_const!(ZENKAKU_PUNCTUATION_4, 0xFF5B, 0xFF60);
range_const!(ZENKAKU_SYMBOLS_CURRENCY, 0xFFE0, 0xFFEE);
range_const!(KANA_PUNCTUATION, 0xFF61, 0xFF65);
range_const!(HANKAKU_KATAKANA, 0xFF66, 0xFF9F);
range_const!(COMMON_CJK, 0x4E00, 0x9FFF);
range_const!(RARE_CJK, 0x3400, 0x4DBF);
range_const!(LATIN_NUMBERS, 0x0030, 0x0039);
range_const!(MODERN_ENGLISH, 0x0000, 0x007f);

const SMART_QUOTE_RANGES: Ranges = &[
    range!(0x2018, 0x2019), // ‘ ’
    range!(0x201C, 0x201D), // “ ”
];

pub(crate) const JA_PUNCTUATION_RANGES: Ranges = &[
    CJK_SYMBOLS_PUNCTUATION,
    KANA_PUNCTUATION,
    KATAKANA_PUNCTUATION,
    ZENKAKU_PUNCTUATION_1,
    ZENKAKU_PUNCTUATION_2,
    ZENKAKU_PUNCTUATION_3,
    ZENKAKU_PUNCTUATION_4,
    ZENKAKU_SYMBOLS_CURRENCY,
];

const KANA_RANGES: Ranges = &[
    HIRAGANA_CHARS,
    KATAKANA_CHARS,
    KANA_PUNCTUATION,
    HANKAKU_KATAKANA,
];

/// All Japanese unicode start and end ranges.
///
/// Includes full-width punctuation and number ranges.
///
/// Incudes latin numbers since they are used in Japanese text as well.
pub(crate) const JAPANESE_RANGES: Ranges = &[
    // KANA_RANGES
    HIRAGANA_CHARS,
    KATAKANA_CHARS,
    KANA_PUNCTUATION,
    HANKAKU_KATAKANA,

    // JA_PUNCTUATION_RANGES
    CJK_SYMBOLS_PUNCTUATION,
    KANA_PUNCTUATION,
    KATAKANA_PUNCTUATION,
    ZENKAKU_PUNCTUATION_1,
    ZENKAKU_PUNCTUATION_2,
    ZENKAKU_PUNCTUATION_3,
    ZENKAKU_PUNCTUATION_4,
    ZENKAKU_SYMBOLS_CURRENCY,

    // Other
    LATIN_NUMBERS,
    ZENKAKU_NUMBERS,
    COMMON_CJK,
    RARE_CJK,
];

/// Basic Latin unicode regex, for determining Romaji + Hepburn romanisation
///
/// Includes upper/lowercase long vowels like "ā, ī, ū, ē, ō"
///
/// Includes smart quotes ‘’ “”
pub(crate) const ROMAJI_RANGES: Ranges = &[
    MODERN_ENGLISH,

    // HEPBURN_MACRON_RANGES
    range!(0x0100, 0x0101), // Ā ā
    range!(0x0112, 0x0113), // Ē ē
    range!(0x012a, 0x012b), // Ī ī
    range!(0x014c, 0x014d), // Ō ō
    range!(0x016a, 0x016b), // Ū ū

    // SMART_QUOTE_RANGES
    range!(0x2018, 0x2019), // ‘ ’
    range!(0x201C, 0x201D), // “ ”
];

pub(crate) const EN_PUNCTUATION_RANGES: Ranges = &[
    range!(0x21, 0x2F),
    range!(0x3A, 0x3F),
    range!(0x5B, 0x60),
    range!(0x7B, 0x7E),

    // SMART_QUOTE_RANGES
    range!(0x2018, 0x2019), // ‘ ’
    range!(0x201C, 0x201D), // “ ”
];

u32_const!(LOWERCASE_START, 0x61);
u32_const!(LOWERCASE_END, 0x7A);
u32_const!(UPPERCASE_START, 0x41);
u32_const!(UPPERCASE_END, 0x5A);
u32_const!(LOWERCASE_FULLWIDTH_START, 0xFF41);
u32_const!(LOWERCASE_FULLWIDTH_END, 0xFF5A);
u32_const!(UPPERCASE_FULLWIDTH_START, 0xFF21);
u32_const!(UPPERCASE_FULLWIDTH_END, 0xFF3A);
u32_const!(HIRAGANA_START, 0x3041);
u32_const!(HIRAGANA_END, 0x3096);
u32_const!(KATAKANA_START, 0x30A1);
u32_const!(KATAKANA_END, 0x30FC);
u32_const!(KANJI_START, 0x4E00);
u32_const!(KANJI_END, 0x9FAF);
u32_const!(PROLONGED_SOUND_MARK, 0x30FC);
u32_const!(KANA_SLASH_DOT, 0x30FB);

pub(crate) const FOUR_CHAR_EDGECASES: &[&'static str] = &["lts", "chy", "shy"];

include!(concat!(env!("OUT_DIR"), "/constants.rs"));
