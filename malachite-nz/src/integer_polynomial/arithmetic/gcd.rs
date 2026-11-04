// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009, 2010 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::arithmetic::mul::multi_mod::next_prime;
use crate::integer_matrix::linear_algebra::determinant::BalancedCrtWord;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::bit_pack::limbs_pack_coefficients;
use crate::integer_polynomial::arithmetic::bit_unpack::limbs_unpack_coefficients;
use crate::integer_polynomial::arithmetic::div_exact::checked_div_exact_slices;
use crate::integer_vector::arithmetic::content::content;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::{LIMB_HIGH_BIT, Natural, WIDTH_MINUS_1};
use crate::platform::{DoubleLimb, Limb, PRIMES_ABOVE_LIMB_HIGH_BIT};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    DivExact, DivRem, Gcd, GcdAssign, NegAssign, Parity, Pow, SubMulAssign, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::ModMul;
use malachite_base::polynomial::arithmetic::traits::L2NormSquared;
use malachite_base::unsigned_polynomial::modular::mod_gcd::checked_monic_gcd;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::Modulus;

// The primes greater than $2^{W-1}$, where $W$ is the limb width, in increasing order: those of
// `PRIMES_ABOVE_LIMB_HIGH_BIT`, and then each found by searching from the last.
pub(crate) struct LargePrimes {
    i: usize,
    p: Limb,
}

impl LargePrimes {
    pub(crate) const fn new() -> Self {
        Self {
            i: 0,
            p: LIMB_HIGH_BIT,
        }
    }
}

impl Iterator for LargePrimes {
    type Item = Limb;

    fn next(&mut self) -> Option<Limb> {
        self.p = PRIMES_ABOVE_LIMB_HIGH_BIT
            .get(self.i)
            .copied()
            .unwrap_or_else(|| next_prime(self.p));
        self.i += 1;
        Some(self.p)
    }
}

// The first `n` primes greater than $2^{W-1}$, where $W$ is the limb width.
crate_test_fn! {large_primes(n: usize) -> Vec<Limb> {
    LargePrimes::new().take(n).collect()
}}

fn polynomial(xs: Vec<Integer>) -> IntegerPolynomial {
    let mut p = IntegerPolynomial { coefficients: xs };
    p.trim();
    p
}

pub(crate) fn trim(xs: &mut Vec<Integer>) {
    while xs.last().is_some_and(|x| *x == 0u32) {
        xs.pop();
    }
}

// The largest number of significant bits of the absolute values of `xs`.
pub(crate) fn max_bits(xs: &[Integer]) -> u64 {
    vec_max_bits(xs).0
}

pub(crate) fn div_exact_by(xs: &[Integer], c: &Integer) -> Vec<Integer> {
    xs.iter().map(|x| x.div_exact(c)).collect()
}

fn mul_by(xs: &[Integer], c: &Integer) -> Vec<Integer> {
    xs.iter().map(|x| x * c).collect()
}

// Whether `ys`, nonempty, divides `xs`; neither ends with a zero.
fn divides(xs: &[Integer], ys: &[Integer]) -> bool {
    checked_div_exact_slices(xs, ys).is_some()
}

// The pseudo-remainder of `xs` by `ys`, where `xs` is at least as long as `ys`, multiplied by the
// full power $c^{m - n + 1}$ of the leading coefficient $c$ of `ys`, where $m$ and $n$ are the
// lengths.
//
// This is equivalent to `_fmpz_poly_pseudo_rem_cohen` from `fmpz_poly/pseudo_rem_cohen.c`, FLINT
// 3.6.0.
pub(crate) fn pseudo_rem_cohen(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    let y_len = ys.len();
    if y_len == 1 {
        // Both callers stop before the divisor becomes a constant.
        fail_on_untested_path("pseudo_rem_cohen, ys.len() == 1");
        return Vec::new();
    }
    let lead = ys.last().unwrap();
    let mut rs = xs.to_vec();
    let mut len = rs.len();
    let mut e = u64::exact_from(len - y_len + 1);
    while len >= y_len {
        let top = core::mem::take(&mut rs[len - 1]);
        for r in &mut rs[..len - 1] {
            *r *= lead;
        }
        for (r, y) in rs[len - y_len..len - 1].iter_mut().zip(&ys[..y_len - 1]) {
            r.sub_mul_assign(&top, y);
        }
        len -= 1;
        while len != 0 && rs[len - 1] == 0u32 {
            len -= 1;
        }
        e -= 1;
    }
    rs.truncate(len);
    if e != 0 {
        let scale = lead.pow(e);
        for r in &mut rs {
            *r *= &scale;
        }
    }
    rs
}

