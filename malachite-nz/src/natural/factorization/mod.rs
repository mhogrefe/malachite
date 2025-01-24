// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`Primes`](malachite_base::num::factorization::traits::Primes), a trait for
/// generating prime numbers.
///
/// # primes_less_than
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::factorization::traits::Primes;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::primes_less_than(&Natural::from(10u32))
///         .collect_vec()
///         .to_debug_string(),
///     "[2, 3, 5, 7]"
/// );
/// assert_eq!(
///     Natural::primes_less_than(&Natural::from(11u32))
///         .collect_vec()
///         .to_debug_string(),
///     "[2, 3, 5, 7]"
/// );
/// assert_eq!(
///     Natural::primes_less_than(&Natural::from(100u32))
///         .collect_vec()
///         .to_debug_string(),
///     "[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, \
///     89, 97]"
/// );
/// ```
///
/// # primes_less_than_or_equal_to
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::factorization::traits::Primes;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::primes_less_than_or_equal_to(&Natural::from(10u32))
///         .collect_vec()
///         .to_debug_string(),
///     "[2, 3, 5, 7]"
/// );
/// assert_eq!(
///     Natural::primes_less_than_or_equal_to(&Natural::from(11u32))
///         .collect_vec()
///         .to_debug_string(),
///     "[2, 3, 5, 7, 11]"
/// );
/// assert_eq!(
///     Natural::primes_less_than_or_equal_to(&Natural::from(100u32))
///         .collect_vec()
///         .to_debug_string(),
///     "[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, \
///     89, 97]"
/// );
/// ```
///
/// # primes
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_base::num::factorization::traits::Primes;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::primes()
///         .take_while(|p| u8::convertible_from(p))
///         .collect_vec()
///         .to_debug_string(),
///     "[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, \
///     89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, \
///     181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251]"
/// );
/// ```
pub mod primes;
