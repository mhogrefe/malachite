// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Generators for the Schönhage–Strassen code in `natural::arithmetic::mul::schonhage_strassen`.
//
// Most inputs there are residues modulo $2^{\text{limbs}\cdot\text{W}} + 1$, each `limbs + 1` limbs
// long, whose top limb is a small signed overflow. Each generator here is described by
//
// - its parameters `P` (sizes, shifts, and flags), listed in full for the exhaustive generator and
//   drawn by a sampling function for the random ones;
// - the number of data limbs and of top limbs that the parameters call for; and
// - a function that builds the inputs from the parameters and the data, consuming each data limb
//   once, so that distinct parameters and data give distinct inputs.
//
// The data limbs come from `exhaustive_unsigneds`, `random_primitive_ints`, or
// `striped_random_unsigneds`, and the top limbs from the range $[-3, 3]$.

use crate::integer::Integer;
use crate::integer::exhaustive::exhaustive_integers;
use crate::integer::random::{random_integers, striped_random_integers};
use crate::integer_polynomial::arithmetic::vec::max_limbs::vec_max_limbs;
use crate::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1::*;
use crate::platform::{Limb, SignedLimb};
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::exhaustive::{
    exhaustive_signed_inclusive_range, exhaustive_unsigneds, primitive_int_increasing_range,
};
use malachite_base::num::iterators::bit_distributor_sequence;
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, geometric_random_unsigneds,
};
use malachite_base::num::random::striped::striped_random_unsigneds;
use malachite_base::num::random::{
    RandomPrimitiveInts, random_primitive_ints, random_signed_inclusive_range,
};
use malachite_base::random::{EXAMPLE_SEED, Seed};
use malachite_base::test_util::generators::common::{GenConfig, It};
use malachite_base::tuples::exhaustive::{
    ExhaustiveDependentPairsYsGenerator, exhaustive_dependent_pairs, exhaustive_pairs,
};
use malachite_base::vecs::exhaustive::{exhaustive_vecs, exhaustive_vecs_fixed_length_from_single};
use malachite_base::vecs::random::random_vecs;

// -- machinery --

const TOP_MAX: SignedLimb = 3;

// The numbers of data limbs and of top limbs that some parameters call for.
type Lens<P> = fn(&P) -> (usize, usize);

// Builds inputs from parameters, data limbs, and top limbs.
type Build<P, T> = fn(P, Vec<Limb>, Vec<Limb>) -> T;

struct SsDataGenerator<P> {
    lens: Lens<P>,
}

impl<P: Clone>
    ExhaustiveDependentPairsYsGenerator<P, (Vec<Limb>, Vec<Limb>), It<(Vec<Limb>, Vec<Limb>)>>
    for SsDataGenerator<P>
{
    fn get_ys(&self, p: &P) -> It<(Vec<Limb>, Vec<Limb>)> {
        let (lows, tops) = (self.lens)(p);
        Box::new(exhaustive_pairs(
            exhaustive_vecs_fixed_length_from_single(
                u64::exact_from(lows),
                exhaustive_unsigneds::<Limb>(),
            ),
            exhaustive_vecs_fixed_length_from_single(
                u64::exact_from(tops),
                exhaustive_signed_inclusive_range(-TOP_MAX, TOP_MAX).map(Limb::wrapping_from),
            ),
        ))
    }
}

fn exhaustive_ss<P: Clone + 'static, T: 'static>(
    ps: Vec<P>,
    lens: Lens<P>,
    build: Build<P, T>,
) -> It<T> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            ps.into_iter(),
            SsDataGenerator { lens },
        )
        .map(move |(p, (lows, tops))| build(p, lows, tops)),
    )
}

// The source of randomness for sampling parameters.
pub(crate) struct SsRng {
    geometric: GeometricRandomNaturalValues<u64>,
    uniform: RandomPrimitiveInts<u64>,
}

impl SsRng {
    // A geometrically distributed small number, whose mean is set by the config.
    fn small(&mut self) -> u64 {
        self.geometric.next().unwrap()
    }

    // A number less than `n`, which must be positive.
    fn below(&mut self, n: u64) -> u64 {
        self.uniform.next().unwrap() % n
    }

    fn one_in(&mut self, n: u64) -> bool {
        self.below(n) == 0
    }
}

struct RandomSs<P, T> {
    rng: SsRng,
    params: fn(&mut SsRng) -> Option<P>,
    lows: Box<dyn Iterator<Item = Limb>>,
    tops: Box<dyn Iterator<Item = Limb>>,
    lens: Lens<P>,
    build: Build<P, T>,
}

impl<P, T> Iterator for RandomSs<P, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            if let Some(p) = (self.params)(&mut self.rng) {
                let (lows, tops) = (self.lens)(&p);
                let lows = (&mut self.lows).take(lows).collect();
                let tops = (&mut self.tops).take(tops).collect();
                return Some((self.build)(p, lows, tops));
            }
        }
    }
}

fn ss_rng(seed: Seed, config: &GenConfig) -> SsRng {
    SsRng {
        geometric: geometric_random_unsigneds(
            seed.fork("small"),
            config.get_or("mean_small_n", 3),
            config.get_or("mean_small_d", 1),
        ),
        uniform: random_primitive_ints(seed.fork("uniform")),
    }
}

