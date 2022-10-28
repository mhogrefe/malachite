use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Neg, ModPowerOf2NegAssign, NegModPowerOf2, NegModPowerOf2Assign,
};

impl ModPowerOf2Neg for Natural {
    type Output = Natural;

    /// Negates a [`Natural`] modulo $2^k$. Assumes the input is already reduced modulo $2^k$. The
    /// [`Natural`] is taken by value.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$ and $-x \equiv y \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_2_neg(5), 0);
    /// assert_eq!(Natural::ZERO.mod_power_of_2_neg(100), 0);
    /// assert_eq!(Natural::from(100u32).mod_power_of_2_neg(8), 156);
    /// assert_eq!(
    ///     Natural::from(100u32).mod_power_of_2_neg(100).to_string(),
    ///     "1267650600228229401496703205276"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_neg(mut self, pow: u64) -> Natural {
        self.neg_mod_power_of_2_assign(pow);
        self
    }
}

impl<'a> ModPowerOf2Neg for &'a Natural {
    type Output = Natural;

    /// Negates a [`Natural`] modulo $2^k$. Assumes the input is already reduced modulo $2^k$. The
    /// [`Natural`] is taken by reference.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$ and $-x \equiv y \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_power_of_2_neg(5), 0);
    /// assert_eq!((&Natural::ZERO).mod_power_of_2_neg(100), 0);
    /// assert_eq!((&Natural::from(100u32)).mod_power_of_2_neg(8), 156);
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_power_of_2_neg(100).to_string(),
    ///     "1267650600228229401496703205276"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_neg(self, pow: u64) -> Natural {
        self.neg_mod_power_of_2(pow)
    }
}

impl ModPowerOf2NegAssign for Natural {
    /// Negates a [`Natural`] modulo $2^k$, in place. Assumes the input is already reduced modulo
    /// $2^k$.
    ///
    /// $x \gets y$, where $x, y < 2^p$ and $-x \equiv y \mod 2^p$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_2_neg_assign(5);
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_2_neg_assign(100);
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::from(100u32);
    /// n.mod_power_of_2_neg_assign(8);
    /// assert_eq!(n, 156);
    ///
    /// let mut n = Natural::from(100u32);
    /// n.mod_power_of_2_neg_assign(100);
    /// assert_eq!(n.to_string(), "1267650600228229401496703205276");
    /// ```
    #[inline]
    fn mod_power_of_2_neg_assign(&mut self, pow: u64) {
        self.neg_mod_power_of_2_assign(pow);
    }
}
