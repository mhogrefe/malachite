// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::test_util::bench::bucketers::{
    triple_1_2_vec_max_len_bucketer, triple_vec_max_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_triple_gen_var_41, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::add_mul::{
    limbs_add_mul, limbs_add_mul_in_place_left, limbs_add_mul_limb,
    limbs_slice_add_mul_limb_same_length_in_place_left,
    limbs_slice_add_mul_limb_same_length_in_place_right, limbs_vec_add_mul_limb_in_place_either,
    limbs_vec_add_mul_limb_in_place_left, limbs_vec_add_mul_limb_in_place_right,
};
use malachite_nz::test_util::bench::bucketers::triple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_triple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_add_mul_limb);
    register_demo!(
        runner,
        demo_limbs_slice_add_mul_limb_same_length_in_place_left
    );
    register_demo!(
        runner,
        demo_limbs_slice_add_mul_limb_same_length_in_place_right
    );
    register_demo!(runner, demo_limbs_vec_add_mul_limb_in_place_left);
    register_demo!(runner, demo_limbs_vec_add_mul_limb_in_place_right);
    register_demo!(runner, demo_limbs_vec_add_mul_limb_in_place_either);
    register_demo!(runner, demo_limbs_add_mul);
    register_demo!(runner, demo_limbs_add_mul_in_place_left);
    register_demo!(runner, demo_natural_add_mul_assign);
    register_demo!(runner, demo_natural_add_mul_assign_val_ref);
    register_demo!(runner, demo_natural_add_mul_assign_ref_val);
    register_demo!(runner, demo_natural_add_mul_assign_ref_ref);
    register_demo!(runner, demo_natural_add_mul);
    register_demo!(runner, demo_natural_add_mul_val_val_ref);
    register_demo!(runner, demo_natural_add_mul_val_ref_val);
    register_demo!(runner, demo_natural_add_mul_val_ref_ref);
    register_demo!(runner, demo_natural_add_mul_ref_ref_ref);

    register_bench!(runner, benchmark_limbs_add_mul_limb);
    register_bench!(
        runner,
        benchmark_limbs_slice_add_mul_limb_same_length_in_place_left
    );
    register_bench!(
        runner,
        benchmark_limbs_slice_add_mul_limb_same_length_in_place_right
    );
    register_bench!(runner, benchmark_limbs_vec_add_mul_limb_in_place_left);
    register_bench!(runner, benchmark_limbs_vec_add_mul_limb_in_place_right);
    register_bench!(runner, benchmark_limbs_vec_add_mul_limb_in_place_either);
    register_bench!(runner, benchmark_limbs_add_mul);
    register_bench!(runner, benchmark_limbs_add_mul_in_place_left);
    register_bench!(runner, benchmark_natural_add_mul_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_add_mul_assign_algorithms);
    register_bench!(runner, benchmark_natural_add_mul_assign_val_ref_algorithms);
    register_bench!(runner, benchmark_natural_add_mul_assign_ref_val_algorithms);
    register_bench!(runner, benchmark_natural_add_mul_assign_ref_ref_algorithms);
    register_bench!(runner, benchmark_natural_add_mul_evaluation_stategy);
    register_bench!(runner, benchmark_natural_add_mul_algorithms);
    register_bench!(runner, benchmark_natural_add_mul_val_val_ref_algorithms);
    register_bench!(runner, benchmark_natural_add_mul_val_ref_val_algorithms);
    register_bench!(runner, benchmark_natural_add_mul_val_ref_ref_algorithms);
    register_bench!(runner, benchmark_natural_add_mul_ref_ref_ref_algorithms);
}

fn demo_limbs_add_mul_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_add_mul_limb({:?}, {:?}, {}) = {:?}",
            a,
            b,
            c,
            limbs_add_mul_limb(&a, &b, c),
        );
    }
}

fn demo_limbs_slice_add_mul_limb_same_length_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let carry = limbs_slice_add_mul_limb_same_length_in_place_left(&mut a, &b, c);
        println!(
            "a := {a_old:?}; \
            limbs_slice_add_mul_limb_same_length_in_place_left(&mut a, {b:?}, {c}) = {carry}; \
             a = {a:?}",
        );
    }
}

fn demo_limbs_slice_add_mul_limb_same_length_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, mut b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12()
        .get(gm, config)
        .take(limit)
    {
        let b_old = b.clone();
        let carry = limbs_slice_add_mul_limb_same_length_in_place_right(&a, &mut b, c);
        println!(
            "b := {b_old:?}; \
            limbs_slice_add_mul_limb_same_length_in_place_right({a:?}, &mut b, {c}) \
             = {carry}; b = {b:?}",
        );
    }
}

