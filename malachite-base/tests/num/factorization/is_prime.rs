// Copyright © 2026 Mikhail Hogrefe
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
use malachite_base::num::factorization::is_prime::{ll_is_prime, test_is_prime_tables};
use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime, Primes};
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_29, unsigned_gen_var_37, unsigned_pair_gen_var_50,
};
use malachite_base::test_util::num::factorization::is_lucas_probable_prime::*;
use malachite_base::test_util::num::factorization::is_prime::is_prime_naive;
use malachite_base::test_util::num::factorization::is_prime_flint_3_1_2::{
    u32_is_prime_flint_3_1_2, u64_is_prime_flint_3_1_2,
};
use malachite_base::test_util::num::factorization::is_strong_probable_prime::*;

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
    test(1114427291855351, true);
}

#[test]
fn test_test_is_prime_tables() {
    test_is_prime_tables();
}

#[test]
fn test_is_prime_strong_pseudoprimes() {
    // The smallest composite numbers that pass the strong probable prime test to each of the first
    // k prime bases, for k from 1 to 9 (the last two are equal), and to the first 12.
    for n in [
        2047u64,
        1373653,
        25326001,
        3215031751,
        2152302898747,
        3474749660383,
        341550071728321,
        3825123056546413051,
    ] {
        assert!(!n.is_prime());
        assert!(!u64_is_prime_flint_3_1_2(n));
    }
}

// Compares the primality test with the one that it replaced, on every input below 2^22 and on
// windows around the powers of 2 where the algorithm changes.
#[test]
fn test_is_prime_against_flint_3_1_2() {
    for n in 0..1u32 << 22 {
        let out = u32_is_prime_flint_3_1_2(n);
        assert_eq!(n.is_prime(), out, "{n}");
        assert_eq!(u64::from(n).is_prime(), out, "{n}");
    }
    for center in [1u64 << 31, 1 << 32, 1 << 62, 1 << 63] {
        for n in center - 100000..center + 100000 {
            assert_eq!(n.is_prime(), u64_is_prime_flint_3_1_2(n), "{n}");
            if let Ok(n) = u32::try_from(n) {
                assert_eq!(n.is_prime(), u32_is_prime_flint_3_1_2(n), "{n}");
            }
        }
    }
    for n in (u64::MAX - 200000)..=u64::MAX {
        assert_eq!(n.is_prime(), u64_is_prime_flint_3_1_2(n), "{n}");
    }
    for n in (u32::MAX - 200000)..=u32::MAX {
        assert_eq!(n.is_prime(), u32_is_prime_flint_3_1_2(n), "{n}");
    }
    // products (a + 1)(ka + 1), which the test handles separately
    for a in (1u64 << 30..(1 << 30) + 20000).chain(3000000000..3000020000) {
        for k in 2..=12 {
            if let Some(n) = (a + 1).checked_mul(k * a + 1) {
                assert_eq!(n.is_prime(), u64_is_prime_flint_3_1_2(n), "{n}");
                assert!(!n.is_prime());
            }
        }
    }
}

// The definitive check of the small-input paths: every u32, as a u32 and as a u64, against the
// sieve. Takes a minute or two in release.
#[test]
fn test_is_prime_all_u32() {
    let mut primes = u32::primes();
    let mut next_prime = primes.next();
    let mut count = 0;
    for n in 0..=u32::MAX {
        let expected = next_prime == Some(n);
        if expected {
            next_prime = primes.next();
            count += 1;
        }
        assert_eq!(n.is_prime(), expected, "{n}");
        assert_eq!(u64::from(n).is_prime(), expected, "{n}");
    }
    assert_eq!(next_prime, None);
    assert_eq!(count, 203280221);
}

#[test]
fn test_is_prime() {
    is_prime_helper::<u8>();
    is_prime_helper::<u16>();
    is_prime_helper::<u32>();
    is_prime_helper::<u64>();
    is_prime_helper::<usize>();
    is_prime_helper::<u128>();
}

