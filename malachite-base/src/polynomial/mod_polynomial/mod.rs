// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::polynomial::modular::ModPolynomialData;

/// Data precomputed for composing polynomials with a fixed inner polynomial $g$ modulo a fixed
/// polynomial $h$, with coefficients modulo $m$ or $2^k$: the powers $g^0, g^1, \ldots, g^{s-1}$
/// modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor + 1$, each padded to $\deg h$ coefficients,
/// and the data for dividing by $h$.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[doc(hidden)]
pub struct ModComposeModPolynomialData<C> {
    pub powers: alloc::vec::Vec<alloc::vec::Vec<C>>,
    pub division: ModPolynomialData<C>,
}

/// The traits for operations on polynomials modulo a polynomial.
pub mod traits;
