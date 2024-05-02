// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use malachite_base::test_util::bench::bucketers::triple_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_triple_gen_var_59, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::sub_mul::{
    limbs_sub_mul, limbs_sub_mul_in_place_left, limbs_sub_mul_limb_greater,
    limbs_sub_mul_limb_greater_in_place_left, limbs_sub_mul_limb_greater_in_place_right,
    limbs_sub_mul_limb_same_length_in_place_left, limbs_sub_mul_limb_same_length_in_place_right,
};
use malachite_nz::test_util::bench::bucketers::triple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_triple_gen_var_7;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_sub_mul_limb_greater);
    register_demo!(runner, demo_limbs_sub_mul_limb_same_length_in_place_left);
    register_demo!(runner, demo_limbs_sub_mul_limb_greater_in_place_left);
    register_demo!(runner, demo_limbs_sub_mul_limb_same_length_in_place_right);
    register_demo!(runner, demo_limbs_sub_mul_limb_greater_in_place_right);
    register_demo!(runner, demo_limbs_sub_mul);
    register_demo!(runner, demo_limbs_sub_mul_in_place_left);
    register_demo!(runner, demo_natural_sub_mul_assign);
    register_demo!(runner, demo_natural_sub_mul_assign_val_ref);
    register_demo!(runner, demo_natural_sub_mul_assign_ref_val);
    register_demo!(runner, demo_natural_sub_mul_assign_ref_ref);
    register_demo!(runner, demo_natural_sub_mul);
    register_demo!(runner, demo_natural_sub_mul_val_val_ref);
    register_demo!(runner, demo_natural_sub_mul_val_ref_val);
    register_demo!(runner, demo_natural_sub_mul_val_ref_ref);
    register_demo!(runner, demo_natural_sub_mul_ref_ref_ref);

    register_bench!(runner, benchmark_limbs_sub_mul_limb_greater);
    register_bench!(
        runner,
        benchmark_limbs_sub_mul_limb_same_length_in_place_left
    );
    register_bench!(runner, benchmark_limbs_sub_mul_limb_greater_in_place_left);
    register_bench!(
        runner,
        benchmark_limbs_sub_mul_limb_same_length_in_place_right
    );
    register_bench!(runner, benchmark_limbs_sub_mul_limb_greater_in_place_right);
    register_bench!(runner, benchmark_limbs_sub_mul);
    register_bench!(runner, benchmark_limbs_sub_mul_in_place_left);
    register_bench!(runner, benchmark_natural_sub_mul_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_sub_mul_assign_algorithms);
    register_bench!(runner, benchmark_natural_sub_mul_assign_val_ref_algorithms);
    register_bench!(runner, benchmark_natural_sub_mul_assign_ref_val_algorithms);
    register_bench!(runner, benchmark_natural_sub_mul_assign_ref_ref_algorithms);
    register_bench!(runner, benchmark_natural_sub_mul_evaluation_stategy);
    register_bench!(runner, benchmark_natural_sub_mul_algorithms);
    register_bench!(runner, benchmark_natural_sub_mul_val_val_ref_algorithms);
    register_bench!(runner, benchmark_natural_sub_mul_val_ref_val_algorithms);
    register_bench!(runner, benchmark_natural_sub_mul_val_ref_ref_algorithms);
    register_bench!(runner, benchmark_natural_sub_mul_ref_ref_ref_algorithms);
}

fn demo_limbs_sub_mul_limb_greater(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, z) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_sub_mul_limb_greater({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            z,
            limbs_sub_mul_limb_greater(&xs, &ys, z),
        );
    }
}

fn demo_limbs_sub_mul_limb_same_length_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut xs, ys, z) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let borrow = limbs_sub_mul_limb_same_length_in_place_left(&mut xs, &ys, z);
        println!(
            "xs := {xs_old:?}; \
            limbs_sub_mul_limb_same_length_in_place_left(&mut xs, {ys:?}, {z}) = {borrow}; \
             xs = {xs:?}",
        );
    }
}

