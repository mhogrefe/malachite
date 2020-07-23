use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable, SignificantBits};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::num::logic::bit_convertible::{to_bits_asc_alt, to_bits_desc_alt};
use malachite_nz_test_util::natural::logic::to_bits::{to_bits_asc_naive, to_bits_desc_naive};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_to_bits_asc);
    register_demo!(registry, demo_natural_to_bits_desc);
    register_bench!(
        registry,
        Large,
        benchmark_natural_to_bits_asc_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_to_bits_asc_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_to_bits_desc_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_to_bits_desc_algorithms);
}

fn demo_natural_to_bits_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_bits_asc({}) = {:?}", n, n.to_bits_asc());
    }
}

fn demo_natural_to_bits_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_bits_desc({}) = {:?}", n, n.to_bits_desc());
    }
}

fn benchmark_natural_to_bits_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.to_bits_asc()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Natural.to_bits_asc()", &mut (|n| no_out!(n.to_bits_asc()))),
            (
                "Natural.bits().collect::<Vec<bool>>()",
                &mut (|n| no_out!(n.bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

fn benchmark_natural_to_bits_asc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.to_bits_asc()",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.to_bits_asc()))),
            ("alt", &mut (|n| no_out!(to_bits_asc_alt(&n)))),
            ("naive", &mut (|n| no_out!(to_bits_asc_naive(&n)))),
        ],
    );
}

fn benchmark_natural_to_bits_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.to_bits_desc()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.to_bits_desc()",
                &mut (|n| no_out!(n.to_bits_desc())),
            ),
            (
                "Natural.bits().rev().collect::<Vec<bool>>()",
                &mut (|n| no_out!(n.bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

fn benchmark_natural_to_bits_desc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.to_bits_desc()",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.to_bits_desc()))),
            ("alt", &mut (|n| no_out!(to_bits_desc_alt(&n)))),
            ("naive", &mut (|n| no_out!(to_bits_desc_naive(&n)))),
        ],
    );
}
