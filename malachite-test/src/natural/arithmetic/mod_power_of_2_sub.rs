use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Sub, ModPowerOf2SubAssign, ModSub, PowerOf2,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::mod_power_of_2_sub::{
    limbs_mod_power_of_2_limb_sub_limbs, limbs_mod_power_of_2_limb_sub_limbs_in_place,
    limbs_mod_power_of_2_sub, limbs_mod_power_of_2_sub_in_place_either,
    limbs_mod_power_of_2_sub_in_place_left, limbs_mod_power_of_2_sub_in_place_right,
};
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_limb_limb_vec_and_u64_var_1, triples_of_limb_vec_limb_vec_and_u64_var_13,
    triples_of_limb_vec_limb_vec_and_u64_var_15,
};
use malachite_test::inputs::natural::triples_of_natural_natural_and_u64_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_power_of_2_limb_sub_limbs);
    register_demo!(registry, demo_limbs_mod_power_of_2_limb_sub_limbs_in_place);
    register_demo!(registry, demo_limbs_mod_power_of_2_sub);
    register_demo!(registry, demo_limbs_mod_power_of_2_sub_in_place_left);
    register_demo!(registry, demo_limbs_mod_power_of_2_sub_in_place_right);
    register_demo!(registry, demo_limbs_mod_power_of_2_sub_in_place_either);
    register_demo!(registry, demo_natural_mod_power_of_2_sub_assign);
    register_demo!(registry, demo_natural_mod_power_of_2_sub_assign_ref);
    register_demo!(registry, demo_natural_mod_power_of_2_sub);
    register_demo!(registry, demo_natural_mod_power_of_2_sub_val_ref);
    register_demo!(registry, demo_natural_mod_power_of_2_sub_ref_val);
    register_demo!(registry, demo_natural_mod_power_of_2_sub_ref_ref);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_2_limb_sub_limbs
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_2_limb_sub_limbs_in_place
    );
    register_bench!(registry, Small, benchmark_limbs_mod_power_of_2_sub);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_2_sub_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_2_sub_in_place_right
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_2_sub_in_place_either
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_sub_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_sub_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_sub_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_2_limb_sub_limbs(gm: GenerationMode, limit: usize) {
    for (x, ys, pow) in triples_of_limb_limb_vec_and_u64_var_1(gm).take(limit) {
        println!(
            "limbs_mod_power_of_2_limb_sub_limbs({}, {:?}, {}) = {:?}",
            x,
            ys,
            pow,
            limbs_mod_power_of_2_limb_sub_limbs(x, &ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_limb_sub_limbs_in_place(gm: GenerationMode, limit: usize) {
    for (x, mut ys, pow) in triples_of_limb_limb_vec_and_u64_var_1(gm).take(limit) {
        let ys_old = ys.clone();
        limbs_mod_power_of_2_limb_sub_limbs_in_place(x, &mut ys, pow);
        println!(
            "ys := {:?}; limbs_mod_power_of_2_limb_sub_limbs_in_place({}, &mut ys, {}); \
            ys = {:?}",
            ys_old, x, pow, ys
        );
    }
}

fn demo_limbs_mod_power_of_2_sub(gm: GenerationMode, limit: usize) {
    for (xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_13(gm).take(limit) {
        println!(
            "limbs_mod_power_of_2_sub({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_mod_power_of_2_sub(&xs, &ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_sub_in_place_left(gm: GenerationMode, limit: usize) {
    for (mut xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_13(gm).take(limit) {
        let xs_old = xs.clone();
        limbs_mod_power_of_2_sub_in_place_left(&mut xs, &ys, pow);
        println!(
            "xs := {:?}; limbs_mod_power_of_2_sub_in_place_left(&mut xs, {:?}, {}); xs = {:?}",
            xs_old, ys, pow, xs
        );
    }
}

fn demo_limbs_mod_power_of_2_sub_in_place_right(gm: GenerationMode, limit: usize) {
    for (xs, mut ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_15(gm).take(limit) {
        let ys_old = ys.clone();
        limbs_mod_power_of_2_sub_in_place_right(&xs, &mut ys, pow);
        println!(
            "ys := {:?}; limbs_mod_power_of_2_sub_in_place_right({:?}, &mut ys, {}); ys = {:?}",
            ys_old, xs, pow, ys
        );
    }
}

fn demo_limbs_mod_power_of_2_sub_in_place_either(gm: GenerationMode, limit: usize) {
    for (mut xs, mut ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_15(gm).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_mod_power_of_2_sub_in_place_either(&mut xs, &mut ys, pow);
        println!(
            "xs := {:?}; ys := {:?}; \
            limbs_mod_power_of_2_sub_in_place_either(&mut xs, &mut ys, {}) = {}; \
            xs = {:?}; ys = {:?}",
            xs_old, ys_old, pow, right, xs, ys
        );
    }
}

fn demo_natural_mod_power_of_2_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        x.mod_power_of_2_sub_assign(y.clone(), pow);
        println!(
            "x := {}; x.mod_power_of_2_sub_assign({}, {}); x = {}",
            x_old, y, pow, x
        );
    }
}

fn demo_natural_mod_power_of_2_sub_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        x.mod_power_of_2_sub_assign(&y, pow);
        println!(
            "x := {}; x.mod_power_of_2_sub_assign(&{}, {}); x = {}",
            x_old, y, pow, x
        );
    }
}

fn demo_natural_mod_power_of_2_sub(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{} - {} === {} mod 2^{}",
            x_old,
            y_old,
            x.mod_power_of_2_sub(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_sub_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{} - {} === {} mod 2^{}",
            x_old,
            y,
            x.mod_power_of_2_sub(&y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_sub_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "{} - {} === {} mod 2^{}",
            x,
            y_old,
            (&x).mod_power_of_2_sub(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        println!(
            "{} - {} === {} mod 2^{}",
            x,
            y,
            (&x).mod_power_of_2_sub(&y, pow),
            pow
        );
    }
}

fn benchmark_limbs_mod_power_of_2_limb_sub_limbs(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mod_power_of_2_limb_sub_limbs(Limb, &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_limb_limb_vec_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(x, ys, pow)| no_out!(limbs_mod_power_of_2_limb_sub_limbs(x, &ys, pow))),
        )],
    );
}

fn benchmark_limbs_mod_power_of_2_limb_sub_limbs_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mod_power_of_2_limb_sub_limbs(Limb, &mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        triples_of_limb_limb_vec_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(x, mut ys, pow)| limbs_mod_power_of_2_limb_sub_limbs_in_place(x, &mut ys, pow)),
        )],
    );
}