// The GCD of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long as
// `ys` and neither is empty or ends with a zero, by the subresultant algorithm.
//
// This is equivalent to `_fmpz_poly_gcd_subresultant` from `fmpz_poly/gcd_subresultant.c`, FLINT
// 3.6.0.
crate_test_fn! {gcd_subresultant(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    if ys.len() == 1 {
        return vec![Integer::from(content(xs).gcd(ys[0].unsigned_abs_ref()))];
    }
    let a_content = Integer::from(content(xs));
    let b_content = Integer::from(content(ys));
    let mut a = div_exact_by(xs, &a_content);
    let mut b = div_exact_by(ys, &b_content);
    let mut d = Integer::from(a_content.unsigned_abs().gcd(b_content.unsigned_abs()));
    let mut g = Integer::ONE;
    let mut h = Integer::ONE;
    loop {
        let delta = u64::exact_from(a.len() - b.len());
        a = pseudo_rem_cohen(&a, &b);
        trim(&mut a);
        if a.len() <= 1 {
            break;
        }
        core::mem::swap(&mut a, &mut b);
        if delta == 1 {
            b = div_exact_by(&b, &(&g * &h));
            g = a.last().unwrap().clone();
            h.clone_from(&g);
        } else {
            let h_pow = (&h).pow(delta);
            b = div_exact_by(&b, &(&g * &h_pow));
            g = h * a.last().unwrap().pow(delta);
            h = (&g).div_exact(h_pow);
            g = a.last().unwrap().clone();
        }
    }
    if a.len() == 1 {
        vec![d]
    } else {
        let b_content = Integer::from(content(&b));
        b = div_exact_by(&b, &b_content);
        if *b.last().unwrap() < 0u32 {
            d.neg_assign();
        }
        mul_by(&b, &d)
    }
}}

// Packs `xs` into `bits`-bit fields, negated if `negate`, as a `Natural`.
fn pack(xs: &[Integer], bits: u64, negate: bool) -> Natural {
    let limbs = usize::exact_from(((bits * u64::exact_from(xs.len()) - 1) >> Limb::LOG_WIDTH) + 1);
    let mut out = vec![0; limbs];
    limbs_pack_coefficients(&mut out, xs, bits, negate);
    Natural::from_owned_limbs_asc(out)
}

// Unpacks the first `len` signed `bits`-bit fields of `x`, without zeros at the end.
fn unpack(x: &Natural, len: usize, bits: u64) -> Vec<Integer> {
    let mut limbs = x.to_limbs_asc();
    limbs.resize(
        usize::exact_from(((bits * u64::exact_from(len)) >> Limb::LOG_WIDTH) + 2),
        0,
    );
    let mut out = vec![Integer::ZERO; len];
    limbs_unpack_coefficients(&mut out, 0, len, &limbs, bits, false);
    trim(&mut out);
    out
}

// Whether `xs`, negated if `negate`, is the product of `qs` and `gs`.
fn multiplies_out(xs: &[Integer], qs: &[Integer], gs: &[Integer], negate: bool) -> bool {
    let mut product = polynomial(qs.to_vec()) * polynomial(gs.to_vec());
    if negate {
        product.neg_assign();
    }
    product.coefficients == xs
}

