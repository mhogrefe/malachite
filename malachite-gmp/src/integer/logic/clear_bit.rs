use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;

impl Integer {
    /// Sets the `index`th bit of a `Integer`, or the coefficient of 2^(`index`) in its binary
    /// expansion, to 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// let mut x = Integer::from(127);
    /// x.clear_bit(0);
    /// x.clear_bit(1);
    /// x.clear_bit(3);
    /// x.clear_bit(4);
    /// assert_eq!(x.to_string(), "100");
    ///
    /// let mut x = Integer::from(-156);
    /// x.clear_bit(2);
    /// x.clear_bit(5);
    /// x.clear_bit(6);
    /// assert_eq!(x.to_string(), "-256");
    /// ```
    pub fn clear_bit(&mut self, index: u64) {
        mutate_with_possible_promotion!(
            self,
            small,
            large,
            {
                if index < 31 {
                    Some(*small & !(1 << index))
                } else if *small < 0 {
                    None
                } else {
                    Some(*small)
                }
            },
            { unsafe { gmp::mpz_clrbit(large, index) } }
        );
        self.demote_if_small();
    }
}
