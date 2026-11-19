// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

// The first `len` tangent numbers, from the boustrophedon (Seidel-Entringer) triangle: with
// $E_{0,0} = 1$, $E_{n,0} = 0$ for $n > 0$, and $E_{n,k} = E_{n,k-1} + E_{n-1,n-k}$, the tangent
// number $T_j$ is $E_{2j-1,2j-1}$.
pub fn tangent_numbers_prefix_naive(len: u64) -> Vec<Natural> {
    let len = usize::exact_from(len);
    let mut ts = Vec::with_capacity(len);
    let mut row = vec![Natural::ONE];
    for n in 1..len << 1 {
        let mut next = Vec::with_capacity(n + 1);
        next.push(Natural::ZERO);
        for k in 1..=n {
            let x = &next[k - 1] + &row[n - k];
            next.push(x);
        }
        row = next;
        if n.odd() {
            ts.push(row[n].clone());
        }
    }
    ts
}
