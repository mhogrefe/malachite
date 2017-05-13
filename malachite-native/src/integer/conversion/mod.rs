use error::ParseIntegerError;
use integer::Integer;
use std::fmt::{self, Debug, Display, Formatter, Write};
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
        let mut i = Integer::new();
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
pub mod from_i32;
pub mod from_u32;
pub mod into_natural;
pub mod to_i32;
pub mod to_u32;
