use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{integers, nrm_integers, nrm_pairs_of_integers, pairs_of_integers,
                      rm_pairs_of_integers};
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use rug::Assign as rug_assign;
use std::cmp::max;

pub fn demo_integer_clone(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

pub fn demo_integer_clone_from(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

pub fn demo_integer_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_integer_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_integer_clone_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.clone()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, _, n)| no_out!(n.clone()))),
            ("num", &mut (|(n, _, _)| no_out!(n.clone()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.clone()))),
        ],
    );
}

pub fn benchmark_integer_clone_from_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.clone_from(Integer)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("malachite", &mut (|(_, _, (mut x, y))| x.clone_from(&y))),
            ("num", &mut (|((mut x, y), _, _)| x.clone_from(&y))),
            ("rug", &mut (|(_, (mut x, y), _)| x.clone_from(&y))),
        ],
    );
}

pub fn benchmark_integer_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.clone_from(Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            ("rug", &mut (|((mut x, y), _)| x.assign(y))),
        ],
    );
}

pub fn benchmark_integer_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("Integer.assign(Integer)", &mut (|(mut x, y)| x.assign(y))),
            ("Integer.assign(&Integer)", &mut (|(mut x, y)| x.assign(&y))),
        ],
    );
}
