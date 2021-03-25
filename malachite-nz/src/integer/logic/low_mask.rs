use integer::Integer;
use malachite_base::num::logic::traits::LowMask;
use natural::Natural;

impl LowMask for Integer {
    /// Returns an `Integer` with the least significant `bits` bits on and the remaining bits off.
    ///
    /// Time: worst case O(`bits`)
    ///
    /// Additional memory: worst case O(`bits`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::LowMask;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::low_mask(0).to_string(), "0");
    /// assert_eq!(Integer::low_mask(3).to_string(), "7");
    /// assert_eq!(Integer::low_mask(100).to_string(), "1267650600228229401496703205375");
    /// ```
    #[inline]
    fn low_mask(bits: u64) -> Integer {
        Integer::from(Natural::low_mask(bits))
    }
}
