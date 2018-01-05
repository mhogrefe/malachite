use error::ParseIntegerError;
use gmp_mpfr_sys::gmp;
use integer::Integer;
use malachite_base::traits::Zero;
use natural::Natural;
use natural::Natural::*;
use std::ffi::CString;
use std::fmt::{self, Debug, Display, Formatter};
use std::os::raw::{c_char, c_int};
use std::str::FromStr;

impl Natural {
    //TODO test
    pub fn assign_str_radix(&mut self, src: &str, radix: i32) -> Result<(), ParseIntegerError> {
        assert!(!src.starts_with('-'));
        let s = check_str_radix(src, radix)?;
        let c_str = CString::new(s).unwrap();
        let mut x = Integer::new_mpz_t();
        let err = unsafe { gmp::mpz_set_str(&mut x, c_str.as_ptr(), radix) };
        assert_eq!(err, 0);
        *self = Large(x);
        self.demote_if_small();
        Ok(())
    }

    //TODO test
    pub fn from_str_radix(src: &str, radix: i32) -> Result<Natural, ParseIntegerError> {
        let mut i = Natural::ZERO;
        i.assign_str_radix(src, radix)?;
        Ok(i)
    }

    //TODO test
    pub fn assign_str(&mut self, src: &str) -> Result<(), ParseIntegerError> {
        self.assign_str_radix(src, 10)
    }
}

fn check_str_radix(src: &str, radix: i32) -> Result<&str, ParseIntegerError> {
    use error::ParseIntegerError as Error;
    use error::ParseErrorKind as Kind;

    assert!(radix >= 2 && radix <= 36, "radix out of range");
    let (skip_plus, chars) = if src.starts_with('+') {
        (&src[1..], src[1..].chars())
    } else if src.starts_with('-') {
        (src, src[1..].chars())
    } else {
        (src, src.chars())
    };
    let mut got_digit = false;
    for c in chars {
        let digit_value = match c {
            '0'...'9' => c as i32 - '0' as i32,
            'a'...'z' => c as i32 - 'a' as i32 + 10,
            'A'...'Z' => c as i32 - 'A' as i32 + 10,
            _ => {
                return Err(Error {
                    kind: Kind::InvalidDigit,
                })
            }
        };
        if digit_value >= radix {
            return Err(Error {
                kind: Kind::InvalidDigit,
            });
        }
        got_digit = true;
    }
    if !got_digit {
        return Err(Error {
            kind: Kind::NoDigits,
        });
    }
    Ok(skip_plus)
}

//TODO test
impl FromStr for Natural {
    type Err = ParseIntegerError;

    fn from_str(src: &str) -> Result<Natural, ParseIntegerError> {
        let mut i = Natural::ZERO;
        i.assign_str(src)?;
        Ok(i)
    }
}

fn make_string(i: &Natural, radix: i32, to_upper: bool) -> String {
    assert!(radix >= 2 && radix <= 36, "radix out of range");
    match *i {
        Small(_) => make_string(&i.promote(), radix, to_upper),
        Large(x) => {
            let size = unsafe { gmp::mpz_sizeinbase(&x, radix) };
            // size + 2 for '-' and nul
            let size = size.checked_add(2).unwrap();
            let mut buf = Vec::<u8>::with_capacity(size);
            let case_radix = if to_upper { -radix } else { radix };
            unsafe {
                buf.set_len(size);
                gmp::mpz_get_str(buf.as_mut_ptr() as *mut c_char, case_radix as c_int, &x);
                let nul_index = buf.iter().position(|&x| x == 0).unwrap();
                buf.set_len(nul_index);
                String::from_utf8_unchecked(buf)
            }
        }
    }
}

fn fmt_radix(
    i: &Natural,
    f: &mut Formatter,
    radix: i32,
    to_upper: bool,
    prefix: &str,
) -> fmt::Result {
    let s = make_string(i, radix, to_upper);
    let (neg, buf) = if s.starts_with('-') {
        (true, &s[1..])
    } else {
        (false, &s[..])
    };
    f.pad_integral(!neg, prefix, buf)
}

//TODO test
impl Display for Natural {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

//TODO test
impl Debug for Natural {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

pub mod assign;
pub mod assign_u32;
pub mod assign_u64;
pub mod clone;
pub mod from_u32;
pub mod from_u64;
pub mod to_integer;
pub mod to_u32;
pub mod to_u64;
