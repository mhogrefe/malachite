use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;

impl Natural {
    /// Sets the `index`th bit of a `Natural`, or the coefficient of 2^(`index`) in its binary
    /// expansion, to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// let mut x = Natural::new();
    /// x.set_bit(2);
    /// x.set_bit(5);
    /// x.set_bit(6);
    /// assert_eq!(x.to_string(), "100");
    /// ```
    pub fn set_bit(&mut self, index: u64) {
        mutate_with_possible_promotion!(self,
                                        small,
                                        large,
                                        {
                                            if index < 32 {
                                                Some(*small | (1 << index))
                                            } else {
                                                None
                                            }
                                        },
                                        {
                                            unsafe { gmp::mpz_setbit(large, index) }
                                        });
    }
}
