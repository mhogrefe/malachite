// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    AddMulShl, Square, SubMul, SubMulShl, SubMulShlAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_rational_rational_unsigned_quadruple_gen_var_1,
    rational_rational_unsigned_triple_gen_var_1, rational_triple_gen,
};
use malachite_q::test_util::rational::arithmetic::sub_mul_shl::sub_mul_shl_naive;

#[test]
fn test_sub_mul_shl() {
    let test = |s, t, u, bits, out| {
        let x = Rational::from_str(s).unwrap();
        let y = Rational::from_str(t).unwrap();
        let z = Rational::from_str(u).unwrap();
        let w = (&x).sub_mul_shl(&y, &z, bits);
        assert!(w.is_valid());
        assert_eq!(w.to_string(), out);
        assert_eq!(x.clone().sub_mul_shl(y.clone(), z.clone(), bits), w);
        assert_eq!(x.clone().sub_mul_shl(y.clone(), &z, bits), w);
        assert_eq!(x.clone().sub_mul_shl(&y, z.clone(), bits), w);
        assert_eq!(x.clone().sub_mul_shl(&y, &z, bits), w);
        let mut v = x.clone();
        v.sub_mul_shl_assign(y.clone(), z.clone(), bits);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.sub_mul_shl_assign(y.clone(), &z, bits);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.sub_mul_shl_assign(&y, z.clone(), bits);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.sub_mul_shl_assign(&y, &z, bits);
        assert!(v.is_valid());
        assert_eq!(v, w);
        assert_eq!(sub_mul_shl_naive(&x, &y, &z, bits), w);
    };
    test("0", "0", "0", 0, "0");
    test("1/2", "2/3", "3/4", 0, "0");
    test("1/2", "2/3", "3/4", 2, "-3/2");
    test("22/7", "-1/2", "1/3", 3, "94/21");
    test("-5/3", "7", "0", 100, "-5/3");
    test("5", "0", "7/9", 100, "5");
    test("1/3", "1/5", "1/7", 64, "-55340232221128654813/105");
    test(
        "18446744073709551615/7",
        "-3/2",
        "5/18446744073709551617",
        70,
        "340282366920938525444434695095861641215/129127208515966861319",
    );
}

#[test]
fn sub_mul_shl_properties() {
    rational_rational_rational_unsigned_quadruple_gen_var_1::<u64>().test_properties(
        |(x, y, z, bits)| {
            let w = (&x).sub_mul_shl(&y, &z, bits);
            assert!(w.is_valid());
            // The forms agree.
            assert_eq!(x.clone().sub_mul_shl(y.clone(), z.clone(), bits), w);
            assert_eq!(x.clone().sub_mul_shl(y.clone(), &z, bits), w);
            assert_eq!(x.clone().sub_mul_shl(&y, z.clone(), bits), w);
            assert_eq!(x.clone().sub_mul_shl(&y, &z, bits), w);
            let mut v = x.clone();
            v.sub_mul_shl_assign(y.clone(), z.clone(), bits);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.sub_mul_shl_assign(y.clone(), &z, bits);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.sub_mul_shl_assign(&y, z.clone(), bits);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.sub_mul_shl_assign(&y, &z, bits);
            assert!(v.is_valid());
            assert_eq!(v, w);

            // It agrees with multiplying by a power of 2, with the unfused combination, and with
            // `sub_mul` on a shifted factor; and the factors commute.
            assert_eq!(sub_mul_shl_naive(&x, &y, &z, bits), w);
            assert_eq!(&x - ((&y * &z) << bits), w);
            assert_eq!((&x).sub_mul(&(&y << bits), &z), w);
            assert_eq!((&x).sub_mul_shl(&z, &y, bits), w);
            // Negating a factor gives the opposite operation, which undoes this one.
            assert_eq!((&x).add_mul_shl(&-&y, &z, bits), w);
            assert_eq!((&w).add_mul_shl(&y, &z, bits), x);
            // Negating everything negates the result.
            assert_eq!((-&x).sub_mul_shl(-&y, &z, bits), -&w);
            // Shifts compose.
            assert_eq!(
                (&x).sub_mul_shl(&(&y << 1u32), &z, bits),
                (&x).sub_mul_shl(&y, &z, bits + 1)
            );
        },
    );

    rational_triple_gen().test_properties(|(x, y, z)| {
        // With no shift, this is `sub_mul`.
        assert_eq!((&x).sub_mul_shl(&y, &z, 0), (&x).sub_mul(&y, &z));
    });

    rational_rational_unsigned_triple_gen_var_1::<u64>().test_properties(|(x, y, bits)| {
        // A zero factor changes nothing.
        assert_eq!((&x).sub_mul_shl(&y, &Rational::ZERO, bits), x);
        assert_eq!((&x).sub_mul_shl(&Rational::ZERO, &y, bits), x);
        // Starting from zero gives the shifted product, negated.
        assert_eq!(
            Rational::ZERO.sub_mul_shl(&y, &y, bits),
            -((&y).square() << bits)
        );
    });
}
