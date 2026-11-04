// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, Content, DivExact, Gcd, GcdAssign, Pow, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::gcd::large_primes;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_7,
    integer_polynomial_triple_gen,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::gcd::*;

// Checks the GCD of `p` and `q` in every way of calling it, against the subresultant algorithm, the
// modular algorithm, and the heuristic algorithm when it succeeds; checks that it is 0 or has a
// positive leading coefficient, that it divides both, and that its content is the GCD of theirs;
// and returns it.
fn verify(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    let g = p.gcd(q);
    assert_eq!(p.clone().gcd(q.clone()), g);
    assert_eq!(p.clone().gcd(q), g);
    assert_eq!(p.gcd(q.clone()), g);
    let mut u = p.clone();
    u.gcd_assign(q);
    assert_eq!(u, g);
    let mut u = p.clone();
    u.gcd_assign(q.clone());
    assert_eq!(u, g);
    assert_eq!(gcd_subresultant_wrapper(p, q), g);
    assert_eq!(gcd_modular_wrapper(p, q), g);
    if let Some(h) = gcd_heuristic_wrapper(p, q) {
        assert_eq!(h, g);
    }
    assert_eq!(q.gcd(p), g);
    assert!(g.is_valid());
    if g != IntegerPolynomial::ZERO {
        assert!(*g.leading_coefficient() > 0u32);
        assert!(p.divisible_by(&g));
        assert!(q.divisible_by(&g));
    }
    assert_eq!(
        if g == IntegerPolynomial::ZERO {
            Natural::ZERO
        } else {
            (&g).content()
        },
        content_or_zero(p).gcd(content_or_zero(q))
    );
    g
}

fn content_or_zero(p: &IntegerPolynomial) -> Natural {
    if *p == IntegerPolynomial::ZERO {
        Natural::ZERO
    } else {
        p.content()
    }
}

