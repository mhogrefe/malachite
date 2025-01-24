// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_pair_gen_var_50};
use malachite_base::test_util::num::factorization::is_prime::is_prime_naive;

fn is_prime_helper<T: IsPrime + PrimitiveUnsigned>() {
    let test = |n: u64, out| {
        if let Ok(n) = T::try_from(n) {
            assert_eq!(n.is_prime(), out);
            assert_eq!(is_prime_naive(n), out);
        }
    };
    // - in u32::is_prime
    // - n < 11 in u32::is_prime
    // - in u64::is_prime
    // - n < 11 in u64::is_prime
    test(0, false);
    test(1, false);
    test(2, true);
    test(3, true);
    test(4, false);
    test(5, true);
    test(6, false);
    test(7, true);
    test(8, false);
    test(9, false);
    test(10, false);

    // - n >= 11 in u32::is_prime
    // - prime factor <= 7 in u32::is_prime
    // - n >= 11 in u64::is_prime
    // - prime factor <= 7 in u64::is_prime
    test(u64::power_of_2(4) - 1, false);
    // - no prime factor <= 7 in u32::is_prime
    // - n < 121 in u32::is_prime
    // - no prime factor <= 7 in u64::is_prime
    // - n < 121 in u64::is_prime
    test(u64::power_of_2(5) - 1, true);
    test(u64::power_of_2(6) - 1, false);
    // - n >= 121 in u32::is_prime
    // - no prime factor <= 53 in u32::is_prime
    // - n < 3481 in u32::is_prime
    // - n >= 121 in u64::is_prime
    // - no prime factor <= 53 in u64::is_prime
    // - n < 3481 in u64::is_prime
    test(u64::power_of_2(7) - 1, true);
    test(u64::power_of_2(8) - 1, false);
    test(u64::power_of_2(9) - 1, false);
    test(u64::power_of_2(10) - 1, false);
    // - prime factor <= 53 in u32::is_prime
    // - prime factor <= 53 in u64::is_prime
    test(u64::power_of_2(11) - 1, false);
    test(u64::power_of_2(12) - 1, false);
    // - n >= 3481 in u32::is_prime
    // - n <= 1000000 or no prime factor <= 149 in u32::is_prime
    // - in n_is_probabprime_u32
    // - FLINT_ODDPRIME_SMALL_CUTOFF <= n < FLINT_PRIMES_TAB_DEFAULT_CUTOFF in n_is_probabprime_u32
    // - in n_is_oddprime_binary_u32
    // - in n_prime_pi_bounds_u32
    // - n >= PRIME_PI_ODD_LOOKUP_CUTOFF in n_prime_pi_bounds_u32
    // - n < primes[prime_hi as usize] in n_is_oddprime_binary_u32
    // - primes[(prime_lo + diff) as usize] > n in n_is_oddprime_binary_u32
    // - diff > 1 in n_is_oddprime_binary_u32
    // - diff <= diff2 in n_is_oddprime_binary_u32
    // - primes[(prime_lo + diff) as usize] <= n in n_is_oddprime_binary_u32
    // - diff <= 1 in n_is_oddprime_binary_u32
    // - n >= 3481 in u64::is_prime
    // - n <= 1000000 or no prime factor <= 149 in u64::is_prime
    // - in n_is_probabprime_u64
    // - FLINT_ODDPRIME_SMALL_CUTOFF <= n < FLINT_PRIMES_TAB_DEFAULT_CUTOFF in n_is_probabprime_u64
    // - in n_is_oddprime_binary_u64
    // - in n_prime_pi_bounds_u64
    // - n >= PRIME_PI_ODD_LOOKUP_CUTOFF in n_prime_pi_bounds_u64
    // - n < primes[prime_hi as usize] in n_is_oddprime_binary_u64
    // - primes[(prime_lo + diff) as usize] > n in n_is_oddprime_binary_u64
    // - diff > 1 in n_is_oddprime_binary_u64
    // - diff <= diff2 in n_is_oddprime_binary_u64
    // - primes[(prime_lo + diff) as usize] <= n in n_is_oddprime_binary_u64
    // - diff <= 1 in n_is_oddprime_binary_u64
    test(u64::power_of_2(13) - 1, true);
    test(u64::power_of_2(14) - 1, false);
    test(u64::power_of_2(15) - 1, false);
    test(u64::power_of_2(16) - 1, false);
    test(u64::power_of_2(17) - 1, true);
    test(u64::power_of_2(18) - 1, false);
    test(u64::power_of_2(19) - 1, true);
    test(u64::power_of_2(20) - 1, false);
    test(u64::power_of_2(21) - 1, false);
    test(u64::power_of_2(22) - 1, false);
    test(u64::power_of_2(23) - 1, false);
    test(u64::power_of_2(24) - 1, false);
    test(u64::power_of_2(25) - 1, false);
    test(u64::power_of_2(26) - 1, false);
    test(u64::power_of_2(27) - 1, false);
    test(u64::power_of_2(28) - 1, false);
    // - n >= FLINT_PRIMES_TAB_DEFAULT_CUTOFF in n_is_probabprime_u32
    // - n >= 9080191 in n_is_probabprime_u32
    // - in n_is_strong_probabprime2_preinv_u32
    // - a > 1 && a != n - 1 in n_is_strong_probabprime2_preinv_u32
    // - in n_powmod2_ui_preinv_u32
    // - exp != 0 in n_powmod2_ui_preinv_u32
    // - a != 0 in n_powmod2_ui_preinv_u32
    // - a < n in n_powmod2_ui_preinv_u32
    // - exp.odd() in n_powmod2_ui_preinv_u32
    // - y == 1 in n_is_strong_probabprime2_preinv_u32
    // - y != 1 in n_is_strong_probabprime2_preinv_u32
    // - n >= FLINT_PRIMES_TAB_DEFAULT_CUTOFF in n_is_probabprime_u64
    // - n < 1050535501 in n_is_probabprime_u64
    // - n >= 341531 in n_is_probabprime_u64
    // - in n_is_strong_probabprime_precomp_u64
    // - a >= n in n_is_strong_probabprime_precomp_u64
    // - in n_mod2_precomp_u64
    // - a >= n in n_mod2_precomp_u64
    // - ni >= 0 in n_mod2_precomp_u64
    // - n != 1 in n_mod2_precomp_u64
    // - rem >= 0 in n_mod2_precomp_u64 first time
    // - a > 1 && a != n - 1 n_is_strong_probabprime_precomp_u64
    // - in n_powmod_ui_precomp_u64
    // - n != 1 in n_powmod_ui_precomp_u64
    // - exp.odd() in n_powmod_ui_precomp_u64
    // - in n_mulmod_precomp_u64
    // - n > rem in n_mulmod_precomp_u64
    // - exp != 0 in n_powmod_ui_precomp_u64
    // - exp == 0 in n_powmod_ui_precomp_u64
    // - y != 1 n_is_strong_probabprime_precomp_u64
    test(u64::power_of_2(29) - 1, false);
    test(u64::power_of_2(30) - 1, false);
    // - n >= 1050535501 in n_is_probabprime_u64
    // - in n_is_probabprime_bpsw_u64
    // - n > 1 in n_is_probabprime_bpsw_u64
    // - n.odd() in n_is_probabprime_bpsw_u64
    // - nm10 == 3 || nm10 == 7 in n_is_probabprime_bpsw_u64
    // - in n_is_probabprime_fermat_u64
    // - n.significant_bits() <= FLINT_D_BITS_U64 in n_is_probabprime_fermat_u64
    // - in n_powmod_u64
    // - in n_powmod_precomp_u64
    // - exp >= 0 in n_powmod_precomp_u64
    // - exp.even() in n_powmod_ui_precomp_u64
    // - in n_is_probabprime_fibonacci_u64
    // - i64::wrapping_from(n).gt_abs(&3) in n_is_probabprime_fibonacci_u64
    // - n.significant_bits() <= FLINT_D_BITS_U64 in n_is_probabprime_fibonacci_u64
    // - in fchain_precomp_u64
    // - m & power != 0 in fchain_precomp_u64
    // - m & power == 0 in fchain_precomp_u64
    test(u64::power_of_2(31) - 1, true);
    test(u64::power_of_2(32) - 1, false);
    test(u64::power_of_2(33) - 1, false);
    test(u64::power_of_2(34) - 1, false);
    test(u64::power_of_2(35) - 1, false);
    test(u64::power_of_2(36) - 1, false);
    // - nm10 != 3 && nm10 != 7 in n_is_probabprime_bpsw_u64
    // - n.significant_bits() <= FLINT_D_BITS_U64 in n_is_probabprime_bpsw_u64
    // - a < n in n_is_strong_probabprime_precomp_u64
    // - y == 1 n_is_strong_probabprime_precomp_u64
    // - n_is_strong_probabprime_precomp_u64(n, npre, 2, d) in n_is_probabprime_bpsw_u64
    // - in n_is_probabprime_lucas_u64
    // - d.gcd(n % d) == 1 in n_is_probabprime_lucas_u64
    // - i.even() in n_is_probabprime_lucas_u64
    // - !neg_d in n_is_probabprime_lucas_u64 first time
    // - jacobi != -1 in n_is_probabprime_lucas_u64
    // - i.odd() in n_is_probabprime_lucas_u64
    // - neg_d in n_is_probabprime_lucas_u64 first time
    // - jacobi == -1 in n_is_probabprime_lucas_u64
    // - j < 100 in n_is_probabprime_lucas_u64
    // - neg_d in n_is_probabprime_lucas_u64 second time
    // - q >= 0 && n >= 52 in n_is_probabprime_lucas_u64
    // - n > FLINT_D_BITS_U64 in n_is_probabprime_lucas_u64
    // - in lchain2_preinv_u64
    // - m & power != 0 in lchain2_preinv_u64
    // - m & power == 0 in lchain2_preinv_u64
    test(u64::power_of_2(37) - 1, false);
    test(u64::power_of_2(38) - 1, false);
    test(u64::power_of_2(39) - 1, false);
    test(u64::power_of_2(40) - 1, false);
    test(u64::power_of_2(41) - 1, false);
    test(u64::power_of_2(42) - 1, false);
    test(u64::power_of_2(43) - 1, false);
    test(u64::power_of_2(44) - 1, false);
    test(u64::power_of_2(45) - 1, false);
    test(u64::power_of_2(46) - 1, false);
    test(u64::power_of_2(47) - 1, false);
    test(u64::power_of_2(48) - 1, false);
    // - n > 1000000 and prime factor <= 149 in u64::is_prime
    test(u64::power_of_2(49) - 1, false);
    test(u64::power_of_2(50) - 1, false);
    test(u64::power_of_2(51) - 1, false);
    test(u64::power_of_2(52) - 1, false);
    test(u64::power_of_2(53) - 1, false);
    test(u64::power_of_2(54) - 1, false);
    test(u64::power_of_2(55) - 1, false);
    test(u64::power_of_2(56) - 1, false);
    test(u64::power_of_2(57) - 1, false);
    test(u64::power_of_2(58) - 1, false);
    // - n.significant_bits() > FLINT_D_BITS_U64 in n_is_probabprime_fermat_u64
    // - in n_powmod2_ui_preinv_u64
    // - exp != 0 in n_powmod2_ui_preinv_u64
    // - a != 0 in n_powmod2_ui_preinv_u64
    // - a < 0 in n_powmod2_ui_preinv_u64
    // - exp.odd() in n_powmod2_ui_preinv_u64
    // - n.significant_bits() > FLINT_D_BITS_U64 in n_is_probabprime_fibonacci_u64
    // - in fchain2_preinv_u64
    // - m & power != 0 in fchain2_preinv_u64
    // - m & power == 0 in fchain2_preinv_u64
    test(u64::power_of_2(59) - 1, false);
    test(u64::power_of_2(60) - 1, false);
    // - n.significant_bits() > FLINT_D_BITS_U64 in n_is_probabprime_bpsw_u64
    // - in n_is_strong_probabprime2_preinv_u64
    // - a > 1 && a != n - 1 in n_is_strong_probabprime2_preinv_u64
    // - y == 1 in n_is_strong_probabprime2_preinv_u64
    // - n_is_strong_probabprime2_preinv_u64(n, ninv, 2, d) in n_is_probabprime_bpsw_u64
    // - !neg_d in n_is_probabprime_lucas_u64 second time
    // - q < 0 in n_is_probabprime_lucas_u64
    // - q < 0 && n >= 52 in n_is_probabprime_lucas_u64
    test(u64::power_of_2(61) - 1, true);
    test(u64::power_of_2(62) - 1, false);
    test(u64::power_of_2(63) - 1, false);
    test(u64::MAX, false);

    test(u64::power_of_2(4) + 1, true);
    test(u64::power_of_2(5) + 1, false);
    test(u64::power_of_2(6) + 1, false);
    test(u64::power_of_2(7) + 1, false);
    test(u64::power_of_2(8) + 1, true);
    test(u64::power_of_2(9) + 1, false);
    test(u64::power_of_2(10) + 1, false);
    test(u64::power_of_2(11) + 1, false);
    test(u64::power_of_2(12) + 1, false);
    test(u64::power_of_2(13) + 1, false);
    test(u64::power_of_2(14) + 1, false);
    test(u64::power_of_2(15) + 1, false);
    test(u64::power_of_2(16) + 1, true);
    test(u64::power_of_2(17) + 1, false);
    test(u64::power_of_2(18) + 1, false);
    test(u64::power_of_2(19) + 1, false);
    test(u64::power_of_2(20) + 1, false);
    test(u64::power_of_2(21) + 1, false);
    test(u64::power_of_2(22) + 1, false);
    test(u64::power_of_2(23) + 1, false);
    // - n > 1000000 and prime factor <= 149 in u32::is_prime
    test(u64::power_of_2(24) + 1, false);
    test(u64::power_of_2(25) + 1, false);
    test(u64::power_of_2(26) + 1, false);
    test(u64::power_of_2(27) + 1, false);
    test(u64::power_of_2(28) + 1, false);
    test(u64::power_of_2(29) + 1, false);
    test(u64::power_of_2(30) + 1, false);
    test(u64::power_of_2(31) + 1, false);
    test(u64::power_of_2(32) + 1, false);
    test(u64::power_of_2(33) + 1, false);
    test(u64::power_of_2(34) + 1, false);
    test(u64::power_of_2(35) + 1, false);
    test(u64::power_of_2(36) + 1, false);
    test(u64::power_of_2(37) + 1, false);
    test(u64::power_of_2(38) + 1, false);
    test(u64::power_of_2(39) + 1, false);
    // - rem < 0 first time in n_mulmod_precomp_u64
    // - rem >= 0 second time in n_mulmod_precomp_u64
    test(u64::power_of_2(40) + 1, false);
    test(u64::power_of_2(41) + 1, false);
    test(u64::power_of_2(42) + 1, false);
    test(u64::power_of_2(43) + 1, false);
    test(u64::power_of_2(44) + 1, false);
    test(u64::power_of_2(45) + 1, false);
    test(u64::power_of_2(46) + 1, false);
    test(u64::power_of_2(47) + 1, false);
    test(u64::power_of_2(48) + 1, false);
    test(u64::power_of_2(49) + 1, false);
    test(u64::power_of_2(50) + 1, false);
    test(u64::power_of_2(51) + 1, false);
    test(u64::power_of_2(52) + 1, false);
    test(u64::power_of_2(53) + 1, false);
    test(u64::power_of_2(54) + 1, false);
    test(u64::power_of_2(55) + 1, false);
    test(u64::power_of_2(56) + 1, false);
    test(u64::power_of_2(57) + 1, false);
    test(u64::power_of_2(58) + 1, false);
    test(u64::power_of_2(59) + 1, false);
    test(u64::power_of_2(60) + 1, false);
    test(u64::power_of_2(61) + 1, false);
    test(u64::power_of_2(62) + 1, false);
    test(u64::power_of_2(63) + 1, false);

    test(u64::power_of_2(4) - 3, true);
    test(u64::power_of_2(5) - 1, true);
    test(u64::power_of_2(6) - 3, true);
    test(u64::power_of_2(7) - 1, true);
    test(u64::power_of_2(8) - 5, true);
    test(u64::power_of_2(9) - 3, true);
    test(u64::power_of_2(10) - 3, true);
    test(u64::power_of_2(11) - 9, true);
    // - n < FLINT_ODDPRIME_SMALL_CUTOFF in n_is_probabprime_u32
    // - in n_is_oddprime_small_u32
    // - n < FLINT_ODDPRIME_SMALL_CUTOFF in n_is_probabprime_u64
    // - in n_is_oddprime_small_u64
    test(u64::power_of_2(12) - 3, true);
    test(u64::power_of_2(13) - 1, true);
    test(u64::power_of_2(14) - 3, true);
    test(u64::power_of_2(15) - 19, true);
    test(u64::power_of_2(16) - 15, true);
    test(u64::power_of_2(17) - 1, true);
    test(u64::power_of_2(18) - 5, true);
    test(u64::power_of_2(19) - 1, true);
    // - n < 9080191 in n_is_probabprime_u32
    test(u64::power_of_2(20) - 3, true);
    // - exp.even() in n_powmod2_ui_preinv_u32
    test(u64::power_of_2(21) - 9, true);
    test(u64::power_of_2(22) - 3, true);
    test(u64::power_of_2(23) - 15, true);
    test(u64::power_of_2(24) - 3, true);
    test(u64::power_of_2(25) - 39, true);
    test(u64::power_of_2(26) - 5, true);
    test(u64::power_of_2(27) - 39, true);
    test(u64::power_of_2(28) - 57, true);
    test(u64::power_of_2(29) - 3, true);
    test(u64::power_of_2(30) - 35, true);
    test(u64::power_of_2(31) - 1, true);
    test(u64::power_of_2(32) - 5, true);
    test(u64::power_of_2(33) - 9, true);
    test(u64::power_of_2(34) - 41, true);
    test(u64::power_of_2(35) - 31, true);
    test(u64::power_of_2(36) - 5, true);
    test(u64::power_of_2(37) - 25, true);
    test(u64::power_of_2(38) - 45, true);
    test(u64::power_of_2(39) - 7, true);
    test(u64::power_of_2(40) - 87, true);
    test(u64::power_of_2(41) - 21, true);
    test(u64::power_of_2(42) - 11, true);
    test(u64::power_of_2(43) - 57, true);
    test(u64::power_of_2(44) - 17, true);
    test(u64::power_of_2(45) - 55, true);
    test(u64::power_of_2(46) - 21, true);
    test(u64::power_of_2(47) - 115, true);
    test(u64::power_of_2(48) - 59, true);
    test(u64::power_of_2(49) - 81, true);
    test(u64::power_of_2(50) - 27, true);
    test(u64::power_of_2(51) - 129, true);
    test(u64::power_of_2(52) - 47, true);
    // - rem < 0 second time in n_mulmod_precomp_u64
    test(u64::power_of_2(53) - 111, true);
    // - exp.even() in n_powmod2_ui_preinv_u64
    test(u64::power_of_2(54) - 33, true);
    test(u64::power_of_2(55) - 55, true);
    // - y != 1 in n_is_strong_probabprime2_preinv_u64
    test(u64::power_of_2(56) - 5, true);
    test(u64::power_of_2(57) - 13, true);
    test(u64::power_of_2(58) - 27, true);
    test(u64::power_of_2(59) - 55, true);
    test(u64::power_of_2(60) - 93, true);
    test(u64::power_of_2(61) - 1, true);
    test(u64::power_of_2(62) - 57, true);
    test(u64::power_of_2(63) - 25, true);
    test(u64::MAX - 58, true);

    test(u64::power_of_2(5) + 5, true);
    test(u64::power_of_2(6) + 3, true);
    test(u64::power_of_2(7) + 3, true);
    test(u64::power_of_2(9) + 9, true);
    test(u64::power_of_2(10) + 7, true);
    test(u64::power_of_2(11) + 5, true);
    test(u64::power_of_2(12) + 3, true);
    test(u64::power_of_2(13) + 17, true);
    test(u64::power_of_2(14) + 27, true);
    // - diff > diff2 in n_is_oddprime_binary_u32
    // - diff > diff2 in n_is_oddprime_binary_u64
    test(u64::power_of_2(15) + 3, true);
    test(u64::power_of_2(17) + 29, true);
    test(u64::power_of_2(18) + 3, true);
    test(u64::power_of_2(19) + 21, true);
    test(u64::power_of_2(20) + 7, true);
    test(u64::power_of_2(21) + 17, true);
    test(u64::power_of_2(22) + 15, true);
    test(u64::power_of_2(23) + 9, true);
    test(u64::power_of_2(24) + 43, true);
    test(u64::power_of_2(25) + 35, true);
    test(u64::power_of_2(26) + 15, true);
    test(u64::power_of_2(27) + 29, true);
    test(u64::power_of_2(28) + 3, true);
    test(u64::power_of_2(29) + 11, true);
    test(u64::power_of_2(30) + 3, true);
    test(u64::power_of_2(31) + 11, true);
    test(u64::power_of_2(32) + 15, true);
    test(u64::power_of_2(33) + 17, true);
    test(u64::power_of_2(34) + 25, true);
    test(u64::power_of_2(35) + 53, true);
    test(u64::power_of_2(36) + 31, true);
    test(u64::power_of_2(37) + 9, true);
    test(u64::power_of_2(38) + 7, true);
    test(u64::power_of_2(39) + 23, true);
    test(u64::power_of_2(40) + 15, true);
    test(u64::power_of_2(41) + 27, true);
    test(u64::power_of_2(42) + 15, true);
    test(u64::power_of_2(43) + 29, true);
    test(u64::power_of_2(44) + 7, true);
    test(u64::power_of_2(45) + 59, true);
    test(u64::power_of_2(46) + 15, true);
    test(u64::power_of_2(47) + 5, true);
    test(u64::power_of_2(48) + 21, true);
    test(u64::power_of_2(49) + 69, true);
    test(u64::power_of_2(50) + 55, true);
    test(u64::power_of_2(51) + 21, true);
    test(u64::power_of_2(52) + 21, true);
    test(u64::power_of_2(53) + 5, true);
    test(u64::power_of_2(54) + 159, true);
    test(u64::power_of_2(55) + 3, true);
    test(u64::power_of_2(56) + 81, true);
    test(u64::power_of_2(57) + 9, true);
    test(u64::power_of_2(58) + 69, true);
    test(u64::power_of_2(59) + 131, true);
    test(u64::power_of_2(60) + 33, true);
    test(u64::power_of_2(61) + 15, true);
    test(u64::power_of_2(62) + 135, true);
    test(u64::power_of_2(63) + 29, true);

    test(11, true);
    test(101, true);
    test(1001, false);
    test(10001, false);
    test(100001, false);
    test(1000001, false);
    test(10000001, false);
    test(100000001, false);
    test(1000000001, false);
    test(10000000001, false);
    test(100000000001, false);
    test(1000000000001, false);
    test(10000000000001, false);
    test(100000000000001, false);
    test(1000000000000001, false);
    // - !n_is_strong_probabprime2_preinv_u64(n, ninv, 2, d) in n_is_probabprime_bpsw_u64
    test(10000000000000001, false);
    test(100000000000000001, false);
    test(1000000000000000001, false);
    test(10000000000000000001, false);

    test(11, true);
    test(101, true);
    test(1009, true);
    test(10007, true);
    test(100003, true);
    test(1000003, true);
    test(10000019, true);
    test(100000007, true);
    test(1000000007, true);
    test(10000000019, true);
    test(100000000003, true);
    // - 0 <= rem <= n in n_mulmod_precomp_u64
    test(1000000000039, true);
    test(10000000000037, true);
    test(100000000000031, true);
    test(1000000000000037, true);
    test(10000000000000061, true);
    test(100000000000000003, true);
    test(1000000000000000003, true);
    test(10000000000000000051, true);

    test(97, true);
    test(997, true);
    test(9973, true);
    test(99991, true);
    test(999983, true);
    test(9999991, true);
    test(99999989, true);
    test(999999937, true);
    test(9999999967, true);
    test(99999999977, true);
    test(999999999989, true);
    test(9999999999971, true);
    test(99999999999973, true);
    test(999999999999989, true);
    test(9999999999999937, true);
    test(99999999999999997, true);
    test(999999999999999989, true);
    test(9999999999999999961, true);

    test(1000003, true);
    test(5509785649208481923, true);
    test(8435959509307532899, true);
    test(15455033058440548141, true);
    test(13200125384684540339, true);
    // - a <= 1 || a == n - 1 n_is_strong_probabprime_precomp_u64
    test(6855593, true);
    // - !n_is_strong_probabprime_precomp_u64(n, npre, 2, d) in n_is_probabprime_bpsw_u64
    test(1050535501, false);
}

