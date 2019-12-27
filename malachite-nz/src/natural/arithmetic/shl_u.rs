use std::ops::{Shl, ShlAssign};

use malachite_base::limbs::limbs_pad_left;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` left-shifted by a `Limb`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()` + `bits` / 32
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl_u::limbs_shl;
///
/// assert_eq!(limbs_shl(&[123, 456], 1), &[246, 912]);
/// assert_eq!(limbs_shl(&[123, 456], 31), &[2_147_483_648, 61, 228]);
/// assert_eq!(limbs_shl(&[123, 456], 32), &[0, 123, 456]);
/// assert_eq!(limbs_shl(&[123, 456], 100), &[0, 0, 0, 1_968, 7_296]);
/// ```
///
/// This is mpn_lshift from mpn/generic/lshift.c, where the result is returned.
pub fn limbs_shl(limbs: &[Limb], bits: u64) -> Vec<Limb> {
    let small_bits = u32::wrapping_from(bits) & Limb::WIDTH_MASK;
    let mut shifted_limbs = vec![0; usize::exact_from(bits >> Limb::LOG_WIDTH)];
    if small_bits == 0 {
        shifted_limbs.extend_from_slice(limbs);
    } else {
        let cobits = Limb::WIDTH - small_bits;
        let mut remaining_bits = 0;
        for limb in limbs {
            shifted_limbs.push((limb << small_bits) | remaining_bits);
            remaining_bits = limb >> cobits;
        }
        if remaining_bits != 0 {
            shifted_limbs.push(remaining_bits);
        }
    }
    shifted_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` left-shifted by a `Limb` to an output slice. The output slice must be at
/// least as long as the input slice. The `Limb` must be between 1 and `Limb::WIDTH` - 1, inclusive.
/// The carry, or the bits that are shifted past the width of the input slice, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`, `bits` is 0, or `bits` is greater than or equal to
/// `Limb::WIDTH`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl_u::limbs_shl_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shl_to_out(&mut out, &[123, 456], 1), 0);
/// assert_eq!(out, &[246, 912, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shl_to_out(&mut out, &[123, 456], 31), 228);
/// assert_eq!(out, &[2_147_483_648, 61, 0]);
/// ```
///
/// This is mpn_lshift from mpn/generic/lshift.c.
pub fn limbs_shl_to_out(out: &mut [Limb], in_limbs: &[Limb], bits: u32) -> Limb {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let cobits = Limb::WIDTH - bits;
    let mut remaining_bits = 0;
    for i in 0..len {
        let limb = in_limbs[i];
        out[i] = (limb << bits) | remaining_bits;
        remaining_bits = limb >> cobits;
    }
    remaining_bits
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` left-shifted by a `Limb` to the input slice. The `Limb` must be between 1
/// and `Limb::WIDTH` - 1, inclusive. The carry, or the bits that are shifted past the width of the
/// input slice, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl_u::limbs_slice_shl_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_shl_in_place(&mut limbs, 1), 0);
/// assert_eq!(limbs, &[246, 912]);
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_shl_in_place(&mut limbs, 31), 228);
/// assert_eq!(limbs, &[2_147_483_648, 61]);
/// ```
///
/// This is mpn_lshift from mpn/generic/lshift.c, where rp == up.
pub fn limbs_slice_shl_in_place(limbs: &mut [Limb], bits: u32) -> Limb {
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let cobits = Limb::WIDTH - bits;
    let mut remaining_bits = 0;
    for limb in limbs.iter_mut() {
        let old_limb = *limb;
        *limb = (old_limb << bits) | remaining_bits;
        remaining_bits = old_limb >> cobits;
    }
    remaining_bits
}

/// Interpreting a nonempty `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the `Natural` left-shifted by a `Limb` to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `limbs.len()` + `bits` / 32, m = `bits`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl_u::limbs_vec_shl_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shl_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[246, 912]);
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_shl_in_place(&mut limbs, 31);
/// assert_eq!(limbs, &[2_147_483_648, 61, 228]);
/// ```
///
/// This is mpn_lshift from mpn/generic/lshift.c, where rp == up and the carry is appended to rp.
pub fn limbs_vec_shl_in_place(limbs: &mut Vec<Limb>, bits: u64) {
    let small_bits = u32::wrapping_from(bits) & Limb::WIDTH_MASK;
    let remaining_bits = if small_bits == 0 {
        0
    } else {
        limbs_slice_shl_in_place(limbs, small_bits)
    };
    limbs_pad_left(limbs, usize::exact_from(bits >> Limb::LOG_WIDTH), 0);
    if remaining_bits != 0 {
        limbs.push(remaining_bits);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` left-shifted by a `Limb`, and complemented, to an output slice. The
/// output slice must be at least as long as the input slice. The `Limb` must be between 1 and
/// `Limb::WIDTH` - 1, inclusive. The carry, or the bits that are shifted past the width of the
/// input slice, is returned. The carry is not complemented.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`, `in_limbs` is empty, `bits` is 0, or `bits` is
/// greater than or equal to `Limb::WIDTH`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl_u::limbs_shl_with_complement_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shl_with_complement_to_out(&mut out, &[123, 456], 1), 0);
/// assert_eq!(out, &[4294967049, 4294966383, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shl_with_complement_to_out(&mut out, &[123, 456], 31), 228);
/// assert_eq!(out, &[2147483647, 4294967234, 0]);
/// ```
///
/// This is mpn_lshiftc from mpn/generic/mpn_lshiftc.
pub fn limbs_shl_with_complement_to_out(out: &mut [Limb], xs: &[Limb], bits: u32) -> Limb {
    let n = xs.len();
    assert_ne!(n, 0);
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let cobits = Limb::WIDTH - bits;
    let mut low_limb = *xs.last().unwrap();
    let remaining_bits = low_limb >> cobits;
    let mut high_limb = low_limb << bits;
    for i in (1..n).rev() {
        low_limb = xs[i - 1];
        out[i] = !(high_limb | (low_limb >> cobits));
        high_limb = low_limb << bits;
    }
    out[0] = !high_limb;
    remaining_bits
}

macro_rules! impl_natural_shl_unsigned {
    ($t:ident) => {
        /// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by value.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(m)
        ///
        /// where n = `self.significant_bits()` + `other`, m = `other`
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::basic::traits::Zero;
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     assert_eq!((Natural::ZERO << 10u8).to_string(), "0");
        ///     assert_eq!((Natural::from(123u32) << 2u16).to_string(), "492");
        ///     assert_eq!((Natural::from(123u32) << 100u64).to_string(),
        ///         "155921023828072216384094494261248");
        /// }
        /// ```
        impl Shl<$t> for Natural {
            type Output = Natural;

            #[inline]
            fn shl(mut self, other: $t) -> Natural {
                self <<= other;
                self
            }
        }
        /// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by
        /// reference.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(n)
        ///
        /// where n = `self.significant_bits()` + `other`
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::basic::traits::Zero;
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     assert_eq!((&Natural::ZERO << 10u8).to_string(), "0");
        ///     assert_eq!((&Natural::from(123u32) << 2u16).to_string(), "492");
        ///     assert_eq!((&Natural::from(123u32) << 100u64).to_string(),
        ///         "155921023828072216384094494261248");
        /// }
        /// ```
        impl<'a> Shl<$t> for &'a Natural {
            type Output = Natural;

            fn shl(self, other: $t) -> Natural {
                if other == 0 || *self == 0 as Limb {
                    return self.clone();
                }
                Natural(match *self {
                    Natural(Small(small)) if other <= $t::wrapping_from(small.leading_zeros()) => {
                        Small(small << other)
                    }
                    Natural(Small(small)) => Large(limbs_shl(&[small], u64::exact_from(other))),
                    Natural(Large(ref limbs)) => Large(limbs_shl(limbs, u64::exact_from(other))),
                })
            }
        }

        /// Shifts a `Natural` left (multiplies it by a power of 2) in place.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(m)
        ///
        /// where n = `self.significant_bits()` + `other`, m = `other`
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::basic::traits::One;
        /// use malachite_nz::natural::Natural;
        ///
        /// fn main() {
        ///     let mut x = Natural::ONE;
        ///     x <<= 1u8;
        ///     x <<= 2u16;
        ///     x <<= 3u32;
        ///     x <<= 4u64;
        ///     assert_eq!(x.to_string(), "1024");
        /// }
        /// ```
        impl ShlAssign<$t> for Natural {
            fn shl_assign(&mut self, other: $t) {
                if other == 0 || *self == 0 as Limb {
                    return;
                }
                //TODO use TrueCheckedShl
                mutate_with_possible_promotion!(
                    self,
                    small,
                    limbs,
                    {
                        if other <= $t::wrapping_from(small.leading_zeros()) {
                            Some(*small << other)
                        } else {
                            None
                        }
                    },
                    {
                        limbs_vec_shl_in_place(limbs, u64::exact_from(other));
                    }
                );
            }
        }
    };
}
impl_natural_shl_unsigned!(u8);
impl_natural_shl_unsigned!(u16);
impl_natural_shl_unsigned!(u32);
impl_natural_shl_unsigned!(u64);
impl_natural_shl_unsigned!(u128);
impl_natural_shl_unsigned!(usize);
