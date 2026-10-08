// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for converting a [`Vec`](alloc::vec::Vec) or slice to a [`Vector`](super::Vector).
pub mod from_elements;
/// Functions for converting a [`Vector`](super::Vector) to and from a
/// [`String`](alloc::string::String).
pub mod string;
/// Functions for converting a [`Vector`](super::Vector) to a [`Vec`](alloc::vec::Vec) or slice.
pub mod to_elements;
