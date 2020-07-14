use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz_test_util::integer::logic::checked_count_ones::{
    integer_checked_count_ones_alt_1, integer_checked_count_ones_alt_2,
};

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::integers;

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
