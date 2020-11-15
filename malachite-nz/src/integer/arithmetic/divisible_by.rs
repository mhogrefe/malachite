use malachite_base::num::arithmetic::traits::DivisibleBy;

use integer::Integer;

impl DivisibleBy<Integer> for Integer {
    /// Returns whether an `Integer` is divisible by another `Integer`; in other words, whether the
    /// first `Integer` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. Both `Integer`s are
    /// taken by value.
    ///
    /// This function is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::ZERO.divisible_by(Integer::ZERO), true);
    /// assert_eq!(Integer::from(-100).divisible_by(Integer::from(-3)), false);
    /// assert_eq!(Integer::from(102).divisible_by(Integer::from(-3)), true);
    /// assert_eq!(Integer::from_str("-1000000000000000000000000").unwrap()
    ///     .divisible_by(Integer::from_str("1000000000000").unwrap()), true);
    /// ```
    fn divisible_by(self, other: Integer) -> bool {
        self.abs.divisible_by(other.abs)
    }
}

impl<'a> DivisibleBy<&'a Integer> for Integer {
    /// Returns whether an `Integer` is divisible by another `Integer`; in other words, whether the
    /// first `Integer` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. The first `Integer`
    /// is taken by value and the second by reference.
    ///
    /// This function is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::ZERO.divisible_by(&Integer::ZERO), true);
    /// assert_eq!(Integer::from(-100).divisible_by(&Integer::from(-3)), false);
    /// assert_eq!(Integer::from(102).divisible_by(&Integer::from(-3)), true);
    /// assert_eq!(Integer::from_str("-1000000000000000000000000").unwrap()
    ///     .divisible_by(&Integer::from_str("1000000000000").unwrap()), true);
    /// ```
    fn divisible_by(self, other: &'a Integer) -> bool {
        self.abs.divisible_by(&other.abs)
    }
}

impl<'a> DivisibleBy<Integer> for &'a Integer {
    /// Returns whether an `Integer` is divisible by another `Integer`; in other words, whether the
    /// first `Integer` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. The first `Integer`
    /// is taken by reference and the second by value.
    ///
    /// This function is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Integer::ZERO).divisible_by(Integer::ZERO), true);
    /// assert_eq!((&Integer::from(-100)).divisible_by(Integer::from(-3)), false);
    /// assert_eq!((&Integer::from(102)).divisible_by(Integer::from(-3)), true);
    /// assert_eq!((&Integer::from_str("-1000000000000000000000000").unwrap())
    ///     .divisible_by(Integer::from_str("1000000000000").unwrap()), true);
    /// ```
    fn divisible_by(self, other: Integer) -> bool {
        (&self.abs).divisible_by(other.abs)
    }
}

impl<'a, 'b> DivisibleBy<&'b Integer> for &'a Integer {
    /// Returns whether an `Integer` is divisible by another `Integer`; in other words, whether the
    /// first `Integer` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. Both `Integer`s are
    /// taken by reference.
    ///
    /// This function is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Integer::ZERO).divisible_by(&Integer::ZERO), true);
    /// assert_eq!((&Integer::from(-100)).divisible_by(&Integer::from(-3)), false);
    /// assert_eq!((&Integer::from(102)).divisible_by(&Integer::from(-3)), true);
    /// assert_eq!((&Integer::from_str("-1000000000000000000000000").unwrap())
    ///     .divisible_by(&Integer::from_str("1000000000000").unwrap()), true);
    /// ```
    fn divisible_by(self, other: &'b Integer) -> bool {
        (&self.abs).divisible_by(&other.abs)
    }
}
