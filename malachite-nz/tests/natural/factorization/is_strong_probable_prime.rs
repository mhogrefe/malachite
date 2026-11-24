// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Two;
use malachite_base::num::factorization::traits::IsStrongProbablePrime;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen_var_8, natural_pair_gen};
use malachite_nz::test_util::natural::factorization::is_strong_probable_prime::*;
use std::str::FromStr;

#[test]
fn test_is_strong_probable_prime() {
    for n in 0..300u32 {
        for a in 0..300u32 {
            assert_eq!(
                Natural::from(n).is_strong_probable_prime(Natural::from(a)),
                n.is_strong_probable_prime(a)
            );
        }
    }
    let test = |s, a, out| {
        let n = Natural::from_str(s).unwrap();
        let a = Natural::from_str(a).unwrap();
        assert_eq!(n.is_strong_probable_prime(&a), out);
        assert_eq!(natural_is_strong_probable_prime_naive(&n, &a), out);
        assert_eq!(n.is_strong_probable_prime(a), out);
    };
    // - *n <= 1u32 in natural_is_strong_probable_prime
    test("0", "2", false);
    test("1", "2", false);
    // - a == 1u32 || a == nm1 || a == 0u32 in natural_is_strong_probable_prime
    test("2", "0", true);
    test("2", "5", true);
    test("4", "3", true);
    // - y != 1u32 && y != nm1 in natural_is_strong_probable_prime
    // - s <= 1, so the loop is empty, in natural_is_strong_probable_prime
    test(
        "1606938044258990275541962092341162602522202993782792835301376",
        "3",
        false,
    );
    // - y == nm1 in natural_is_strong_probable_prime
    test("170141183460469231731687303715884105727", "3", true);
    test("5444517870735015415413993718908291383453", "4", true);
    // - y == 1u32 in natural_is_strong_probable_prime
    test(
        "68647976601306097149819007990813932172694353001433054093944634591855431833976560521225596\
        40661454554977296311391480858037121987999716643812574028291115057151",
        "68647976601306097149819007990813932172694353001433054093944634591855431833976560521225596\
        40661454554977296311391480858037121987999716643812574028291115057153",
        true,
    );
    // - a later square is n - 1 in natural_is_strong_probable_prime
    test("1971171711155257004609808164365888160584741", "2", true);
    test(
        "170141183460469231731687303715884105727",
        "68647976601306097149819007990813932172694353001433054093944634591855431833976560521225596\
        40661454554977296311391480858037121987999716643812574028291115057151",
        true,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377",
        "1606938044258990275541962092341162602522202993782792835301376",
        true,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377",
        "0",
        true,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377",
        "1",
        true,
    );
    // - every square fails in natural_is_strong_probable_prime
    test("1971171711155257004609808164365888160584741", "3", false);
    test("1395527206227585972942124661268276727634161", "3", true);
    test("1395527206227585972942124661268276727634161", "5", false);
    test(
        "68647976601306097149819007990813932172694353001433054093944634591855431833976560521225596\
        40661454554977296311391480858037121987999716643812574028291115057151",
        "3",
        true,
    );
    test(
        "27459190640522438859927603196325572869077741200573221637577853836742172733590624208490238\
        562645818219909185245565923432148487951998866575250296113164460228607",
        "2",
        true,
    );
    test(
        "27459190640522438859927603196325572869077741200573221637577853836742172733590624208490238\
        562645818219909185245565923432148487951998866575250296113164460228607",
        "3",
        false,
    );
}

#[test]
fn is_strong_probable_prime_properties() {
    natural_pair_gen().test_properties(|(n, a)| {
        let p = n.is_strong_probable_prime(&a);
        assert_eq!(natural_is_strong_probable_prime_naive(&n, &a), p);
        if let (Ok(x), Ok(y)) = (u128::try_from(&n), u128::try_from(&a)) {
            assert_eq!(x.is_strong_probable_prime(y), p);
        }
        assert_eq!(n.is_strong_probable_prime(a), p);
    });
    let two = Natural::TWO;
    natural_gen_var_8().test_properties(|n| {
        assert_eq!(
            n.is_strong_probable_prime(&two),
            natural_is_strong_probable_prime_naive(&n, &two)
        );
    });
}
