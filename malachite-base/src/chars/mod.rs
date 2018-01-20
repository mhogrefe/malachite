use num::Walkable;
use std::char;

//TODO doc and test everything

pub const NUMBER_OF_CHARS: u32 = 0x10_f800;

pub fn char_to_contiguous_range(c: char) -> u32 {
    if c <= '\u{d7ff}' {
        c as u32
    } else {
        c as u32 - 2048
    }
}

pub fn contiguous_range_to_char(i: u32) -> Option<char> {
    if i <= 0xd7ff {
        char::from_u32(i)
    } else if i < NUMBER_OF_CHARS {
        char::from_u32(i + 2048)
    } else {
        None
    }
}

impl Walkable for char {
    fn increment(&mut self) {
        *self = contiguous_range_to_char(char_to_contiguous_range(*self) + 1).unwrap()
    }

    fn decrement(&mut self) {
        *self = contiguous_range_to_char(char_to_contiguous_range(*self) - 1).unwrap()
    }
}
