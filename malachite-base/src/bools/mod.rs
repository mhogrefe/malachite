// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Constants associated with [`bool`]s.
///
/// The constants [`MIN`](crate::comparison::traits::Min::MIN) and
/// [`MAX`](crate::comparison::traits::Max::MAX) are defined as for [`bool`]s as `false` and `true`,
/// respectively. The constant [`NAME`](crate::named::Named::NAME) is defined as "bool".
pub mod constants;
/// An iterator that generates [`bool`]s without repetition.
pub mod exhaustive;
/// The implementation of [`NotAssign`](crate::num::logic::traits::NotAssign) for [`bool`].
pub mod not_assign;
#[cfg(feature = "random")]
/// Iterators that generate [`bool`]s randomly.
pub mod random;
