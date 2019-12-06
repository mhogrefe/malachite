use std::mem::swap;
use std::ops::{Add, AddAssign};

use malachite_base::num::conversion::traits::Assign;

use integer::Integer;
use natural::Natural;
use platform::Limb;

/// Adds a `Natural` to an `Integer`, taking both by value.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Integer::from(-123) + Natural::ZERO).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + Natural::from(456u32)).to_string(), "333");
///     assert_eq!((-Integer::trillion() + Natural::trillion() * Natural::from(2u32)).to_string(),
///         "1000000000000");
/// }
/// ```
impl Add<Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn add(mut self, other: Natural) -> Integer {
        self += other;
        self
    }
}

/// Adds a `Natural` to an `Integer`, taking the `Integer` by value and the `Natural` by reference.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Integer::from(-123) + &Natural::ZERO).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + &Natural::from(456u32)).to_string(), "333");
///     assert_eq!(
///         (-Integer::trillion() + &(Natural::trillion() * Natural::from(2u32))).to_string(),
///         "1000000000000"
///     );
/// }
/// ```
impl<'a> Add<&'a Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn add(mut self, other: &'a Natural) -> Integer {
        self += other;
        self
    }
}

/// Adds a `Natural` to an `Integer`, taking the `Integer` by reference and the `Natural` by value.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO + Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + Natural::ZERO).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + Natural::from(456u32)).to_string(), "333");
///     assert_eq!(
///         (&(-Integer::trillion()) + Natural::trillion() * Natural::from(2u32)).to_string(),
///         "1000000000000"
///     );
/// }
/// ```
impl<'a> Add<Natural> for &'a Integer {
    type Output = Integer;

    fn add(self, mut other: Natural) -> Integer {
        if self.sign {
            other += &self.abs;
            Integer::from(other)
        } else if other >= self.abs {
            other -= &self.abs;
            Integer::from(other)
        } else {
            other.sub_right_assign_no_panic(&self.abs);
            -other
        }
    }
}

/// Adds a `Natural` to an `Integer`, taking both by reference.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO + &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + &Natural::ZERO).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + &Natural::from(456u32)).to_string(), "333");
///     assert_eq!(
///         (&(-Integer::trillion()) + &(Natural::trillion() * Natural::from(2u32))).to_string(),
///         "1000000000000"
///     );
/// }
/// ```
impl<'a, 'b> Add<&'a Natural> for &'b Integer {
    type Output = Integer;

    fn add(self, other: &'a Natural) -> Integer {
        if *self == 0 as Limb {
            Integer::from(other)
        } else if *other == 0 as Limb {
            self.clone()
        } else {
            match (self, other) {
                // e.g. 10 + 5 or -10 + -5; sign of result is sign of self
                (
                    &Integer {
                        sign: sx,
                        abs: ref ax,
                    },
                    ay,
                ) if sx == (*ay != 0 as Limb) => Integer {
                    sign: sx,
                    abs: ax + ay,
                },
                // e.g. -10 + 5; sign of result is sign of self
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
                // e.g. -5 + 10 or -5 + 5; sign of result is sign of other
                (&Integer { abs: ref ax, .. }, ay) => Integer::from(ay - ax),
            }
        }
    }
}

/// Adds a `Natural` to an `Integer` in place, taking the `Natural` by value.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x += Natural::trillion();
///     x += Natural::trillion() * Natural::from(2u32);
///     x += Natural::trillion() * Natural::from(3u32);
///     x += Natural::trillion() * Natural::from(4u32);
///     assert_eq!(x.to_string(), "10000000000000");
/// }
/// ```
impl AddAssign<Natural> for Integer {
    fn add_assign(&mut self, mut other: Natural) {
        if other == 0 as Limb {
            return;
        } else if *self == 0 as Limb {
            self.abs = other;
            return;
        }
        let add_strategy = match (&mut (*self), &other) {
            (&mut Integer { sign: sx, .. }, ay) if sx == (*ay != 0 as Limb) => 0,
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                ay,
            ) if sx && *ax == *ay || *ax > *ay => 1,
            _ => 2,
        };
        match add_strategy {
            // e.g. 10 + 5; sign of self is unchanged
            0 => self.abs += other,
            // e.g. -10 + 5; sign of self is unchanged
            1 => self.abs -= other,
            // e.g. -5 + 10 or -5 + 5; sign of self is flipped
            _ => {
                swap(&mut self.abs, &mut other);
                self.sign = true;
                self.abs -= other;
            }
        }
    }
}

