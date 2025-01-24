// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Contributed to the GNU project by Torbjörn Granlund and Marco Bodrato.
//
//      Copyright © 1998-2012, 2013, 2015-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::div::limbs_hensel_div_limb_in_place;
use crate::natural::arithmetic::div_exact::{
    limbs_modular_div_schoolbook_in_place, limbs_modular_invert_limb,
};
use crate::natural::arithmetic::div_round::double_cmp;
use crate::natural::arithmetic::factorial::{bit_to_n, limbs_odd_factorial, log_n_max};
use crate::natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::mul::product_of_limbs::limbs_product;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::Natural;
use crate::platform::{
    Limb, CENTRAL_BINOMIAL_2FAC_TABLE, ODD_CENTRAL_BINOMIAL_OFFSET,
    ODD_CENTRAL_BINOMIAL_TABLE_LIMIT, ODD_FACTORIAL_EXTTABLE_LIMIT, ODD_FACTORIAL_TABLE_LIMIT,
    ODD_FACTORIAL_TABLE_MAX, ONE_LIMB_ODD_CENTRAL_BINOMIAL_INVERSE_TABLE,
    ONE_LIMB_ODD_CENTRAL_BINOMIAL_TABLE, ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE,
    ONE_LIMB_ODD_FACTORIAL_TABLE, TABLE_2N_MINUS_POPC_2N,
};
use alloc::vec::Vec;
use core::cmp::{max, min, Ordering::*};
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, BinomialCoefficient, DivAssignMod, DivExact, Parity, PowerOf2, Square,
    WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::factorization::prime_sieve::limbs_prime_sieve_u32;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::factorization::prime_sieve::limbs_prime_sieve_u64;
use malachite_base::num::factorization::prime_sieve::{id_to_n, limbs_prime_sieve_size, n_to_bit};
use malachite_base::num::logic::traits::{CountOnes, LeadingZeros, SignificantBits};

// This is similar to `mulfunc` from `mpz/bin_uiui.c`, GMP 6.2.1.
const fn apply_mul_func(n: Limb, m: Limb) -> Limb {
    match n {
        1 => m,
        2 => (m | 1) * ((m + 1) >> 1),
        3 => {
            let m01 = (m * (m + 1)) >> 1;
            let m2 = m + 2;
            m01 * m2
        }
        4 => {
            let m03 = (m * (m + 3)) >> 1;
            m03 * (m03 + 1)
        }
        5 => {
            let m03 = (m * (m + 3)) >> 1;
            let m034 = m03 * (m + 4);
            (m03 + 1) * m034
        }
        6 => {
            let m05 = m * (m + 5);
            let m1234 = ((m05 + 5) * (m05 + 5)) >> 3;
            m1234 * (m05 >> 1)
        }
        7 => {
            let m05 = m * (m + 5);
            let m1234 = ((m05 + 5) * (m05 + 5)) >> 3;
            let m056 = (m05 * (m + 6)) >> 1;
            m1234 * m056
        }
        8 => {
            let m07 = m * (m + 7);
            let m0257 = (m07 * (m07 + 10)) >> 3;
            let m1346 = m07 + 9 + m0257;
            m0257 * m1346
        }
        _ => panic!(),
    }
}

const M: Limb = 8;

const SOME_THRESHOLD: usize = 20;