// The GCD of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long as
// `ys` and neither is empty or ends with a zero, by evaluating at a power of 2 and taking the GCD
// of the values, or `None` if that does not reveal the GCD.
//
// This is equivalent to `_fmpz_poly_gcd_heuristic` from `fmpz_poly/gcd_heuristic.c`, FLINT 3.6.0,
// except that the second quotient is checked with the sign of the second polynomial, where FLINT
// uses the sign of the first and so gives up whenever the leading coefficients differ in sign.
crate_test_fn! {gcd_heuristic(xs: &[Integer], ys: &[Integer]) -> Option<Vec<Integer>> {
    let a_content = content(xs);
    let b_content = content(ys);
    let d = Integer::from((&a_content).gcd(&b_content));
    if ys.len() == 1 {
        return Some(vec![d]);
    }
    let a = div_exact_by(xs, &Integer::from(a_content));
    let b = div_exact_by(ys, &Integer::from(b_content));
    let (len1, len2) = (a.len(), b.len());
    if len2 == 2 {
        return Some(if divides(&a, &b) {
            let mut r = mul_by(&b, &d);
            if r[1] < 0u32 {
                for c in &mut r {
                    c.neg_assign();
                }
            }
            r
        } else {
            vec![d]
        });
    }
    let bits1 = max_bits(&a);
    let bits2 = max_bits(&b);
    let max_bits_ab = max(bits1, bits2) + 1;
    let bound_bits = min(bits1, bits2) + 6;
    let mut pack_bits = max(bound_bits, max_bits_ab);
    let pack_limbs = ((pack_bits - 1) >> Limb::LOG_WIDTH) + 1;
    if pack_bits >= 32 {
        pack_bits = pack_limbs << Limb::LOG_WIDTH;
    }
    let negate1 = *a.last().unwrap() < 0u32;
    let negate2 = *b.last().unwrap() < 0u32;
    let array1 = pack(&a, pack_bits, negate1);
    let array2 = pack(&b, pack_bits, negate2);
    let limbs1 = array1.limb_count();
    let limbs2 = array2.limb_count();
    let mut array_g = (&array1).gcd(&array2);
    let limbs_g = array_g.limb_count();
    let g_len = min(
        usize::exact_from((limbs_g << Limb::LOG_WIDTH) / pack_bits + 1),
        len2,
    );
    let mut g = unpack(&array_g, g_len, pack_bits);
    let g_len = g.len();
    let g_content = content(&g);
    if g_content != 1u32 {
        array_g /= &g_content;
    }
    let limbs_g = array_g.limb_count();
    let q_len = min(
        len1,
        usize::exact_from(((limbs1 - limbs_g + 1) << Limb::LOG_WIDTH) / pack_bits + 1),
    );
    let q_len_2 = min(
        len2,
        usize::exact_from(((limbs2 - limbs_g + 1) << Limb::LOG_WIDTH) / pack_bits + 1),
    );
    let (q, r) = (&array1).div_rem(&array_g);
    if r != 0u32 {
        // The packed GCD divides the packed polynomial unless dividing it by the content of its
        // unpacked form was inexact.
        fail_on_untested_path("gcd_heuristic, first r != 0u32");
        return None;
    }
    let qs = unpack(&q, q_len, pack_bits);
    g = div_exact_by(&g, &Integer::from(g_content));
    let bits_g = max_bits(&g);
    let log_g_len = g_len.significant_bits();
    let log_length = min(log_g_len, qs.len().significant_bits());
    if bits_g + max_bits(&qs) + log_length >= pack_bits && !multiplies_out(&a, &qs, &g, negate1) {
        return None;
    }
    let (q, r) = (&array2).div_rem(&array_g);
    if r != 0u32 {
        // As for the first polynomial.
        fail_on_untested_path("gcd_heuristic, second r != 0u32");
        return None;
    }
    let qs = unpack(&q, q_len_2, pack_bits);
    let log_length = min(log_g_len, qs.len().significant_bits());
    if bits_g + max_bits(&qs) + log_length >= pack_bits && !multiplies_out(&b, &qs, &g, negate2) {
        return None;
    }
    Some(mul_by(&g, &d))
}}

// `x` modulo the word `p`, in $[0, p)$.
pub(crate) fn reduce(x: &Integer, p: Limb) -> Limb {
    let r = x.unsigned_abs_ref().rem_limb_ref(p);
    if *x >= 0u32 || r == 0 { r } else { p - r }
}

// The value in $(-p/2, p/2]$ congruent to `x` modulo the odd word `p`.
pub(crate) fn lift_balanced(x: Limb, p: Limb) -> Integer {
    if x > p >> 1 {
        Integer::from(x) - Integer::from(p)
    } else {
        Integer::from(x)
    }
}

