use gmp_mpfr_sys::gmp::{self, mpz_t};
use error::ParseIntegerError;
use integer::Integer;
use integer::Integer::*;
use std::ffi::CString;
use std::fmt::{self, Debug, Display, Formatter};
use std::os::raw::{c_char, c_int, c_long};
use std::slice;
use std::str::FromStr;

//TODO remove
pub struct IntegerContent<'a> {
    x: &'a Integer,
    i: u64,
    mask: u32,
    length: u64,
}

impl<'a> IntegerContent<'a> {
    //TODO test
    pub fn new(x: &'a Integer) -> IntegerContent {
        IntegerContent {
            x: x,
            i: 0,
            mask: 1,
            length: x.significant_bits(),
        }
    }
}

//TODO remove
impl<'a> Iterator for IntegerContent<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut val: u32 = 0;
        while self.i < self.length {
            if self.x.get_bit(self.i) {
                val |= self.mask;
            }
            self.mask <<= 1;
            self.i += 1;
            if self.mask == 0 {
                val = 0;
                self.mask = 1;
                return Some(val);
            }
        }
        if val == 0 { None } else { Some(val) }
    }
}

impl Integer {
    //TODO test
    pub fn to_string_radix(&self, radix: i32) -> String {
        make_string(self, radix, false)
    }

    //TODO test
    pub fn assign_str_radix(&mut self, src: &str, radix: i32) -> Result<(), ParseIntegerError> {
        let s = check_str_radix(src, radix)?;
        let c_str = CString::new(s).unwrap();
        let mut x = Integer::new_mpz_t();
        let err = unsafe { gmp::mpz_set_str(&mut x, c_str.as_ptr(), radix.into()) };
        assert_eq!(err, 0);
        self.assign_mpz_t(x);
        Ok(())
    }

    //TODO test
    pub fn from_str_radix(src: &str, radix: i32) -> Result<Integer, ParseIntegerError> {
        let mut i = Integer::new();
        i.assign_str_radix(src, radix)?;
        Ok(i)
    }

    //TODO test
    pub fn assign_str(&mut self, src: &str) -> Result<(), ParseIntegerError> {
        self.assign_str_radix(src, 10)
    }

    //TODO test
    pub fn assign_bits_unsigned(&mut self, bits: &[bool]) {
        let bit_length = bits.len();
        let mut significant_bits = bit_length;
        while significant_bits != 0 && !bits[significant_bits - 1] {
            significant_bits -= 1;
        }
        if significant_bits < 32 {
            let mut x = 0;
            let mut mask = 1;
            for &bit in bits.iter().take(significant_bits) {
                if bit {
                    x |= mask;
                }
                mask <<= 1;
            }
            *self = Small(x);
            return;
        }
        let limb_bits = gmp::LIMB_BITS as usize;
        let whole_limbs = bit_length / limb_bits;
        let extra_bits = bit_length % limb_bits;
        // Avoid conditions and overflow, equivalent to:
        // let total_limbs = whole_limbs + if extra_bits == 0 { 0 } else { 1 };
        let total_limbs = whole_limbs + ((extra_bits + limb_bits - 1) / limb_bits) as usize;
        let mut x: mpz_t = Integer::new_mpz_t();
        let limbs = unsafe {
            if (x.alloc as usize) < total_limbs {
                gmp::_mpz_realloc(&mut x, total_limbs as c_long);
            }
            slice::from_raw_parts_mut(x.d, total_limbs)
        };
        let mut limbs_used: c_int = 0;
        let mut j = 0;
        let mut mask = 1;
        for (i, limb) in limbs.iter_mut().enumerate() {
            let mut val: gmp::limb_t = 0;
            while j < bit_length && mask != 0 {
                if bits[j] {
                    val |= mask;
                }
                j += 1;
                mask <<= 1;
            }
            if val != 0 {
                limbs_used = i as c_int + 1;
            }
            *limb = val;
            if j == bit_length {
                break;
            }
        }
        x.size = limbs_used;
        *self = Large(x);
    }

    //TODO remove
    pub fn to_u32s(&self) -> IntegerContent {
        IntegerContent::new(self)
    }
}

fn make_string(i: &Integer, radix: i32, to_upper: bool) -> String {
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

fn fmt_radix(i: &Integer,
             f: &mut Formatter,
             radix: i32,
             to_upper: bool,
             prefix: &str)
             -> fmt::Result {
    let s = make_string(i, radix, to_upper);
    let (neg, buf) = if s.starts_with('-') {
        (true, &s[1..])
    } else {
        (false, &s[..])
    };
    f.pad_integral(!neg, prefix, buf)
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
            _ => return Err(Error { kind: Kind::InvalidDigit }),
        };
        if digit_value >= radix {
            return Err(Error { kind: Kind::InvalidDigit });
        }
        got_digit = true;
    }
    if !got_digit {
        return Err(Error { kind: Kind::NoDigits });
    }
    Ok(skip_plus)
}

//TODO test
impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

//TODO test
impl Debug for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

//TODO test
impl FromStr for Integer {
    type Err = ParseIntegerError;

    fn from_str(src: &str) -> Result<Integer, ParseIntegerError> {
        let mut i = Integer::new();
        i.assign_str(src)?;
        Ok(i)
    }
}

pub mod assign_i32;
pub mod assign_integer;
pub mod assign_natural;
pub mod assign_u32;
pub mod clone;
pub mod from_i32;
pub mod from_u32;
pub mod into_natural;
pub mod to_i32;
pub mod to_u32;
