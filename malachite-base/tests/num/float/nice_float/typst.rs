// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::float::NiceFloat;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::primitive_float_gen;

#[test]
pub fn test_nice_float_to_typst() {
    fn test<T: PrimitiveFloat + ToTypst>(x: T, out: &str) {
        assert_eq!(NiceFloat(x).to_typst_string(), out);
    }
    test(f64::NAN, r#""NaN""#);
    test(f64::INFINITY, "infinity");
    test(f64::NEGATIVE_INFINITY, "-infinity");
    test(0.0f64, "0.0");
    test(-0.0f64, "-0.0");
    test(1.0f64, "1.0");
    test(0.00123f64, "0.00123");
    test(1.0e16f64, "1.0 times 10^(16)");
    test(f32::MIN_POSITIVE_SUBNORMAL, "1.0 times 10^(-45)");
}

#[test]
fn nice_float_to_typst_properties() {
    primitive_float_gen::<f64>().test_properties(|x| {
        // The wrapper adds nothing of its own.
        assert_eq!(NiceFloat(x).to_typst_string(), x.to_typst_string());
    });
    primitive_float_gen::<f32>().test_properties(|x| {
        assert_eq!(NiceFloat(x).to_typst_string(), x.to_typst_string());
    });
}
