// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use malachite_base::test_util::bench::bucketers::{
    triple_1_2_vec_max_len_bucketer, triple_vec_max_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_triple_gen_var_39, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::arithmetic::sub_mul::{
    limbs_overflowing_sub_mul, limbs_overflowing_sub_mul_in_place_left,
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left, limbs_overflowing_sub_mul_limb_in_place_right,
};
use malachite_nz::test_util::bench::bucketers::triple_integer_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_triple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_overflowing_sub_mul_limb);
    register_demo!(runner, demo_limbs_overflowing_sub_mul_limb_in_place_left);
    register_demo!(runner, demo_limbs_overflowing_sub_mul_limb_in_place_right);
    register_demo!(runner, demo_limbs_overflowing_sub_mul_limb_in_place_either);
    register_demo!(runner, demo_limbs_overflowing_sub_mul);
    register_demo!(runner, demo_limbs_overflowing_sub_mul_in_place_left);

    register_demo!(runner, demo_integer_sub_mul);
    register_demo!(runner, demo_integer_sub_mul_val_val_ref);
    register_demo!(runner, demo_integer_sub_mul_val_ref_val);
    register_demo!(runner, demo_integer_sub_mul_val_ref_ref);
    register_demo!(runner, demo_integer_sub_mul_ref_ref_ref);
    register_demo!(runner, demo_integer_sub_mul_assign);
    register_demo!(runner, demo_integer_sub_mul_assign_val_ref);
    register_demo!(runner, demo_integer_sub_mul_assign_ref_val);
    register_demo!(runner, demo_integer_sub_mul_assign_ref_ref);

    register_bench!(runner, benchmark_limbs_overflowing_sub_mul_limb);
    register_bench!(
        runner,
        benchmark_limbs_overflowing_sub_mul_limb_in_place_left
    );
    register_bench!(
        runner,
        benchmark_limbs_overflowing_sub_mul_limb_in_place_right
    );
    register_bench!(
        runner,
        benchmark_limbs_overflowing_sub_mul_limb_in_place_either
    );
    register_bench!(runner, benchmark_limbs_overflowing_sub_mul);
    register_bench!(runner, benchmark_limbs_overflowing_sub_mul_in_place_left);

    register_bench!(runner, benchmark_integer_sub_mul_evaluation_strategy);
    register_bench!(runner, benchmark_integer_sub_mul_algorithms);
    register_bench!(runner, benchmark_integer_sub_mul_val_val_ref_algorithms);
    register_bench!(runner, benchmark_integer_sub_mul_val_ref_val_algorithms);
    register_bench!(runner, benchmark_integer_sub_mul_val_ref_ref_algorithms);
    register_bench!(runner, benchmark_integer_sub_mul_ref_ref_ref_algorithms);
    register_bench!(runner, benchmark_integer_sub_mul_assign_evaluation_strategy);
    register_bench!(runner, benchmark_integer_sub_mul_assign_algorithms);
    register_bench!(runner, benchmark_integer_sub_mul_assign_val_ref_algorithms);
    register_bench!(runner, benchmark_integer_sub_mul_assign_ref_val_algorithms);
    register_bench!(runner, benchmark_integer_sub_mul_assign_ref_ref_algorithms);
}

fn demo_limbs_overflowing_sub_mul_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_overflowing_sub_mul_limb({:?}, {:?}, {}) = {:?}",
            a,
            b,
            c,
            limbs_overflowing_sub_mul_limb(&a, &b, c),
        );
    }
}

fn demo_limbs_overflowing_sub_mul_limb_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let borrow = limbs_overflowing_sub_mul_limb_in_place_left(&mut a, &b, c);
        println!(
            "a := {a_old:?}; \
            limbs_overflowing_sub_mul_limb_in_place_left(&mut a, {b:?}, {c}) = {borrow}; \
            a = {a:?}",
        );
    }
}

fn demo_limbs_overflowing_sub_mul_limb_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, mut b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let b_old = b.clone();
        let borrow = limbs_overflowing_sub_mul_limb_in_place_right(&a, &mut b, c);
        println!(
            "b := {b_old:?}; \
            limbs_overflowing_sub_mul_limb_in_place_right({a:?}, &mut b, {c}) = {borrow}; \
            b = {b:?}",
        );
    }
}

