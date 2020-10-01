use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::pow::limbs_pow;
use malachite_nz_test_util::natural::arithmetic::pow::{
    natural_pow_naive, natural_pow_simple_binary,
};
use num::traits::Pow as NumPow;
use rug::ops::Pow as RugPow;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_unsigned_var_3;
use malachite_test::inputs::natural::{
    nrm_pairs_of_natural_and_small_unsigned, pairs_of_natural_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_pow);
    register_demo!(registry, demo_natural_pow_assign);
    register_demo!(registry, demo_natural_pow);
    register_demo!(registry, demo_natural_pow_ref);
    register_bench!(registry, Small, benchmark_limbs_pow);
    register_bench!(registry, Large, benchmark_natural_pow_assign);
    register_bench!(registry, Large, benchmark_natural_pow_algorithms);
    register_bench!(registry, Large, benchmark_natural_pow_library_comparison);
    register_bench!(registry, Large, benchmark_natural_pow_evaluation_strategy);
}

fn demo_limbs_pow(gm: GenerationMode, limit: usize) {
    for (ref xs, exp) in pairs_of_unsigned_vec_and_small_unsigned_var_3(gm).take(limit) {
        println!("limbs_pow({:?}, {}) = {:?}", xs, exp, limbs_pow(xs, exp));
    }
}

fn demo_natural_pow_assign(gm: GenerationMode, limit: usize) {
    for (mut n, pow) in pairs_of_natural_and_small_unsigned(gm.with_scale(16)).take(limit) {
        let n_old = n.clone();
        n.pow_assign(pow);
        println!("x := {}; x.pow_assign({}); x = {}", n_old, pow, n);
    }
}

fn demo_natural_pow(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_small_unsigned(gm.with_scale(16)).take(limit) {
        let n_old = n.clone();
        println!("{}.pow({}) = {}", n_old, pow, n.pow(pow));
    }
}

fn demo_natural_pow_ref(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_small_unsigned(gm.with_scale(16)).take(limit) {
        println!("(&{}).pow({}) = {}", n, pow, (&n).pow(pow));
    }
}

fn benchmark_limbs_pow(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_pow(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, exp)| xs.len() * usize::exact_from(exp)),
        "xs.len() * exp",
        &mut [(
            "Malachite",
            &mut (|(ref xs, exp)| no_out!(limbs_pow(xs, exp))),
        )],
    );
}

fn benchmark_natural_pow_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.pow_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm.with_scale(16)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, exp)| usize::exact_from(n.significant_bits() * exp)),
        "self.significant_bits() * exp",
        &mut [("Malachite", &mut (|(mut x, exp)| x.pow_assign(exp)))],
    );
}

fn benchmark_natural_pow_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.pow(u64)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned(gm.with_scale(16)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, exp)| usize::exact_from(n.significant_bits() * exp)),
        "self.significant_bits() * exp",
        &mut [
            ("default", &mut (|(x, exp)| no_out!((&x).pow(exp)))),
            (
                "naive",
                &mut (|(x, exp)| no_out!(natural_pow_naive(&x, exp))),
            ),
            (
                "simple binary",
                &mut (|(x, exp)| no_out!(natural_pow_simple_binary(&x, exp))),
            ),
            ("alt", &mut (|(x, exp)| no_out!(x.pow_ref_alt(exp)))),
        ],
    );
}

fn benchmark_natural_pow_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.pow(u64)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_small_unsigned(gm.with_scale(16)),
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

fn benchmark_natural_pow_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_unsigned(gm.with_scale(16)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, exp)| usize::exact_from(x.significant_bits() * exp)),
        "self.significant_bits() * exp",
        &mut [
            ("Natural.pow(u64)", &mut (|(x, exp)| no_out!(x.pow(exp)))),
            (
                "(&Natural).pow(u64)",
                &mut (|(x, exp)| no_out!((&x).pow(exp))),
            ),
        ],
    );
}
