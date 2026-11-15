// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsGaussianInteger;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_to_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1,
};
use malachite_nz::integer::Integer;

#[test]
fn test_is_gaussian_integer() {
    let test = |x: &str, x_hex: &str, out| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(x.is_gaussian_integer(), out);
    };
    test("8.1e330985980541", "0x1.0E+274877906944", true);
    test("1.2e-330985980542", "0x1.0E-274877906944", false);
}

#[test]
fn is_gaussian_integer_properties() {
    big_exponent_float_gen().test_properties(|x| {
        if !big_exponent_float_exponent_is_small(&x) {
            return;
        }
        assert_eq!(x.is_gaussian_integer(), Integer::try_from(&x).is_ok());
    });

    big_exponent_float_gen_var_1().test_properties(|x| {
        assert_eq!(
            x.is_gaussian_integer(),
            big_exponent_float_to_float(&x)
                .unwrap()
                .is_gaussian_integer()
        );
    });
}
