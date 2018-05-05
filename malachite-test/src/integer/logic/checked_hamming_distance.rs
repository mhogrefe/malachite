use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_u32_vec_var_1;
use inputs::integer::pairs_of_integers;
use malachite_base::num::{CheckedHammingDistance, HammingDistance, SignificantBits};
use malachite_nz::integer::logic::checked_hamming_distance::limbs_hamming_distance_neg;
use malachite_nz::integer::Integer;
use std::cmp::max;
use std::iter::repeat;
use std::u32;

pub fn integer_checked_hamming_distance_alt_1(x: &Integer, y: &Integer) -> Option<u64> {
    let negative = *x < 0;
    if negative != (*y < 0) {
        return None;
    }
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if x.twos_complement_bits().count() >= y.twos_complement_bits().count() {
            Box::new(
                x.twos_complement_bits()
                    .zip(y.twos_complement_bits().chain(repeat(negative))),
            )
        } else {
            Box::new(
                x.twos_complement_bits()
                    .chain(repeat(negative))
                    .zip(y.twos_complement_bits()),
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

pub fn integer_checked_hamming_distance_alt_2(x: &Integer, y: &Integer) -> Option<u64> {
    let extension = if *x < 0 { u32::MAX } else { 0 };
    if (*x < 0) != (*y < 0) {
        return None;
    }
    let limb_zip: Box<Iterator<Item = (u32, u32)>> =
        if x.twos_complement_limbs().count() >= y.twos_complement_limbs().count() {
            Box::new(
                x.twos_complement_limbs()
                    .zip(y.twos_complement_limbs().chain(repeat(extension))),
            )
        } else {
            Box::new(
                x.twos_complement_limbs()
                    .chain(repeat(extension))
                    .zip(y.twos_complement_limbs()),
            )
        };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    Some(distance)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_neg);
    register_demo!(registry, demo_integer_checked_hamming_distance);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_neg);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_algorithms
    );
}

fn demo_limbs_hamming_distance_neg(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_u32_vec_var_1(gm).take(limit) {
        println!(
            "limbs_hamming_distance_neg({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance_neg(xs, ys)
        );
    }
}

fn demo_integer_checked_hamming_distance(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            x,
            y,
            x.checked_hamming_distance(&y)
        );
    }
}

fn benchmark_limbs_hamming_distance_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance_neg(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_u32_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys)| no_out!(limbs_hamming_distance_neg(xs, ys))),
        )],
    );
}

fn benchmark_integer_checked_hamming_distance_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_hamming_distance(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "default",
                &mut (|(n, other)| no_out!(n.checked_hamming_distance(&other))),
            ),
            (
                "using bits explicitly",
                &mut (|(n, other)| no_out!(integer_checked_hamming_distance_alt_1(&n, &other))),
            ),
            (
                "using limbs explicitly",
                &mut (|(n, other)| no_out!(integer_checked_hamming_distance_alt_2(&n, &other))),
            ),
        ],
    );
}
