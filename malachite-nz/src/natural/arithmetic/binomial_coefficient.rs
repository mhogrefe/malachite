use crate::natural::arithmetic::div_round::double_cmp;
use crate::natural::exhaustive::exhaustive_natural_inclusive_range;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{BinomialCoefficient, DivExact, Gcd, Parity};
use malachite_base::num::basic::traits::{One, Two, Zero};
use std::cmp::Ordering;

impl BinomialCoefficient for Natural {
    /// Computes the binomial coefficient of two [`Natural`]s, taking both by value.
    ///
    /// $$
    /// f(n, k) = \binom{n}{k} = \frac{n!}{k!(n-k)!}.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BinomialCoefficient;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::binomial_coefficient(Natural::from(4u32), Natural::from(0u32)), 1);
    /// assert_eq!(Natural::binomial_coefficient(Natural::from(4u32), Natural::from(1u32)), 4);
    /// assert_eq!(Natural::binomial_coefficient(Natural::from(4u32), Natural::from(2u32)), 6);
    /// assert_eq!(Natural::binomial_coefficient(Natural::from(4u32), Natural::from(3u32)), 4);
    /// assert_eq!(Natural::binomial_coefficient(Natural::from(4u32), Natural::from(4u32)), 1);
    /// assert_eq!(Natural::binomial_coefficient(Natural::from(10u32), Natural::from(5u32)), 252);
    /// assert_eq!(
    ///     Natural::binomial_coefficient(Natural::from(100u32), Natural::from(50u32)).to_string(),
    ///     "100891344545564193334812497256"
    /// );
    /// ```
    fn binomial_coefficient(n: Natural, mut k: Natural) -> Natural {
        if k > n {
            return Natural::ZERO;
        }
        if k == 0u32 || n == k {
            return Natural::ONE;
        }
        if double_cmp(&k, &n) == Ordering::Greater {
            k = &n - &k;
        }
        if k == 1u32 {
            n
        } else if k == 2u32 {
            (&n >> 1) * (if n.even() { n - Natural::ONE } else { n })
        } else {
            let mut product = n - &k + Natural::ONE;
            let mut numerator = product.clone();
            for i in exhaustive_natural_inclusive_range(Natural::TWO, k) {
                numerator += Natural::ONE;
                let gcd = (&numerator).gcd(&i);
                product /= i.div_exact(&gcd);
                product *= (&numerator).div_exact(gcd);
            }
            product
        }
    }
}

impl<'a> BinomialCoefficient<&'a Natural> for Natural {
    /// Computes the binomial coefficient of two [`Natural`]s, taking both by reference.
    ///
    /// $$
    /// f(n, k) = \binom{n}{k} = \frac{n!}{k!(n-k)!}.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BinomialCoefficient;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(0u32)), 1);
    /// assert_eq!(Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(1u32)), 4);
    /// assert_eq!(Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(2u32)), 6);
    /// assert_eq!(Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(3u32)), 4);
    /// assert_eq!(Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(4u32)), 1);
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(10u32), &Natural::from(5u32)),
    ///     252
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(100u32), &Natural::from(50u32))
    ///         .to_string(),
    ///     "100891344545564193334812497256"
    /// );
    /// ```
    fn binomial_coefficient(n: &'a Natural, k: &'a Natural) -> Natural {
        Natural::binomial_coefficient(n.clone(), k.clone())
    }
}
