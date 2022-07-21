use crate::integer::Integer;
use crate::natural::Natural;
use std::ops::Neg;

impl Neg for Natural {
    type Output = Integer;

    /// Negates a [`Natural`], taking it by value and returning an [`Integer`].
    ///
    /// $$
    /// f(x) = -x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(-Natural::ZERO, 0);
    /// assert_eq!(-Natural::from(123u32), -123);
    /// ```
    fn neg(self) -> Integer {
        Integer::from_sign_and_abs(self == 0, self)
    }
}

impl<'a> Neg for &'a Natural {
    type Output = Integer;

    /// Negates a [`Natural`], taking it by reference and returning an [`Integer`].
    ///
    /// $$
    /// f(x) = -x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(-&Natural::ZERO, 0);
    /// assert_eq!(-&Natural::from(123u32), -123);
    /// ```
    fn neg(self) -> Integer {
        Integer::from_sign_and_abs_ref(*self == 0, self)
    }
}
