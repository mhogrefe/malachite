use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz_test_util::integer::logic::trailing_zeros::integer_trailing_zeros_alt;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_trailing_zeros);
    register_bench!(registry, Large, benchmark_integer_trailing_zeros_algorithms);
}

fn demo_integer_trailing_zeros(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

fn benchmark_integer_trailing_zeros_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.trailing_zeros()",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.trailing_zeros()))),
            (
                "using bits explicitly",
                &mut (|n| no_out!(integer_trailing_zeros_alt(&n))),
            ),
        ],
    );
}
