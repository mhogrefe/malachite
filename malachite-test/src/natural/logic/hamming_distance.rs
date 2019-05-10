use std::cmp::max;
use std::iter::repeat;

use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{HammingDistance, SignificantBits};
use malachite_nz::natural::logic::hamming_distance::{
    limbs_hamming_distance, limbs_hamming_distance_same_length,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_2};
use inputs::natural::{pairs_of_naturals, rm_pairs_of_naturals};

pub fn natural_hamming_distance_alt_1(x: &Natural, y: &Natural) -> u64 {
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if x.significant_bits() >= y.significant_bits() {
            Box::new(x.bits().zip(y.bits().chain(repeat(false))))
        } else {
            Box::new(x.bits().chain(repeat(false)).zip(y.bits()))
        };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    distance
}

pub fn natural_hamming_distance_alt_2(x: &Natural, y: &Natural) -> u64 {
    let limb_zip: Box<Iterator<Item = (Limb, Limb)>> = if x.limb_count() >= y.limb_count() {
        Box::new(x.limbs().zip(y.limbs().chain(repeat(0))))
    } else {
        Box::new(x.limbs().chain(repeat(0)).zip(y.limbs()))
    };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    distance
}

pub fn rug_hamming_distance(x: &rug::Integer, y: &rug::Integer) -> u64 {
    u64::from(x.hamming_dist(y).unwrap())
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_same_length);
    register_demo!(registry, demo_limbs_hamming_distance);
    register_demo!(registry, demo_natural_hamming_distance);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_hamming_distance_same_length
    );
    register_bench!(registry, Small, benchmark_limbs_hamming_distance);
    register_bench!(
        registry,
        Large,
        benchmark_natural_hamming_distance_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_hamming_distance_algorithms
    );
}

fn demo_limbs_hamming_distance_same_length(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        println!(
            "limbs_hamming_distance_same_length({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance_same_length(&xs, &ys),
        );
    }
}

fn demo_limbs_hamming_distance(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_2(gm).take(limit) {
        println!(
            "limbs_hamming_distance({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance(&xs, &ys)
        );
    }
}

fn demo_natural_hamming_distance(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!(
            "hamming_distance({}, {}) = {}",
            x,
            y,
            x.hamming_distance(&y)
        );
    }
}

fn benchmark_limbs_hamming_distance_same_length(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance_same_length(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(xs, ys)| no_out!(limbs_hamming_distance_same_length(&xs, &ys))),
        )],
    );
}

fn benchmark_limbs_hamming_distance(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(xs, ys)| no_out!(limbs_hamming_distance(&xs, &ys))),
        )],
    );
}

fn benchmark_natural_hamming_distance_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.hamming_distance(&Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.hamming_distance(&y))),
            ),
            (
                "rug",
                &mut (|((x, y), _)| no_out!(rug_hamming_distance(&x, &y))),
            ),
        ],
    );
}

fn benchmark_natural_hamming_distance_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.hamming_distance(&Natural)",
        BenchmarkType::Algorithms,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(x, y)| no_out!(x.hamming_distance(&y)))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(natural_hamming_distance_alt_1(&x, &y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(natural_hamming_distance_alt_2(&x, &y))),
            ),
        ],
    );
}
