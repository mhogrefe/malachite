// Copyright © 2025 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Parity, Pow, PowerOf2};
use malachite_base::num::basic::traits::One;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_nz::natural::factorization::is_prime::{
    is_probable_prime_bpsw, is_probable_prime_lucas, is_strong_probable_prime,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;

#[test]
fn test_is_prime() {
    let test = |n: u64, out| {
        let nat = Natural::from(n);
        assert_eq!(nat.is_prime(), out);
        // Verify consistency with u64::is_prime for single-limb values
        if n <= u64::MAX {
            assert_eq!(n.is_prime(), out);
        }
    };

    // - self <= ONE in Natural::is_prime
    test(0, false);
    test(1, false);
    // - self == TWO in Natural::is_prime
    test(2, true);
    // - self.even() in Natural::is_prime
    test(4, false);
    test(6, false);
    test(8, false);
    // - !self.even() && self.significant_bits() <= 64 in Natural::is_prime
    // - delegates to u64::is_prime
    test(3, true);
    test(5, true);
    test(7, true);
    test(9, false);
    test(11, true);
    test(13, true);
    test(15, false);
    test(17, true);
    test(19, true);
    test(21, false);
    test(23, true);
    test(25, false);
    test(29, true);
    test(31, true);

    // Small primes
    test(37, true);
    test(41, true);
    test(43, true);
    test(47, true);
    test(53, true);
    test(59, true);
    test(61, true);
    test(67, true);
    test(71, true);
    test(73, true);
    test(79, true);
    test(83, true);
    test(89, true);
    test(97, true);
    test(101, true);
    test(103, true);
    test(107, true);
    test(109, true);
    test(113, true);

    // Small composites
    test(33, false);
    test(35, false);
    test(39, false);
    test(45, false);
    test(49, false);
    test(51, false);
    test(55, false);
    test(57, false);
    test(63, false);
    test(65, false);
    test(69, false);
    test(75, false);
    test(77, false);
    test(81, false);
    test(85, false);
    test(87, false);
    test(91, false);
    test(93, false);
    test(95, false);
    test(99, false);

    // Powers of 2
    test(u64::power_of_2(4), false);
    test(u64::power_of_2(8), false);
    test(u64::power_of_2(16), false);
    test(u64::power_of_2(32), false);

    // Mersenne numbers 2^n - 1 (some prime, some composite)
    test(u64::power_of_2(2) - 1, true); // 3
    test(u64::power_of_2(3) - 1, true); // 7
    test(u64::power_of_2(4) - 1, false); // 15 = 3 * 5
    test(u64::power_of_2(5) - 1, true); // 31
    test(u64::power_of_2(6) - 1, false); // 63 = 7 * 9
    test(u64::power_of_2(7) - 1, true); // 127
    test(u64::power_of_2(8) - 1, false); // 255 = 3 * 5 * 17
    test(u64::power_of_2(13) - 1, true); // 8191
    test(u64::power_of_2(17) - 1, true); // 131071
    test(u64::power_of_2(19) - 1, true); // 524287
    test(u64::power_of_2(31) - 1, true); // 2147483647 (M31)

    // Fermat numbers 2^(2^n) + 1 (some prime, some composite)
    test(u64::power_of_2(1) + 1, true); // F0 = 3
    test(u64::power_of_2(2) + 1, true); // F1 = 5
    test(u64::power_of_2(4) + 1, true); // F2 = 17
    test(u64::power_of_2(8) + 1, true); // F3 = 257
    test(u64::power_of_2(16) + 1, true); // F4 = 65537

    // Large primes
    test(1000000007, true);
    test(1000000009, true);
    test(2147483647, true); // 2^31 - 1 (Mersenne prime M31)
    test(4294967291, true); // Largest 32-bit prime

    // Large composites
    test(1000000000, false);
    test(1000000001, false);
    test(1000000008, false);
    test(2147483646, false);
    test(4294967295, false); // 2^32 - 1 = 3 * 5 * 17 * 257 * 65537

    // Carmichael numbers (composite but pass Fermat test for many bases)
    // BPSW should correctly identify these as composite
    test(561, false); // 3 × 11 × 17
    test(1105, false); // 5 × 13 × 17
    test(1729, false); // 7 × 13 × 19
    test(2465, false); // 5 × 17 × 29
    test(2821, false); // 7 × 13 × 31
    test(6601, false); // 7 × 23 × 41
    test(8911, false); // 7 × 19 × 67

    // Near powers of 2
    test(u64::power_of_2(10) - 3, true); // 1021
    test(u64::power_of_2(10) + 7, true); // 1031
    test(u64::power_of_2(20) - 3, true); // 1048573
    test(u64::power_of_2(20) + 7, true); // 1048583

    // Products of two primes (semiprimes)
    test(15, false); // 3 * 5
    test(21, false); // 3 * 7
    test(35, false); // 5 * 7
    test(77, false); // 7 * 11
    test(143, false); // 11 * 13
    test(221, false); // 13 * 17

    // Perfect squares
    let square1 = 101u64.pow(2);
    test(square1, false); // 10201
    let square2 = 1009u64.pow(2);
    test(square2, false); // 1018081

    // Perfect cubes
    let cube = 101u64.pow(3);
    test(cube, false); // 1030301
}

#[test]
fn test_is_prime_multi_limb() {
    let test = |n: Natural, out| {
        assert_eq!(n.is_prime(), out);
    };

    // - self.significant_bits() > 64 in Natural::is_prime
    // - uses is_probable_prime_bpsw

    // Mersenne prime M127 = 2^127 - 1
    let m127: Natural = (Natural::from(1u32) << 127) - Natural::ONE;
    test(m127, true);

    // 2^128
    let pow_128: Natural = Natural::from(1u32) << 128;
    test(pow_128, false);

    // 2^128 + 1
    let pow_128_plus_1: Natural = (Natural::from(1u32) << 128) + Natural::ONE;
    test(pow_128_plus_1, false);

    // 2^64
    let pow_64: Natural = Natural::from(1u32) << 64;
    test(pow_64, false);

    // 2^65 + 1
    let pow_65_plus_1: Natural = (Natural::from(1u32) << 65) + Natural::ONE;
    test(pow_65_plus_1, false);

    // 2^100 * 2
    let very_large_even: Natural = (Natural::from(1u32) << 100) * Natural::from(2u32);
    test(very_large_even, false);

    // Product of two large primes
    let semiprime: Natural = Natural::from(1000000007u64) * Natural::from(1000000009u64);
    test(semiprime, false);

    // Product of M31 and largest 32-bit prime
    let semiprime2: Natural = Natural::from(2147483647u64) * Natural::from(4294967291u64);
    test(semiprime2, false);

    // Large perfect square
    let square: Natural = Natural::from(1000000007u32).pow(2);
    test(square, false);

    // Large perfect cube
    let cube: Natural = Natural::from(10007u32).pow(3);
    test(cube, false);
}

#[test]
fn test_is_strong_probable_prime() {
    let test = |n: u64, base: u64, out| {
        assert_eq!(
            is_strong_probable_prime(&Natural::from(n), &Natural::from(base)),
            out
        );
    };

    // Small primes with base 2
    test(2, 2, true);
    test(3, 2, true);
    test(5, 2, true);
    test(7, 2, true);
    test(11, 2, true);
    test(13, 2, true);
    test(17, 2, true);
    test(19, 2, true);
    test(23, 2, true);

    // Small composites with base 2
    test(4, 2, false);
    test(6, 2, false);
    test(8, 2, false);
    test(9, 2, false);
    test(10, 2, false);
    test(12, 2, false);
    test(15, 2, false);
    test(21, 2, false);

    // Edge cases
    // - n <= ONE
    test(0, 2, false);
    test(1, 2, false);

    // Different bases
    test(7, 3, true);
    test(7, 5, true);
    test(9, 2, false);
    test(9, 3, false);

    // Large primes
    test(1000000007, 2, true);
    test(2147483647, 2, true); // M31
}

#[test]
fn test_is_probable_prime_lucas() {
    let test = |n: u64, out| {
        assert_eq!(is_probable_prime_lucas(&Natural::from(n)), out);
    };

    // - n <= TWO in is_probable_prime_lucas
    test(0, false);
    test(1, false);
    test(2, true);

    // - n.even() in is_probable_prime_lucas
    test(4, false);
    test(6, false);
    test(8, false);

    // - n == 3 in is_probable_prime_lucas (early return for small primes)
    test(3, true);
    // - n == 5 in is_probable_prime_lucas
    test(5, true);

    // Small odd primes
    test(7, true);
    test(11, true);
    test(13, true);
    test(17, true);
    test(19, true);
    test(23, true);
    test(29, true);
    test(31, true);

    // Small odd composites
    test(9, false);
    test(15, false);
    test(21, false);
    test(25, false);
    test(27, false);
    test(33, false);
    test(35, false);

    // Carmichael numbers
    test(561, false);
    test(1105, false);
    test(1729, false);

    // Larger primes
    test(1000000007, true);
    test(2147483647, true);
}

#[test]
fn test_is_probable_prime_bpsw() {
    let test = |n: u64, out| {
        assert_eq!(is_probable_prime_bpsw(&Natural::from(n)), out);
    };

    // - n <= ONE in is_probable_prime_bpsw
    test(0, false);
    test(1, false);

    // - n == TWO in is_probable_prime_bpsw
    test(2, true);

    // - n.even() in is_probable_prime_bpsw
    test(4, false);
    test(6, false);
    test(8, false);

    // Small primes
    test(3, true);
    test(5, true);
    test(7, true);
    test(11, true);
    test(13, true);
    test(17, true);
    test(19, true);
    test(23, true);
    test(29, true);
    test(31, true);
    test(37, true);
    test(41, true);
    test(43, true);
    test(47, true);

    // Small composites
    test(9, false);
    test(10, false);
    test(12, false);
    test(14, false);
    test(15, false);
    test(16, false);
    test(18, false);
    test(20, false);
    test(21, false);
    test(22, false);
    test(24, false);
    test(25, false);
    test(26, false);

    // Carmichael numbers (BPSW should correctly identify as composite)
    test(561, false);
    test(1105, false);
    test(1729, false);
    test(2465, false);
    test(2821, false);

    // Large primes
    test(1000000007, true);
    test(1000000009, true);
    test(2147483647, true);
    test(4294967291, true);

    // Large composites
    test(1000000000, false);
    test(1000000001, false);
    test(1000000008, false);
    test(4294967295, false);
}

#[test]
fn is_prime_properties() {
    // Property: products of two primes > 1 are composite
    natural_gen().test_properties(|n| {
        let is_prime = n.is_prime();
        // If n is prime and > 2, then n is odd
        if is_prime && n > Natural::from(2u32) {
            assert!(n.odd());
        }
        // If n is even and > 2, then n is composite
        if n.even() && n > Natural::from(2u32) {
            assert!(!is_prime);
        }
    });
}
