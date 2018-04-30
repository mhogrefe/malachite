use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};
use malachite_base::num::{CheckedHammingDistance, HammingDistance, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::iter::repeat;

pub fn integer_checked_hamming_distance_u32_alt_1(n: &Integer, u: u32) -> Option<u64> {
    if *n < 0 {
        return None;
    }
    let u = Natural::from(u);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.significant_bits() >= u.significant_bits() {
            Box::new(n.twos_complement_bits().zip(u.bits().chain(repeat(false))))
        } else {
            Box::new(n.twos_complement_bits().chain(repeat(false)).zip(u.bits()))
        };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    Some(distance)
}

pub fn integer_checked_hamming_distance_u32_alt_2(n: &Integer, u: u32) -> Option<u64> {
    if *n < 0 {
        return None;
    }
    let u = Natural::from(u);
    let limb_zip: Box<Iterator<Item = (u32, u32)>> =
        if n.twos_complement_limbs().count() as u64 >= u.limb_count() {
            Box::new(n.twos_complement_limbs().zip(u.limbs().chain(repeat(0))))
        } else {
            Box::new(n.twos_complement_limbs().chain(repeat(0)).zip(u.limbs()))
        };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    Some(distance)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_checked_hamming_distance_u32);
    register_demo!(registry, demo_u32_checked_hamming_distance_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_u32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_checked_hamming_distance_integer
    );
}

fn demo_integer_checked_hamming_distance_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            n,
            u,
            n.checked_hamming_distance(u)
        );
    }
}

fn demo_u32_checked_hamming_distance_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            u,
            n,
            u.checked_hamming_distance(&n)
        );
    }
}

fn benchmark_integer_checked_hamming_distance_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_hamming_distance(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(ref n, other)| no_out!(n.checked_hamming_distance(other))),
            ),
            (
                "using bits explicitly",
                &mut (|(ref n, other)| {
                    no_out!(integer_checked_hamming_distance_u32_alt_1(&n, other))
                }),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref n, other)| {
                    no_out!(integer_checked_hamming_distance_u32_alt_2(&n, other))
                }),
            ),
        ],
    );
}

fn benchmark_u32_checked_hamming_distance_integer(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.checked_hamming_distance(&Integer)",
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "default",
            &mut (|(u, ref other)| no_out!(u.checked_hamming_distance(other))),
        )],
    );
}