fn random_ss<P: 'static, T: 'static>(
    config: &GenConfig,
    striped: bool,
    params: fn(&mut SsRng) -> Option<P>,
    lens: Lens<P>,
    build: Build<P, T>,
) -> It<T> {
    let seed = EXAMPLE_SEED;
    let lows: Box<dyn Iterator<Item = Limb>> = if striped {
        Box::new(striped_random_unsigneds(
            seed.fork("lows"),
            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
    } else {
        Box::new(random_primitive_ints(seed.fork("lows")))
    };
    Box::new(RandomSs {
        rng: ss_rng(seed, config),
        params,
        lows,
        tops: Box::new(
            random_signed_inclusive_range(seed.fork("tops"), -TOP_MAX, TOP_MAX)
                .map(Limb::wrapping_from),
        ),
        lens,
        build,
    })
}

// Defines the exhaustive, random, and special random versions of a generator.
macro_rules! ss_generator {
    (
        $t: ty,
        $exhaustive: ident,
        $random: ident,
        $special_random: ident,
        $params: expr,
        $random_params: expr,
        $lens: expr,
        $build: expr
    ) => {
        pub fn $exhaustive() -> It<$t> {
            exhaustive_ss($params, $lens, $build)
        }

        pub fn $random(config: &GenConfig) -> It<$t> {
            random_ss(config, false, $random_params, $lens, $build)
        }

        pub fn $special_random(config: &GenConfig) -> It<$t> {
            random_ss(config, true, $random_params, $lens, $build)
        }
    };
}

// -- building blocks --

const W: u64 = Limb::WIDTH;

const LW: u64 = Limb::LOG_WIDTH;

// Consumes `limbs` data limbs and one top limb, making a residue.
fn residue(
    lows: &mut impl Iterator<Item = Limb>,
    tops: &mut impl Iterator<Item = Limb>,
    limbs: usize,
) -> Vec<Limb> {
    let mut r: Vec<Limb> = lows.take(limbs).collect();
    r.push(tops.next().unwrap());
    r
}

// Consumes `limbs` data limbs, unless `max`, making a normalized residue: either one whose top limb
// is zero, or, if `max`, $2^{\text{limbs}\cdot\text{W}}$.
fn normalized_residue(lows: &mut impl Iterator<Item = Limb>, limbs: usize, max: bool) -> Vec<Limb> {
    if max {
        let mut r = vec![0; limbs];
        r.push(1);
        r
    } else {
        let mut r: Vec<Limb> = lows.take(limbs).collect();
        r.push(0);
        r
    }
}

fn residues(lows: Vec<Limb>, tops: Vec<Limb>, count: usize, limbs: usize) -> Vec<Vec<Limb>> {
    let mut lows = lows.into_iter();
    let mut tops = tops.into_iter();
    (0..count)
        .map(|_| residue(&mut lows, &mut tops, limbs))
        .collect()
}

// Whether a transform of length `2 << log_n` can use residues of `limbs` limbs: whether $n$ divides
// $\text{limbs}\cdot\text{W}$.
const fn transform_fits(log_n: u64, limbs: usize) -> bool {
    ((limbs as u64) << LW).trailing_zeros() as u64 >= log_n
}

const fn transform_w(log_n: u64, limbs: usize) -> u64 {
    ((limbs as u64) << LW) >> log_n
}

fn limbs_list(max: usize) -> Vec<usize> {
    (1..=max).collect()
}

// The numbers of limbs above `FFT_MULMOD_2EXPP1_CUTOFF` that `fft_mulmod_2expp1` accepts, in
// increasing order.
fn large_mulmod_limbs() -> impl Iterator<Item = usize> {
    (FFT_MULMOD_2EXPP1_CUTOFF + 1..).filter(|&l| fft_adjust_limbs(l) == l)
}

// A number of limbs that `fft_mulmod_2expp1` accepts: small ones usually, and occasionally one of
// the smallest above the cutoff.
fn random_mulmod_limbs(r: &mut SsRng) -> usize {
    if r.one_in(16) {
        large_mulmod_limbs()
            .nth(usize::exact_from(r.below(3)))
            .unwrap()
    } else {
        usize::exact_from(r.small()) + 1
    }
}

// -- residue functions --

// `(t, limbs)`: a residue.
ss_generator!(
    (Vec<Limb>, usize),
    exhaustive_large_type_gen_var_29,
    random_large_type_gen_var_29,
    special_random_large_type_gen_var_29,
    limbs_list(6),
    |r| Some(usize::exact_from(r.small()) + 1),
    |&limbs| (limbs, 1),
    |limbs, lows, tops| (residues(lows, tops, 1, limbs).pop().unwrap(), limbs)
);

// `(a, limbs)`: a normalized residue.
ss_generator!(
    (Vec<Limb>, usize),
    exhaustive_large_type_gen_var_30,
    random_large_type_gen_var_30,
    special_random_large_type_gen_var_30,
    (1..=6)
        .flat_map(|limbs| [(limbs, false), (limbs, true)])
        .collect(),
    |r| Some((usize::exact_from(r.small()) + 1, r.one_in(8))),
    |&(limbs, max)| (if max { 0 } else { limbs }, 0),
    |(limbs, max), lows, _| (normalized_residue(&mut lows.into_iter(), limbs, max), limbs)
);

// `(t, limbs, d)`: a residue and a shift less than `Limb::WIDTH`.
ss_generator!(
    (Vec<Limb>, usize, u64),
    exhaustive_large_type_gen_var_31,
    random_large_type_gen_var_31,
    special_random_large_type_gen_var_31,
    (1..=4)
        .flat_map(|limbs| (0..W).map(move |d| (limbs, d)))
        .collect(),
    |r| Some((usize::exact_from(r.small()) + 1, r.below(W))),
    |&(limbs, _)| (limbs, 1),
    |(limbs, d), lows, tops| (residues(lows, tops, 1, limbs).pop().unwrap(), limbs, d)
);

// `(r, limbs, c)`: a residue and any limb.
ss_generator!(
    (Vec<Limb>, usize, Limb),
    exhaustive_large_type_gen_var_32,
    random_large_type_gen_var_32,
    special_random_large_type_gen_var_32,
    limbs_list(6),
    |r| Some(usize::exact_from(r.small()) + 1),
    |&limbs| (limbs + 1, 1),
    |limbs, mut lows, tops| {
        let c = lows.pop().unwrap();
        (residues(lows, tops, 1, limbs).pop().unwrap(), limbs, c)
    }
);

// `(x, y)`: two slices of the same positive length.
ss_generator!(
    (Vec<Limb>, Vec<Limb>),
    exhaustive_large_type_gen_var_33,
    random_large_type_gen_var_33,
    special_random_large_type_gen_var_33,
    limbs_list(8),
    |r| Some(usize::exact_from(r.small()) + 1),
    |&n| (n << 1, 0),
    |n, mut xs, _| {
        let ys = xs.split_off(n);
        (xs, ys)
    }
);

// -- twiddle factors and butterflies --

// All `(limbs, w, i)` with `i * w < limbs * Limb::WIDTH`, for small `limbs` and `w`.
fn adjust_params(max_limbs: usize, count: usize) -> Vec<(usize, u64, usize, usize)> {
    let mut ps = Vec::new();
    for limbs in 1..=max_limbs {
        for w in 1..=W << 1 {
            let bound = usize::exact_from((u64::exact_from(limbs) << LW).div_ceil(w));
            for i in 0..bound {
                ps.push((limbs, w, i, count));
            }
        }
    }
    ps
}

fn random_adjust_params(r: &mut SsRng, count: usize) -> (usize, u64, usize, usize) {
    let limbs = usize::exact_from(r.small()) + 1;
    let w = r.small() + 1;
    let bound = (u64::exact_from(limbs) << LW).div_ceil(w);
    (limbs, w, usize::exact_from(r.below(bound)), count)
}

// All `(n, w, i)` for the square-root-of-2 twiddle factors: `n` is a power of 2 at least
// `Limb::WIDTH`, `w` is odd, `limbs = n * w / Limb::WIDTH`, and `i` is odd and less than `2 * n`.
fn sqrt2_params(count: usize) -> Vec<(usize, u64, usize, usize)> {
    let mut ps = Vec::new();
    for log_n in LW..=LW + 1 {
        let n = 1usize << log_n;
        for w in (1..=5).step_by(2) {
            for i in (1..n << 1).step_by(2) {
                ps.push((n, w, i, count));
            }
        }
    }
    ps
}

fn random_sqrt2_params(r: &mut SsRng, count: usize) -> (usize, u64, usize, usize) {
    let n = 1usize << (LW + r.below(2));
    let w = (r.small() << 1) + 1;
    let i = (usize::exact_from(r.below(u64::exact_from(n))) << 1) + 1;
    (n, w, i, count)
}

const fn sqrt2_limbs(n: usize, w: u64) -> usize {
    ((n as u64 * w) >> LW) as usize
}

// `(i1, i, limbs, w)`: a residue and the parameters of `fft_adjust`.
ss_generator!(
    (Vec<Limb>, usize, usize, u64),
    exhaustive_large_type_gen_var_34,
    random_large_type_gen_var_34,
    special_random_large_type_gen_var_34,
    adjust_params(3, 1),
    |r| Some(random_adjust_params(r, 1)),
    |&(limbs, _, _, _)| (limbs, 1),
    |(limbs, w, i, _), lows, tops| (residues(lows, tops, 1, limbs).pop().unwrap(), i, limbs, w)
);

// `(i1, i, limbs, w)`: a residue and the parameters of `fft_adjust_sqrt2`.
ss_generator!(
    (Vec<Limb>, usize, usize, u64),
    exhaustive_large_type_gen_var_35,
    random_large_type_gen_var_35,
    special_random_large_type_gen_var_35,
    sqrt2_params(1),
    |r| Some(random_sqrt2_params(r, 1)),
    |&(n, w, _, _)| (sqrt2_limbs(n, w), 1),
    |(n, w, i, _), lows, tops| {
        let limbs = sqrt2_limbs(n, w);
        (residues(lows, tops, 1, limbs).pop().unwrap(), i, limbs, w)
    }
);

// `(i1, i2, limbs, x, y)`: two residues and limb shifts less than `limbs`.
ss_generator!(
    (Vec<Limb>, Vec<Limb>, usize, usize, usize),
    exhaustive_large_type_gen_var_36,
    random_large_type_gen_var_36,
    special_random_large_type_gen_var_36,
    (1..=4)
        .flat_map(|limbs| (0..limbs).flat_map(move |x| (0..limbs).map(move |y| (limbs, x, y))))
        .collect(),
    |r| {
        let limbs = r.small() + 1;
        Some((
            usize::exact_from(limbs),
            usize::exact_from(r.below(limbs)),
            usize::exact_from(r.below(limbs)),
        ))
    },
    |&(limbs, _, _)| (limbs << 1, 2),
    |(limbs, x, y), lows, tops| {
        let mut rs = residues(lows, tops, 2, limbs);
        let i2 = rs.pop().unwrap();
        (rs.pop().unwrap(), i2, limbs, x, y)
    }
);

// `(i1, i2, i, limbs, w)`: two residues and the parameters of `fft_butterfly`.
ss_generator!(
    (Vec<Limb>, Vec<Limb>, usize, usize, u64),
    exhaustive_large_type_gen_var_37,
    random_large_type_gen_var_37,
    special_random_large_type_gen_var_37,
    adjust_params(3, 2),
    |r| Some(random_adjust_params(r, 2)),
    |&(limbs, _, _, _)| (limbs << 1, 2),
    |(limbs, w, i, _), lows, tops| {
        let mut rs = residues(lows, tops, 2, limbs);
        let i2 = rs.pop().unwrap();
        (rs.pop().unwrap(), i2, i, limbs, w)
    }
);

// `(i1, i2, i, limbs, w)`: two residues and the parameters of `fft_butterfly_sqrt2`.
ss_generator!(
    (Vec<Limb>, Vec<Limb>, usize, usize, u64),
    exhaustive_large_type_gen_var_38,
    random_large_type_gen_var_38,
    special_random_large_type_gen_var_38,
    sqrt2_params(2),
    |r| Some(random_sqrt2_params(r, 2)),
    |&(n, w, _, _)| (sqrt2_limbs(n, w) << 1, 2),
    |(n, w, i, _), lows, tops| {
        let limbs = sqrt2_limbs(n, w);
        let mut rs = residues(lows, tops, 2, limbs);
        let i2 = rs.pop().unwrap();
        (rs.pop().unwrap(), i2, i, limbs, w)
    }
);

// `(s, t, limbs, b1, b2)`: two residues and shifts less than `2 * limbs * Limb::WIDTH`.
ss_generator!(
    (Vec<Limb>, Vec<Limb>, usize, u64, u64),
    exhaustive_large_type_gen_var_39,
    random_large_type_gen_var_39,
    special_random_large_type_gen_var_39,
    (1..=2usize)
        .flat_map(|limbs| {
            let bound = u64::exact_from(limbs) << (LW + 1);
            (0..bound).flat_map(move |b1| (0..bound).map(move |b2| (limbs, b1, b2)))
        })
        .collect(),
    |r| {
        let limbs = r.small() + 1;
        let bound = limbs << (LW + 1);
        Some((usize::exact_from(limbs), r.below(bound), r.below(bound)))
    },
    |&(limbs, _, _)| (limbs << 1, 2),
    |(limbs, b1, b2), lows, tops| {
        let mut rs = residues(lows, tops, 2, limbs);
        let t = rs.pop().unwrap();
        (rs.pop().unwrap(), t, limbs, b1, b2)
    }
);

// -- transforms --

// All `(log_n, limbs)` with `log_n <= max_log_n` and `limbs <= max_limbs` for which a transform of
// length `2 << log_n` fits residues of `limbs` limbs.
fn transform_params(min_log_n: u64, max_log_n: u64, max_limbs: usize) -> Vec<(u64, usize)> {
    let mut ps = Vec::new();
    for log_n in min_log_n..=max_log_n {
        for limbs in 1..=max_limbs {
            if transform_fits(log_n, limbs) {
                ps.push((log_n, limbs));
            }
        }
    }
    ps
}

fn random_transform_params(r: &mut SsRng, min_log_n: u64, max_log_n: u64) -> Option<(u64, usize)> {
    let log_n = min_log_n + r.below(max_log_n - min_log_n + 1);
    let limbs = usize::exact_from(r.small()) + 1;
    if transform_fits(log_n, limbs) {
        Some((log_n, limbs))
    } else {
        None
    }
}

// `(ii, n, w)`: `2 * n` residues, for `fft_radix2`.
ss_generator!(
    (Vec<Vec<Limb>>, usize, u64),
    exhaustive_large_type_gen_var_40,
    random_large_type_gen_var_40,
    special_random_large_type_gen_var_40,
    transform_params(0, 4, 3),
    |r| random_transform_params(r, 0, 5),
    |&(log_n, limbs)| (limbs << (log_n + 1), 2 << log_n),
    |(log_n, limbs), lows, tops| {
        (
            residues(lows, tops, 2 << log_n, limbs),
            1 << log_n,
            transform_w(log_n, limbs),
        )
    }
);

// `(ii, n, w, trunc)`: `2 * n` residues and an even `trunc` with `2 <= trunc <= 2 * n`, for
// `fft_truncate`.
ss_generator!(
    (Vec<Vec<Limb>>, usize, u64, usize),
    exhaustive_large_type_gen_var_41,
    random_large_type_gen_var_41,
    special_random_large_type_gen_var_41,
    transform_params(0, 4, 3)
        .into_iter()
        .flat_map(|(log_n, limbs)| (1..=1usize << log_n).map(move |k| (log_n, limbs, k << 1)))
        .collect(),
    |r| {
        let (log_n, limbs) = random_transform_params(r, 0, 5)?;
        let trunc = usize::exact_from(r.below(1 << log_n) + 1) << 1;
        Some((log_n, limbs, trunc))
    },
    |&(log_n, limbs, _)| (limbs << (log_n + 1), 2 << log_n),
    |(log_n, limbs, trunc), lows, tops| {
        (
            residues(lows, tops, 2 << log_n, limbs),
            1 << log_n,
            transform_w(log_n, limbs),
            trunc,
        )
    }
);

// `(ii, n, w, trunc)`: `4 * n` residues and an even `trunc` with `2 * n < trunc <= 4 * n`, for
// `fft_truncate_sqrt2`. `w` is odd for some of the larger `n`.
ss_generator!(
    (Vec<Vec<Limb>>, usize, u64, usize),
    exhaustive_large_type_gen_var_42,
    random_large_type_gen_var_42,
    special_random_large_type_gen_var_42,
    transform_params(0, LW, 2)
        .into_iter()
        .flat_map(|(log_n, limbs)| {
            let n = 1usize << log_n;
            (1..=n).map(move |k| (log_n, limbs, (n + k) << 1))
        })
        .collect(),
    |r| {
        let (log_n, limbs) = random_transform_params(r, 0, LW)?;
        let n = 1usize << log_n;
        let trunc = (n + usize::exact_from(r.below(u64::exact_from(n))) + 1) << 1;
        Some((log_n, limbs, trunc))
    },
    |&(log_n, limbs, _)| (limbs << (log_n + 2), 4 << log_n),
    |(log_n, limbs, trunc), lows, tops| {
        (
            residues(lows, tops, 4 << log_n, limbs),
            1 << log_n,
            transform_w(log_n, limbs),
            trunc,
        )
    }
);

// `(ii, n, w)`: `2 * n` residues, with `n >= 2`, for `fft_negacyclic`. `w` is odd for some of the
// larger `n`.
ss_generator!(
    (Vec<Vec<Limb>>, usize, u64),
    exhaustive_large_type_gen_var_43,
    random_large_type_gen_var_43,
    special_random_large_type_gen_var_43,
    transform_params(1, LW, 2),
    |r| random_transform_params(r, 1, LW),
    |&(log_n, limbs)| (limbs << (log_n + 1), 2 << log_n),
    |(log_n, limbs), lows, tops| {
        (
            residues(lows, tops, 2 << log_n, limbs),
            1 << log_n,
            transform_w(log_n, limbs),
        )
    }
);

// All `(log_n1, log_n2, limbs)` for a matrix of `n1 = 1 << log_n1` columns and `n2 = 1 << log_n2`
// rows, a transform of length `2 * n = n1 * n2`, and residues of `limbs` limbs.
fn matrix_params(min_log_n1: u64, max_log_n: u64, max_limbs: usize) -> Vec<(u64, u64, usize)> {
    let mut ps = Vec::new();
    for log_n1 in min_log_n1..=max_log_n {
        for log_n2 in 1..=max_log_n + 1 - log_n1 {
            for (log_n, limbs) in
                transform_params(log_n1 + log_n2 - 1, log_n1 + log_n2 - 1, max_limbs)
            {
                assert_eq!(log_n, log_n1 + log_n2 - 1);
                ps.push((log_n1, log_n2, limbs));
            }
        }
    }
    ps
}

fn random_matrix_params(
    r: &mut SsRng,
    min_log_n1: u64,
    max_log_n: u64,
) -> Option<(u64, u64, usize)> {
    let log_n1 = min_log_n1 + r.below(max_log_n - min_log_n1 + 1);
    let log_n2 = 1 + r.below(max_log_n + 1 - log_n1);
    let limbs = usize::exact_from(r.small()) + 1;
    if transform_fits(log_n1 + log_n2 - 1, limbs) {
        Some((log_n1, log_n2, limbs))
    } else {
        None
    }
}

// `(ii, n1, n2, w, c)`: a matrix of `n1 * n2` residues, the `w` of the whole transform, and a
// column `c < n1`, for `fft_radix2_twiddle` on that column.
ss_generator!(
    (Vec<Vec<Limb>>, usize, usize, u64, usize),
    exhaustive_large_type_gen_var_44,
    random_large_type_gen_var_44,
    special_random_large_type_gen_var_44,
    matrix_params(0, 4, 2)
        .into_iter()
        .flat_map(|(log_n1, log_n2, limbs)| {
            (0..1usize << log_n1).map(move |c| (log_n1, log_n2, limbs, c, 0))
        })
        .collect(),
    |r| {
        let (log_n1, log_n2, limbs) = random_matrix_params(r, 0, 5)?;
        let c = usize::exact_from(r.below(1 << log_n1));
        Some((log_n1, log_n2, limbs, c, 0))
    },
    |&(log_n1, log_n2, limbs, _, _)| (limbs << (log_n1 + log_n2), 1 << (log_n1 + log_n2)),
    |(log_n1, log_n2, limbs, c, _), lows, tops| {
        (
            residues(lows, tops, 1 << (log_n1 + log_n2), limbs),
            1 << log_n1,
            1 << log_n2,
            transform_w(log_n1 + log_n2 - 1, limbs),
            c,
        )
    }
);

// `(ii, n1, n2, w, c, trunc)`: as for `fft_radix2_twiddle`, and an even `trunc` with `2 <= trunc <=
// n2`, for `fft_truncate1_twiddle` on column `c`.
ss_generator!(
    (Vec<Vec<Limb>>, usize, usize, u64, usize, usize),
    exhaustive_large_type_gen_var_45,
    random_large_type_gen_var_45,
    special_random_large_type_gen_var_45,
    matrix_params(0, 4, 2)
        .into_iter()
        .flat_map(|(log_n1, log_n2, limbs)| {
            (0..1usize << log_n1).flat_map(move |c| {
                (1..=1usize << (log_n2 - 1)).map(move |k| (log_n1, log_n2, limbs, c, k << 1))
            })
        })
        .collect(),
    |r| {
        let (log_n1, log_n2, limbs) = random_matrix_params(r, 0, 5)?;
        let c = usize::exact_from(r.below(1 << log_n1));
        let trunc = usize::exact_from(r.below(1 << (log_n2 - 1)) + 1) << 1;
        Some((log_n1, log_n2, limbs, c, trunc))
    },
    |&(log_n1, log_n2, limbs, _, _)| (limbs << (log_n1 + log_n2), 1 << (log_n1 + log_n2)),
    |(log_n1, log_n2, limbs, c, trunc), lows, tops| {
        (
            residues(lows, tops, 1 << (log_n1 + log_n2), limbs),
            1 << log_n1,
            1 << log_n2,
            transform_w(log_n1 + log_n2 - 1, limbs),
            c,
            trunc,
        )
    }
);

// All `(log_n1, log_n2, limbs, trunc)` for a matrix Fourier transform of length `4 * n`, where `2 *
// n = n1 * n2`, with `trunc` a multiple of `2 * n1` and `2 * n < trunc <= 4 * n`.
fn mfa_params(max_log_n: u64, max_limbs: usize) -> Vec<(u64, u64, usize, usize, bool)> {
    let mut ps = Vec::new();
    for (log_n1, log_n2, limbs) in matrix_params(1, max_log_n, max_limbs) {
        let n = 1usize << (log_n1 + log_n2 - 1);
        for k in 1..=1usize << (log_n2 - 1) {
            let trunc = (n << 1) + (k << (log_n1 + 1));
            ps.push((log_n1, log_n2, limbs, trunc, false));
            ps.push((log_n1, log_n2, limbs, trunc, true));
        }
    }
    ps
}

fn random_mfa_params(r: &mut SsRng) -> Option<(u64, u64, usize, usize, bool)> {
    let (log_n1, log_n2, limbs) = random_matrix_params(r, 1, LW)?;
    let n = 1usize << (log_n1 + log_n2 - 1);
    let k = usize::exact_from(r.below(1 << (log_n2 - 1)) + 1);
    Some((
        log_n1,
        log_n2,
        limbs,
        (n << 1) + (k << (log_n1 + 1)),
        r.one_in(4),
    ))
}

// `(ii, n, w, n1, trunc)`: `4 * n` residues and the parameters of `fft_mfa_truncate_sqrt2_outer`.
// `w` is odd for some of the larger `n`.
ss_generator!(
    (Vec<Vec<Limb>>, usize, u64, usize, usize),
    exhaustive_large_type_gen_var_46,
    random_large_type_gen_var_46,
    special_random_large_type_gen_var_46,
    mfa_params(LW, 2).into_iter().filter(|p| !p.4).collect(),
    random_mfa_params,
    |&(log_n1, log_n2, limbs, _, _)| (limbs << (log_n1 + log_n2 + 1), 2 << (log_n1 + log_n2)),
    |(log_n1, log_n2, limbs, trunc, _), lows, tops| {
        let log_n = log_n1 + log_n2 - 1;
        (
            residues(lows, tops, 4 << log_n, limbs),
            1 << log_n,
            transform_w(log_n, limbs),
            1 << log_n1,
            trunc,
        )
    }
);

// `(ii, jj, n, w, n1, trunc)`: `4 * n` residues, another `4 * n` residues unless squaring, and the
// parameters of `fft_mfa_truncate_sqrt2_inner`. `w` is odd for some of the larger `n`.
ss_generator!(
    (
        Vec<Vec<Limb>>,
        Option<Vec<Vec<Limb>>>,
        usize,
        u64,
        usize,
        usize
    ),
    exhaustive_large_type_gen_var_47,
    random_large_type_gen_var_47,
    special_random_large_type_gen_var_47,
    mfa_params(LW, 2),
    random_mfa_params,
    |&(log_n1, log_n2, limbs, _, square)| {
        let count = 2 << (log_n1 + log_n2);
        let count = if square { count } else { count << 1 };
        (limbs * count, count)
    },
    |(log_n1, log_n2, limbs, trunc, square), lows, tops| {
        let log_n = log_n1 + log_n2 - 1;
        let count = 4 << log_n;
        let mut ii = residues(lows, tops, if square { count } else { count << 1 }, limbs);
        let jj = if square {
            None
        } else {
            Some(ii.split_off(count))
        };
        (
            ii,
            jj,
            1 << log_n,
            transform_w(log_n, limbs),
            1 << log_n1,
            trunc,
        )
    }
);

// -- pointwise products --

// Generates the inputs of `flint_mpn_mulmod_2expp1_basecase`: `(xs, ys, c, b)`, where `xs` and `ys`
// (or just `xs`, if `ys` is `None`) are fully reduced modulo $2^b + 1$, given by their low `ceil(b
// / Limb::WIDTH)` limbs and a top bit, bit 1 of `c` for `xs` and bit 0 for `ys`.
//
// The data of each nonmaximal input is its limbs below the top limb, and the top limb, which is
// less than $2^{b \bmod \text{W}}$ unless `b` is a multiple of W; the top limbs are drawn as top
// limbs, from the right range, so that each input is generated once.
#[allow(clippy::type_complexity)]
fn basecase_params() -> Vec<(u64, Limb, bool)> {
    let mut ps = Vec::new();
    for b in 1..=3 * W {
        for c in 0..4 {
            ps.push((b, c, false));
        }
        ps.push((b, 0, true));
        ps.push((b, 3, true));
    }
    ps
}

// The number of limbs in the inputs of the basecase, and the number of bits in the top one.
const fn basecase_sizes(b: u64) -> (usize, u64) {
    let n = b.div_ceil(W) as usize;
    let r = b & (W - 1);
    (n, if r == 0 { W } else { r })
}

// Which inputs are nonmaximal.
const fn basecase_free(c: Limb, square: bool) -> (bool, bool) {
    (c & 2 == 0, !square && c & 1 == 0)
}

struct BasecaseDataGenerator;

impl ExhaustiveDependentPairsYsGenerator<(u64, Limb, bool), Vec<Limb>, It<Vec<Limb>>>
    for BasecaseDataGenerator
{
    fn get_ys(&self, &(b, c, square): &(u64, Limb, bool)) -> It<Vec<Limb>> {
        let (n, top_bits) = basecase_sizes(b);
        let (x_free, y_free) = basecase_free(c, square);
        let free = usize::from(x_free) + usize::from(y_free);
        let top_bound = if top_bits == W { 0 } else { 1 << top_bits };
        let tops: It<Limb> = if top_bound == 0 {
            Box::new(exhaustive_unsigneds::<Limb>())
        } else {
            Box::new(primitive_int_increasing_range(0, top_bound))
        };
        Box::new(
            exhaustive_pairs(
                exhaustive_vecs_fixed_length_from_single(
                    u64::exact_from((n - 1) * free),
                    exhaustive_unsigneds::<Limb>(),
                ),
                exhaustive_vecs_fixed_length_from_single(u64::exact_from(free), tops),
            )
            .map(|(mut lows, tops)| {
                lows.extend(tops);
                lows
            }),
        )
    }
}

// Builds the inputs from `(n - 1) * free` limbs below the top limbs followed by `free` top limbs.
#[allow(clippy::type_complexity)]
fn build_basecase(
    (b, c, square): (u64, Limb, bool),
    mut data: Vec<Limb>,
) -> (Vec<Limb>, Option<Vec<Limb>>, Limb, u64) {
    let n = basecase_sizes(b).0;
    let (x_free, y_free) = basecase_free(c, square);
    let free = usize::from(x_free) + usize::from(y_free);
    let mut tops = data.split_off((n - 1) * free).into_iter();
    let mut lows = data.into_iter();
    let mut input = |free: bool| {
        if free {
            let mut xs: Vec<Limb> = (&mut lows).take(n - 1).collect();
            xs.push(tops.next().unwrap());
            xs
        } else {
            vec![0; n]
        }
    };
    let xs = input(x_free);
    let ys = if square { None } else { Some(input(y_free)) };
    (xs, ys, c, b)
}

#[allow(clippy::type_complexity)]
pub fn exhaustive_large_type_gen_var_48() -> It<(Vec<Limb>, Option<Vec<Limb>>, Limb, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            basecase_params().into_iter(),
            BasecaseDataGenerator,
        )
        .map(|(p, data)| build_basecase(p, data)),
    )
}

