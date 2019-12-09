use std::mem::swap;
use std::ops::{Add, AddAssign};

use malachite_base::num::conversion::traits::CheckedFrom;

use integer::Integer;
use natural::Natural;
use platform::Limb;

impl Integer {
    pub(crate) fn add_assign_limb(&mut self, other: Limb) {
        if other == 0 {
            return;
        }
        if *self == 0 as Limb {
            *self = Integer::from(other);
            return;
        }
        match *self {
            // e.g. 10 + 5; self stays positive
            Integer {
                sign: true,
                ref mut abs,
            } => abs.add_assign_limb(other),
            // e.g. -10 + 5; self stays negative
            Integer {
                sign: false,
                ref mut abs,
            } if *abs > other => abs.sub_assign_limb(other),
            // e.g. -5 + 10 or -5 + 5; self becomes non-negative
            Integer {
                ref mut sign,
                ref mut abs,
            } => {
                *sign = true;
                let small_abs = Limb::checked_from(&*abs).unwrap();
                *abs = Natural::from(other - small_abs);
            }
        }
    }
}

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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + Integer::from(123)).to_string(), "123");
///     assert_eq!((Integer::from(-123) + Integer::ZERO).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + Integer::from(456)).to_string(), "333");
///     assert_eq!((-Integer::trillion() + Integer::trillion() * Integer::from(2u32)).to_string(),
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO + &Integer::from(123)).to_string(), "123");
///     assert_eq!((Integer::from(-123) + &Integer::ZERO).to_string(), "-123");
///     assert_eq!((Integer::from(-123) + &Integer::from(456)).to_string(), "333");
///     assert_eq!(
///         (-Integer::trillion() + &(Integer::trillion() * Integer::from(2u32))).to_string(),
///         "1000000000000"
///     );
/// }
/// ```
impl<'a> Add<&'a Integer> for Integer {
    type Output = Integer;

    #[inline]
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO + Integer::from(123)).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + Integer::ZERO).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + Integer::from(456)).to_string(), "333");
///     assert_eq!(
///         (&(-Integer::trillion()) + Integer::trillion() * Integer::from(2u32)).to_string(),
///         "1000000000000"
///     );
/// }
/// ```
impl<'a> Add<Integer> for &'a Integer {
    type Output = Integer;

    #[inline]
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO + &Integer::from(123)).to_string(), "123");
///     assert_eq!((&Integer::from(-123) + &Integer::ZERO).to_string(), "-123");
///     assert_eq!((&Integer::from(-123) + &Integer::from(456)).to_string(), "333");
///     assert_eq!(
///         (&(-Integer::trillion()) + &(Integer::trillion() * Integer::from(2u32))).to_string(),
///         "1000000000000"
///     );
/// }
/// ```
impl<'a, 'b> Add<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn add(self, other: &'a Integer) -> Integer {
        if self as *const Integer == other as *const Integer {
            self << 1
        } else if *self == 0 as Limb {
            other.clone()
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
                    &Integer {
                        sign: sy,
                        abs: ref ay,
                    },
                ) if sx == (sy && *ay != 0 as Limb) => Integer {
                    sign: sx,
                    abs: ax + ay,
                },
                // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of result is sign of self
                (
                    &Integer {
                        sign: sx,
                        abs: ref ax,
                    },
                    &Integer { abs: ref ay, .. },
                ) if sx && *ax == *ay || *ax > *ay => Integer {
                    sign: sx,
                    abs: ax - ay,
                },
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x += -Integer::trillion();
///     x += Integer::trillion() * Integer::from(2u32);
///     x += -Integer::trillion() * Integer::from(3u32);
///     x += Integer::trillion() * Integer::from(4u32);
///     assert_eq!(x.to_string(), "2000000000000");
/// }
/// ```
impl AddAssign<Integer> for Integer {
    fn add_assign(&mut self, mut other: Integer) {
        if other == 0 as Limb {
            return;
        } else if *self == 0 as Limb {
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
            ) if sx == (sy && *ay != 0 as Limb) => 0,
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => 1,
            _ => 2,
        };
        match add_strategy {
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            0 => self.abs += other.abs,
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            1 => self.abs -= other.abs,
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            _ => {
                swap(self, &mut other);
                self.abs -= other.abs;
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x += &(-Integer::trillion());
///     x += &(Integer::trillion() * Integer::from(2u32));
///     x += &(-Integer::trillion() * Integer::from(3u32));
///     x += &(Integer::trillion() * Integer::from(4u32));
///     assert_eq!(x.to_string(), "2000000000000");
/// }
/// ```
impl<'a> AddAssign<&'a Integer> for Integer {
    fn add_assign(&mut self, other: &'a Integer) {
        if *other == 0 as Limb {
            return;
        } else if *self == 0 as Limb {
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
            ) if sx == (sy && *ay != 0 as Limb) => 0,
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => 1,
            _ => 2,
        };
        match add_strategy {
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            0 => self.abs += &other.abs,
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            1 => self.abs -= &other.abs,
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            _ => {
                self.sign = other.sign;
                self.abs.sub_right_assign_no_panic(&other.abs);
            }
        }
    }
}
