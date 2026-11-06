// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_triple_natural_quadruple_gen_var_2,
    natural_polynomial_vec_natural_polynomial_pair_natural_quadruple_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::mod_polynomial::mod_compose_mod_polynomial_many::*;

fn verify(
    fs: &[NaturalPolynomial],
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> Vec<NaturalPolynomial> {
    let rs = fs.mod_compose_mod_polynomial(g, h, m);
    assert_eq!(rs.len(), fs.len());
    assert!(rs.iter().all(NaturalPolynomial::is_valid));
    assert_eq!(fs.mod_compose_mod_polynomial(g, h, m.clone()), rs);
    assert_eq!(fs.mod_compose_mod_polynomial(g, h.clone(), m), rs);
    assert_eq!(fs.mod_compose_mod_polynomial(g, h.clone(), m.clone()), rs);
    assert_eq!(fs.mod_compose_mod_polynomial(g.clone(), h, m), rs);
    assert_eq!(fs.mod_compose_mod_polynomial(g.clone(), h, m.clone()), rs);
    assert_eq!(fs.mod_compose_mod_polynomial(g.clone(), h.clone(), m), rs);
    assert_eq!(
        fs.mod_compose_mod_polynomial(g.clone(), h.clone(), m.clone()),
        rs
    );
    let data = NaturalPolynomial::precompute_mod_div_rem_data(h, m);
    assert_eq!(
        fs.mod_compose_mod_polynomial_precomputed(g, h, m, &data),
        rs
    );
    assert_eq!(
        fs.mod_compose_mod_polynomial_precomputed(g, h, m.clone(), &data),
        rs
    );
    assert_eq!(
        fs.mod_compose_mod_polynomial_precomputed(g, h.clone(), m, &data),
        rs
    );
    assert_eq!(
        fs.mod_compose_mod_polynomial_precomputed(g, h.clone(), m.clone(), &data),
        rs
    );
    assert_eq!(
        fs.mod_compose_mod_polynomial_precomputed(g.clone(), h, m, &data),
        rs
    );
    assert_eq!(
        fs.mod_compose_mod_polynomial_precomputed(g.clone(), h, m.clone(), &data),
        rs
    );
    assert_eq!(
        fs.mod_compose_mod_polynomial_precomputed(g.clone(), h.clone(), m, &data),
        rs
    );
    assert_eq!(
        fs.mod_compose_mod_polynomial_precomputed(g.clone(), h.clone(), m.clone(), &data),
        rs
    );
    assert_eq!(
        mod_compose_mod_polynomial_many_one_at_a_time(fs, g, h, m),
        rs
    );
    assert_eq!(mod_compose_mod_polynomial_many_naive(fs, g, h, m), rs);
    assert_eq!(mod_compose_mod_polynomial_many_brent_kung(fs, g, h, m), rs);
    rs
}

#[test]
fn test_mod_compose_mod_polynomial_many() {
    let test = |fs: &[&str], g: &str, h: &str, m: &str, out: &[&str]| {
        let fs: Vec<NaturalPolynomial> = fs
            .iter()
            .map(|f| NaturalPolynomial::from_str(f).unwrap())
            .collect();
        let g = NaturalPolynomial::from_str(g).unwrap();
        let h = NaturalPolynomial::from_str(h).unwrap();
        let m = Natural::from_str(m).unwrap();
        let rs = verify(&fs, &g, &h, &m);
        let rs: Vec<String> = rs.iter().map(ToString::to_string).collect();
        assert_eq!(rs, out);
    };
    // - m fits in a limb, with the data given, and computed
    // - lens.is_empty()
    // an empty batch
    test(&[], "x+1", "x^3+2*x^2+1", "7", &[]);
    // - trivial
    // m == 1
    test(&["0", "0"], "0", "0", "1", &["0", "0"]);
    // - compose_mod_special(ring, f, g, n).is_some()
    // h constant
    test(&["3*x^2+2*x+1", "5"], "0", "3", "7", &["0", "0"]);
    // h linear: composition is evaluation
    test(
        &["7*x^2+6*x+5", "x+1", "0"],
        "3",
        "x+4",
        "101",
        &["86", "4", "0"],
    );
    // only constants, so no matrix is needed
    test(
        &["5", "0", "2"],
        "x+1",
        "x^3+2*x^2+1",
        "7",
        &["5", "0", "2"],
    );
    // - fs.len() == 1
    test(&["3*x^2+2*x+1"], "x+1", "x^3+2*x^2+1", "7", &["3*x^2+x+6"]);
    // - the total length is less than BRENT_KUNG_VEC_CUTOFF
    // - compose_mod_special(ring, f, g, n).is_none()
    // - rows.len() == 1
    // - 1 < rows.len() <= 10
    // constants together with polynomials that need the matrix
    test(
        &["3*x^2+2*x+1", "5", "5*x^4+4*x^3+3*x^2+2*x+1"],
        "x+1",
        "x^3+2*x^2+1",
        "7",
        &["3*x^2+x+6", "5", "3*x^2+1"],
    );
    // - the total length is at least BRENT_KUNG_VEC_CUTOFF
    // Brent–Kung, with polynomials of several lengths
    test(
        &["8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1", "3*x^2+2*x+1", "5*x^4+6*x^3+7*x^2+8*x+9"],
        "5*x^4+6*x^3+7*x^2+8*x+9",
        "x^8+6*x^7+2*x^6+9*x^5+5*x^4+x^3+4*x^2+x+3",
        "11",
        &[
            "3*x^7+2*x^6+7*x^5+5*x^3+5*x^2+6*x+10",
            "5*x^7+3*x^6+4*x^5+10*x^4+3*x^3+9*x^2+10*x+4",
            "4*x^7+10*x^6+x^5+8*x^4+3*x^3+x^2+x+5",
        ],
    );
    // a large modulus
    test(
        &["18446744073709551556*x^2+1", "2*x+18446744073709551556"],
        "18446744073709551555*x+3",
        "x^3+5",
        "18446744073709551557",
        &["18446744073709551553*x^2+12*x+18446744073709551549", "18446744073709551553*x+5"],
    );
    // - m does not fit in a limb, with the data given, and computed
    test(
        &["3*x^2+2*x+1", "5", "5*x^4+4*x^3+3*x^2+2*x+1"],
        "170141183460469231731687303715884105726*x+3",
        "x^3+2*x^2+1",
        "170141183460469231731687303715884105727",
        &[
            "3*x^2+170141183460469231731687303715884105707*x+34",
            "5",
            "457*x^2+170141183460469231731687303715884105054*x+621",
        ],
    );
    // - rows.len() > 10
    // A polynomial with more than ten blocks, which are combined by rectangular splitting, and one
    // with a single block.
    test(
        &[
            "25*x^149+28*x^148+33*x^147+40*x^146+49*x^145+60*x^144+73*x^143+88*x^142+4*x^141+23*x^\
            140+44*x^139+67*x^138+92*x^137+18*x^136+47*x^135+78*x^134+10*x^133+45*x^132+82*x^131+2\
            0*x^130+61*x^129+3*x^128+48*x^127+95*x^126+43*x^125+94*x^124+46*x^123+57*x^121+15*x^12\
            0+76*x^119+38*x^118+2*x^117+69*x^116+37*x^115+7*x^114+80*x^113+54*x^112+30*x^111+8*x^1\
            10+89*x^109+71*x^108+55*x^107+41*x^106+29*x^105+19*x^104+11*x^103+5*x^102+x^101+100*x^\
            100+100*x^99+x^98+5*x^97+11*x^96+19*x^95+29*x^94+41*x^93+55*x^92+71*x^91+89*x^90+8*x^8\
            9+30*x^88+54*x^87+80*x^86+7*x^85+37*x^84+69*x^83+2*x^82+38*x^81+76*x^80+15*x^79+57*x^7\
            8+46*x^76+94*x^75+43*x^74+95*x^73+48*x^72+3*x^71+61*x^70+20*x^69+82*x^68+45*x^67+10*x^\
            66+78*x^65+47*x^64+18*x^63+92*x^62+67*x^61+44*x^60+23*x^59+4*x^58+88*x^57+73*x^56+60*x\
            ^55+49*x^54+40*x^53+33*x^52+28*x^51+25*x^50+24*x^49+25*x^48+28*x^47+33*x^46+40*x^45+49\
            *x^44+60*x^43+73*x^42+88*x^41+4*x^40+23*x^39+44*x^38+67*x^37+92*x^36+18*x^35+47*x^34+7\
            8*x^33+10*x^32+45*x^31+82*x^30+20*x^29+61*x^28+3*x^27+48*x^26+95*x^25+43*x^24+94*x^23+\
            46*x^22+57*x^20+15*x^19+76*x^18+38*x^17+2*x^16+69*x^15+37*x^14+7*x^13+80*x^12+54*x^11+\
            30*x^10+8*x^9+89*x^8+71*x^7+55*x^6+41*x^5+29*x^4+19*x^3+11*x^2+5*x+1",
            "7*x^3+x+5",
        ],
        "3*x^4+x^2+2",
        "x^6+5*x^5+x+9",
        "101",
        &["19*x^5+82*x^4+75*x^3+95*x^2+46*x+64", "51*x^5+33*x^4+71*x^3+91*x^2+47*x+54"],
    );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_many_fail_1() {
    // m is 0.
    [NaturalPolynomial::from_str("x+1").unwrap(), NaturalPolynomial::from_str("x").unwrap()]
        .as_slice()
        .mod_compose_mod_polynomial(
            &NaturalPolynomial::from_str("0").unwrap(),
            &NaturalPolynomial::from_str("x^2+1").unwrap(),
            &Natural::ZERO,
        );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_many_fail_2() {
    // a polynomial of the batch is not reduced.
    [NaturalPolynomial::from_str("x+1").unwrap(), NaturalPolynomial::from_str("7*x").unwrap()]
        .as_slice()
        .mod_compose_mod_polynomial(
            &NaturalPolynomial::from_str("0").unwrap(),
            &NaturalPolynomial::from_str("x^2+1").unwrap(),
            &Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_many_fail_3() {
    // g is not shorter than h.
    [NaturalPolynomial::from_str("x+1").unwrap(), NaturalPolynomial::from_str("x").unwrap()]
        .as_slice()
        .mod_compose_mod_polynomial(
            &NaturalPolynomial::from_str("x^2").unwrap(),
            &NaturalPolynomial::from_str("x^2+1").unwrap(),
            &Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_many_fail_4() {
    // h is zero.
    [NaturalPolynomial::from_str("x+1").unwrap(), NaturalPolynomial::from_str("x").unwrap()]
        .as_slice()
        .mod_compose_mod_polynomial(
            &NaturalPolynomial::from_str("0").unwrap(),
            &NaturalPolynomial::from_str("0").unwrap(),
            &Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_many_fail_5() {
    // the leading coefficient of h is not invertible.
    [NaturalPolynomial::from_str("x+1").unwrap(), NaturalPolynomial::from_str("x").unwrap()]
        .as_slice()
        .mod_compose_mod_polynomial(
            &NaturalPolynomial::from_str("0").unwrap(),
            &NaturalPolynomial::from_str("2*x^2+1").unwrap(),
            &Natural::from(6u32),
        );
}

#[test]
fn mod_compose_mod_polynomial_many_properties() {
    natural_polynomial_vec_natural_polynomial_pair_natural_quadruple_gen_var_1().test_properties(
        |(fs, g, h, m)| {
            let rs = verify(&fs, &g, &h, &m);
            // Every result is reduced modulo h.
            if m != 1u32 {
                assert!(rs.iter().all(|r| r.len() < h.len()));
            }
            // Splitting the batch does not change the results.
            let mid = fs.len() >> 1;
            let mut split = fs[..mid].mod_compose_mod_polynomial(&g, &h, &m);
            split.extend(fs[mid..].mod_compose_mod_polynomial(&g, &h, &m));
            assert_eq!(split, rs);

            // Modulo a power of 2, this agrees with mod_power_of_2_compose_mod_polynomial.
            if m.is_power_of_2() {
                assert_eq!(
                    fs.as_slice().mod_power_of_2_compose_mod_polynomial(
                        &g,
                        &h,
                        m.significant_bits() - 1
                    ),
                    rs
                );
            }
        },
    );

    natural_polynomial_triple_natural_quadruple_gen_var_2().test_properties(|(f, g, h, m)| {
        // A batch of one polynomial is composed as that polynomial is.
        assert_eq!(
            [f.clone()].mod_compose_mod_polynomial(&g, &h, &m),
            [f.mod_compose_mod_polynomial(&g, &h, &m)]
        );
    });

    unsigned_polynomial_vec_unsigned_polynomial_pair_unsigned_quadruple_gen_var_1::<u64>()
        .test_properties(|(fs, g, h, m)| {
            // For a word-sized modulus, this agrees with the `UnsignedPolynomial` version.
            let nfs: Vec<NaturalPolynomial> = fs.iter().map(NaturalPolynomial::from).collect();
            assert_eq!(
                nfs.as_slice().mod_compose_mod_polynomial(
                    &NaturalPolynomial::from(&g),
                    &NaturalPolynomial::from(&h),
                    &Natural::from(m)
                ),
                fs.as_slice()
                    .mod_compose_mod_polynomial(&g, &h, m)
                    .iter()
                    .map(NaturalPolynomial::from)
                    .collect::<Vec<_>>()
            );
        });
}