fn demo_limbs_vec_add_mul_limb_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        limbs_vec_add_mul_limb_in_place_left(&mut a, &b, c);
        println!(
            "a := {a_old:?}; limbs_vec_add_mul_limb_in_place_left(&mut a, {b:?}, {c}); a = {a:?}",
        );
    }
}

fn demo_limbs_vec_add_mul_limb_in_place_right(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, mut b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let b_old = b.clone();
        limbs_vec_add_mul_limb_in_place_right(&a, &mut b, c);
        println!(
            "b := {b_old:?}; limbs_vec_add_mul_limb_in_place_right({a:?}, &mut b, {c}); b = {b:?}",
        );
    }
}

fn demo_limbs_vec_add_mul_limb_in_place_either(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, mut b, c) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        limbs_vec_add_mul_limb_in_place_either(&mut a, &mut b, c);
        println!(
            "a := {a_old:?}; \
            b := {b_old:?}; limbs_vec_add_mul_limb_in_place_either(&mut a, &mut b, {c}); \
             a = {a:?}; b = {b:?}",
        );
    }
}

fn demo_limbs_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in unsigned_vec_triple_gen_var_41().get(gm, config).take(limit) {
        println!(
            "limbs_add_mul({:?}, {:?}, {:?}) = {:?}",
            a,
            b,
            c,
            limbs_add_mul(&a, &b, &c),
        );
    }
}

fn demo_limbs_add_mul_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in unsigned_vec_triple_gen_var_41().get(gm, config).take(limit) {
        let a_old = a.clone();
        limbs_add_mul_in_place_left(&mut a, &b, &c);
        println!("a := {a_old:?}; limbs_add_mul_in_place_left(&mut a, {b:?}, {c:?}); a = {a:?}");
    }
}

fn demo_natural_add_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.add_mul_assign(b, c);
        println!("a := {a_old}; x.add_mul_assign({b_old}, {c_old}); x = {a}");
    }
}

fn demo_natural_add_mul_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, &c);
        println!("a := {a_old}; x.add_mul_assign({b_old}, &{c}); x = {a}");
    }
}

fn demo_natural_add_mul_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.add_mul_assign(&b, c);
        println!("a := {a_old}; x.add_mul_assign(&{b}, {c_old}); x = {a}");
    }
}

fn demo_natural_add_mul_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.add_mul_assign(&b, &c);
        println!("a := {a_old}; x.add_mul_assign(&{b}, &{c}); x = {a}");
    }
}

fn demo_natural_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.add_mul({}, {}) = {}",
            a_old,
            b_old,
            c_old,
            a.add_mul(b, c)
        );
    }
}

fn demo_natural_add_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.add_mul({}, &{}) = {}",
            a_old,
            b_old,
            c,
            a.add_mul(b, &c)
        );
    }
}

fn demo_natural_add_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.add_mul(&{}, {}) = {}",
            a_old,
            b,
            c_old,
            a.add_mul(&b, c)
        );
    }
}

fn demo_natural_add_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, &{}) = {}", a_old, b, c, a.add_mul(&b, &c));
    }
}

fn demo_natural_add_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).add_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).add_mul(&b, &c)
        );
    }
}

fn benchmark_limbs_add_mul_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_mul_limb(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("a", "b"),
        &mut [("Malachite", &mut |(a, b, c)| {
            no_out!(limbs_add_mul_limb(&a, &b, c))
        })],
    );
}

fn benchmark_limbs_slice_add_mul_limb_same_length_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_mul_limb_same_length_in_place_left(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("a", "b"),
        &mut [("Malachite", &mut |(mut a, b, c)| {
            no_out!(limbs_slice_add_mul_limb_same_length_in_place_left(
                &mut a, &b, c
            ))
        })],
    );
}

fn benchmark_limbs_slice_add_mul_limb_same_length_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_mul_limb_same_length_in_place_left(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("a", "b"),
        &mut [("Malachite", &mut |(a, mut b, c)| {
            no_out!(limbs_slice_add_mul_limb_same_length_in_place_right(
                &a, &mut b, c
            ))
        })],
    );
}

fn benchmark_limbs_vec_add_mul_limb_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_mul_limb_in_place_left(&mut Vec<Limb>, &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("a", "b"),
        &mut [("Malachite", &mut |(mut a, b, c)| {
            limbs_vec_add_mul_limb_in_place_left(&mut a, &b, c)
        })],
    );
}

