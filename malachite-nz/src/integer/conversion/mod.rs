use error::ParseIntegerError;
use integer::Integer;
use malachite_base::num::Zero;
use std::fmt::{self, Binary, Debug, Display, Formatter, Write};
use std::str::FromStr;

impl Integer {
    //TODO test
    pub fn assign_str_radix(&mut self, src: &str, radix: i32) -> Result<(), ParseIntegerError> {
        assert_eq!(radix, 10);
        if src.starts_with('-') {
            self.sign = false;
            self.abs.assign_str_radix(&src[1..], radix)?;
        } else {
            self.sign = true;
            self.abs.assign_str_radix(src, radix)?;
        }
        Ok(())
    }

    //TODO test
    pub fn from_str_radix(src: &str, radix: i32) -> Result<Integer, ParseIntegerError> {
        let mut i = Integer::ZERO;
        i.assign_str_radix(src, radix)?;
        Ok(i)
    }

    //TODO test
    pub fn assign_str(&mut self, src: &str) -> Result<(), ParseIntegerError> {
        self.assign_str_radix(src, 10)
    }
}

//TODO test
impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if !self.sign {
            f.write_char('-').unwrap();
        }
        Display::fmt(&self.abs, f)
    }
}

//TODO test
impl Debug for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if !self.sign {
            f.write_char('-').unwrap();
        }
        Debug::fmt(&self.abs, f)
    }
}

//TODO test
impl Binary for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if !self.sign {
            f.write_char('-').unwrap();
        }
        Binary::fmt(&self.abs, f)
    }
}

//TODO test
impl FromStr for Integer {
    type Err = ParseIntegerError;

    fn from_str(src: &str) -> Result<Integer, ParseIntegerError> {
        let mut i = Integer::ZERO;
        i.assign_str(src)?;
        Ok(i)
    }
}

pub mod assign;
pub mod assign_double_limb;
pub mod assign_limb;
pub mod assign_natural;
pub mod assign_signed_double_limb;
pub mod assign_signed_limb;
pub mod double_limb_from_integer;
pub mod from_double_limb;
pub mod from_limb;
pub mod from_natural;
pub mod from_sign_and_limbs;
pub mod from_signed_double_limb;
pub mod from_signed_limb;
pub mod from_twos_complement_bits;
pub mod from_twos_complement_limbs;
pub mod limb_from_integer;
pub mod natural_assign_integer;
pub mod natural_from_integer;
pub mod signed_double_limb_from_integer;
pub mod signed_limb_from_integer;
pub mod to_sign_and_limbs;
pub mod to_twos_complement_bits;
pub mod to_twos_complement_limbs;
