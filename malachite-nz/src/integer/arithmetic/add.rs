use integer::Integer;
use std::mem::swap;
use std::ops::{Add, AddAssign};

/// Adds an `Integer` to an `Integer`, taking both `Integer`s by value.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + Integer::from(123)).to_string(), "123");
///     assert_eq!((Integer::from(-123) + Integer::ZERO).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + Integer::from(456)).to_string(), "333");
///     assert_eq!((-Integer::trillion() + Integer::trillion() * 2u32).to_string(),
///         "1000000000000");
/// }
/// ```
impl Add<Integer> for Integer {
    type Output = Integer;

    fn add(mut self, mut other: Integer) -> Integer {
        if self.abs.limb_count() >= other.abs.limb_count() {
            self += other;
            self
        } else {
            other += self;
            other
        }
    }
}

/// Adds an `Integer` to an `Integer`, taking the left `Integer` by value and the right `Integer` by
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + &Integer::from(123)).to_string(), "123");
///     assert_eq!((Integer::from(-123) + &Integer::ZERO).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + &Integer::from(456)).to_string(), "333");
///     assert_eq!((-Integer::trillion() + &(Integer::trillion() * 2u32)).to_string(),
///         "1000000000000");
/// }
/// ```
impl<'a> Add<&'a Integer> for Integer {
    type Output = Integer;

    fn add(mut self, other: &'a Integer) -> Integer {
        self += other;
        self
    }
}

/// Adds an `Integer` to an `Integer`, taking the left `Integer` by reference and the right
/// `Integer` by value.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO + Integer::from(123)).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + Integer::ZERO).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + Integer::from(456)).to_string(), "333");
///     assert_eq!((&(-Integer::trillion()) + Integer::trillion() * 2u32).to_string(),
///         "1000000000000");
/// }
/// ```
impl<'a> Add<Integer> for &'a Integer {
    type Output = Integer;

    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

/// Adds an `Integer` to an `Integer`, taking both `Integer`s by reference.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO + &Integer::from(123)).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + &Integer::ZERO).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + &Integer::from(456)).to_string(), "333");
///     assert_eq!((&(-Integer::trillion()) + &(Integer::trillion() * 2u32)).to_string(),
///         "1000000000000");
/// }
/// ```
impl<'a, 'b> Add<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn add(self, other: &'a Integer) -> Integer {
        if self as *const Integer == other as *const Integer {
            self << 1
        } else if *self == 0 {
            other.clone()
        } else if *other == 0 {
            self.clone()
        } else {
            match (self, other) {
                // e.g. 10 + 5 or -10 + -5; sign of result is sign of self
                (
                    &Integer {
                        sign: sx,
                        abs: ref ax,
                    },
                    &Integer {
                        sign: sy,
                        abs: ref ay,
                    },
                ) if sx == (sy && *ay != 0) =>
                {
                    Integer {
                        sign: sx,
                        abs: ax + ay,
                    }
                }
                // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of result is sign of self
                (
                    &Integer {
                        sign: sx,
                        abs: ref ax,
                    },
                    &Integer { abs: ref ay, .. },
                ) if sx && *ax == *ay || *ax > *ay =>
                {
                    Integer {
                        sign: sx,
                        abs: ax - ay,
                    }
                }
                // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of result is sign of other
                (
                    &Integer { abs: ref ax, .. },
                    &Integer {
                        sign: sy,
                        abs: ref ay,
                    },
                ) => Integer {
                    sign: sy,
                    abs: ay - ax,
                },
            }
        }
    }
}

/// Adds an `Integer` to an `Integer` in place, taking the `Integer` on the RHS by value.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x += -Integer::trillion();
///     x += Integer::trillion() * 2;
///     x += -Integer::trillion() * 3;
///     x += Integer::trillion() * 4;
///     assert_eq!(x.to_string(), "2000000000000");
/// }
/// ```
impl AddAssign<Integer> for Integer {
    fn add_assign(&mut self, mut other: Integer) {
        if other == 0 {
            return;
        } else if *self == 0 {
            *self = other;
            return;
        }
        let add_strategy = match (&mut (*self), &other) {
            (
                &mut Integer { sign: sx, .. },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (sy && *ay != 0) =>
            {
                0
            }
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay =>
            {
                1
            }
            _ => 2,
        };
        match add_strategy {
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            0 => self.abs += other.abs,
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            1 => self.abs -= &other.abs,
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            _ => {
                swap(self, &mut other);
                self.abs -= &other.abs;
            }
        }
    }
}

/// Adds an `Integer` to an `Integer` in place, taking the `Integer` on the RHS by reference.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x += &(-Integer::trillion());
///     x += &(Integer::trillion() * 2);
///     x += &(-Integer::trillion() * 3);
///     x += &(Integer::trillion() * 4);
///     assert_eq!(x.to_string(), "2000000000000");
/// }
/// ```
impl<'a> AddAssign<&'a Integer> for Integer {
    fn add_assign(&mut self, other: &'a Integer) {
        if *other == 0 {
            return;
        } else if *self == 0 {
            *self = other.clone();
            return;
        }
        let add_strategy = match (&mut (*self), other) {
            (
                &mut Integer { sign: sx, .. },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (sy && *ay != 0) =>
            {
                0
            }
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay =>
            {
                1
            }
            _ => 2,
        };
        match add_strategy {
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            0 => self.abs += &other.abs,
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            1 => self.abs -= &other.abs,
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            _ => {
                *self = Integer {
                    sign: other.sign,
                    abs: &other.abs - &self.abs,
                }
            }
        }
    }
}