#[allow(clippy::type_complexity)]
fn random_basecase(
    config: &GenConfig,
    striped: bool,
) -> It<(Vec<Limb>, Option<Vec<Limb>>, Limb, u64)> {
    let seed = EXAMPLE_SEED;
    let mut r = ss_rng(seed, config);
    let mut lows: Box<dyn Iterator<Item = Limb>> = if striped {
        Box::new(striped_random_unsigneds(
            seed.fork("lows"),
            config.get_or("mean_stripe_n", Limb::WIDTH >> 1),
            config.get_or("mean_stripe_d", 1),
        ))
    } else {
        Box::new(random_primitive_ints(seed.fork("lows")))
    };
    Box::new(core::iter::repeat_with(move || {
        let b = r.small() * W + r.below(W) + 1;
        let square = r.one_in(4);
        let c = Limb::exact_from(if square { r.below(2) * 3 } else { r.below(4) });
        let (n, top_bits) = basecase_sizes(b);
        let (x_free, y_free) = basecase_free(c, square);
        let free = usize::from(x_free) + usize::from(y_free);
        let mut data: Vec<Limb> = (&mut lows).take((n - 1) * free).collect();
        for _ in 0..free {
            let top = lows.next().unwrap();
            data.push(if top_bits == W {
                top
            } else {
                top >> (W - top_bits)
            });
        }
        build_basecase((b, c, square), data)
    }))
}

