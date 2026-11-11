// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::{BigExponentFloat, Float};

#[test]
fn test_non_dyadic_from_bits() {
    for prec in [1, 2, 10, 100] {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let bits = || [false, true, true].into_iter().cycle();
            let (x, o) = BigExponentFloat::non_dyadic_from_bits_prec_round(bits(), prec, rm);
            let (f, o_f) = Float::non_dyadic_from_bits_prec_round(bits(), prec, rm);
            assert!(x.is_valid());
            assert_eq!(
                format!("{x:#x}"),
                format!("{:#x}", BigExponentFloat::from(f))
            );
            assert_eq!(o, o_f);
        }
        let (x, o) =
            BigExponentFloat::non_dyadic_from_bits_prec([true, false].into_iter().cycle(), prec);
        let (f, o_f) = Float::non_dyadic_from_bits_prec([true, false].into_iter().cycle(), prec);
        assert_eq!(
            format!("{x:#x}"),
            format!("{:#x}", BigExponentFloat::from(f))
        );
        assert_eq!(o, o_f);
    }
}
