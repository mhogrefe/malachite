use integer::Integer;
use traits::{AddMul, AddMulAssign, SubMul, SubMulAssign};

/// Adds the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` and b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), 4i32), 22);
/// assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                     .add_mul(Integer::from(-65536i32), -65536i32).to_string(),
///            "-995705032704");
/// ```
impl AddMul<Integer, i32> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: Integer, c: i32) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` by
/// value and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), 4i32), 22);
/// assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                     .add_mul(&Integer::from(-65536i32), -65536i32).to_string(),
///            "-995705032704");
/// ```
impl<'a> AddMul<&'a Integer, i32> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: &'a Integer, c: i32) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` by
/// reference and b by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(10u32)).add_mul(Integer::from(3u32), 4i32), 22);
/// assert_eq!((&Integer::from_str("-1000000000000").unwrap())
///                     .add_mul(Integer::from(-65536i32), -65536i32).to_string(),
///            "-995705032704");
/// ```
impl<'a> AddMul<Integer, i32> for &'a Integer {
    type Output = Integer;

    fn add_mul(self, b: Integer, c: i32) -> Integer {
        self.add_mul(&b, c)
    }
}

/// Adds the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` and b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), 4i32), 22);
/// assert_eq!((&Integer::from_str("-1000000000000").unwrap())
///                     .add_mul(&Integer::from(-65536i32), -65536i32).to_string(),
///             "-995705032704");
/// ```
impl<'a, 'b> AddMul<&'a Integer, i32> for &'b Integer {
    type Output = Integer;

    fn add_mul(self, b: &'a Integer, c: i32) -> Integer {
        if c >= 0 {
            self.add_mul(b, c as u32)
        } else {
            self.sub_mul(b, c.wrapping_neg() as u32)
        }
    }
}

/// Adds the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), in place, taking b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Integer::from(10u32);
/// x.add_mul_assign(Integer::from(3u32), 4i32);
/// assert_eq!(x, 22);
///
/// let mut x = Integer::from_str("-1000000000000").unwrap();
/// x.add_mul_assign(Integer::from(-65536i32), -65536i32);
/// assert_eq!(x.to_string(), "-995705032704");
/// ```
impl AddMulAssign<Integer, i32> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: i32) {
        self.add_mul_assign(&b, c);
    }
}

/// Adds the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), in place, taking b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Integer::from(10u32);
/// x.add_mul_assign(&Integer::from(3u32), 4i32);
/// assert_eq!(x, 22);
///
/// let mut x = Integer::from_str("-1000000000000").unwrap();
/// x.add_mul_assign(&Integer::from(-65536i32), -65536i32);
/// assert_eq!(x.to_string(), "-995705032704");
/// ```
impl<'a> AddMulAssign<&'a Integer, i32> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: i32) {
        if c >= 0 {
            self.add_mul_assign(b, c as u32);
        } else {
            self.sub_mul_assign(b, c.wrapping_neg() as u32)
        }
    }
}
