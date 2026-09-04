// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    AbsSquared, CanonicalizeUnit, CheckedRoot, Conjugate, MulI, MulIPow, Parity, Pow, PowAssign,
    PowerOf2, Reciprocal, Square,
};
use malachite_base::num::basic::traits::{I, One, Two, Zero};
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{signed_gen_var_5, unsigned_gen_var_5};
use malachite_nz::test_util::generators::gaussian_integer_unsigned_pair_gen_var_1;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::gaussian_rational::arithmetic::pow::*;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_gen_var_3, gaussian_rational_pair_gen,
    gaussian_rational_signed_pair_gen_var_1, gaussian_rational_unsigned_pair_gen_var_1,
    rational_signed_pair_gen_var_2, rational_unsigned_pair_gen_var_1,
};
use std::str::FromStr;

#[test]
fn test_pow() {
    let test = |s, exp: i64, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let mut mut_x = x.clone();
        mut_x.pow_assign(exp);
        assert_eq!(mut_x.to_string(), out);
        assert!(mut_x.real.is_valid());
        assert!(mut_x.imaginary.is_valid());

        assert_eq!(x.clone().pow(exp).to_string(), out);
        assert_eq!((&x).pow(exp).to_string(), out);
        assert_eq!(gaussian_rational_pow_naive(&x, exp).to_string(), out);

        if exp >= 0 {
            let exp = exp.unsigned_abs();
            let mut mut_x = x.clone();
            mut_x.pow_assign(exp);
            assert_eq!(mut_x.to_string(), out);
            assert_eq!(x.clone().pow(exp).to_string(), out);
            assert_eq!((&x).pow(exp).to_string(), out);
        }
    };
    test("0", 0, "1");
    test("0", 3, "0");
    test("1", 100, "1");
    test("1", -100, "1");
    test("i", 3, "-i");
    test("i", -1, "-i");
    test("i", -2, "-1");
    test("i", -3, "i");
    test("1/2", 3, "1/8");
    test("1/2", -3, "8");
    test("2i/3", 3, "-8i/27");
    test("2i/3", -3, "27i/8");
    test("1+i", 2, "2i");
    test("1+i", -1, "1/2-i/2");
    test("1+i", -2, "-i/2");
    test("2+i", -1, "2/5-i/5");
    test("2+i", -2, "3/25-4i/25");
    test("2+i", 5, "-38+41i");
    test("2+i", -5, "-38/3125-41i/3125");
    test("1/2+i/3", 3, "-1/24+23i/108");
    test("1/2+i/3", -3, "-1944/2197-9936i/2197");
    test("22/7-i/3", 4, "17696473/194481-379016i/9261");
    test(
        "22/7-i/3",
        -4,
        "3441627765513/376516186200625+1547939624616i/376516186200625",
    );
    test("-3/5+4i/5", 7, "-76443/78125+16124i/78125");
    test("-3/5+4i/5", -7, "-76443/78125-16124i/78125");
}

#[test]
#[should_panic]
fn pow_i64_fail() {
    GaussianRational::ZERO.pow(-1i64);
}

#[test]
#[should_panic]
fn pow_i64_ref_fail() {
    (&GaussianRational::ZERO).pow(-1i64);
}

#[test]
#[should_panic]
fn pow_assign_i64_fail() {
    let mut x = GaussianRational::ZERO;
    x.pow_assign(-1i64);
}

// The root with argument in (-pi/g, pi/g], g = gcd(exp, 4), among the rotations of `x`.
fn principal(x: GaussianRational, exp: u64) -> GaussianRational {
    match TrailingZeros::trailing_zeros(exp) {
        0 => x,
        1 => {
            if (&x.real, &x.imaginary) > (&Rational::ZERO, &Rational::ZERO) {
                x
            } else {
                -x
            }
        }
        _ => x.canonicalize_unit(),
    }
}

