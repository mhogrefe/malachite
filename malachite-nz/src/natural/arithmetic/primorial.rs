use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Primorial;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::Primes;

impl Primorial for Natural {
    /// Computes the primorial of a [`Natural`]: the product of all primes less than or equal to
    /// it.
    ///
    /// The [`product_of_first_n_primes`](Natural::product_of_first_n_primes) function is similar;
    /// it computes the primorial of the $n$th prime.
    ///
    /// $$
    /// f(n) = n\\# = \prod_{p \leq n \atop p \\ \\text {prime}} p.
    /// $$
    /// 
    /// $n\\# = O(e^{(1+o(1))n})$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Primorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::primorial(0), 1);
    /// assert_eq!(Natural::primorial(1), 1);
    /// assert_eq!(Natural::primorial(2), 2);
    /// assert_eq!(Natural::primorial(3), 6);
    /// assert_eq!(Natural::primorial(4), 6);
    /// assert_eq!(Natural::primorial(5), 30);
    /// assert_eq!(Natural::primorial(100).to_string(), "2305567963945518424753102147331756070");
    /// ```
    ///
    /// This is equivalent to `mpz_primorial_ui` from `mpz/primorial_ui.c`, GMP 6.2.1.
    #[inline]
    fn primorial(n: u64) -> Natural {
        Natural::primes_less_than_or_equal_to(&Natural::from(n)).product()
    }

    /// Computes the product of the first $n$ primes.
    ///
    /// The [`primorial`](Natural::primorial) function is similar; it computes the product of all
    /// primes less than or equal to $n$.
    ///
    /// $$
    /// f(n) = p_n\\# = \prod_{k=1}^n p_n,
    /// $$
    /// where $p_n$ is the $n$th prime number.
    /// 
    /// $p_n\\# = O\left ( \left ( \frac{1}{e}k\log k\left ( \frac{\log k}{e^2}k
    /// \right )^{1/\log k} \right )^k \omega(1)\right )$.
    /// 
    /// This asymptotic approximation is due to
    /// [Bart Michels](https://math.stackexchange.com/a/1594930).
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Primorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::product_of_first_n_primes(0), 1);
    /// assert_eq!(Natural::product_of_first_n_primes(1), 2);
    /// assert_eq!(Natural::product_of_first_n_primes(2), 6);
    /// assert_eq!(Natural::product_of_first_n_primes(3), 30);
    /// assert_eq!(Natural::product_of_first_n_primes(4), 210);
    /// assert_eq!(Natural::product_of_first_n_primes(5), 2310);
    /// assert_eq!(
    ///     Natural::product_of_first_n_primes(100).to_string(),
    ///     "4711930799906184953162487834760260422020574773409675520188634839616415335845034221205\
    ///     28925670554468197243910409777715799180438028421831503871944494399049257903072063599053\
    ///     8452312528339864352999310398481791730017201031090"
    /// );
    /// ```
    #[inline]
    fn product_of_first_n_primes(n: u64) -> Natural {
        Natural::primes().take(usize::exact_from(n)).product()
    }
}
