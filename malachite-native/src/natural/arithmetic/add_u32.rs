use natural::Natural::{self, Large, Small};
use std::ops::{Add, AddAssign};

/// Adds a `u32` to a `Natural`, taking ownership of the input `Natural`.
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0) + 123).to_string(), "123");
/// assert_eq!((Natural::from(123) + 0).to_string(), "123");
/// assert_eq!((Natural::from(123) + 456).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + 123).to_string(), "1000000000123");
/// ```
impl Add<u32> for Natural {
    type Output = Natural;

    fn add(mut self, op: u32) -> Natural {
        self.add_assign(op);
        self
    }
}

/// Adds a `Natural` to a `u32`, taking ownership of the input `Natural`.
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((123 + Natural::from(0)).to_string(), "123");
/// assert_eq!((0 + Natural::from(123)).to_string(), "123");
/// assert_eq!((456 + Natural::from(123)).to_string(), "579");
/// assert_eq!((123 + Natural::from_str("1000000000000").unwrap()).to_string(), "1000000000123");
/// ```
impl Add<Natural> for u32 {
    type Output = Natural;

    fn add(self, mut op: Natural) -> Natural {
        op.add_assign(self);
        op
    }
}

/// Adds a `u32` to a `Natural` in place.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// let mut x = Natural::new();
/// x += 1;
/// x += 2;
/// x += 3;
/// x += 4;
/// assert_eq!(x.to_string(), "10");
/// ```
impl AddAssign<u32> for Natural {
    fn add_assign(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        mutate_with_possible_promotion!(self,
                                        small,
                                        limbs,
                                        {
                                            small.checked_add(other)
                                        },
                                        {
                                            let mut addend = other;
                                            for limb in limbs.iter_mut() {
                                                let (sum, overflow) = limb.overflowing_add(addend);
                                                *limb = sum;
                                                if overflow {
                                                    addend = 1;
                                                } else {
                                                    addend = 0;
                                                    break;
                                                }
                                            }
                                            if addend == 1 {
                                                limbs.push(1);
                                            }
                                        });
    }
}
