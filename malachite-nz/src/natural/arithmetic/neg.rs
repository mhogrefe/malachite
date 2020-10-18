use integer::Integer;
use natural::Natural;
use std::ops::Neg;

impl Neg for Natural {
    type Output = Integer;

    /// Returns the negative of a `Natural`, taking the `Natural` by value and returning an
    /// `Integer`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((-Natural::ZERO).to_string(), "0");
    /// assert_eq!((-Natural::from(123u32)).to_string(), "-123");
    /// ```
    fn neg(self) -> Integer {
        if self == 0 {
            Integer {
                sign: true,
                abs: self,
            }
        } else {
            Integer {
                sign: false,
                abs: self,
            }
        }
    }
}

impl<'a> Neg for &'a Natural {
    type Output = Integer;

    /// Returns the negative of a `Natural`, taking the `Natural` by reference and returning an
    /// `Integer`.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((-&Natural::ZERO).to_string(), "0");
    /// assert_eq!((-&Natural::from(123u32)).to_string(), "-123");
    /// ```
    fn neg(self) -> Integer {
        if *self == 0 {
            Integer {
                sign: true,
                abs: self.clone(),
            }
        } else {
            Integer {
                sign: false,
                abs: self.clone(),
            }
        }
    }
}
