use error::ParseIntegerError;
use natural::Natural::{self, Small};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use traits::Assign;

impl Natural {
    //TODO test
    pub fn assign_str_radix(&mut self, src: &str, radix: i32) -> Result<(), ParseIntegerError> {
        assert!(src.chars().next() != Some('-'));
        self.assign(0);
        for c in src.chars() {
            self.mul_in_place_u32(radix as u32);
            if c >= '0' && c <= '9' {
                *self += c as u32 - 48;
            }
        }
        Ok(())
    }

    //TODO test
    pub fn from_str_radix(src: &str, radix: i32) -> Result<Natural, ParseIntegerError> {
        let mut i = Natural::new();
        i.assign_str_radix(src, radix)?;
        Ok(i)
    }

    //TODO test
    pub fn assign_str(&mut self, src: &str) -> Result<(), ParseIntegerError> {
        self.assign_str_radix(src, 10)
    }
}

fn make_string(i: &Natural, radix: i32, to_upper: bool) -> String {
    assert!(!to_upper);
    assert!(radix >= 2 && radix <= 36, "radix out of range");
    if *i == Small(0) {
        return "0".to_string();
    }
    let mut i_cloned = i.clone();
    let mut cs = Vec::new();
    while i_cloned != Natural::new() {
        cs.push(i_cloned.div_rem_in_place_u32(10)
                    .to_string()
                    .chars()
                    .next()
                    .unwrap());
    }
    cs.reverse();
    cs.into_iter().collect()
}

fn fmt_radix(i: &Natural,
             f: &mut Formatter,
             radix: i32,
             to_upper: bool,
             prefix: &str)
             -> fmt::Result {
    f.pad_integral(true, prefix, &make_string(i, radix, to_upper))
}

//TODO test
impl Display for Natural {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_radix(self, f, 10, false, "")
    }
}

//TODO test
impl FromStr for Natural {
    type Err = ParseIntegerError;

    fn from_str(src: &str) -> Result<Natural, ParseIntegerError> {
        let mut i = Natural::new();
        i.assign_str(src)?;
        Ok(i)
    }
}

pub mod assign_natural;
pub mod assign_u32;
pub mod from_u32;
