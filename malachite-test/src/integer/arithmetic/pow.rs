use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use num::traits::Pow as NumPow;
use rug::ops::Pow as RugPow;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{
    nrm_pairs_of_integer_and_small_unsigned, pairs_of_integer_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_pow_assign);
    register_demo!(registry, demo_integer_pow);
    register_demo!(registry, demo_integer_pow_ref);
    register_bench!(registry, Large, benchmark_integer_pow_assign);
    register_bench!(registry, Large, benchmark_integer_pow_library_comparison);
    register_bench!(registry, Large, benchmark_integer_pow_evaluation_strategy);
}

fn demo_integer_pow_assign(gm: GenerationMode, limit: usize) {
    for (mut n, pow) in pairs_of_integer_and_small_unsigned(gm.with_scale(16)).take(limit) {
        let n_old = n.clone();
        n.pow_assign(pow);
        println!("x := {}; x.pow_assign({}); x = {}", n_old, pow, n);
    }
}

fn demo_integer_pow(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_integer_and_small_unsigned(gm.with_scale(16)).take(limit) {
        let n_old = n.clone();
        println!("{}.pow({}) = {}", n_old, pow, n.pow(pow));
    }
}

fn demo_integer_pow_ref(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_integer_and_small_unsigned(gm.with_scale(16)).take(limit) {
        println!("(&{}).pow({}) = {}", n, pow, (&n).pow(pow));
    }
}

fn benchmark_integer_pow_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.pow_assign(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm.with_scale(16)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, exp)| usize::exact_from(n.significant_bits() * exp)),
        "self.significant_bits() * exp",
        &mut [("Malachite", &mut (|(mut x, exp)| x.pow_assign(exp)))],
    );
}

fn benchmark_integer_pow_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.pow(u64)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_small_unsigned(gm.with_scale(16)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, exp))| usize::exact_from(x.significant_bits() * exp)),
        "self.significant_bits() * exp",
        &mut [
            ("Malachite", &mut (|(_, _, (x, exp))| no_out!(x.pow(exp)))),
            ("num", &mut (|((x, exp), _, _)| no_out!(x.pow(exp)))),
            (
                "rug",
                &mut (|(_, (x, exp), _)| no_out!(x.pow(u32::exact_from(exp)))),
            ),
        ],
    );
}

fn benchmark_integer_pow_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_unsigned(gm.with_scale(16)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, exp)| usize::exact_from(x.significant_bits() * exp)),
        "self.significant_bits() * exp",
        &mut [
            ("Integer.pow(u64)", &mut (|(x, exp)| no_out!(x.pow(exp)))),
            (
                "(&Integer).pow(u64)",
                &mut (|(x, exp)| no_out!((&x).pow(exp))),
            ),
        ],
    );
}