#[allow(clippy::type_complexity)]
pub fn random_large_type_gen_var_48(
    config: &GenConfig,
) -> It<(Vec<Limb>, Option<Vec<Limb>>, Limb, u64)> {
    random_basecase(config, false)
}

#[allow(clippy::type_complexity)]
pub fn special_random_large_type_gen_var_48(
    config: &GenConfig,
) -> It<(Vec<Limb>, Option<Vec<Limb>>, Limb, u64)> {
    random_basecase(config, true)
}

fn mulmod_params() -> Vec<(usize, u64, bool, bool, bool)> {
    let mut ps = Vec::new();
    for limbs in (1..=4).chain(large_mulmod_limbs().take(2)) {
        for log_w in 0..=LW {
            for (x_max, y_max, square) in [
                (false, false, false),
                (true, false, false),
                (false, true, false),
                (true, true, false),
                (false, false, true),
                (true, true, true),
            ] {
                ps.push((limbs, log_w, x_max, y_max, square));
            }
        }
    }
    ps
}

// `(r, i2, n, w)`: normalized residues of `n * w / Limb::WIDTH` limbs, the second one absent when
// squaring, for `fft_mulmod_2expp1`. The number of limbs is small or one of the smallest that
// `fft_adjust_limbs` leaves unchanged above `FFT_MULMOD_2EXPP1_CUTOFF`.
ss_generator!(
    (Vec<Limb>, Option<Vec<Limb>>, usize, u64),
    exhaustive_large_type_gen_var_49,
    random_large_type_gen_var_49,
    special_random_large_type_gen_var_49,
    mulmod_params(),
    |r| {
        let limbs = random_mulmod_limbs(r);
        let log_w = r.below(LW + 1);
        let square = r.one_in(4);
        let x_max = r.one_in(8);
        let y_max = if square { x_max } else { r.one_in(8) };
        Some((limbs, log_w, x_max, y_max, square))
    },
    |&(limbs, _, x_max, y_max, square)| {
        let free = usize::from(!x_max) + usize::from(!square && !y_max);
        (limbs * free, 0)
    },
    |(limbs, log_w, x_max, y_max, square), lows, _| {
        let mut lows = lows.into_iter();
        let r = normalized_residue(&mut lows, limbs, x_max);
        let i2 = if square {
            None
        } else {
            Some(normalized_residue(&mut lows, limbs, y_max))
        };
        (r, i2, (limbs << LW) >> log_w, 1 << log_w)
    }
);

