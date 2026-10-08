// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart) and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which compute the
/// primitive part of a vector with its first nonzero element made positive.
pub mod canonical_primitive_part;
/// Implementations of
/// [`CanonicalizeSign`](malachite_base::num::arithmetic::traits::CanonicalizeSign) and
/// [`CanonicalizeSignAssign`](malachite_base::num::arithmetic::traits::CanonicalizeSignAssign),
/// which negate a vector if its pivot is negative.
pub mod canonicalize_sign;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which split a vector into a rational scalar and a vector of coprime integers.
pub mod content;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a vector.
pub mod neg;
