// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::conversion::to_twos_complement_limbs::{
    limbs_twos_complement, limbs_twos_complement_in_place,
};
use crate::integer::Integer;
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;

impl Integer {
    /// Converts a slice of [limbs](crate#limbs) to an [`Integer`], in ascending order, so that less
    /// significant limbs have lower indices in the input slice.
    ///
    /// The limbs are in two's complement, and the most significant bit of the limbs indicates the
    /// sign; if the bit is zero, the [`Integer`] is non-negative, and if the bit is one it is
    /// negative. If the slice is empty, zero is returned.
    ///
    /// This function borrows a slice. If taking ownership of a [`Vec`] is possible instead,
    /// [`from_owned_twos_complement_limbs_asc`](`Self::from_owned_twos_complement_limbs_asc`) is
    /// more efficient.
    ///
    /// This function is more efficient than
    /// [`from_twos_complement_limbs_desc`](`Self::from_twos_complement_limbs_desc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Integer::from_twos_complement_limbs_asc(&[]), 0);
    ///     assert_eq!(Integer::from_twos_complement_limbs_asc(&[123]), 123);
    ///     assert_eq!(Integer::from_twos_complement_limbs_asc(&[4294967173]), -123);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from_twos_complement_limbs_asc(&[3567587328, 232]),
    ///         1000000000000u64
    ///     );
    ///     assert_eq!(
    ///         Integer::from_twos_complement_limbs_asc(&[727379968, 4294967063]),
    ///         -1000000000000i64
    ///     );
    /// }
    /// ```
    pub fn from_twos_complement_limbs_asc(xs: &[Limb]) -> Integer {
        match xs {
            &[] => Integer::ZERO,
            &[.., last] if !last.get_highest_bit() => Integer::from(Natural::from_limbs_asc(xs)),
            xs => -Natural::from_owned_limbs_asc(limbs_twos_complement(xs)),
        }
    }

    /// Converts a slice of [limbs](crate#limbs) to an [`Integer`], in descending order, so that
    /// less significant limbs have higher indices in the input slice.
    ///
    /// The limbs are in two's complement, and the most significant bit of the limbs indicates the
    /// sign; if the bit is zero, the [`Integer`] is non-negative, and if the bit is one it is
    /// negative. If the slice is empty, zero is returned.
    ///
    /// This function borrows a slice. If taking ownership of a [`Vec`] is possible instead,
    /// [`from_owned_twos_complement_limbs_desc`](`Self::from_owned_twos_complement_limbs_desc`) is
    /// more efficient.
    ///
    /// This function is less efficient than
    /// [`from_twos_complement_limbs_asc`](`Self::from_twos_complement_limbs_asc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Integer::from_twos_complement_limbs_desc(&[]), 0);
    ///     assert_eq!(Integer::from_twos_complement_limbs_desc(&[123]), 123);
    ///     assert_eq!(
    ///         Integer::from_twos_complement_limbs_desc(&[4294967173]),
    ///         -123
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from_twos_complement_limbs_desc(&[232, 3567587328]),
    ///         1000000000000u64
    ///     );
    ///     assert_eq!(
    ///         Integer::from_twos_complement_limbs_desc(&[4294967063, 727379968]),
    ///         -1000000000000i64
    ///     );
    /// }
    /// ```
    pub fn from_twos_complement_limbs_desc(xs: &[Limb]) -> Integer {
        Integer::from_owned_twos_complement_limbs_asc(xs.iter().copied().rev().collect())
    }

    /// Converts a slice of [limbs](crate#limbs) to an [`Integer`], in ascending order, so that less
    /// significant limbs have lower indices in the input slice.
    ///
    /// The limbs are in two's complement, and the most significant bit of the limbs indicates the
    /// sign; if the bit is zero, the [`Integer`] is non-negative, and if the bit is one it is
    /// negative. If the slice is empty, zero is returned.
    ///
    /// This function takes ownership of a [`Vec`]. If it's necessary to borrow a slice instead, use
    /// [`from_twos_complement_limbs_asc`](`Self::from_twos_complement_limbs_asc`)
    ///
    /// This function is more efficient than
    /// [`from_owned_twos_complement_limbs_desc`](`Self::from_owned_twos_complement_limbs_desc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Integer::from_owned_twos_complement_limbs_asc(vec![]), 0);
    ///     assert_eq!(
    ///         Integer::from_owned_twos_complement_limbs_asc(vec![123]),
    ///         123
    ///     );
    ///     assert_eq!(
    ///         Integer::from_owned_twos_complement_limbs_asc(vec![4294967173]),
    ///         -123
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from_owned_twos_complement_limbs_asc(vec![3567587328, 232]),
    ///         1000000000000i64
    ///     );
    ///     assert_eq!(
    ///         Integer::from_owned_twos_complement_limbs_asc(vec![727379968, 4294967063]),
    ///         -1000000000000i64
    ///     );
    /// }
    /// ```
    pub fn from_owned_twos_complement_limbs_asc(mut xs: Vec<Limb>) -> Integer {
        match *xs.as_slice() {
            [] => Integer::ZERO,
            [.., last] if !last.get_highest_bit() => {
                Integer::from(Natural::from_owned_limbs_asc(xs))
            }
            _ => {
                assert!(!limbs_twos_complement_in_place(&mut xs));
                -Natural::from_owned_limbs_asc(xs)
            }
        }
    }

    /// Converts a slice of [limbs](crate#limbs) to an [`Integer`], in descending order, so that
    /// less significant limbs have higher indices in the input slice.
    ///
    /// The limbs are in two's complement, and the most significant bit of the limbs indicates the
    /// sign; if the bit is zero, the [`Integer`] is non-negative, and if the bit is one it is
    /// negative. If the slice is empty, zero is returned.
    ///
    /// This function takes ownership of a [`Vec`]. If it's necessary to borrow a slice instead, use
    /// [`from_twos_complement_limbs_desc`](`Self::from_twos_complement_limbs_desc`).
    ///
    /// This function is less efficient than
    /// [`from_owned_twos_complement_limbs_asc`](`Self::from_owned_twos_complement_limbs_asc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Integer::from_owned_twos_complement_limbs_desc(vec![]), 0);
    ///     assert_eq!(
    ///         Integer::from_owned_twos_complement_limbs_desc(vec![123]),
    ///         123
    ///     );
    ///     assert_eq!(
    ///         Integer::from_owned_twos_complement_limbs_desc(vec![4294967173]),
    ///         -123
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from_owned_twos_complement_limbs_desc(vec![232, 3567587328]),
    ///         1000000000000i64
    ///     );
    ///     assert_eq!(
    ///         Integer::from_owned_twos_complement_limbs_desc(vec![4294967063, 727379968]),
    ///         -1000000000000i64
    ///     );
    /// }
    /// ```
    pub fn from_owned_twos_complement_limbs_desc(mut xs: Vec<Limb>) -> Integer {
        xs.reverse();
        Integer::from_owned_twos_complement_limbs_asc(xs)
    }
}