// The content of `xs`, negated if the last element of `xs` is negative.
fn signed_content(xs: &[Integer]) -> Integer {
    let c = Integer::from(content(xs));
    if *xs.last().unwrap() < 0u32 { -c } else { c }
}

// The GCD of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long as
// `ys` and neither is empty or ends with a zero, from their GCDs modulo primes.
//
// This is equivalent to `_fmpz_poly_gcd_modular` from `fmpz_poly/gcd_modular.c`, FLINT 3.6.0.
crate_test_fn! {gcd_modular(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    let a_content = content(xs);
    let b_content = content(ys);
    let d = Integer::from((&a_content).gcd(&b_content));
    if ys.len() == 1 {
        return vec![d];
    }
    let a = div_exact_by(xs, &Integer::from(a_content));
    let b = div_exact_by(ys, &Integer::from(b_content));
    let (len1, len2) = (a.len(), b.len());
    let lead_a = a.last().unwrap();
    let lead_b = b.last().unwrap();
    let bits1 = max_bits(&a);
    let bits2 = max_bits(&b);
    let (nb1, nb2) = if len1 < 64 && len2 < 64 {
        (
            polynomial(a.clone()).l2_norm_squared().significant_bits(),
            polynomial(b.clone()).l2_norm_squared().significant_bits(),
        )
    } else {
        (
            (bits1 << 1) + len1.significant_bits(),
            (bits2 << 1) + len2.significant_bits(),
        )
    };
    let g = lead_a.unsigned_abs_ref().gcd(lead_b.unsigned_abs_ref());
    let l = (lead_a * lead_b).unsigned_abs();
    let g_is_one = g == 1u32;
    // -A(-1) and -B(-1)
    let mut eval_a = Integer::ZERO;
    for (i, c) in a.iter().enumerate() {
        if i.odd() {
            eval_a += c;
        } else {
            eval_a -= c;
        }
    }
    let mut eval_b = Integer::ZERO;
    for (i, c) in b.iter().enumerate() {
        if i.odd() {
            eval_b += c;
        } else {
            eval_b -= c;
        }
    }
    let eval_gcd = eval_a.unsigned_abs().gcd(eval_b.unsigned_abs());
    let bits_small = max(
        max(eval_gcd.significant_bits(), g.significant_bits()),
        2,
    );
    let mut primes = LargePrimes::new();
    let mut modulus = Natural::ZERO;
    let mut res: Vec<Integer> = Vec::new();
    let mut n = len2;
    let n0 = u64::exact_from(len1 - 1);
    let bound = (n0 + 3) * max(nb1, nb2) + n0 + 1;
    let mut unlucky = 0;
    let mut curr_bits = 0;
    loop {
        let p = primes.next().unwrap();
        if l.rem_limb_ref(p) == 0 {
            unlucky += WIDTH_MINUS_1;
            continue;
        }
        let a_p: Vec<Limb> = a.iter().map(|c| reduce(c, p)).collect();
        let b_p: Vec<Limb> = b.iter().map(|c| reduce(c, p)).collect();
        let mut h = checked_monic_gcd(&a_p, &b_p, &Modulus(p)).unwrap();
        let h_len = h.len();
        if h_len == 1 {
            res = vec![Integer::ONE];
            break;
        }
        if h_len > n + 1 {
            unlucky += WIDTH_MINUS_1;
            continue;
        }
        if !g_is_one {
            let g_p = g.rem_limb_ref(p);
            for c in &mut h {
                *c = c.mod_mul(g_p, p);
            }
        }
        if h_len <= n {
            unlucky += modulus.significant_bits();
            res = h.iter().map(|&c| lift_balanced(c, p)).collect();
            if g_is_one {
                if divides(&b, &res) && divides(&a, &res) {
                    break;
                }
            } else {
                if WIDTH_MINUS_1 + unlucky >= bound {
                    let h_content = Integer::from(content(&res));
                    res = div_exact_by(&res, &h_content);
                    break;
                }
                if WIDTH_MINUS_1 >= bits_small {
                    let h_content = signed_content(&res);
                    res = div_exact_by(&res, &h_content);
                    if divides(&b, &res) && divides(&a, &res) {
                        break;
                    }
                    res = mul_by(&res, &h_content);
                }
            }
            curr_bits = max_bits(&res);
            modulus = Natural::from(p);
            n = h_len - 1;
            continue;
        }
        let crt = BalancedCrtWord::new(&modulus, p);
        for (r, &c) in res.iter_mut().zip(h.iter()) {
            *r = crt.combine(r, c);
        }
        modulus = crt.into_new_product();
        let new_bits = max_bits(&res);
        if new_bits == curr_bits || modulus.significant_bits() >= bits_small {
            let h_content = if g_is_one {
                None
            } else {
                let h_content = signed_content(&res);
                res = div_exact_by(&res, &h_content);
                Some(h_content)
            };
            if modulus.significant_bits() + unlucky >= bound
                || divides(&b, &res) && divides(&a, &res)
            {
                break;
            }
            if let Some(h_content) = h_content {
                res = mul_by(&res, &h_content);
            }
        }
        curr_bits = new_bits;
    }
    mul_by(&res, &d)
}}

