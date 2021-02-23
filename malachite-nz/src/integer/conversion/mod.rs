use error::ParseIntegerError;
use integer::Integer;
use malachite_base::num::basic::traits::Zero;
use std::str::FromStr;

impl Integer {
    //TODO test
    pub fn assign_str_radix(&mut self, src: &str, radix: u64) -> Result<(), ParseIntegerError> {
        assert_eq!(radix, 10);
        if let Some(suffix) = src.strip_prefix('-') {
            self.sign = false;
            self.abs.assign_str_radix(suffix, radix)?;
        } else {
            self.sign = true;
            self.abs.assign_str_radix(src, radix)?;
        }
        Ok(())
    }

    //TODO test
    pub fn from_str_radix(src: &str, radix: u64) -> Result<Integer, ParseIntegerError> {
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
impl FromStr for Integer {
    type Err = ParseIntegerError;

    fn from_str(src: &str) -> Result<Integer, ParseIntegerError> {
        let mut i = Integer::ZERO;
        i.assign_str(src)?;
        Ok(i)
    }
}

pub mod floating_point_from_integer;
pub mod from_floating_point;
pub mod from_natural;
pub mod from_primitive_int;
pub mod from_twos_complement_limbs;
pub mod natural_from_integer;
pub mod primitive_int_from_integer;
pub mod string;
pub mod to_twos_complement_limbs;
