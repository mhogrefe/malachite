// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Elliptic curve primality certificates for tests and demos, and ways of corrupting them. Without a
// prover, valid certificates come from fixtures: certificates that FLINT's ecpp_prove produced for
// primes of 65 to 512 bits (and one that Azurite produced, for 2^64 + 961), in the Pari format, one
// "n: certificate" per line, and a few in the Flint format, which keeps D.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::natural::factorization::ecpp::point::EcppPoint;
use crate::natural::factorization::ecpp::{EcppCertificate, EcppStep};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Parity, PowerOf2};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::num::modular::traits::{
    ModAdd, ModInverse, ModMul, ModPowerOf2, ModSquare, ModSub,
};
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::Seed;

pub const ECPP_CERTIFICATES_PARI: &str = include_str!("ecpp_certificates.txt");
pub const ECPP_CERTIFICATES_FLINT: &str = include_str!("ecpp_certificates_flint.txt");

// The Pari-format fixtures, as (n, certificate, the certificate's text), smallest n first.
pub fn ecpp_fixtures() -> Vec<(Natural, EcppCertificate, String)> {
    ECPP_CERTIFICATES_PARI
        .lines()
        .map(|line| {
            let (n, s) = line.split_once(": ").unwrap();
            (
                Natural::from_str(n).unwrap(),
                EcppCertificate::from_str(s).unwrap(),
                s.to_string(),
            )
        })
        .collect()
}

// The Flint-format fixtures, as text, each ending with a newline.
pub fn ecpp_flint_fixture_strings() -> Vec<String> {
    ECPP_CERTIFICATES_FLINT
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let mut s = s.trim_end().to_string();
            s.push('\n');
            s
        })
        .collect()
}

// The fields N, t, s, a, x, y of each step of the Pari format.
fn pari_fields(cert: &EcppCertificate) -> Vec<[Integer; 6]> {
    cert.steps
        .iter()
        .map(|st| {
            [
                Integer::from(&st.n),
                Integer::from(&st.n) + Integer::ONE - Integer::from(&st.m),
                Integer::from(&st.m / &st.q),
                Integer::from(&st.a),
                Integer::from(&st.x),
                Integer::from(&st.y),
            ]
        })
        .collect()
}

fn pari_string(fields: &[[Integer; 6]]) -> String {
    let mut s = String::from("[");
    for (i, [n, t, sq, a, x, y]) in fields.iter().enumerate() {
        if i > 0 {
            s.push_str(", ");
        }
        s.push_str(&alloc::format!("[{n}, {t}, {sq}, {a}, [{x}, {y}]]"));
    }
    s.push(']');
    s
}

// A corruption of a fixture, as the input n and the certificate. Certificates are only changed in
// ways the Pari format can express (b always puts the point on the curve), so that every result can
// be printed in that format.
#[derive(Clone, Debug)]
enum CertMutation {
    None,
    // add the value to the input n
    InputN(Integer),
    DropFirst,
    DropLast,
    SwapSteps(usize, usize),
    DuplicateStep(usize),
    // add the value to field f of step i
    Field(usize, usize, Integer),
    // set field f of step i to the value
    SetField(usize, usize, Integer),
}

fn apply_cert_mutation(
    n: &Natural,
    cert: &EcppCertificate,
    mutation: &CertMutation,
) -> Option<(EcppCertificate, Natural)> {
    let mut fields = pari_fields(cert);
    let mut n = n.clone();
    match mutation {
        CertMutation::None => {}
        CertMutation::InputN(d) => {
            n = Natural::try_from(Integer::from(&n) + d).ok()?;
        }
        CertMutation::DropFirst => {
            fields.remove(0);
        }
        CertMutation::DropLast => {
            fields.pop();
        }
        CertMutation::SwapSteps(i, j) => {
            if i == j || *i >= fields.len() || *j >= fields.len() {
                return None;
            }
            fields.swap(*i, *j);
        }
        CertMutation::DuplicateStep(i) => {
            let st = fields.get(*i)?.clone();
            fields.insert(*i, st);
        }
        CertMutation::Field(i, f, d) => {
            *fields.get_mut(*i)?.get_mut(*f)? += d;
        }
        CertMutation::SetField(i, f, v) => {
            *fields.get_mut(*i)?.get_mut(*f)? = v.clone();
        }
    }
    if fields.is_empty() {
        return None;
    }
    Some((EcppCertificate::from_str(&pari_string(&fields)).ok()?, n))
}

