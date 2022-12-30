use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Primorial;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::Primes;

impl Primorial for Natural {
    /// Computes the primorial of a number: the product of all primes less than or equal to
    /// it.
    ///
    /// The [`product_of_first_n_primes`](Primorial::product_of_first_n_primes) function is
    /// similar; it computes the primorial of the $n$th prime.
    ///
    /// If the input is too large, the function panics. For a function that returns `None`
    /// instead, try
    /// [`checked_primorial`](malachite_base::num::arithmetic::traits::CheckedPrimorial::checked_primorial).
    ///
    /// $$
    /// f(n) = n\\# = \prod_{p \leq n \atop p \\ \\text {prime}} p.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the output is too large to be represented.
    ///
    /// # Examples
    /// See [here](super::primorial#primorial).
    #[inline]
    fn primorial(n: u64) -> Natural {
        Natural::primes_less_than_or_equal_to(&Natural::from(n)).product()
    }

    /// Computes the product of the first $n$ primes.
    ///
    /// The [`primorial`](Primorial::primorial) function is similar; it computes the
    /// product of all primes less than or equal to $n$.
    ///
    /// If the input is too large, the function panics. For a function that returns `None`
    /// instead, try
    /// [`checked_product_of_first_n_primes`](malachite_base::num::arithmetic::traits::CheckedPrimorial::checked_product_of_first_n_primes).
    ///
    /// $$
    /// f(n) = p_n\\# = \prod_{k=1}^n p_n,
    /// $$
    /// where $p_n$ is the $n$th prime number.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if the output is too large to be represented.
    ///
    /// # Examples
    /// See [here](super::primorial#product_of_first_n_primes).
    #[inline]
    fn product_of_first_n_primes(n: u64) -> Natural {
        Natural::primes().take(usize::exact_from(n)).product()
    }
}
