use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_natural_and_integer, rm_pairs_of_integer_and_natural,
    rm_pairs_of_natural_and_integer,
};
use malachite_base::num::SignificantBits;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_partial_eq_natural);
    register_demo!(registry, demo_natural_partial_eq_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_eq_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_integer_library_comparison
    );
}

fn demo_integer_partial_eq_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

fn demo_natural_partial_eq_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

fn benchmark_integer_partial_eq_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer == Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x == y))),
            ("rug", &mut (|((x, y), _)| no_out!(x == y))),
        ],
    );
}

fn benchmark_natural_partial_eq_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural == Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x == y))),
            ("rug", &mut (|((x, y), _)| no_out!(x == y))),
        ],
    );
}
