use std::cmp::max;

use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_natural_and_integer, rm_pairs_of_integer_and_natural,
    rm_pairs_of_natural_and_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_sub_natural_assign);
    register_demo!(registry, demo_integer_sub_natural_assign_ref);
    register_demo!(registry, demo_integer_sub_natural);
    register_demo!(registry, demo_integer_sub_natural_val_ref);
    register_demo!(registry, demo_integer_sub_natural_ref_val);
    register_demo!(registry, demo_integer_sub_natural_ref_ref);
    register_demo!(registry, demo_natural_sub_integer);
    register_demo!(registry, demo_natural_sub_integer_val_ref);
    register_demo!(registry, demo_natural_sub_integer_ref_val);
    register_demo!(registry, demo_natural_sub_integer_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_natural_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_natural_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_integer_evaluation_strategy
    );
}

fn demo_integer_sub_natural_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        x -= y.clone();
        println!("x := {}; x -= {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_sub_natural_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_sub_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_integer_sub_natural_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_integer_sub_natural_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_integer_sub_natural_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn demo_natural_sub_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_natural_sub_integer_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_natural_sub_integer_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_natural_sub_integer_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn benchmark_integer_sub_natural_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer -= Natural",
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
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

fn benchmark_integer_sub_natural_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer -= Natural",
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
            ("Integer -= Natural", &mut (|(mut x, y)| no_out!(x -= y))),
            ("Integer -= &Natural", &mut (|(mut x, y)| no_out!(x -= &y))),
        ],
    );
}

fn benchmark_integer_sub_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - Natural",
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
            ("malachite", &mut (|(_, (x, y))| no_out!(x - y))),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_integer_sub_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - Natural",
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
            ("Integer - Natural", &mut (|(x, y)| no_out!(x - y))),
            ("Integer - &Natural", &mut (|(x, y)| no_out!(x - &y))),
            ("&Integer - Natural", &mut (|(x, y)| no_out!(&x - y))),
            ("&Integer - &Natural", &mut (|(x, y)| no_out!(&x - &y))),
        ],
    );
}

fn benchmark_natural_sub_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural - Integer",
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
            ("malachite", &mut (|(_, (x, y))| no_out!(x - y))),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_natural_sub_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural - Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural - Integer", &mut (|(x, y)| no_out!(x - y))),
            ("Natural - &Integer", &mut (|(x, y)| no_out!(x - &y))),
            ("&Natural - Integer", &mut (|(x, y)| no_out!(&x - y))),
            ("&Natural - &Integer", &mut (|(x, y)| no_out!(&x - &y))),
        ],
    );
}
