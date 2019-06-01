use std::cmp::{max, min};

use malachite_base::comparison::Max;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::logic::traits::NotAssign;

use integer::Integer;
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul_limb::limbs_mul_limb_with_carry_to_out;
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_same_length_in_place_left;
use natural::logic::not::limbs_not_in_place;
use natural::Natural::{self, Large};
use platform::Limb;

pub fn limbs_overflowing_sub_mul_limb(xs: &[Limb], ys: &[Limb], limb: Limb) -> (Vec<Limb>, bool) {
    let mut xs = xs.to_vec();
    let sign = limbs_overflowing_sub_mul_limb_in_place_left(&mut xs, ys, limb);
    (xs, sign)
}

// limb != 0, xs_len != 0, ys_len != 0
pub fn limbs_overflowing_sub_mul_limb_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    limb: Limb,
) -> bool {
    let mut sign = true;
    // xs unaffected if ys == 0 or limb == 0
    let ys_len = ys.len();
    let xs_len = xs.len();
    let mut new_xs_len = max(xs_len, ys_len);
    xs.resize(new_xs_len + 1, 0);
    let min_len = min(xs_len, ys_len);
    // submul of absolute values
    let mut cy =
        limbs_sub_mul_limb_same_length_in_place_left(&mut xs[..min_len], &ys[..min_len], limb);
    if xs_len >= ys_len {
        // if xs bigger than ys, then propagate borrow through it
        if xs_len != ys_len {
            cy = if limbs_sub_limb_in_place(&mut xs[ys_len..xs_len], cy) {
                1
            } else {
                0
            };
        }

        if cy != 0 {
            // Borrow out of xs, take twos complement negative to get absolute value, flip sign
            // of xs.
            xs[new_xs_len] = cy.wrapping_sub(1);
            limbs_not_in_place(&mut xs[..new_xs_len]);
            new_xs_len += 1;
            limbs_slice_add_limb_in_place(&mut xs[..new_xs_len], 1);
            if new_xs_len != 0 {
                sign.not_assign();
            }
        }
    } else {
        // xs_len < ys_len

        // ys bigger than xs, so want ys * limb - xs. Submul has given xs-ys*limb, so take twos
        // complement and use an mpn_mul_1 for the rest.

        // -(-cy*b^n + xs-ys*limb) = (cy-1)*b^n + ~(xs-ys*limb) + 1
        limbs_not_in_place(&mut xs[..xs_len]);
        if !limbs_slice_add_limb_in_place(&mut xs[..xs_len], 1) {
            cy.wrapping_sub_assign(1);
        }

        // If cy - 1 == -1 then hold that -1 for latter. mpn_submul_1 never returns
        // cy == MP_LIMB_T_MAX so that value always indicates a -1.
        let cy2 = if cy == Limb::MAX { 1 } else { 0 };
        cy.wrapping_add_assign(cy2);
        cy = limbs_mul_limb_with_carry_to_out(
            &mut xs[xs_len..ys_len],
            &ys[xs_len..ys_len],
            limb,
            cy,
        );
        xs[new_xs_len] = cy;
        if cy != 0 {
            new_xs_len += 1;
        }

        // Apply any -1 from above. The value at wp+xs_len is non-zero because limb != 0 and the
        // high limb of ys will be non-zero.
        if cy2 != 0 {
            limbs_sub_limb_in_place(&mut xs[xs_len..new_xs_len], 1);
        }
        if new_xs_len != 0 {
            sign.not_assign();
        }
    }
    xs.resize(new_xs_len, 0);
    if limbs_test_zero(xs) {
        sign = true;
    }
    sign
}

