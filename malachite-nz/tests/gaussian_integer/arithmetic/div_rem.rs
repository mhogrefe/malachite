// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    AbsSquared, Conjugate, DivAssignRem, DivExact, DivRem,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::div_rem::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_3, gaussian_integer_pair_gen_var_1,
    gaussian_integer_pair_gen_var_3,
};
use std::str::FromStr;

#[test]
fn test_div_rem() {
    let test = |s, t, q_out, r_out| {
        let u = GaussianInteger::from_str(s).unwrap();
        let v = GaussianInteger::from_str(t).unwrap();

        let mut x = u.clone();
        let r = x.div_assign_rem(v.clone());
        assert_eq!(x.to_string(), q_out);
        assert_eq!(r.to_string(), r_out);
        assert!(x.real.is_valid());
        assert!(x.imaginary.is_valid());
        assert!(r.real.is_valid());
        assert!(r.imaginary.is_valid());

        let mut x = u.clone();
        let r = x.div_assign_rem(&v);
        assert_eq!(x.to_string(), q_out);
        assert_eq!(r.to_string(), r_out);

        let (q, r) = u.clone().div_rem(v.clone());
        assert_eq!(q.to_string(), q_out);
        assert_eq!(r.to_string(), r_out);

        let (q, r) = u.clone().div_rem(&v);
        assert_eq!(q.to_string(), q_out);
        assert_eq!(r.to_string(), r_out);

        let (q, r) = (&u).div_rem(v.clone());
        assert_eq!(q.to_string(), q_out);
        assert_eq!(r.to_string(), r_out);

        let (q, r) = (&u).div_rem(&v);
        assert_eq!(q.to_string(), q_out);
        assert_eq!(r.to_string(), r_out);

        let (q, r) = gaussian_integer_div_rem_naive(&u, &v);
        assert_eq!(q.to_string(), q_out);
        assert_eq!(r.to_string(), r_out);
    };
    // zero dividend
    test("0", "1", "0", "0");
    test("0", "3+4i", "0", "0");
    // dividend much smaller than divisor
    test("1", "100+100i", "0", "1");
    test("-3i", "1000", "0", "-3i");
    // exact
    test("23+14i", "5-2i", "3+4i", "0");
    test("6+9i", "3", "2+3i", "0");
    test("6+9i", "3i", "3-2i", "0");
    test("1+i", "1+i", "1", "0");
    // inexact; ties round up in each part
    test("5+3i", "2+i", "3", "-1");
    test("1", "1+i", "1", "-i");
    test("i", "1+i", "1+i", "-i");
    test("7", "2", "4", "-1");
    test("-7", "2", "-3", "-1");
    test("3i", "2", "2i", "-i");
    test("1", "2", "1", "-1");
    test("-1", "2", "0", "-1");
    test("1+i", "2", "1+i", "-1-i");
    test("-1-i", "2", "0", "-1-i");
    test("1000000000001", "7", "142857142857", "2");
    test(
        "123456789012345678901234567890+98765432109876543210i",
        "12345678901234567890-9876543210i",
        "10000000000+16i",
        "-156790123470+1790123456970i",
    );
}

#[test]
#[should_panic]
fn div_rem_fail() {
    GaussianInteger::ONE.div_rem(GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_rem_val_ref_fail() {
    GaussianInteger::ONE.div_rem(&GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_rem_ref_val_fail() {
    (&GaussianInteger::ONE).div_rem(GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_rem_ref_ref_fail() {
    (&GaussianInteger::ONE).div_rem(&GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_assign_rem_fail() {
    let mut x = GaussianInteger::ONE;
    x.div_assign_rem(GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_assign_rem_ref_fail() {
    let mut x = GaussianInteger::ONE;
    x.div_assign_rem(&GaussianInteger::ZERO);
}

#[allow(clippy::needless_pass_by_value)]
fn div_rem_properties_helper(x: GaussianInteger, y: GaussianInteger) {
    let mut mut_x = x.clone();
    let r = mut_x.div_assign_rem(&y);
    assert!(mut_x.real.is_valid());
    assert!(mut_x.imaginary.is_valid());
    assert!(r.real.is_valid());
    assert!(r.imaginary.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    let r_alt = mut_x.div_assign_rem(y.clone());
    assert_eq!(mut_x, q);
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = (&x).div_rem(&y);
    assert!(q_alt.real.is_valid());
    assert!(q_alt.imaginary.is_valid());
    assert!(r_alt.real.is_valid());
    assert!(r_alt.imaginary.is_valid());
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);
    assert_eq!((&x).div_rem(y.clone()), (q.clone(), r.clone()));
    assert_eq!(x.clone().div_rem(&y), (q.clone(), r.clone()));
    assert_eq!(x.clone().div_rem(y.clone()), (q.clone(), r.clone()));

    assert_eq!(
        gaussian_integer_div_rem_naive(&x, &y),
        (q.clone(), r.clone())
    );

    // x = qy + r
    assert_eq!(&q * &y + &r, x);
    // N(r) <= N(y) / 2
    let norm = (&y).abs_squared();
    assert!(r.abs_squared() << 1u32 <= norm);
    // Each part of the exact quotient x conj(y) / N(y) lies in [q - 1/2, q + 1/2), i.e. -N(y) <= 2
    // x conj(y) - 2 q N(y) < N(y), part by part.
    let t = &x * (&y).conjugate();
    let neg_norm = -&norm;
    for (t_part, q_part) in [(&t.real, &q.real), (&t.imaginary, &q.imaginary)] {
        let diff: Integer = (t_part << 1u32) - ((q_part * &norm) << 1u32);
        assert!(diff >= neg_norm);
        assert!(diff < norm);
    }
    // Conjugation commutes with the division only away from ties, and the tie rule is not symmetric
    // under negation either, so no such properties are asserted.
}

#[test]
fn div_rem_properties() {
    gaussian_integer_pair_gen_var_3().test_properties(|(x, y)| {
        div_rem_properties_helper(x, y);
    });

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 512);
    gaussian_integer_pair_gen_var_3().test_properties_with_config(&config, |(x, y)| {
        div_rem_properties_helper(x, y);
    });

    gaussian_integer_pair_gen_var_1().test_properties(|(x, y)| {
        let (q, r) = (&x).div_rem(&y);
        assert_eq!(q, x.div_exact(y));
        assert_eq!(r, GaussianInteger::ZERO);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(
            (&x).div_rem(GaussianInteger::ONE),
            (x.clone(), GaussianInteger::ZERO)
        );
    });

    gaussian_integer_gen_var_3().test_properties(|x| {
        assert_eq!(
            GaussianInteger::ZERO.div_rem(&x),
            (GaussianInteger::ZERO, GaussianInteger::ZERO)
        );
        assert_eq!(
            (&x).div_rem(&x),
            (GaussianInteger::ONE, GaussianInteger::ZERO)
        );
    });
}
