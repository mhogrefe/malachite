use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_nonempty_unsigned_vec_and_unsigned;
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};
use malachite_base::num::{HammingDistance, SignificantBits};
use malachite_nz::natural::logic::hamming_distance_u32::limbs_hamming_distance_limb;
use malachite_nz::natural::Natural;
use std::iter::repeat;

pub fn natural_hamming_distance_u32_alt(n: &Natural, u: u32) -> u64 {
    let u = Natural::from(u);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.significant_bits() >= u.significant_bits() {
            Box::new(n.bits().zip(u.bits().chain(repeat(false))))
        } else {
            Box::new(n.bits().chain(repeat(false)).zip(u.bits()))
        };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    distance
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_limb);
    register_demo!(registry, demo_natural_hamming_distance_u32);
    register_demo!(registry, demo_u32_hamming_distance_natural);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_limb);
    register_bench!(
        registry,
        Large,
        benchmark_natural_hamming_distance_u32_algorithms
    );
    register_bench!(registry, Large, benchmark_u32_hamming_distance_natural);
}

fn demo_limbs_hamming_distance_limb(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_hamming_distance_limb({:?}, {}) = {}",
            limbs,
            limb,
            limbs_hamming_distance_limb(limbs, limb)
        );
    }
}

fn demo_natural_hamming_distance_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        println!("hamming_distance({}, {}) = {}", n, u, n.hamming_distance(u));
    }
}

fn demo_u32_hamming_distance_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        println!("hamming_distance({}, {}) = {}", u, n, u.hamming_distance(&n));
    }
}

fn benchmark_limbs_hamming_distance_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance_limb(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|(ref limbs, limb)| no_out!(limbs_hamming_distance_limb(limbs, limb))),
            ),
        ],
    );
}

fn benchmark_natural_hamming_distance_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.hamming_distance(u32)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(ref n, other)| no_out!(n.hamming_distance(other))),
            ),
            (
                "using bits explicitly",
                &mut (|(ref n, other)| no_out!(natural_hamming_distance_u32_alt(&n, other))),
            ),
        ],
    );
}

fn benchmark_u32_hamming_distance_natural(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.hamming_distance(&Natural)",
        BenchmarkType::Single,
        pairs_of_unsigned_and_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(u, ref other)| no_out!(u.hamming_distance(other))),
            ),
        ],
    );
}
