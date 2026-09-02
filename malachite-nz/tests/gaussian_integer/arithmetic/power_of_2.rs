// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::IsReal;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;

#[test]
fn test_power_of_2() {
    let test = |pow, out| assert_eq!(GaussianInteger::power_of_2(pow).to_string(), out);
    test(0, "1");
    test(1, "2");
    test(2, "4");
    test(3, "8");
    test(32, "4294967296");
    test(100, "1267650600228229401496703205376");
}

#[test]
fn power_of_2_properties() {
    unsigned_gen_var_5().test_properties(|pow| {
        let n = GaussianInteger::power_of_2(pow);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        assert!(n.is_real());
        assert_eq!(n.imaginary, Integer::ZERO);
        assert_eq!(n.real, Integer::power_of_2(pow));
        assert_eq!(n, GaussianInteger::from(Integer::power_of_2(pow)));
        assert!(n.is_power_of_2());
    });
}
