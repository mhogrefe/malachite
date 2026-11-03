// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Content;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, FromRationalRoots};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_vec_gen;
use malachite_q::test_util::rational_polynomial::arithmetic::from_rational_roots::*;

#[test]
fn test_from_rational_roots() {
    let test = |roots: &[&str], out| {
        let roots: Vec<Rational> = roots
            .iter()
            .map(|x| Rational::from_str(x).unwrap())
            .collect();
        let p = IntegerPolynomial::from_rational_roots(&roots);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(from_rational_roots_naive(&roots).to_string(), out);
    };
    // - no roots
    test(&[], "1");
    // - one root
    test(&["5/6"], "6*x-5");
    // - fewer roots than the basecase cutoff
    test(&["4", "13/7", "13/6"], "42*x^3-337*x^2+845*x-676");
    // - large roots
    test(
        &[
            "-93597248250315879572/57255296359",
            "-77194837342498124967/149323327150",
            "224880542900256698040/41034731659",
            "163093171506827785506/552241489379",
        ],
        "193742078440932007276730952498091508875102850*x^4-702097808312108463954387222082084137934\
        495436604079667*x^3-1930393884667245163277318797779883859379221636712518955284495778*x^2-2\
        70940551074519009917655977240904301559830131842227827806553516194423416*x+2649958034060694\
        47865171978644427034545971441244280084576627991922218082421181760",
    );
    // - an odd number of roots, split into two products built one factor at a time
    test(
        &[
            "-1", "7/4", "3/2", "-1", "-4", "5", "-6", "0", "5/3", "0", "0", "3/4", "-1/4", "-1",
            "3/2", "0", "-1", "7/4", "4", "3/4", "-4",
        ],
        "12288*x^21-5120*x^20-782848*x^19+883840*x^18+16317424*x^17-28616932*x^16-126460544*x^15+3\
        10701647*x^14+251329078*x^13-1020506041*x^12-44311738*x^11+1517116501*x^10-330447910*x^9-1\
        116254607*x^8+327090330*x^7+373271112*x^6-94258080*x^5-38102400*x^4",
    );
    // - 22 roots
    test(
        &[
            "16/5", "11", "-5/3", "-7/3", "10/7", "1/5", "1", "8", "5", "2/3", "27/8", "1/10",
            "-23/7", "2", "5/2", "-11/4", "-11/8", "-13/14", "1/2", "25/8", "-9/4", "25/2",
        ],
        "303464448000*x^22-12140384256000*x^21+175159482839040*x^20-931442145254912*x^19-156053962\
        8561408*x^18+34091943293403872*x^17-72400862156590544*x^16-370142919869848128*x^15+1518462\
        287385186344*x^14+1254059992772249470*x^13-12049516475597688609*x^12+4286174973181451843*x\
        ^11+46824676929291864973*x^10-44329561038536639937*x^9-87278825089014743257*x^8+1243812811\
        82384449957*x^7+55764880674275086911*x^6-141760359408360506665*x^5+25881753231103959550*x^\
        4+49458696302574334500*x^3-28301089383950185000*x^2+5030924428035000000*x-2707781076000000\
        00",
    );
}

#[test]
fn from_rational_roots_properties() {
    let test = |xs: Vec<Rational>| {
        let p = IntegerPolynomial::from_rational_roots(&xs);
        assert!(p.is_valid());
        assert_eq!(from_rational_roots_naive(&xs), p);
        assert_eq!(p.len(), u64::try_from(xs.len()).unwrap() + 1);
        assert!(*p.leading_coefficient() > 0u32);
        assert_eq!((&p).content(), 1u32);
        let q = RationalPolynomial::from(p.clone());
        for x in &xs {
            assert_eq!((&q).evaluate(x), 0u32);
        }
        let mut ys = xs.clone();
        ys.reverse();
        assert_eq!(IntegerPolynomial::from_rational_roots(&ys), p);
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        assert_eq!(
            IntegerPolynomial::from_rational_roots(xs_1)
                * IntegerPolynomial::from_rational_roots(xs_2),
            p
        );
    };
    rational_vec_gen().test_properties(test);

    // Long enough to split into products of the halves.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    rational_vec_gen().test_properties_with_config(&config, test);
}
