use constants;
use std::ops::Range;

fn is_between(c: char, lower: u32, upper: u32) -> bool {
    lower <= (c as u32) && (c as u32) <= upper
}

pub(crate) fn is_in_ranges(c: char, ranges: &[Range<u32>]) -> bool {
    ranges.iter().any(|range| is_in_range(c, range))
}

fn is_in_range(c: char, range: &Range<u32>) -> bool {
    is_between(c, range.start, range.end)
}

pub(crate) trait CharExt {
    fn is_hiragana(self) -> bool;
    fn is_katakana(self) -> bool;
    fn is_kana(self) -> bool;
    fn is_romaji(self) -> bool;
    fn is_kanji(self) -> bool;
}

impl CharExt for char {
    fn is_hiragana(self) -> bool {
        (self as u32) == constants::PROLONGED_SOUND_MARK ||
            is_between(self, constants::HIRAGANA_START, constants::HIRAGANA_END)
    }

    fn is_katakana(self) -> bool {
        is_between(self, constants::KATAKANA_START, constants::KATAKANA_END)
    }

    fn is_kana(self) -> bool {
        is_in_ranges(self, constants::KANA_RANGES)
    }

    fn is_romaji(self) -> bool {
        is_in_ranges(self, constants::ROMAJI_RANGES)
    }

    fn is_kanji(self) -> bool {
        is_between(self, constants::KANJI_START, constants::KANJI_END)
    }
}