// The `depth` and `w` that `fft_mulmod_2expp1` passes to `fft_mulmod_2expp1_negacyclic` for `limbs`
// limbs.
fn mulmod_depth_and_w(limbs: usize) -> (u64, u64) {
    let bits = u64::exact_from(limbs) << LW;
    let depth = bits.ceiling_log_base_2().max(1);
    let off = [4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 1, 1]
        [usize::exact_from(depth.clamp(12, 30) - 12)];
    let depth1 = (depth >> 1) - off;
    (depth1, bits >> (depth1 << 1))
}

// `(r1, i2, r_limbs, depth, w)`: residues with top limb zero, the second one absent when squaring,
// for `fft_mulmod_2expp1_negacyclic`, with the `depth` and `w` that `fft_mulmod_2expp1` would
// choose.
ss_generator!(
    (Vec<Limb>, Option<Vec<Limb>>, usize, u64, u64),
    exhaustive_large_type_gen_var_50,
    random_large_type_gen_var_50,
    special_random_large_type_gen_var_50,
    large_mulmod_limbs()
        .take(2)
        .flat_map(|limbs| [(limbs, false), (limbs, true)])
        .collect(),
    |r| {
        let limbs = large_mulmod_limbs()
            .nth(usize::exact_from(r.below(3)))
            .unwrap();
        Some((limbs, r.one_in(4)))
    },
    |&(limbs, square)| (if square { limbs } else { limbs << 1 }, 0),
    |(limbs, square), lows, _| {
        let mut lows = lows.into_iter();
        let r1 = normalized_residue(&mut lows, limbs, false);
        let i2 = if square {
            None
        } else {
            Some(normalized_residue(&mut lows, limbs, false))
        };
        let (depth, w) = mulmod_depth_and_w(limbs);
        (r1, i2, limbs, depth, w)
    }
);

