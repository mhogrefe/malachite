use natural::Natural::{self, Large, Small};
use std::ops::{Add, AddAssign};
use traits::Assign;

/// Adds a `u32` to a `Natural`. This implementation takes `self` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) + 123).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + 0).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + 456).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + 123).to_string(), "1000000000123");
/// ```
impl Add<u32> for Natural {
    type Output = Natural;

    fn add(mut self, other: u32) -> Natural {
        self.add_assign(other);
        self
    }
}

/// Adds a `u32` to a `Natural`. This implementation takes `self` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(0u32) + 123).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + 0).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + 456).to_string(), "579");
/// assert_eq!((&Natural::from_str("1000000000000").unwrap() + 123).to_string(), "1000000000123");
/// ```
impl<'a> Add<u32> for &'a Natural {
    type Output = Natural;

    fn add(self, other: u32) -> Natural {
        if other == 0 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                match small.overflowing_add(other) {
                    (sum, false) => Small(sum),
                    (sum, true) => Large(vec![sum, 1]),
                }
            }
            Large(ref limbs) => {
                let mut sum_limbs = Vec::with_capacity(limbs.len());
                let mut addend = other;
                for limb in limbs {
                    if addend == 0 {
                        sum_limbs.push(*limb);
                    } else {
                        let (sum, overflow) = limb.overflowing_add(addend);
                        sum_limbs.push(sum);
                        if overflow {
                            addend = 1;
                        } else {
                            addend = 0;
                        }
                    }
                }
                if addend == 1 {
                    sum_limbs.push(1);
                }
                Large(sum_limbs)
            }
        }
    }
}

/// Adds a `Natural` to a `u32`. This implementation takes `other` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((123 + Natural::from(0u32)).to_string(), "123");
/// assert_eq!((0 + Natural::from(123u32)).to_string(), "123");
/// assert_eq!((456 + Natural::from(123u32)).to_string(), "579");
/// assert_eq!((123 + Natural::from_str("1000000000000").unwrap()).to_string(), "1000000000123");
/// ```
impl Add<Natural> for u32 {
    type Output = Natural;

    fn add(self, mut other: Natural) -> Natural {
        other += self;
        other
    }
}

/// Adds a `Natural` to a `u32`. This implementation takes `other` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((123 + &Natural::from(0u32)).to_string(), "123");
/// assert_eq!((0 + &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((456 + &Natural::from(123u32)).to_string(), "579");
/// assert_eq!((123 + &Natural::from_str("1000000000000").unwrap()).to_string(), "1000000000123");
/// ```
impl<'a> Add<&'a Natural> for u32 {
    type Output = Natural;

    fn add(self, other: &'a Natural) -> Natural {
        other + self
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
        if *self == 0 {
            self.assign(other);
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
