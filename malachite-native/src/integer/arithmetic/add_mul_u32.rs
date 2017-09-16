use integer::Integer;
use natural::arithmetic::add_u32::mpn_add_1_in_place;
use natural::Natural::{self, Large, Small};
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
/// assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), 4), 22);
/// assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                     .add_mul(Integer::from(65536u32), 65536).to_string(),
///            "-995705032704");
/// ```
impl AddMul<Integer, u32> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: Integer, c: u32) -> Integer {
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
/// assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), 4), 22);
/// assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                     .add_mul(&Integer::from(65536u32), 65536).to_string(),
///            "-995705032704");
/// ```
impl<'a> AddMul<&'a Integer, u32> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: &'a Integer, c: u32) -> Integer {
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
/// assert_eq!((&Integer::from(10u32)).add_mul(Integer::from(3u32), 4), 22);
/// assert_eq!((&Integer::from_str("-1000000000000").unwrap())
///                     .add_mul(Integer::from(65536u32), 65536).to_string(),
///            "-995705032704");
/// ```
impl<'a> AddMul<Integer, u32> for &'a Integer {
    type Output = Integer;

    fn add_mul(self, b: Integer, c: u32) -> Integer {
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
/// assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), 4), 22);
/// assert_eq!((&Integer::from_str("-1000000000000").unwrap())
///                     .add_mul(&Integer::from(65536u32), 65536).to_string(),
///             "-995705032704");
/// ```
impl<'a, 'b> AddMul<&'a Integer, u32> for &'b Integer {
    type Output = Integer;

    fn add_mul(self, b: &'a Integer, c: u32) -> Integer {
        if c == 0 {
            self.clone()
        } else if self.sign == b.sign {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(&b.abs, c),
            }
        } else {
            if let Small(a) = self.abs {
                if a == 0 {
                    return Integer {
                        sign: false,
                        abs: &b.abs * c,
                    };
                } else if let Small(small_b) = b.abs {
                    if small_b == 0 {
                        return self.clone();
                    } else if let Some(product) = small_b.checked_mul(c) {
                        return if b.sign {
                            self + product
                        } else {
                            self - product
                        };
                    }
                }
            }
            large_aors_ref(self.sign, &self.abs, &b.abs, c)
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
/// x.add_mul_assign(Integer::from(3u32), 4);
/// assert_eq!(x, 22);
///
/// let mut x = Integer::from_str("-1000000000000").unwrap();
/// x.add_mul_assign(Integer::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "-995705032704");
/// ```
impl AddMulAssign<Integer, u32> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: u32) {
        if c == 0 {
        } else if self.sign == b.sign {
            self.abs.add_mul_assign(b.abs, c);
        } else {
            if let Small(a) = self.abs {
                if a == 0 {
                    self.abs = b.abs * c;
                    self.sign = false;
                    return;
                } else if let Small(small_b) = b.abs {
                    if small_b == 0 {
                        return;
                    } else if let Some(product) = small_b.checked_mul(c) {
                        if b.sign {
                            *self += product;
                        } else {
                            *self -= product;
                        }
                        return;
                    }
                }
            }
            large_aors_val(&mut self.sign, &mut self.abs, &b.abs, c);
        }
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
/// x.add_mul_assign(&Integer::from(3u32), 4);
/// assert_eq!(x, 22);
///
/// let mut x = Integer::from_str("-1000000000000").unwrap();
/// x.add_mul_assign(&Integer::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "-995705032704");
/// ```
impl<'a> AddMulAssign<&'a Integer, u32> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: u32) {
        if c == 0 {
        } else if self.sign == b.sign {
            self.abs.add_mul_assign(&b.abs, c);
        } else {
            if let Small(a) = self.abs {
                if a == 0 {
                    self.abs = &b.abs * c;
                    self.sign = false;
                    return;
                } else if let Small(small_b) = b.abs {
                    if small_b == 0 {
                        return;
                    } else if let Some(product) = small_b.checked_mul(c) {
                        if b.sign {
                            *self += product;
                        } else {
                            *self -= product;
                        }
                        return;
                    }
                }
            }
            large_aors_val(&mut self.sign, &mut self.abs, &b.abs, c);
        }
    }
}

pub(crate) fn large_aors_val(a_sign: &mut bool, a_abs: &mut Natural, b_abs: &Natural, c: u32) {
    let b_limb_count = b_abs.limb_count();
    if a_abs.limb_count() > b_limb_count + 1 {
        a_abs.sub_mul_assign(&b_abs, c);
    } else {
        {
            let a_limbs = a_abs.promote_in_place();
            a_limbs.resize(b_limb_count as usize + 1, 0);
            // push a limb so that sub_mul_assign won't overflow
            a_limbs.push(1);
        }
        a_abs.sub_mul_assign(&b_abs, c);
        if a_abs.limb_count() == b_limb_count + 2 {
            // extra limb wasn't needed
            {
                let a_limbs = a_abs.promote_in_place();
                a_limbs.pop();
            }
        } else {
            {
                let a_limbs = a_abs.promote_in_place();
                for limb in a_limbs.iter_mut() {
                    *limb = !*limb;
                }
                if mpn_add_1_in_place(a_limbs, 1) {
                    a_limbs.push(1);
                }
            }
            *a_sign = !*a_sign;
        }
        a_abs.trim();
    }
}

pub(crate) fn large_aors_ref(a_sign: bool, a_abs: &Natural, b_abs: &Natural, c: u32) -> Integer {
    let b_limb_count = b_abs.limb_count();
    if a_abs.limb_count() > b_limb_count + 1 {
        Integer {
            sign: a_sign,
            abs: a_abs.sub_mul(b_abs, c).unwrap(),
        }
    } else {
        let mut a_limbs = a_abs.to_limbs_le();
        a_limbs.resize(b_limb_count as usize + 1, 0);
        // push a limb so that sub_mul_assign won't overflow
        a_limbs.push(1);
        let mut result_abs = Large(a_limbs);
        result_abs.sub_mul_assign(&b_abs, c);
        let result_sign = if result_abs.limb_count() == b_limb_count + 2 {
            // extra limb wasn't needed
            {
                let a_limbs = result_abs.promote_in_place();
                a_limbs.pop();
            }
            a_sign
        } else {
            {
                let a_limbs = result_abs.promote_in_place();
                for limb in a_limbs.iter_mut() {
                    *limb = !*limb;
                }
                if mpn_add_1_in_place(a_limbs, 1) {
                    a_limbs.push(1);
                }
            }
            !a_sign
        };
        result_abs.trim();
        Integer {
            sign: result_sign,
            abs: result_abs,
        }
    }
}
