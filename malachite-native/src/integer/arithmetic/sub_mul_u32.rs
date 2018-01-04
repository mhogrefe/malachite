use integer::arithmetic::add_mul_u32::{large_aorsmul_ref, large_aorsmul_val};
use integer::Integer;
use malachite_base::traits::{AddMul, AddMulAssign, SubMul, SubMulAssign};
use natural::Natural::Small;

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` and b
/// by value.
///
/// Time: worst case O(n)
///
/// Subitional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::SubMul;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(-10i32).sub_mul(Integer::from(3u32), 4u32), -22);
///     assert_eq!(Integer::trillion().sub_mul(Integer::from(0x1_0000), 0x1_0000u32).to_string(),
///                "995705032704");
/// }
/// ```
impl SubMul<Integer, u32> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: Integer, c: u32) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` by
/// value and b by reference.
///
/// Time: worst case O(n)
///
/// Subitional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::SubMul;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(-10i32).sub_mul(&Integer::from(3u32), 4u32), -22);
///     assert_eq!(Integer::trillion().sub_mul(&Integer::from(0x1_0000), 0x1_0000u32).to_string(),
///                "995705032704");
/// }
/// ```
impl<'a> SubMul<&'a Integer, u32> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: &'a Integer, c: u32) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` by
/// reference and b by value.
///
/// Time: worst case O(n)
///
/// Subitional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::SubMul;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(-10i32)).sub_mul(Integer::from(3u32), 4u32), -22);
///     assert_eq!((&Integer::trillion()).sub_mul(Integer::from(0x1_0000), 0x1_0000u32).to_string(),
///                "995705032704");
/// }
/// ```
impl<'a> SubMul<Integer, u32> for &'a Integer {
    type Output = Integer;

    fn sub_mul(self, b: Integer, c: u32) -> Integer {
        self.sub_mul(&b, c)
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` and b
/// by reference.
///
/// Time: worst case O(n)
///
/// Subitional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::SubMul;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(-10i32)).sub_mul(&Integer::from(3u32), 4u32), -22);
///     assert_eq!((&Integer::trillion()).sub_mul(&Integer::from(0x1_0000),
///         0x1_0000u32).to_string(), "995705032704");
///     assert_eq!((&(-Integer::trillion())).sub_mul(&Integer::from(-0x1_0000),
///         0x1_0000u32).to_string(), "-995705032704");
/// }
/// ```
impl<'a, 'b> SubMul<&'a Integer, u32> for &'b Integer {
    type Output = Integer;

    fn sub_mul(self, b: &'a Integer, c: u32) -> Integer {
        if c == 0 {
            self.clone()
        } else if self.sign != b.sign {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(&b.abs, c),
            }
        } else {
            if let Small(a) = self.abs {
                if a == 0 {
                    return Integer {
                        sign: b.abs == 0,
                        abs: &b.abs * c,
                    };
                } else if let Small(small_b) = b.abs {
                    if small_b == 0 {
                        return self.clone();
                    } else if let Some(product) = small_b.checked_mul(c) {
                        return if b.sign {
                            self - product
                        } else {
                            self + product
                        };
                    }
                }
            }
            large_aorsmul_ref(self.sign, &self.abs, b.sign, &b.abs, c, false)
        }
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), in place, taking b
/// by value.
///
/// Time: worst case O(n)
///
/// Subitional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(-10i32);
///     x.sub_mul_assign(Integer::from(3u32), 4u32);
///     assert_eq!(x, -22);
///
///     let mut x = Integer::trillion();
///     x.sub_mul_assign(Integer::from(0x1_0000), 0x1_0000u32);
///     assert_eq!(x.to_string(), "995705032704");
/// }
/// ```
impl SubMulAssign<Integer, u32> for Integer {
    fn sub_mul_assign(&mut self, b: Integer, c: u32) {
        if c == 0 {
        } else if self.sign != b.sign {
            self.abs.add_mul_assign(b.abs, c);
        } else {
            if let Small(a) = self.abs {
                if a == 0 {
                    self.sign = b.abs == 0;
                    self.abs = b.abs * c;
                    return;
                } else if let Small(small_b) = b.abs {
                    if small_b == 0 {
                        return;
                    } else if let Some(product) = small_b.checked_mul(c) {
                        if b.sign {
                            *self -= product;
                        } else {
                            *self += product;
                        }
                        return;
                    }
                }
            }
            large_aorsmul_val(&mut self.sign, &mut self.abs, b.sign, &b.abs, c, false);
        }
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), in place, taking b
/// by reference.
///
/// Time: worst case O(n)
///
/// Subitional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(-10i32);
///     x.sub_mul_assign(&Integer::from(3u32), 4u32);
///     assert_eq!(x, -22);
///
///     let mut x = Integer::trillion();
///     x.sub_mul_assign(&Integer::from(0x1_0000), 0x1_0000u32);
///     assert_eq!(x.to_string(), "995705032704");
/// }
/// ```
impl<'a> SubMulAssign<&'a Integer, u32> for Integer {
    fn sub_mul_assign(&mut self, b: &'a Integer, c: u32) {
        if c == 0 {
        } else if self.sign != b.sign {
            self.abs.add_mul_assign(&b.abs, c);
        } else {
            if let Small(a) = self.abs {
                if a == 0 {
                    self.abs = &b.abs * c;
                    self.sign = b.abs == 0;
                    return;
                } else if let Small(small_b) = b.abs {
                    if small_b == 0 {
                    } else if let Some(product) = small_b.checked_mul(c) {
                        if b.sign {
                            *self -= product;
                        } else {
                            *self += product;
                        }
                        return;
                    }
                }
            }
            large_aorsmul_val(&mut self.sign, &mut self.abs, b.sign, &b.abs, c, false);
        }
    }
}
