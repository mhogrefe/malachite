// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulShl, AddMulShlAssign, Square};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_natural_natural_unsigned_quadruple_gen_var_1,
    natural_natural_unsigned_triple_gen_var_1, natural_triple_gen,
};
use malachite_nz::test_util::natural::arithmetic::add_mul_shl::add_mul_shl_naive;

#[test]
fn test_add_mul_shl() {
    let test = |s, t, u, bits, out| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let z = Natural::from_str(u).unwrap();
        let w = (&x).add_mul_shl(&y, &z, bits);
        assert!(w.is_valid());
        assert_eq!(w.to_string(), out);
        assert_eq!(x.clone().add_mul_shl(y.clone(), z.clone(), bits), w);
        assert_eq!(x.clone().add_mul_shl(y.clone(), &z, bits), w);
        assert_eq!(x.clone().add_mul_shl(&y, z.clone(), bits), w);
        assert_eq!(x.clone().add_mul_shl(&y, &z, bits), w);
        let mut v = x.clone();
        v.add_mul_shl_assign(y.clone(), z.clone(), bits);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.add_mul_shl_assign(y.clone(), &z, bits);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.add_mul_shl_assign(&y, z.clone(), bits);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.add_mul_shl_assign(&y, &z, bits);
        assert!(v.is_valid());
        assert_eq!(v, w);
        assert_eq!(add_mul_shl_naive(&x, &y, &z, bits), w);
    };
    test("0", "0", "0", 0, "0");
    test("10", "3", "4", 0, "22");
    test("10", "3", "4", 2, "58");
    test(
        "1000000000000",
        "65536",
        "1000000000000",
        3,
        "524289000000000000",
    );
    test("5", "0", "7", 100, "5");
    test("5", "7", "0", 100, "5");
    test("0", "1", "1", 64, "18446744073709551616");
    test(
        "18446744073709551615",
        "18446744073709551617",
        "18446744073709551615",
        70,
        "401734511064747568885490523085290650629388603569054507073535",
    );
    test(
        "1267650600228229401496703205376",
        "3",
        "5",
        128,
        "5104235505081727552178848512973226377216",
    );
}

#[test]
fn add_mul_shl_properties() {
    natural_natural_natural_unsigned_quadruple_gen_var_1::<u64>().test_properties(
        |(x, y, z, bits)| {
            let w = (&x).add_mul_shl(&y, &z, bits);
            assert!(w.is_valid());
            // The forms agree.
            assert_eq!(x.clone().add_mul_shl(y.clone(), z.clone(), bits), w);
            assert_eq!(x.clone().add_mul_shl(y.clone(), &z, bits), w);
            assert_eq!(x.clone().add_mul_shl(&y, z.clone(), bits), w);
            assert_eq!(x.clone().add_mul_shl(&y, &z, bits), w);
            let mut v = x.clone();
            v.add_mul_shl_assign(y.clone(), z.clone(), bits);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.add_mul_shl_assign(y.clone(), &z, bits);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.add_mul_shl_assign(&y, z.clone(), bits);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.add_mul_shl_assign(&y, &z, bits);
            assert!(v.is_valid());
            assert_eq!(v, w);

            // It agrees with multiplying by a power of 2, with the unfused combination, and with
            // `add_mul` on a shifted factor; and the factors commute.
            assert_eq!(add_mul_shl_naive(&x, &y, &z, bits), w);
            assert_eq!(&x + ((&y * &z) << bits), w);
            assert_eq!((&x).add_mul(&(&y << bits), &z), w);
            assert_eq!((&x).add_mul_shl(&z, &y, bits), w);
            // Adding never decreases the value.
            assert!(w >= x);
            // Shifts compose.
            assert_eq!(
                (&x).add_mul_shl(&(&y << 1u32), &z, bits),
                (&x).add_mul_shl(&y, &z, bits + 1)
            );
            // It agrees with the `Integer` implementation.
            assert_eq!(
                Integer::from(&x).add_mul_shl(Integer::from(&y), Integer::from(&z), bits),
                Integer::from(w)
            );
        },
    );

    natural_triple_gen().test_properties(|(x, y, z)| {
        // With no shift, this is `add_mul`.
        assert_eq!((&x).add_mul_shl(&y, &z, 0), (&x).add_mul(&y, &z));
    });

    natural_natural_unsigned_triple_gen_var_1::<u64>().test_properties(|(x, y, bits)| {
        // A zero factor changes nothing.
        assert_eq!((&x).add_mul_shl(&y, &Natural::ZERO, bits), x);
        assert_eq!((&x).add_mul_shl(&Natural::ZERO, &y, bits), x);
        // Starting from zero gives the shifted square.
        assert_eq!(
            Natural::ZERO.add_mul_shl(&y, &y, bits),
            (&y).square() << bits
        );
    });
}
