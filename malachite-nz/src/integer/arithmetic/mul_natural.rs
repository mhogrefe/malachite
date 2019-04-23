use integer::Integer;
use natural::Natural;
use platform::Limb;
use std::ops::{Mul, MulAssign};

/// Multiplies an `Integer` by a `Natural`, taking both by value.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Integer::from(123) * Natural::ZERO).to_string(), "0");
///     assert_eq!((Integer::from(-123) * Natural::from(456u32)).to_string(), "-56088");
///     assert_eq!((Integer::from_str("-123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "-121932631112635269000000");
/// }
/// ```
impl Mul<Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn mul(mut self, other: Natural) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by a `Natural`, taking the `Integer` by value and the `Natural` by
/// reference.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Integer::from(123) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((Integer::from(-123) * &Natural::from(456u32)).to_string(), "-56088");
///     assert_eq!((Integer::from_str("-123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "-121932631112635269000000");
/// }
/// ```
impl<'a> Mul<&'a Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn mul(mut self, other: &'a Natural) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by a `Natural`, taking the `Integer` by reference and the `Natural` by
/// value.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Integer::from(123) * Natural::ZERO).to_string(), "0");
///     assert_eq!((&Integer::from(-123) * Natural::from(456u32)).to_string(), "-56088");
///     assert_eq!((&Integer::from_str("-123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "-121932631112635269000000");
/// }
/// ```
impl<'a> Mul<Natural> for &'a Integer {
    type Output = Integer;

    fn mul(self, other: Natural) -> Integer {
        let abs_product = &self.abs * other;
        Integer {
            sign: self.sign || abs_product == 0 as Limb,
            abs: abs_product,
        }
    }
}

/// Multiplies an `Integer` by a `Natural`, taking both by reference.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Integer::from(123) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((&Integer::from(-123) * &Natural::from(456u32)).to_string(), "-56088");
///     assert_eq!((&Integer::from_str("-123456789000").unwrap() *
///                 &Natural::from_str("987654321000").unwrap()).to_string(),
///                 "-121932631112635269000000");
/// }
/// ```
impl<'a, 'b> Mul<&'a Natural> for &'b Integer {
    type Output = Integer;

    fn mul(self, other: &'a Natural) -> Integer {
        let abs_product = &self.abs * other;
        Integer {
            sign: self.sign || abs_product == 0 as Limb,
            abs: abs_product,
        }
    }
}

/// Multiplies an `Integer` by a `Natural` in place, taking the `Natural` by value.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::NegativeOne;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x *= Natural::from_str("1000").unwrap();
///     x *= Natural::from_str("2000").unwrap();
///     x *= Natural::from_str("3000").unwrap();
///     x *= Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "-24000000000000");
/// }
/// ```
impl MulAssign<Natural> for Integer {
    fn mul_assign(&mut self, other: Natural) {
        self.abs *= other;
        if !self.sign && self.abs == 0 as Limb {
            self.sign = true;
        }
    }
}

/// Multiplies an `Integer` by a `Natural` in place, taking the `Natural` by reference.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::NegativeOne;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x *= &Natural::from_str("1000").unwrap();
///     x *= &Natural::from_str("2000").unwrap();
///     x *= &Natural::from_str("3000").unwrap();
///     x *= &Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "-24000000000000");
/// }
/// ```
impl<'a> MulAssign<&'a Natural> for Integer {
    fn mul_assign(&mut self, other: &'a Natural) {
        self.abs *= other;
        if !self.sign && self.abs == 0 as Limb {
            self.sign = true;
        }
    }
}

/// Multiplies a `Natural` by an `Integer`, taking both by value.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * Integer::from(123)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * Integer::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "-121932631112635269000000");
/// }
/// ```
impl Mul<Integer> for Natural {
    type Output = Integer;

    #[inline]
    fn mul(self, other: Integer) -> Integer {
        other * self
    }
}

/// Multiplies a `Natural` by an `Integer`, taking the `Natural` by value and the `Integer` by
/// reference.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * &Integer::from(123)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * &Integer::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * &Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * &Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "-121932631112635269000000");
/// }
/// ```
impl<'a> Mul<&'a Integer> for Natural {
    type Output = Integer;

    #[inline]
    fn mul(self, other: &'a Integer) -> Integer {
        other * self
    }
}

/// Multiplies a `Natural` by an `Integer`, taking the `Natural` by reference and the `Integer` by
/// value.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * Integer::from(123)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * Integer::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * Integer::from_str("-987654321000")
///                .unwrap()).to_string(), "-121932631112635269000000");
/// }
/// ```
impl<'a> Mul<Integer> for &'a Natural {
    type Output = Integer;

    #[inline]
    fn mul(self, other: Integer) -> Integer {
        other * self
    }
}

/// Multiplies a `Natural` by an `Integer`, taking both by reference.
///
/// Time: worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: worst case TODO
///
/// where n = `self.significant_bits()` + `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * &Integer::from(123)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * &Integer::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * &Integer::from(-456)).to_string(), "-56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() *
///                 &Integer::from_str("-987654321000").unwrap()).to_string(),
///                 "-121932631112635269000000");
/// }
/// ```
impl<'a, 'b> Mul<&'a Integer> for &'b Natural {
    type Output = Integer;

    #[inline]
    fn mul(self, other: &'a Integer) -> Integer {
        other * self
    }
}
