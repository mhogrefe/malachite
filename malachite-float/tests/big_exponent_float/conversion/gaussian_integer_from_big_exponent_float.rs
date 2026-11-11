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
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;

#[test]
fn gaussian_integer_from_big_exponent_float_properties() {
    big_exponent_float_gen().test_properties(|x| {
        if !big_exponent_float_exponent_is_small(&x) {
            return;
        }
        let result = GaussianInteger::try_from(&x);
        assert_eq!(result.is_ok(), GaussianInteger::convertible_from(&x));
        assert_eq!(result.is_ok(), Integer::convertible_from(&x));
        if let Ok(z) = result {
            assert_eq!(z, GaussianInteger::from(Integer::try_from(&x).unwrap()));
        }
        assert_eq!(
            GaussianInteger::try_from(x.clone()).is_ok(),
            GaussianInteger::convertible_from(&x)
        );
    });
}