// The GCD of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long as
// `ys` and neither is empty or ends with a zero.
//
// This is equivalent to `_fmpz_poly_gcd` from `fmpz_poly/gcd.c`, FLINT 3.6.0.
fn gcd_nonzero(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    let (len1, len2) = (xs.len(), ys.len());
    // Remove powers of x.
    let val1 = xs[..len1 - 1].iter().take_while(|&x| *x == 0u32).count();
    let val2 = ys[..len2 - 1].iter().take_while(|&y| *y == 0u32).count();
    if val1 != 0 || val2 != 0 {
        let d = min(val1, val2);
        let inner = if len1 - val1 >= len2 - val2 {
            gcd_nonzero(&xs[val1..], &ys[val2..])
        } else {
            gcd_nonzero(&ys[val2..], &xs[val1..])
        };
        let mut out = vec![Integer::ZERO; d];
        out.extend(inner);
        return out;
    }
    if len1 < 6 {
        gcd_subresultant(xs, ys)
    } else {
        if max_bits(xs) + max_bits(ys) < DoubleLimb::WIDTH
            && let Some(g) = gcd_heuristic(xs, ys)
        {
            return g;
        }
        gcd_modular(xs, ys)
    }
}

// The GCD of `p` and `q`: the product of the GCD of their contents and the GCD of their primitive
// parts, with a positive leading coefficient. The GCD of 0 and 0 is 0.
//
// This is equivalent to `fmpz_poly_gcd` from `fmpz_poly/gcd.c`, FLINT 3.6.0.
pub(crate) fn gcd_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    let (p, q) = if p.coefficients.len() < q.coefficients.len() {
        (q, p)
    } else {
        (p, q)
    };
    if p.coefficients.is_empty() {
        IntegerPolynomial::ZERO
    } else if q.coefficients.is_empty() {
        if *p.coefficients.last().unwrap() > 0u32 {
            p.clone()
        } else {
            -p
        }
    } else {
        polynomial(gcd_nonzero(&p.coefficients, &q.coefficients))
    }
}

impl Gcd<Self> for IntegerPolynomial {
    type Output = Self;

    /// Computes the GCD of two [`IntegerPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(a, b) = \gcd(a, b).
    /// $$
    ///
    /// The GCD has a positive leading coefficient, and the GCD of 0 and 0 is 0. It is the product
    /// of the GCD of the contents and the GCD of the primitive parts.
    ///
    /// Short polynomials use the subresultant algorithm. For longer ones with small coefficients, a
    /// heuristic first evaluates both at a large integer and recovers the GCD from the GCD of the
    /// values; otherwise, the GCD is found modulo many primes and reconstructed with the Chinese
    /// remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert_eq!(p.gcd(q).to_string(), "2*x+2");
    ///
    /// let p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.gcd(q).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_gcd` from `fmpz_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: Self) -> Self {
        gcd_helper(&self, &other)
    }
}

impl Gcd<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Computes the GCD of two [`IntegerPolynomial`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(a, b) = \gcd(a, b).
    /// $$
    ///
    /// The GCD has a positive leading coefficient, and the GCD of 0 and 0 is 0. It is the product
    /// of the GCD of the contents and the GCD of the primitive parts.
    ///
    /// Short polynomials use the subresultant algorithm. For longer ones with small coefficients, a
    /// heuristic first evaluates both at a large integer and recovers the GCD from the GCD of the
    /// values; otherwise, the GCD is found modulo many primes and reconstructed with the Chinese
    /// remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert_eq!(p.gcd(&q).to_string(), "2*x+2");
    ///
    /// let p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.gcd(&q).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_gcd` from `fmpz_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: &Self) -> Self {
        gcd_helper(&self, other)
    }
}

