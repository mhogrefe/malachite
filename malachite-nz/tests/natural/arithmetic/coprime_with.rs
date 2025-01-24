// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CoprimeWith, Gcd};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_pair_gen_var_27;
use malachite_nz::natural::arithmetic::coprime_with::{
    coprime_with_check_2, coprime_with_check_2_3, coprime_with_check_2_3_5,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen};
use std::str::FromStr;

#[test]
fn test_coprime_with() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u.clone().coprime_with(v.clone()), out);
        assert_eq!((&u).coprime_with(v.clone()), out);
        assert_eq!(u.clone().coprime_with(&v), out);
        assert_eq!((&u).coprime_with(&v), out);
    };
    test("0", "0", false);
    test("0", "6", false);
    test("6", "0", false);
    test("1", "6", true);
    test("6", "1", true);
    test("8", "12", false);
    test("54", "24", false);
    test("42", "56", false);
    test("48", "18", false);
    test("3", "5", true);
    test("12", "60", false);
    test("12", "90", false);
    test("12345678987654321", "98765432123456789", true);
    test("12345678987654321", "98765432123456827", false);
}

#[test]
fn coprime_with_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let c = (&x).coprime_with(&y);
        assert_eq!(x.clone().coprime_with(y.clone()), c);
        assert_eq!(x.clone().coprime_with(&y), c);
        assert_eq!((&x).coprime_with(y.clone()), c);

        assert_eq!((&x).gcd(&y) == 1, c);
        assert_eq!(coprime_with_check_2(x.clone(), y.clone()), c);
        assert_eq!(coprime_with_check_2_3(x.clone(), y.clone()), c);
        assert_eq!(coprime_with_check_2_3_5(x.clone(), y.clone()), c);
        assert_eq!(y.coprime_with(x), c);
    });

    natural_gen().test_properties(|x| {
        assert_eq!((&x).coprime_with(&x), x == 1);
        assert!((&x).coprime_with(Natural::ONE));
        assert_eq!((&x).coprime_with(Natural::ZERO), x == 1);
        let y = &x + Natural::ONE;
        assert!(x.coprime_with(y));
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(
            Natural::from(x).coprime_with(Natural::from(y)),
            x.coprime_with(y)
        );
    });
}
