use std::cmp::max;

use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::{CheckedHammingDistance, SignificantBits};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_natural, pairs_of_natural_and_integer};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_checked_hamming_distance_natural);
    register_demo!(registry, demo_natural_checked_hamming_distance_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_natural
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_hamming_distance_integer
    );
}

fn demo_integer_checked_hamming_distance_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            x,
            y,
            x.checked_hamming_distance(&y)
        );
    }
}

fn demo_natural_checked_hamming_distance_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            x,
            y,
            x.checked_hamming_distance(&y)
        );
    }
}

fn benchmark_integer_checked_hamming_distance_natural(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_hamming_distance(Natural)",
        BenchmarkType::Single,
        pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(n, other)| no_out!(n.checked_hamming_distance(&other))),
        )],
    );
}

fn benchmark_natural_checked_hamming_distance_integer(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.checked_hamming_distance(&Integer)",
        BenchmarkType::Single,
        pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(n, other)| no_out!(n.checked_hamming_distance(&other))),
        )],
    );
}
