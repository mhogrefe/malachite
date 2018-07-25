use malachite_base::limbs::limbs_pad_left;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::PrimitiveInteger;
use natural::Natural::{self, Large, Small};
use std::ops::{Shl, ShlAssign};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` left-shifted by a `u32`.
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
pub fn limbs_shl(limbs: &[u32], bits: u64) -> Vec<u32> {
    let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
    let mut shifted_limbs = vec![0; (bits >> u32::LOG_WIDTH) as usize];
    if small_bits == 0 {
        shifted_limbs.extend_from_slice(limbs);
    } else {
        let cobits = u32::WIDTH - small_bits;
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

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` left-shifted by a `u32` to an output slice. The output slice must be at
/// least as long as the input slice. The `u32` must be between 1 and 31, inclusive. The carry,
/// or the bits that are shifted past the width of the input slice, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs`, `bits` is 0, or `bits` is greater than 31.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl_u::limbs_shl_to_out;
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_shl_to_out(&mut out_limbs, &[123, 456], 1), 0);
/// assert_eq!(out_limbs, &[246, 912, 0]);
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_shl_to_out(&mut out_limbs, &[123, 456], 31), 228);
/// assert_eq!(out_limbs, &[2_147_483_648, 61, 0]);
/// ```
pub fn limbs_shl_to_out(out_limbs: &mut [u32], in_limbs: &[u32], bits: u32) -> u32 {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    assert!(bits > 0);
    assert!(bits < u32::WIDTH);
    let cobits = u32::WIDTH - bits;
    let mut remaining_bits = 0;
    for i in 0..len {
        let limb = in_limbs[i];
        out_limbs[i] = (limb << bits) | remaining_bits;
        remaining_bits = limb >> cobits;
    }
    remaining_bits
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` left-shifted by a `u32` to the input slice. The `u32` must be between 1
/// and 31, inclusive. The carry, or the bits that are shifted past the width of the input slice, is
/// returned.
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
pub fn limbs_slice_shl_in_place(limbs: &mut [u32], bits: u32) -> u32 {
    assert!(bits > 0);
    assert!(bits < u32::WIDTH);
    let cobits = u32::WIDTH - bits;
    let mut remaining_bits = 0;
    for limb in limbs.iter_mut() {
        let old_limb = *limb;
        *limb = (old_limb << bits) | remaining_bits;
        remaining_bits = old_limb >> cobits;
    }
    remaining_bits
}

/// Interpreting a nonempty `Vec` of `u32`s as the limbs (in ascending order) of a `Natural`, writes
/// the limbs of the `Natural` left-shifted by a `u32` to the input `Vec`.
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
pub fn limbs_vec_shl_in_place(limbs: &mut Vec<u32>, bits: u64) {
    let small_bits = u32::wrapping_from(bits) & u32::WIDTH_MASK;
    let remaining_bits = if small_bits == 0 {
        0
    } else {
        limbs_slice_shl_in_place(limbs, small_bits)
    };
    limbs_pad_left(limbs, (bits >> u32::LOG_WIDTH) as usize, 0);
    if remaining_bits != 0 {
        limbs.push(remaining_bits);
    }
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
        /// use malachite_base::num::Zero;
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
        /// use malachite_base::num::Zero;
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
                if other == 0 || *self == 0 {
                    return self.clone();
                }
                match *self {
                    Small(small) if other <= $t::wrapping_from(small.leading_zeros()) => {
                        Small(small << other)
                    }
                    Small(small) => Large(limbs_shl(&[small], u64::from(other))),
                    Large(ref limbs) => Large(limbs_shl(limbs, u64::from(other))),
                }
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
        /// use malachite_base::num::One;
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
                if other == 0 || *self == 0 {
                    return;
                }
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
                        limbs_vec_shl_in_place(limbs, u64::from(other));
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
