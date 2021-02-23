use error::ParseIntegerError;
use malachite_base::num::basic::traits::Zero;
use natural::Natural;
use std::str::FromStr;

impl Natural {
    //TODO test
    pub fn assign_str_radix(&mut self, src: &str, radix: u64) -> Result<(), ParseIntegerError> {
        assert!(!src.starts_with('-'));
        *self = Natural::ZERO;
        let radix = Natural::from(radix);
        for c in src.chars() {
            *self *= &radix;
            if ('0'..='9').contains(&c) {
                *self += Natural::from(c.to_digit(10).unwrap());
            }
        }
        Ok(())
    }

    //TODO test
    pub fn from_str_radix(src: &str, radix: u64) -> Result<Natural, ParseIntegerError> {
        let mut i = Natural::ZERO;
        i.assign_str_radix(src, radix)?;
        Ok(i)
    }

    //TODO test
    pub fn assign_str(&mut self, src: &str) -> Result<(), ParseIntegerError> {
        self.assign_str_radix(src, 10)
    }
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

pub mod digits;
pub mod floating_point_from_natural;
pub mod from_floating_point;
pub mod from_limbs;
pub mod from_primitive_int;
pub mod limb_count;
pub mod primitive_int_from_natural;
pub mod string;
pub mod to_limbs;