fn special_values(x: &Integer) -> Vec<Integer> {
    let mut vs = alloc::vec![
        Integer::NEGATIVE_ONE,
        Integer::ZERO,
        Integer::ONE,
        Integer::TWO,
        Integer::from(3u32),
        x - Integer::ONE,
        x + Integer::ONE,
        -x,
        Integer::power_of_2(64),
    ];
    vs.retain(|v| v != x);
    vs
}

// Every fixture, then, for each fixture in turn, each of a fixed list of corruptions.
pub fn exhaustive_ecpp_certificate_pairs() -> impl Iterator<Item = (EcppCertificate, Natural)> {
    let fixtures = ecpp_fixtures();
    let mut out: Vec<(EcppCertificate, Natural)> = fixtures
        .iter()
        .map(|(n, cert, _)| (cert.clone(), n.clone()))
        .collect();
    for (n, cert, _) in &fixtures {
        let mut mutations = alloc::vec![
            CertMutation::InputN(Integer::TWO),
            CertMutation::InputN(Integer::from(-2)),
            CertMutation::DropFirst,
            CertMutation::DropLast,
            CertMutation::DuplicateStep(0),
        ];
        if cert.steps.len() > 1 {
            mutations.push(CertMutation::SwapSteps(0, 1));
        }
        for i in 0..cert.steps.len() {
            for f in 0..6 {
                for d in [-1, 1] {
                    mutations.push(CertMutation::Field(i, f, Integer::from(d)));
                }
            }
        }
        out.extend(
            mutations
                .iter()
                .filter_map(|m| apply_cert_mutation(n, cert, m)),
        );
    }
    out.into_iter()
}

// A random fixture, unchanged half of the time and otherwise corrupted at random. With `special`,
// fields are set to special values (0, 1, N - 1, N + 1, ...) rather than offset by a power of 2.
pub fn random_ecpp_certificate_pairs(
    seed: Seed,
    special: bool,
) -> impl Iterator<Item = (EcppCertificate, Natural)> {
    let fixtures = ecpp_fixtures();
    let mut rs = random_primitive_ints::<u64>(seed);
    core::iter::repeat(()).filter_map(move |()| {
        let mut r = || rs.next().unwrap();
        let (n, cert, _) = &fixtures[usize::exact_from(r() % fixtures.len() as u64)];
        let len = cert.steps.len();
        let i = usize::exact_from(r() % len as u64);
        let f = usize::exact_from(r() % 6);
        let mutation = match r() % 16 {
            0..8 => CertMutation::None,
            8 => CertMutation::InputN(Integer::from(i64::exact_from(r() % 9) - 4)),
            9 => CertMutation::DropFirst,
            10 => CertMutation::DropLast,
            11 => CertMutation::SwapSteps(i, usize::exact_from(r() % len as u64)),
            12 => CertMutation::DuplicateStep(i),
            _ => {
                let x = &pari_fields(cert)[i][f];
                if special {
                    let vs = special_values(x);
                    CertMutation::SetField(
                        i,
                        f,
                        vs[usize::exact_from(r() % vs.len() as u64)].clone(),
                    )
                } else {
                    let k = r() % (x.significant_bits() + 2);
                    let d = Integer::power_of_2(k);
                    CertMutation::Field(i, f, if r().even() { d } else { -d })
                }
            }
        };
        apply_cert_mutation(n, cert, &mutation)
    })
}

// The steps of all fixtures: those of the Pari-format ones (D = 0) and those of the Flint-format
// ones (which keep D).
pub fn ecpp_fixture_steps() -> Vec<EcppStep> {
    let mut steps: Vec<EcppStep> = ecpp_fixtures()
        .into_iter()
        .flat_map(|(_, cert, _)| cert.steps)
        .collect();
    for s in ecpp_flint_fixture_strings() {
        steps.extend(EcppCertificate::from_str(&s).unwrap().steps);
    }
    steps
}