fn benchmark_limbs_vec_add_mul_limb_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_mul_limb_in_place_left(&mut Vec<Limb>, &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("a", "b"),
        &mut [("Malachite", &mut |(a, mut b, c)| {
            limbs_vec_add_mul_limb_in_place_right(&a, &mut b, c)
        })],
    );
}

fn benchmark_limbs_vec_add_mul_limb_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_mul_limb_in_place_either(&mut Vec<Limb>, &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("a", "b"),
        &mut [("Malachite", &mut |(mut a, b, c)| {
            limbs_vec_add_mul_limb_in_place_left(&mut a, &b, c)
        })],
    );
}

fn benchmark_limbs_add_mul(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_mul(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_41().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_vec_max_len_bucketer("a", "b", "c"),
        &mut [("Malachite", &mut |(a, b, c)| {
            no_out!(limbs_add_mul(&a, &b, &c))
        })],
    );
}

fn benchmark_limbs_add_mul_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_add_mul_in_place_left(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_41().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_vec_max_len_bucketer("a", "b", "c"),
        &mut [("Malachite", &mut |(mut a, b, c)| {
            no_out!(limbs_add_mul_in_place_left(&mut a, &b, &c))
        })],
    );
}

fn benchmark_natural_add_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.add_mul_assign(Natural, Natural)",
                &mut |(mut a, b, c)| a.add_mul_assign(b, c),
            ),
            (
                "Natural.add_mul_assign(Natural, &Natural)",
                &mut |(mut a, b, c)| a.add_mul_assign(b, &c),
            ),
            (
                "Natural.add_mul_assign(&Natural, Natural)",
                &mut |(mut a, b, c)| a.add_mul_assign(&b, c),
            ),
            (
                "Natural.add_mul_assign(&Natural, &Natural)",
                &mut |(mut a, b, c)| a.add_mul_assign(&b, &c),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.add_mul_assign(Natural, Natural)",
                &mut |(mut a, b, c)| a.add_mul_assign(b, c),
            ),
            ("Natural += Natural * Natural", &mut |(mut a, b, c)| {
                a += b * c
            }),
        ],
    );
}

fn benchmark_natural_add_mul_assign_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul_assign(Natural, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.add_mul_assign(Natural, &Natural)",
                &mut |(mut a, b, c)| a.add_mul_assign(b, &c),
            ),
            ("Natural += Natural * &Natural", &mut |(mut a, b, c)| {
                a += b * &c
            }),
        ],
    );
}

fn benchmark_natural_add_mul_assign_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul_assign(&Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.add_mul_assign(&Natural, Natural)",
                &mut |(mut a, b, c)| a.add_mul_assign(&b, c),
            ),
            ("Natural += &Natural * Natural", &mut |(mut a, b, c)| {
                a += &b * c
            }),
        ],
    );
}

fn benchmark_natural_add_mul_assign_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul_assign(&Natural, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "Natural.add_mul_assign(&Natural, &Natural)",
                &mut |(mut a, b, c)| a.add_mul_assign(&b, &c),
            ),
            ("Natural += &Natural * &Natural", &mut |(mut a, b, c)| {
                a += &b * &c
            }),
        ],
    );
}

fn benchmark_natural_add_mul_evaluation_stategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.add_mul(Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.add_mul(b, c))
            }),
            ("Natural.add_mul(Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.add_mul(b, &c))
            }),
            ("Natural.add_mul(&Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.add_mul(&b, c))
            }),
            ("Natural.add_mul(&Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.add_mul(&b, &c))
            }),
            (
                "(&Natural).add_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!((&a).add_mul(&b, &c)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_add_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.add_mul(Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.add_mul(b, c))
            }),
            ("Natural + Natural * Natural", &mut |(a, b, c)| {
                no_out!(a + b * c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_add_mul_val_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul(Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.add_mul(Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.add_mul(b, &c))
            }),
            ("Natural + Natural * &Natural", &mut |(a, b, c)| {
                no_out!(a + b * &c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_add_mul_val_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul(&Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.add_mul(&Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.add_mul(&b, c))
            }),
            ("Natural + &Natural * Natural", &mut |(a, b, c)| {
                no_out!(a + &b * c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_add_mul_val_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            ("Natural.add_mul(&Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.add_mul(&b, &c))
            }),
            ("Natural + &Natural * &Natural", &mut |(a, b, c)| {
                no_out!(a + &b * &c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_add_mul_ref_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Natural).add_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("a", "b", "c"),
        &mut [
            (
                "(&Natural).add_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!((&a).add_mul(&b, &c)),
            ),
            ("(&Natural) + &Natural * &Natural", &mut |(a, b, c)| {
                no_out!((&a) + &b * &c)
            }),
        ],
    );
}