fn demo_limbs_sub_mul_limb_greater_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, z) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let borrow = limbs_sub_mul_limb_greater_in_place_left(&mut xs, &ys, z);
        println!(
            "xs := {xs_old:?}; \
            limbs_sub_mul_limb_greater_in_place_left(&mut xs, {ys:?}, {z}) = {borrow}; \
             xs = {xs:?}",
        );
    }
}

fn demo_limbs_sub_mul_limb_same_length_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, mut ys, z) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12()
        .get(gm, config)
        .take(limit)
    {
        let ys_old = ys.clone();
        limbs_sub_mul_limb_same_length_in_place_right(&xs, &mut ys, z);
        println!(
            "ys := {ys_old:?}; \
            limbs_sub_mul_limb_same_length_in_place_right({xs:?}, &mut ys, {z}); \
            ys = {ys:?}",
        );
    }
}

fn demo_limbs_sub_mul_limb_greater_in_place_right(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys, z) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let ys_old = ys.clone();
        limbs_sub_mul_limb_greater_in_place_right(&xs, &mut ys, z);
        println!(
            "ys := {ys_old:?}; \
            limbs_sub_mul_limb_greater_in_place_right({xs:?}, &mut ys, {z}); ys = {ys:?}",
        );
    }
}

fn demo_limbs_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, zs) in unsigned_vec_triple_gen_var_59().get(gm, config).take(limit) {
        println!(
            "limbs_sub_mul({:?}, {:?}, {:?}) = {:?}",
            xs,
            ys,
            zs,
            limbs_sub_mul(&xs, &ys, &zs),
        );
    }
}

fn demo_limbs_sub_mul_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, zs) in unsigned_vec_triple_gen_var_59().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_sub_mul_in_place_left(&mut xs, &ys, &zs);
        println!(
            "xs := {xs_old:?}; limbs_sub_mul_in_place_left(&mut xs, {ys:?}, {zs:?}); xs = {xs:?}",
        );
    }
}

fn demo_natural_sub_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.sub_mul_assign(b, c);
        println!("a := {a_old}; x.sub_mul_assign({b_old}, {c_old}); x = {a}");
    }
}

fn demo_natural_sub_mul_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, &c);
        println!("a := {a_old}; x.sub_mul_assign({b_old}, &{c}); x = {a}");
    }
}

fn demo_natural_sub_mul_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {a_old}; x.sub_mul_assign(&{b}, {c_old}); x = {a}");
    }
}

fn demo_natural_sub_mul_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, &c);
        println!("a := {a_old}; x.sub_mul_assign(&{b}, &{c}); x = {a}");
    }
}

fn demo_natural_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c_old,
            a.sub_mul(b, c)
        );
    }
}

fn demo_natural_sub_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.sub_mul({}, &{}) = {}",
            a_old,
            b_old,
            c,
            a.sub_mul(b, &c)
        );
    }
}

fn demo_natural_sub_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c_old,
            a.sub_mul(&b, c)
        );
    }
}

fn demo_natural_sub_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, &{}) = {}", a_old, b, c, a.sub_mul(&b, &c));
    }
}

fn demo_natural_sub_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen_var_7().get(gm, config).take(limit) {
        println!(
            "(&{}).sub_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).sub_mul(&b, &c)
        );
    }
}

fn benchmark_limbs_sub_mul_limb_greater(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_mul_limb_greater(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, ys, z)| {
            no_out!(limbs_sub_mul_limb_greater(&xs, &ys, z))
        })],
    );
}

fn benchmark_limbs_sub_mul_limb_same_length_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_mul_limb_same_length_in_place_left(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, ys, z)| {
            no_out!(limbs_sub_mul_limb_same_length_in_place_left(
                &mut xs, &ys, z
            ))
        })],
    );
}

fn benchmark_limbs_sub_mul_limb_greater_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_mul_limb_greater_in_place_left(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, ys, z)| {
            no_out!(limbs_sub_mul_limb_greater_in_place_left(&mut xs, &ys, z))
        })],
    );
}

fn benchmark_limbs_sub_mul_limb_same_length_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_mul_limb_same_length_in_place_right(&[Limb], &mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, mut ys, z)| {
            no_out!(limbs_sub_mul_limb_same_length_in_place_right(
                &xs, &mut ys, z
            ))
        })],
    );
}