#[test]
fn test_gcd() {
    let test = |p, q, out: Option<&str>| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        assert_eq!(
            Some(verify(&p, &q)).map(|g| g.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - both 0
    // - !(p.coefficients.len() < q.coefficients.len())
    // - p.coefficients.is_empty()
    test("0", "0", Some("0"));
    // - the second is 0, with a negative leading coefficient
    // - p.coefficients.len() < q.coefficients.len()
    // - q.coefficients.is_empty()
    // - !(*p.coefficients.last().unwrap() > 0u32)
    test("-3*x+6", "0", Some("3*x-6"));
    // - the first is 0
    // - *p.coefficients.last().unwrap() > 0u32
    test("0", "2*x-4", Some("2*x-4"));
    // - constants
    // - ys.len() == 1, the second
    // - ys.len() == 1, the third
    // - ys.len() == 1, the fourth
    // - len1 < 6
    // - !(p.coefficients.is_empty()) && !(q.coefficients.is_empty())
    test("6", "-4", Some("2"));
    // - a constant and a nonconstant
    test("6*x^2+3", "9", Some("3"));
    // - coprime
    // - d != e
    // - a.len() <= 1
    // - a.len() == 1
    // - len2 == 2
    // - !(divides(&a, &b))
    // - *x >= 0u32 || r == 0
    // - len1 < 64 && len2 < 64
    // - i.odd(), the first
    // - !(i.odd()), the first
    // - i.odd(), the second
    // - !(i.odd()), the second
    // - h_len == 1
    test("x^2+1", "x+1", Some("1"));
    // - a common content
    // - !(a.len() == 1)
    // - divides(&a, &b)
    // - !(*x >= 0u32 || r == 0)
    // - !(x > p >> 1)
    // - h_len <= n
    // - g_is_one, the first
    // - divides(&b, &res) && divides(&a, &res), the first
    test("6*x+6", "4*x^2-4", Some("2*x+2"));
    // - powers of x
    // - xs.last().is_some_and(|x| *x == 0u32)
    // - delta == 1
    // - val1 != 0 || val2 != 0
    // - len1 - val1 >= len2 - val2
    test("x^5+x^3", "2*x^4", Some("x^3"));
    // - short polynomials, by the subresultant algorithm
    // - x > p >> 1
    test("x^3-1", "x^2-1", Some("x-1"));
    // - one short polynomial dividing the other
    test("x^4-1", "x^3+x^2+x+1", Some("x^3+x^2+x+1"));
    // - small coefficients and degree at least 5, by the heuristic algorithm
    // - g_content != 1u32
    // - !(len1 < 6)
    test("x^6-1", "x^5+x^4-x-1", Some("x^2-1"));
    // - the heuristic with a quotient whose signs differ
    // - *b.last().unwrap() < 0u32
    test("-x^6+1", "x^5+x^4-x-1", Some("x^2-1"));
    // - a linear second polynomial in the heuristic algorithm
    test("x^6-1", "2*x-2", Some("x-1"));
    // - large coefficients, by the modular algorithm
    // - pack_bits >= 32
    test(
        "100000000000000000000*x^6-100000000000000000000",
        "100000000000000000000*x^5+100000000000000000000*x^4-x-1",
        Some("x+1"),
    );
    // - large coefficients and a nonmonic GCD, by the modular algorithm
    // - !(*xs.last().unwrap() < 0u32)
    // - !g_is_one
    // - !(g_is_one), the first
    // - new_bits == curr_bits || modulus.significant_bits() >= bits_small
    // - !(g_is_one), the second
    test(
        "30000000000000000000000*x^6+20000000000000000000001*x+1",
        "60000000000000000000000*x^7+40000000000000000000002*x^2+2*x",
        Some("30000000000000000000000*x^6+20000000000000000000001*x+1"),
    );
    // - a degree difference of 2 in a subresultant step
    // - !(delta == 1)
    test("x^4+1", "x^2+x+1", Some("1"));
    // - a linear second polynomial with a negative leading coefficient, in the heuristic algorithm
    // - r[1] < 0u32
    test("x^6-1", "-2*x+2", Some("x-1"));
    // - the heuristic algorithm's first quotient failing to multiply out
    // - bits_g + max_bits(&qs) + log_length >= pack_bits && !multiplies_out(&a, &qs, &g, negate1)
    test("x^5+x^4-x^3+x", "-x^5+x^4-x^3+x^2+x", Some("x"));
    // - the heuristic algorithm's second quotient failing to multiply out
    // - negate
    // - bits_g + max_bits(&qs) + log_length >= pack_bits && !multiplies_out(&b, &qs, &g, negate2)
    // - !(len1 - val1 >= len2 - val2)
    test("x^4-3*x^3", "-3*x^4-3*x-1", Some("1"));
    // - the heuristic algorithm negating a product to check a quotient
    test("-2*x^5+3*x^4-x^3+2*x^2-2*x-3", "-x^2-x+2", Some("1"));
    // - a first polynomial made shorter than the second by removing powers of x
    test("x^5+x^4", "x^3+x+1", Some("1"));
    // - a prime dividing a leading coefficient, in the modular algorithm
    // - l.rem_limb_ref(p) == 0
    test("9223372036854775837*x+1", "x+2", Some("1"));
    // - a nonmonic GCD found with one prime when the bound is small
    // - WIDTH_MINUS_1 + unlucky >= bound
    test("2*x^2+3*x+1", "2*x^2+5*x+2", Some("2*x+1"));
    // - a nonmonic GCD checked by division after one prime
    // - WIDTH_MINUS_1 >= bits_small
    // - divides(&b, &res) && divides(&a, &res), the second
    test(
        "2592*x^9+3814*x^8+3703*x^7+3592*x^6+3481*x^5+3370*x^4+3259*x^3+3148*x^2+3037*x+1000",
        "1534*x^9+2359*x^8+2446*x^7+2533*x^6+2620*x^5+2707*x^4+2794*x^3+2881*x^2+2968*x+999",
        Some("2*x+1"),
    );
    // - an unlucky prime, in the modular algorithm
    // - h_len > n + 1
    test(
        "1180591620717411303424*x^2+x",
        "1180591620717411303424*x^2-10889035741470030947706557888840301805567*x-922337203685477590\
        7",
        Some("1180591620717411303424*x+1"),
    );
    // - a monic GCD with large coefficients, reconstructed from two primes
    // - g_is_one, the second
    test(
        "x^2+1180591620717411303427*x+3541774862152233910272",
        "x^2+1180591620717411303429*x+5902958103587056517120",
        Some("x+1180591620717411303424"),
    );
    // - a nonmonic GCD whose first reconstruction fails
    // - let Some(h_content) = h_content
    test(
        "3*x^3+1393796574908163946345982392040522594123785*x^2+55751862996326557853839295681620903\
        76495104*x+4181389724724491839037947176121567782371328",
        "3*x^3+1393796574908163946345982392040522594123791*x^2+83627794494489836780758943522431355\
        64742656*x+6968982874540819731729911960202612970618880",
        Some(
            "3*x^2+1393796574908163946345982392040522594123776*x+139379657490816394634598239204052\
            2594123776",
        ),
    );
    // - a nonmonic GCD whose reconstruction has a negative leading coefficient
    // - *xs.last().unwrap() < 0u32
    test(
        "63802943797675962784826454431515019369*x^2+191408831393027888354479363294545058108*x+3",
        "63802943797675962784826454431515019369*x^2+319014718988379813924132272157575096846*x+5",
        Some("63802943797675962784826454431515019369*x+1"),
    );
}

#[test]
fn gcd_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        verify(&p, &q);
    });

    integer_polynomial_pair_gen_var_7().test_properties(|(p, q)| {
        let g = verify(&p, &q);
        if g != IntegerPolynomial::ZERO {
            // The cofactors are coprime.
            assert_eq!((&p).div_exact(&g).gcd((&q).div_exact(&g)).len(), 1);
        }
    });

    integer_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // The GCD of multiples of r is r, normalized, times the GCD of the cofactors.
        assert_eq!(
            (&p * &r).gcd(&q * &r),
            p.gcd(q) * (&r).canonical_primitive_part() * IntegerPolynomial::from(r.content())
        );
    });

    integer_polynomial_gen().test_properties(|p| {
        let normalized = (&p).gcd(IntegerPolynomial::ZERO);
        assert_eq!((&p).gcd(&p), normalized);
        assert_eq!(normalized == p || normalized == -&p, true);
        if p != IntegerPolynomial::ZERO {
            assert_eq!((&p).gcd(IntegerPolynomial::one()), IntegerPolynomial::one());
        }
    });
}

