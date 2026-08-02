// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `limb_apprsqrt`, `mpz_2multiswing_1`, `mpz_oddfac_1`, `mpz_fac_ui`, and `mpz_2fac_ui`
//      contributed to the GNU project by Marco Bodrato.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mul::product_of_limbs::limbs_product;
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::{LIMB_MAX_DIV_3, Natural, bit_to_limb_count_floor};
use crate::platform::{
    Limb, NTH_ROOT_NUMB_MASK_TABLE, ODD_DOUBLEFACTORIAL_TABLE_LIMIT, ODD_DOUBLEFACTORIAL_TABLE_MAX,
    ODD_FACTORIAL_TABLE_LIMIT, ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE, ONE_LIMB_ODD_FACTORIAL_TABLE,
    TABLE_2N_MINUS_POPC_2N, TABLE_LIMIT_2N_MINUS_POPC_2N,
};
use alloc::vec::Vec;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DoubleFactorial, Factorial, Gcd, Multifactorial, Parity, Pow, PowerOf2, Square,
    Subfactorial, XMulYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, WrappingFrom};
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::factorization::prime_sieve::limbs_prime_sieve_u32;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::factorization::prime_sieve::limbs_prime_sieve_u64;
use malachite_base::num::factorization::prime_sieve::{id_to_n, limbs_prime_sieve_size, n_to_bit};
use malachite_base::num::logic::traits::{BitAccess, CountOnes, NotAssign, SignificantBits};

const ODD_DOUBLEFACTORIAL_TABLE_LIMIT_PLUS_1: usize = ODD_DOUBLEFACTORIAL_TABLE_LIMIT + 1;

private_test_fn! {subfactorial_naive(n: u64) -> Natural {
    let mut f = Natural::ONE;
    let mut b = true;
    for i in 1..=n {
        f *= Natural::from(i);
        if b {
            f -= Natural::ONE;
        } else {
            f += Natural::ONE;
        }
        b.not_assign();
    }
    f
}}

// The number of pairs a leaf of the binary-splitting recursion handles iteratively.
const SUBFACTORIAL_SPLIT_LEAF_PAIRS: u64 = 32;

// Below this, the naive iterative algorithm is faster than binary splitting. The crossover is
// shallow - the two algorithms are within a factor of 2 of each other from about n = 512 to n =
// 2048 - so this does not need to be precise. Measured with 64-bit limbs on Apple Silicon; both
// paths are correct at any n, so an untuned width only costs a constant factor near the crossover.
const SUBFACTORIAL_SPLIT_THRESHOLD: u64 = 1024;

// The subfactorial satisfies !n = n * !(n - 1) + (-1) ^ n. Fusing two consecutive steps, for even
// k,
//
// !(k + 1) = k * (k + 1) * !(k - 1) + k,
//
// so the affine map x -> k(k + 1)x + k sends !(k - 1) to !(k + 1), and both of its coefficients are
// positive, which keeps all intermediate values `Natural`s. Applying x -> ax + b and then x -> cx +
// d is the same as applying x -> (ca)x + (cb + d), so the maps for consecutive pairs may be
// combined by binary splitting: a few multiplications of large, similarly-sized numbers replace the
// naive algorithm's many multiplications of a large number by a small one.
//
// `subfactorial_split` returns the coefficients (a, b) of the combined map for the pairs (lo, lo +
// 1), (lo + 2, lo + 3), ..., (hi, hi + 1), where lo and hi are even and lo <= hi.
fn subfactorial_split(lo: u64, hi: u64) -> (Natural, Natural) {
    let pairs = ((hi - lo) >> 1) + 1;
    if pairs <= SUBFACTORIAL_SPLIT_LEAF_PAIRS {
        let mut a = Natural::ONE;
        let mut b = Natural::ZERO;
        for i in 0..pairs {
            let k = lo + (i << 1);
            let m = Natural::from(u128::from(k) * u128::from(k + 1));
            a *= &m;
            b *= m;
            b += Natural::from(k);
        }
        (a, b)
    } else {
        // The left range gets the extra pair when the count is odd, since its values are smaller.
        let mid = lo + ((pairs - (pairs >> 1)) << 1);
        let (a_lo, b_lo) = subfactorial_split(lo, mid - 2);
        let (a_hi, mut b_hi) = subfactorial_split(mid, hi);
        let a = &a_hi * a_lo;
        b_hi.add_mul_assign(a_hi, b_lo);
        (a, b_hi)
    }
}

