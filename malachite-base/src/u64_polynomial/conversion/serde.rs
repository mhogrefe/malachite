// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::u64_polynomial::{SerdeU64Polynomial, U64Polynomial};
use alloc::string::{String, ToString};

impl From<U64Polynomial> for SerdeU64Polynomial {
    #[inline]
    fn from(p: U64Polynomial) -> Self {
        Self(p.coefficients)
    }
}

impl TryFrom<SerdeU64Polynomial> for U64Polynomial {
    type Error = String;

    // The condition is the one `U64Polynomial::is_valid` checks. It is spelled out here rather than
    // deferring to it because that function is only built for testing.
    #[inline]
    fn try_from(p: SerdeU64Polynomial) -> Result<Self, String> {
        if p.0.last() == Some(&0) {
            return Err("Last coefficient is zero".to_string());
        }
        Ok(Self { coefficients: p.0 })
    }
}
