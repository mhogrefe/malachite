// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::One;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::factorization::traits::Primes;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::crt_comb::CrtComb;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::unsigned_vec_natural_pair_gen_var_1;
use malachite_nz::test_util::integer::arithmetic::crt::balanced_to_canonical;
use std::panic::catch_unwind;
#[cfg(not(feature = "32_bit_limbs"))]
use std::str::FromStr;

#[cfg(not(feature = "32_bit_limbs"))]
fn n(s: &str) -> Natural {
    Natural::from_str(s).unwrap()
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_crt_comb() {
    // - a single tiny group
    let comb = CrtComb::new(&[3, 5, 7]).unwrap();
    assert_eq!(comb.prime_count(), 3);
    assert_eq!(comb.modulus(), &Natural::from(105u32));
    assert_eq!(comb.reduce(&Natural::from(1000u32)), &[1, 0, 6]);
    assert_eq!(comb.reduce(&Natural::ZERO), &[0, 0, 0]);
    assert_eq!(comb.combine(&[1, 0, 6]), Natural::from(55u32));
    assert_eq!(comb.combine(&[0, 0, 0]), Natural::ZERO);
    assert_eq!(comb.combine(&[2, 4, 6]), Natural::from(104u32));
    assert_eq!(comb.combine_balanced(&[2, 4, 6]).to_string(), "-1");
    // - a single modulus
    let comb = CrtComb::new(&[7]).unwrap();
    assert_eq!(comb.reduce(&Natural::from(100u32)), &[2]);
    assert_eq!(comb.combine(&[5]), Natural::from(5u32));
    assert_eq!(comb.combine_balanced(&[5]).to_string(), "-2");
    // - large single-modulus groups: the packed all-large path, with premultiplied idempotents
    let big: [Limb; 4] =
        [18446744073709551615, 18446744073709551613, 18446744073709551611, 18446744073709551607];
    let comb = CrtComb::new(&big).unwrap();
    assert_eq!(
        comb.modulus(),
        &n("115792089237316195310583153771727654139615141207680168745710967996100881940615"),
    );
    let rs: [Limb; 4] = [123456789, 987654321, 555555555, 111111111];
    assert_eq!(
        comb.combine(&rs),
        n("75988558562129616631228382723732276951497187460512564757824751443088375855849"),
    );
    assert_eq!(
        comb.combine_balanced(&rs).to_string(),
        "-39803530675186578679354771047995377188117953747167603987886216553012506084766",
    );
    assert_eq!(
        comb.reduce(&n("12345678987654321012345678901234567890")),
        &[15527410016293092465, 16865931214215438255, 18204452412137784045, 2434750734272924018,],
    );
    // - a three-modulus group and a single-modulus group in the same chunk: the general path,
    //   including its single-modulus case
    let comb = CrtComb::new(&[3, 5, 7, 18446744073709551611]).unwrap();
    assert_eq!(comb.combine(&[1, 2, 3, 12345]), n("866996971464348938062"),);
    assert_eq!(
        comb.reduce(&Natural::from(10u32).pow(30)),
        &[1, 0, 1, 5076944541355806736],
    );
    // - unusable moduli
    assert!(CrtComb::new(&[4, 6]).is_none());
    assert!(CrtComb::new(&[5, 5]).is_none());
    assert!(CrtComb::new(&[1, 3]).is_none());
    assert!(CrtComb::new(&[0, 3]).is_none());
    assert!(CrtComb::new(&[1]).is_none());
}

#[test]
fn test_crt_comb_many_primes() {
    // Enough primes that both directions split into multiple chunks, exercising the subproduct
    // trees, the chunk-merging, and working-slot reuse. The expected values are checked against
    // one-modulus-at-a-time arithmetic rather than hardcoded.
    let primes: Vec<Limb> = Limb::primes().skip(1).take(2000).collect();
    let comb = CrtComb::new(&primes).unwrap();
    assert_eq!(comb.prime_count(), 2000);
    let p = primes
        .iter()
        .fold(Natural::ONE, |acc, &m| acc * Natural::from(m));
    assert_eq!(comb.modulus(), &p);
    let x = Natural::from(3u32).pow(20000);
    let rs = comb.reduce(&x);
    for (r, m) in rs.iter().copied().zip(primes.iter().copied()) {
        assert_eq!(Natural::from(r), &x % Natural::from(m));
    }
    assert_eq!(comb.combine(&rs), &x % &p);
    let y = comb.combine_balanced(&rs);
    assert_eq!(balanced_to_canonical(&y, &p), &x % &p);
}

#[test]
fn crt_comb_fail() {
    assert_panic!(CrtComb::new(&[]));
    assert_panic!({
        let comb = CrtComb::new(&[3, 5]).unwrap();
        comb.combine(&[1])
    });
    assert_panic!({
        let comb = CrtComb::new(&[3, 5]).unwrap();
        comb.combine(&[3, 0])
    });
    assert_panic!({
        let comb = CrtComb::new(&[3, 5]).unwrap();
        comb.combine_balanced(&[0, 5])
    });
}

#[test]
fn crt_comb_properties() {
    unsigned_vec_natural_pair_gen_var_1().test_properties(|(ms, x)| {
        let comb = CrtComb::new(&ms).unwrap();
        assert_eq!(comb.prime_count(), ms.len());
        let p = ms
            .iter()
            .fold(Natural::ONE, |acc, &m| acc * Natural::from(m));
        assert_eq!(comb.modulus(), &p);

        let rs = comb.reduce(&x);
        for (r, m) in rs.iter().copied().zip(ms.iter().copied()) {
            assert_eq!(Natural::from(r), &x % Natural::from(m));
        }

        let combined = comb.combine(&rs);
        assert_eq!(combined, &x % &p);
        // The comb agrees with the general-moduli combination.
        let moduli = ms.iter().map(|&m| Natural::from(m)).collect::<Vec<_>>();
        let values = rs.iter().map(|&r| Natural::from(r)).collect::<Vec<_>>();
        assert_eq!(Natural::multi_crt(&moduli, &values), Some(combined.clone()));

        let y = comb.combine_balanced(&rs);
        let doubled = y.unsigned_abs_ref() << 1u64;
        if y >= 0u32 {
            assert!(doubled <= p);
        } else {
            assert!(doubled < p);
        }
        assert_eq!(balanced_to_canonical(&y, &p), combined);

        // The context is reusable.
        assert_eq!(comb.reduce(&x), rs);
    });
}
