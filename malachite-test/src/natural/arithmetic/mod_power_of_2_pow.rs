use malachite_base::num::arithmetic::traits::{ModPowerOf2Pow, ModPowerOf2PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::mod_power_of_2_pow::{
    limbs_mod_power_of_2_pow, limbs_pow_low,
};
use malachite_nz_test_util::natural::arithmetic::mod_power_of_2_pow::*;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_var_28, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_17,
};
use malachite_test::inputs::natural::triples_of_natural_natural_and_u64_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_pow_low);
    register_demo!(registry, demo_limbs_mod_power_of_2_pow);
    register_demo!(registry, demo_natural_mod_power_of_2_pow_assign);
    register_demo!(registry, demo_natural_mod_power_of_2_pow_assign_ref);
    register_demo!(registry, demo_natural_mod_power_of_2_pow);
    register_demo!(registry, demo_natural_mod_power_of_2_pow_val_ref);
    register_demo!(registry, demo_natural_mod_power_of_2_pow_ref_val);
    register_demo!(registry, demo_natural_mod_power_of_2_pow_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_pow_low);
    register_bench!(registry, Small, benchmark_limbs_mod_power_of_2_pow);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_pow_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_pow_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_pow_evaluation_strategy
    );
}

fn demo_limbs_pow_low(gm: GenerationMode, limit: usize) {
    for (xs, es) in pairs_of_unsigned_vec_var_28(gm.with_scale(32)).take(limit) {
        let xs_old = xs;
        let mut xs = xs_old.clone();
        let mut scratch = vec![0; xs.len()];
        limbs_pow_low(&mut xs, &es, &mut scratch);
        println!(
            "xs := {:?}; limbs_pow_low(&mut xs, {:?}, &mut scratch); xs = {:?}",
            xs_old, es, xs
        );
    }
}

fn demo_limbs_mod_power_of_2_pow(gm: GenerationMode, limit: usize) {
    for (xs, es, pow) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_17(gm.with_scale(32)).take(limit)
    {
        let xs_old = xs;
        let mut xs = xs_old.clone();
        limbs_mod_power_of_2_pow(&mut xs, &es, pow);
        println!(
            "xs := {:?}; limbs_mod_power_of_2_pow(&mut xs, {:?}, {}); xs = {:?}",
            xs_old, es, pow, xs
        );
    }
}

fn demo_natural_mod_power_of_2_pow_assign(gm: GenerationMode, limit: usize) {
    for (mut x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        x.mod_power_of_2_pow_assign(exp, pow);
        println!(
            "x := {}; x.mod_power_of_2_pow_assign({}, {}); x = {}",
            x_old, exp_old, pow, x
        );
    }
}

fn demo_natural_mod_power_of_2_pow_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let x_old = x.clone();
        x.mod_power_of_2_pow_assign(&exp, pow);
        println!(
            "x := {}; x.mod_power_of_2_pow_assign(&{}, {}); x = {}",
            x_old, exp, pow, x
        );
    }
}

fn demo_natural_mod_power_of_2_pow(gm: GenerationMode, limit: usize) {
    for (x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x_old,
            exp_old,
            x.mod_power_of_2_pow(exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_pow_val_ref(gm: GenerationMode, limit: usize) {
    for (x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x_old,
            exp,
            x.mod_power_of_2_pow(&exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_pow_ref_val(gm: GenerationMode, limit: usize) {
    for (x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x,
            exp_old,
            (&x).mod_power_of_2_pow(exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_pow_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x,
            exp,
            (&x).mod_power_of_2_pow(&exp, pow),
            pow
        );
    }
}

fn benchmark_limbs_pow_low(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_pow_low(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_28(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref es)| xs.len() * es.len()),
        "xs.len() * es.len()",
        &mut [(
            "Malachite",
            &mut (|(mut xs, es)| {
                let mut scratch = vec![0; xs.len()];
                limbs_pow_low(&mut xs, &es, &mut scratch)
            }),
        )],
    );
}

fn benchmark_limbs_mod_power_of_2_pow(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_mod_power_of_2_pow(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_17(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref es, _)| xs.len() * es.len()),
        "xs.len() * es.len()",
        &mut [(
            "Malachite",
            &mut (|(mut xs, es, pow)| limbs_mod_power_of_2_pow(&mut xs, &es, pow)),
        )],
    );
}

fn benchmark_natural_mod_power_of_2_pow_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_pow_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref exp, pow)| usize::exact_from(exp.significant_bits()) * usize::exact_from(pow)),
        "exp.significant_bits() * pow",
        &mut [
            (
                "Natural.mod_power_of_2_pow_assign(Natural, u64)",
                &mut (|(mut x, exp, pow)| no_out!(x.mod_power_of_2_pow_assign(exp, pow))),
            ),
            (
                "Natural.mod_power_of_2_pow_assign(&Natural, u64)",
                &mut (|(mut x, exp, pow)| no_out!(x.mod_power_of_2_pow_assign(&exp, pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_pow_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_pow(Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_u64_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref exp, pow)| usize::exact_from(exp.significant_bits()) * usize::exact_from(pow)),
        "exp.significant_bits() * pow",
        &mut [
            (
                "default",
                &mut (|(x, exp, pow)| no_out!(x.mod_power_of_2_pow(exp, pow))),
            ),
            (
                "simple binary",
                &mut (|(x, exp, pow)| no_out!(simple_binary_mod_power_of_2_pow(&x, &exp, pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_pow_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_pow(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref exp, pow)| usize::exact_from(exp.significant_bits()) * usize::exact_from(pow)),
        "exp.significant_bits() * pow",
        &mut [
            (
                "Natural.mod_power_of_2_pow(Natural, u64)",
                &mut (|(x, exp, pow)| no_out!(x.mod_power_of_2_pow(exp, pow))),
            ),
            (
                "Natural.mod_power_of_2_pow(&Natural, u64)",
                &mut (|(x, exp, pow)| no_out!(x.mod_power_of_2_pow(&exp, pow))),
            ),
            (
                "(&Natural).mod_power_of_2_pow(Natural, u64)",
                &mut (|(x, exp, pow)| no_out!((&x).mod_power_of_2_pow(exp, pow))),
            ),
            (
                "(&Natural).mod_power_of_2_pow(&Natural, u64)",
                &mut (|(x, exp, pow)| no_out!((&x).mod_power_of_2_pow(&exp, pow))),
            ),
        ],
    );
}
