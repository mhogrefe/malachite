// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Data precomputed for dividing polynomials modulo $m$ or $2^k$ by a fixed divisor: the inverse,
/// modulo $m$ or $2^k$, of the divisor's reversal as a power series, truncated to the divisor's
/// length. With it, the quotient of a dividend no more than twice as long as the divisor takes one
/// truncated multiplication rather than a series inversion.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[doc(hidden)]
pub struct ModPolynomialData<C> {
    pub inverse: alloc::vec::Vec<C>,
}

/// Data precomputed for evaluating polynomials modulo $m$ or $2^k$ at a fixed set of points: the
/// subproduct tree of the linear factors $x - x_j$, as FLINT lays it out. Level $i$ holds the
/// products of $2^i$ consecutive factors, monic and of degree $2^i$, each stored in $2^i + 1$
/// coefficients one after another (the last product of a level may be shorter). The top level, the
/// product of all the factors, is not stored, since evaluation never divides by it. Alongside it
/// are the inverses of the reversals of the nodes, laid out the same way, so that each remainder
/// down the tree is a truncated multiplication.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[doc(hidden)]
pub struct ModEvaluateManyData<C, M> {
    pub tree: alloc::vec::Vec<alloc::vec::Vec<C>>,
    pub inverses: alloc::vec::Vec<alloc::vec::Vec<C>>,
    pub modulus: M,
}

impl<C, M> ModEvaluateManyData<C, M> {
    /// The number of points: level 0 of the tree holds a factor $x - x_j$ in two coefficients for
    /// each point, plus one more coefficient.
    #[inline]
    pub fn len(&self) -> usize {
        self.tree.first().map_or(0, |level| level.len() >> 1)
    }

    /// Whether there are no points.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}

/// The traits for operations on polynomials whose coefficients are reduced modulo $m$ or $2^k$.
pub mod traits;