fn benchmark_limbs_sub_mul_limb_greater_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_mul_limb_greater_in_place_right(&[Limb], &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, mut ys, z)| {
            no_out!(limbs_sub_mul_limb_same_length_in_place_right(
                &xs, &mut ys, z
            ))
        })],
    );
}

fn benchmark_limbs_sub_mul(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sub_mul(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_59().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, ys, zs)| {
            no_out!(limbs_sub_mul(&xs, &ys, &zs))
        })],
    );
}

fn benchmark_limbs_sub_mul_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_mul_in_place_left(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_59().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, ys, zs)| {
            no_out!(limbs_sub_mul_in_place_left(&mut xs, &ys, &zs))
        })],
    );
}

fn benchmark_natural_sub_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.sub_mul_assign(Natural, Natural)",
                &mut |(mut a, b, c)| a.sub_mul_assign(b, c),
            ),
            (
                "Natural.sub_mul_assign(Natural, &Natural)",
                &mut |(mut a, b, c)| a.sub_mul_assign(b, &c),
            ),
            (
                "Natural.sub_mul_assign(&Natural, Natural)",
                &mut |(mut a, b, c)| a.sub_mul_assign(&b, c),
            ),
            (
                "Natural.sub_mul_assign(&Natural, &Natural)",
                &mut |(mut a, b, c)| a.sub_mul_assign(&b, &c),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.sub_mul_assign(Natural, Natural)",
                &mut |(mut a, b, c)| a.sub_mul_assign(b, c),
            ),
            ("Natural += Natural * Natural", &mut |(mut a, b, c)| {
                a += b * c
            }),
        ],
    );
}

fn benchmark_natural_sub_mul_assign_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul_assign(Natural, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.sub_mul_assign(Natural, &Natural)",
                &mut |(mut a, b, c)| a.sub_mul_assign(b, &c),
            ),
            ("Natural += Natural * &Natural", &mut |(mut a, b, c)| {
                a += b * &c
            }),
        ],
    );
}

fn benchmark_natural_sub_mul_assign_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul_assign(&Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.sub_mul_assign(&Natural, Natural)",
                &mut |(mut a, b, c)| a.sub_mul_assign(&b, c),
            ),
            ("Natural += &Natural * Natural", &mut |(mut a, b, c)| {
                a += &b * c
            }),
        ],
    );
}

fn benchmark_natural_sub_mul_assign_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul_assign(&Natural, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.sub_mul_assign(&Natural, &Natural)",
                &mut |(mut a, b, c)| a.sub_mul_assign(&b, &c),
            ),
            ("Natural += &Natural * &Natural", &mut |(mut a, b, c)| {
                a += &b * &c
            }),
        ],
    );
}

fn benchmark_natural_sub_mul_evaluation_stategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.sub_mul(Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(b, c))
            }),
            ("Natural.sub_mul(Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(b, &c))
            }),
            ("Natural.sub_mul(&Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(&b, c))
            }),
            ("Natural.sub_mul(&Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(&b, &c))
            }),
            (
                "(&Natural).sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!((&a).sub_mul(&b, &c)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_sub_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.sub_mul(Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(b, c))
            }),
            ("Natural - Natural * Natural", &mut |(a, b, c)| {
                no_out!(a - b * c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_sub_mul_val_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul(Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.sub_mul(Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(b, &c))
            }),
            ("Natural - Natural * &Natural", &mut |(a, b, c)| {
                no_out!(a - b * &c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_sub_mul_val_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul(&Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.sub_mul(&Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(&b, c))
            }),
            ("Natural - &Natural * Natural", &mut |(a, b, c)| {
                no_out!(a - &b * c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_sub_mul_val_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.sub_mul(&Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(&b, &c))
            }),
            ("Natural - &Natural * &Natural", &mut |(a, b, c)| {
                no_out!(a - &b * &c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_sub_mul_ref_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Natural).sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "(&Natural).sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!((&a).sub_mul(&b, &c)),
            ),
            ("(&Natural) - &Natural * &Natural", &mut |(a, b, c)| {
                no_out!((&a) - &b * &c)
            }),
        ],
    );
}
