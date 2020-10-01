use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{integers, nrm_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_significant_bits);
    register_bench!(registry, Large, benchmark_integer_significant_bits);
}

fn demo_integer_significant_bits(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

fn benchmark_integer_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.significant_bits()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Malachite",
                &mut (|(_, _, n)| no_out!(n.significant_bits())),
            ),
            ("num", &mut (|(n, _, _)| no_out!(n.bits()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.significant_bits()))),
        ],
    );
}