pub fn limbs_overflowing_sub_mul_limb_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut [Limb],
    limb: Limb,
) -> (bool, bool) {
    //TODO
    (
        false,
        limbs_overflowing_sub_mul_limb_in_place_left(xs, ys, limb),
    )
}

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
/// use malachite_base::num::arithmetic::traits::AddMul;
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
/// use malachite_base::num::arithmetic::traits::AddMul;
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
/// use malachite_base::num::arithmetic::traits::AddMul;
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
/// use malachite_base::num::arithmetic::traits::AddMul;
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
        if self.sign == b.sign {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(&b.abs, c),
            }
        } else {
            let (abs, abs_result_sign) = self.abs.add_mul_neg(&b.abs, c);
            Integer {
                sign: self.sign == abs_result_sign || abs == 0 as Limb,
                abs,
            }
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
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
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
        if self.sign == b.sign {
            self.abs.add_mul_assign(b.abs, c);
        } else {
            if !self.abs.add_mul_assign_neg(b.abs, c) {
                self.sign.not_assign();
            }
            if self.abs == 0 as Limb {
                self.sign = true;
            }
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
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
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
        if self.sign == b.sign {
            self.abs.add_mul_assign(&b.abs, c);
        } else {
            if !self.abs.add_mul_assign_neg_ref(&b.abs, c) {
                self.sign.not_assign();
            }
            if self.abs == 0 as Limb {
                self.sign = true;
            }
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

impl Natural {
    // self - b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_neg(&self, b: &Natural, c: Limb) -> (Natural, bool) {
        if c == 0 || *b == 0 as Limb {
            return (self.clone(), true);
        }
        if c == 1 {
            return if self >= b {
                (self - b, true)
            } else {
                (b - self, false)
            };
        }
        match (self, b) {
            (Large(ref a_limbs), Large(ref b_limbs)) => {
                let (limbs, sign) = limbs_overflowing_sub_mul_limb(a_limbs, b_limbs, c);
                let mut result = Large(limbs);
                result.trim();
                (result, sign)
            }
            _ => {
                let bc = b * c;
                let sign = *self >= bc;
                return if sign {
                    (self - bc, true)
                } else {
                    (bc - self, false)
                };
            }
        }
    }

    // self -= b * c, returns sign (true means non-negative)
    fn add_mul_assign_neg(&mut self, mut b: Natural, c: Limb) -> bool {
        if c == 0 || b == 0 as Limb {
            return true;
        }
        if c == 1 {
            let sign = *self >= b;
            if sign {
                *self -= b;
            } else {
                //TODO right assign
                self.sub_right_assign_no_panic(&b);
            }
            return sign;
        }
        let (fallback, (right, mut sign)) = match (&mut *self, &mut b) {
            (&mut Large(ref mut a_limbs), &mut Large(ref mut b_limbs)) => (
                false,
                limbs_overflowing_sub_mul_limb_in_place_either(a_limbs, b_limbs, c),
            ),
            _ => (true, (false, false)),
        };
        if fallback {
            let bc = b * c;
            sign = *self >= bc;
            if sign {
                *self -= bc;
            } else {
                //TODO right assign
                self.sub_right_assign_no_panic(&bc);
            }
        } else if right {
            b.trim();
            *self = b;
        } else {
            self.trim();
        }
        sign
    }

    // self -= b * c, returns sign (true means non-negative)
    fn add_mul_assign_neg_ref(&mut self, b: &Natural, c: Limb) -> bool {
        if c == 0 || *b == 0 as Limb {
            return true;
        }
        if c == 1 {
            let sign = *self >= *b;
            if sign {
                *self -= b;
            } else {
                //TODO this is why we need a public right assign
                self.sub_right_assign_no_panic(b);
            }
            return sign;
        }
        let (mut sign, fallback) = match (&mut *self, b) {
            (&mut Large(ref mut a_limbs), &Large(ref b_limbs)) => (
                limbs_overflowing_sub_mul_limb_in_place_left(a_limbs, b_limbs, c),
                false,
            ),
            _ => (false, true),
        };
        if fallback {
            let bc = b * c;
            sign = *self >= bc;
            if sign {
                *self -= bc;
            } else {
                //TODO right assign
                self.sub_right_assign_no_panic(&bc);
            }
        } else {
            self.trim();
        }
        sign
    }
}
