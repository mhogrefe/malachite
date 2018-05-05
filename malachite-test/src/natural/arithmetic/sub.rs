use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    nrm_pairs_of_naturals, pairs_of_naturals, pairs_of_naturals_var_1, rm_pairs_of_naturals_var_1,
};
use malachite_base::num::SignificantBits;
use std::cmp::max;
use std::ops::Sub;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_sub_assign);
    register_demo!(registry, demo_natural_sub);
    register_demo!(registry, demo_natural_sub_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_assign_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_sub_library_comparison);
    register_bench!(registry, Large, benchmark_natural_sub_evaluation_strategy);
}

pub fn checked_sub<T: Ord + Sub>(x: T, y: T) -> Option<<T as Sub>::Output> {
    if x >= y {
        Some(x - y)
    } else {
        None
    }
}

fn demo_natural_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {:?}", x_old, y, x - &y);
    }
}

fn demo_natural_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} - &{} = {:?}", x, y, &x - &y);
    }
}

fn benchmark_natural_sub_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural -= &Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x -= &y)),
            ("rug", &mut (|((mut x, y), _)| x -= &y)),
        ],
    );
}

fn benchmark_natural_sub_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural - Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - &y))),
            ("num", &mut (|((x, y), _, _)| no_out!(checked_sub(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(checked_sub(x, y)))),
        ],
    );
}

fn benchmark_natural_sub_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural - Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural - &Natural", &mut (|(x, y)| no_out!(x - &y))),
            ("&Natural - &Natural", &mut (|(x, y)| no_out!(&x - &y))),
        ],
    );
}
