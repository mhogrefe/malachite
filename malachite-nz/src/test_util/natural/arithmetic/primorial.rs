use crate::natural::Natural;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::Primes;

pub fn primorial_naive(n: u64) -> Natural {
    Natural::primes_less_than_or_equal_to(&Natural::from(n)).product()
}

pub fn product_of_first_n_primes_naive(n: u64) -> Natural {
    Natural::primes().take(usize::exact_from(n)).product()
}