// Since !1 = 0, applying the combined map of the pairs (2, 3), (4, 5), ..., (n - 1, n) to it gives
// !n = a * 0 + b = b, so the a-coefficient of the leftmost part of the range is never needed. This
// function computes b alone, skipping the a-products along the left spine of the recursion.
fn subfactorial_split_only_b(lo: u64, hi: u64) -> Natural {
    let pairs = ((hi - lo) >> 1) + 1;
    if pairs <= SUBFACTORIAL_SPLIT_LEAF_PAIRS {
        let mut b = Natural::ZERO;
        for i in 0..pairs {
            let k = lo + (i << 1);
            b *= Natural::from(u128::from(k) * u128::from(k + 1));
            b += Natural::from(k);
        }
        b
    } else {
        let mid = lo + ((pairs - (pairs >> 1)) << 1);
        let b_lo = subfactorial_split_only_b(lo, mid - 2);
        let (a_hi, mut b_hi) = subfactorial_split(mid, hi);
        b_hi.add_mul_assign(a_hi, b_lo);
        b_hi
    }
}

// Returns an approximation of the square root of x.
//
// It gives:
// ```
// limb_apprsqrt(x) ^ 2 <= x < (limb_apprsqrt(x) + 1) ^ 2
// ```
// or
// ```
// x <= limb_apprsqrt(x) ^ 2 <= x * 9 / 8
// ```
//
// This is equivalent to `limb_apprsqrt` in `mpz/oddfac_1.c`, GMP 6.2.1.
fn limbs_approx_sqrt(x: u64) -> u64 {
    assert!(x > 2);
    let s = x.significant_bits() >> 1;
    (u64::power_of_2(s) + (x >> s)) >> 1
}

pub(crate) const fn bit_to_n(bit: u64) -> u64 {
    (bit * 3 + 4) | 1
}

