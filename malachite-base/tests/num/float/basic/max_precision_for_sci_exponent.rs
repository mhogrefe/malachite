// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::signed_gen_var_11;
use std::panic::catch_unwind;

#[test]
fn max_precision_for_sci_exponent() {
    fn test<T: PrimitiveFloat>(n: i64, out: u64) {
        assert_eq!(T::max_precision_for_sci_exponent(n), out);
    }
    test::<f32>(0, 24);
    test::<f32>(127, 24);
    test::<f32>(-149, 1);
    test::<f32>(-148, 2);
    test::<f32>(-147, 3);

    test::<f64>(0, 53);
    test::<f64>(1023, 53);
    test::<f64>(-1074, 1);
    test::<f64>(-1073, 2);
    test::<f64>(-1072, 3);
}

fn max_precision_for_sci_exponent_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::max_precision_for_sci_exponent(10000));
    assert_panic!(T::max_precision_for_sci_exponent(-10000));
}

#[test]
pub fn max_precision_for_sci_exponent_fail() {
    apply_fn_to_primitive_floats!(max_precision_for_sci_exponent_fail_helper);
}

fn max_precision_for_sci_exponent_properties_helper<T: PrimitiveFloat>() {
    signed_gen_var_11::<T>().test_properties(|exp| {
        let p = T::max_precision_for_sci_exponent(exp);
        assert_ne!(p, 0);
        assert!(p <= u64::exact_from(T::MAX_EXPONENT));
    });
}

#[test]
fn max_precision_for_sci_exponent_properties() {
    apply_fn_to_primitive_floats!(max_precision_for_sci_exponent_properties_helper);
}
