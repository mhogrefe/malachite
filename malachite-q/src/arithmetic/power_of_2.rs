use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;
use Rational;

impl PowerOf2<u64> for Rational {
    /// Raises 2 to an integer power.
    ///
    /// $f(k) = 2^k$.
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
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::power_of_2(0u64), 1);
    /// assert_eq!(Rational::power_of_2(3u64), 8);
    /// assert_eq!(Rational::power_of_2(100u64).to_string(), "1267650600228229401496703205376");
    /// ```
    fn power_of_2(pow: u64) -> Rational {
        Rational::from(Natural::power_of_2(pow))
    }
}

impl PowerOf2<i64> for Rational {
    /// Raises 2 to an integer power.
    ///
    /// $f(k) = 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow.abs()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::power_of_2(0i64), 1);
    /// assert_eq!(Rational::power_of_2(3i64), 8);
    /// assert_eq!(Rational::power_of_2(100i64).to_string(), "1267650600228229401496703205376");
    /// assert_eq!(Rational::power_of_2(-3i64).to_string(), "1/8");
    /// assert_eq!(Rational::power_of_2(-100i64).to_string(), "1/1267650600228229401496703205376");
    /// ```
    fn power_of_2(pow: i64) -> Rational {
        let pow_abs = pow.unsigned_abs();
        if pow >= 0 {
            Rational::from(Natural::power_of_2(pow_abs))
        } else {
            Rational {
                sign: true,
                numerator: Natural::ONE,
                denominator: Natural::power_of_2(pow_abs),
            }
        }
    }
}
