// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::test_util::generators::primitive_float_unsigned_rounding_mode_triple_gen_var_3;
use malachite_float::{BigExponentFloat, Float};

// A non-generic conversion, which a generic helper's bounds cannot shadow.
fn bef(f: &Float) -> BigExponentFloat {
    BigExponentFloat::from(f)
}

#[test]
fn test_from_primitive_float() {
    let test = |x: f64, out| {
        let y = BigExponentFloat::from(x);
        assert!(y.is_valid());
        assert_eq!(format!("{y:#x}"), out);
    };
    test(f64::NAN, "NaN");
    test(f64::INFINITY, "Infinity");
    test(f64::NEGATIVE_INFINITY, "-Infinity");
    test(0.0, "0x0.0");
    test(-0.0, "0x0.0");
    test(1.5, "0x1.8");
    test(-0.1, "-0x0.1999999999999a");
    test(f64::MIN_POSITIVE_SUBNORMAL, "0x4.0E-269");
    test(f64::MAX, "0xf.ffffffffffff8E+255");
}

fn from_primitive_float_properties_helper<T>()
where
    T: PrimitiveFloat + for<'a> RoundingFrom<&'a BigExponentFloat>,
    Float: From<T>,
    BigExponentFloat: From<T>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let y = BigExponentFloat::from(x);
        assert!(y.is_valid());
        assert_eq!(format!("{y:#x}"), format!("{:#x}", bef(&Float::from(x))));
        // - the conversion is exact, apart from the sign of zero
        let (z, o) = T::rounding_from(&y, Exact);
        assert_eq!(o, core::cmp::Ordering::Equal);
        if x == T::ZERO {
            assert_eq!(z, T::ZERO);
        } else {
            assert_eq!(NiceFloat(z), NiceFloat(x));
        }
    });

    primitive_float_unsigned_rounding_mode_triple_gen_var_3::<T>().test_properties(
        |(x, prec, rm)| {
            let (y, o) = BigExponentFloat::from_primitive_float_prec_round(x, prec, rm);
            assert!(y.is_valid());
            let (f, o_f) = Float::from_primitive_float_prec_round(x, prec, rm);
            assert_eq!(format!("{:#x}", bef(&f)), format!("{y:#x}"));
            assert_eq!(o_f, o);
            if rm == Nearest {
                let (y_alt, o_alt) = BigExponentFloat::from_primitive_float_prec(x, prec);
                assert_eq!(format!("{y_alt:#x}"), format!("{y:#x}"));
                assert_eq!(o_alt, o);
            }
        },
    );
}

#[test]
fn from_primitive_float_properties() {
    apply_fn_to_primitive_floats!(from_primitive_float_properties_helper);
}