fn demo_limbs_overflowing_sub_mul_limb_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, mut b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        limbs_overflowing_sub_mul_limb_in_place_either(&mut a, &mut b, c);
        println!(
            "a := {a_old:?}; b := {b_old:?}; \
            limbs_overflowing_sub_mul_limb_in_place_either(&mut a, &mut b, {c}); \
            a = {a:?}; b = {b:?}",
        );
    }
}

fn demo_limbs_overflowing_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in unsigned_vec_triple_gen_var_39().get(gm, config).take(limit) {
        println!(
            "limbs_overflowing_sub_mul({:?}, {:?}, {:?}) = {:?}",
            a,
            b,
            c,
            limbs_overflowing_sub_mul(&a, &b, &c),
        );
    }
}

fn demo_limbs_overflowing_sub_mul_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in unsigned_vec_triple_gen_var_39().get(gm, config).take(limit) {
        let a_old = a.clone();
        let sign = limbs_overflowing_sub_mul_in_place_left(&mut a, &b, &c);
        println!(
            "a := {a_old:?}; \
            limbs_overflowing_sub_mul_in_place_left(&mut a, {b:?}, {c:?}) = {sign}; \
            a = {a:?}",
        );
    }
}

fn demo_integer_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!(
            "{}.sub_mul({}, {}) = {}",
            x_old,
            y_old,
            z_old,
            x.sub_mul(y, z)
        );
    }
}

fn demo_integer_sub_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.sub_mul({}, &({})) = {}",
            x_old,
            y_old,
            z,
            x.sub_mul(y, &z)
        );
    }
}

fn demo_integer_sub_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let z_old = z.clone();
        println!(
            "{}.sub_mul(&({}), {}) = {}",
            x_old,
            y,
            z_old,
            x.sub_mul(&y, z)
        );
    }
}

fn demo_integer_sub_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.sub_mul(&({}), &({})) = {}",
            x_old,
            y,
            z,
            x.sub_mul(&y, &z)
        );
    }
}

fn demo_integer_sub_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).sub_mul(&({}), &({})) = {}",
            x,
            y,
            z,
            (&x).sub_mul(&y, &z)
        );
    }
}

fn demo_integer_sub_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        x.sub_mul_assign(y, z);
        println!("x := {x_old}; x.sub_mul_assign({y_old}, {z_old}); x = {x}");
    }
}

fn demo_integer_sub_mul_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.sub_mul_assign(y, &z);
        println!("x := {x_old}; x.sub_mul_assign({y_old}, &({z})); x = {x}");
    }
}

fn demo_integer_sub_mul_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let z_old = z.clone();
        x.sub_mul_assign(&y, z);
        println!("x := {x_old}; x.sub_mul_assign(&({y}), {z_old}); x = {x}");
    }
}

fn demo_integer_sub_mul_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.sub_mul_assign(&y, &z);
        println!("x := {x_old}; x.sub_mul_assign(&({y}), &({z})); x = {x}");
    }
}

fn benchmark_limbs_overflowing_sub_mul_limb(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_overflowing_sub_mul_limb(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(a, b, c)| {
            no_out!(limbs_overflowing_sub_mul_limb(&a, &b, c))
        })],
    );
}

fn benchmark_limbs_overflowing_sub_mul_limb_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_overflowing_sub_mul_limb_in_place_left(&mut Vec<Limb>, &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut a, b, c)| {
            no_out!(limbs_overflowing_sub_mul_limb_in_place_left(&mut a, &b, c))
        })],
    );
}

fn benchmark_limbs_overflowing_sub_mul_limb_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_overflowing_sub_mul_limb_in_place_right(&[Limb], &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(a, mut b, c)| {
            no_out!(limbs_overflowing_sub_mul_limb_in_place_right(&a, &mut b, c))
        })],
    );
}

fn benchmark_limbs_overflowing_sub_mul_limb_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_overflowing_sub_mul_limb_in_place_either(&mut Vec<Limb>, &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut a, mut b, c)| {
            no_out!(limbs_overflowing_sub_mul_limb_in_place_either(
                &mut a, &mut b, c
            ))
        })],
    );
}

