// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::{Compose, Evaluate, FromRoots};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_vec_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::from_roots::from_roots_naive;

#[test]
fn test_from_roots() {
    let test = |roots: &[&str], out| {
        let roots: Vec<Integer> = roots
            .iter()
            .map(|x| Integer::from_str(x).unwrap())
            .collect();
        let p = IntegerPolynomial::from_roots(&roots);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(from_roots_naive(&roots).to_string(), out);
    };
    // - no roots
    test(&[], "1");
    // - one root
    test(&["5"], "x-5");
    // - fewer roots than the basecase cutoff
    test(&["-13", "14", "-10"], "x^3+9*x^2-192*x-1820");
    // - large roots
    test(
        &[
            "931153874795101477622",
            "-375964224561368704309",
            "-280791744750947638716",
            "-974989684287071079371",
            "-441798763636081541240",
        ],
        "x^5+1142390542440367486014*x^4-463987760587809920955905398327291810478033*x^3-93335342136\
        0426030055456467739652420096276208014883047299993614*x^2-357217493850607456801953346839929\
        264980237614686504111473884085565217399023017824488*x-423425323623016244486814239213163128\
        52855103223912067507060339805078881146881770404321949803956821030720",
    );
    // - an odd number of roots, split into two products built one factor at a time
    test(
        &[
            "0", "-8", "8", "11", "-12", "3", "-4", "0", "-10", "-10", "-13", "-15", "-6", "0",
            "-14", "-4", "-8", "-7", "3", "0", "-5",
        ],
        "x^21+91*x^20+3437*x^19+65571*x^18+507823*x^17-4031311*x^16-132894521*x^15-1194422695*x^14\
        -1723838036*x^13+56806420776*x^12+516925778864*x^11+1601197167184*x^10-2459083427648*x^9-3\
        2223285501696*x^8-71941164272640*x^7+65054212300800*x^6+488365645824000*x^5+55794106368000\
        0*x^4",
    );
    // - 25 roots
    test(
        &[
            "-57", "-711", "-413", "-681", "-87", "201", "-227", "-965", "-264", "-623", "-345",
            "-183", "-178", "828", "696", "-138", "1019", "960", "656", "804", "34", "-232", "304",
            "-462", "149",
        ],
        "x^25-85*x^24-3708710*x^23-219754018*x^22+5713710265927*x^21+1176899588623277*x^20-4649353\
        857644393788*x^19-1660179199275607224308*x^18+2058031494744636533758127*x^17+1105957712963\
        901767498503237*x^16-425602252823277284140873513078*x^15-377380671258231572589942038993618\
        *x^14+2179297424441097785741387658479097*x^13+60558918037734413072705982642524345427*x^12+\
        12976481162113998044498469554706270307512*x^11-2937176288517207135803294158076810087634168\
        *x^10-1459250512073589119787658002067144702040055888*x^9-962718266089621301560628168646283\
        38482257087248*x^8+41802558746226210209516904780661840743283115094912*x^7+8418629756571696\
        270267631628067143329881682959328512*x^6+1482932802603296836423189134520855656216720003020\
        94336*x^5-107129357241556708958247341840403946524428256453200068608*x^4-115647688368718767\
        53514555199412971277398930339119937486848*x^3-31294246318249420058301427688651231575072740\
        6764299282022400*x^2+11505049981338008080440712236688028775550674693070707385958400*x+5466\
        19296219611346710713321741705587465445586641748999798784000",
    );
}

#[test]
fn from_roots_properties() {
    let test = |xs: Vec<Integer>| {
        let p = IntegerPolynomial::from_roots(&xs);
        assert!(p.is_valid());
        assert_eq!(from_roots_naive(&xs), p);
        assert_eq!(p.len(), u64::try_from(xs.len()).unwrap() + 1);
        assert_eq!(*p.leading_coefficient(), 1u32);
        for x in &xs {
            assert_eq!((&p).evaluate(x), 0u32);
        }
        let mut ys = xs.clone();
        ys.reverse();
        assert_eq!(IntegerPolynomial::from_roots(&ys), p);
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        assert_eq!(
            IntegerPolynomial::from_roots(xs_1) * IntegerPolynomial::from_roots(xs_2),
            p
        );
        // Negating every root substitutes -x for x, up to the sign that keeps the result monic.
        let negated: Vec<Integer> = xs.iter().map(|x| -x).collect();
        let substituted = (&p).compose(&IntegerPolynomial::from_str("-x").unwrap());
        assert_eq!(
            IntegerPolynomial::from_roots(&negated),
            if xs.len().is_multiple_of(2) {
                substituted
            } else {
                -substituted
            }
        );
    };
    integer_vec_gen().test_properties(test);

    // Long enough to split into products of the halves.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    integer_vec_gen().test_properties_with_config(&config, test);
}