// -- splitting and combining --

// `(limbs, coeff_limbs, output_limbs)`: a nonempty slice to split into coefficients of
// `coeff_limbs` limbs, with `coeff_limbs <= output_limbs + 1`.
ss_generator!(
    (Vec<Limb>, usize, usize),
    exhaustive_large_type_gen_var_51,
    random_large_type_gen_var_51,
    special_random_large_type_gen_var_51,
    (1..=10usize)
        .flat_map(|total| {
            (1..=4usize).flat_map(move |coeff| {
                (coeff - 1..=coeff + 1).map(move |output| (total, coeff, output))
            })
        })
        .collect(),
    |r| {
        let coeff = usize::exact_from(r.small()) + 1;
        let output = coeff - 1 + usize::exact_from(r.small());
        Some((usize::exact_from(r.small() + r.small()) + 1, coeff, output))
    },
    |&(total, _, _)| (total, 0),
    |(_, coeff, output), lows, _| (lows, coeff, output)
);

// `(limbs, bits, output_limbs)`: a nonempty slice to split into coefficients of `bits` bits, with
// `bits / Limb::WIDTH <= output_limbs`.
ss_generator!(
    (Vec<Limb>, u64, usize),
    exhaustive_large_type_gen_var_52,
    random_large_type_gen_var_52,
    special_random_large_type_gen_var_52,
    (1..=6usize)
        .flat_map(|total| {
            (1..=3 * W).flat_map(move |bits| {
                let min = usize::exact_from(bits >> LW);
                (min..=min + 1).map(move |output| (total, bits, output))
            })
        })
        .collect(),
    |r| {
        let bits = r.small() * W + r.below(W) + 1;
        let output = usize::exact_from((bits >> LW) + r.small());
        Some((usize::exact_from(r.small() + r.small()) + 1, bits, output))
    },
    |&(total, _, _)| (total, 0),
    |(_, bits, output), lows, _| (lows, bits, output)
);

