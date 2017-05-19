use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Shl, ShlAssign};

/// Shifts a `Natural` left (multiplies it by a power of 2), taking ownership of the input
/// `Natural`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert_eq!((Natural::from(0) << 10).to_string(), "0");
/// assert_eq!((Natural::from(123) << 2).to_string(), "492");
/// assert_eq!((Natural::from(123) << 100).to_string(), "155921023828072216384094494261248");
/// ```
impl Shl<u32> for Natural {
    type Output = Natural;
    fn shl(mut self, other: u32) -> Natural {
        self <<= other;
        self
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2) in place.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// let mut x = Natural::from(1);
/// x <<= 1;
/// x <<= 2;
/// x <<= 3;
/// x <<= 4;
/// assert_eq!(x.to_string(), "1024");
/// ```
impl ShlAssign<u32> for Natural {
    fn shl_assign(&mut self, other: u32) {
        if other == 0 || *self == 0 {
            return;
        }
        mutate_with_possible_promotion!(self,
                                        small,
                                        large,
                                        {
                                            if other <= small.leading_zeros() {
                                                Some(*small << other)
                                            } else {
                                                None
                                            }
                                        },
                                        {
                                            unsafe { gmp::mpz_mul_2exp(large, large, other.into()) }
                                        });
    }
}
