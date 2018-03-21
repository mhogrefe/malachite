use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_natural_and_natural_integer,
                      rm_pairs_of_natural_and_natural_integer};
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use rug::Assign as rug_assign;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_assign_integer);
    register_demo!(registry, demo_natural_assign_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_assign_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_assign_integer_evaluation_strategy
    );
}

fn demo_natural_assign_integer(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_natural_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_natural_assign_integer_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_natural_integer(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

fn benchmark_natural_assign_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.assign(Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_natural_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            ("rug", &mut (|((mut x, y), _)| x.assign(y))),
        ],
    );
}

fn benchmark_natural_assign_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_natural_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural.assign(Integer)", &mut (|(mut x, y)| x.assign(y))),
            ("Natural.assign(&Integer)", &mut (|(mut x, y)| x.assign(&y))),
        ],
    );
}
