use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{pairs_of_integer_and_natural, rm_pairs_of_integer_and_natural};
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use rug::Assign as rug_assign;
use std::cmp::max;

pub fn demo_integer_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_integer_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_integer_assign_natural_library_comparison(
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
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            ("rug", &mut (|((mut x, y), _)| x.assign(y))),
        ],
    );
}

pub fn benchmark_integer_assign_natural_evaluation_strategy(
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
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer.assign(Natural)", &mut (|(mut x, y)| x.assign(y))),
            ("Integer.assign(&Natural)", &mut (|(mut x, y)| x.assign(&y))),
        ],
    );
}
