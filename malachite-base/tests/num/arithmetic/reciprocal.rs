// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_gen;

#[test]
fn test_reciprocal() {
    fn test<T: PrimitiveFloat>(x: T, out: T) {
        assert_eq!(NiceFloat(x.reciprocal()), NiceFloat(out));

        let mut x = x;
        x.reciprocal_assign();
        assert_eq!(NiceFloat(x), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, -0.0);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(1.0, 1.0);
    test::<f32>(-1.0, -1.0);
    test::<f32>(0.5, 2.0);
    test::<f32>(-0.5, -2.0);
    test::<f32>(core::f32::consts::SQRT_2, 0.70710677);
    test::<f32>(-core::f32::consts::SQRT_2, -0.70710677);
    test::<f32>(core::f32::consts::PI, std::f32::consts::FRAC_1_PI);
    test::<f32>(-core::f32::consts::PI, -std::f32::consts::FRAC_1_PI);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, -0.0);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(1.0, 1.0);
    test::<f64>(-1.0, -1.0);
    test::<f64>(0.5, 2.0);
    test::<f64>(-0.5, -2.0);
    test::<f64>(core::f64::consts::SQRT_2, 0.7071067811865475);
    test::<f64>(-core::f64::consts::SQRT_2, -0.7071067811865475);
    test::<f64>(core::f64::consts::PI, std::f64::consts::FRAC_1_PI);
    test::<f64>(-core::f64::consts::PI, -std::f64::consts::FRAC_1_PI);
}

fn reciprocal_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let mut reciprocal = x;
        reciprocal.reciprocal_assign();
        assert_eq!(NiceFloat(reciprocal), NiceFloat(x.reciprocal()));
        assert_eq!(NiceFloat(reciprocal), NiceFloat(x.pow(-1)));
        assert_eq!(NiceFloat((-x).reciprocal()), NiceFloat(-reciprocal));
    });
}

#[test]
fn reciprocal_properties() {
    apply_fn_to_primitive_floats!(reciprocal_properties_helper);
}
