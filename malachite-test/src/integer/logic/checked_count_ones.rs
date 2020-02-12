use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitIterable, SignificantBits};
use malachite_nz::integer::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;

pub fn integer_checked_count_ones_alt_1(n: &Integer) -> Option<u64> {
    if *n >= 0 {
        Some(u64::wrapping_from(n.bits().filter(|&b| b).count()))
    } else {
        None
    }
}

pub fn integer_checked_count_ones_alt_2(n: &Integer) -> Option<u64> {
    if *n >= 0 {
        Some(
            n.twos_complement_limbs()
                .map(|limb| u64::from(limb.count_ones()))
                .sum(),
        )
    } else {
        None
    }
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_checked_count_ones);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_count_ones_algorithms
    );
}

fn demo_integer_checked_count_ones(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("checked_count_ones({}) = {:?}", n, n.checked_count_ones());
    }
}

fn benchmark_integer_checked_count_ones_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_count_ones()",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.checked_count_ones()))),
            (
                "using bits explicitly",
                &mut (|n| no_out!(integer_checked_count_ones_alt_1(&n))),
            ),
            (
                "using limbs explicitly",
                &mut (|n| no_out!(integer_checked_count_ones_alt_2(&n))),
            ),
        ],
    );
}
