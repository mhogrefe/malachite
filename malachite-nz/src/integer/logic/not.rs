use integer::Integer;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::NotAssign;
use natural::Natural;
use std::ops::Not;

impl Not for Integer {
    type Output = Integer;

    /// Returns the bitwise complement of an `Integer`, as if it were represented in two's
    /// complement, taking the `Integer` by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((!Integer::ZERO).to_string(), "-1");
    /// assert_eq!((!Integer::from(123)).to_string(), "-124");
    /// assert_eq!((!Integer::from(-123)).to_string(), "122");
    /// ```
    #[inline]
    fn not(mut self) -> Integer {
        self.not_assign();
        self
    }
}

impl<'a> Not for &'a Integer {
    type Output = Integer;

    /// Returns the bitwise complement of an `Integer`, as if it were represented in two's
    /// complement, taking the `Integer` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((!&Integer::ZERO).to_string(), "-1");
    /// assert_eq!((!&Integer::from(123)).to_string(), "-124");
    /// assert_eq!((!&Integer::from(-123)).to_string(), "122");
    /// ```
    fn not(self) -> Integer {
        match *self {
            Integer {
                sign: true,
                ref abs,
            } => Integer {
                sign: false,
                abs: abs.add_limb_ref(1),
            },
            Integer {
                sign: false,
                ref abs,
            } => Integer {
                sign: true,
                abs: abs.sub_limb_ref(1),
            },
        }
    }
}

impl NotAssign for Integer {
    /// Replaces an `Integer` with its bitwise complement, as if it were represented in two's
    /// complement.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::NotAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x.not_assign();
    /// assert_eq!(x.to_string(), "-1");
    ///
    /// let mut x = Integer::from(123);
    /// x.not_assign();
    /// assert_eq!(x.to_string(), "-124");
    ///
    /// let mut x = Integer::from(-123);
    /// x.not_assign();
    /// assert_eq!(x.to_string(), "122");
    /// ```
    fn not_assign(&mut self) {
        if self.sign {
            self.sign = false;
            self.abs += Natural::ONE;
        } else {
            self.sign = true;
            self.abs -= Natural::ONE;
        }
    }
}
