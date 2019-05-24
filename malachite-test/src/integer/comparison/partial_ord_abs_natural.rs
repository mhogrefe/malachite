use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_natural_and_integer, rm_pairs_of_integer_and_natural,
    rm_pairs_of_natural_and_integer,
};
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{PartialOrdAbs, SignificantBits};
use std::cmp::{max, Ordering};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_partial_cmp_abs_natural);
    register_demo!(registry, demo_natural_partial_cmp_abs_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_abs_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_cmp_abs_integer_library_comparison
    );
}

fn demo_integer_partial_cmp_abs_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        match x.partial_cmp_abs(&y).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", x, y),
            Ordering::Equal => println!("|{}| = |{}|", x, y),
            Ordering::Greater => println!("|{}| > |{}|", x, y),
        }
    }
}

fn demo_natural_partial_cmp_abs_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        match x.partial_cmp_abs(&y).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", x, y),
            Ordering::Equal => println!("|{}| = |{}|", x, y),
            Ordering::Greater => println!("|{}| > |{}|", x, y),
        }
    }
}

fn benchmark_integer_partial_cmp_abs_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.partial_cmp_abs(&Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_natural(gm),
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
                &mut (|(_, (x, y))| no_out!(x.partial_cmp_abs(&y))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x.cmp_abs(&y)))),
        ],
    );
}

fn benchmark_natural_partial_cmp_abs_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.partial_cmp_abs(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_integer(gm),
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
                &mut (|(_, (x, y))| no_out!(x.partial_cmp_abs(&y))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x.cmp_abs(&y)))),
        ],
    );
}
