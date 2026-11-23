// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::oracle_row;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::integer::Integer;
use malachite_nz::integer::number_theory::euler_number::{
    euler_numbers_prefix, even_euler_numbers_multi_mod,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::zeta_helpers::{
    dirichlet_beta_enclosure, pi_enclosure, zeta_enclosure,
};
use malachite_nz::test_util::integer::number_theory::euler_number::euler_numbers_prefix_naive;

#[test]
fn test_euler_number() {
    let test = |n, out| {
        oracle_row!("flint", "arith_euler_number", "euler_number({n}) = {out}");

        assert_eq!(Integer::euler_number(n).to_string(), out);
    };
    // - n < EULER_NUMBER_BETA_LIMIT
    test(0, "1");
    // - n.odd()
    test(1, "0");
    test(2, "-1");
    test(3, "0");
    test(4, "5");
    test(6, "-61");
    test(8, "1385");
    test(10, "-50521");
    test(34, "-80723299235887898062168247453281");
    test(35, "0");
    // - n >= EULER_NUMBER_BETA_LIMIT
    // - in euler_number_abs: x.unique_integer() is Some
    // - in pi_bounds: 47 * terms < w + 60 + (terms + 1).significant_bits()
    // - in chudnovsky: b - a == 1
    // - in chudnovsky: b - a != 1
    // - in chudnovsky: a == 0
    // - in chudnovsky: a != 0
    // - in chudnovsky: a.odd()
    // - in chudnovsky: a.even()
    // - in dirichlet_beta_bounds: p.mod_power_of_2(2) == 1
    // - in dirichlet_beta_bounds: p.mod_power_of_2(2) == 3
    // - in euler_product_limit: (s - 1) * ... < w + 2
    // - in Bounds::normalize: excess != 0
    // - in Bounds::normalize: excess == 0
    // - in Bounds::div: shift != 0
    // - in Bounds::pow: n.odd()
    // - in Bounds::pow: n.even()
    // - in Bounds::unique_integer: self.exp < 0
    test(36, "41222060339517702122347079671259045");
    test(38, "-23489580527043108252017828576198947741");
    test(
        100,
        "29035283466610974970546038347644358750775530066461589450804923191469976433706250238893534\
        47129967354174648294748510553528692457632980625125",
    );
    test(
        300,
        "56852563686050582715789855530042087626546497108957923947456306633037454242223164692660463\
        310486845949879941738638619394532581064314311727579362078288887093936753672105014711780188\
        473923202164711381764187934127638590727240305611085738040798981873907165544254238668046181\
        552827686778704604664787163688922140571977298415638208962689185051347676391981219529503486\
        862975224833150672917063747203773810477861750732559828745451428651485470843218671125038534\
        016849393822470167087747835323493170554257982219346317712704455658384440405059351562509469\
        05537050798594125",
    );
}

#[test]
fn test_euler_numbers_prefix() {
    // in even_euler_numbers_multi_mod: len == 0
    assert!(euler_numbers_prefix(0).is_empty());
    assert_eq!(
        euler_numbers_prefix(12)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["1", "0", "-1", "0", "5", "0", "-61", "0", "1385", "0", "-50521", "0"]
    );
    // - long enough for the entries to need different numbers of primes
    assert_eq!(euler_numbers_prefix(301), euler_numbers_prefix_naive(301));
    #[cfg(feature = "oracle")]
    for len in [0, 12, 301] {
        oracle_row!(
            "flint",
            "arith_euler_number_vec",
            "euler_numbers_prefix({len}) = {:?}",
            euler_numbers_prefix(len)
        );
    }
}

// Checks that [lo * 2^exp, hi * 2^exp] contains the value whose floor times 2^400 is `reference`,
// and that the enclosure has a relative width of at most about 2^-bits.
fn check_enclosure((lo, hi, exp): (Natural, Natural, i64), bits: u64, reference: &str) {
    let reference = Natural::from_str(reference).unwrap();
    let shift = exp + 400;
    // compare x * 2^shift with y, exactly
    let scale = |x: &Natural, y: &Natural| {
        if shift >= 0 {
            (x << shift, y.clone())
        } else {
            (x.clone(), y << -shift)
        }
    };
    let (a, b) = scale(&lo, &(&reference + Natural::ONE));
    assert!(a <= b, "the lower end is too high");
    let (a, b) = scale(&hi, &reference);
    assert!(a >= b, "the upper end is too low");
    assert!(lo <= hi);
    assert!(
        (&hi - &lo) << (bits.saturating_sub(4)) <= lo,
        "the enclosure is too wide"
    );
}

fn test_pi(bits: u64) {
    check_enclosure(
        pi_enclosure(bits),
        bits,
        "81123772467309712388622713513008945191611516001792652035911910376152309053739904987455233\
        60926816815427566779727327576236",
    );
}

#[test]
fn test_pi_enclosure() {
    // in Bounds::div: shift == 0
    test_pi(1);
    test_pi(10);
    test_pi(64);
    test_pi(100);
    test_pi(300);
}

#[test]
fn test_zeta_enclosure() {
    check_enclosure(
        zeta_enclosure(36, 10),
        10,
        "25822498781244852881849925520974526455592938000048057256432996970774791117224606044871377\
        06801863967226715008468007270770",
    );
    check_enclosure(
        zeta_enclosure(36, 64),
        64,
        "25822498781244852881849925520974526455592938000048057256432996970774791117224606044871377\
        06801863967226715008468007270770",
    );
    check_enclosure(
        zeta_enclosure(40, 200),
        200,
        "25822498780892571324510919940707536878444710413405404926421807316473509811201832776449575\
        51705718528685837935524666512620",
    );
    check_enclosure(
        zeta_enclosure(100, 64),
        64,
        "25822498780869085896559191720050489103060402789205023630753638246798506205089181435948620\
        82285154931421285569677742735889",
    );
    check_enclosure(
        zeta_enclosure(100, 300),
        300,
        "25822498780869085896559191720050489103060402789205023630753638246798506205089181435948620\
        82285154931421285569677742735889",
    );
}

#[test]
fn test_dirichlet_beta_enclosure() {
    check_enclosure(
        dirichlet_beta_enclosure(37, 10),
        10,
        "25822498780869085839212041881381056004340195848084348025832381907141847132704765854211038\
        91732441096193608684652222799836",
    );
    check_enclosure(
        dirichlet_beta_enclosure(37, 64),
        64,
        "25822498780869085839212041881381056004340195848084348025832381907141847132704765854211038\
        91732441096193608684652222799836",
    );
    check_enclosure(
        dirichlet_beta_enclosure(41, 200),
        200,
        "25822498780869085895851202212035622737347864280064751425748707876922648575730399606858489\
        69795737509910664287338037621722",
    );
    check_enclosure(
        dirichlet_beta_enclosure(101, 64),
        64,
        "25822498780869085896559191720030118743297057928275533779785615288606120395616297763439590\
        05988693428573795055627576245584",
    );
    check_enclosure(
        dirichlet_beta_enclosure(101, 300),
        300,
        "25822498780869085896559191720030118743297057928275533779785615288606120395616297763439590\
        05988693428573795055627576245584",
    );
}

#[test]
fn euler_number_properties() {
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        let e = Integer::euler_number(n);
        assert_eq!(e, euler_numbers_prefix_naive(n + 1).pop().unwrap());
        if n.odd() {
            assert_eq!(e, 0u32);
        } else {
            // E_n is odd, and its sign is (-1)^(n / 2)
            assert!(e.odd());
            assert_eq!(e > 0u32, n % 4 == 0);
        }
    });
}

#[test]
fn euler_numbers_prefix_properties() {
    unsigned_gen_var_5::<u8>().test_properties(|len| {
        let len = u64::from(len);
        let prefix = euler_numbers_prefix(len);
        assert_eq!(u64::exact_from(prefix.len()), len);
        assert_eq!(prefix, euler_numbers_prefix_naive(len));
        let evens = even_euler_numbers_multi_mod(usize::exact_from(len.div_ceil(2)));
        assert!(prefix.iter().step_by(2).eq(evens.iter()));
        assert!(prefix.iter().skip(1).step_by(2).all(|x| *x == 0u32));
        if let Some(last) = prefix.last() {
            assert_eq!(*last, Integer::euler_number(len - 1));
        }
    });
}
