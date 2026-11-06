// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial_many::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn verify<T: PrimitiveUnsigned>(
    fs: &[UnsignedPolynomial<T>],
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    pow: u64,
) -> Vec<UnsignedPolynomial<T>> {
    let rs = fs.mod_power_of_2_compose_mod_polynomial(g, h, pow);
    assert_eq!(rs.len(), fs.len());
    assert!(rs.iter().all(UnsignedPolynomial::is_valid));
    assert_eq!(
        fs.mod_power_of_2_compose_mod_polynomial(g.clone(), h, pow),
        rs
    );
    assert_eq!(
        fs.mod_power_of_2_compose_mod_polynomial(g, h.clone(), pow),
        rs
    );
    assert_eq!(
        fs.mod_power_of_2_compose_mod_polynomial(g.clone(), h.clone(), pow),
        rs
    );
    let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(h, pow);
    assert_eq!(
        fs.mod_power_of_2_compose_mod_polynomial_precomputed(g, h, pow, &data),
        rs
    );
    assert_eq!(
        fs.mod_power_of_2_compose_mod_polynomial_precomputed(g.clone(), h, pow, &data),
        rs
    );
    assert_eq!(
        fs.mod_power_of_2_compose_mod_polynomial_precomputed(g, h.clone(), pow, &data),
        rs
    );
    assert_eq!(
        fs.mod_power_of_2_compose_mod_polynomial_precomputed(g.clone(), h.clone(), pow, &data),
        rs
    );
    assert_eq!(
        mod_power_of_2_compose_mod_polynomial_many_one_at_a_time(fs, g, h, pow),
        rs
    );
    assert_eq!(
        mod_power_of_2_compose_mod_polynomial_many_naive(fs, g, h, pow),
        rs
    );
    assert_eq!(
        mod_power_of_2_compose_mod_polynomial_many_brent_kung(fs, g, h, pow),
        rs
    );
    rs
}

