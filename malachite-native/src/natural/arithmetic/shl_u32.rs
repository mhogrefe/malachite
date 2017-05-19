use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS, pad_left};
use natural::Natural::{self, Large, Small};
use std::ops::{Shl, ShlAssign};

/// Shifts a `Natural` left (multiplies it by a power of 2), taking ownership of the input
/// `Natural`.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!((Natural::from(0) << 10).to_string(), "0");
/// assert_eq!((Natural::from(123) << 2).to_string(), "492");
/// assert_eq!((Natural::from(123) << 100).to_string(), "155921023828072216384094494261248");
/// ```
impl Shl<u32> for Natural {
    type Output = Natural;
    fn shl(mut self, other: u32) -> Natural {
        self <<= other;
        self
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2) in place.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// let mut x = Natural::from(1);
/// x <<= 1;
/// x <<= 2;
/// x <<= 3;
/// x <<= 4;
/// assert_eq!(x.to_string(), "1024");
/// ```
impl ShlAssign<u32> for Natural {
    fn shl_assign(&mut self, other: u32) {
        if other == 0 || *self == 0 {
            return;
        }
        mutate_with_possible_promotion!(self,
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
                                            // rustfmt doesn't like when the next two lines are
                                            // chained
                                            let last = *limbs.last().unwrap();
                                            let leading_zeros = last.leading_zeros();
                                            let remaining_shift = other & LIMB_BITS_MASK;
                                            if remaining_shift != 0 {
                                                let shift_complement = LIMB_BITS - remaining_shift;
                                                let mut previous = 0;
                                                for limb in limbs.iter_mut() {
                                                    let old_limb = *limb;
                                                    *limb = (*limb << remaining_shift) |
                                                            (previous >> shift_complement);
                                                    previous = old_limb;
                                                }
                                                if remaining_shift > leading_zeros {
                                                    limbs.push(previous >> shift_complement);
                                                }
                                            }
                                            pad_left(limbs, (other >> LOG_LIMB_BITS) as usize, 0);
                                        });
    }
}
