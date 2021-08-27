use integer::Integer;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;

impl PowerOf2<u64> for Integer {
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
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::power_of_2(0).to_string(), "1");
    /// assert_eq!(Integer::power_of_2(3).to_string(), "8");
    /// assert_eq!(Integer::power_of_2(100).to_string(), "1267650600228229401496703205376");
    /// ```
    #[inline]
    fn power_of_2(pow: u64) -> Integer {
        Integer::ONE << pow
    }
}