// This is equivalent to `mpz_bdiv_bin_uiui` from `mpz/bin_uiui.c`, GMP 6.2.1, where a `Vec` of
// limbs is returned.
pub_test! {limbs_binomial_coefficient_limb_limb_bdiv(n: Limb, k: Limb) -> Vec<Limb> {
    assert!(k > Limb::wrapping_from(ODD_FACTORIAL_TABLE_LIMIT));
    let max_n = 1 + usize::exact_from(n >> Limb::LOG_WIDTH);
    let alloc = min(
        SOME_THRESHOLD - 1 + max((3 * max_n) >> 1, SOME_THRESHOLD),
        usize::exact_from(k),
    ) + 1;
    let mut big_scratch = vec![0; alloc + SOME_THRESHOLD + 1];
    let (ns, ks) = big_scratch.split_at_mut(alloc);
    let n_max = Limb::wrapping_from(log_n_max(n));
    assert!(n_max <= M);
    let mut k_max = Limb::wrapping_from(log_n_max(k));
    assert!(k_max <= M);
    assert!(k >= M);
    assert!(n >= k);
    let mut i = n - k + 1;
    ns[0] = 1;
    let mut n_len = 1;
    let mut num_fac = 1;
    let mut j = Limb::wrapping_from(ODD_FACTORIAL_TABLE_LIMIT + 1);
    let mut jjj = ODD_FACTORIAL_TABLE_MAX;
    assert_eq!(
        ONE_LIMB_ODD_FACTORIAL_TABLE[ODD_FACTORIAL_TABLE_LIMIT],
        ODD_FACTORIAL_TABLE_MAX
    );
    loop {
        ks[0] = jjj;
        let mut k_len = 1;
        let mut t = k + 1 - j;
        k_max = min(k_max, t);
        while k_max != 0 && k_len < SOME_THRESHOLD {
            jjj = apply_mul_func(k_max, j);
            j += k_max;
            jjj >>= jjj.trailing_zeros();
            let cy = limbs_slice_mul_limb_in_place(&mut ks[..k_len], jjj);
            ks[k_len] = cy;
            if cy != 0 {
                k_len += 1;
            }
            t = k + 1 - j;
            k_max = min(k_max, t);
        }
        num_fac = j - num_fac;
        while num_fac != 0 {
            let n_max_now = min(n_max, num_fac);
            let mut iii = apply_mul_func(n_max_now, i);
            i.wrapping_add_assign(n_max_now);
            iii >>= iii.trailing_zeros();
            let carry = limbs_slice_mul_limb_in_place(&mut ns[..n_len], iii);
            ns[n_len] = carry;
            if carry != 0 {
                n_len += 1;
            }
            num_fac -= n_max_now;
        }
        if ns[n_len - 1] >= ks[k_len - 1] {
            n_len += 1;
        }
        n_len -= k_len;
        let d_inv = limbs_modular_invert_limb(ks[0]);
        limbs_modular_div_schoolbook_in_place(ns, &ks[..min(k_len, n_len)], d_inv.wrapping_neg());
        limbs_neg_in_place(&mut ns[..n_len]);
        if k_max == 0 {
            break;
        }
        num_fac = j;
        jjj = apply_mul_func(k_max, j);
        j += k_max;
        jjj >>= jjj.trailing_zeros();
    }
    // Put back the right number of factors of 2.
    let ones = CountOnes::count_ones(n - k) + CountOnes::count_ones(k) - CountOnes::count_ones(n);
    if ones != 0 {
        assert!(ones < Limb::WIDTH);
        let carry = limbs_slice_shl_in_place(&mut ns[..n_len], ones);
        ns[n_len] = carry;
        if carry != 0 {
            n_len += 1;
        }
    }
    if ns[n_len - 1] == 0 {
        n_len -= 1;
    }
    let mut ns = big_scratch;
    ns.truncate(n_len);
    ns
}}

// Number of factors-of-2 removed by the corresponding mulN function.
//
// This is equivalent to `tcnttab` from `mpz/bin_uiui.c`, GMP 6.2.1.
const TCNT_TAB: [u8; 8] = [0, 1, 1, 2, 2, 4, 4, 6];

// This is equivalent to `mpz_smallk_bin_uiui` from `mpz/bin_uiui.c`, GMP 6.2.1, where a `Vec` of
// limbs is returned.
pub_test! {limbs_binomial_coefficient_limb_limb_small_k(n: Limb, k: Limb) -> Vec<Limb> {
    let n_max = Limb::wrapping_from(log_n_max(n));
    let mut n_max = min(n_max, M);
    let mut i = n - k + 1;
    let k_u = usize::exact_from(k);
    assert!(k_u <= ODD_FACTORIAL_TABLE_LIMIT);
    let mut i2_count = TABLE_2N_MINUS_POPC_2N[(k_u >> 1) - 1];
    let factorial_inverse = ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE[k_u - 2];
    if n_max >= k {
        return vec![
            (apply_mul_func(k, i).wrapping_mul(factorial_inverse))
                >> (i2_count - TCNT_TAB[k_u - 1]),
        ];
    }
    let alloc =
        usize::exact_from((n.significant_bits() * u64::exact_from(k)) >> Limb::LOG_WIDTH) + 3;
    let mut out = vec![0; alloc];
    out[0] = apply_mul_func(n_max, i);
    let mut out_len = 1;
    i += n_max;
    i2_count -= TCNT_TAB[usize::exact_from(n_max - 1)];
    let mut num_fac = k - n_max;
    while num_fac != 0 {
        n_max = min(n_max, num_fac);
        let iii = apply_mul_func(n_max, i);
        i.wrapping_add_assign(n_max);
        i2_count -= TCNT_TAB[usize::exact_from(n_max - 1)];
        let carry = limbs_slice_mul_limb_in_place(&mut out[..out_len], iii);
        out[out_len] = carry;
        if carry != 0 {
            out_len += 1;
        }
        num_fac -= n_max;
    }
    assert!(out_len < alloc);
    limbs_hensel_div_limb_in_place(
        &mut out[..out_len],
        ONE_LIMB_ODD_FACTORIAL_TABLE[k_u],
        factorial_inverse,
        u64::from(i2_count),
    );
    while *out.last().unwrap() == 0 {
        out.pop();
    }
    out
}}