fn benchmark_limbs_mod_power_of_2_sub(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_mod_power_of_2_sub(&[Limb], &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_13(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(ref xs, ref ys, pow)| no_out!(limbs_mod_power_of_2_sub(xs, ys, pow))),
        )],
    );
}

fn benchmark_limbs_mod_power_of_2_sub_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mod_power_of_2_sub_in_place_left(&mut Vec<Limb>, &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_13(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(ref mut xs, ref ys, pow)| limbs_mod_power_of_2_sub_in_place_left(xs, ys, pow)),
        )],
    );
}

fn benchmark_limbs_mod_power_of_2_sub_in_place_right(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mod_power_of_2_sub_in_place_right(&[Limb], &mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_15(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(ref xs, ref mut ys, pow)| limbs_mod_power_of_2_sub_in_place_right(xs, ys, pow)),
        )],
    );
}

fn benchmark_limbs_mod_power_of_2_sub_in_place_either(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_mod_power_of_2_sub_in_place_left(&mut Vec<Limb>, &mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_15(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(ref mut xs, ref mut ys, pow)| {
                no_out!(limbs_mod_power_of_2_sub_in_place_either(xs, ys, pow))
            }),
        )],
    );
}

fn benchmark_natural_mod_power_of_2_sub_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_sub_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_2_sub_assign(Natural, u64)",
                &mut (|(mut x, y, pow)| no_out!(x.mod_power_of_2_sub_assign(y, pow))),
            ),
            (
                "Natural.mod_power_of_2_sub_assign(&Natural, u64)",
                &mut (|(mut x, y, pow)| no_out!(x.mod_power_of_2_sub_assign(&y, pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_sub_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_sub(Natural, u64)",
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
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_2_sub(y, pow))),
            ),
            (
                "alt",
                &mut (|(x, y, pow)| {
                    if x >= y {
                        x - y
                    } else {
                        let mut x = x.clone();
                        x.set_bit(pow);
                        x - y
                    };
                }),
            ),
            (
                "naive",
                &mut (|(x, y, pow)| {
                    no_out!((Integer::from(x) - Integer::from(y)).mod_power_of_2(pow))
                }),
            ),
            (
                "using mod_sub",
                &mut (|(x, y, pow)| no_out!(x.mod_sub(y, Natural::power_of_2(pow)))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_sub_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_sub(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_2_sub(Natural, u64)",
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_2_sub(y, pow))),
            ),
            (
                "Natural.mod_power_of_2_sub(&Natural, u64)",
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_2_sub(&y, pow))),
            ),
            (
                "(&Natural).mod_power_of_2_sub(Natural, u64)",
                &mut (|(x, y, pow)| no_out!((&x).mod_power_of_2_sub(y, pow))),
            ),
            (
                "(&Natural).mod_power_of_2_sub(&Natural, u64)",
                &mut (|(x, y, pow)| no_out!((&x).mod_power_of_2_sub(&y, pow))),
            ),
        ],
    );
}