#[test]
fn pow_properties() {
    // Powers of Gaussian rationals grow quickly in both parts, so the exponents are kept smaller
    // than the generators' default.
    let mut config = GenConfig::new();
    config.insert("mean_small_n", 32);

    // exponent is u64

    gaussian_rational_unsigned_pair_gen_var_1::<u64>().test_properties_with_config(
        &config,
        |(x, exp)| {
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

            // the oracles reduce after every multiplication, so the slower one is kept to small
            // exponents
            if exp <= 8 {
                assert_eq!(
                    gaussian_rational_pow_naive(&x, i64::try_from(exp).unwrap()),
                    power
                );
            }
            if exp <= 32 {
                assert_eq!(gaussian_rational_pow_binary(&x, exp), power);
            }
            assert_eq!((&x).pow(i64::try_from(exp).unwrap()), power);

            // each of these is another full power, so the test keeps to a few
            assert_eq!((&power).abs_squared(), (&x).abs_squared().pow(exp));
            assert_eq!(
                (-&x).pow(exp),
                if exp.even() { power.clone() } else { -&power }
            );
            assert_eq!((&x).mul_i().pow(exp), (&power).mul_i_pow(exp));
            assert_eq!((&x).pow(exp + 1), &power * &x);
            // the root of the power is the principal rotation of the base, which is among the
            // roots
            if exp != 0 {
                assert_eq!((&power).checked_root(exp), Some(principal(x.clone(), exp)));
                assert!(power.checked_roots(exp).contains(&x));
            }
        },
    );

    gaussian_rational_pair_gen().test_properties(|(x, y)| {
        for exp in 0..4u64 {
            assert_eq!((&x * &y).pow(exp), (&x).pow(exp) * (&y).pow(exp));
        }
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!((&x).pow(0u64), GaussianRational::ONE);
        assert_eq!((&x).pow(1u64), x);
        assert_eq!((&x).pow(2u64), (&x).square());
    });

    unsigned_gen_var_5::<u64>().test_properties(|exp| {
        assert_eq!(
            GaussianRational::ZERO.pow(exp),
            GaussianRational::from(u64::from(exp == 0))
        );
        assert_eq!(GaussianRational::ONE.pow(exp), GaussianRational::ONE);
        assert_eq!(
            GaussianRational::TWO.pow(exp),
            GaussianRational::power_of_2(exp)
        );
        assert_eq!(
            GaussianRational::I.pow(exp),
            GaussianRational::ONE.mul_i_pow(exp)
        );
    });

    gaussian_integer_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, exp)| {
        assert_eq!(
            GaussianRational::from(x.clone()).pow(exp),
            GaussianRational::from(x.pow(exp))
        );
    });

    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, exp)| {
        assert_eq!(
            GaussianRational::from(x.clone()).pow(exp),
            GaussianRational::from(x.pow(exp))
        );
    });

    // exponent is i64

    gaussian_rational_signed_pair_gen_var_1::<i64>().test_properties_with_config(
        &config,
        |(x, exp)| {
            if x == GaussianRational::ZERO && exp < 0 {
                return;
            }
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

            if exp.unsigned_abs() <= 8 {
                assert_eq!(gaussian_rational_pow_naive(&x, exp), power);
            }

            assert_eq!((&power).abs_squared(), (&x).abs_squared().pow(exp));
            assert_eq!((&x).conjugate().pow(exp), (&power).conjugate());
            if x != GaussianRational::ZERO {
                assert_eq!((&x).pow(-exp), (&power).reciprocal());
            }
            // a positive exponent's power has the base among its roots, and a negative one's has
            // the reciprocal, since (1/x)^|e| = x^e
            if exp != 0 {
                let base = if exp > 0 {
                    x.clone()
                } else {
                    (&x).reciprocal()
                };
                let abs_exp = exp.unsigned_abs();
                assert_eq!(
                    (&power).checked_root(abs_exp),
                    Some(principal(base.clone(), abs_exp))
                );
                assert!(power.checked_roots(abs_exp).contains(&base));
            }
        },
    );

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!((&x).pow(0i64), GaussianRational::ONE);
        assert_eq!((&x).pow(1i64), x);
        assert_eq!((&x).pow(2i64), (&x).square());
    });

    gaussian_rational_gen_var_3().test_properties(|x| {
        assert_eq!((&x).pow(-1i64), (&x).reciprocal());
        assert_eq!((&x).pow(-2i64), (&x).square().reciprocal());
    });

    signed_gen_var_5::<i64>().test_properties(|exp| {
        if exp >= 0 {
            assert_eq!(
                GaussianRational::ZERO.pow(exp),
                GaussianRational::from(u64::from(exp == 0))
            );
        }
        assert_eq!(GaussianRational::ONE.pow(exp), GaussianRational::ONE);
        assert_eq!(
            GaussianRational::TWO.pow(exp),
            GaussianRational::power_of_2(exp)
        );
        assert_eq!(
            GaussianRational::I.pow(exp),
            GaussianRational::ONE.mul_i_pow(exp.rem_euclid(4).unsigned_abs())
        );
    });

    rational_signed_pair_gen_var_2::<i64>().test_properties(|(x, exp)| {
        assert_eq!(
            GaussianRational::from(x.clone()).pow(exp),
            GaussianRational::from(x.pow(exp))
        );
    });
}
