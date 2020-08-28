use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::natural::arithmetic::pow::{
    _limb_pow_alt_estimated_scratch_len, _limbs_pow_alt_estimated_scratch_len, limb_pow_alt,
    limb_pow_to_out_alt, limbs_pow_alt, limbs_pow_to_out_alt,
};
use malachite_nz_test_util::natural::arithmetic::pow::{
    natural_pow_naive, natural_pow_simple_binary,
};
use num::traits::Pow as NumPow;
use rug::ops::Pow as RugPow;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned_var_2, pairs_of_unsigned_vec_and_small_unsigned_var_3,
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_2,
};
use malachite_test::inputs::natural::{
    nrm_pairs_of_natural_and_small_unsigned, pairs_of_natural_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limb_pow_to_out);
    register_demo!(registry, demo_limb_pow);
    register_demo!(registry, demo_limbs_pow_to_out);
    register_demo!(registry, demo_limbs_pow);
    register_demo!(registry, demo_natural_pow_assign);
    register_demo!(registry, demo_natural_pow);
    register_demo!(registry, demo_natural_pow_ref);
    register_bench!(registry, Small, benchmark_limb_pow_to_out);
    register_bench!(registry, Small, benchmark_limb_pow);
    register_bench!(registry, Small, benchmark_limbs_pow_to_out);
    register_bench!(registry, Small, benchmark_limbs_pow);
    register_bench!(registry, Large, benchmark_natural_pow_assign);
    register_bench!(registry, Large, benchmark_natural_pow_algorithms);
    register_bench!(registry, Large, benchmark_natural_pow_library_comparison);
    register_bench!(registry, Large, benchmark_natural_pow_evaluation_strategy);
}

fn demo_limb_pow_to_out(gm: GenerationMode, limit: usize) {
    for (mut out, x, exp) in
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_3(gm).take(limit)
    {
        let out_old = out.clone();
        let mut scratch = vec![0; _limb_pow_alt_estimated_scratch_len(x, exp)];
        let out_len = limb_pow_to_out_alt(&mut out, x, exp, &mut scratch);
        println!(
            "out := {:?}; limb_pow_to_out(&mut out, {}, {}, &mut scratch) = {}; out = {:?}",
            out_old, x, exp, out_len, out
        );
    }
}

fn demo_limb_pow(gm: GenerationMode, limit: usize) {
    for (x, exp) in pairs_of_unsigned_and_small_unsigned_var_2(gm).take(limit) {
        println!("limb_pow({}, {}) = {:?}", x, exp, limb_pow_alt(x, exp));
    }
}

fn demo_limbs_pow_to_out(gm: GenerationMode, limit: usize) {
    for (mut out, xs, exp) in
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_2(gm).take(limit)
    {
        let out_old = out.clone();
        let mut scratch = vec![0; _limbs_pow_alt_estimated_scratch_len(&xs, exp)];
        let out_len = limbs_pow_to_out_alt(&mut out, &xs, exp, &mut scratch);
        println!(
            "out := {:?}; limb_pow_to_out(&mut out, {:?}, {}, &mut scratch) = {}; out = {:?}",
            out_old, xs, exp, out_len, out
        );
    }
}

fn demo_limbs_pow(gm: GenerationMode, limit: usize) {
    for (ref xs, exp) in pairs_of_unsigned_vec_and_small_unsigned_var_3(gm).take(limit) {
        println!(
            "limbs_pow({:?}, {}) = {:?}",
            xs,
            exp,
            limbs_pow_alt(xs, exp)
        );
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

fn benchmark_limb_pow_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limb_pow_to_out(&mut [Limb], Limb, u64, &mut [Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, x, exp)| usize::exact_from(x.significant_bits() * exp)),
        "x.significant_bits() * exp",
        &mut [(
            "malachite",
            &mut (|(mut out, x, exp)| {
                let mut scratch = vec![0; _limb_pow_alt_estimated_scratch_len(x, exp)];
                limb_pow_to_out_alt(&mut out, x, exp, &mut scratch);
            }),
        )],
    );
}

fn benchmark_limb_pow(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limb_pow(Limb, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, exp)| usize::exact_from(x.significant_bits() * exp)),
        "x.significant_bits() * exp",
        &mut [("malachite", &mut (|(x, exp)| no_out!(limb_pow_alt(x, exp))))],
    );
}

fn benchmark_limbs_pow_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_pow_to_out(&mut [Limb], &[Limb], u64, &mut [Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, exp)| xs.len() * usize::exact_from(exp)),
        "xs.len() * exp",
        &mut [(
            "malachite",
            &mut (|(mut out, xs, exp)| {
                let mut scratch = vec![0; _limbs_pow_alt_estimated_scratch_len(&xs, exp)];
                limbs_pow_to_out_alt(&mut out, &xs, exp, &mut scratch);
            }),
        )],
    );
}

fn benchmark_limbs_pow(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_pow(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, exp)| xs.len() * usize::exact_from(exp)),
        "xs.len() * exp",
        &mut [(
            "malachite",
            &mut (|(ref xs, exp)| no_out!(limbs_pow_alt(xs, exp))),
        )],
    );
}

fn benchmark_natural_pow_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.pow_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm.with_scale(16)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, exp)| usize::exact_from(n.significant_bits() * exp)),
        "self.significant_bits() * exp",
        &mut [("malachite", &mut (|(mut x, exp)| x.pow_assign(exp)))],
    );
}

fn benchmark_natural_pow_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
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
        ],
    );
}

fn benchmark_natural_pow_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.pow(u64)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_small_unsigned(gm.with_scale(16)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, exp))| usize::exact_from(x.significant_bits() * exp)),
        "self.significant_bits() * exp",
        &mut [
            ("malachite", &mut (|(_, _, (x, exp))| no_out!(x.pow(exp)))),
            ("num", &mut (|((x, exp), _, _)| no_out!(x.pow(exp)))),
            (
                "rug",
                &mut (|(_, (x, exp), _)| no_out!(x.pow(u32::exact_from(exp)))),
            ),
        ],
    );
}

fn benchmark_natural_pow_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
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
