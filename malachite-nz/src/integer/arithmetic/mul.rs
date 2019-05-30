use std::ops::{Mul, MulAssign};

use integer::Integer;
use platform::Limb;

/// Multiplies an `Integer` by an `Integer`, taking both `Integer`s by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::ONE * Integer::from(123)).to_string(), "123");
///     assert_eq!((Integer::from(123) * Integer::ZERO).to_string(), "0");
///     assert_eq!((Integer::from(123) * Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((Integer::from_str("-123456789000").unwrap() * Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl Mul<Integer> for Integer {
    type Output = Integer;

    fn mul(mut self, other: Integer) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by an `Integer`, taking the left `Integer` by value and the right
/// `Integer` by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::ONE * &Integer::from(123)).to_string(), "123");
///     assert_eq!((Integer::from(123) * &Integer::ZERO).to_string(), "0");
///     assert_eq!((Integer::from(123) * &Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((Integer::from_str("-123456789000").unwrap() *
///                 &Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<&'a Integer> for Integer {
    type Output = Integer;

    fn mul(mut self, other: &'a Integer) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by an `Integer`, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::ONE * Integer::from(123)).to_string(), "123");
///     assert_eq!((&Integer::from(123) * Integer::ZERO).to_string(), "0");
///     assert_eq!((&Integer::from(123) * Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((&Integer::from_str("-123456789000").unwrap() *
///                 Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<Integer> for &'a Integer {
    type Output = Integer;

    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

/// Multiplies an `Integer` by an `Integer`, taking both `Integer`s by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::ONE * &Integer::from(123)).to_string(), "123");
///     assert_eq!((&Integer::from(123) * &Integer::ZERO).to_string(), "0");
///     assert_eq!((&Integer::from(123) * &Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((&Integer::from_str("-123456789000").unwrap() *
///                 &Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a, 'b> Mul<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn mul(self, other: &'a Integer) -> Integer {
        let product_abs = &self.abs * &other.abs;
        Integer {
            sign: product_abs == 0 as Limb || self.sign == other.sign,
            abs: product_abs,
        }
    }
}

/// Multiplies an `Integer` by an `Integer` in place, taking the `Integer` on the RHS by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::NegativeOne;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x *= Integer::from_str("1000").unwrap();
///     x *= Integer::from_str("2000").unwrap();
///     x *= Integer::from_str("3000").unwrap();
///     x *= Integer::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "-24000000000000");
/// }
/// ```
impl MulAssign<Integer> for Integer {
    fn mul_assign(&mut self, other: Integer) {
        self.abs *= other.abs;
        self.sign = self.abs == 0 as Limb || self.sign == other.sign;
    }
}

/// Multiplies an `Integer` by an `Integer` in place, taking the `Integer` on the RHS by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::NegativeOne;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x *= &Integer::from_str("1000").unwrap();
///     x *= &Integer::from_str("2000").unwrap();
///     x *= &Integer::from_str("3000").unwrap();
///     x *= &Integer::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "-24000000000000");
/// }
/// ```
impl<'a> MulAssign<&'a Integer> for Integer {
    fn mul_assign(&mut self, other: &'a Integer) {
        self.abs *= &other.abs;
        self.sign = self.abs == 0 as Limb || self.sign == other.sign;
    }
}
