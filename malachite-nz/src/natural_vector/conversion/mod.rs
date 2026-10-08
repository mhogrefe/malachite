// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for converting a [`Vec`](alloc::vec::Vec) or slice of
/// [`Natural`](crate::natural::Natural)s to a [`NaturalVector`](super::NaturalVector).
pub mod from_elements;
/// Functions for converting a [`NaturalVector`](super::NaturalVector) to and from a [`String`].
pub mod string;
/// Functions for converting a [`NaturalVector`](super::NaturalVector) to a [`Vec`](alloc::vec::Vec)
/// or slice of [`Natural`](crate::natural::Natural)s.
pub mod to_elements;
