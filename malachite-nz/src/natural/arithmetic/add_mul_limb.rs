use malachite_base::num::{AddMul, AddMulAssign, PrimitiveInteger, SplitInHalf};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul_limb::{limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place};
use natural::Natural::{self, Large};
use platform::{DoubleLimb, Limb};

pub fn limbs_add_mul_limb(xs: &[Limb], ys: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut result;
    if xs.len() >= ys.len() {
        result = xs.to_vec();
        limbs_vec_add_mul_limb_greater_in_place_left(&mut result, ys, limb);
    } else {
        result = ys.to_vec();
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, &mut result, limb);
    }
    result
}

// Multiply ys and limb, and add the ys.len() least significant limbs of the product to xs and
// write the result to xs. Return the most significant limb of the product, plus carry-out from the
// addition. xs.len() >= ys.len()
pub fn limbs_slice_add_mul_limb_greater_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    limb: Limb,
) -> Limb {
    let ys_len = ys.len();
    assert!(xs.len() >= ys_len);
    let mut carry = 0;
    let limb_double = DoubleLimb::from(limb);
    for i in 0..ys_len {
        let limb_result = DoubleLimb::from(xs[i]) + DoubleLimb::from(ys[i]) * limb_double + carry;
        xs[i] = limb_result.lower_half();
        carry = limb_result >> Limb::WIDTH;
    }
    carry as Limb
}

pub fn limbs_slice_add_mul_limb_same_length_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    limb: Limb,
) -> Limb {
    let xs_len = ys.len();
    assert_eq!(ys.len(), xs_len);
    let mut carry = 0;
    let limb_double = DoubleLimb::from(limb);
    for i in 0..xs_len {
        let limb_result = DoubleLimb::from(xs[i]) + DoubleLimb::from(ys[i]) * limb_double + carry;
        ys[i] = limb_result.lower_half();
        carry = limb_result >> Limb::WIDTH;
    }
    carry as Limb
}

// xs.len() > 0, ys.len() > 0, limb != 0
pub fn limbs_vec_add_mul_limb_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], limb: Limb) {
    if xs.len() >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_left(xs, ys, limb);
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_left(xs, ys, limb);
    }
}

// ys.len() > 0, xs.len() >= ys.len(), limb != 0
fn limbs_vec_add_mul_limb_greater_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], limb: Limb) {
    let carry = limbs_slice_add_mul_limb_greater_in_place_left(xs, ys, limb);
    let ys_len = ys.len();
    if carry != 0 {
        if xs.len() == ys_len {
            xs.push(carry);
        } else if limbs_slice_add_limb_in_place(&mut xs[ys_len..], carry) {
            xs.push(1);
        }
    }
}

// xs.len() > 0, xs.len() < ys.len(), limb != 0
fn limbs_vec_add_mul_limb_smaller_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], limb: Limb) {
    let xs_len = xs.len();
    let (ys_lo, ys_hi) = ys.split_at(xs_len);
    xs.resize(ys.len(), 0);
    let mut carry;
    {
        let (xs_lo, xs_hi) = xs.split_at_mut(xs_len);
        carry = limbs_mul_limb_to_out(xs_hi, ys_hi, limb);
        let inner_carry = limbs_slice_add_mul_limb_greater_in_place_left(xs_lo, ys_lo, limb);
        if inner_carry != 0 && limbs_slice_add_limb_in_place(xs_hi, inner_carry) {
            carry += 1;
        }
    }
    if carry != 0 {
        xs.push(carry);
    }
}

pub fn limbs_vec_add_mul_limb_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, limb: Limb) {
    if xs.len() >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_right(xs, ys, limb);
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, ys, limb);
    }
}

// ys.len() > 0, xs.len() >= ys.len(), limb != 0
fn limbs_vec_add_mul_limb_greater_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, limb: Limb) {
    let ys_len = ys.len();
    let carry = limbs_slice_add_mul_limb_same_length_in_place_right(&xs[..ys_len], ys, limb);
    ys.extend_from_slice(&xs[ys_len..]);
    if carry != 0 {
        if xs.len() == ys_len {
            ys.push(carry);
        } else if limbs_slice_add_limb_in_place(&mut ys[ys_len..], carry) {
            ys.push(1);
        }
    }
}

