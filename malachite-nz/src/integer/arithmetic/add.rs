use std::mem::swap;
use std::ops::{Add, AddAssign};

use integer::Integer;
use natural::InnerNatural::Small;
use natural::Natural;

impl Add<Integer> for Integer {
    type Output = Integer;

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
    /// assert_eq!((Integer::ZERO + Integer::from(123)).to_string(), "123");
    /// assert_eq!((Integer::from(-123) + Integer::ZERO).to_string(), "-123");
    /// assert_eq!((Integer::from(-123) + Integer::from(456)).to_string(), "333");
    /// assert_eq!((-Integer::trillion() + Integer::trillion() * Integer::from(2u32)).to_string(),
    ///     "1000000000000");
    /// ```
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

impl<'a> Add<&'a Integer> for Integer {
    type Output = Integer;

    /// Adds an `Integer` to an `Integer`, taking the left `Integer` by value and the right
    /// `Integer` by reference.
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
    /// assert_eq!((Integer::ZERO + &Integer::from(123)).to_string(), "123");
    /// assert_eq!((Integer::from(-123) + &Integer::ZERO).to_string(), "-123");
    /// assert_eq!((Integer::from(-123) + &Integer::from(456)).to_string(), "333");
    /// assert_eq!(
    ///     (-Integer::trillion() + &(Integer::trillion() * Integer::from(2u32))).to_string(),
    ///     "1000000000000"
    /// );
    /// ```
    #[inline]
    fn add(mut self, other: &'a Integer) -> Integer {
        self += other;
        self
    }
}

impl<'a> Add<Integer> for &'a Integer {
    type Output = Integer;

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
    /// assert_eq!((&Integer::ZERO + Integer::from(123)).to_string(), "123");
    /// assert_eq!((&Integer::from(-123) + Integer::ZERO).to_string(), "-123");
    /// assert_eq!((&Integer::from(-123) + Integer::from(456)).to_string(), "333");
    /// assert_eq!(
    ///     (&(-Integer::trillion()) + Integer::trillion() * Integer::from(2u32)).to_string(),
    ///     "1000000000000"
    /// );
    /// ```
    #[inline]
    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

impl<'a, 'b> Add<&'a Integer> for &'b Integer {
    type Output = Integer;

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
    /// assert_eq!((&Integer::ZERO + &Integer::from(123)).to_string(), "123");
    /// assert_eq!((&Integer::from(-123) + &Integer::ZERO).to_string(), "-123");
    /// assert_eq!((&Integer::from(-123) + &Integer::from(456)).to_string(), "333");
    /// assert_eq!(
    ///     (&(-Integer::trillion()) + &(Integer::trillion() * Integer::from(2u32))).to_string(),
    ///     "1000000000000"
    /// );
    /// ```
    fn add(self, other: &'a Integer) -> Integer {
        match (self, other) {
            (x, y) if x as *const Integer == y as *const Integer => x << 1,
            (&integer_zero!(), y) => y.clone(),
            (x, &integer_zero!()) => x.clone(),
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
            ) if sx == (sy && *ay != 0) => Integer {
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

impl AddAssign<Integer> for Integer {
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
    /// let mut x = Integer::ZERO;
    /// x += -Integer::trillion();
    /// x += Integer::trillion() * Integer::from(2u32);
    /// x += -Integer::trillion() * Integer::from(3u32);
    /// x += Integer::trillion() * Integer::from(4u32);
    /// assert_eq!(x.to_string(), "2000000000000");
    /// ```
    fn add_assign(&mut self, mut other: Integer) {
        match (&mut *self, &other) {
            (_, &integer_zero!()) => {}
            (&mut integer_zero!(), _) => {
                *self = other;
            }
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (sy && *ay != 0) => *ax += ay,
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => *ax -= ay,
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            _ => {
                swap(self, &mut other);
                self.abs -= other.abs;
            }
        };
    }
}

impl<'a> AddAssign<&'a Integer> for Integer {
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
    /// let mut x = Integer::ZERO;
    /// x += &(-Integer::trillion());
    /// x += &(Integer::trillion() * Integer::from(2u32));
    /// x += &(-Integer::trillion() * Integer::from(3u32));
    /// x += &(Integer::trillion() * Integer::from(4u32));
    /// assert_eq!(x.to_string(), "2000000000000");
    /// ```
    fn add_assign(&mut self, other: &'a Integer) {
        match (&mut *self, other) {
            (_, &integer_zero!()) => {}
            (&mut integer_zero!(), _) => {
                *self = other.clone();
            }
            // e.g. 10 + 5 or -10 + -5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (sy && *ay != 0) => *ax += ay,
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => *ax -= ay,
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of self is flipped
            (
                &mut Integer {
                    sign: ref mut sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) => {
                *sx = sy;
                ax.sub_right_assign_no_panic(ay);
            }
        };
    }
}
