use std::mem::swap;
use std::ops::{Sub, SubAssign};

use malachite_base::num::traits::{Assign, NegAssign, NotAssign};

use integer::Integer;
use natural::Natural;
use platform::Limb;

/// Subtracts a `Natural` from an `Integer`, taking both inputs by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Integer::ZERO - Natural::from(123u32)).to_string(), "-123");
///     assert_eq!((Integer::from(123) - Natural::ZERO).to_string(), "123");
///     assert_eq!((Integer::from(-456) - Natural::from(123u32)).to_string(), "-579");
///     assert_eq!((Integer::trillion() - Natural::trillion() * 2u32).to_string(),
///         "-1000000000000");
/// }
/// ```
impl Sub<Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn sub(mut self, other: Natural) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts a `Natural` from an `Integer`, taking the `Integer` by value and the right `Natural`
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Integer::ZERO - &Natural::from(123u32)).to_string(), "-123");
///     assert_eq!((Integer::from(123) - &Natural::ZERO).to_string(), "123");
///     assert_eq!((Integer::from(-456) - &Natural::from(123u32)).to_string(), "-579");
///     assert_eq!((Integer::trillion() - &(Natural::trillion() * 2u32)).to_string(),
///         "-1000000000000");
/// }
/// ```
impl<'a> Sub<&'a Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn sub(mut self, other: &'a Natural) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts a `Natural` from an `Integer`, taking the `Integer` by reference and the `Natural` by
/// value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO - Natural::from(123u32)).to_string(), "-123");
///     assert_eq!((&Integer::from(123) - Natural::ZERO).to_string(), "123");
///     assert_eq!((&Integer::from(-456) - Natural::from(123u32)).to_string(), "-579");
///     assert_eq!((&Integer::trillion() - Natural::trillion() * 2u32).to_string(),
///         "-1000000000000");
/// }
/// ```
impl<'a> Sub<Natural> for &'a Integer {
    type Output = Integer;

    fn sub(self, other: Natural) -> Integer {
        if *self == 0 as Limb {
            -other.clone()
        } else if other == 0 as Limb {
            self.clone()
        } else {
            match (self, other) {
                // e.g. -10 - 5; sign of result is sign of self
                (
                    &Integer {
                        sign: false,
                        abs: ref ax,
                    },
                    ref ay,
                ) => Integer {
                    sign: false,
                    abs: ax + ay,
                },
                // e.g. 10 - 5 or 5 - 5; sign of result is sign of self
                (
                    &Integer {
                        sign: sx,
                        abs: ref ax,
                    },
                    ref ay,
                ) if sx && *ax == *ay || *ax > *ay => Integer {
                    sign: sx,
                    abs: ax - ay,
                },
                // e.g. 5 - 10; sign of result is opposite of sign of other
                (&Integer { abs: ref ax, .. }, ref ay) => Integer {
                    sign: false,
                    abs: ay - ax,
                },
            }
        }
    }
}

/// Subtracts a `Natural` from an `Integer`, taking both inputs by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO - &Natural::from(123u32)).to_string(), "-123");
///     assert_eq!((&Integer::from(123) - &Natural::ZERO).to_string(), "123");
///     assert_eq!((&Integer::from(-456) - &Natural::from(123u32)).to_string(), "-579");
///     assert_eq!((&Integer::trillion() - &(Natural::trillion() * 2)).to_string(),
///         "-1000000000000");
/// }
/// ```
impl<'a, 'b> Sub<&'a Natural> for &'b Integer {
    type Output = Integer;

    fn sub(self, other: &'a Natural) -> Integer {
        if *self == 0 as Limb {
            -other.clone()
        } else if *other == 0 as Limb {
            self.clone()
        } else {
            match (self, other) {
                // e.g. -10 - 5; sign of result is sign of self
                (
                    &Integer {
                        sign: false,
                        abs: ref ax,
                    },
                    ay,
                ) => Integer {
                    sign: false,
                    abs: ax + ay,
                },
                // e.g. 10 - 5 or 5 - 5; sign of result is sign of self
                (
                    &Integer {
                        sign: sx,
                        abs: ref ax,
                    },
                    ay,
                ) if sx && *ax == *ay || *ax > *ay => Integer {
                    sign: sx,
                    abs: ax - ay,
                },
                // e.g. 5 - 10; sign of result is opposite of sign of other
                (&Integer { abs: ref ax, .. }, ay) => Integer {
                    sign: false,
                    abs: ay - ax,
                },
            }
        }
    }
}

/// Subtracts a `Natural` from an `Integer` in place, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x -= Natural::trillion();
///     x -= Natural::trillion() * 2;
///     x -= Natural::trillion() * 3;
///     x -= Natural::trillion() * 4;
///     assert_eq!(x.to_string(), "-10000000000000");
/// }
/// ```
impl SubAssign<Natural> for Integer {
    fn sub_assign(&mut self, mut other: Natural) {
        if other == 0 as Limb {
            return;
        } else if *self == 0 as Limb {
            self.assign(other);
            self.neg_assign();
            return;
        }
        let add_strategy = match (&mut (*self), &other) {
            (&mut Integer { sign: false, .. }, _) => 0,
            (
                &mut Integer {
                    sign: true,
                    abs: ref mut ax,
                },
                ay,
            ) if *ax >= *ay => 1,
            _ => 2,
        };
        match add_strategy {
            // e.g. -10 - 5; sign of self is unchanged
            0 => self.abs += other,
            // e.g. 10 - 5 or 5 - 5; sign of self is unchanged
            1 => self.abs -= other,
            // e.g. 5 - 10; sign of self is flipped
            _ => {
                swap(&mut self.abs, &mut other);
                self.abs -= other;
                self.sign.not_assign();
            }
        }
    }
}

/// Subtracts a `Natural` from an `Integer` in place, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x -= &Natural::trillion();
///     x -= &(Natural::trillion() * 2);
///     x -= &(Natural::trillion() * 3);
///     x -= &(Natural::trillion() * 4);
///     assert_eq!(x.to_string(), "-10000000000000");
/// }
/// ```
impl<'a> SubAssign<&'a Natural> for Integer {
    fn sub_assign(&mut self, other: &'a Natural) {
        if *other == 0 as Limb {
            return;
        } else if *self == 0 as Limb {
            *self = -other;
            return;
        }
        let add_strategy = match (&mut (*self), other) {
            (&mut Integer { sign: false, .. }, _) => 0,
            (
                &mut Integer {
                    sign: true,
                    abs: ref mut ax,
                },
                ay,
            ) if *ax >= *ay => 1,
            _ => 2,
        };
        match add_strategy {
            // e.g. -10 - 5; sign of self is unchanged
            0 => self.abs += other,
            // e.g. 10 - 5 or 5 - 5; sign of self is unchanged
            1 => self.abs -= other,
            // e.g. 5 - 10; sign of self is flipped
            _ => {
                *self = Integer {
                    sign: false,
                    abs: other - &self.abs,
                }
            }
        }
    }
}

/// Subtracts an `Integer` from a `Natural`, taking both inputs by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO - Integer::from(123)).to_string(), "-123");
///     assert_eq!((Natural::from(123u32) - Integer::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(456u32) - Integer::from(-123)).to_string(), "579");
///     assert_eq!((Natural::trillion() - Integer::trillion() * 2u32).to_string(),
///         "-1000000000000");
/// }
/// ```
impl Sub<Integer> for Natural {
    type Output = Integer;

    fn sub(self, other: Integer) -> Integer {
        -(other - self)
    }
}

/// Subtracts an `Integer` from a `Natural`, taking the `Natural` by value and the `Integer` by
/// reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO - &Integer::from(123)).to_string(), "-123");
///     assert_eq!((Natural::from(123u32) - &Integer::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(456u32) - &Integer::from(-123)).to_string(), "579");
///     assert_eq!((Natural::trillion() - &(Integer::trillion() * 2u32)).to_string(),
///         "-1000000000000");
/// }
/// ```
impl<'a> Sub<&'a Integer> for Natural {
    type Output = Integer;

    fn sub(self, other: &'a Integer) -> Integer {
        -(other - self)
    }
}

/// Subtracts an `Integer` from a `Natural`, taking the `Natural` by reference and the `Integer` by
/// value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO - Integer::from(123)).to_string(), "-123");
///     assert_eq!((&Natural::from(123u32) - Integer::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(456u32) - Integer::from(-123)).to_string(), "579");
///     assert_eq!((&Natural::trillion() - Integer::trillion() * 2u32).to_string(),
///         "-1000000000000");
/// }
/// ```
impl<'a> Sub<Integer> for &'a Natural {
    type Output = Integer;

    fn sub(self, other: Integer) -> Integer {
        -(other - self)
    }
}

/// Subtracts an `Integer` from a `Natural`, taking both inputs by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO - &Integer::from(123)).to_string(), "-123");
///     assert_eq!((&Natural::from(123u32) - &Integer::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(456u32) - &Integer::from(-123)).to_string(), "579");
///     let x: Integer = &Natural::trillion() - &(Integer::trillion() * 2);
///     assert_eq!(x.to_string(), "-1000000000000");
/// }
/// ```
impl<'a, 'b> Sub<&'a Integer> for &'b Natural {
    type Output = Integer;

    fn sub(self, other: &'a Integer) -> Integer {
        -(other - self)
    }
}