fn benchmark_limbs_overflowing_sub_mul(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_overflowing_sub_mul(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_39().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_vec_max_len_bucketer("xs", "ys", "zs"),
        &mut [("Malachite", &mut |(a, b, c)| {
            no_out!(limbs_overflowing_sub_mul(&a, &b, &c))
        })],
    );
}

fn benchmark_limbs_overflowing_sub_mul_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_overflowing_sub_mul_in_place_left(&mut Vec<Limb>, &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_39().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_vec_max_len_bucketer("xs", "ys", "zs"),
        &mut [("Malachite", &mut |(mut a, b, c)| {
            no_out!(limbs_overflowing_sub_mul_in_place_left(&mut a, &b, &c))
        })],
    );
}

fn benchmark_integer_sub_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.sub_mul(Integer, Integer)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(b, c))
            }),
            ("Integer.sub_mul(Integer, &Integer)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(b, &c))
            }),
            ("Integer.sub_mul(&Integer, Integer)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(&b, c))
            }),
            ("Integer.sub_mul(&Integer, &Integer)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(&b, &c))
            }),
            (
                "(&Integer).sub_mul(&Integer, &Integer)",
                &mut |(a, b, c)| no_out!((&a).sub_mul(&b, &c)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_sub_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul(Integer, Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.sub_mul(Integer, Integer)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(b, c))
            }),
            ("Integer - Integer * Integer", &mut |(a, b, c)| {
                no_out!(a - b * c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_sub_mul_val_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul(Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.sub_mul(Integer, &Integer)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(b, &c))
            }),
            ("Integer - Integer * &Integer", &mut |(a, b, c)| {
                no_out!(a - b * &c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_sub_mul_val_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul(&Integer, Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.sub_mul(&Integer, Integer)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(&b, c))
            }),
            ("Integer - &Integer * Integer", &mut |(a, b, c)| {
                no_out!(a - &b * c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_sub_mul_val_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.sub_mul(&Integer, &Integer)", &mut |(a, b, c)| {
                no_out!(a.sub_mul(&b, &c))
            }),
            ("Integer - Integer * Integer", &mut |(a, b, c)| {
                no_out!(a - &b * &c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_sub_mul_ref_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "(&Integer).sub_mul(&Integer, &Integer)",
                &mut |(a, b, c)| no_out!((&a).sub_mul(&b, &c)),
            ),
            ("&Integer - Integer * Integer", &mut |(a, b, c)| {
                no_out!(&a - &b * &c)
            }),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul_assign(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.sub_mul_assign(Integer, Integer)",
                &mut |(mut a, b, c)| a.sub_mul_assign(b, c),
            ),
            (
                "Integer.sub_mul_assign(Integer, &Integer)",
                &mut |(mut a, b, c)| a.sub_mul_assign(b, &c),
            ),
            (
                "Integer.sub_mul_assign(&Integer, Integer)",
                &mut |(mut a, b, c)| a.sub_mul_assign(&b, c),
            ),
            (
                "Integer.sub_mul_assign(&Integer, &Integer)",
                &mut |(mut a, b, c)| a.sub_mul_assign(&b, &c),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul_assign(Integer, Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.sub_mul_assign(Integer, Integer)",
                &mut |(mut a, b, c)| a.sub_mul_assign(b, c),
            ),
            ("Integer -= Integer * Integer", &mut |(mut a, b, c)| {
                a -= b * c
            }),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul_assign(Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.sub_mul_assign(Integer, &Integer)",
                &mut |(mut a, b, c)| a.sub_mul_assign(b, &c),
            ),
            ("Integer -= Integer * &Integer", &mut |(mut a, b, c)| {
                a -= b * &c
            }),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul_assign(&Integer, Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.sub_mul_assign(&Integer, Integer)",
                &mut |(mut a, b, c)| a.sub_mul_assign(&b, c),
            ),
            ("Integer -= &Integer * Integer", &mut |(mut a, b, c)| {
                a -= &b * c
            }),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul_assign(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.sub_mul_assign(&Integer, &Integer)",
                &mut |(mut a, b, c)| a.sub_mul_assign(&b, &c),
            ),
            ("Integer -= &Integer * &Integer", &mut |(mut a, b, c)| {
                a -= &b * &c
            }),
        ],
    );
}
