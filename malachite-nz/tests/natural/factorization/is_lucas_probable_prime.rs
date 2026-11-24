// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::factorization::traits::IsLucasProbablePrime;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_8};
use malachite_nz::test_util::natural::factorization::is_lucas_probable_prime::*;
use std::str::FromStr;

#[test]
fn test_is_lucas_probable_prime() {
    for n in 0..20000u32 {
        assert_eq!(
            Natural::from(n).is_lucas_probable_prime(),
            n.is_lucas_probable_prime()
        );
    }
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.is_lucas_probable_prime(), out);
        assert_eq!(natural_is_lucas_probable_prime_naive(&n), out);
    };
    // - n fits in a u128 in natural_is_lucas_probable_prime
    test("0", false);
    test("1", false);
    test("2", true);
    test("4", false);
    test("9", false);
    // - jacobi(&r) == 0 && r != *zero in selfridge_q
    test("15", false);
    test("1295", false);
    // - jacobi(&r) == 0 && r == *zero in selfridge_q
    // - jacobi(&r) == -1 in selfridge_q
    test("5", true);
    test("7", true);
    test("170141183460469231731687303715884105727", true);
    // - n.even() || n.is_square() in natural_is_lucas_probable_prime
    test(
        "1606938044258990275541962092341162602522202993782792835301376",
        false,
    );
    // - selfridge_q returns None in natural_is_lucas_probable_prime
    test("850705917302346158658436518579420528635", false);
    // - a != 0u32 in natural_is_lucas_probable_prime
    // - m.get_bit(bit) in natural_is_lucas_probable_prime
    // - !m.get_bit(bit) in natural_is_lucas_probable_prime
    // - U_{n+1} != 0 in natural_is_lucas_probable_prime
    test("340282366920938463463374607431768211457", false);
    test(
        "1606938044258990275541962092341162602522202993782792835301377",
        false,
    );
    // - jacobi(&r) == 1 in selfridge_q
    // - U_{n+1} = 0 in natural_is_lucas_probable_prime
    test("100000000000000000000000000000000000000000000000151", true);
    test(
        "383123885216472214589586755549637256619304505646776321",
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
}

// Inputs of at least 100 limbs, for which the Montgomery reduction switches from limbs_redc_limb to
// limbs_redc. 2^9689 - 1 and 2^9941 - 1 are Mersenne primes.
#[test]
fn test_is_lucas_probable_prime_large() {
    let test = |k: u64, offset: i32, out| {
        let mut n = Natural::power_of_2(k);
        if offset < 0 {
            n -= Natural::from(offset.unsigned_abs());
        } else {
            n += Natural::from(offset.unsigned_abs());
        }
        assert_eq!(n.is_lucas_probable_prime(), out);
    };
    test(9689, -1, true);
    test(9689, 1, false);
    test(9689, -3, false);
    test(9941, -1, true);
    test(9941, -5, false);
}

fn is_lucas_probable_prime_properties_helper(n: &Natural) {
    let p = n.is_lucas_probable_prime();
    assert_eq!(natural_is_lucas_probable_prime_naive(n), p);
    if let Ok(x) = u128::try_from(n) {
        assert_eq!(x.is_lucas_probable_prime(), p);
    }
}

#[test]
fn is_lucas_probable_prime_properties() {
    natural_gen().test_properties(|n| is_lucas_probable_prime_properties_helper(&n));
    natural_gen_var_8().test_properties(|n| is_lucas_probable_prime_properties_helper(&n));
}
