// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Two;
use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime};
use malachite_base::test_util::generators::unsigned_gen_var_29;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_8, natural_pair_gen};
use malachite_nz::test_util::natural::factorization::is_lucas_probable_prime::*;
use malachite_nz::test_util::natural::factorization::is_strong_probable_prime::*;
use std::str::FromStr;

fn bpsw_naive(n: &Natural) -> bool {
    natural_is_strong_probable_prime_naive(n, &Natural::TWO)
        && natural_is_lucas_probable_prime_naive(n)
}

fn rug_is_probable_prime(n: &Natural) -> bool {
    rug::Integer::from(n).is_probably_prime(24) != rug::integer::IsPrime::No
}

#[test]
fn test_is_probable_prime() {
    for n in 0..10000u32 {
        assert_eq!(Natural::from(n).is_probable_prime(), n.is_prime());
    }
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.is_probable_prime(), out);
        assert_eq!(bpsw_naive(&n), out);
        assert_eq!(rug_is_probable_prime(&n), out);
    };
    // - n fits in a u64 in natural_is_probable_prime
    test("0", false);
    test("1", false);
    test("2", true);
    test("3", true);
    test("4", false);
    test("2305843009213693951", true);
    test("618970019642690137449562111", true);
    test("162259276829213363391578010288127", true);
    test("170141183460469231731687303715884105727", true);
    // - n fits in a u128 in natural_is_probable_prime
    test("18446744073709551616", false);
    test("18446744073709551629", true);
    // - xs[0].even() in natural_is_probable_prime
    test("340282366920938463463374607431768211456", false);
    // - a prime divides xs in limbs_factor_trial
    // - limbs_factor_trial(xs, 1, trial_primes) != 0 in natural_is_probable_prime
    test("680564733841876926926749214863536422913", false);
    // - no prime divides xs in limbs_factor_trial
    // - n.is_square() in natural_is_probable_prime
    test(
        "383123885216472214589586755549637256619304505646776321",
        false,
    );
    // - !n.is_strong_probable_prime(&Natural::TWO) in natural_is_probable_prime
    test("1278732109815440543723434985857665049917079988417", false);
    // - n is a base-2 strong probable prime in natural_is_probable_prime
    test("340282366920938463463374607431768211457", false);
    test("340282366920938463463374607431768211507", true);
    test("100000000000000000000000000000000000000000000000151", true);
    test("100000000000000000000000000000000000000000000000153", false);
    test(
        "1606938044258990275541962092341162602522202993782792835301376",
        false,
    );
    test(
        "20594392980391829144945702397244179651808305900429916228183390377556629550192968156367678\
        921984363664931888934174442574111365963999149931437722084873345171453",
        false,
    );
    test("1971171711155257004609808164365888160584741", false);
    test("1395527206227585972942124661268276727634161", false);
    test(
        "68647976601306097149819007990813932172694353001433054093944634591855431833976560521225596\
        40661454554977296311391480858037121987999716643812574028291115057151",
        true,
    );
    test(
        "27459190640522438859927603196325572869077741200573221637577853836742172733590624208490238\
        562645818219909185245565923432148487951998866575250296113164460228607",
        false,
    );
    test(
        "53113799281676709868958820655246862732959311772703192319944413820040355986085224273916250\
        226522928566888932948624650101534657933765270723940951997876658735194383127083539321903172\
        8127",
        true,
    );
}

fn is_probable_prime_properties_helper(n: &Natural) {
    let p = n.is_probable_prime();
    assert_eq!(bpsw_naive(n), p);
    assert_eq!(rug_is_probable_prime(n), p);
    if let Ok(x) = u128::try_from(n) {
        assert_eq!(x.is_probable_prime(), p);
    }
}

#[test]
fn is_probable_prime_properties() {
    natural_gen().test_properties(|n| is_probable_prime_properties_helper(&n));
    natural_gen_var_8().test_properties(|n| is_probable_prime_properties_helper(&n));
    natural_pair_gen().test_properties(|(a, b)| {
        assert!(!((a + Natural::TWO) * (b + Natural::TWO)).is_probable_prime());
    });
    unsigned_gen_var_29::<u64>().test_properties(|p| {
        assert!(Natural::from(p).is_probable_prime());
    });
}
