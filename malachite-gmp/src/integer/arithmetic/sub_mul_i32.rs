use integer::Integer;
use malachite_base::traits::{AddMul, AddMulAssign, SubMul, SubMulAssign};

/// Adds the product of an `Integer` (b) and an `i32` (c) to an `Integer` (self), taking `self` and
/// b by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(Integer::from(3u32), -4i32), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(Integer::from(-0x1_0000), 0x1_0000i32)
///         .to_string(), "-995705032704");
/// }
/// ```
impl SubMul<Integer, i32> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: Integer, c: i32) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Adds the product of an `Integer` (b) and an `i32` (c) to an `Integer` (self), taking `self` by
/// value and b by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(&Integer::from(3u32), -4i32), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(&Integer::from(-0x1_0000), 0x1_0000i32)
///         .to_string(), "-995705032704");
/// }
/// ```
impl<'a> SubMul<&'a Integer, i32> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: &'a Integer, c: i32) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Adds the product of an `Integer` (b) and an `i32` (c) to an `Integer` (self), taking `self` by
/// reference and b by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).sub_mul(Integer::from(3u32), -4i32), 22);
///     assert_eq!((&(-Integer::trillion())).sub_mul(Integer::from(-0x1_0000), 0x1_0000i32)
///         .to_string(), "-995705032704");
/// }
/// ```
impl<'a> SubMul<Integer, i32> for &'a Integer {
    type Output = Integer;

    fn sub_mul(self, b: Integer, c: i32) -> Integer {
        self.sub_mul(&b, c)
    }
}

/// Adds the product of an `Integer` (b) and an `i32` (c) to an `Integer` (self), taking `self` and
/// b by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).sub_mul(&Integer::from(3u32), -4i32), 22);
///     assert_eq!((&(-Integer::trillion())).sub_mul(&Integer::from(-0x1_0000), 0x1_0000i32)
///         .to_string(), "-995705032704");
/// }
/// ```
impl<'a, 'b> SubMul<&'a Integer, i32> for &'b Integer {
    type Output = Integer;

    fn sub_mul(self, b: &'a Integer, c: i32) -> Integer {
        if c >= 0 {
            self.sub_mul(b, c as u32)
        } else {
            self.add_mul(b, c.wrapping_neg() as u32)
        }
    }
}

/// Adds the product of an `Integer` (b) and an `i32` (c) to an `Integer` (self), in place, taking b
/// by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.sub_mul_assign(Integer::from(3u32), -4i32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.sub_mul_assign(Integer::from(-0x1_0000), 0x1_0000i32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl SubMulAssign<Integer, i32> for Integer {
    fn sub_mul_assign(&mut self, b: Integer, c: i32) {
        self.sub_mul_assign(&b, c);
    }
}

/// Adds the product of an `Integer` (b) and an `i32` (c) to an `Integer` (self), in place, taking b
/// by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.sub_mul_assign(&Integer::from(3u32), -4i32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.sub_mul_assign(&Integer::from(-0x1_0000), 0x1_0000i32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl<'a> SubMulAssign<&'a Integer, i32> for Integer {
    fn sub_mul_assign(&mut self, b: &'a Integer, c: i32) {
        if c >= 0 {
            self.sub_mul_assign(b, c as u32);
        } else {
            self.add_mul_assign(b, c.wrapping_neg() as u32)
        }
    }
}
