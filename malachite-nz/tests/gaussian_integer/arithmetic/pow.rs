// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    AbsSquared, CanonicalizeUnit, CheckedRoot, Conjugate, MulI, MulIPow, Parity, Pow, PowAssign,
    PowerOf2, Square,
};
use malachite_base::num::basic::traits::{I, One, Two, Zero};
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::pow::*;
use malachite_nz::test_util::gaussian_integer::factorization::remove_one_plus_i::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_pair_gen, gaussian_integer_unsigned_pair_gen_var_1,
    integer_unsigned_pair_gen_var_2,
};
use std::str::FromStr;

#[test]
fn test_pow() {
    let test = |s, exp, out| {
        let x = GaussianInteger::from_str(s).unwrap();

        let mut mut_x = x.clone();
        mut_x.pow_assign(exp);
        assert_eq!(mut_x.to_string(), out);
        assert!(mut_x.real.is_valid());
        assert!(mut_x.imaginary.is_valid());

        assert_eq!(x.clone().pow(exp).to_string(), out);
        assert_eq!((&x).pow(exp).to_string(), out);

        assert_eq!(gaussian_integer_pow_naive(&x, exp).to_string(), out);
    };
    test("0", 0, "1");
    test("0", 1, "0");
    test("0", 5, "0");
    test("1", 0, "1");
    test("1", 100, "1");
    // powers of i
    test("i", 0, "1");
    test("i", 1, "i");
    test("i", 2, "-1");
    test("i", 3, "-i");
    test("i", 4, "1");
    test("i", 5, "i");
    test("i", 6, "-1");
    test("i", 7, "-i");
    test("-i", 3, "i");
    // purely real and purely imaginary bases
    test("2", 10, "1024");
    test("-2", 3, "-8");
    test("2i", 3, "-8i");
    test("3i", 4, "81");
    test("-3i", 5, "-243i");
    // (1+i)^2 = 2i
    test("1+i", 2, "2i");
    test("1+i", 4, "-4");
    test("1+i", 8, "16");
    test("1+i", 10, "32i");
    // binary exponentiation
    test("2+i", 2, "3+4i");
    test("2+i", 3, "2+11i");
    test("2+i", 5, "-38+41i");
    test("-3+2i", 3, "9+46i");
    test("2+3i", 10, "-341525-145668i");
    test("-7+24i", 4, "164833+354144i");
    test("123-456i", 7, "-5047356678475874157-1415768128068037176i");
}

#[test]
fn pow_properties() {
    gaussian_integer_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, exp)| {
        let power = (&x).pow(exp);
        assert!(power.real.is_valid());
        assert!(power.imaginary.is_valid());

        let power_alt = x.clone().pow(exp);
        assert!(power_alt.real.is_valid());
        assert!(power_alt.imaginary.is_valid());
        assert_eq!(power_alt, power);

        let mut power_alt = x.clone();
        power_alt.pow_assign(exp);
        assert!(power_alt.real.is_valid());
        assert!(power_alt.imaginary.is_valid());
        assert_eq!(power_alt, power);

        assert_eq!(gaussian_integer_pow_naive(&x, exp), power);

        // the norm is multiplicative and conjugation is a ring homomorphism
        assert_eq!((&power).abs_squared(), (&x).abs_squared().pow(exp));
        assert_eq!((&x).conjugate().pow(exp), (&power).conjugate());
        // units come out of the base as powers
        assert_eq!(
            (-&x).pow(exp),
            if exp.even() { power.clone() } else { -&power }
        );
        assert_eq!((&x).mul_i().pow(exp), (&power).mul_i_pow(exp));
        // exponent laws, against small extra exponents
        for f in 0..4 {
            assert_eq!((&x).pow(exp + f), &power * (&x).pow(f));
            assert_eq!((&x).pow(exp * f), (&power).pow(f));
        }
        // the root of the power is the principal rotation of the base, which is among the roots
        if exp != 0 {
            let principal = match TrailingZeros::trailing_zeros(exp) {
                0 => x.clone(),
                1 => {
                    if (&x.real, &x.imaginary) > (&Integer::ZERO, &Integer::ZERO) {
                        x.clone()
                    } else {
                        -&x
                    }
                }
                _ => (&x).canonicalize_unit(),
            };
            assert_eq!((&power).checked_root(exp), Some(principal));
            assert!(power.checked_roots(exp).contains(&x));
        }
    });

    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        for exp in 0..4 {
            assert_eq!((&x * &y).pow(exp), (&x).pow(exp) * (&y).pow(exp));
        }
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!((&x).pow(0), GaussianInteger::ONE);
        assert_eq!((&x).pow(1), x);
        assert_eq!((&x).pow(2), (&x).square());
    });

    unsigned_gen_var_5().test_properties(|exp| {
        assert_eq!(
            GaussianInteger::ZERO.pow(exp),
            GaussianInteger::from(u64::from(exp == 0))
        );
        assert_eq!(GaussianInteger::ONE.pow(exp), GaussianInteger::ONE);
        assert_eq!(
            GaussianInteger::TWO.pow(exp),
            GaussianInteger::power_of_2(exp)
        );
        assert_eq!(
            GaussianInteger::I.pow(exp),
            GaussianInteger::ONE.mul_i_pow(exp)
        );
        let one_plus_i = GaussianInteger {
            real: Integer::ONE,
            imaginary: Integer::ONE,
        };
        assert_eq!(one_plus_i.pow(exp), gaussian_integer_one_plus_i_pow(exp));
    });

    integer_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, exp)| {
        assert_eq!(
            GaussianInteger::from(x.clone()).pow(exp),
            GaussianInteger::from(x.pow(exp))
        );
    });
}
