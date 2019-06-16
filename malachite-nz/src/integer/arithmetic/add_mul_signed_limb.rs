use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, SubMul, SubMulAssign, UnsignedAbs,
};

use integer::Integer;
use platform::{Limb, SignedLimb};

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), taking
/// `self` and b by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), 4i32), 22);
///     assert_eq!((-Integer::trillion()).add_mul(Integer::from(-0x1_0000), -0x1_0000i32)
///         .to_string(), "-995705032704");
/// }
/// ```
impl AddMul<Integer, SignedLimb> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: Integer, c: SignedLimb) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl AddMul<Integer, i32> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: Integer, c: i32) -> Integer {
        self.add_mul(b, SignedLimb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), taking
/// `self` by value and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), 4i32), 22);
///     assert_eq!((-Integer::trillion()).add_mul(&Integer::from(-0x1_0000),
///         -0x1_0000i32).to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMul<&'a Integer, SignedLimb> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: &'a Integer, c: SignedLimb) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> AddMul<&'a Integer, i32> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: &'a Integer, c: i32) -> Integer {
        self.add_mul(b, SignedLimb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), taking
/// `self` by reference and b by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(Integer::from(3u32), 4i32), 22);
///     assert_eq!((&(-Integer::trillion())).add_mul(Integer::from(-0x1_0000),
///         -0x1_0000i32).to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMul<Integer, SignedLimb> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: Integer, c: SignedLimb) -> Integer {
        self.add_mul(&b, c)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> AddMul<Integer, i32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: Integer, c: i32) -> Integer {
        self.add_mul(b, SignedLimb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), taking
/// `self` and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), 4i32), 22);
///     assert_eq!((&(-Integer::trillion())).add_mul(&Integer::from(-0x1_0000),
///         -0x1_0000i32).to_string(), "-995705032704");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Integer, SignedLimb> for &'b Integer {
    type Output = Integer;

    fn add_mul(self, b: &'a Integer, c: SignedLimb) -> Integer {
        if c >= 0 {
            self.add_mul(b, c as Limb)
        } else {
            self.sub_mul(b, c.unsigned_abs())
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a, 'b> AddMul<&'a Integer, i32> for &'b Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: &'a Integer, c: i32) -> Integer {
        self.add_mul(b, SignedLimb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), in place,
/// taking b by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), 4i32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(Integer::from(-0x1_0000), -0x1_0000i32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl AddMulAssign<Integer, SignedLimb> for Integer {
    #[inline]
    fn add_mul_assign(&mut self, b: Integer, c: SignedLimb) {
        self.add_mul_assign(&b, c);
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl AddMulAssign<Integer, i32> for Integer {
    #[inline]
    fn add_mul_assign(&mut self, b: Integer, c: i32) {
        self.add_mul_assign(b, SignedLimb::from(c));
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), in place,
/// taking b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), 4i32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(&Integer::from(-0x1_0000), -0x1_0000i32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMulAssign<&'a Integer, SignedLimb> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: SignedLimb) {
        if c >= 0 {
            self.add_mul_assign(b, c as Limb);
        } else {
            self.sub_mul_assign(b, c.unsigned_abs())
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> AddMulAssign<&'a Integer, i32> for Integer {
    #[inline]
    fn add_mul_assign(&mut self, b: &'a Integer, c: i32) {
        self.add_mul_assign(b, SignedLimb::from(c));
    }
}