// All `(length, x, output_limbs, total_limbs)` for combining `length` coefficients of `output_limbs
// + 1` limbs into `total_limbs` limbs.
fn combine_params(min_x: u64, max_x: u64) -> Vec<(usize, u64, usize, usize)> {
    let mut ps = Vec::new();
    for length in 1..=4 {
        for x in min_x..=max_x {
            for output in 1..=3 {
                for total in 1..=10 {
                    ps.push((length, x, output, total));
                }
            }
        }
    }
    ps
}

fn random_combine_params(r: &mut SsRng, x: u64) -> (usize, u64, usize, usize) {
    (
        usize::exact_from(r.small()) + 1,
        x,
        usize::exact_from(r.small()) + 1,
        usize::exact_from(r.small() + r.small() + r.small()) + 1,
    )
}

const fn combine_lens(&(length, _, output, total): &(usize, u64, usize, usize)) -> (usize, usize) {
    (length * (output + 1) + total, 0)
}

#[allow(clippy::type_complexity)]
fn build_combine(
    (length, x, output, _): (usize, u64, usize, usize),
    mut lows: Vec<Limb>,
    _: Vec<Limb>,
) -> (Vec<Limb>, Vec<Vec<Limb>>, u64, usize) {
    let res = lows.split_off(length * (output + 1));
    let poly = lows.chunks(output + 1).map(<[Limb]>::to_vec).collect();
    (res, poly, x, output)
}

// `(res, poly, coeff_limbs, output_limbs)`: an accumulator and coefficients of `output_limbs + 1`
// limbs to add into it every `coeff_limbs` limbs.
ss_generator!(
    (Vec<Limb>, Vec<Vec<Limb>>, u64, usize),
    exhaustive_large_type_gen_var_53,
    random_large_type_gen_var_53,
    special_random_large_type_gen_var_53,
    combine_params(1, 3),
    |r| {
        let coeff = r.small() + 1;
        Some(random_combine_params(r, coeff))
    },
    combine_lens,
    build_combine
);

// `(res, poly, bits, output_limbs)`: an accumulator and coefficients of `output_limbs + 1` limbs to
// add into it every `bits` bits.
ss_generator!(
    (Vec<Limb>, Vec<Vec<Limb>>, u64, usize),
    exhaustive_large_type_gen_var_54,
    random_large_type_gen_var_54,
    special_random_large_type_gen_var_54,
    combine_params(1, 3 * W),
    |r| {
        let bits = r.small() * W + r.below(W) + 1;
        Some(random_combine_params(r, bits))
    },
    combine_lens,
    build_combine
);

// -- convolution --

// All `(depth, limbs, len1, len2)` for `fft_convolution` with `depth` in the given range: `n = 1 <<
// depth` divides `limbs * Limb::WIDTH`, the inputs are nonzero only in their first `len1` and
// `len2` residues (`len2` is `None` when squaring, and then equal to `len1`), and `trunc`, the
// length of their product, satisfies `2 * n < trunc <= 4 * n`. The truncated transforms need the
// product to be zero past `trunc`, as it is when `trunc` is its length.
fn convolution_params(
    min_depth: u64,
    max_depth: u64,
    max_limbs: usize,
) -> Vec<(u64, usize, usize, Option<usize>)> {
    let mut ps = Vec::new();
    for (depth, limbs) in transform_params(min_depth, max_depth, max_limbs) {
        let n = 1usize << depth;
        for len1 in 1..=n << 2 {
            for len2 in 1..=n << 2 {
                let trunc = len1 + len2 - 1;
                if trunc > n << 1 && trunc <= n << 2 {
                    ps.push((depth, limbs, len1, Some(len2)));
                }
            }
            let trunc = (len1 << 1) - 1;
            if trunc > n << 1 && trunc <= n << 2 {
                ps.push((depth, limbs, len1, None));
            }
        }
    }
    ps
}

fn random_convolution_params(
    r: &mut SsRng,
    min_depth: u64,
    max_depth: u64,
) -> Option<(u64, usize, usize, Option<usize>)> {
    let (depth, limbs) = random_transform_params(r, min_depth, max_depth)?;
    let n = 1usize << depth;
    let len1 = usize::exact_from(r.below(u64::exact_from(n << 2))) + 1;
    if r.one_in(4) {
        let trunc = (len1 << 1) - 1;
        return if trunc > n << 1 && trunc <= n << 2 {
            Some((depth, limbs, len1, None))
        } else {
            None
        };
    }
    // `len2` must make `2 * n < len1 + len2 - 1 <= 4 * n`.
    let lo = ((n << 1) + 2).saturating_sub(len1).max(1);
    let hi = (n << 2) + 1 - len1;
    if lo > hi {
        return None;
    }
    let len2 = lo + usize::exact_from(r.below(u64::exact_from(hi - lo + 1)));
    Some((depth, limbs, len1, Some(len2)))
}

