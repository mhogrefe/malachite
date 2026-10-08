// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Gcd;

// The GCD of `x` and the absolute values of the elements of `xs`, taken one element at a time with
// none of the shortcuts.
pub fn integers_content_chained_naive(xs: &[Integer], x: &Natural) -> Natural {
    xs.iter()
        .fold(x.clone(), |g, c| g.gcd(c.unsigned_abs_ref()))
}
