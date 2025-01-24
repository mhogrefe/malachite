// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Primorial;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::{IsPrime, Primes};
use malachite_base::test_util::generators::{
    unsigned_gen_var_27, unsigned_gen_var_28, unsigned_gen_var_5,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::natural::arithmetic::primorial::{
    primorial_naive, product_of_first_n_primes_naive,
};
use rug::Complete;

#[test]
fn test_primorial() {
    fn test(n: u64, out: &str) {
        let p = Natural::primorial(n);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(primorial_naive(n).to_string(), out);
        assert_eq!(
            rug::Integer::primorial(u32::exact_from(n))
                .complete()
                .to_string(),
            out
        );
    }
    test(0, "1");
    test(1, "1");
    test(2, "2");
    test(3, "6");
    test(4, "6");
    test(5, "30");
    test(20, "9699690");
    // - sieve[index] & mask == 0
    // - prod <= max_prod
    // - sieve[index] & mask != 0
    // - prod > max_prod
    test(53, "32589158477190044730");
    test(100, "2305567963945518424753102147331756070");
}

#[test]
fn test_product_of_first_n_primes() {
    fn test(n: u64, out: &str) {
        let p = Natural::product_of_first_n_primes(n);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(product_of_first_n_primes_naive(n).to_string(), out);
    }
    test(0, "1");
    test(1, "2");
    test(2, "6");
    test(3, "30");
    test(4, "210");
    test(5, "2310");
    test(10, "6469693230");
    test(
        100,
        "47119307999061849531624878347602604220205747734096755201886348396164153358450342212052892\
        567055446819724391040977771579918043802842183150387194449439904925790307206359905384523125\
        28339864352999310398481791730017201031090",
    );
}

#[test]
fn primorial_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let p = Natural::primorial(n);
        assert!(p.is_valid());
        assert_eq!(primorial_naive(n), p);
        assert_eq!(
            Natural::exact_from(&rug::Integer::primorial(u32::exact_from(n)).complete()),
            p
        );
        assert_ne!(p, 0u32);
        if n != 0 {
            let q = Natural::primorial(n - 1);
            if n.is_prime() {
                assert!(q < p);
            } else {
                assert_eq!(q, p);
            }
        }
    });

    unsigned_gen_var_27::<Limb>().test_properties(|n| {
        assert_eq!(Natural::primorial(n), Limb::primorial(n));
    });
}

#[test]
fn product_of_first_n_primes_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let f = Natural::product_of_first_n_primes(n);
        assert!(f.is_valid());
        assert_eq!(product_of_first_n_primes_naive(n), f);
        assert_ne!(f, 0);
        if n != 0 {
            let p = u64::primes().nth(usize::exact_from(n) - 1).unwrap();
            assert_eq!(Natural::primorial(p), f);
            assert_eq!(
                f / Natural::product_of_first_n_primes(n - 1),
                Natural::exact_from(p)
            );
        }
    });

    unsigned_gen_var_28::<Limb>().test_properties(|n| {
        assert_eq!(
            Natural::product_of_first_n_primes(n),
            Limb::product_of_first_n_primes(n)
        );
    });
}
