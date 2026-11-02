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

/// The traits for operations on polynomials whose coefficients are reduced modulo $m$ or $2^k$.
pub mod traits;
