/// An efficient prime sieve.
pub mod prime_sieve;
/// [`Primes`](traits::Primes), a trait for generating prime numbers.
///
/// # primes_less_than
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::factorization::traits::Primes;
///
/// assert_eq!(u8::primes_less_than(&10).collect_vec(), &[2, 3, 5, 7]);
/// assert_eq!(u16::primes_less_than(&11).collect_vec(), &[2, 3, 5, 7]);
/// assert_eq!(
///     u32::primes_less_than(&100).collect_vec(),
///     &[
///         2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
///         89, 97
///     ]
/// );
/// ```
///
/// # primes_less_than_or_equal_to
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::factorization::traits::Primes;
///
/// assert_eq!(u8::primes_less_than_or_equal_to(&10).collect_vec(), &[2, 3, 5, 7]);
/// assert_eq!(u16::primes_less_than_or_equal_to(&11).collect_vec(), &[2, 3, 5, 7, 11]);
/// assert_eq!(
///     u32::primes_less_than_or_equal_to(&100).collect_vec(),
///     &[
///         2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
///         89, 97
///     ]
/// );
/// ```
///
/// # primes
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::factorization::traits::Primes;
///
/// assert_eq!(
///     u8::primes().collect_vec(),
///     &[
///         2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
///         89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
///         181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251
///     ]
/// );
/// ```
pub mod primes;
/// Various traits for generating primes, primality testing, and factorization.
pub mod traits;