#[test]
fn test_mod_power_of_2_compose_mod_polynomial_many() {
    fn test<T: PrimitiveUnsigned>(fs: &[&str], g: &str, h: &str, pow: u64, out: &[&str]) {
        let fs: Vec<UnsignedPolynomial<T>> = fs
            .iter()
            .map(|f| UnsignedPolynomial::from_str(f).unwrap())
            .collect();
        let g = UnsignedPolynomial::<T>::from_str(g).unwrap();
        let h = UnsignedPolynomial::<T>::from_str(h).unwrap();
        let rs = verify(&fs, &g, &h, pow);
        let rs: Vec<String> = rs.iter().map(ToString::to_string).collect();
        assert_eq!(rs, out);
    }
    // - the data is given, and computed
    // - lens.is_empty()
    // an empty batch
    test::<u8>(&[], "x+1", "x^3+2*x^2+1", 3, &[]);
    // - trivial
    // pow == 0
    test::<u8>(&["0", "0"], "0", "0", 0, &["0", "0"]);
    // - compose_mod_special(ring, f, g, n).is_some()
    // h constant
    test::<u8>(&["3*x^2+2*x+1", "5"], "0", "3", 3, &["0", "0"]);
    // h linear: composition is evaluation
    test::<u8>(
        &["7*x^2+6*x+5", "x+1", "0"],
        "3",
        "x+5",
        7,
        &["86", "4", "0"],
    );
    // only constants, so no matrix is needed
    test::<u8>(&["5", "0", "2"], "x+1", "x^3+2*x^2+1", 3, &["5", "0", "2"]);
    // - fs.len() == 1
    test::<u8>(&["3*x^2+2*x+1"], "x+1", "x^3+2*x^2+1", 3, &["3*x^2+6"]);
    // - the total length is less than BRENT_KUNG_VEC_CUTOFF
    // - compose_mod_special(ring, f, g, n).is_none()
    // - rows.len() == 1
    // - 1 < rows.len() <= 10
    // constants together with polynomials that need the matrix
    test::<u8>(
        &["3*x^2+2*x+1", "5", "5*x^4+4*x^3+3*x^2+2*x+1"],
        "x+1",
        "x^3+2*x^2+1",
        3,
        &["3*x^2+6", "5", "x^2+3*x+1"],
    );
    // - the total length is at least BRENT_KUNG_VEC_CUTOFF
    // Brent–Kung, with polynomials of several lengths
    test::<u8>(
        &["8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1", "3*x^2+2*x+1", "5*x^4+6*x^3+7*x^2+8*x+9"],
        "5*x^4+6*x^3+7*x^2+8*x+9",
        "x^8+6*x^7+2*x^6+9*x^5+5*x^4+x^3+4*x^2+x+3",
        4,
        &[
            "6*x^7+13*x^6+2*x^5+x^4+10*x^3+15*x^2+4*x+9",
            "2*x^7+8*x^6+9*x^5+4*x^4+5*x^3+12*x^2+5*x+5",
            "9*x^7+2*x^6+2*x^5+4*x^3+8*x^2+15*x+15",
        ],
    );
    // pow == 64
    test::<u64>(
        &["18446744073709551615*x^2+1", "2*x+18446744073709551615"],
        "18446744073709551613*x+3",
        "x^3+5",
        64,
        &["18446744073709551607*x^2+18*x+18446744073709551608", "18446744073709551610*x+5"],
    );
    // - rows.len() > 10
    // A polynomial with more than ten blocks, which are combined by rectangular splitting, and one
    // with a single block.
    test::<u64>(
        &[
            "121*x^149+77*x^148+35*x^147+123*x^146+85*x^145+49*x^144+15*x^143+111*x^142+81*x^141+5\
            3*x^140+27*x^139+3*x^138+109*x^137+89*x^136+71*x^135+55*x^134+41*x^133+29*x^132+19*x^1\
            31+11*x^130+5*x^129+x^128+127*x^127+127*x^126+x^125+5*x^124+11*x^123+19*x^122+29*x^121\
            +41*x^120+55*x^119+71*x^118+89*x^117+109*x^116+3*x^115+27*x^114+53*x^113+81*x^112+111*\
            x^111+15*x^110+49*x^109+85*x^108+123*x^107+35*x^106+77*x^105+121*x^104+39*x^103+87*x^1\
            02+9*x^101+61*x^100+115*x^99+43*x^98+101*x^97+33*x^96+95*x^95+31*x^94+97*x^93+37*x^92+\
            107*x^91+51*x^90+125*x^89+73*x^88+23*x^87+103*x^86+57*x^85+13*x^84+99*x^83+59*x^82+21*\
            x^81+113*x^80+79*x^79+47*x^78+17*x^77+117*x^76+91*x^75+67*x^74+45*x^73+25*x^72+7*x^71+\
            119*x^70+105*x^69+93*x^68+83*x^67+75*x^66+69*x^65+65*x^64+63*x^63+63*x^62+65*x^61+69*x\
            ^60+75*x^59+83*x^58+93*x^57+105*x^56+119*x^55+7*x^54+25*x^53+45*x^52+67*x^51+91*x^50+1\
            17*x^49+17*x^48+47*x^47+79*x^46+113*x^45+21*x^44+59*x^43+99*x^42+13*x^41+57*x^40+103*x\
            ^39+23*x^38+73*x^37+125*x^36+51*x^35+107*x^34+37*x^33+97*x^32+31*x^31+95*x^30+33*x^29+\
            101*x^28+43*x^27+115*x^26+61*x^25+9*x^24+87*x^23+39*x^22+121*x^21+77*x^20+35*x^19+123*\
            x^18+85*x^17+49*x^16+15*x^15+111*x^14+81*x^13+53*x^12+27*x^11+3*x^10+109*x^9+89*x^8+71\
            *x^7+55*x^6+41*x^5+29*x^4+19*x^3+11*x^2+5*x+1",
            "7*x^3+x+5",
        ],
        "3*x^4+x^2+2",
        "x^6+5*x^5+x+9",
        7,
        &["86*x^5+71*x^4+37*x^3+67*x^2+39*x+71", "24*x^5+97*x^4+47*x^3+38*x^2+x+68"],
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_mod_polynomial_many_fail_1() {
    // pow is greater than the width of the type.
    [
        UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
        UnsignedPolynomial::<u8>::from_str("x").unwrap(),
    ]
    .as_slice()
    .mod_power_of_2_compose_mod_polynomial(
        &UnsignedPolynomial::<u8>::from_str("0").unwrap(),
        &UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap(),
        9,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_mod_polynomial_many_fail_2() {
    // a polynomial of the batch is not reduced.
    [
        UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
        UnsignedPolynomial::<u8>::from_str("8*x").unwrap(),
    ]
    .as_slice()
    .mod_power_of_2_compose_mod_polynomial(
        &UnsignedPolynomial::<u8>::from_str("0").unwrap(),
        &UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_mod_polynomial_many_fail_3() {
    // g is not shorter than h.
    [
        UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
        UnsignedPolynomial::<u8>::from_str("x").unwrap(),
    ]
    .as_slice()
    .mod_power_of_2_compose_mod_polynomial(
        &UnsignedPolynomial::<u8>::from_str("x^2").unwrap(),
        &UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_mod_polynomial_many_fail_4() {
    // h is zero.
    [
        UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
        UnsignedPolynomial::<u8>::from_str("x").unwrap(),
    ]
    .as_slice()
    .mod_power_of_2_compose_mod_polynomial(
        &UnsignedPolynomial::<u8>::from_str("0").unwrap(),
        &UnsignedPolynomial::<u8>::from_str("0").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_mod_polynomial_many_fail_5() {
    // the leading coefficient of h is even.
    [
        UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
        UnsignedPolynomial::<u8>::from_str("x").unwrap(),
    ]
    .as_slice()
    .mod_power_of_2_compose_mod_polynomial(
        &UnsignedPolynomial::<u8>::from_str("0").unwrap(),
        &UnsignedPolynomial::<u8>::from_str("2*x^2+1").unwrap(),
        3,
    );
}

fn mod_power_of_2_compose_mod_polynomial_many_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_vec_unsigned_polynomial_pair_unsigned_quadruple_gen_var_2::<T>()
        .test_properties(|(fs, g, h, pow)| {
            let rs = verify(&fs, &g, &h, pow);
            // Every result is reduced modulo h.
            if pow != 0 {
                assert!(rs.iter().all(|r| r.len() < h.len()));
            }
            // Splitting the batch does not change the results.
            let mid = fs.len() >> 1;
            let mut split = fs[..mid].mod_power_of_2_compose_mod_polynomial(&g, &h, pow);
            split.extend(fs[mid..].mod_power_of_2_compose_mod_polynomial(&g, &h, pow));
            assert_eq!(split, rs);

            // Below the width of `T`, this agrees with mod_compose_mod_polynomial modulo 2^pow.
            if pow < T::WIDTH {
                assert_eq!(
                    fs.as_slice()
                        .mod_compose_mod_polynomial(&g, &h, T::power_of_2(pow)),
                    rs
                );
            }
        });

    unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>().test_properties(
        |(f, g, h, pow)| {
            // A batch of one polynomial is composed as that polynomial is.
            assert_eq!(
                [f.clone()].mod_power_of_2_compose_mod_polynomial(&g, &h, pow),
                [f.mod_power_of_2_compose_mod_polynomial(&g, &h, pow)]
            );
        },
    );
}

#[test]
fn mod_power_of_2_compose_mod_polynomial_many_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_compose_mod_polynomial_many_properties_helper);
}