const fn step_field(st: &mut EcppStep, f: usize) -> &mut Natural {
    match f {
        0 => &mut st.n,
        1 => &mut st.a,
        2 => &mut st.b,
        3 => &mut st.m,
        4 => &mut st.q,
        5 => &mut st.x,
        _ => &mut st.y,
    }
}

// Every fixture step, then each step with one of its seven Natural fields increased or decreased by
// 1.
pub fn exhaustive_ecpp_steps() -> impl Iterator<Item = EcppStep> {
    let steps = ecpp_fixture_steps();
    let mut out = steps.clone();
    for st in &steps {
        for f in 0..7 {
            let mut up = st.clone();
            *step_field(&mut up, f) += Natural::ONE;
            out.push(up);
            let mut down = st.clone();
            let x = step_field(&mut down, f);
            if *x != 0u32 {
                *x -= Natural::ONE;
                out.push(down);
            }
        }
    }
    out.into_iter()
}

// A random fixture step, unchanged half of the time and otherwise with one field changed: by a
// power of 2, or, with `special`, to a special value.
pub fn random_ecpp_steps(seed: Seed, special: bool) -> impl Iterator<Item = EcppStep> {
    let steps = ecpp_fixture_steps();
    let mut rs = random_primitive_ints::<u64>(seed);
    core::iter::repeat(()).filter_map(move |()| {
        let mut r = || rs.next().unwrap();
        let mut st = steps[usize::exact_from(r() % steps.len() as u64)].clone();
        if r().even() {
            return Some(st);
        }
        let f = usize::exact_from(r() % 8);
        if f == 7 {
            st.d += i64::exact_from(r() % 9) - 4;
            return Some(st);
        }
        let x = Integer::from(&*step_field(&mut st, f));
        let v = if special {
            let vs = special_values(&x);
            vs[usize::exact_from(r() % vs.len() as u64)].clone()
        } else {
            let d = Integer::power_of_2(r() % (x.significant_bits() + 2));
            if r().even() { x + d } else { x - d }
        };
        *step_field(&mut st, f) = Natural::try_from(v).ok()?;
        Some(st)
    })
}

// The fixtures as text: each Pari-format one, each Flint-format one, and each Pari-format one
// printed in the Flint format.
pub fn ecpp_fixture_strings() -> Vec<String> {
    let fixtures = ecpp_fixtures();
    let mut out: Vec<String> = fixtures.iter().map(|(_, _, s)| s.clone()).collect();
    out.extend(ecpp_flint_fixture_strings());
    out.extend(fixtures.iter().map(|(_, cert, _)| cert.to_string()));
    out
}

const EDIT_CHARS: [char; 10] = [' ', '-', '+', '0', '9', '[', ']', ',', '\n', 'x'];

fn edit(s: &str, kind: u64, pos: usize, c: char) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let pos = pos.min(chars.len());
    match kind {
        // truncate
        0 => chars.truncate(pos),
        // delete
        1 => {
            if pos < chars.len() {
                chars.remove(pos);
            }
        }
        // insert
        2 => chars.insert(pos, c),
        // replace
        _ => {
            if pos < chars.len() {
                chars[pos] = c;
            }
        }
    }
    chars.into_iter().collect()
}

// Every fixture string, then, for the shortest string of each of the two formats, every truncation
// and deletion, and every replacement with one of a few characters.
pub fn exhaustive_ecpp_strings() -> impl Iterator<Item = String> {
    let strings = ecpp_fixture_strings();
    let mut out = strings.clone();
    let shortest_pari = strings
        .iter()
        .filter(|s| s.starts_with('['))
        .min_by_key(|s| s.len());
    let shortest_flint = strings
        .iter()
        .filter(|s| s.starts_with('e'))
        .min_by_key(|s| s.len());
    for s in [shortest_pari, shortest_flint].into_iter().flatten() {
        let len = s.chars().count();
        for pos in 0..len {
            out.push(edit(s, 0, pos, ' '));
            out.push(edit(s, 1, pos, ' '));
            for c in EDIT_CHARS {
                out.push(edit(s, 3, pos, c));
            }
        }
    }
    out.into_iter()
}

