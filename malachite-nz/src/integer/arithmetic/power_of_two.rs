use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::traits::One;

use integer::Integer;

impl PowerOfTwo for Integer {
    /// Computes 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::PowerOfTwo;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::power_of_two(0).to_string(), "1");
    /// assert_eq!(Integer::power_of_two(3).to_string(), "8");
    /// assert_eq!(Integer::power_of_two(100).to_string(), "1267650600228229401496703205376");
    /// ```
    #[inline]
    fn power_of_two(pow: u64) -> Integer {
        Integer::ONE << pow
    }
}
