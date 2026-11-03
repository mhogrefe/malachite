// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::{
    Evaluate, FromRationalRoots, FromRoots, MakeMonic,
};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_vec_gen;
use malachite_q::test_util::rational_polynomial::arithmetic::from_roots::from_roots_naive;

#[test]
fn test_from_roots() {
    let test = |roots: &[&str], out| {
        let roots: Vec<Rational> = roots
            .iter()
            .map(|x| Rational::from_str(x).unwrap())
            .collect();
        let p = RationalPolynomial::from_roots(&roots);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(from_roots_naive(&roots).to_string(), out);
    };
    // - no roots
    test(&[], "1");
    // - one root
    test(&["5/6"], "x-5/6");
    // - fewer roots than the basecase cutoff
    test(&["4", "13/7", "13/6"], "x^3-337/42*x^2+845/42*x-338/21");
    // - large roots
    test(
        &[
            "-93597248250315879572/57255296359",
            "-77194837342498124967/149323327150",
            "224880542900256698040/41034731659",
            "163093171506827785506/552241489379",
        ],
        "x^4-702097808312108463954387222082084137934495436604079667/193742078440932007276730952498\
        091508875102850*x^3-965196942333622581638659398889941929689610818356259477642247889/968710\
        39220466003638365476249045754437551425*x^2-13547027553725950495882798862045215077991506592\
        1113913903276758097211708/96871039220466003638365476249045754437551425*x+95562857340811196\
        48942372111230690030507444689660298758623440026044647761312/698673200291857220615690416509\
        5258163545",
    );
    // - an odd number of roots, split into two products built one factor at a time
    test(
        &[
            "-1", "7/4", "3/2", "-1", "-4", "5", "-6", "0", "5/3", "0", "0", "3/4", "-1/4", "-1",
            "3/2", "0", "-1", "7/4", "4", "3/4", "-4",
        ],
        "x^21-5/12*x^20-1529/24*x^19+6905/96*x^18+1019839/768*x^17-7154233/3072*x^16-987973/96*x^1\
        5+310701647/12288*x^14+125664539/6144*x^13-1020506041/12288*x^12-22155869/6144*x^11+151711\
        6501/12288*x^10-165223955/6144*x^9-372084869/4096*x^8+54515055/2048*x^7+15552963/512*x^6-9\
        81855/128*x^5-99225/32*x^4",
    );
    // - 22 roots
    test(
        &[
            "16/5", "11", "-5/3", "-7/3", "10/7", "1/5", "1", "8", "5", "2/3", "27/8", "1/10",
            "-23/7", "2", "5/2", "-11/4", "-11/8", "-13/14", "1/2", "25/8", "-9/4", "25/2",
        ],
        "x^22-6721/168*x^21+407271863/705600*x^20-1819222939951/592704000*x^19-507988160339/987840\
        00*x^18+1065373227918871/9483264000*x^17-4525053884786909/18966528000*x^16-642609235885153\
        /526848000*x^15+189807785923148293/37933056000*x^14+125405999277224947/30346444800*x^13-40\
        16505491865896203/101154816000*x^12+4286174973181451843/303464448000*x^11+4682467692929186\
        4973/303464448000*x^10-14776520346178879979/101154816000*x^9-87278825089014743257/30346444\
        8000*x^8+124381281182384449957/303464448000*x^7+2065365950899077293/11239424000*x^6-283520\
        71881672101333/60692889600*x^5+517635064622079191/6069288960*x^4+32972464201716223/2023096\
        32*x^3-28301089383950185/303464448*x^2+69873950389375/4214784*x-11192878125/12544",
    );
}

#[test]
fn from_roots_properties() {
    let test = |xs: Vec<Rational>| {
        let p = RationalPolynomial::from_roots(&xs);
        assert!(p.is_valid());
        assert_eq!(from_roots_naive(&xs), p);
        assert_eq!(p.len(), u64::try_from(xs.len()).unwrap() + 1);
        assert_eq!(p.leading_coefficient(), 1u32);
        for x in &xs {
            assert_eq!((&p).evaluate(x), 0u32);
        }
        let mut ys = xs.clone();
        ys.reverse();
        assert_eq!(RationalPolynomial::from_roots(&ys), p);
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        assert_eq!(
            RationalPolynomial::from_roots(xs_1) * RationalPolynomial::from_roots(xs_2),
            p
        );
        // The monic product is the primitive one, made monic.
        assert_eq!(
            RationalPolynomial::from(IntegerPolynomial::from_rational_roots(&xs)).make_monic(),
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