// Tabulate factorials (k!/2^t)^(-1) mod B (where t is chosen such that k!/2^t is odd).
//
// This is equivalent to `bc_bin_uiui` from `mpz/bin_uiui.c`, GMP 6.2.1, where a single limb is
// returned.
pub_test! {limbs_binomial_coefficient_limb_limb_basecase(n: Limb, k: Limb) -> Limb {
    assert!(n <= ODD_FACTORIAL_EXTTABLE_LIMIT as Limb);
    assert!(n >= k + 2);
    assert!(k >= 2);
    let n = usize::wrapping_from(n);
    let k = usize::wrapping_from(k);
    let diff = n - k;
    (ONE_LIMB_ODD_FACTORIAL_TABLE[n]
        .wrapping_mul(ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE[k - 2])
        .wrapping_mul(ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE[diff - 2]))
        << (TABLE_2N_MINUS_POPC_2N[(n >> 1) - 1]
            - TABLE_2N_MINUS_POPC_2N[(k >> 1) - 1]
            - TABLE_2N_MINUS_POPC_2N[(diff >> 1) - 1])
}}

pub(crate) const BIN_UIUI_RECURSIVE_SMALLDC: bool = Limb::WIDTH > u32::WIDTH;

// Recursively exploit the relation bin(n, k) = bin(n, k >> 1) * bin(n - k >> 1, k - k >> 1) /
// bin(k, k >> 1).
//
// Values for binomial(k, k >> 1) that fit in a limb are precomputed (with inverses).
//
// This is equivalent to `mpz_smallkdc_bin_uiui` from `mpz/bin_uiui.c`, GMP 6.2.1, where a `Vec` of
// limbs is returned.
pub_test! {limbs_binomial_coefficient_limb_limb_small_k_divide_and_conquer(
    mut n: Limb,
    mut k: Limb,
) -> Vec<Limb> {
    let half_k = k >> 1;
    let mut out = if !BIN_UIUI_RECURSIVE_SMALLDC || half_k <= ODD_FACTORIAL_TABLE_LIMIT as Limb {
        limbs_binomial_coefficient_limb_limb_small_k(n, half_k)
    } else {
        limbs_binomial_coefficient_limb_limb_small_k_divide_and_conquer(n, half_k)
    };
    k -= half_k;
    n -= half_k;
    let mut out_len = out.len();
    if n <= ODD_FACTORIAL_EXTTABLE_LIMIT as Limb {
        out.push(0);
        let carry = limbs_slice_mul_limb_in_place(
            &mut out[..out_len],
            limbs_binomial_coefficient_limb_limb_basecase(n, k),
        );
        out[out_len] = carry;
        if carry != 0 {
            out_len += 1;
        }
    } else {
        let t = if !BIN_UIUI_RECURSIVE_SMALLDC || k <= ODD_FACTORIAL_TABLE_LIMIT as Limb {
            limbs_binomial_coefficient_limb_limb_small_k(n, k)
        } else {
            limbs_binomial_coefficient_limb_limb_small_k_divide_and_conquer(n, k)
        };
        let out_copy = out[..out_len].to_vec();
        out = limbs_mul(&out_copy, &t);
        out_len = out.len();
    }
    let k_u = usize::exact_from(k);
    let mut shift = CENTRAL_BINOMIAL_2FAC_TABLE[k_u - ODD_CENTRAL_BINOMIAL_OFFSET];
    if k != half_k {
        // k was odd
        shift -= 1;
    }
    let k_offset = k_u - ODD_CENTRAL_BINOMIAL_OFFSET;
    limbs_hensel_div_limb_in_place(
        &mut out[..out_len],
        ONE_LIMB_ODD_CENTRAL_BINOMIAL_TABLE[k_offset],
        ONE_LIMB_ODD_CENTRAL_BINOMIAL_INVERSE_TABLE[k_offset],
        shift,
    );
    while *out.last().unwrap() == 0 {
        out.pop();
    }
    out
}}