// xs.len() > 0, xs.len() < ys.len(), limb != 0
fn limbs_vec_add_mul_limb_smaller_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, limb: Limb) {
    let mut carry;
    {
        let (ys_lo, ys_hi) = ys.split_at_mut(xs.len());
        carry = limbs_slice_mul_limb_in_place(ys_hi, limb);
        let inner_carry = limbs_slice_add_mul_limb_same_length_in_place_right(xs, ys_lo, limb);
        if inner_carry != 0 && limbs_slice_add_limb_in_place(ys_hi, inner_carry) {
            carry += 1;
        }
    }
    if carry != 0 {
        ys.push(carry);
    }
}

pub fn limbs_vec_add_mul_limb_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    limb: Limb,
) -> bool {
    if xs.len() >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_left(xs, ys, limb);
        false
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, ys, limb);
        true
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` and b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), 4), 22);
///     assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32), 0x1_0000u32).to_string(),
///                "1004294967296");
/// }
/// ```
impl AddMul<Natural, Limb> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: Natural, c: Limb) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl AddMul<Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` by
/// value and b by reference.
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
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), 4), 22);
///     assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMul<&'a Natural, Limb> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: &'a Natural, c: Limb) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` by
/// reference and b by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(10u32)).add_mul(Natural::from(3u32), 4), 22);
///     assert_eq!((&Natural::trillion()).add_mul(Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMul<Natural, Limb> for &'a Natural {
    type Output = Natural;

    fn add_mul(self, mut b: Natural, c: Limb) -> Natural {
        if c == 0 || b == 0 as Limb {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        let fallback = match (self, &mut b) {
            (Large(ref a_limbs), Large(ref mut b_limbs)) => {
                limbs_vec_add_mul_limb_in_place_right(a_limbs, b_limbs, c);
                false
            }
            _ => true,
        };
        if fallback {
            self + b * c
        } else {
            b
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` and b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), 4), 22);
///     assert_eq!((&Natural::trillion()).add_mul(&Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Natural, Limb> for &'b Natural {
    type Output = Natural;

    fn add_mul(self, b: &'a Natural, c: Limb) -> Natural {
        if c == 0 || *b == 0 as Limb {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        match (self, b) {
            (Large(ref a_limbs), Large(ref b_limbs)) => {
                Large(limbs_add_mul_limb(a_limbs, b_limbs, c))
            }
            _ => self + b * c,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> AddMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), in place, taking b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(Natural::from(3u32), 4);
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(Natural::from(0x1_0000u32), 0x1_0000u32);
///     assert_eq!(x.to_string(), "1004294967296");
/// }
/// ```
impl AddMulAssign<Natural, Limb> for Natural {
    fn add_mul_assign(&mut self, mut b: Natural, c: Limb) {
        if c == 0 || b == 0 as Limb {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        let (fallback, right) = match (&mut *self, &mut b) {
            (&mut Large(ref mut a_limbs), &mut Large(ref mut b_limbs)) => (
                false,
                limbs_vec_add_mul_limb_in_place_either(a_limbs, b_limbs, c),
            ),
            _ => (true, false),
        };
        if fallback {
            *self += b * c;
        } else if right {
            *self = b;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl AddMulAssign<Natural, u32> for Natural {
    #[inline]
    fn add_mul_assign(&mut self, b: Natural, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), in place, taking b
/// by reference.
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
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(&Natural::from(3u32), 4);
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(&Natural::from(0x1_0000u32), 0x1_0000u32);
///     assert_eq!(x.to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMulAssign<&'a Natural, Limb> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: Limb) {
        if c == 0 || *b == 0 as Limb {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        let fallback = match (&mut *self, b) {
            (&mut Large(ref mut a_limbs), &Large(ref b_limbs)) => {
                limbs_vec_add_mul_limb_in_place_left(a_limbs, b_limbs, c);
                false
            }
            _ => true,
        };
        if fallback {
            *self += b * c;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMulAssign<&'a Natural, u32> for Natural {
    #[inline]
    fn add_mul_assign(&mut self, b: &'a Natural, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}
