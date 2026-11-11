// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_float::test_util::big_exponent_float::big_exponent_float_exponent_is_small;
use malachite_float::test_util::generators::big_exponent_float_gen;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

#[test]
fn gaussian_rational_from_big_exponent_float_properties() {
    big_exponent_float_gen().test_properties(|x| {
        if !big_exponent_float_exponent_is_small(&x) {
            return;
        }
        let result = GaussianRational::try_from(&x);
        assert_eq!(result.is_ok(), GaussianRational::convertible_from(&x));
        assert_eq!(result.is_ok(), Rational::convertible_from(&x));
        if let Ok(z) = result {
            assert_eq!(z, GaussianRational::from(Rational::try_from(&x).unwrap()));
        }
        assert_eq!(
            GaussianRational::try_from(x.clone()).is_ok(),
            GaussianRational::convertible_from(&x)
        );
    });
}