// Returns an approximation of the sqare root of x. It gives:
// ```
//   limb_apprsqrt (x) ^ 2 <= x < (limb_apprsqrt (x)+1) ^ 2
// ```
// or
// ```
// x <= limb_apprsqrt (x) ^ 2 <= x * 9/8
// ```
fn limbs_approx_sqrt(x: Limb) -> Limb {
    assert!(x > 2);
    let s = (Limb::WIDTH - LeadingZeros::leading_zeros(x)) >> 1;
    (Limb::power_of_2(s) + (x >> s)) >> 1
}

pub(crate) const BIN_GOETGHELUCK_THRESHOLD: Limb = 512;

// Implementation of the algorithm by P. Goetgheluck, "Computing Binomial Coefficients", The
// American Mathematical Monthly, Vol. 94, No. 4 (April 1987), pp. 360-365.
//
// This is equivalent to `mpz_goetgheluck_bin_uiui` from `mpz/bin_uiui.c`, GMP 6.2.1, where a `Vec`
// of limbs is returned.
pub_test! {
#[allow(clippy::useless_conversion)]
limbs_binomial_coefficient_limb_limb_goetgheluck(n: Limb, k: Limb) -> Vec<Limb> {
    assert!(BIN_GOETGHELUCK_THRESHOLD >= 13);
    assert!(n >= 25);
    let n_64 = u64::from(n);
    let k_64 = u64::from(k);
    let mut sieve = vec![0; limbs_prime_sieve_size::<Limb>(n_64)];
    #[cfg(feature = "32_bit_limbs")]
    let count = limbs_prime_sieve_u32(&mut sieve, n_64) + 1;
    #[cfg(not(feature = "32_bit_limbs"))]
    let count = limbs_prime_sieve_u64(&mut sieve, n) + 1;
    let mut factors = vec![0; usize::exact_from(count / log_n_max(n) + 1)];
    let mut max_prod = Limb::MAX / n;
    // Handle primes = 2, 3 separately.
    let mut prod = Limb::power_of_2(
        CountOnes::count_ones(n - k) + CountOnes::count_ones(k) - CountOnes::count_ones(n),
    );
    let mut j = 0;
    let mut a = n;
    let mut b = k;
    let mut mb = 0;
    if prod > max_prod {
        // would only happen for very large outputs
        factors[j] = prod;
        j += 1;
        prod = 1;
    }
    while a >= 3 {
        mb += b.div_assign_mod(3);
        let ma = a.div_assign_mod(3);
        if ma < mb {
            mb = 1;
            prod *= 3;
        } else {
            mb = 0;
        }
    }
    // Accumulate prime factors from 5 to n / 2
    let s = n_to_bit(u64::from(limbs_approx_sqrt(n)));
    assert!(bit_to_n(s + 1).square() > n_64);
    let half_n_bit = n_to_bit(n_64 >> 1);
    assert!(s <= half_n_bit);
    let mut index = 0;
    let mut mask = 1;
    for i in 1..=s + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::exact_from(id_to_n(i));
            let mut a = n;
            let mut b = k;
            let mut mb = 0;
            if prod > max_prod {
                factors[j] = prod;
                j += 1;
                prod = 1;
            }
            while a >= prime {
                mb += b.div_assign_mod(prime);
                let ma = a.div_assign_mod(prime);
                if ma < mb {
                    mb = 1;
                    prod *= prime;
                } else {
                    mb = 0;
                }
            }
        }
        mask <<= 1;
        if mask == 0 {
            mask = 1;
            index += 1;
        }
    }
    assert!(max_prod <= Limb::MAX >> 1);
    max_prod <<= 1;
    for i in s + 2..=half_n_bit + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::exact_from(id_to_n(i));
            if n % prime < k % prime {
                if prod > max_prod {
                    factors[j] = prod;
                    j += 1;
                    prod = prime;
                } else {
                    prod *= prime;
                }
            }
        }
        mask <<= 1;
        if mask == 0 {
            mask = 1;
            index += 1;
        }
    }
    max_prod >>= 1;
    // Store primes from (n-k)+1 to n
    let n_bit = n_to_bit(n_64);
    let n_minus_k_bit = n_to_bit(n_64 - k_64);
    assert!(n_minus_k_bit < n_bit);
    let i = n_minus_k_bit + 1;
    let mut index = usize::exact_from(i >> Limb::LOG_WIDTH);
    let mut mask = Limb::power_of_2(i & Limb::WIDTH_MASK);
    for i in i + 1..=n_bit + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::exact_from(id_to_n(i));
            if prod > max_prod {
                factors[j] = prod;
                j += 1;
                prod = prime;
            } else {
                prod *= prime;
            }
        }
        mask <<= 1;
        if mask == 0 {
            mask = 1;
            index += 1;
        }
    }
    assert_ne!(j, 0);
    factors[j] = Limb::wrapping_from(prod);
    j += 1;
    let mut r = vec![0; j];
    limbs_product(&mut r, &mut factors[..j]);
    while *r.last().unwrap() == 0 {
        r.pop();
    }
    r
}}

