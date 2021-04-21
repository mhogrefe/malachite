use integer::Integer;
use malachite_base::num::conversion::traits::FromStringBase;
use natural::Natural;
use std::ops::Neg;
use std::str::FromStr;

impl FromStringBase for Integer {
    #[inline]
    fn from_string_base(base: u64, s: &str) -> Option<Integer> {
        if let Some(abs_string) = s.strip_prefix('-') {
            Natural::from_string_base(base, abs_string).map(Neg::neg)
        } else {
            Natural::from_string_base(base, s).map(Integer::from)
        }
    }
}

impl FromStr for Integer {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Integer, ()> {
        Integer::from_string_base(10, s).ok_or(())
    }
}