// `limbs_2_multiswing_odd` computes the odd part of the 2-multiswing factorial of the parameter n.
// The result x is an odd positive integer so that multiswing(n, 2) = x * 2 ^ a.
//
// The algorithm is described by Peter Luschny in "Divide, Swing and Conquer the Factorial!".
//
// The pointer sieve points to `limbs_prime_sieve_size(n)` limbs containing a bit array where primes
// are marked as 0. Enough limbs must be pointed by `factors`.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpz_2multiswing_1` from `mpz/oddfac_1.c`, GMP 6.2.1, where `x_and_sieve`
// is provided as a single slice, allowing the sieve to be overwritten.
#[allow(clippy::useless_conversion)]
fn limbs_2_multiswing_odd(
    x_and_sieve: &mut [Limb],
    x_len: usize,
    mut n: Limb,
    factors: &mut [Limb],
) -> usize {
    assert!(n > 25);
    let mut prod = if n.odd() { n } else { 1 };
    n.clear_bit(0);
    let max_prod = Limb::MAX / (n - 1);
    // Handle prime = 3 separately
    let mut j = 0;
    if prod > max_prod {
        factors[j] = prod;
        j += 1;
        prod = 1;
    }
    let mut q = n;
    while q >= 3 {
        q /= 3;
        if q.odd() {
            prod *= 3;
        }
    }
    let limb_n = n;
    let n = u64::exact_from(n);
    // Swing primes from 5 to n / 3
    let mut s = limbs_approx_sqrt(n);
    assert!(s >= 5);
    s = n_to_bit(s);
    assert!(bit_to_n(s + 1).square() > n);
    assert!(s < n_to_bit(n / 3));
    let start = const { n_to_bit(5) };
    let mut index = bit_to_limb_count_floor(start);
    let mut mask = Limb::power_of_2(start & Limb::WIDTH_MASK);
    let sieve = &mut x_and_sieve[x_len..];
    for i in start + 1..=s + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::exact_from(id_to_n(i));
            if prod > max_prod {
                factors[j] = prod;
                j += 1;
                prod = 1;
            }
            let mut q = limb_n;
            while q >= prime {
                q /= prime;
                if q.odd() {
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
    assert!(max_prod <= LIMB_MAX_DIV_3);
    let l_max_prod = max_prod * 3;
    for i in s + 2..=n_to_bit(n / 3) + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::exact_from(id_to_n(i));
            if (limb_n / prime).odd() {
                if prod > l_max_prod {
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
    // Store primes from (n + 1) / 2 to n
    let start = n_to_bit(n >> 1) + 1;
    let mut index = bit_to_limb_count_floor(start);
    let mut mask = Limb::power_of_2(start & Limb::WIDTH_MASK);
    for i in start + 1..=n_to_bit(n) + 1 {
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
    if j != 0 {
        factors[j] = prod;
        j += 1;
        match limbs_product(&mut x_and_sieve[..x_len], &mut factors[..j]) {
            (size, None) => size,
            (size, Some(new_x_and_sieve)) => {
                x_and_sieve[..size].copy_from_slice(&new_x_and_sieve[..size]);
                size
            }
        }
    } else {
        // not triggered by the first billion inputs
        fail_on_untested_path("limbs_2_multiswing_odd, j == 0");
        x_and_sieve[0] = prod;
        1
    }
}

pub(crate) const FAC_DSC_THRESHOLD: usize = 236;

const fn clb2(x: usize) -> usize {
    let floor_log_base_2 = (usize::WIDTH as usize - x.leading_zeros() as usize) - 1;
    if x.is_power_of_two() {
        floor_log_base_2
    } else {
        floor_log_base_2 + 1
    }
}

const FACTORS_PER_LIMB: usize =
    (Limb::WIDTH << 1) as usize / (clb2(FAC_DSC_THRESHOLD * FAC_DSC_THRESHOLD - 1) + 1) - 1;

// n ^ log <= Limb::MAX: a limb can store log factors less than n.
//
// This is equivalent to log_n_max, `gmp-impl.h`, GMP 6.2.1.
pub(crate) fn log_n_max(n: Limb) -> u64 {
    // NTH_ROOT_NUMB_MASK_TABLE[0] is Limb::MAX, so a match will always be found
    u64::wrapping_from(
        NTH_ROOT_NUMB_MASK_TABLE
            .iter()
            .rposition(|&x| n <= x)
            .unwrap(),
    ) + 1
}

// `limbs_odd_factorial` computes the odd part of the factorial of the parameter n, i.e. n! = x * 2
// ^ a, where x is the returned value: an odd positive integer.
//
// If `double` is `true`, a square is skipped in the DSC part, e.g. if n is odd, n >
// FAC_DSC_THRESHOLD and `double` is true, x is set to n!!.
//
// If n is too small, `double` is ignored, and an assert can be triggered.
//
// TODO: FAC_DSC_THRESHOLD is used here with two different roles:
// - to decide when prime factorisation is needed,
// - to stop the recursion, once sieving is done.
// Maybe two thresholds can do a better job.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpz_oddfac_1` from `mpz/oddfac_1.c`, GMP 6.2.1.

crate_test_fn! {
#[allow(clippy::redundant_comparisons)]
limbs_odd_factorial(n: usize, double: bool) -> Vec<Limb> {
    assert!(Limb::convertible_from(n));
    if double {
        assert!(n > ODD_DOUBLEFACTORIAL_TABLE_LIMIT_PLUS_1 && n >= FAC_DSC_THRESHOLD);
    }
    if n <= ODD_FACTORIAL_TABLE_LIMIT {
        vec![ONE_LIMB_ODD_FACTORIAL_TABLE[n]]
    } else if n <= ODD_DOUBLEFACTORIAL_TABLE_LIMIT_PLUS_1 {
        let (hi, lo) = Limb::x_mul_y_to_zz(
            ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE[(n - 1) >> 1],
            ONE_LIMB_ODD_FACTORIAL_TABLE[n >> 1],
        );
        vec![lo, hi]
    } else {
        // Compute the number of recursive steps for the DSC algorithm
        let mut m = n;
        let mut s = 0;
        while m >= FAC_DSC_THRESHOLD {
            m >>= 1;
            s += 1;
        }
        let mut factors = vec![0; m / FACTORS_PER_LIMB + 1];
        assert!(m >= FACTORS_PER_LIMB);
        assert!(m > ODD_DOUBLEFACTORIAL_TABLE_LIMIT_PLUS_1);
        let mut j = 0;
        let mut prod = 1;
        let mut max_prod = const { Limb::MAX / (FAC_DSC_THRESHOLD * FAC_DSC_THRESHOLD) as Limb };
        assert!(m > ODD_DOUBLEFACTORIAL_TABLE_LIMIT_PLUS_1);
        loop {
            factors[j] = ODD_DOUBLEFACTORIAL_TABLE_MAX;
            j += 1;
            let mut diff = (m - ODD_DOUBLEFACTORIAL_TABLE_LIMIT) & const { 2usize.wrapping_neg() };
            if diff & 2 != 0 {
                let f = (ODD_DOUBLEFACTORIAL_TABLE_LIMIT + diff) as Limb;
                if prod > max_prod {
                    factors[j] = prod;
                    j += 1;
                    prod = f;
                } else {
                    prod *= f;
                }
                diff -= 2;
            }
            if diff != 0 {
                let mut fac = const { ODD_DOUBLEFACTORIAL_TABLE_LIMIT + 2 }
                    * (ODD_DOUBLEFACTORIAL_TABLE_LIMIT + diff);
                loop {
                    let f = fac as Limb;
                    if prod > max_prod {
                        factors[j] = prod;
                        j += 1;
                        prod = f;
                    } else {
                        prod *= f;
                    }
                    diff -= 4;
                    fac += diff << 1;
                    if diff == 0 {
                        break;
                    }
                }
            }
            max_prod <<= 2;
            m >>= 1;
            if m <= ODD_DOUBLEFACTORIAL_TABLE_LIMIT_PLUS_1 {
                break;
            }
        }
        factors[j] = prod;
        j += 1;
        factors[j] = ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE[(m - 1) >> 1];
        j += 1;
        factors[j] = ONE_LIMB_ODD_FACTORIAL_TABLE[m >> 1];
        j += 1;
        let mut out = Vec::new();
        let (out_size, new_out) = limbs_product(&mut out, &mut factors[..j]);
        out = new_out.unwrap();
        out.truncate(out_size);
        if s != 0 {
            // Use the algorithm described by Peter Luschny in "Divide, Swing and Conquer the
            // Factorial!".
            let mut size = (n >> Limb::LOG_WIDTH) + 4;
            let n_m_1 = u64::exact_from(n - 1);
            assert!(limbs_prime_sieve_size::<Limb>(n_m_1) < size - (size >> 1));
            // 2-multiswing(n) < 2^(n - 1) * sqrt(n / pi) < 2 ^ (n + Limb::WIDTH); One more can be
            // overwritten by mul, another for the sieve.
            let mut swing_and_sieve = vec![0; size];
            // Put the sieve on the second half; it will be overwritten by the last
            // `limbs_2_multiswing_odd`.
            let sieve_offset = (size >> 1) + 1;
            let ss_len = swing_and_sieve.len() - 1;
            #[cfg(feature = "32_bit_limbs")]
            let count = limbs_prime_sieve_u32(&mut swing_and_sieve[sieve_offset..ss_len], n_m_1);
            #[cfg(not(feature = "32_bit_limbs"))]
            let count = limbs_prime_sieve_u64(&mut swing_and_sieve[sieve_offset..ss_len], n_m_1);
            size = usize::exact_from((count + 1) / log_n_max(Limb::exact_from(n)) + 1);
            let mut factors = vec![0; size];
            let mut out_len = out.len();
            // The squaring scratch is reused across iterations, growing when needed. Its length is
            // not monotonic in the operand size - it is 0 in both the basecase and FFT regimes - so
            // it is sliced to the exact length each time.
            let mut square_scratch: Vec<Limb> = Vec::new();
            // The square buffer is likewise reused across iterations, growing when needed. Only
            // square[..size] is written and read, so stale limbs beyond that are harmless.
            let mut square: Vec<Limb> = Vec::new();
            for i in (0..s).rev() {
                let ns = limbs_2_multiswing_odd(
                    &mut swing_and_sieve,
                    sieve_offset,
                    Limb::exact_from(n >> i),
                    &mut factors,
                );
                if double && i == 0 {
                    size = out_len;
                    if square.len() < size {
                        square.resize(size, 0);
                    }
                    square[..out_len].copy_from_slice(&out[..out_len]);
                } else {
                    size = out_len << 1;
                    if square.len() < size {
                        square.resize(size, 0);
                    }
                    let scratch_len = limbs_square_to_out_scratch_len(out_len);
                    if square_scratch.len() < scratch_len {
                        square_scratch.resize(scratch_len, 0);
                    }
                    limbs_square_to_out(
                        &mut square,
                        &out[..out_len],
                        &mut square_scratch[..scratch_len],
                    );
                    if square[size - 1] == 0 {
                        size -= 1;
                    }
                }
                out_len = size + ns;
                out.resize(out_len, 0);
                assert!(ns <= size);
                // n != n$ * floor(n / 2)! ^ 2
                let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(size, ns)];
                if limbs_mul_greater_to_out(
                    &mut out,
                    &square[..size],
                    &swing_and_sieve[..ns],
                    &mut mul_scratch,
                ) == 0
                {
                    out_len -= 1;
                }
            }
        }
        if *out.last().unwrap() == 0 {
            out.pop();
        }
        out
    }
}}

const FAC_ODD_THRESHOLD: Limb = 24;

#[cfg(feature = "32_bit_limbs")]
const SMALL_FACTORIAL_LIMIT: u64 = 13;
#[cfg(not(feature = "32_bit_limbs"))]
const SMALL_FACTORIAL_LIMIT: u64 = 21;

impl Factorial for Natural {
    /// Computes the factorial of a number.
    ///
    /// $$
    /// f(n) = n! = 1 \times 2 \times 3 \times \cdots \times n.
    /// $$
    ///
    /// $n! = O(\sqrt{n}(n/e)^n)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Factorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::factorial(0), 1);
    /// assert_eq!(Natural::factorial(1), 1);
    /// assert_eq!(Natural::factorial(2), 2);
    /// assert_eq!(Natural::factorial(3), 6);
    /// assert_eq!(Natural::factorial(4), 24);
    /// assert_eq!(Natural::factorial(5), 120);
    /// assert_eq!(
    ///     Natural::factorial(100).to_string(),
    ///     "9332621544394415268169923885626670049071596826438162146859296389521759999322991560894\
    ///     1463976156518286253697920827223758251185210916864000000000000000000000000"
    /// );
    /// ```
    ///
    /// This is equivalent to `mpz_fac_ui` from `mpz/fac_ui.c`, GMP 6.2.1.
    // `FAC_ODD_THRESHOLD` is a `Limb`, so the cast below widens on 32-bit and is a no-op on 64-bit;
    // dropping it would break the 32-bit build.
    #[allow(clippy::useless_conversion, clippy::unnecessary_cast)]
    fn factorial(n: u64) -> Self {
        assert!(Limb::convertible_from(n));
        if n < SMALL_FACTORIAL_LIMIT {
            Self::from(Limb::factorial(n))
        } else if n < const { FAC_ODD_THRESHOLD as u64 } {
            let mut factors =
                vec![0; usize::wrapping_from(n - SMALL_FACTORIAL_LIMIT) / FACTORS_PER_LIMB + 2];
            factors[0] = Limb::factorial(const { SMALL_FACTORIAL_LIMIT - 1 });
            let mut j = 1;
            let n = Limb::wrapping_from(n);
            let mut prod = n;
            const MAX_PROD: Limb = Limb::MAX / (FAC_ODD_THRESHOLD | 1);
            const LIMB_SMALL_FACTORIAL_LIMIT: Limb = SMALL_FACTORIAL_LIMIT as Limb;
            for i in (LIMB_SMALL_FACTORIAL_LIMIT..n).rev() {
                if prod > MAX_PROD {
                    factors[j] = prod;
                    j += 1;
                    prod = i;
                } else {
                    prod *= i;
                }
            }
            factors[j] = prod;
            j += 1;
            let mut xs = Vec::new();
            let new_xs = limbs_product(&mut xs, &mut factors[..j]).1;
            xs = new_xs.unwrap();
            Self::from_owned_limbs_asc(xs)
        } else {
            let count = if n <= TABLE_LIMIT_2N_MINUS_POPC_2N {
                u64::from(TABLE_2N_MINUS_POPC_2N[usize::exact_from((n >> 1) - 1)])
            } else {
                n - CountOnes::count_ones(n)
            };
            Self::from_owned_limbs_asc(limbs_odd_factorial(usize::exact_from(n), false)) << count
        }
    }
}

const FAC_2DSC_THRESHOLD: Limb = ((FAC_DSC_THRESHOLD << 1) | (FAC_DSC_THRESHOLD & 1)) as Limb;

impl DoubleFactorial for Natural {
    /// Computes the double factorial of a number.
    ///
    /// $$
    /// f(n) = n!! = n \times (n - 2) \times (n - 4) \times \cdots \times i,
    /// $$
    /// where $i$ is 1 if $n$ is odd and $2$ if $n$ is even.
    ///
    /// $n!! = O(\sqrt{n}(n/e)^{n/2})$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DoubleFactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::double_factorial(0), 1);
    /// assert_eq!(Natural::double_factorial(1), 1);
    /// assert_eq!(Natural::double_factorial(2), 2);
    /// assert_eq!(Natural::double_factorial(3), 3);
    /// assert_eq!(Natural::double_factorial(4), 8);
    /// assert_eq!(Natural::double_factorial(5), 15);
    /// assert_eq!(Natural::double_factorial(6), 48);
    /// assert_eq!(Natural::double_factorial(7), 105);
    /// assert_eq!(
    ///     Natural::double_factorial(99).to_string(),
    ///     "2725392139750729502980713245400918633290796330545803413734328823443106201171875"
    /// );
    /// assert_eq!(
    ///     Natural::double_factorial(100).to_string(),
    ///     "34243224702511976248246432895208185975118675053719198827915654463488000000000000"
    /// );
    /// ```
    ///
    /// This is equivalent to `mpz_2fac_ui` from `mpz/2fac_ui.c`, GMP 6.2.1.
    // `FAC_2DSC_THRESHOLD` is a `Limb`, so the cast below widens on 32-bit and is a no-op on
    // 64-bit; dropping it would break the 32-bit build.
    #[allow(clippy::unnecessary_cast)]
    fn double_factorial(n: u64) -> Self {
        assert!(Limb::convertible_from(n));
        if n.even() {
            // n is even, n = 2k, (2k)!! = k! 2^k
            let half_n = usize::wrapping_from(n >> 1);
            let count = if n <= TABLE_LIMIT_2N_MINUS_POPC_2N && n != 0 {
                u64::from(TABLE_2N_MINUS_POPC_2N[half_n - 1])
            } else {
                n - CountOnes::count_ones(n)
            };
            Self::from_owned_limbs_asc(limbs_odd_factorial(half_n, false)) << count
        } else if n <= const { ODD_DOUBLEFACTORIAL_TABLE_LIMIT as u64 } {
            Self::from(ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE[usize::wrapping_from(n >> 1)])
        } else if n < const { FAC_2DSC_THRESHOLD as u64 } {
            let mut factors = vec![0; usize::exact_from(n) / const { FACTORS_PER_LIMB << 1 } + 1];
            factors[0] = ODD_DOUBLEFACTORIAL_TABLE_MAX;
            let mut j = 1;
            let mut n = Limb::wrapping_from(n);
            let mut prod = n;
            const MAX_PROD: Limb = Limb::MAX / FAC_2DSC_THRESHOLD;
            const LIMIT: Limb = ODD_DOUBLEFACTORIAL_TABLE_LIMIT as Limb + 2;
            while n > LIMIT {
                n -= 2;
                if prod > MAX_PROD {
                    factors[j] = prod;
                    j += 1;
                    prod = n;
                } else {
                    prod *= n;
                }
            }
            factors[j] = prod;
            j += 1;
            let mut xs = Vec::new();
            let new_xs = limbs_product(&mut xs, &mut factors[..j]).1;
            xs = new_xs.unwrap();
            Self::from_owned_limbs_asc(xs)
        } else {
            Self::from_owned_limbs_asc(limbs_odd_factorial(usize::exact_from(n), true))
        }
    }
}

impl Multifactorial for Natural {
    /// Computes a multifactorial of a number.
    ///
    /// $$
    /// f(n, m) = n!^{(m)} = n \times (n - m) \times (n - 2m) \times \cdots \times i.
    /// $$
    /// If $n$ is divisible by $m$, then $i$ is $m$; otherwise, $i$ is the remainder when $n$ is
    /// divided by $m$.
    ///
    /// $n!^{(m)} = O(\sqrt{n}(n/e)^{n/m})$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(n \log n)$
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Multifactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::multifactorial(0, 1), 1);
    /// assert_eq!(Natural::multifactorial(1, 1), 1);
    /// assert_eq!(Natural::multifactorial(2, 1), 2);
    /// assert_eq!(Natural::multifactorial(3, 1), 6);
    /// assert_eq!(Natural::multifactorial(4, 1), 24);
    /// assert_eq!(Natural::multifactorial(5, 1), 120);
    ///
    /// assert_eq!(Natural::multifactorial(0, 2), 1);
    /// assert_eq!(Natural::multifactorial(1, 2), 1);
    /// assert_eq!(Natural::multifactorial(2, 2), 2);
    /// assert_eq!(Natural::multifactorial(3, 2), 3);
    /// assert_eq!(Natural::multifactorial(4, 2), 8);
    /// assert_eq!(Natural::multifactorial(5, 2), 15);
    /// assert_eq!(Natural::multifactorial(6, 2), 48);
    /// assert_eq!(Natural::multifactorial(7, 2), 105);
    ///
    /// assert_eq!(Natural::multifactorial(0, 3), 1);
    /// assert_eq!(Natural::multifactorial(1, 3), 1);
    /// assert_eq!(Natural::multifactorial(2, 3), 2);
    /// assert_eq!(Natural::multifactorial(3, 3), 3);
    /// assert_eq!(Natural::multifactorial(4, 3), 4);
    /// assert_eq!(Natural::multifactorial(5, 3), 10);
    /// assert_eq!(Natural::multifactorial(6, 3), 18);
    /// assert_eq!(Natural::multifactorial(7, 3), 28);
    /// assert_eq!(Natural::multifactorial(8, 3), 80);
    /// assert_eq!(Natural::multifactorial(9, 3), 162);
    ///
    /// assert_eq!(
    ///     Natural::multifactorial(100, 3).to_string(),
    ///     "174548867015437739741494347897360069928419328000000000"
    /// );
    /// ```
    fn multifactorial(mut n: u64, mut m: u64) -> Self {
        assert_ne!(m, 0);
        assert!(Limb::convertible_from(n));
        assert!(Limb::convertible_from(m));
        if n < 3 || n - 3 < m - 1 {
            // n < 3 || n - 1 <= m
            if n == 0 { Self::ONE } else { Self::from(n) }
        } else {
            // 0 < m < n - 1 < Limb::MAX
            let gcd = n.gcd(m);
            if gcd > 1 {
                n /= gcd;
                m /= gcd;
            }
            if m <= 2 {
                // fac or 2fac
                if m == 1 {
                    match gcd {
                        gcd if gcd > 2 => Self::from(gcd).pow(n) * Self::factorial(n),
                        2 => Self::double_factorial(n << 1),
                        _ => Self::factorial(n),
                    }
                } else if gcd > 1 {
                    // m == 2
                    Self::from(gcd).pow((n >> 1) + 1) * Self::double_factorial(n)
                } else {
                    Self::double_factorial(n)
                }
            } else {
                // m >= 3, gcd(n,m) = 1
                let reduced_n = n / m + 1;
                let mut n = Limb::exact_from(n);
                let m = Limb::exact_from(m);
                let mut j = 0;
                let mut prod = n;
                n -= m;
                let max_prod = Limb::MAX / n;
                let mut factors = vec![0; usize::exact_from(reduced_n / log_n_max(n) + 2)];
                while n > m {
                    if prod > max_prod {
                        factors[j] = prod;
                        j += 1;
                        prod = n;
                    } else {
                        prod *= n;
                    }
                    n -= m;
                }
                factors[j] = n;
                j += 1;
                factors[j] = prod;
                j += 1;
                let mut xs = Vec::new();
                let new_xs = limbs_product(&mut xs, &mut factors[..j]).1;
                xs = new_xs.unwrap();
                let x = Self::from_owned_limbs_asc(xs);
                if gcd == 1 {
                    x
                } else {
                    Self::from(gcd).pow(reduced_n) * x
                }
            }
        }
    }
}

impl Subfactorial for Natural {
    /// Computes the subfactorial of a number.
    ///
    /// The subfactorial of $n$ counts the number of derangements of a set of size $n$; a
    /// derangement is a permutation with no fixed points.
    ///
    /// $$
    /// f(n) = \\ !n = \lfloor n!/e \rfloor.
    /// $$
    ///
    /// $!n = O(n!) = O(\sqrt{n}(n/e)^n)$.
    ///
    /// The subfactorial is computed by binary splitting on the recurrence $!n = n \cdot !(n - 1) +
    /// (-1)^n$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Subfactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::subfactorial(0), 1);
    /// assert_eq!(Natural::subfactorial(1), 0);
    /// assert_eq!(Natural::subfactorial(2), 1);
    /// assert_eq!(Natural::subfactorial(3), 2);
    /// assert_eq!(Natural::subfactorial(4), 9);
    /// assert_eq!(Natural::subfactorial(5), 44);
    /// assert_eq!(
    ///     Natural::subfactorial(100).to_string(),
    ///     "3433279598416380476519597752677614203236578380537578498354340028268518079332763243279\
    ///     1396429850988990237345920155783984828001486412574060553756854137069878601"
    /// );
    /// ```
    fn subfactorial(n: u64) -> Self {
        if n < SUBFACTORIAL_SPLIT_THRESHOLD {
            subfactorial_naive(n)
        } else if n.odd() {
            // The pairs (2, 3), (4, 5), ..., (n - 1, n) send !1 = 0 to !n.
            subfactorial_split_only_b(2, n - 1)
        } else {
            // !n = n * !(n - 1) + 1, since n is even
            let mut f = subfactorial_split_only_b(2, n - 2);
            f *= Self::from(n);
            f += Self::ONE;
            f
        }
    }
}
