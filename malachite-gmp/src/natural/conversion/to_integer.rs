use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::mem;

impl Natural {
    /// Converts a `Natural` to an `Integer`. This implementation takes `self` by value.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123u32).into_integer().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().into_integer().to_string(),
    ///            "1000000000000");
    /// ```
    pub fn into_integer(mut self) -> Integer {
        match self {
            Natural::Small(small) if small <= i32::max_value() as u32 => {
                Integer::Small(small as i32)
            }
            Natural::Small(small) => unsafe {
                let mut large: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut large, small.into());
                Integer::Large(large)
            },
            Natural::Large(ref mut large) => unsafe {
                let mut swapped_large: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut swapped_large);
                mem::swap(&mut swapped_large, large);
                Integer::Large(swapped_large)
            },
        }
    }

    /// Converts a `Natural` to an `Integer`. This implementation takes `self` by reference.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123u32).to_integer().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_integer().to_string(),
    ///            "1000000000000");
    /// ```
    pub fn to_integer(&self) -> Integer {
        match *self {
            Natural::Small(small) if small <= i32::max_value() as u32 => {
                Integer::Small(small as i32)
            }
            Natural::Small(small) => unsafe {
                let mut large: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut large, small.into());
                Integer::Large(large)
            },
            Natural::Large(ref large) => unsafe {
                let mut large_copy: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut large_copy, large);
                Integer::Large(large_copy)
            },
        }
    }
}
