use malachite_base::num::arithmetic::traits::{
    ModMul, ModPowerOf2, ModPowerOf2Mul, ModPowerOf2MulAssign, PowerOf2,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::mod_power_of_2_mul::{
    limbs_mod_power_of_2_mul, limbs_mod_power_of_2_mul_ref_ref, limbs_mod_power_of_2_mul_val_ref,
};
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::triples_of_limb_vec_limb_vec_and_u64_var_16;
use malachite_test::inputs::natural::triples_of_natural_natural_and_u64_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_power_of_2_mul);
    register_demo!(registry, demo_limbs_mod_power_of_2_mul_val_ref);
    register_demo!(registry, demo_limbs_mod_power_of_2_mul_ref_ref);
    register_demo!(registry, demo_natural_mod_power_of_2_mul_assign);
    register_demo!(registry, demo_natural_mod_power_of_2_mul_assign_ref);
    register_demo!(registry, demo_natural_mod_power_of_2_mul);
    register_demo!(registry, demo_natural_mod_power_of_2_mul_val_ref);
    register_demo!(registry, demo_natural_mod_power_of_2_mul_ref_val);
    register_demo!(registry, demo_natural_mod_power_of_2_mul_ref_ref);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_2_mul_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_mul_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_mul_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_mul_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_2_mul(gm: GenerationMode, limit: usize) {
    for (mut xs, mut ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_16(gm).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        println!(
            "limbs_mod_power_of_2_mul({:?}, {:?}, {}) = {:?}",
            xs_old,
            ys_old,
            pow,
            limbs_mod_power_of_2_mul(&mut xs, &mut ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_mul_val_ref(gm: GenerationMode, limit: usize) {
    for (mut xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_16(gm).take(limit) {
        let xs_old = xs.clone();
        println!(
            "limbs_mod_power_of_2_mul({:?}, {:?}, {}) = {:?}",
            xs_old,
            ys,
            pow,
            limbs_mod_power_of_2_mul_val_ref(&mut xs, &ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_mul_ref_ref(gm: GenerationMode, limit: usize) {
    for (xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_16(gm).take(limit) {
        println!(
            "limbs_mod_power_of_2_mul_ref_ref({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_mod_power_of_2_mul_ref_ref(&xs, &ys, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_power_of_2_mul_assign(y, pow);
        println!(
            "x := {}; x.mod_power_of_2_mul_assign({}, {}); x = {}",
            x_old, y_old, pow, x
        );
    }
}

fn demo_natural_mod_power_of_2_mul_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        x.mod_power_of_2_mul_assign(&y, pow);
        println!(
            "x := {}; x.mod_power_of_2_mul_assign(&{}, {}); x = {}",
            x_old, y, pow, x
        );
    }
}

fn demo_natural_mod_power_of_2_mul(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{} * {} === {} mod 2^{}",
            x_old,
            y_old,
            x.mod_power_of_2_mul(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_mul_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{} * {} === {} mod 2^{}",
            x_old,
            y,
            x.mod_power_of_2_mul(&y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_mul_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "{} * {} === {} mod 2^{}",
            x,
            y_old,
            (&x).mod_power_of_2_mul(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_mul_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        println!(
            "{} * {} === {} mod 2^{}",
            x,
            y,
            (&x).mod_power_of_2_mul(&y, pow),
            pow
        );
    }
}

fn benchmark_limbs_mod_power_of_2_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mod_power_of_2_mul(&[Limb], &[Limb], u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_limb_vec_limb_vec_and_u64_var_16(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "limbs_mod_power_of_2_mul",
                &mut (|(ref mut xs, ref mut ys, pow)| {
                    no_out!(limbs_mod_power_of_2_mul(xs, ys, pow))
                }),
            ),
            (
                "limbs_mod_power_of_2_mul_val_ref",
                &mut (|(ref mut xs, ref ys, pow)| {
                    no_out!(limbs_mod_power_of_2_mul_val_ref(xs, ys, pow))
                }),
            ),
            (
                "limbs_mod_power_of_2_mul_ref_ref",
                &mut (|(ref xs, ref ys, pow)| {
                    no_out!(limbs_mod_power_of_2_mul_ref_ref(xs, ys, pow))
                }),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_mul_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_2_mul_assign(Natural, u64)",
                &mut (|(mut x, y, pow)| no_out!(x.mod_power_of_2_mul_assign(y, pow))),
            ),
            (
                "Natural.mod_power_of_2_mul_assign(&Natural, u64)",
                &mut (|(mut x, y, pow)| no_out!(x.mod_power_of_2_mul_assign(&y, pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_mul_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_mul(Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "default",
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_2_mul(y, pow))),
            ),
            (
                "naive",
                &mut (|(x, y, pow)| no_out!((x * y).mod_power_of_2(pow))),
            ),
            (
                "using mod_mul",
                &mut (|(x, y, pow)| no_out!(x.mod_mul(y, Natural::power_of_2(pow)))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_mul(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_2_mul(Natural, u64)",
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_2_mul(y, pow))),
            ),
            (
                "Natural.mod_power_of_2_mul(&Natural, u64)",
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_2_mul(&y, pow))),
            ),
            (
                "(&Natural).mod_power_of_2_mul(Natural, u64)",
                &mut (|(x, y, pow)| no_out!((&x).mod_power_of_2_mul(y, pow))),
            ),
            (
                "(&Natural).mod_power_of_2_mul(&Natural, u64)",
                &mut (|(x, y, pow)| no_out!((&x).mod_power_of_2_mul(&y, pow))),
            ),
        ],
    );
}
