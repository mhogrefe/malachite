use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;

impl Integer {
    /// Sets the `index`th bit of a `Integer`, or the coefficient of 2^(`index`) in its binary
    /// expansion, to 1.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::integer::Integer;
    ///
    /// fn main() {
    ///     let mut x = Integer::ZERO;
    ///     x.set_bit(2);
    ///     x.set_bit(5);
    ///     x.set_bit(6);
    ///     assert_eq!(x.to_string(), "100");
    ///
    ///     let mut x = Integer::from(-256);
    ///     x.set_bit(2);
    ///     x.set_bit(5);
    ///     x.set_bit(6);
    ///     assert_eq!(x.to_string(), "-156");
    /// }
    /// ```
    pub fn set_bit(&mut self, index: u64) {
        mutate_with_possible_promotion!(
            self,
            small,
            large,
            {
                if index < 31 {
                    Some(*small | (1 << index))
                } else if *small < 0 {
                    Some(*small)
                } else {
                    None
                }
            },
            { unsafe { gmp::mpz_setbit(large, index) } }
        );
        self.demote_if_small();
    }
}
