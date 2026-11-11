// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::mag::{MAG_BITS, Mag};
use core::str::FromStr;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;

impl FromStr for Mag {
    type Err = ();

    // Parses the format written by `Display`: `m` or `m*2^k`, for a decimal `m` and a signed
    // decimal `k`. The value must be exactly representable: `m` with its trailing zeros removed may
    // have at most `MAG_BITS` bits.
    fn from_str(s: &str) -> Result<Self, ()> {
        let (m, k) = match s.split_once("*2^") {
            Some((m, k)) => (m, k.parse::<i64>().map_err(|_| ())?),
            None => (s, 0),
        };
        if m.starts_with('+') {
            return Err(());
        }
        let m = m.parse::<u64>().map_err(|_| ())?;
        if m == 0 {
            return if k == 0 { Ok(Self::ZERO) } else { Err(()) };
        }
        let zeros = m.trailing_zeros();
        let odd = m >> zeros;
        let bits = odd.significant_bits();
        if bits > MAG_BITS {
            return Err(());
        }
        // m * 2^k = (odd << (MAG_BITS - bits)) * 2^(k + zeros + bits - MAG_BITS)
        let exp = k
            .checked_add(i64::wrapping_from(u64::from(zeros) + bits))
            .ok_or(())?;
        Ok(Self {
            man: odd << (MAG_BITS - bits),
            exp,
        })
    }
}