const fn convolution_lens(
    &(_, limbs, len1, len2): &(u64, usize, usize, Option<usize>),
) -> (usize, usize) {
    let count = match len2 {
        Some(len2) => len1 + len2,
        None => len1,
    };
    (limbs * count, count)
}

#[allow(clippy::type_complexity)]
fn build_convolution(
    (depth, limbs, len1, len2): (u64, usize, usize, Option<usize>),
    lows: Vec<Limb>,
    tops: Vec<Limb>,
) -> (Vec<Vec<Limb>>, Option<Vec<Vec<Limb>>>, u64, usize, usize) {
    let count = 4 << depth;
    let mut rs = residues(lows, tops, len1 + len2.unwrap_or(0), limbs);
    let pad = |mut xs: Vec<Vec<Limb>>| {
        xs.resize(count, vec![0; limbs + 1]);
        xs
    };
    let jj = len2.map(|_| pad(rs.split_off(len1)));
    (pad(rs), jj, depth, limbs, len1 + len2.unwrap_or(len1) - 1)
}

// `(ii, jj, depth, limbs, trunc)`: inputs to `fft_convolution` that take its truncated
// square-root-of-2 path (`depth <= 6`): `4 << depth` residues, and the same for `jj` unless
// squaring, as in `convolution_params`.
ss_generator!(
    (Vec<Vec<Limb>>, Option<Vec<Vec<Limb>>>, u64, usize, usize),
    exhaustive_large_type_gen_var_55,
    random_large_type_gen_var_55,
    special_random_large_type_gen_var_55,
    convolution_params(0, 3, 2),
    |r| random_convolution_params(r, 0, 6),
    convolution_lens,
    build_convolution
);

// `(ii, jj, depth, limbs, trunc)`: inputs to `fft_convolution` that take its matrix Fourier path
// (`depth == 7`).
ss_generator!(
    (Vec<Vec<Limb>>, Option<Vec<Vec<Limb>>>, u64, usize, usize),
    exhaustive_large_type_gen_var_56,
    random_large_type_gen_var_56,
    special_random_large_type_gen_var_56,
    convolution_params(7, 7, 4),
    |r| random_convolution_params(r, 7, 7),
    convolution_lens,
    build_convolution
);

// -- getting and setting coefficients --

// `(xs, limbs)`: coefficients and a number of limbs that holds the absolute value of each.
pub fn exhaustive_large_type_gen_var_57() -> It<(Vec<Integer>, usize)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_vecs(exhaustive_integers()),
            exhaustive_unsigneds::<usize>(),
        )
        .map(|(xs, k)| {
            let limbs = usize::exact_from(vec_max_limbs(&xs)) + k;
            (xs, limbs)
        }),
    )
}

fn random_get_fft(config: &GenConfig, striped: bool) -> It<(Vec<Integer>, usize)> {
    let seed = EXAMPLE_SEED;
    let mean_bits_n = config.get_or("mean_bits_n", 256);
    let mean_bits_d = config.get_or("mean_bits_d", 1);
    let xss: It<Vec<Integer>> = if striped {
        let mean_stripe_n = config.get_or("mean_stripe_n", 32);
        let mean_stripe_d = config.get_or("mean_stripe_d", 1);
        Box::new(random_vecs(
            seed.fork("xs"),
            &|seed| {
                striped_random_integers(
                    seed,
                    mean_stripe_n,
                    mean_stripe_d,
                    mean_bits_n,
                    mean_bits_d,
                )
            },
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
    } else {
        Box::new(random_vecs(
            seed.fork("xs"),
            &|seed| random_integers(seed, mean_bits_n, mean_bits_d),
            config.get_or("mean_length_n", 4),
            config.get_or("mean_length_d", 1),
        ))
    };
    let mut r = ss_rng(seed, config);
    Box::new(xss.map(move |xs| {
        let limbs = usize::exact_from(vec_max_limbs(&xs) + r.small());
        (xs, limbs)
    }))
}

pub fn random_large_type_gen_var_57(config: &GenConfig) -> It<(Vec<Integer>, usize)> {
    random_get_fft(config, false)
}

pub fn special_random_large_type_gen_var_57(config: &GenConfig) -> It<(Vec<Integer>, usize)> {
    random_get_fft(config, true)
}

fn set_fft_params() -> Vec<(usize, bool, Vec<bool>)> {
    let mut ps = Vec::new();
    for count in 0..=4usize {
        for limbs in 1..=3 {
            for sign in [false, true] {
                for mask in 0..1usize << count {
                    let maxes = (0..count).map(|i| (mask >> i).odd()).collect();
                    ps.push((limbs, sign, maxes));
                }
            }
        }
    }
    ps
}

// `(coeffs_f, limbs, sign)`: normalized residues to read coefficients from.
ss_generator!(
    (Vec<Vec<Limb>>, usize, bool),
    exhaustive_large_type_gen_var_58,
    random_large_type_gen_var_58,
    special_random_large_type_gen_var_58,
    set_fft_params(),
    |r| {
        let count = usize::exact_from(r.small());
        let limbs = usize::exact_from(r.small()) + 1;
        let maxes: Vec<bool> = (0..count).map(|_| r.one_in(8)).collect();
        Some((limbs, r.one_in(2), maxes))
    },
    |(limbs, _, maxes)| (limbs * maxes.iter().filter(|&&max| !max).count(), 0),
    |(limbs, sign, maxes), lows, _| {
        let mut lows = lows.into_iter();
        let coeffs_f = maxes
            .into_iter()
            .map(|max| normalized_residue(&mut lows, limbs, max))
            .collect();
        (coeffs_f, limbs, sign)
    }
);

// -- the rest --

// `limbs`: numbers of limbs for `fft_adjust_limbs`, up to $2^{40}$.
pub fn exhaustive_large_type_gen_var_59() -> It<usize> {
    Box::new(exhaustive_unsigneds::<usize>().take_while(|&limbs| limbs <= 1 << 40))
}

fn random_adjust_limbs(config: &GenConfig) -> It<usize> {
    Box::new(
        geometric_random_unsigneds::<u64>(
            EXAMPLE_SEED,
            config.get_or("mean_limbs_n", 1000),
            config.get_or("mean_limbs_d", 1),
        )
        .filter(|&limbs| limbs <= 1 << 40)
        .map(usize::exact_from),
    )
}

pub fn random_large_type_gen_var_59(config: &GenConfig) -> It<usize> {
    random_adjust_limbs(config)
}

pub fn special_random_large_type_gen_var_59(config: &GenConfig) -> It<usize> {
    random_adjust_limbs(config)
}
