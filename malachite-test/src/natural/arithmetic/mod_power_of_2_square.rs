use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Mul, ModPowerOf2Square, ModPowerOf2SquareAssign, ModSquare, PowerOf2,
    Square,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::mod_power_of_2_square::{
    limbs_mod_power_of_2_square, limbs_mod_power_of_2_square_ref, limbs_square_low,
    limbs_square_low_basecase, limbs_square_low_divide_and_conquer, limbs_square_low_scratch_len,
};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::natural::arithmetic::mod_power_of_2_square::*;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_small_unsigned_var_4, pairs_of_unsigned_vec_var_26,
    pairs_of_unsigned_vec_var_27, pairs_of_unsigned_vec_var_4,
};
use malachite_test::inputs::natural::pairs_of_natural_and_u64_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_square_low_basecase);
    register_demo!(registry, demo_limbs_square_low_divide_and_conquer);
    register_demo!(registry, demo_limbs_square_low);
    register_demo!(registry, demo_limbs_mod_power_of_2_square);
    register_demo!(registry, demo_limbs_mod_power_of_2_square_ref);
    register_demo!(registry, demo_natural_mod_power_of_2_square_assign);
    register_demo!(registry, demo_natural_mod_power_of_2_square);
    register_demo!(registry, demo_natural_mod_power_of_2_square_ref);
    register_bench!(registry, Small, benchmark_limbs_square_low_basecase);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_square_low_divide_and_conquer_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_square_low);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_2_square_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_square_assign
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_square_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_square_algorithms
    );
}

fn demo_limbs_square_low_basecase(gm: GenerationMode, limit: usize) {
    for (out, xs) in pairs_of_unsigned_vec_var_26(gm).take(limit) {
        let out_old = out;
        let mut out = out_old.clone();
        limbs_square_low_basecase(&mut out, &xs);
        println!(
            "out := {:?}; limbs_square_low_basecase(&mut out, {:?}); out = {:?}",
            out_old, xs, out
        );
    }
}

fn demo_limbs_square_low_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (out, xs) in pairs_of_unsigned_vec_var_27(gm).take(limit) {
        let out_old = out;
        let mut out = out_old.clone();
        let mut scratch = vec![0; limbs_square_low_scratch_len(xs.len())];
        limbs_square_low_divide_and_conquer(&mut out, &xs, &mut scratch);
        println!(
            "out := {:?}; limbs_square_low_divide_and_conquer(&mut out, {:?}, &mut scratch); \
            out = {:?}",
            out_old, xs, out
        );
    }
}

fn demo_limbs_square_low(gm: GenerationMode, limit: usize) {
    for (out, xs) in pairs_of_unsigned_vec_var_4(gm).take(limit) {
        let out_old = out;
        let mut out = out_old.clone();
        limbs_square_low(&mut out, &xs);
        println!(
            "out := {:?}; limbs_square_low(&mut out, {:?}); out = {:?}",
            out_old, xs, out
        );
    }
}

fn demo_limbs_mod_power_of_2_square(gm: GenerationMode, limit: usize) {
    for (mut xs, pow) in pairs_of_unsigned_vec_and_small_unsigned_var_4(gm).take(limit) {
        let xs_old = xs.clone();
        println!(
            "limbs_mod_power_of_2_square({:?}, {}) = {:?}",
            xs_old,
            pow,
            limbs_mod_power_of_2_square(&mut xs, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_square_ref(gm: GenerationMode, limit: usize) {
    for (xs, pow) in pairs_of_unsigned_vec_and_small_unsigned_var_4(gm).take(limit) {
        println!(
            "limbs_mod_power_of_2_square_ref({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_mod_power_of_2_square_ref(&xs, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_square_assign(gm: GenerationMode, limit: usize) {
    for (mut n, pow) in pairs_of_natural_and_u64_var_1(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_2_square_assign(pow);
        println!(
            "x := {}; x.mod_power_of_2_square_assign({}); x = {}",
            n_old, pow, n
        );
    }
}

fn demo_natural_mod_power_of_2_square(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_u64_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.square() === {} mod 2^{}",
            n_old,
            n.mod_power_of_2_square(pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_square_ref(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_u64_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "(&{}).square() === {} mod 2^{}",
            n_old,
            n.mod_power_of_2_square(pow),
            pow
        );
    }
}

fn benchmark_limbs_square_low_basecase(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_square_low_basecase(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_26(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut out, xs)| limbs_square_low_basecase(&mut out, &xs)),
        )],
    );
}

fn benchmark_limbs_square_low_divide_and_conquer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_square_low_divide_and_conquer(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_27(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs)| limbs_square_low_basecase_unrestricted(&mut out, &xs)),
            ),
            (
                "divide and conquer",
                &mut (|(mut out, xs)| {
                    let mut scratch = vec![0; limbs_square_low_scratch_len(xs.len())];
                    limbs_square_low_divide_and_conquer(&mut out, &xs, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_square_low(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_square_low(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut out, xs)| limbs_square_low(&mut out, &xs)),
        )],
    );
}

fn benchmark_limbs_mod_power_of_2_square_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mod_power_of_2_square(&[Limb], u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_vec_and_small_unsigned_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "limbs_mod_power_of_2_square",
                &mut (|(ref mut xs, pow)| no_out!(limbs_mod_power_of_2_square(xs, pow))),
            ),
            (
                "limbs_mod_power_of_2_square_ref",
                &mut (|(ref mut xs, pow)| no_out!(limbs_mod_power_of_2_square_ref(xs, pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_square_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_square_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Natural.mod_power_of_2_square_assign(u64)",
            &mut (|(mut n, pow)| n.mod_power_of_2_square_assign(pow)),
        )],
    );
}

fn benchmark_natural_mod_power_of_2_square_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_square(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_2_square(u64)",
                &mut (|(n, pow)| no_out!(n.mod_power_of_2_square(pow))),
            ),
            (
                "(&Natural).mod_power_of_2_square(u64)",
                &mut (|(n, pow)| no_out!((&n).mod_power_of_2_square(pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_square_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_square(u64)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_2_square(u64)",
                &mut (|(n, pow)| no_out!(n.mod_power_of_2_square(pow))),
            ),
            (
                "Natural.mod_power_of_2_mul(Natural, u64)",
                &mut (|(n, pow)| no_out!(n.clone().mod_power_of_2_mul(n, pow))),
            ),
            (
                "Natural.square().mod_power_of_2(u64)",
                &mut (|(n, pow)| no_out!(n.square().mod_power_of_2(pow))),
            ),
            (
                "Natural.mod_square(Natural::power_of_2(u64))",
                &mut (|(n, pow)| no_out!(n.mod_square(Natural::power_of_2(pow)))),
            ),
        ],
    );
}
