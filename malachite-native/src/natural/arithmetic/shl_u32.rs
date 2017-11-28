use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS, pad_left};
use natural::Natural::{self, Large, Small};
use std::ops::{Shl, ShlAssign};

// Shift u left by bits, and write the result to r. The bits shifted out at the left are returned in
// the least significant bits of the return value (the rest of the return value is zero).
// u.len() > 0, r.len() >= u.len(), 1 <= bits < LIMB_BITS
pub fn mpn_lshift(r: &mut [u32], u: &[u32], bits: u32) -> u32 {
    let u_len = u.len();
    assert!(u_len > 0);
    assert!(r.len() >= u_len);
    assert!(bits > 0);
    assert!(bits < LIMB_BITS);
    let cobits = LIMB_BITS - bits;
    let mut remaining_bits = 0;
    for i in 0..u_len {
        let limb = u[i];
        r[i] = (limb << bits) | remaining_bits;
        remaining_bits = limb >> cobits;
    }
    remaining_bits
}

// Shift u left by bits, and write the result to u. The bits shifted out at the left are returned in
// the least significant bits of the return value (the rest of the return value is zero).
// u.len() > 0, 1 <= bits < LIMB_BITS
pub fn mpn_lshift_in_place(u: &mut [u32], bits: u32) -> u32 {
    assert!(!u.is_empty());
    assert!(bits > 0);
    assert!(bits < LIMB_BITS);
    let cobits = LIMB_BITS - bits;
    let mut remaining_bits = 0;
    for limb in u.iter_mut() {
        let old_limb = *limb;
        *limb = (old_limb << bits) | remaining_bits;
        remaining_bits = old_limb >> cobits;
    }
    remaining_bits
}

/// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by value.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO << 10).to_string(), "0");
///     assert_eq!((Natural::from(123u32) << 2).to_string(), "492");
///     assert_eq!((Natural::from(123u32) << 100).to_string(), "155921023828072216384094494261248");
/// }
/// ```
impl Shl<u32> for Natural {
    type Output = Natural;

    fn shl(mut self, other: u32) -> Natural {
        self <<= other;
        self
    }
}

fn shl_helper(limbs: &[u32], other: u32) -> Natural {
    let small_shift = other & LIMB_BITS_MASK;
    Large(if small_shift != 0 {
        let mut shifted_limbs = vec![0; limbs.len()];
        let remaining_bits = mpn_lshift(&mut shifted_limbs, limbs, small_shift);
        pad_left(&mut shifted_limbs, (other >> LOG_LIMB_BITS) as usize, 0);
        if remaining_bits != 0 {
            shifted_limbs.push(remaining_bits);
        }
        shifted_limbs
    } else {
        let mut shifted_limbs = limbs.to_vec();
        pad_left(&mut shifted_limbs, (other >> LOG_LIMB_BITS) as usize, 0);
        shifted_limbs
    })
}

/// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by reference.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO << 10).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) << 2).to_string(), "492");
///     assert_eq!((&Natural::from(123u32) << 100).to_string(),
///         "155921023828072216384094494261248");
/// }
/// ```
impl<'a> Shl<u32> for &'a Natural {
    type Output = Natural;

    fn shl(self, other: u32) -> Natural {
        if other == 0 || self == &0 {
            return self.clone();
        }
        match *self {
            Small(small) if other <= small.leading_zeros() => Small(small << other),
            Small(small) => shl_helper(&[small], other),
            Large(ref limbs) => shl_helper(limbs, other),
        }
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2) in place.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::One;
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x <<= 1;
///     x <<= 2;
///     x <<= 3;
///     x <<= 4;
///     assert_eq!(x.to_string(), "1024");
/// }
/// ```
impl ShlAssign<u32> for Natural {
    fn shl_assign(&mut self, other: u32) {
        if other == 0 || *self == 0 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                if other <= small.leading_zeros() {
                    Some(*small << other)
                } else {
                    None
                }
            },
            {
                let small_shift = other & LIMB_BITS_MASK;
                let remaining_bits = if small_shift != 0 {
                    mpn_lshift_in_place(limbs, small_shift)
                } else {
                    0
                };
                pad_left(limbs, (other >> LOG_LIMB_BITS) as usize, 0);
                if remaining_bits != 0 {
                    limbs.push(remaining_bits);
                }
            }
        );
    }
}
