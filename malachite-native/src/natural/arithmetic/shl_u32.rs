use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS, pad_left};
use natural::Natural::{self, Large, Small};
use std::ops::{Shl, ShlAssign};

/// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by value.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!((Natural::from(0u32) << 10).to_string(), "0");
/// assert_eq!((Natural::from(123u32) << 2).to_string(), "492");
/// assert_eq!((Natural::from(123u32) << 100).to_string(), "155921023828072216384094494261248");
/// ```
impl Shl<u32> for Natural {
    type Output = Natural;

    fn shl(mut self, other: u32) -> Natural {
        self <<= other;
        self
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by reference.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!((&Natural::from(0u32) << 10).to_string(), "0");
/// assert_eq!((&Natural::from(123u32) << 2).to_string(), "492");
/// assert_eq!((&Natural::from(123u32) << 100).to_string(), "155921023828072216384094494261248");
/// ```
impl<'a> Shl<u32> for &'a Natural {
    type Output = Natural;

    fn shl(self, other: u32) -> Natural {
        if other == 0 || self == &0 {
            return self.clone();
        }
        match *self {
            Small(small) if other <= small.leading_zeros() => Small(small << other),
            Small(small) => {
                let mut shifted_limbs = vec![0; (other >> LOG_LIMB_BITS) as usize];
                let remaining_shift = other & LIMB_BITS_MASK;
                if remaining_shift == 0 {
                    shifted_limbs.push(small);
                } else {
                    shifted_limbs.push(small << remaining_shift);
                    if remaining_shift > small.leading_zeros() {
                        shifted_limbs.push(small >> (LIMB_BITS - remaining_shift));
                    }
                };
                Large(shifted_limbs)
            }
            Large(ref limbs) => {
                let mut shifted_limbs = vec![0; (other >> LOG_LIMB_BITS) as usize];
                let remaining_shift = other & LIMB_BITS_MASK;
                if remaining_shift == 0 {
                    shifted_limbs.extend(limbs.iter().cloned());
                } else {
                    let shift_complement = LIMB_BITS - remaining_shift;
                    let mut previous = 0;
                    for &limb in limbs.iter() {
                        shifted_limbs.push((limb << remaining_shift) |
                                           (previous >> shift_complement));
                        previous = limb;
                    }
                    if previous.leading_zeros() < remaining_shift {
                        shifted_limbs.push(previous >> shift_complement);
                    }
                }
                Large(shifted_limbs)
            }
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
/// use malachite_native::natural::Natural;
///
/// let mut x = Natural::from(1u32);
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
                                                if previous.leading_zeros() < remaining_shift {
                                                    limbs.push(previous >> shift_complement);
                                                }
                                            }
                                            pad_left(limbs, (other >> LOG_LIMB_BITS) as usize, 0);
                                        });
    }
}
