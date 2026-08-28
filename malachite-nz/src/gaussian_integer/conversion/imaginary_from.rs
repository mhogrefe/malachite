// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ImaginaryFrom;

// Anything that converts to an `Integer` converts to a purely imaginary `GaussianInteger`,
// including `Integer` itself.
impl<T> ImaginaryFrom<T> for GaussianInteger
where
    Integer: From<T>,
{
    fn imaginary_from(x: T) -> Self {
        Self {
            real: Integer::ZERO,
            imaginary: Integer::from(x),
        }
    }
}
