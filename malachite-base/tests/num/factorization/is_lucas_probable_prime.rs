// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::is_lucas_probable_prime::*;
use malachite_base::num::factorization::traits::{IsLucasProbablePrime, IsPrime};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_29};
use malachite_base::test_util::num::factorization::is_lucas_probable_prime::*;

// The Lucas pseudoprimes with Selfridge's parameters below 10^5 (OEIS A217120)
const LUCAS_PSEUDOPRIMES: [u32; 57] = [
    323, 377, 1159, 1829, 3827, 5459, 5777, 9071, 9179, 10877, 11419, 11663, 13919, 14839, 16109,
    16211, 18407, 18971, 19043, 22499, 23407, 24569, 25199, 25877, 26069, 27323, 32759, 34943,
    35207, 39059, 39203, 39689, 40309, 44099, 46979, 47879, 50183, 51983, 53663, 56279, 58519,
    60377, 63881, 69509, 72389, 73919, 75077, 77219, 79547, 79799, 82983, 84419, 86063, 90287,
    94667, 97019, 97439,
];

#[test]
fn test_is_lucas_probable_prime() {
    for n in 0..100000u32 {
        let out = n.is_prime() || LUCAS_PSEUDOPRIMES.binary_search(&n).is_ok();
        assert_eq!(n.is_lucas_probable_prime(), out, "{n}");
        assert_eq!(u64::from(n).is_lucas_probable_prime(), out, "{n}");
        assert_eq!(u128::from(n).is_lucas_probable_prime(), out, "{n}");
        assert_eq!(
            u128_is_lucas_probable_prime_two_limb(u128::from(n)),
            out,
            "{n}"
        );
        assert_eq!(is_lucas_probable_prime_naive(u128::from(n)), out, "{n}");
        if let Ok(n) = u16::try_from(n) {
            assert_eq!(n.is_lucas_probable_prime(), out, "{n}");
        }
    }

    let test = |n: u64, out| {
        assert_eq!(n.is_lucas_probable_prime(), out);
        assert_eq!(is_lucas_probable_prime_naive(u128::from(n)), out);
    };
    // - n <= 1
    test(0, false);
    // - n.even()
    test(2, true);
    test(4, false);
    // - (D / n) = -1 for the first D
    // - m.get_bit(bit)
    // - !m.get_bit(bit)
    test(3, true);
    // - (D / n) = 0 and n divides D
    test(5, true);
    // - n.is_square()
    test(9, false);
    // - (D / n) = 1
    test(11, true);
    // - (D / n) = 0 and gcd(D, n) is a proper factor of n
    test(15, false);

    let test = |n: u128, out| {
        assert_eq!(n.is_lucas_probable_prime(), out);
        assert_eq!(is_lucas_probable_prime_naive(n), out);
    };
    // - n >= 2^64 && n.is_square()
    test(18446744082299486209, false);
    // - n >= 2^64 and gcd(D, n) is a proper factor of n
    test(18446744073709551621, false);
    // - n >= 2^64 && m.get_bit(bit)
    test(18446744073709551617, false);
    // - n >= 2^127, so s == 0 in rem_256_by_128
    test(175458095443608895223302531957005484033, false);
    // Strong pseudoprimes to many bases are not Lucas pseudoprimes
    test(318665857834031151167461, false);
    test(3317044064679887385961981, false);
    test(3825123056546413051, false);
    test(18446744073709551629, true);
    test(618970019642690137449562111, true);
    test(170141183460469231731687303715884105727, true);
    test(340282366920938463463374607431768211297, true);
    test(340282366920938463463374607431768211455, false);
    test(170141183460469231731687303715884105728, false);
}

#[test]
fn is_lucas_probable_prime_properties() {
    unsigned_gen::<u64>().test_properties(|n| {
        let b = n.is_lucas_probable_prime();
        assert_eq!(is_lucas_probable_prime_naive(u128::from(n)), b);
        assert_eq!(u128_is_lucas_probable_prime_two_limb(u128::from(n)), b);
        if n.is_prime() {
            assert!(b);
        }
    });
    unsigned_gen::<u128>().test_properties(|n| {
        assert_eq!(
            n.is_lucas_probable_prime(),
            is_lucas_probable_prime_naive(n)
        );
    });
    unsigned_gen::<u32>().test_properties(|n| {
        assert_eq!(
            n.is_lucas_probable_prime(),
            u64::from(n).is_lucas_probable_prime()
        );
    });
    unsigned_gen::<usize>().test_properties(|n| {
        assert_eq!(
            n.is_lucas_probable_prime(),
            u64::try_from(n).unwrap().is_lucas_probable_prime()
        );
    });
    unsigned_gen_var_29::<u64>().test_properties(|p| {
        assert!(p.is_lucas_probable_prime());
    });
}
