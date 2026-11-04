// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{CheckedModGcd, ModCoprimeWith, ModGcd};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_5,
};

const P192: &str = "6277101735386680763835789423207666416102355444464034513029";

// The polynomial with the same coefficients as `p`, which must fit in limbs.
fn to_limbs(p: &NaturalPolynomial) -> UnsignedPolynomial<Limb> {
    UnsignedPolynomial::from_coefficients_asc(
        p.coefficients_asc()
            .iter()
            .map(Limb::wrapping_from)
            .collect(),
    )
}

// The polynomial `p`, reduced modulo `m`.
fn reduce(p: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    NaturalPolynomial::from_coefficients_asc(p.coefficients_asc().iter().map(|x| x % m).collect())
}

// Checks the coprimality of `p` and `q` modulo `m` in every way of calling it, symmetry, agreement
// with the checked GCD where that succeeds, and, when `m` fits in a limb, agreement with
// `UnsignedPolynomial`; and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> bool {
    let coprime = p.mod_coprime_with(q, m);
    assert_eq!(p.clone().mod_coprime_with(q.clone(), m.clone()), coprime);
    assert_eq!(p.clone().mod_coprime_with(q.clone(), m), coprime);
    assert_eq!(p.clone().mod_coprime_with(q, m.clone()), coprime);
    assert_eq!(p.clone().mod_coprime_with(q, m), coprime);
    assert_eq!(p.mod_coprime_with(q.clone(), m.clone()), coprime);
    assert_eq!(p.mod_coprime_with(q.clone(), m), coprime);
    assert_eq!(p.mod_coprime_with(q, m.clone()), coprime);
    assert_eq!(q.mod_coprime_with(p, m), coprime);
    if *m != 1u32
        && let Some(g) = p.checked_mod_gcd(q, m)
    {
        assert_eq!(g == NaturalPolynomial::one(), coprime);
    }
    if m.significant_bits() <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).mod_coprime_with(to_limbs(q), Limb::wrapping_from(m)),
            coprime
        );
    }
    coprime
}

#[test]
fn test_mod_coprime_with() {
    let test = |p, q, m, out| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(verify(&p, &q, &m), out);
    };
    // - bits <= Limb::WIDTH
    test("0", "0", "1", true);
    test("0", "0", "7", false);
    test("x^2+1", "x+1", "7", true);
    test("x^2+3*x+2", "x^2+4*x+3", "7", false);
    test("x^2+1", "2*x+1", "6", true);
    test("x^2+x+1", "x+2", "6", false);
    test("2*x+1", "0", "4", true);
    // - Limb::WIDTH < bits <= DoubleLimb::WIDTH
    test(
        "x^2+1",
        "x+1",
        "340282366920938463463374607431768211297",
        true,
    );
    test(
        "x^2+3*x+2",
        "x^2+4*x+3",
        "340282366920938463463374607431768211297",
        false,
    );
    // - bits > DoubleLimb::WIDTH, a prime
    // - bits > DoubleLimb::WIDTH && coprime_or_non_invertible(..).is_ok()
    test("x^2+1", "x+1", P192, true);
    test("x^2+3*x+2", "x^2+4*x+3", P192, false);
    // - bits > DoubleLimb::WIDTH, a non-invertible leading coefficient, splitting 3 * P192 into 3
    //   and P192
    // - bits > DoubleLimb::WIDTH && coprime_or_non_invertible(..).is_err()
    // - ys.last().is_some_and(|y| *y == 0u32)
    test(
        "x+1",
        "3*x+1",
        "18831305206160042291507368269622999248307066333392103539087",
        true,
    );
    test(
        "x^2+1",
        "3*x+3",
        "18831305206160042291507368269622999248307066333392103539087",
        false,
    );
    // - a non-invertible leading coefficient splitting P192^2 into P192 and P192: 1 + P192 x is a
    //   unit
    test(
        "6277101735386680763835789423207666416102355444464034513029*x+1",
        "0",
        "39402006196394479212279040100143613805079739270465446669618002465858578854677530\
        597987505521568142189034061170754841",
        true,
    );
    test(
        "6277101735386680763835789423207666416102355444464034513029*x",
        "6277101735386680763835789423207666416102355444464034513029",
        "39402006196394479212279040100143613805079739270465446669618002465858578854677530\
        597987505521568142189034061170754841",
        false,
    );
}

#[test]
#[should_panic]
fn mod_coprime_with_fail_1() {
    NaturalPolynomial::from_str("x+7")
        .unwrap()
        .mod_coprime_with(
            NaturalPolynomial::from_str("x").unwrap(),
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_coprime_with_fail_2() {
    NaturalPolynomial::from_str("x")
        .unwrap()
        .mod_coprime_with(NaturalPolynomial::from_str("x").unwrap(), Natural::ZERO);
}

#[test]
fn mod_coprime_with_properties() {
    let big_prime = Natural::from_str(P192).unwrap();
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            let coprime = verify(&p, &q, &m);
            // Coprimality depends only on the prime factors of the modulus.
            assert_eq!((&p).mod_coprime_with(&q, &m * &m), coprime);
            let mp = &m * &big_prime;
            assert_eq!(
                (&p).mod_coprime_with(&q, &mp),
                coprime
                    && reduce(&p, &big_prime).mod_coprime_with(reduce(&q, &big_prime), &big_prime)
            );
        },
    );

    natural_polynomial_natural_polynomial_natural_triple_gen_var_5().test_properties(
        |(p, q, m)| {
            assert_eq!(
                verify(&p, &q, &m),
                p.mod_gcd(q, m) == NaturalPolynomial::one()
            );
        },
    );
}
