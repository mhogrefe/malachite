// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::test_util::generators::integer_unsigned_rounding_mode_triple_gen_var_3;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_from_integer() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();
        let x = BigExponentFloat::from(n.clone());
        assert!(x.is_valid());
        assert_eq!(format!("{x:#x}"), out);
        assert_eq!(format!("{:#x}", BigExponentFloat::from(&n)), out);
    };
    test("0", "0x0.0");
    test("1", "0x1.0");
    test("123", "0x7b.0");
    test("-123", "-0x7b.0");
    test("-1000000000000000000000000", "-0xd.3c21bcecceda1E+19");
    test("1000000000000000000000000", "0xd.3c21bcecceda1E+19");
}

#[test]
fn from_integer_properties() {
    integer_gen().test_properties(|n| {
        let x = BigExponentFloat::from(&n);
        assert!(x.is_valid());
        assert_eq!(Integer::try_from(&x).unwrap(), n);
        assert_eq!(
            format!("{x:#x}"),
            format!("{:#x}", BigExponentFloat::from(Float::exact_from(&n)))
        );
    });

    integer_unsigned_rounding_mode_triple_gen_var_3().test_properties(|(n, prec, rm)| {
        let (x, o) = BigExponentFloat::from_integer_prec_round_ref(&n, prec, rm);
        assert!(x.is_valid());
        let (x_alt, o_alt) = BigExponentFloat::from_integer_prec_round(n.clone(), prec, rm);
        assert_eq!(format!("{x_alt:#x}"), format!("{x:#x}"));
        assert_eq!(o_alt, o);
        // - agreement with `Float`
        let (f, o_f) = Float::from_integer_prec_round_ref(&n, prec, rm);
        assert_eq!(
            format!("{:#x}", BigExponentFloat::from(&f)),
            format!("{x:#x}")
        );
        assert_eq!(o_f, o);
        if rm == Nearest {
            let (x_alt, o_alt) = BigExponentFloat::from_integer_prec_ref(&n, prec);
            assert_eq!(format!("{x_alt:#x}"), format!("{x:#x}"));
            assert_eq!(o_alt, o);
            let (x_alt, o_alt) = BigExponentFloat::from_integer_prec(n, prec);
            assert_eq!(format!("{x_alt:#x}"), format!("{x:#x}"));
            assert_eq!(o_alt, o);
        }
    });
}