#[test]
fn test_u128_is_prime() {
    for n in 0..100000u32 {
        assert_eq!(u128::from(n).is_prime(), n.is_prime());
    }
    let test = |n: u128, out| {
        assert_eq!(n.is_prime(), out);
    };
    // n < 2^64
    test(2, true);
    // n >= 2^64, and ll_is_prime settles n
    test(18446744073709551629, true);
    test(4533471823554859405148161, true);
    test(340282366920938463463374607431768211455, false);
    // - the n ± 1 tests leave n unsettled, so APR-CL proves it: the largest prime below 2^128
    // - q | n - 1
    // - q | n + 1
    // - the cofactor is a probable prime
    // - a base fails for some q in the Pocklington test, so the next base is tried
    // - m.get_bit(i) and !m.get_bit(i) in chain_full
    // - !k.get_bit(i) for the trailing bits in chain_mul, and later bits, some set
    // - t even and t odd in v_to_u
    // - D < 0
    // - (D / n) != -1 for a = 3
    // - B fails for some q in the Morrison test, so the next B is tried
    // - a cofactor of n ∓ 1 is a base-2 strong probable prime
    // - Fc / 2 is tested for feasibility, and the tests are infeasible
    // - F1 and F2 are both even in the combined test
    // - n needs a full primality proof
    test(340282366920938463463374607431768211297, true);
    // - the cofactor of n - 1 is proven prime
    // - F1^2 overflows, so F1^2 >= n
    test(618970019642690137449562111, true);
    // - a cofactor of n ∓ 1 is 1
    // - F1^3 >= n: the Brillhart, Lehmer, Selfridge test
    test(170141183460469231731687303715884105727, true);
    // - rem >= t and rem < t in Wide::sqrt_rem
    // - all sign combinations in Wide::cmp_signed
    // - Lenstra's search: i even and i odd, q < c1, s1 in range, d a square, a1 == 0, the chain
    //   ending, and no divisor found
    // - lcm(F1, F2)^3 > n: Lenstra's divisors in residue classes
    test(72535549176877750482370559, true);
    // - i even && a0 == 0 in Lenstra's search
    test(167329879230126280997564823109631, true);
    // - (F2 - 1)^3 > n: the improved n + 1 test
    test(110985345351181940559839359037079551, true);
    // - lcm(F1, F2)^2 > n
    test(15820279490848302930678856003092479, true);
    // Carmichael base-2 strong pseudoprimes above the Sorenson-Webster bound. Every base coprime to
    // the first fails the Pocklington test, so APR-CL settles it.
    test(3557725523452902604315321, false);
    test(3560180573160531764146441, false);
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
fn is_prime_properties_against_flint_3_1_2() {
    unsigned_gen::<u32>().test_properties(|n| {
        assert_eq!(n.is_prime(), u32_is_prime_flint_3_1_2(n));
    });
    unsigned_gen::<u64>().test_properties(|n| {
        assert_eq!(n.is_prime(), u64_is_prime_flint_3_1_2(n));
    });
    unsigned_pair_gen_var_50::<u32>().test_properties(|(a, b)| {
        let n = u64::from(a) * u64::from(b);
        assert_eq!(n.is_prime(), u64_is_prime_flint_3_1_2(n));
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
    is_prime_properties_helper_2::<u64, u128>();

    for n in exhaustive_unsigneds::<u128>().take(10_000_000) {
        is_prime_properties_helper_helper(n);
    }
    unsigned_gen::<u128>().test_properties(|n| {
        assert_eq!(n.is_prime(), n.is_probable_prime());
    });
    unsigned_gen_var_37().test_properties(|n| {
        assert!(n.is_prime());
    });
}

// The Sorenson–Webster bound, below which n_ll_is_prime's 13-base test proves primality
const SORENSON_WEBSTER_BOUND: u128 = 3317044064679887385961981;

fn ll_is_prime_check(n: u128) {
    let bpsw = is_strong_probable_prime_naive(n, 2) && is_lucas_probable_prime_naive(n);
    match ll_is_prime(n) {
        Some(true) => assert!(n < SORENSON_WEBSTER_BOUND && bpsw, "{n}"),
        Some(false) => assert!(!bpsw, "{n}"),
        None => assert!(
            n >= SORENSON_WEBSTER_BOUND && is_strong_probable_prime_naive(n, 2),
            "{n}"
        ),
    }
}

#[test]
fn test_ll_is_prime() {
    let test = |n: u128, out| {
        assert_eq!(ll_is_prime(n), out);
        ll_is_prime_check(n);
    };
    // - n.even()
    test(18446744073709551616, Some(false));
    // - a trial prime from 3 to 137 divides n
    test(18446744073709551621, Some(false));
    // - n < SORENSON_WEBSTER_BOUND
    // - u128::from(n2) >= d1 in div_3_by_2_normalized
    // - the correction loop runs in div_3_by_2_normalized
    // - ll_small_sqrmod subtracts m
    // - !(ebits >= 3 && b3 <= MAX_7TH_ROOT) in ll_small_powmod_triple
    // - exp.get_bit(bit) in ll_small_powmod_triple
    // - ebits < 6 in ll_small_2_powmod
    // - exp.get_bit(bit) in ll_small_2_powmod
    // - ll_small_sprp_stage2 breaks early
    // - all squarings fail in ll_small_sprp_stage2
    // - an odd base fails in ll_small_is_prime_mr
    test(18446744073709551617, Some(false));
    // - ebits >= 6 in ll_small_2_powmod
    // - ll_small_reduce2_precise subtracts m
    // - base 2 fails in ll_small_is_prime_mr
    test(18446744073709551619, Some(false));
    // the smallest prime above 2^64
    // - ebits >= 3 && b3 <= MAX_7TH_ROOT in ll_small_powmod_triple
    // - !exp.get_bit(ebits - 2) in ll_small_powmod_triple
    // - !exp.get_bit(ebits - 3) in ll_small_powmod_triple
    // - y == 1 in ll_small_sprp_stage2
    // - all bases pass in ll_small_is_prime_mr
    test(18446744073709551629, Some(true));
    // - exp.get_bit(ebits - 2) in ll_small_powmod_triple
    // - exp.get_bit(ebits - 3) in ll_small_powmod_triple
    test(9149585060559937601537, Some(true));
    // - n >= SORENSON_WEBSTER_BOUND && n.upper_half() < LL_SMALL_LIMIT_HIGH
    // - base_2_only in ll_small_is_prime_mr
    test(4533471823554859405148161, None);
    // - n.upper_half() >= LL_SMALL_LIMIT_HIGH
    // - the slow base-2 test fails
    test(8665580274997661924293869569, Some(false));
    // - a trial prime from 139 to 313 divides n
    test(673439381371246869545123577857, Some(false));
    // - the slow base-2 test passes
    test(34662321099990647697175478273, None);
    // a multiple of 313, the largest trial prime
    test(313 * 58935284580541696407, Some(false));
    // psi_12 is caught by base 41; psi_13 is the bound itself, so only the base-2 test applies
    test(318665857834031151167461, Some(false));
    test(3317044064679887385961981, None);
    test(3317044064679887385961979, Some(false));
    test(618970019642690137449562111, None);
    test(170141183460469231731687303715884105727, None);
    test(340282366920938463463374607431768211297, None);
    test(340282366920938463463374607431768211455, Some(false));
    // The windows above 2^64, around the bound, around the start of the slow base-2 path, and below
    // 2^128
    for n in (1u128 << 64)..(1u128 << 64) + 100000 {
        ll_is_prime_check(n);
    }
    for n in SORENSON_WEBSTER_BOUND - 50000..SORENSON_WEBSTER_BOUND + 50000 {
        ll_is_prime_check(n);
    }
    let slow = 357913941u128 << 64;
    for n in slow - 50000..slow + 50000 {
        ll_is_prime_check(n);
    }
    for n in u128::MAX - 100000..=u128::MAX {
        ll_is_prime_check(n);
    }
}

#[test]
fn ll_is_prime_properties() {
    unsigned_gen::<u128>().test_properties(|n| {
        if n >> 64 != 0 {
            ll_is_prime_check(n);
        }
    });
    unsigned_gen_var_29::<u64>().test_properties(|p| {
        for q in [18446744073709551557u64, 4294967291, 2305843009213693951] {
            let n = u128::from(p) * u128::from(q);
            if n >> 64 != 0 {
                assert_ne!(ll_is_prime(n), Some(true));
            }
        }
    });
}
