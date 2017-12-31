use integer::Integer;
use malachite_base::traits::{AddMul, AddMulAssign};
use natural::arithmetic::add_mul_u32::mpn_addmul_1;
use natural::arithmetic::add_u32::mpn_add_1_in_place;
use natural::arithmetic::mul_u32::{mpn_mul_1, mpn_mul_1c};
use natural::arithmetic::sub_mul_u32::mpn_submul_1;
use natural::arithmetic::sub_u32::mpn_sub_1_in_place;
use natural::logic::not::mpn_com_in_place;
use natural::mpn_zero_p;
use natural::Natural::{self, Large, Small};
use std::cmp::{max, min};

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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), 4u32), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(Integer::from(65536u32), 65536u32).to_string(),
///                "-995705032704");
/// }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), 4u32), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(&Integer::from(65536u32), 65536u32).to_string(),
///                "-995705032704");
/// }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(Integer::from(3u32), 4u32), 22);
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap())
///                         .add_mul(Integer::from(65536u32), 65536u32).to_string(),
///                "-995705032704");
/// }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), 4u32), 22);
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap())
///                         .add_mul(&Integer::from(65536u32), 65536u32).to_string(),
///                 "-995705032704");
/// }
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
            large_aorsmul_ref(self.sign, &self.abs, b.sign, &b.abs, c, true)
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), 4u32);
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(Integer::from(65536u32), 65536u32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
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
            large_aorsmul_val(&mut self.sign, &mut self.abs, b.sign, &b.abs, c, true);
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), 4u32);
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(&Integer::from(65536u32), 65536u32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
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
            large_aorsmul_val(&mut self.sign, &mut self.abs, b.sign, &b.abs, c, true);
        }
    }
}

pub(crate) fn large_aorsmul_val(
    a_sign: &mut bool,
    a_abs: &mut Natural,
    b_sign: bool,
    b_abs: &Natural,
    c: u32,
    add: bool,
) {
    {
        let mut a_limbs = a_abs.promote_in_place();
        match b_abs {
            &Small(small_b) => mpz_aorsmul_1(a_sign, &mut a_limbs, b_sign, &[small_b], c, add),
            &Large(ref b_limbs) => mpz_aorsmul_1(a_sign, &mut a_limbs, b_sign, b_limbs, c, add),
        }
    }
    a_abs.trim();
}

pub(crate) fn large_aorsmul_ref(
    a_sign: bool,
    a_abs: &Natural,
    b_sign: bool,
    b_abs: &Natural,
    c: u32,
    add: bool,
) -> Integer {
    let mut result_sign = a_sign;
    let mut result_limbs = a_abs.to_limbs_le();
    match b_abs {
        &Small(small_b) => mpz_aorsmul_1(
            &mut result_sign,
            &mut result_limbs,
            b_sign,
            &[small_b],
            c,
            add,
        ),
        &Large(ref b_limbs) => {
            mpz_aorsmul_1(&mut result_sign, &mut result_limbs, b_sign, b_limbs, c, add)
        }
    }
    let mut result_abs = Natural::Large(result_limbs);
    result_abs.trim();
    Integer {
        sign: result_sign,
        abs: result_abs,
    }
}

pub(crate) fn mpz_aorsmul_1(
    w_sign: &mut bool,
    w: &mut Vec<u32>,
    x_sign: bool,
    x: &[u32],
    y: u32,
    mut sub: bool,
) {
    // w unaffected if x == 0 or y == 0
    let mut xsize = x.len();
    if xsize == 0 || y == 0 {
        return;
    }
    sub ^= !x_sign;
    let wsize = w.len();
    if wsize == 0 {
        // nothing to add to, just set x * y, `sub` gives the sign
        w.resize(xsize + 1, 0);
        let cy = mpn_mul_1(w, &x[0..xsize], y);
        w[xsize] = cy;
        if cy != 0 {
            xsize += 1;
        }
        w.resize(xsize, 0);
        *w_sign = sub;
        return;
    }
    sub ^= !*w_sign;
    let mut new_wsize = max(wsize, xsize);
    w.resize(new_wsize + 1, 0);
    let min_size = min(wsize, xsize);

    if sub {
        // addmul of absolute values
        let mut cy = mpn_addmul_1(w, &x[0..min_size], y);
        let mut dsize = xsize as isize - wsize as isize;
        if dsize != 0 {
            let cy2 = if dsize > 0 {
                mpn_mul_1(
                    &mut w[min_size..],
                    &x[min_size..min_size + dsize as usize],
                    y,
                )
            } else {
                dsize = -dsize;
                0
            };
            cy = cy2 + if mpn_add_1_in_place(&mut w[min_size..min_size + dsize as usize], cy) {
                1
            } else {
                0
            };
        }
        let dsize = dsize as usize;
        w[min_size + dsize] = cy;
    } else {
        // submul of absolute values
        let mut cy = mpn_submul_1(w, &x[0..min_size], y);
        if wsize >= xsize {
            // if w bigger than x, then propagate borrow through it
            if wsize != xsize {
                cy = if mpn_sub_1_in_place(&mut w[xsize..wsize], cy) {
                    1
                } else {
                    0
                };
            }

            if cy != 0 {
                // Borrow out of w, take twos complement negative to get absolute value, flip sign
                // of w.
                w[new_wsize] = !(!cy).wrapping_add(1); // extra limb is 0 - cy
                mpn_com_in_place(&mut w[0..new_wsize]);
                new_wsize += 1;
                mpn_add_1_in_place(&mut w[0..new_wsize], 1);
                if new_wsize != 0 {
                    *w_sign = !*w_sign;
                }
            }
        } else {
            // wsize < xsize

            // x bigger than w, so want x*y-w. Submul has given w-x*y, so take twos complement and
            // use an mpn_mul_1 for the rest.

            // -(-cy*b^n + w-x*y) = (cy-1)*b^n + ~(w-x*y) + 1
            mpn_com_in_place(&mut w[0..wsize]);
            if !mpn_add_1_in_place(&mut w[0..wsize], 1) {
                cy = cy.wrapping_sub(1);
            }

            // If cy - 1 == -1 then hold that -1 for latter. mpn_submul_1 never returns
            // cy == MP_LIMB_T_MAX so that value always indicates a -1.
            let cy2 = if cy == u32::max_value() { 1 } else { 0 };
            cy = cy.wrapping_add(cy2);
            cy = mpn_mul_1c(&mut w[wsize..xsize], &x[wsize..xsize], y, cy);
            w[new_wsize] = cy;
            if cy != 0 {
                new_wsize += 1;
            }

            // Apply any -1 from above. The value at wp+wsize is non-zero because y != 0 and the
            // high limb of x will be non-zero.
            if cy2 != 0 {
                mpn_sub_1_in_place(&mut w[wsize..new_wsize], 1);
            }
            if new_wsize != 0 {
                *w_sign = !*w_sign;
            }
        }
        w.resize(new_wsize, 0);
        if mpn_zero_p(w) {
            *w_sign = true;
        }
    }
}
