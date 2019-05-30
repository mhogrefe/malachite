use std::iter::repeat;

use malachite_base::comparison::Max;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::{
    CheckedHammingDistance, HammingDistance, SignificantBits,
};
use malachite_nz::integer::logic::checked_hamming_distance_signed_limb::*;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_limb_vec_and_positive_limb_var_1;
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer};

pub fn integer_checked_hamming_distance_signed_limb_alt_1(
    n: &Integer,
    i: SignedLimb,
) -> Option<u64> {
    let negative = i < 0;
    if (*n < 0 as Limb) != negative {
        return None;
    }
    let i = Integer::from(i);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.twos_complement_bits().count() >= i.twos_complement_bits().count() {
            Box::new(
                n.twos_complement_bits()
                    .zip(i.twos_complement_bits().chain(repeat(negative))),
            )
        } else {
            Box::new(
                n.twos_complement_bits()
                    .chain(repeat(negative))
                    .zip(i.twos_complement_bits()),
            )
        };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    Some(distance)
}

pub fn integer_checked_hamming_distance_signed_limb_alt_2(
    n: &Integer,
    i: SignedLimb,
) -> Option<u64> {
    let extension = if i < 0 { Limb::MAX } else { 0 };
    if (*n < 0 as Limb) != (i < 0) {
        return None;
    }
    let i = Integer::from(i);
    let limb_zip: Box<Iterator<Item = (Limb, Limb)>> =
        if n.twos_complement_limbs().count() >= i.twos_complement_limbs().count() {
            Box::new(
                n.twos_complement_limbs()
                    .zip(i.twos_complement_limbs().chain(repeat(extension))),
            )
        } else {
            Box::new(
                n.twos_complement_limbs()
                    .chain(repeat(extension))
                    .zip(i.twos_complement_limbs()),
            )
        };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    Some(distance)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_limb_neg);
    register_demo!(registry, demo_integer_checked_hamming_distance_signed_limb);
    register_demo!(registry, demo_signed_limb_checked_hamming_distance_integer);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_limb_neg);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_checked_hamming_distance_integer
    );
}

fn demo_limbs_hamming_distance_limb_neg(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_limb_vec_and_positive_limb_var_1(gm).take(limit) {
        println!(
            "limbs_hamming_distance_limb_neg({:?}, {}) = {}",
            limbs,
            limb,
            limbs_hamming_distance_limb_neg(limbs, limb)
        );
    }
}

fn demo_integer_checked_hamming_distance_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<SignedLimb>(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            n,
            i,
            n.checked_hamming_distance(i)
        );
    }
}

fn demo_signed_limb_checked_hamming_distance_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<SignedLimb>(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            i,
            n,
            i.checked_hamming_distance(&n)
        );
    }
}

fn benchmark_limbs_hamming_distance_limb_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance_limb_neg(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_limb_vec_and_positive_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, limb)| no_out!(limbs_hamming_distance_limb_neg(limbs, limb))),
        )],
    );
}

fn benchmark_integer_checked_hamming_distance_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_hamming_distance(SignedLimb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(ref n, other)| no_out!(n.checked_hamming_distance(other))),
            ),
            (
                "using bits explicitly",
                &mut (|(ref n, other)| {
                    no_out!(integer_checked_hamming_distance_signed_limb_alt_1(
                        &n, other
                    ))
                }),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref n, other)| {
                    no_out!(integer_checked_hamming_distance_signed_limb_alt_2(
                        &n, other
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_signed_limb_checked_hamming_distance_integer(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.checked_hamming_distance(&Integer)",
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "i.significant_bits()",
        &mut [(
            "default",
            &mut (|(i, ref other)| no_out!(i.checked_hamming_distance(other))),
        )],
    );
}