const BIN_UIUI_ENABLE_SMALLDC: bool = true;

// This is equivalent to `mpz_bin_uiui` from `mpz/bin_uiui.c`, GMP 6.2.1, where a `Natural` is
// returned.
pub_test! {binomial_coefficient_limb_limb(n: Limb, mut k: Limb) -> Natural {
    if n < k {
        Natural::ZERO
    } else {
        // Rewrite bin(n, k) as bin(n, n - k) if that is smaller.
        k = min(k, n - k);
        if k == 0 {
            Natural::ONE
        } else if k == 1 {
            Natural::from(n)
        } else if n <= ODD_FACTORIAL_EXTTABLE_LIMIT as Limb {
            // k >= 2, n >= 4
            Natural::from(limbs_binomial_coefficient_limb_limb_basecase(n, k))
        } else if k <= ODD_FACTORIAL_TABLE_LIMIT as Limb {
            Natural::from_owned_limbs_asc(limbs_binomial_coefficient_limb_limb_small_k(n, k))
        } else if BIN_UIUI_ENABLE_SMALLDC
            && k <= (if BIN_UIUI_RECURSIVE_SMALLDC {
                ODD_CENTRAL_BINOMIAL_TABLE_LIMIT
            } else {
                ODD_FACTORIAL_TABLE_LIMIT
            } as Limb)
                << 1
        {
            Natural::from_owned_limbs_asc(
                limbs_binomial_coefficient_limb_limb_small_k_divide_and_conquer(n, k),
            )
        } else if k >= BIN_GOETGHELUCK_THRESHOLD && k > n >> 4 {
            // k > ODD_FACTORIAL_TABLE_LIMIT
            Natural::from_owned_limbs_asc(limbs_binomial_coefficient_limb_limb_goetgheluck(n, k))
        } else {
            Natural::from_owned_limbs_asc(limbs_binomial_coefficient_limb_limb_bdiv(n, k))
        }
    }
}}

// Computes r = n * (n + (2 * k - 1)) / 2.
//
// It uses a square instead of a product, computing r = ((n + k - 1) ^ 2 + n - (k - 1) ^ 2) / 2 As a
// side effect, sets t = n + k - 1.
//
// This is equivalent to `mpz_hmul_nbnpk` from `mpz/bin_ui.c`, GMP 6.2.1.
fn binomial_coefficient_hmul_nbnpk(n: &Natural, mut k: Limb) -> Natural {
    assert_ne!(k, 0);
    assert_ne!(*n, 0u32);
    k -= 1;
    (((n + Natural::from(k)).square() + n) >> 1u32)
        - Natural::from(k + (k & 1)) * Natural::from(k >> 1)
}

// This is equivalent to `rek_raising_fac4` from `mpz/bin_ui.c`, GMP 6.2.1.
fn binomial_coefficient_raising_factorial_4_rec(
    r: &mut Natural,
    p: &mut Natural,
    big_p: &mut Natural,
    k: Limb,
    lk: Limb,
) {
    assert!(k >= lk);
    if k - lk < 5 {
        for i in (lk + 1..=k).rev() {
            let four_i = i << 2;
            *p += Natural::from(four_i + 2);
            big_p.add_mul_assign(&*p, Natural::from(four_i));
            *big_p -= Natural::from(i);
            *r *= &*big_p;
        }
    } else {
        let m = ((k + lk) >> 1) + 1;
        binomial_coefficient_raising_factorial_4_rec(r, p, big_p, k, m);
        let four_m = m << 2;
        *p += Natural::from(four_m + 2);
        big_p.add_mul_assign(&*p, Natural::from(four_m));
        *big_p -= Natural::from(m);
        let mut t = big_p.clone();
        binomial_coefficient_raising_factorial_4_rec(&mut t, p, big_p, m - 1, lk);
        *r *= t;
    }
}