// A polynomial with `len` coefficients of up to `bits` bits, all different and different for each
// `seed`, with every third negated: the fifth powers of `seed * 7919 + i + 1` reduced modulo
// $2^\text{bits}$.
fn generated(len: usize, bits: u64, seed: u64) -> IntegerPolynomial {
    let m = Natural::power_of_2(bits);
    IntegerPolynomial::from_coefficients_asc(
        (0..len)
            .map(|i| {
                let x = Integer::from(
                    Natural::from(seed * 7919 + u64::wrapping_from(i) + 1).pow(5) % &m,
                );
                if i % 3 == 1 { -x } else { x }
            })
            .collect(),
    )
}

// Long polynomials sharing a factor, with small coefficients and with large ones.
#[test]
fn test_gcd_algorithms() {
    for &(len, bits) in &[(70, 20), (80, 200)] {
        let c = generated(len / 3, bits, 1);
        let p = generated(len, bits, 2) * &c;
        let q = generated(len - 2, bits, 3) * &c;
        // - !(len1 < 64 && len2 < 64)
        let g = verify(&p, &q);
        assert!(g.divisible_by(&c.canonical_primitive_part()));
    }
}

// The primes that the multimodular algorithms work modulo, from the table and beyond it, are the
// consecutive primes after $2^{W-1}$.
#[test]
fn test_large_primes() {
    let primes = large_primes(300);
    let mut previous = Limb::power_of_2(Limb::WIDTH - 1);
    for p in primes {
        assert!(p.is_prime());
        assert!((previous + 1..p).all(|n| !n.is_prime()));
        previous = p;
    }
}
