use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{
    nrm_pairs_of_integers, pairs_of_integers, rm_pairs_of_integers,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_sub_assign);
    register_demo!(registry, demo_integer_sub_assign_ref);
    register_demo!(registry, demo_integer_sub);
    register_demo!(registry, demo_integer_sub_val_ref);
    register_demo!(registry, demo_integer_sub_ref_val);
    register_demo!(registry, demo_integer_sub_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_sub_library_comparison);
    register_bench!(registry, Large, benchmark_integer_sub_evaluation_strategy);
}

fn demo_integer_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x -= y.clone();
        println!("x := {}; x -= {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_sub_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_integer_sub_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_integer_sub_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_integer_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn benchmark_integer_sub_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer -= Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

fn benchmark_integer_sub_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer -= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer -= Integer", &mut (|(mut x, y)| no_out!(x -= y))),
            ("Integer -= &Integer", &mut (|(mut x, y)| no_out!(x -= &y))),
        ],
    );
}

fn benchmark_integer_sub_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer - Integer",
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
            ("Malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x - y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_integer_sub_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer - Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer - Integer", &mut (|(x, y)| no_out!(x - y))),
            ("Integer - &Integer", &mut (|(x, y)| no_out!(x - &y))),
            ("&Integer - Integer", &mut (|(x, y)| no_out!(&x - y))),
            ("&Integer - &Integer", &mut (|(x, y)| no_out!(&x - &y))),
        ],
    );
}