// This is equivalent to `mpz_raising_fac4` from `mpz/bin_ui.c`, GMP 6.2.1, where r is returned.
fn binomial_coefficient_raising_factorial_4(mut n: Natural, mut k: Limb) -> Natural {
    assert!(k >= 2);
    n += Natural::ONE;
    let mut r = Natural::ZERO;
    if k.odd() {
        r = n.clone();
        n += Natural::ONE;
    }
    k >>= 1;
    let mut p = binomial_coefficient_hmul_nbnpk(&n, k);
    if k.odd() {
        if r != 0u32 {
            r *= &p;
        } else {
            r = p.clone();
        }
        p += Natural::from(k - 1);
    }
    k >>= 1;
    if k == 0 {
        return r;
    }
    let mut t = binomial_coefficient_hmul_nbnpk(&p, k);
    if r != 0u32 {
        r *= &t;
    } else {
        r = t.clone();
    }
    if k > 1 {
        p -= Natural::from(k);
        binomial_coefficient_raising_factorial_4_rec(&mut r, &mut p, &mut t, k - 1, 0);
    }
    r
}

// This is equivalent to `mpz_bin_ui` from `mpz/bin_ui.c`, GMP 6.2.1, where n is non-negative, n >=
// k, k <= n - k, and r is returned.
fn binomial_coefficient_helper(n: Natural, k: Limb) -> Natural {
    assert_ne!(k, 0);
    if k < 2 {
        n
    } else if let Ok(small_n) = Limb::try_from(&n) {
        binomial_coefficient_limb_limb(small_n, k)
    } else {
        (binomial_coefficient_raising_factorial_4(n - Natural::from(k), k)
            >> (k - (k >> 1) - (k >> 2))
                .checked_sub(Limb::wrapping_from(CountOnes::count_ones(k)))
                .unwrap())
        .div_exact(Natural::from_owned_limbs_asc(limbs_odd_factorial(
            usize::exact_from(k),
            false,
        )))
    }
}

impl BinomialCoefficient for Natural {
    /// Computes the binomial coefficient of two [`Natural`]s, taking both by value.
    ///
    /// $$
    /// f(n, k) =binom{n}{k} =frac{n!}{k!(n-k)!}.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BinomialCoefficient;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::binomial_coefficient(Natural::from(4u32), Natural::from(0u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(Natural::from(4u32), Natural::from(1u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(Natural::from(4u32), Natural::from(2u32)),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(Natural::from(4u32), Natural::from(3u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(Natural::from(4u32), Natural::from(4u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(Natural::from(10u32), Natural::from(5u32)),
    ///     252
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(Natural::from(100u32), Natural::from(50u32)).to_string(),
    ///     "100891344545564193334812497256"
    /// );
    /// ```
    fn binomial_coefficient(n: Natural, mut k: Natural) -> Natural {
        if k > n {
            return Natural::ZERO;
        }
        if k == 0u32 || n == k {
            return Natural::ONE;
        }
        if double_cmp(&k, &n) == Greater {
            k = &n - &k;
        }
        binomial_coefficient_helper(n, Limb::try_from(&k).expect("k is too large"))
    }
}

impl<'a> BinomialCoefficient<&'a Natural> for Natural {
    /// Computes the binomial coefficient of two [`Natural`]s, taking both by reference.
    ///
    /// $$
    /// f(n, k) =binom{n}{k} =frac{n!}{k!(n-k)!}.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BinomialCoefficient;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(0u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(1u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(2u32)),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(3u32)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(4u32), &Natural::from(4u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(10u32), &Natural::from(5u32)),
    ///     252
    /// );
    /// assert_eq!(
    ///     Natural::binomial_coefficient(&Natural::from(100u32), &Natural::from(50u32))
    ///         .to_string(),
    ///     "100891344545564193334812497256"
    /// );
    /// ```
    fn binomial_coefficient(n: &'a Natural, k: &'a Natural) -> Natural {
        if k > n {
            return Natural::ZERO;
        }
        if *k == 0u32 || n == k {
            return Natural::ONE;
        }
        let k = if double_cmp(k, n) == Greater {
            Limb::try_from(&(n - k))
        } else {
            Limb::try_from(k)
        }
        .expect("k is too large");
        binomial_coefficient_helper(n.clone(), k)
    }
}