/// Adds a `Natural` to an `Integer` in place, taking the `Natural` by reference.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x += &Natural::trillion();
///     x += &(Natural::trillion() * Natural::from(2u32));
///     x += &(Natural::trillion() * Natural::from(3u32));
///     x += &(Natural::trillion() * Natural::from(4u32));
///     assert_eq!(x.to_string(), "10000000000000");
/// }
/// ```
impl<'a> AddAssign<&'a Natural> for Integer {
    fn add_assign(&mut self, other: &'a Natural) {
        if *other == 0 as Limb {
            return;
        } else if *self == 0 as Limb {
            self.assign(other);
            return;
        }
        let add_strategy = match (&mut (*self), other) {
            (&mut Integer { sign: sx, .. }, ay) if sx == (*ay != 0 as Limb) => 0,
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                ay,
            ) if sx && *ax == *ay || *ax > *ay => 1,
            _ => 2,
        };
        match add_strategy {
            // e.g. 10 + 5; sign of self is unchanged
            0 => self.abs += other,
            // e.g. -10 + 5; sign of self is unchanged
            1 => self.abs -= other,
            // e.g. -5 + 10 or -5 + 5; sign of self is flipped
            _ => {
                self.sign = true;
                self.abs.sub_right_assign_no_panic(other);
            }
        }
    }
}

/// Adds an `Integer` to a `Natural`, taking both by value.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::from(123u32) + Integer::ZERO).to_string(), "123");
///     assert_eq!((Natural::ZERO + Integer::from(-123)).to_string(), "-123");
///     assert_eq!((Natural::from(456u32) + Integer::from(-123)).to_string(), "333");
///     assert_eq!((Natural::trillion() * Natural::from(2u32) + (-Integer::trillion())).to_string(),
///         "1000000000000");
/// }
/// ```
impl Add<Integer> for Natural {
    type Output = Integer;

    #[inline]
    fn add(self, other: Integer) -> Integer {
        other + self
    }
}

/// Adds an `Integer` to a `Natural`, taking the `Natural` by value and the `Integer` by reference.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::from(123u32) + &Integer::ZERO).to_string(), "123");
///     assert_eq!((Natural::ZERO + &Integer::from(-123)).to_string(), "-123");
///     assert_eq!((Natural::from(456u32) + &Integer::from(-123)).to_string(), "333");
///     assert_eq!((Natural::trillion() * Natural::from(2u32) + &-Integer::trillion()).to_string(),
///         "1000000000000");
/// }
/// ```
impl<'a> Add<&'a Integer> for Natural {
    type Output = Integer;

    #[inline]
    fn add(self, other: &'a Integer) -> Integer {
        other + self
    }
}

/// Adds an `Integer` to a `Natural`, taking the `Natural` by reference and the `Integer` by value.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(123u32) + Integer::ZERO).to_string(), "123");
///     assert_eq!((&Natural::ZERO + Integer::from(-123)).to_string(), "-123");
///     assert_eq!((&Natural::from(456u32) + Integer::from(-123)).to_string(), "333");
///     assert_eq!(
///         (&Natural::trillion() * Natural::from(2u32) + (-Integer::trillion())).to_string(),
///         "1000000000000"
///     );
/// }
/// ```
impl<'a> Add<Integer> for &'a Natural {
    type Output = Integer;

    #[inline]
    fn add(self, other: Integer) -> Integer {
        other + self
    }
}

/// Adds an `Integer` to a `Natural`, taking both by reference.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(123u32) + Integer::ZERO).to_string(), "123");
///     assert_eq!((&Natural::ZERO + Integer::from(-123)).to_string(), "-123");
///     assert_eq!((&Natural::from(456u32) + Integer::from(-123)).to_string(), "333");
///     assert_eq!(
///         (&Natural::trillion() * Natural::from(2u32) + (-Integer::trillion())).to_string(),
///         "1000000000000"
///     );
/// }
/// ```
impl<'a, 'b> Add<&'a Integer> for &'b Natural {
    type Output = Integer;

    #[inline]
    fn add(self, other: &'a Integer) -> Integer {
        other + self
    }
}
