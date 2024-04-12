// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_vec_min_len_bucketer, triple_1_2_vec_min_len_bucketer,
    triple_2_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_31, unsigned_vec_pair_gen_var_6, unsigned_vec_triple_gen_var_31,
    unsigned_vec_triple_gen_var_40, unsigned_vec_unsigned_pair_gen,
    unsigned_vec_unsigned_pair_gen_var_1, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::sub::{
    limbs_slice_sub_in_place_right, limbs_sub, limbs_sub_greater_in_place_left,
    limbs_sub_greater_to_out, limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_in_place_with_overlap, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap, limbs_vec_sub_in_place_right,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_natural_max_bit_bucketer, pair_natural_max_bit_bucketer,
    triple_3_pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_pair_gen_var_10, natural_pair_gen_var_10_nrm, natural_pair_gen_var_10_rm,
};
use malachite_nz::test_util::natural::arithmetic::sub::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_sub_limb);
    register_demo!(runner, demo_limbs_sub_limb_to_out);
    register_demo!(runner, demo_limbs_sub_limb_in_place);
    register_demo!(runner, demo_limbs_sub);
    register_demo!(runner, demo_limbs_sub_same_length_to_out);
    register_demo!(runner, demo_limbs_sub_greater_to_out);
    register_demo!(runner, demo_limbs_sub_same_length_in_place_left);
    register_demo!(runner, demo_limbs_sub_greater_in_place_left);
    register_demo!(runner, demo_limbs_sub_same_length_in_place_right);
    register_demo!(runner, demo_limbs_slice_sub_in_place_right);
    register_demo!(runner, demo_limbs_vec_sub_in_place_right);
    register_demo!(runner, demo_limbs_sub_same_length_in_place_with_overlap);
    register_demo!(runner, demo_limbs_sub_same_length_to_out_with_overlap);
    register_demo!(runner, demo_natural_sub_assign);
    register_demo!(runner, demo_natural_sub_assign_ref);
    register_demo!(runner, demo_natural_sub);
    register_demo!(runner, demo_natural_sub_val_ref);
    register_demo!(runner, demo_natural_sub_ref_val);
    register_demo!(runner, demo_natural_sub_ref_ref);

    register_bench!(runner, benchmark_limbs_sub_limb);
    register_bench!(runner, benchmark_limbs_sub_limb_to_out);
    register_bench!(runner, benchmark_limbs_sub_limb_in_place);
    register_bench!(runner, benchmark_limbs_sub);
    register_bench!(runner, benchmark_limbs_sub_same_length_to_out);
    register_bench!(runner, benchmark_limbs_sub_greater_to_out);
    register_bench!(runner, benchmark_limbs_sub_same_length_in_place_left);
    register_bench!(runner, benchmark_limbs_sub_greater_in_place_left);
    register_bench!(runner, benchmark_limbs_sub_same_length_in_place_right);
    register_bench!(runner, benchmark_limbs_slice_sub_in_place_right);
    register_bench!(runner, benchmark_limbs_vec_sub_in_place_right);
    register_bench!(
        runner,
        benchmark_limbs_sub_same_length_in_place_with_overlap_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_sub_same_length_to_out_with_overlap_algorithms
    );
    register_bench!(runner, benchmark_natural_sub_assign_library_comparison);
    register_bench!(runner, benchmark_natural_sub_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_sub_library_comparison);
    register_bench!(runner, benchmark_natural_sub_evaluation_strategy);
}

fn demo_limbs_sub_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen().get(gm, config).take(limit) {
        println!(
            "limbs_sub_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_sub_limb(&xs, y)
        );
    }
}

fn demo_limbs_sub_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        let borrow = limbs_sub_limb_to_out(&mut out, &xs, y);
        println!(
            "out := {out_old:?}; \
            limbs_sub_limb_to_out(&mut out, {xs:?}, {y}) = {borrow}; out = {out:?}",
        );
    }
}

fn demo_limbs_sub_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let borrow = limbs_sub_limb_in_place(&mut xs, y);
        println!(
            "xs := {xs_old:?}; \
            limbs_sub_limb_in_place(&mut xs, {y}) = {borrow}; xs = {xs:?}"
        );
    }
}

fn demo_limbs_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_31().get(gm, config).take(limit) {
        println!("limbs_sub({:?}, {:?}) = {:?}", xs, ys, limbs_sub(&xs, &ys));
    }
}

fn demo_limbs_sub_same_length_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_31().get(gm, config).take(limit) {
        let out_old = out.clone();
        let borrow = limbs_sub_same_length_to_out(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_sub_same_length_to_out(&mut out, {xs:?}, {ys:?}) = {borrow}; out = {out:?}",
        );
    }
}

fn demo_limbs_sub_greater_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_40().get(gm, config).take(limit) {
        let out_old = out.clone();
        let borrow = limbs_sub_greater_to_out(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_sub_greater_to_out(&mut out, {xs:?}, {ys:?}) = {borrow}; out = {out:?}",
        );
    }
}

fn demo_limbs_sub_same_length_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_6().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_in_place_left(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_sub_same_length_in_place_left(&mut xs, {ys:?}) = {borrow}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_sub_greater_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_31().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let borrow = limbs_sub_greater_in_place_left(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_sub_greater_in_place_left(&mut xs, {ys:?}) = {borrow}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_sub_same_length_in_place_right(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys) in unsigned_vec_pair_gen_var_6().get(gm, config).take(limit) {
        let ys_old = ys.clone();
        let borrow = limbs_sub_same_length_in_place_right(&xs, &mut ys);
        println!(
            "ys := {ys_old:?}; \
            limbs_sub_same_length_in_place_right({xs:?}, &mut ys) = {borrow}; ys = {ys:?}",
        );
    }
}

fn demo_limbs_slice_sub_in_place_right(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys, len) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let ys_old = ys.clone();
        let borrow = limbs_slice_sub_in_place_right(&xs, &mut ys, len);
        println!(
            "ys := {ys_old:?}; \
            limbs_slice_sub_in_place_right({xs:?}, &mut ys, {len}) = {borrow}; ys = {ys:?}",
        );
    }
}

