// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! Exercises the FFT multiplication and checks it against products assembled only from
//! multiplications below the FFT threshold, printing a digest of every result.
//!
//! The point is the AVX2 instantiation of the FFT (`malachite-nz/src/natural/arithmetic/mul/fft/`),
//! which an Apple Silicon machine can only run under emulation: `x86-fft-check.sh` builds this for
//! `x86_64-unknown-linux-musl`, runs it in an amd64 container under QEMU, and compares the output
//! with the native run. The first line says which kernels ran. `quick` as the only argument skips
//! the larger cases.

use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use std::time::Instant;

// Below the FFT threshold (389 limbs), so chunk products never take the FFT path.
const CHUNK: usize = 256;

fn limbs(n: usize, seed: u64) -> Vec<u64> {
    let mut x = seed | 1;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        })
        .collect()
}

// x * y from products of 256-limb chunks (Toom at most), shifted and added.
fn chunked_product(xs: &[u64], ys: &[u64]) -> Natural {
    let mut acc = Natural::from(0u32);
    for (i, xc) in xs.chunks(CHUNK).enumerate() {
        let xc = Natural::from_limbs_asc(xc);
        for (j, yc) in ys.chunks(CHUNK).enumerate() {
            let yc = Natural::from_limbs_asc(yc);
            acc += (&xc * yc) << (64 * CHUNK * (i + j)) as u64;
        }
    }
    acc
}

fn digest(x: &Natural) -> String {
    let ls = x.to_limbs_asc();
    let h = ls.iter().fold(0xcbf29ce484222325u64, |h, &l| {
        (h ^ l).wrapping_mul(0x100000001b3)
    });
    format!(
        "bits={} lo={:016x} hi={:016x} fnv={:016x}",
        x.significant_bits(),
        ls[0],
        ls[ls.len() - 1],
        h
    )
}

// Polynomials with 600 coefficients of about 300 bits take the polynomial FFT (len >= 100 always
// prefers it); the reference is the quadratic schoolbook product of small coefficients.
fn polynomial_case() -> (bool, String) {
    let coefficient = |limbs: &[u64], k: usize| {
        let n = Natural::from_limbs_asc(&limbs[5 * k..5 * k + 5]);
        if k % 3 == 0 {
            -Integer::from(n)
        } else {
            Integer::from(n)
        }
    };
    let xl = limbs(3000, 0x1234567);
    let yl = limbs(3000, 0x89abcdef);
    let xs: Vec<Integer> = (0..600).map(|k| coefficient(&xl, k)).collect();
    let ys: Vec<Integer> = (0..600).map(|k| coefficient(&yl, k)).collect();
    let mut expected = vec![Integer::from(0u32); 1199];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            expected[i + j] += x * y;
        }
    }
    let p =
        IntegerPolynomial::from_coefficients_asc(xs) * IntegerPolynomial::from_coefficients_asc(ys);
    let got = p.into_coefficients_asc();
    let h = got.iter().fold(0xcbf29ce484222325u64, |h, c| {
        c.unsigned_abs_ref()
            .to_limbs_asc()
            .iter()
            .fold(h, |h, &l| (h ^ l).wrapping_mul(0x100000001b3))
    });
    (
        got == expected,
        format!("coeffs={} fnv={h:016x}", got.len()),
    )
}

fn main() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    println!(
        "x86: avx2={} fma={}",
        std::is_x86_feature_detected!("avx2"),
        std::is_x86_feature_detected!("fma")
    );
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    println!("non-x86: baseline kernels");
    let quick = std::env::args().nth(1).as_deref() == Some("quick");
    // (xs limbs, ys limbs): balanced and unbalanced sizes across the FFT's profiles.
    let mut cases = vec![
        (400, 400),
        (389, 1),
        (1000, 1000),
        (3000, 700),
        (10000, 10000),
    ];
    if !quick {
        cases.push((20000, 20000));
        cases.push((60000, 1000));
    }
    let mut ok = true;
    for (seed, &(n, m)) in cases.iter().enumerate() {
        let xs = limbs(n, 0x9e3779b97f4a7c15 ^ seed as u64);
        let ys = limbs(m, 0xd1b54a32d192ed03 ^ seed as u64);
        let x = Natural::from_limbs_asc(&xs);
        let y = Natural::from_limbs_asc(&ys);
        let t = Instant::now();
        let p = &x * &y;
        let s = (&x).square();
        let elapsed = t.elapsed().as_secs_f64();
        let p_ref = chunked_product(&xs, &ys);
        let s_ref = chunked_product(&xs, &xs);
        let good = p == p_ref && s == s_ref;
        ok &= good;
        println!(
            "{n:>6} x {m:<6} mul: {}  square: {}  [{}] ({elapsed:.3}s fft)",
            digest(&p),
            digest(&s),
            if good { "ok" } else { "MISMATCH" }
        );
    }
    if !quick {
        // Random sizes just above the FFT threshold exercise the truncated transforms' many
        // length-dependent paths. Only a digest of all the products is printed.
        let mut r = 0x2545f4914f6cdd1du64;
        let mut h = 0xcbf29ce484222325u64;
        let mut all = true;
        for _ in 0..300 {
            r ^= r << 13;
            r ^= r >> 7;
            r ^= r << 17;
            let n = 389 + (r % 2612) as usize;
            let m = 389 + ((r >> 20) % 2612) as usize;
            let xs = limbs(n, r);
            let ys = limbs(m, r.rotate_left(32));
            let p = Natural::from_limbs_asc(&xs) * Natural::from_limbs_asc(&ys);
            all &= p == chunked_product(&xs, &ys);
            h = p
                .to_limbs_asc()
                .iter()
                .fold(h, |h, &l| (h ^ l).wrapping_mul(0x100000001b3));
        }
        ok &= all;
        println!(
            "300 random sizes in [389, 3000]: fnv={h:016x}  [{}]",
            if all { "ok" } else { "MISMATCH" }
        );
    }
    let (good, d) = polynomial_case();
    ok &= good;
    println!(
        "poly 600 x 600 (300-bit): {d}  [{}]",
        if good { "ok" } else { "MISMATCH" }
    );
    if ok {
        println!("ALL OK");
    } else {
        println!("FAILED");
        std::process::exit(1);
    }
}
