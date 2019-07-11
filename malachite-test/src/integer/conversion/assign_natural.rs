use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_natural, rm_pairs_of_integer_and_natural};
use malachite_base::num::conversion::traits::{Assign, CheckedFrom};
use malachite_base::num::logic::traits::SignificantBits;
use rug::Assign as rug_assign;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_assign_natural);
    register_demo!(registry, demo_integer_assign_natural_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_assign_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_assign_natural_evaluation_strategy
    );
}

fn demo_integer_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_integer_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

fn benchmark_integer_assign_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(Natural)",
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
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            ("rug", &mut (|((mut x, y), _)| x.assign(y))),
        ],
    );
}

fn benchmark_integer_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer.assign(Natural)", &mut (|(mut x, y)| x.assign(y))),
            ("Integer.assign(&Natural)", &mut (|(mut x, y)| x.assign(&y))),
        ],
    );
}