fn demo_limbs_vec_sub_in_place_right(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys) in unsigned_vec_pair_gen_var_31().get(gm, config).take(limit) {
        let ys_old = ys.clone();
        let borrow = limbs_vec_sub_in_place_right(&xs, &mut ys);
        println!(
            "ys := {ys_old:?}; \
            limbs_vec_sub_in_place_right({xs:?}, &mut ys) = {borrow}; ys = {ys:?}",
        );
    }
}

fn demo_limbs_sub_same_length_in_place_with_overlap(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, right_start) in unsigned_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_in_place_with_overlap(&mut xs, right_start);
        println!(
            "xs := {xs_old:?}; \
            limbs_sub_same_length_in_place_with_overlap(&mut xs, {right_start}) = {borrow}; \
            xs = {xs:?}",
        );
    }
}

fn demo_limbs_sub_same_length_to_out_with_overlap(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_31().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_to_out_with_overlap(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_sub_same_length_to_out_with_overlap(&mut xs, {ys:?}) = {borrow}; xs = {xs:?}",
        );
    }
}

fn demo_natural_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        x -= y.clone();
        println!("x := {x_old}; x -= {y}; x = {x}");
    }
}

fn demo_natural_sub_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {x_old}; x -= &{y}; x = {x}");
    }
}

fn demo_natural_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_natural_sub_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_natural_sub_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_10().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_natural_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_10().get(gm, config).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn benchmark_limbs_sub_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sub_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| no_out!(limbs_sub_limb(&xs, y)))],
    );
}

fn benchmark_limbs_sub_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sub_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            no_out!(limbs_sub_limb_to_out(&mut out, &xs, y))
        })],
    );
}

fn benchmark_limbs_sub_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            no_out!(limbs_sub_limb_in_place(&mut xs, y))
        })],
    );
}

fn benchmark_limbs_sub(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_sub(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, ys)| no_out!(limbs_sub(&xs, &ys)))],
    );
}

fn benchmark_limbs_sub_same_length_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            no_out!(limbs_sub_same_length_to_out(&mut out, &xs, &ys))
        })],
    );
}

fn benchmark_limbs_sub_greater_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_greater_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_40().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            no_out!(limbs_sub_greater_to_out(&mut out, &xs, &ys))
        })],
    );
}

fn benchmark_limbs_sub_same_length_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_same_length_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, ys)| {
            no_out!(limbs_sub_same_length_in_place_left(&mut xs, &ys))
        })],
    );
}

fn benchmark_limbs_sub_greater_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_greater_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_min_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, ys)| {
            no_out!(limbs_sub_greater_in_place_left(&mut xs, &ys))
        })],
    );
}

fn benchmark_limbs_sub_same_length_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_same_length_in_place_right(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, mut ys)| {
            no_out!(limbs_sub_same_length_in_place_right(&xs, &mut ys))
        })],
    );
}

fn benchmark_limbs_slice_sub_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_sub_in_place_right(&[Limb], &mut [Limb], usize)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_min_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, mut ys, len)| {
            no_out!(limbs_slice_sub_in_place_right(&xs, &mut ys, len))
        })],
    );
}

fn benchmark_limbs_vec_sub_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_sub_in_place_right(&[Limb], &mut Vec<Limb>)",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_min_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, mut ys)| {
            no_out!(limbs_vec_sub_in_place_right(&xs, &mut ys))
        })],
    );
}

fn benchmark_limbs_sub_same_length_in_place_with_overlap_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_same_length_in_place_with_overlap(&mut [Limb], usize)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("standard", &mut |(mut xs, right_start)| {
                no_out!(limbs_sub_same_length_in_place_with_overlap(
                    &mut xs,
                    right_start
                ))
            }),
            ("naive", &mut |(mut xs, right_start)| {
                no_out!(limbs_sub_same_length_in_place_with_overlap_naive(
                    &mut xs,
                    right_start
                ))
            }),
        ],
    );
}

fn benchmark_limbs_sub_same_length_to_out_with_overlap_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_sub_same_length_to_out_with_overlap(&mut [Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("standard", &mut |(mut xs, ys)| {
                no_out!(limbs_sub_same_length_to_out_with_overlap(&mut xs, &ys))
            }),
            ("naive", &mut |(mut xs, ys)| {
                no_out!(limbs_sub_same_length_to_out_with_overlap_naive(
                    &mut xs, &ys
                ))
            }),
        ],
    );
}

fn benchmark_natural_sub_assign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural -= Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_10_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(_, (mut x, y))| x -= y), ("rug", &mut |((mut x, y), _)| x -= y)],
    );
}

fn benchmark_natural_sub_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural -= Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural -= Natural", &mut |(mut x, y)| x -= y),
            ("Natural -= &Natural", &mut |(mut x, y)| x -= &y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_sub_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural - Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_10_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x - y)),
            ("num", &mut |((x, y), _, _)| no_out!(x - y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x - y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural - Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural - Natural", &mut |(x, y)| no_out!(x - y)),
            ("Natural - &Natural", &mut |(x, y)| no_out!(x - &y)),
            ("&Natural - Natural", &mut |(x, y)| no_out!(&x - y)),
            ("&Natural - &Natural", &mut |(x, y)| no_out!(&x - &y)),
        ],
    );
}
