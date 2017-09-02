use std::ops::Range;

pub(crate) fn is_char_in_ranges(ranges: &[Range<u32>], c: char) -> bool {
    ranges.iter().any(|range| {
        range.start <= (c as u32) && (c as u32) <= range.end
    })
}
