use malachite_base::num::traits::{Assign, SplitInHalf, Zero};
use natural::Natural::{self, Large, Small};
use platform::{DoubleLimb, Limb};
use std::ops::{Mul, MulAssign};

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the product of the `Natural` and a `Limb`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_limb::limbs_mul_limb;
///
/// assert_eq!(limbs_mul_limb(&[123, 456], 789), &[97_047, 359_784]);
/// assert_eq!(limbs_mul_limb(&[0xffff_ffff, 5], 2), &[4_294_967_294, 11]);
/// assert_eq!(limbs_mul_limb(&[0xffff_ffff], 2), &[4_294_967_294, 1]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c, where the result is returned.
pub fn limbs_mul_limb(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut carry = 0;
    let limb = DoubleLimb::from(limb);
    let mut result_limbs = Vec::with_capacity(limbs.len());
    for &x in limbs {
        let limb_result = DoubleLimb::from(x) * limb + DoubleLimb::from(carry);
        result_limbs.push(limb_result.lower_half());
        carry = limb_result.upper_half();
    }
    if carry != 0 {
        result_limbs.push(carry);
    }
    result_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb`, plus a carry, to an output slice. The output
/// slice must be at least as long as the input slice. Returns the carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_limb::limbs_mul_limb_with_carry_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_with_carry_to_out(&mut out, &[123, 456], 789, 10), 0);
/// assert_eq!(out, &[97_057, 359_784, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_with_carry_to_out(&mut out, &[0xffff_ffff], 2, 3), 2);
/// assert_eq!(out, &[1, 0, 0]);
/// ```
///
/// This is mul_1c from gmp-impl.h.
pub fn limbs_mul_limb_with_carry_to_out(
    out: &mut [Limb],
    in_limbs: &[Limb],
    limb: Limb,
    mut carry: Limb,
) -> Limb {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    let limb = DoubleLimb::from(limb);
    for i in 0..len {
        let limb_result = DoubleLimb::from(in_limbs[i]) * limb + DoubleLimb::from(carry);
        out[i] = limb_result.lower_half();
        carry = limb_result.upper_half();
    }
    carry
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb` to an output slice. The output slice must be
/// at least as long as the input slice. Returns the carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_limb::limbs_mul_limb_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_to_out(&mut out, &[123, 456], 789), 0);
/// assert_eq!(out, &[97_047, 359_784, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_to_out(&mut out, &[0xffff_ffff], 2), 1);
/// assert_eq!(out, &[4_294_967_294, 0, 0]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c.
#[inline]
pub fn limbs_mul_limb_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) -> Limb {
    limbs_mul_limb_with_carry_to_out(out, in_limbs, limb, 0)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb` to the input slice. Returns the carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_limb::limbs_slice_mul_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_mul_limb_in_place(&mut limbs, 789), 0);
/// assert_eq!(limbs, &[97_047, 359_784]);
///
/// let mut limbs = vec![0xffff_ffff];
/// assert_eq!(limbs_slice_mul_limb_in_place(&mut limbs, 2), 1);
/// assert_eq!(limbs, &[4_294_967_294]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c, where rp == up.
pub fn limbs_slice_mul_limb_in_place(limbs: &mut [Limb], limb: Limb) -> Limb {
    let mut carry = 0;
    let limb = DoubleLimb::from(limb);
    for x in limbs.iter_mut() {
        let limb_result = DoubleLimb::from(*x) * limb + DoubleLimb::from(carry);
        *x = limb_result.lower_half();
        carry = limb_result.upper_half();
    }
    carry
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb` to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_limb::limbs_vec_mul_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_mul_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[97_047, 359_784]);
///
/// let mut limbs = vec![0xffff_ffff];
/// limbs_vec_mul_limb_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[4_294_967_294, 1]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c, where the rp == up and instead of returning the
/// carry, it is appended to rp.
pub fn limbs_vec_mul_limb_in_place(limbs: &mut Vec<Limb>, limb: Limb) {
    let carry = limbs_slice_mul_limb_in_place(limbs, limb);
    if carry != 0 {
        limbs.push(carry);
    }
}

/// Multiplies a `Natural` by a `Limb`, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO * 123).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * 1).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * 456).to_string(), "56088");
///     assert_eq!((Natural::trillion() * 123).to_string(), "123000000000000");
/// }
/// ```
impl Mul<Limb> for Natural {
    type Output = Natural;

    #[inline]
    fn mul(mut self, other: Limb) -> Natural {
        self *= other;
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Mul<u32> for Natural {
    type Output = Natural;

    #[inline]
    fn mul(self, other: u32) -> Natural {
        self * Limb::from(other)
    }
}

/// Multiplies a `Natural` by a `Limb`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO * 123).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * 1).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * 456).to_string(), "56088");
///     assert_eq!((&Natural::trillion() * 123).to_string(), "123000000000000");
/// }
/// ```
impl<'a> Mul<Limb> for &'a Natural {
    type Output = Natural;

    fn mul(self, other: Limb) -> Natural {
        if *self == 0 as Limb || other == 0 {
            return Natural::ZERO;
        }
        if other == 1 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                let product = DoubleLimb::from(small) * DoubleLimb::from(other);
                let (upper, lower) = product.split_in_half();
                if upper == 0 {
                    Small(lower)
                } else {
                    Large(vec![lower, upper])
                }
            }
            Large(ref limbs) => Large(limbs_mul_limb(limbs, other)),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Mul<u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn mul(self, other: u32) -> Natural {
        self * Limb::from(other)
    }
}

/// Multiplies a `Limb` by a `Natural`, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((123 * Natural::ZERO).to_string(), "0");
///     assert_eq!((1 * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 * Natural::from(123u32)).to_string(), "56088");
///     assert_eq!((123 * Natural::trillion()).to_string(), "123000000000000");
/// }
/// ```
impl Mul<Natural> for Limb {
    type Output = Natural;

    #[inline]
    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

#[cfg(feature = "64_bit_limbs")]
impl Mul<Natural> for u32 {
    type Output = Natural;

    #[inline]
    fn mul(self, other: Natural) -> Natural {
        Limb::from(self) * other
    }
}

/// Multiplies a `Limb` by a `Natural`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((123 * &Natural::ZERO).to_string(), "0");
///     assert_eq!((1 * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 * &Natural::from(123u32)).to_string(), "56088");
///     assert_eq!((123 * &Natural::trillion()).to_string(), "123000000000000");
/// }
/// ```
impl<'a> Mul<&'a Natural> for Limb {
    type Output = Natural;

    #[inline]
    fn mul(self, other: &'a Natural) -> Natural {
        other * self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> Mul<&'a Natural> for u32 {
    type Output = Natural;

    #[inline]
    fn mul(self, other: &'a Natural) -> Natural {
        Limb::from(self) * other
    }
}

/// Multiplies a `Natural` by a `Limb` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::One;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= 1;
///     x *= 2;
///     x *= 3;
///     x *= 4;
///     assert_eq!(x.to_string(), "24");
/// }
/// ```
impl MulAssign<Limb> for Natural {
    fn mul_assign(&mut self, other: Limb) {
        if *self == 0 as Limb || other == 0 {
            self.assign(Limb::ZERO);
            return;
        }
        if other == 1 {
            return;
        }
        if *self == 1 as Limb {
            self.assign(other);
            return;
        }
        mutate_with_possible_promotion!(self, small, limbs, { small.checked_mul(other) }, {
            limbs_vec_mul_limb_in_place(limbs, other);
        });
    }
}

#[cfg(feature = "64_bit_limbs")]
impl MulAssign<u32> for Natural {
    #[inline]
    fn mul_assign(&mut self, other: u32) {
        *self *= Limb::from(other);
    }
}