// A random fixture string, unchanged a quarter of the time and otherwise truncated, or with a
// character deleted, inserted, or replaced.
pub fn random_ecpp_strings(seed: Seed) -> impl Iterator<Item = String> {
    let strings = ecpp_fixture_strings();
    let mut rs = random_primitive_ints::<u64>(seed);
    core::iter::repeat(()).map(move |()| {
        let mut r = || rs.next().unwrap();
        let s = &strings[usize::exact_from(r() % strings.len() as u64)];
        let kind = r() % 5;
        if kind == 4 {
            return s.clone();
        }
        let pos = usize::exact_from(r() % (s.len() as u64 + 1));
        let c = EDIT_CHARS[usize::exact_from(r() % EDIT_CHARS.len() as u64)];
        edit(s, kind, pos, c)
    })
}

// The affine coordinates of a Jacobian point modulo a prime n, or None for the point at infinity.
pub fn jacobian_to_affine(p: &EcppPoint, n: &Natural) -> Option<(Natural, Natural)> {
    if p.z == 0u32 {
        return None;
    }
    let zi = (&p.z).mod_inverse(n).unwrap();
    let zi2 = (&zi).mod_square(n);
    let x = (&p.x).mod_mul(&zi2, n);
    let y = (&p.y).mod_mul(zi2.mod_mul(zi, n), n);
    Some((x, y))
}

fn affine_add(
    p: &Option<(Natural, Natural)>,
    q: &Option<(Natural, Natural)>,
    a: &Natural,
    n: &Natural,
) -> Option<(Natural, Natural)> {
    let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
        return if p.is_none() { q.clone() } else { p.clone() };
    };
    let lambda = if x1 == x2 {
        if (y1).mod_add(y2, n) == 0u32 {
            return None;
        }
        // (3 x^2 + a) / (2 y)
        let num = x1
            .mod_square(n)
            .mod_mul(Natural::from(3u32), n)
            .mod_add(a, n);
        num.mod_mul(y1.mod_add(y1, n).mod_inverse(n).unwrap(), n)
    } else {
        y2.mod_sub(y1, n)
            .mod_mul(x2.mod_sub(x1, n).mod_inverse(n).unwrap(), n)
    };
    let x3 = (&lambda).mod_square(n).mod_sub(x1, n).mod_sub(x2, n);
    let y3 = lambda.mod_mul(x1.mod_sub(&x3, n), n).mod_sub(y1, n);
    Some((x3, y3))
}

// k (x, y) on y^2 = x^3 + a x + b modulo a prime n, by affine double-and-add; None for the point at
// infinity.
pub fn ecpp_point_mul_naive(
    x: &Natural,
    y: &Natural,
    k: &Natural,
    a: &Natural,
    n: &Natural,
) -> Option<(Natural, Natural)> {
    let p = Some((x.clone(), y.clone()));
    let mut r = None;
    for i in (0..k.significant_bits()).rev() {
        r = affine_add(&r, &r, a, n);
        if k.get_bit(i) {
            r = affine_add(&r, &p, a, n);
        }
    }
    r
}

/// The width-4 non-adjacent form of `k`, least significant digit first, computed as FLINT's
/// `ecpp_point_mul_gr` computes it: while `k` is nonzero, an odd `k` gives the digit `k` mod 16,
/// taken in `(-8, 8)` and subtracted from `k`, an even one the digit 0, and `k` is halved.
pub fn naf_digits_naive(k: &Natural) -> Vec<i8> {
    let mut naf = Vec::new();
    let mut k = Integer::from(k);
    while k != 0u32 {
        if k.odd() {
            let mut d = i8::exact_from(&(&k).mod_power_of_2(4));
            if d >= 8 {
                d -= 16;
            }
            naf.push(d);
            k -= Integer::from(d);
        } else {
            naf.push(0);
        }
        k >>= 1u32;
    }
    naf
}
