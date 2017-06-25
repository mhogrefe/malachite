use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Set the `index`th bit of `self`, or the coefficient of 2^(`index`) in the binary expansion
    /// of `self`, to 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// let mut x = Natural::from(127u32);
    /// x.clear_bit(0);
    /// x.clear_bit(1);
    /// x.clear_bit(3);
    /// x.clear_bit(4);
    /// assert_eq!(x.to_string(), "100");
    /// ```
    pub fn clear_bit(&mut self, index: u64) {
        match *self {
            Small(ref mut small) => {
                if index < 32 {
                    *small &= !(1 << index);
                }
                return;
            }
            Large(ref mut large) => unsafe {
                gmp::mpz_clrbit(large, index);
            },
        }
        self.demote_if_small();
    }
}
