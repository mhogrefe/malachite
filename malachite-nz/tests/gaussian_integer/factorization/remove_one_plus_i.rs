// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivI, ModPowerOf2, MulI, Parity};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::factorization::remove_one_plus_i::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_unsigned_pair_gen_var_1, integer_gen,
};
use std::str::FromStr;

#[test]
fn test_remove_one_plus_i() {
    let test = |s, out, k_out| {
        let x = GaussianInteger::from_str(s).unwrap();

        let (q, k) = x.remove_one_plus_i();
        assert!(q.real.is_valid());
        assert!(q.imaginary.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(k, k_out);

        let mut x_mut = x.clone();
        assert_eq!(x_mut.remove_one_plus_i_assign(), k_out);
        assert!(x_mut.real.is_valid());
        assert!(x_mut.imaginary.is_valid());
        assert_eq!(x_mut.to_string(), out);

        let (q, k) = gaussian_integer_remove_one_plus_i_naive(&x);
        assert_eq!(q.to_string(), out);
        assert_eq!(k, k_out);
    };
    test("0", "0", 0);
    test("1", "1", 0);
    test("i", "i", 0);
    test("-1", "-1", 0);
    test("3+2i", "3+2i", 0);
    test("1+i", "1", 1);
    test("-1+i", "i", 1);
    test("3+i", "2-i", 1);
    test("2", "-i", 2);
    test("2i", "1", 2);
    test("-2", "i", 2);
    test("4", "-1", 4);
    test("4i", "-i", 4);
    test("8", "i", 6);
    test("16", "1", 8);
    test("8+4i", "-2-i", 4);
    test("6+2i", "-1-2i", 3);
    test("1000000000000", "244140625", 24);
    test("1000000000000i", "244140625i", 24);
    test("1000000000000+1000000000000i", "244140625", 25);
}

#[test]
fn remove_one_plus_i_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let (q, k) = x.remove_one_plus_i();
        assert!(q.real.is_valid());
        assert!(q.imaginary.is_valid());

        let mut x_mut = x.clone();
        assert_eq!(x_mut.remove_one_plus_i_assign(), k);
        assert!(x_mut.real.is_valid());
        assert!(x_mut.imaginary.is_valid());
        assert_eq!(x_mut, q);

        assert_eq!(gaussian_integer_remove_one_plus_i_naive(&x), (q.clone(), k));

        assert_eq!(&q * gaussian_integer_one_plus_i_pow(k), x);
        if x == 0u32 {
            assert_eq!(q, GaussianInteger::ZERO);
            assert_eq!(k, 0);
        } else {
            // 1 + i divides a + bi iff a + b is even.
            assert!((&q.real + &q.imaginary).odd());
            assert_eq!(k == 0, (&x.real + &x.imaginary).odd());
        }

        let one_plus_i = GaussianInteger {
            real: Integer::ONE,
            imaginary: Integer::ONE,
        };
        let (q_alt, k_alt) = (&x * one_plus_i).remove_one_plus_i();
        assert_eq!(q_alt, q);
        assert_eq!(k_alt, if x == 0u32 { 0 } else { k + 1 });
        assert_eq!((&x).mul_i().remove_one_plus_i(), ((&q).mul_i(), k));
        assert_eq!((-&x).remove_one_plus_i(), (-&q, k));
        assert_eq!(q.remove_one_plus_i(), (q.clone(), 0));
    });

    // Shifting by u multiplies by 2^u = (1 + i)^(2u) i^(-u).
    gaussian_integer_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let (q, k) = x.remove_one_plus_i();
        let (q_shifted, k_shifted) = (&x << u).remove_one_plus_i();
        if x == 0u32 {
            assert_eq!(k_shifted, 0);
        } else {
            assert_eq!(k_shifted, k + (u << 1));
        }
        let mut q_expected = q;
        for _ in 0..u.mod_power_of_2(2) {
            q_expected = q_expected.div_i();
        }
        assert_eq!(q_shifted, q_expected);
    });

    integer_gen().test_properties(|n| {
        let (q, k) = GaussianInteger::from(n.clone()).remove_one_plus_i();
        if let Some(s) = n.trailing_zeros() {
            assert_eq!(k, s << 1);
            let mut q_expected = GaussianInteger::from(&n >> s);
            for _ in 0..s.mod_power_of_2(2) {
                q_expected = q_expected.div_i();
            }
            assert_eq!(q, q_expected);
        } else {
            assert_eq!(k, 0);
            assert_eq!(q, GaussianInteger::ZERO);
        }
    });
}
