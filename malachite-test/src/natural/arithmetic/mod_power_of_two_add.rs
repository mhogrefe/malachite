use std::cmp::{max, min};

use malachite_base::num::arithmetic::traits::{
    ModAdd, ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoAddAssign, PowerOfTwo,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::natural::arithmetic::mod_power_of_two_add::{
    limbs_mod_power_of_two_add, limbs_mod_power_of_two_add_greater,
    limbs_mod_power_of_two_add_in_place_either, limbs_mod_power_of_two_add_limb,
    limbs_slice_mod_power_of_two_add_greater_in_place_left,
    limbs_slice_mod_power_of_two_add_limb_in_place, limbs_vec_mod_power_of_two_add_in_place_left,
    limbs_vec_mod_power_of_two_add_limb_in_place,
};
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_limb_vec_limb_and_u64_var_1, triples_of_limb_vec_limb_and_u64_var_2,
    triples_of_limb_vec_limb_vec_and_u64_var_13, triples_of_limb_vec_limb_vec_and_u64_var_14,
};
use inputs::natural::triples_of_natural_natural_and_u64_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_power_of_two_add_limb);
    register_demo!(
        registry,
        demo_limbs_slice_mod_power_of_two_add_limb_in_place
    );
    register_demo!(registry, demo_limbs_vec_mod_power_of_two_add_limb_in_place);
    register_demo!(registry, demo_limbs_mod_power_of_two_add_greater);
    register_demo!(registry, demo_limbs_mod_power_of_two_add);
    register_demo!(
        registry,
        demo_limbs_slice_mod_power_of_two_add_greater_in_place_left
    );
    register_demo!(registry, demo_limbs_vec_mod_power_of_two_add_in_place_left);
    register_demo!(registry, demo_limbs_mod_power_of_two_add_in_place_either);
    register_demo!(registry, demo_natural_mod_power_of_two_add_assign);
    register_demo!(registry, demo_natural_mod_power_of_two_add_assign_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_add);
    register_demo!(registry, demo_natural_mod_power_of_two_add_val_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_add_ref_val);
    register_demo!(registry, demo_natural_mod_power_of_two_add_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_mod_power_of_two_add_limb);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_mod_power_of_two_add_limb_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_mod_power_of_two_add_limb_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_two_add_greater
    );
    register_bench!(registry, Small, benchmark_limbs_mod_power_of_two_add);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_mod_power_of_two_add_greater_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_mod_power_of_two_add_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_power_of_two_add_in_place_either
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_add_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_add_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_add_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_two_add_limb(gm: GenerationMode, limit: usize) {
    for (xs, y, pow) in triples_of_limb_vec_limb_and_u64_var_1(gm).take(limit) {
        println!(
            "limbs_mod_power_of_two_add_limb({:?}, {}, {}) = {:?}",
            xs,
            y,
            pow,
            limbs_mod_power_of_two_add_limb(&xs, y, pow)
        );
    }
}

fn demo_limbs_slice_mod_power_of_two_add_limb_in_place(gm: GenerationMode, limit: usize) {
    for (xs, y, pow) in triples_of_limb_vec_limb_and_u64_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_slice_mod_power_of_two_add_limb_in_place(&mut xs, y, pow);
        println!(
            "xs := {:?}; limbs_slice_mod_power_of_two_add_limb_in_place(&mut xs, {}, {}) = {}; \
            xs = {:?}",
            xs_old, y, pow, carry, xs
        );
    }
}

fn demo_limbs_vec_mod_power_of_two_add_limb_in_place(gm: GenerationMode, limit: usize) {
    for (mut xs, y, pow) in triples_of_limb_vec_limb_and_u64_var_2(gm).take(limit) {
        let xs_old = xs.clone();
        limbs_vec_mod_power_of_two_add_limb_in_place(&mut xs, y, pow);
        println!(
            "xs := {:?}; limbs_vec_mod_power_of_two_add_limb_in_place(&mut xs, {}, {}); xs = {:?}",
            xs_old, y, pow, xs
        );
    }
}

fn demo_limbs_mod_power_of_two_add_greater(gm: GenerationMode, limit: usize) {
    for (xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_14(gm).take(limit) {
        println!(
            "limbs_add_greater({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_mod_power_of_two_add_greater(&xs, &ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_two_add(gm: GenerationMode, limit: usize) {
    for (xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_13(gm).take(limit) {
        println!(
            "limbs_mod_power_of_two_add({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_mod_power_of_two_add(&xs, &ys, pow)
        );
    }
}

fn demo_limbs_slice_mod_power_of_two_add_greater_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_14(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_slice_mod_power_of_two_add_greater_in_place_left(&mut xs, &ys, pow);
        println!(
            "xs := {:?}; limbs_slice_mod_power_of_two_add_greater_in_place_left(&mut xs, {:?}, {}) \
            = {}; xs = {:?}",
            xs_old, ys, pow, carry, xs
        );
    }
}

fn demo_limbs_vec_mod_power_of_two_add_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_13(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_mod_power_of_two_add_in_place_left(&mut xs, &ys, pow);
        println!(
            "xs := {:?}; limbs_vec_mod_power_of_two_add_in_place_left(&mut xs, {:?}, {}); \
            xs = {:?}",
            xs_old, ys, pow, xs
        );
    }
}

fn demo_limbs_mod_power_of_two_add_in_place_either(gm: GenerationMode, limit: usize) {
    for (xs, ys, pow) in triples_of_limb_vec_limb_vec_and_u64_var_13(gm).take(limit) {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_mod_power_of_two_add_in_place_either(&mut xs, &mut ys, pow);
        println!(
            "xs := {:?}; ys := {:?}; limbs_mod_power_of_two_add_in_place_either(&mut xs, &mut ys, \
            {}) = {}; xs = {:?}; ys = {:?}",
            xs_old, ys_old, pow, right, xs, ys
        );
    }
}

fn demo_natural_mod_power_of_two_add_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_power_of_two_add_assign(y, pow);
        println!(
            "x := {}; x.mod_power_of_two_add_assign({}, {}); x = {}",
            x_old, y_old, pow, x
        );
    }
}

