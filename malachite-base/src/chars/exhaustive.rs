use chars::crement::increment_char;
use chars::CharType;
use comparison::traits::Min;
use std::ops::RangeInclusive;

/// Generates all ASCII `char`s, in ascending order.
///
/// For a friendlier order (_e.g._ nonprintable `char`s coming last), try `exhaustive_ascii_char`s.
///
/// The output length is 128.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::exhaustive::ascii_chars_increasing;
///
/// assert_eq!(
///     ascii_chars_increasing().collect::<String>(),
///     "\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\n\u{b}\u{c}\r\u{e}\u{f}\u{10}\u{11}\u{12}\
///     \u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&\'()*\
///     +,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"
/// );
/// ```
pub const fn ascii_chars_increasing() -> RangeInclusive<char> {
    char::MIN..='\u{7f}'
}

/// Generates all `char`s, in ascending order.
///
/// For a friendlier order (_e.g_. nonprintable `char`s coming last), use `exhaustive_char`s.
///
/// The output length is 1,112,064.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::exhaustive::chars_increasing;
///
/// assert_eq!(
///     chars_increasing().take(200).collect::<String>(),
///     "\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\n\u{b}\u{c}\r\u{e}\u{f}\u{10}\u{11}\u{12}\
///     \u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&\'()*\
///     +,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}\
///     \u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\
///     \u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\
///     \u{9e}\u{9f}\u{a0}¡¢£¤¥¦§¨©ª«¬\u{ad}®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇ"
/// );
/// ```
pub const fn chars_increasing() -> RangeInclusive<char> {
    char::MIN..=char::MAX
}

/// Generates all `char`s, in a friendly order, so that more familiar characters come first.
///
/// The order is
/// 1. Lowercase ASCII letters
/// 2. Uppercase ASCII letters
/// 3. ASCII digits
/// 4. "Printable" ASCII characters (not alphanumeric and not control), including ' ' but no other
///     whitespace
/// 5. (only if `ascii_only` is false) "Printable" Non-ASCII characters; all non-ASCII characters
///     whose `Debug` representations don't start with '\\'
/// 6. All remaining characters.
///
/// This `struct` is created by the `exhaustive_chars` and `exhaustive_ascii_chars` functions. See
/// their documentation for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExhaustiveChars {
    ascii_only: bool,
    first: bool,
    c: char,
    current_type: CharType,
}

impl Iterator for ExhaustiveChars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.first {
            self.first = false;
        } else {
            match self.current_type {
                CharType::AsciiLower => {
                    if self.c == 'z' {
                        self.current_type = CharType::AsciiUpper;
                        self.c = 'A';
                    } else {
                        increment_char(&mut self.c);
                    }
                }
                CharType::AsciiUpper => {
                    if self.c == 'Z' {
                        self.current_type = CharType::AsciiNumeric;
                        self.c = '0';
                    } else {
                        increment_char(&mut self.c);
                    }
                }
                CharType::AsciiNumeric => {
                    if self.c == '9' {
                        self.current_type = CharType::AsciiNonAlphanumericGraphic;
                        self.c = ' ';
                    } else {
                        increment_char(&mut self.c);
                    }
                }
                CharType::AsciiNonAlphanumericGraphic => {
                    if self.c == '~' {
                        if self.ascii_only {
                            self.current_type = CharType::NonGraphic;
                            self.c = '\0';
                        } else {
                            self.current_type = CharType::NonAsciiGraphic;
                            self.c = '\u{a1}';
                        };
                    } else {
                        increment_char(&mut self.c);
                        // No control chars between ' ' and '~'
                        while self.c.is_ascii_alphanumeric() {
                            increment_char(&mut self.c);
                        }
                    }
                }
                CharType::NonAsciiGraphic => {
                    if self.c == '\u{3134a}' {
                        self.current_type = CharType::NonGraphic;
                        self.c = '\0';
                    } else {
                        increment_char(&mut self.c);
                        while !CharType::NonAsciiGraphic.contains(self.c) {
                            increment_char(&mut self.c);
                        }
                    }
                }
                CharType::NonGraphic => {
                    let limit = if self.ascii_only { '\u{7f}' } else { char::MAX };
                    if self.c == limit {
                        return None;
                    } else {
                        increment_char(&mut self.c);
                        while !self.c.is_ascii_control()
                            && (self.c.is_ascii() || CharType::NonAsciiGraphic.contains(self.c))
                        {
                            increment_char(&mut self.c);
                        }
                    }
                }
            }
        }
        Some(self.c)
    }
}

/// Generates all ASCII `char`s, in a friendly order, so that more familiar characters come first.
///
/// The order is
/// 1. Lowercase ASCII letters
/// 2. Uppercase ASCII letters
/// 3. ASCII digits
/// 4. "Printable" ASCII characters (not alphanumeric and not control), including ' ' but no other
///     whitespace
/// 5. All remaining ASCII characters.
///
/// Within each group, the characters are ordered according to their usual order.
///
/// The output length is 128.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
///
/// assert_eq!(
///     exhaustive_ascii_chars().collect::<String>(),
///     "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !\"#$%&\'()*+,-./:;<=>?@[\\\
///     ]^_`{|}~\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\n\u{b}\u{c}\r\u{e}\u{f}\u{10}\u{11}\
///     \u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f}\u{7f}"
/// );
/// ```
pub const fn exhaustive_ascii_chars() -> ExhaustiveChars {
    ExhaustiveChars {
        ascii_only: true,
        first: true,
        c: 'a',
        current_type: CharType::AsciiLower,
    }
}

/// Generates all `char`s, in a friendly order, so that more familiar characters come first.
///
/// The order is
/// 1. Lowercase ASCII letters
/// 2. Uppercase ASCII letters
/// 3. ASCII digits
/// 4. "Printable" ASCII characters (not alphanumeric and not control), including ' ' but no other
///     whitespace
/// 5. "Printable" Non-ASCII characters; all non-ASCII characters whose `Debug` representations
///     don't start with '\\'
/// 6. All remaining characters.
///
/// Within each group, the characters are ordered according to their usual order.
///
/// The output length is 1,112,064.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::exhaustive::exhaustive_chars;
///
/// assert_eq!(
///     exhaustive_chars().take(200).collect::<String>(),
///     "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !\"#$%&\'()*+,-./:;<=>?@[\\\
///     ]^_`{|}~¡¢£¤¥¦§¨©ª«¬®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóô\
///     õö÷øùúûüýþÿĀāĂăĄąĆćĈĉĊ"
/// );
/// ```
pub const fn exhaustive_chars() -> ExhaustiveChars {
    ExhaustiveChars {
        ascii_only: false,
        first: true,
        c: 'a',
        current_type: CharType::AsciiLower,
    }
}
