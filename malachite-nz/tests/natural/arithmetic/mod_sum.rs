// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModNeg, ModSum};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_pair_gen_var_8, natural_vector_natural_pair_gen_var_2,
};

#[test]
fn test_mod_sum() {
    let test = |s, m, out| {
        let xs = vec_from_str::<Natural>(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let sum = Natural::mod_sum(xs.iter(), &m);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(Natural::mod_sum(xs.iter(), m.clone()), sum);
        assert_eq!(Natural::mod_sum(xs.iter().cloned(), &m), sum);
        let sum_alt = Natural::mod_sum(xs.into_iter(), m);
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);
    };
    test("[]", "1", "0");
    test("[]", "1000000000000000000000000000000", "0");
    test("[0]", "1", "0");
    test("[2, 3, 5, 7]", "10", "7");
    test("[2, 3, 5, 7]", "18", "17");
    test(
        "[18446744073709551614, 18446744073709551614, 1]",
        "18446744073709551615",
        "18446744073709551614",
    );
    test(
        "[1000000000000000000000000000056, 1000000000000000000000000000056]",
        "1000000000000000000000000000057",
        "1000000000000000000000000000055",
    );
    test(
        "[1267650600228229401496703205374, 1267650600228229401496703205374]",
        "1267650600228229401496703205375",
        "1267650600228229401496703205373",
    );
}

#[test]
#[should_panic]
fn mod_sum_fail_1() {
    Natural::mod_sum(core::iter::empty::<Natural>(), Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_sum_fail_2() {
    Natural::mod_sum([Natural::ONE].iter(), &Natural::ONE);
}

#[test]
#[should_panic]
fn mod_sum_fail_3() {
    let m = Natural::from_str("1000000000000000000000000000000").unwrap();
    Natural::mod_sum([Natural::ONE, m.clone()].into_iter(), m);
}

#[test]
fn mod_sum_properties() {
    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        let xs = v.elements;
        let sum = Natural::mod_sum(xs.iter(), &m);
        assert!(sum.is_valid());
        assert!(sum.mod_is_reduced(&m));
        assert_eq!(Natural::mod_sum(xs.iter(), m.clone()), sum);
        assert_eq!(Natural::mod_sum(xs.iter().cloned(), &m), sum);
        assert_eq!(Natural::mod_sum(xs.iter().cloned(), m.clone()), sum);
        // It is the exact sum, reduced, and the modular sum taken one element at a time, in any
        // order.
        assert_eq!(Natural::sum(xs.iter()) % &m, sum);
        assert_eq!(xs.iter().fold(Natural::ZERO, |s, x| s.mod_add(x, &m)), sum);
        assert_eq!(Natural::mod_sum(xs.iter().rev(), &m), sum);
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        assert_eq!(Natural::mod_sum([&x].into_iter(), &m), x);
        assert_eq!(
            Natural::mod_sum([&x, &(&x).mod_neg(&m)].into_iter(), &m),
            0u32
        );
    });

    unsigned_vector_unsigned_pair_gen_var_5::<Limb>().test_properties(|(v, m)| {
        // It agrees with the primitive implementation.
        assert_eq!(
            Natural::mod_sum(
                v.elements.iter().map(|&x| Natural::from(x)),
                Natural::from(m)
            ),
            Limb::mod_sum(v.elements.iter(), m)
        );
    });
}
