// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// Returns the length of `xs`, excluding trailing zeros.
fn limbs_significant_length(xs: &[Limb]) -> usize {
    xs.iter()
        .enumerate()
        .rev()
        .find(|&(_, &x)| x != 0)
        .map_or(0, |(i, _)| i + 1)
}

impl Natural {
    /// Converts a slice of [limbs](crate#limbs) to a [`Natural`].
    ///
    /// The limbs are in ascending order, so that less-significant limbs have lower indices in the
    /// input slice.
    ///
    /// This function borrows the limbs. If taking ownership of limbs is possible,
    /// [`from_owned_limbs_asc`](Self::from_owned_limbs_asc) is more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// This function is more efficient than [`from_limbs_desc`](Self::from_limbs_desc).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::from_limbs_asc(&[]), 0);
    ///     assert_eq!(Natural::from_limbs_asc(&[123]), 123);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from_limbs_asc(&[3567587328, 232]),
    ///         1000000000000u64
    ///     );
    /// }
    /// ```
    pub fn from_limbs_asc(xs: &[Limb]) -> Natural {
        let significant_length = limbs_significant_length(xs);
        match significant_length {
            0 => Natural::ZERO,
            1 => Natural(Small(xs[0])),
            _ => Natural(Large(xs[..significant_length].to_vec())),
        }
    }

    /// Converts a slice of [limbs](crate#limbs) to a [`Natural`].
    ///
    /// The limbs in descending order, so that less-significant limbs have higher indices in the
    /// input slice.
    ///
    /// This function borrows the limbs. If taking ownership of the limbs is possible,
    /// [`from_owned_limbs_desc`](Self::from_owned_limbs_desc) is more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// This function is less efficient than [`from_limbs_asc`](Self::from_limbs_asc).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::from_limbs_desc(&[]), 0);
    ///     assert_eq!(Natural::from_limbs_desc(&[123]), 123);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from_limbs_desc(&[232, 3567587328]),
    ///         1000000000000u64
    ///     );
    /// }
    /// ```
    pub fn from_limbs_desc(xs: &[Limb]) -> Natural {
        Natural::from_owned_limbs_asc(xs.iter().copied().rev().collect())
    }

    /// Converts a [`Vec`] of [limbs](crate#limbs) to a [`Natural`].
    ///
    /// The limbs are in ascending order, so that less-significant limbs have lower indices in the
    /// input [`Vec`].
    ///
    /// This function takes ownership of the limbs. If it's necessary to borrow the limbs instead,
    /// use [`from_limbs_asc`](Self::from_limbs_asc).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// This function is more efficient than [`from_limbs_desc`](Self::from_limbs_desc).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::from_owned_limbs_asc(vec![]), 0);
    ///     assert_eq!(Natural::from_owned_limbs_asc(vec![123]), 123);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from_owned_limbs_asc(vec![3567587328, 232]),
    ///         1000000000000u64
    ///     );
    /// }
    /// ```
    pub fn from_owned_limbs_asc(mut xs: Vec<Limb>) -> Natural {
        let significant_length = limbs_significant_length(&xs);
        match significant_length {
            0 => Natural::ZERO,
            1 => Natural(Small(xs[0])),
            _ => {
                xs.truncate(significant_length);
                Natural(Large(xs))
            }
        }
    }

    /// Converts a [`Vec`] of [limbs](crate#limbs) to a [`Natural`].
    ///
    /// The limbs are in descending order, so that less-significant limbs have higher indices in the
    /// input [`Vec`].
    ///
    /// This function takes ownership of the limbs. If it's necessary to borrow the limbs instead,
    /// use [`from_limbs_desc`](Self::from_limbs_desc).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// This function is less efficient than [`from_limbs_asc`](Self::from_limbs_asc).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::from_owned_limbs_desc(vec![]), 0);
    ///     assert_eq!(Natural::from_owned_limbs_desc(vec![123]), 123);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from_owned_limbs_desc(vec![232, 3567587328]),
    ///         1000000000000u64
    ///     );
    /// }
    /// ```
    pub fn from_owned_limbs_desc(mut xs: Vec<Limb>) -> Natural {
        xs.reverse();
        Natural::from_owned_limbs_asc(xs)
    }
}