fn demo_natural_mod_power_of_two_add_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        x.mod_power_of_two_add_assign(&y, pow);
        println!(
            "x := {}; x.mod_power_of_two_add_assign(&{}, {}); x = {}",
            x_old, y, pow, x
        );
    }
}

fn demo_natural_mod_power_of_two_add(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{} + {} === {} mod 2^{}",
            x_old,
            y_old,
            x.mod_power_of_two_add(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_two_add_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{} + {} === {} mod 2^{}",
            x_old,
            y,
            x.mod_power_of_two_add(&y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_two_add_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "{} + {} === {} mod 2^{}",
            x,
            y_old,
            (&x).mod_power_of_two_add(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_two_add_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, pow) in triples_of_natural_natural_and_u64_var_1(gm).take(limit) {
        println!(
            "{} + {} === {} mod 2^{}",
            x,
            y,
            (&x).mod_power_of_two_add(&y, pow),
            pow
        );
    }
}

fn benchmark_limbs_mod_power_of_two_add_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_power_of_two_add_limb(&[Limb], Limb, u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(xs, y, pow)| no_out!(limbs_mod_power_of_two_add_limb(&xs, y, pow))),
        )],
    );
}

fn benchmark_limbs_slice_mod_power_of_two_add_limb_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_slice_mod_power_of_two_add_limb_in_place(&mut [Limb], Limb, u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb, pow)| {
                no_out!(limbs_slice_mod_power_of_two_add_limb_in_place(
                    &mut limbs, limb, pow
                ))
            }),
        )],
    );
}

fn benchmark_limbs_vec_mod_power_of_two_add_limb_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_vec_mod_power_of_two_add_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb, pow)| {
                limbs_vec_mod_power_of_two_add_limb_in_place(&mut limbs, limb, pow)
            }),
        )],
    );
}

fn benchmark_limbs_mod_power_of_two_add_greater(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_power_of_two_add_greater(&[Limb], &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_14(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys, pow)| no_out!(limbs_mod_power_of_two_add_greater(xs, ys, pow))),
        )],
    );
}

fn benchmark_limbs_mod_power_of_two_add(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_power_of_two_add(&[Limb], &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_13(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys, pow)| no_out!(limbs_mod_power_of_two_add(xs, ys, pow))),
        )],
    );
}

fn benchmark_limbs_slice_mod_power_of_two_add_greater_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_slice_mod_power_of_two_add_greater_in_place_left(&mut [Limb], &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_14(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, pow)| {
                no_out!(limbs_slice_mod_power_of_two_add_greater_in_place_left(
                    &mut xs, &ys, pow
                ))
            }),
        )],
    );
}

fn benchmark_limbs_vec_mod_power_of_two_add_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_vec_mod_power_of_two_add_in_place_left(&Vec<Limb>, &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_13(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, pow)| {
                no_out!(limbs_vec_mod_power_of_two_add_in_place_left(
                    &mut xs, &ys, pow
                ))
            }),
        )],
    );
}

fn benchmark_limbs_mod_power_of_two_add_in_place_either(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_mod_power_of_two_add_in_place_either(&mut [Limb], &mut [Limb], u64)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_vec_and_u64_var_13(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, mut ys, pow)| {
                no_out!(limbs_mod_power_of_two_add_in_place_either(
                    &mut xs, &mut ys, pow
                ))
            }),
        )],
    );
}

fn benchmark_natural_mod_power_of_two_add_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_power_of_two_add_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_two_add_assign(Natural, u64)",
                &mut (|(mut x, y, pow)| no_out!(x.mod_power_of_two_add_assign(y, pow))),
            ),
            (
                "Natural.mod_power_of_two_add_assign(&Natural, u64)",
                &mut (|(mut x, y, pow)| no_out!(x.mod_power_of_two_add_assign(&y, pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_two_add_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_power_of_two_add(Natural, u64)",
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
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_add(y, pow))),
            ),
            (
                "alt",
                &mut (|(x, y, pow)| {
                    let mut sum = x + y;
                    sum.clear_bit(pow);
                }),
            ),
            (
                "naive",
                &mut (|(x, y, pow)| no_out!((x + y).mod_power_of_two(pow))),
            ),
            (
                "using mod_add",
                &mut (|(x, y, pow)| no_out!(x.mod_add(y, Natural::power_of_two(pow)))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_two_add_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_power_of_two_add(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_two_add(Natural, u64)",
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_add(y, pow))),
            ),
            (
                "Natural.mod_power_of_two_add(&Natural, u64)",
                &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_add(&y, pow))),
            ),
            (
                "(&Natural).mod_power_of_two_add(Natural, u64)",
                &mut (|(x, y, pow)| no_out!((&x).mod_power_of_two_add(y, pow))),
            ),
            (
                "(&Natural).mod_power_of_two_add(&Natural, u64)",
                &mut (|(x, y, pow)| no_out!((&x).mod_power_of_two_add(&y, pow))),
            ),
        ],
    );
}