impl Gcd<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the GCD of two [`IntegerPolynomial`]s, taking the first by reference and the second
    /// by value.
    ///
    /// $$
    /// f(a, b) = \gcd(a, b).
    /// $$
    ///
    /// The GCD has a positive leading coefficient, and the GCD of 0 and 0 is 0. It is the product
    /// of the GCD of the contents and the GCD of the primitive parts.
    ///
    /// Short polynomials use the subresultant algorithm. For longer ones with small coefficients, a
    /// heuristic first evaluates both at a large integer and recovers the GCD from the GCD of the
    /// values; otherwise, the GCD is found modulo many primes and reconstructed with the Chinese
    /// remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert_eq!((&p).gcd(q).to_string(), "2*x+2");
    ///
    /// let p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!((&p).gcd(q).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_gcd` from `fmpz_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: IntegerPolynomial) -> IntegerPolynomial {
        gcd_helper(self, &other)
    }
}

impl Gcd<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the GCD of two [`IntegerPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(a, b) = \gcd(a, b).
    /// $$
    ///
    /// The GCD has a positive leading coefficient, and the GCD of 0 and 0 is 0. It is the product
    /// of the GCD of the contents and the GCD of the primitive parts.
    ///
    /// Short polynomials use the subresultant algorithm. For longer ones with small coefficients, a
    /// heuristic first evaluates both at a large integer and recovers the GCD from the GCD of the
    /// values; otherwise, the GCD is found modulo many primes and reconstructed with the Chinese
    /// remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// assert_eq!((&p).gcd(&q).to_string(), "2*x+2");
    ///
    /// let p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!((&p).gcd(&q).to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_gcd` from `fmpz_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        gcd_helper(self, other)
    }
}

impl GcdAssign<Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its GCD with another, taking the other polynomial by
    /// value.
    ///
    /// $$
    /// a \gets \gcd(a, b).
    /// $$
    ///
    /// The GCD has a positive leading coefficient, and the GCD of 0 and 0 is 0. It is the product
    /// of the GCD of the contents and the GCD of the primitive parts.
    ///
    /// Short polynomials use the subresultant algorithm. For longer ones with small coefficients, a
    /// heuristic first evaluates both at a large integer and recovers the GCD from the GCD of the
    /// values; otherwise, the GCD is found modulo many primes and reconstructed with the Chinese
    /// remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// p.gcd_assign(q);
    /// assert_eq!(p.to_string(), "2*x+2");
    ///
    /// let mut p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// p.gcd_assign(q);
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_gcd` from `fmpz_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd_assign(&mut self, other: Self) {
        *self = gcd_helper(self, &other);
    }
}

impl GcdAssign<&Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its GCD with another, taking the other polynomial by
    /// reference.
    ///
    /// $$
    /// a \gets \gcd(a, b).
    /// $$
    ///
    /// The GCD has a positive leading coefficient, and the GCD of 0 and 0 is 0. It is the product
    /// of the GCD of the contents and the GCD of the primitive parts.
    ///
    /// Short polynomials use the subresultant algorithm. For longer ones with small coefficients, a
    /// heuristic first evaluates both at a large integer and recovers the GCD from the GCD of the
    /// values; otherwise, the GCD is found modulo many primes and reconstructed with the Chinese
    /// remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x+6").unwrap();
    /// let q = IntegerPolynomial::from_str("4*x^2-4").unwrap();
    /// p.gcd_assign(&q);
    /// assert_eq!(p.to_string(), "2*x+2");
    ///
    /// let mut p = IntegerPolynomial::from_str("-x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// p.gcd_assign(&q);
    /// assert_eq!(p.to_string(), "x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_gcd` from `fmpz_poly/gcd.c`, FLINT 3.6.0.
    #[inline]
    fn gcd_assign(&mut self, other: &Self) {
        *self = gcd_helper(self, other);
    }
}
