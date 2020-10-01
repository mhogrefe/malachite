use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{
    integers, nrm_integers, nrm_pairs_of_integers, pairs_of_integers,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_clone);
    register_demo!(registry, demo_integer_clone_from);
    register_bench!(registry, Large, benchmark_integer_clone_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_integer_clone_from_library_comparison
    );
}

fn demo_integer_clone(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

fn demo_integer_clone_from(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

fn benchmark_integer_clone_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.clone()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, _, n)| no_out!(n.clone()))),
            ("num", &mut (|(n, _, _)| no_out!(n.clone()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.clone()))),
        ],
    );
}

fn benchmark_integer_clone_from_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.clone_from(Integer)",
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
            ("Malachite", &mut (|(_, _, (mut x, y))| x.clone_from(&y))),
            ("num", &mut (|((mut x, y), _, _)| x.clone_from(&y))),
            ("rug", &mut (|(_, (mut x, y), _)| x.clone_from(&y))),
        ],
    );
}
