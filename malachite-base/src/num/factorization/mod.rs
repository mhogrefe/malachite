// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`Factor`](traits::Factor), a trait for computing the prime factorization of a number.
pub mod factor;
/// [`IsPower`](traits::IsPower) and [`ExpressAsPower`](traits::ExpressAsPower), traits for testing
/// if a number is a perfect power and, if it is, expressing it as such.
///
/// # is_power
/// ```
/// use malachite_base::num::factorization::traits::IsPower;
///
/// assert!(0u8.is_power());
/// assert!(1u16.is_power());
/// assert!(36u32.is_power());
/// assert!(64u32.is_power());
/// assert!(100u64.is_power());
/// assert!(1728u64.is_power());
///
/// assert!(0u8.is_power());
/// assert!(1u16.is_power());
/// assert!(!2u64.is_power());
/// assert!(!3u64.is_power());
/// ```
///
/// # express_as_power
/// ```
/// use malachite_base::num::factorization::traits::ExpressAsPower;
///
/// assert_eq!(0u8.express_as_power().unwrap(), (0, 2));
/// assert_eq!(1u16.express_as_power().unwrap(), (1, 2));
/// assert_eq!(36u32.express_as_power().unwrap(), (6, 2));
/// assert_eq!(64u32.express_as_power().unwrap(), (2, 6));
/// assert_eq!(100u64.express_as_power().unwrap(), (10, 2));
/// assert_eq!(1728u64.express_as_power().unwrap(), (12, 3));
///
/// assert!(0u8.express_as_power().is_some());
/// assert!(1u16.express_as_power().is_some());
/// assert!(2u64.express_as_power().is_none());
/// assert!(3u64.express_as_power().is_none());
/// ```
pub mod is_power;
/// [`IsPrime`](traits::IsPrime), a trait for testing a number for primality.
pub mod is_prime;
/// [`IsSquare`](traits::IsSquare), a trait for testing if a number if a perfect square.
///
/// # is_square
/// ```
/// use malachite_base::num::factorization::traits::IsSquare;
///
/// assert!(0u8.is_square());
/// assert!(1u16.is_square());
/// assert!(4u32.is_square());
/// assert!(256u64.is_square());
///
/// assert!(!2u8.is_square());
/// assert!(!5u16.is_square());
/// assert!(!8u32.is_square());
/// assert!(!128u64.is_square());
/// ```
pub mod is_square;
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
/// assert_eq!(
///     u8::primes_less_than_or_equal_to(&10).collect_vec(),
///     &[2, 3, 5, 7]
/// );
/// assert_eq!(
///     u16::primes_less_than_or_equal_to(&11).collect_vec(),
///     &[2, 3, 5, 7, 11]
/// );
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
/// [`PrimitiveRootPrime`](traits::PrimitiveRootPrime), a trait for finding a primitive root modulo
/// a prime number.
///
/// # primitive_root_prime
/// ```
/// use malachite_base::num::factorization::traits::PrimitiveRootPrime;
///
/// assert_eq!(5u32.primitive_root_prime(), 2);
/// assert_eq!(191u32.primitive_root_prime(), 19);
/// assert_eq!(4294967291u32.primitive_root_prime(), 2);
/// ```
pub mod primitive_root_prime;
/// Various traits for generating primes, primality testing, and factorization.
pub mod traits;
