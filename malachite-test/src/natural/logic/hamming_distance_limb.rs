use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_nonempty_unsigned_vec_and_unsigned;
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::{HammingDistance, SignificantBits};
use malachite_nz::natural::logic::hamming_distance_limb::limbs_hamming_distance_limb;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::iter::repeat;

pub fn natural_hamming_distance_limb_alt_1(n: &Natural, u: Limb) -> u64 {
    let u = Natural::from(u);
    let bit_zip: Box<dyn Iterator<Item = (bool, bool)>> =
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

pub fn natural_hamming_distance_limb_alt_2(n: &Natural, u: Limb) -> u64 {
    let u = Natural::from(u);
    let limb_zip: Box<dyn Iterator<Item = (Limb, Limb)>> = if n.limb_count() >= u.limb_count() {
        Box::new(n.limbs().zip(u.limbs().chain(repeat(0))))
    } else {
        Box::new(n.limbs().chain(repeat(0)).zip(u.limbs()))
    };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    distance
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_limb);
    register_demo!(registry, demo_natural_hamming_distance_limb);
    register_demo!(registry, demo_limb_hamming_distance_natural);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_limb);
    register_bench!(
        registry,
        Large,
        benchmark_natural_hamming_distance_limb_algorithms
    );
    register_bench!(registry, Large, benchmark_limb_hamming_distance_natural);
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

fn demo_natural_hamming_distance_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        println!("hamming_distance({}, {}) = {}", n, u, n.hamming_distance(u));
    }
}

fn demo_limb_hamming_distance_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        println!(
            "hamming_distance({}, {}) = {}",
            u,
            n,
            u.hamming_distance(&n)
        );
    }
}

fn benchmark_limbs_hamming_distance_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, limb)| no_out!(limbs_hamming_distance_limb(limbs, limb))),
        )],
    );
}

fn benchmark_natural_hamming_distance_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.hamming_distance(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(ref n, other)| no_out!(n.hamming_distance(other))),
            ),
            (
                "using bits explicitly",
                &mut (|(ref n, other)| no_out!(natural_hamming_distance_limb_alt_1(&n, other))),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref n, other)| no_out!(natural_hamming_distance_limb_alt_2(&n, other))),
            ),
        ],
    );
}

fn benchmark_limb_hamming_distance_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Limb.hamming_distance(&Natural)",
        BenchmarkType::Single,
        pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "default",
            &mut (|(u, ref other)| no_out!(u.hamming_distance(other))),
        )],
    );
}