#[test]
fn test_is_prime() {
    is_prime_helper::<u8>();
    is_prime_helper::<u16>();
    is_prime_helper::<u32>();
    is_prime_helper::<u64>();
    is_prime_helper::<usize>();
}

fn is_prime_properties_helper_helper<T: IsPrime + PrimitiveUnsigned>(n: T) {
    let is_prime = n.is_prime();
    assert_eq!(is_prime_naive(n), is_prime);
}

fn is_prime_properties_helper_1<T: IsPrime + PrimitiveUnsigned>() {
    if T::WIDTH < u32::WIDTH {
        for n in exhaustive_unsigneds::<T>() {
            is_prime_properties_helper_helper(n);
        }
    } else {
        for n in exhaustive_unsigneds::<T>().take(10_000_000) {
            is_prime_properties_helper_helper(n);
        }
        unsigned_gen::<T>().test_properties(|n| {
            is_prime_properties_helper_helper(n);
        });
    }
}

fn is_prime_properties_helper_2<T: PrimitiveUnsigned, DT: From<T> + IsPrime + PrimitiveUnsigned>() {
    unsigned_pair_gen_var_50::<T>().test_properties(|(a, b)| {
        assert!(!(DT::from(a) * DT::from(b)).is_prime());
    });
}

#[test]
fn is_prime_properties() {
    is_prime_properties_helper_1::<u8>();
    is_prime_properties_helper_1::<u16>();
    is_prime_properties_helper_1::<u32>();
    is_prime_properties_helper_1::<u64>();
    is_prime_properties_helper_1::<usize>();

    is_prime_properties_helper_2::<u16, u32>();
    is_prime_properties_helper_2::<u32, u64>();
}
