use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    naturals, nrm_naturals, nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals,
};
use malachite_base::num::Assign;
use malachite_base::num::SignificantBits;
use rug::Assign as rug_assign;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_clone);
    register_demo!(registry, demo_natural_clone_from);
    register_demo!(registry, demo_natural_assign);
    register_demo!(registry, demo_natural_assign_ref);
    register_bench!(registry, Large, benchmark_natural_clone_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_natural_clone_from_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_assign_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_natural_assign_evaluation_strategy
    );
}

fn demo_natural_clone(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

fn demo_natural_clone_from(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

fn demo_natural_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_natural_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

fn benchmark_natural_clone_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.clone()",
        BenchmarkType::LibraryComparison,
        nrm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(n.clone()))),
            ("num", &mut (|(n, _, _)| no_out!(n.clone()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.clone()))),
        ],
    );
}

fn benchmark_natural_clone_from_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.clone_from(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, _, (mut x, y))| x.clone_from(&y))),
            ("num", &mut (|((mut x, y), _, _)| x.clone_from(&y))),
            ("rug", &mut (|(_, (mut x, y), _)| x.clone_from(&y))),
        ],
    );
}

fn benchmark_natural_assign_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.assign(Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals(gm),
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

fn benchmark_natural_assign_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural.assign(Natural)", &mut (|(mut x, y)| x.assign(y))),
            ("Natural.assign(&Natural)", &mut (|(mut x, y)| x.assign(&y))),
        ],
    );
}
