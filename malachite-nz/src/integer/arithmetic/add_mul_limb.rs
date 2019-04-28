use integer::Integer;
use malachite_base::comparison::Max;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::traits::{
    AddMul, AddMulAssign, NegAssign, NotAssign, WrappingAddAssign, WrappingSubAssign,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::add_mul_limb::limbs_slice_add_mul_limb_greater_in_place_left;
use natural::arithmetic::mul_limb::{limbs_mul_limb_to_out, limbs_mul_limb_with_carry_to_out};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::mpn_submul_1;
use natural::logic::not::limbs_not_in_place;
use natural::Natural::{self, Large, Small};
use platform::Limb;
use std::cmp::{max, min};

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), taking `self` and
/// b by value.
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), 4u32), 22);
///     assert_eq!((-Integer::trillion()).add_mul(Integer::from(0x1_0000), 0x1_0000u32).to_string(),
///                "-995705032704");
/// }
/// ```
impl AddMul<Integer, Limb> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: Integer, c: Limb) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl AddMul<Integer, u32> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: Integer, c: u32) -> Integer {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), taking `self` by
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), 4u32), 22);
///     assert_eq!((-Integer::trillion()).add_mul(&Integer::from(0x1_0000),
///         0x1_0000u32).to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMul<&'a Integer, Limb> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: &'a Integer, c: Limb) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMul<&'a Integer, u32> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: &'a Integer, c: u32) -> Integer {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), taking `self` by
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(Integer::from(3u32), 4u32), 22);
///     assert_eq!((&(-Integer::trillion())).add_mul(Integer::from(0x1_0000),
///         0x1_0000u32).to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMul<Integer, Limb> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: Integer, c: Limb) -> Integer {
        self.add_mul(&b, c)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMul<Integer, u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: Integer, c: u32) -> Integer {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), taking `self` and
/// b by reference.
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
/// use malachite_base::num::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), 4u32), 22);
///     assert_eq!((&(-Integer::trillion())).add_mul(&Integer::from(0x1_0000),
///         0x1_0000u32).to_string(), "-995705032704");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Integer, Limb> for &'b Integer {
    type Output = Integer;

    fn add_mul(self, b: &'a Integer, c: Limb) -> Integer {
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

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> AddMul<&'a Integer, u32> for &'b Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: &'a Integer, c: u32) -> Integer {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), in place, taking b
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), 4u32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(Integer::from(0x1_0000), 0x1_0000u32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl AddMulAssign<Integer, Limb> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: Limb) {
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

#[cfg(feature = "64_bit_limbs")]
impl AddMulAssign<Integer, u32> for Integer {
    #[inline]
    fn add_mul_assign(&mut self, b: Integer, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), in place, taking b
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), 4u32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(&Integer::from(0x1_0000), 0x1_0000u32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMulAssign<&'a Integer, Limb> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: Limb) {
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

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMulAssign<&'a Integer, u32> for Integer {
    #[inline]
    fn add_mul_assign(&mut self, b: &'a Integer, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}

pub(crate) fn large_aorsmul_val(
    a_sign: &mut bool,
    a_abs: &mut Natural,
    b_sign: bool,
    b_abs: &Natural,
    c: Limb,
    add: bool,
) {
    {
        let mut a_limbs = a_abs.promote_in_place();
        match *b_abs {
            Small(small_b) => mpz_aorsmul_1(a_sign, &mut a_limbs, b_sign, &[small_b], c, add),
            Large(ref b_limbs) => mpz_aorsmul_1(a_sign, &mut a_limbs, b_sign, b_limbs, c, add),
        }
    }
    a_abs.trim();
}

pub(crate) fn large_aorsmul_ref(
    a_sign: bool,
    a_abs: &Natural,
    b_sign: bool,
    b_abs: &Natural,
    c: Limb,
    add: bool,
) -> Integer {
    let mut result_sign = a_sign;
    let mut result_limbs = a_abs.to_limbs_asc();
    match *b_abs {
        Small(small_b) => mpz_aorsmul_1(
            &mut result_sign,
            &mut result_limbs,
            b_sign,
            &[small_b],
            c,
            add,
        ),
        Large(ref b_limbs) => {
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
    w: &mut Vec<Limb>,
    x_sign: bool,
    x: &[Limb],
    y: Limb,
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
        let cy = limbs_mul_limb_to_out(w, &x[..xsize], y);
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
        let mut cy = limbs_slice_add_mul_limb_greater_in_place_left(w, &x[..min_size], y);
        let mut dsize = xsize as isize - wsize as isize;
        if dsize != 0 {
            let cy2 = if dsize > 0 {
                limbs_mul_limb_to_out(
                    &mut w[min_size..],
                    &x[min_size..min_size + dsize as usize],
                    y,
                )
            } else {
                dsize.neg_assign();
                0
            };
            cy = cy2
                + if limbs_slice_add_limb_in_place(&mut w[min_size..min_size + dsize as usize], cy)
                {
                    1
                } else {
                    0
                };
        }
        let dsize = dsize as usize;
        w[min_size + dsize] = cy;
    } else {
        // submul of absolute values
        let mut cy = mpn_submul_1(w, &x[..min_size], y);
        if wsize >= xsize {
            // if w bigger than x, then propagate borrow through it
            if wsize != xsize {
                cy = if limbs_sub_limb_in_place(&mut w[xsize..wsize], cy) {
                    1
                } else {
                    0
                };
            }

            if cy != 0 {
                // Borrow out of w, take twos complement negative to get absolute value, flip sign
                // of w.
                w[new_wsize] = !(!cy).wrapping_add(1); // extra limb is 0 - cy
                limbs_not_in_place(&mut w[..new_wsize]);
                new_wsize += 1;
                limbs_slice_add_limb_in_place(&mut w[..new_wsize], 1);
                if new_wsize != 0 {
                    w_sign.not_assign();
                }
            }
        } else {
            // wsize < xsize

            // x bigger than w, so want x*y-w. Submul has given w-x*y, so take twos complement and
            // use an mpn_mul_1 for the rest.

            // -(-cy*b^n + w-x*y) = (cy-1)*b^n + ~(w-x*y) + 1
            limbs_not_in_place(&mut w[..wsize]);
            if !limbs_slice_add_limb_in_place(&mut w[..wsize], 1) {
                cy.wrapping_sub_assign(1);
            }

            // If cy - 1 == -1 then hold that -1 for latter. mpn_submul_1 never returns
            // cy == MP_LIMB_T_MAX so that value always indicates a -1.
            let cy2 = if cy == Limb::MAX { 1 } else { 0 };
            cy.wrapping_add_assign(cy2);
            cy = limbs_mul_limb_with_carry_to_out(&mut w[wsize..xsize], &x[wsize..xsize], y, cy);
            w[new_wsize] = cy;
            if cy != 0 {
                new_wsize += 1;
            }

            // Apply any -1 from above. The value at wp+wsize is non-zero because y != 0 and the
            // high limb of x will be non-zero.
            if cy2 != 0 {
                limbs_sub_limb_in_place(&mut w[wsize..new_wsize], 1);
            }
            if new_wsize != 0 {
                w_sign.not_assign();
            }
        }
        w.resize(new_wsize, 0);
        if limbs_test_zero(w) {
            *w_sign = true;
        }
    }
}
