use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{nrm_pairs_of_integers, pairs_of_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_eq);
    register_bench!(registry, Large, benchmark_integer_eq_library_comparison);
}

fn demo_integer_eq(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

fn benchmark_integer_eq_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer == Integer",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Malachite", &mut (|(_, _, (x, y))| no_out!(x == y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x == y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x == y))),
        ],
    );
}
