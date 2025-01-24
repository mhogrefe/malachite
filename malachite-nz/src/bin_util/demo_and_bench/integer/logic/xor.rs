// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_vec_max_len_bucketer, triple_2_3_vec_max_len_bucketer,
    triple_2_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_8, unsigned_vec_triple_gen_var_34,
    unsigned_vec_unsigned_pair_gen_var_15, unsigned_vec_unsigned_pair_gen_var_18,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::logic::xor::{
    limbs_neg_xor_limb, limbs_neg_xor_limb_neg, limbs_neg_xor_limb_neg_in_place,
    limbs_neg_xor_limb_neg_to_out, limbs_neg_xor_limb_to_out, limbs_pos_xor_limb_neg,
    limbs_pos_xor_limb_neg_to_out, limbs_slice_neg_xor_limb_in_place,
    limbs_slice_pos_xor_limb_neg_in_place, limbs_vec_neg_xor_limb_in_place,
    limbs_vec_pos_xor_limb_neg_in_place, limbs_xor_neg_neg, limbs_xor_neg_neg_in_place_either,
    limbs_xor_neg_neg_in_place_left, limbs_xor_neg_neg_to_out, limbs_xor_pos_neg,
    limbs_xor_pos_neg_in_place_either, limbs_xor_pos_neg_in_place_left,
    limbs_xor_pos_neg_in_place_right, limbs_xor_pos_neg_to_out,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_integer_max_bit_bucketer, pair_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen, integer_pair_gen_rm};
use malachite_nz::test_util::integer::logic::xor::{integer_xor_alt_1, integer_xor_alt_2};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_neg_xor_limb);
    register_demo!(runner, demo_limbs_neg_xor_limb_to_out);
    register_demo!(runner, demo_limbs_slice_neg_xor_limb_in_place);
    register_demo!(runner, demo_limbs_vec_neg_xor_limb_in_place);
    register_demo!(runner, demo_limbs_pos_xor_limb_neg);
    register_demo!(runner, demo_limbs_pos_xor_limb_neg_to_out);
    register_demo!(runner, demo_limbs_slice_pos_xor_limb_neg_in_place);
    register_demo!(runner, demo_limbs_vec_pos_xor_limb_neg_in_place);
    register_demo!(runner, demo_limbs_neg_xor_limb_neg);
    register_demo!(runner, demo_limbs_neg_xor_limb_neg_to_out);
    register_demo!(runner, demo_limbs_neg_xor_limb_neg_in_place);
    register_demo!(runner, demo_limbs_xor_pos_neg);
    register_demo!(runner, demo_limbs_xor_pos_neg_to_out);
    register_demo!(runner, demo_limbs_xor_pos_neg_in_place_left);
    register_demo!(runner, demo_limbs_xor_pos_neg_in_place_right);
    register_demo!(runner, demo_limbs_xor_pos_neg_in_place_either);
    register_demo!(runner, demo_limbs_xor_neg_neg);
    register_demo!(runner, demo_limbs_xor_neg_neg_to_out);
    register_demo!(runner, demo_limbs_xor_neg_neg_in_place_left);
    register_demo!(runner, demo_limbs_xor_neg_neg_in_place_either);
    register_demo!(runner, demo_integer_xor_assign);
    register_demo!(runner, demo_integer_xor_assign_ref);
    register_demo!(runner, demo_integer_xor);
    register_demo!(runner, demo_integer_xor_val_ref);
    register_demo!(runner, demo_integer_xor_ref_val);
    register_demo!(runner, demo_integer_xor_ref_ref);

    register_bench!(runner, benchmark_limbs_neg_xor_limb);
    register_bench!(runner, benchmark_limbs_neg_xor_limb_to_out);
    register_bench!(runner, benchmark_limbs_slice_neg_xor_limb_in_place);
    register_bench!(runner, benchmark_limbs_vec_neg_xor_limb_in_place);
    register_bench!(runner, benchmark_limbs_pos_xor_limb_neg);
    register_bench!(runner, benchmark_limbs_pos_xor_limb_neg_to_out);
    register_bench!(runner, benchmark_limbs_slice_pos_xor_limb_neg_in_place);
    register_bench!(runner, benchmark_limbs_vec_pos_xor_limb_neg_in_place);
    register_bench!(runner, benchmark_limbs_neg_xor_limb_neg);
    register_bench!(runner, benchmark_limbs_neg_xor_limb_neg_to_out);
    register_bench!(runner, benchmark_limbs_neg_xor_limb_neg_in_place);
    register_bench!(runner, benchmark_limbs_xor_pos_neg);
    register_bench!(runner, benchmark_limbs_xor_pos_neg_to_out);
    register_bench!(runner, benchmark_limbs_xor_pos_neg_in_place_left);
    register_bench!(runner, benchmark_limbs_xor_pos_neg_in_place_right);
    register_bench!(runner, benchmark_limbs_xor_pos_neg_in_place_either);
    register_bench!(runner, benchmark_limbs_xor_neg_neg);
    register_bench!(runner, benchmark_limbs_xor_neg_neg_to_out);
    register_bench!(runner, benchmark_limbs_xor_neg_neg_in_place_left);
    register_bench!(runner, benchmark_limbs_xor_neg_neg_in_place_either);
    register_bench!(runner, benchmark_integer_xor_assign_library_comparison);
    register_bench!(runner, benchmark_integer_xor_assign_evaluation_strategy);
    register_bench!(runner, benchmark_integer_xor_library_comparison);
    register_bench!(runner, benchmark_integer_xor_algorithms);
    register_bench!(runner, benchmark_integer_xor_evaluation_strategy);
}

fn demo_limbs_neg_xor_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_neg_xor_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_neg_xor_limb(&xs, y)
        );
    }
}

fn demo_limbs_neg_xor_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        let carry = limbs_neg_xor_limb_to_out(&mut out, &xs, y);
        println!(
            "out := {out_old:?}; \
            limbs_neg_xor_limb_to_out(&mut out, {xs:?}, {y}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_slice_neg_xor_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_slice_neg_xor_limb_in_place(&mut xs, y);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_neg_xor_limb_in_place(&mut xs, {y}) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_neg_xor_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_neg_xor_limb_in_place(&mut xs, y);
        println!("xs := {xs_old:?}; limbs_vec_neg_xor_limb_in_place(&mut xs, {y}); xs = {xs:?}");
    }
}

fn demo_limbs_pos_xor_limb_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_pos_xor_limb_neg({:?}, {}) = {:?}",
            xs,
            y,
            limbs_pos_xor_limb_neg(&xs, y)
        );
    }
}

fn demo_limbs_pos_xor_limb_neg_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        let carry = limbs_pos_xor_limb_neg_to_out(&mut out, &xs, y);
        println!(
            "out := {out_old:?}; \
            limbs_pos_xor_limb_neg_to_out(&mut out, {xs:?}, {y}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_slice_pos_xor_limb_neg_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_slice_pos_xor_limb_neg_in_place(&mut xs, y);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_pos_xor_limb_neg_in_place(&mut xs, {y}) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_pos_xor_limb_neg_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_pos_xor_limb_neg_in_place(&mut xs, y);
        println!(
            "xs := {xs_old:?}; limbs_vec_pos_xor_limb_neg_in_place(&mut xs, {y}); xs = {xs:?}",
        );
    }
}

fn demo_limbs_neg_xor_limb_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_neg_xor_limb_neg({:?}, {}) = {:?}",
            xs,
            y,
            limbs_neg_xor_limb_neg(&xs, y)
        );
    }
}

fn demo_limbs_neg_xor_limb_neg_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        limbs_neg_xor_limb_neg_to_out(&mut out, &xs, y);
        println!(
            "out := {out_old:?}; \
            limbs_neg_xor_limb_neg_to_out(&mut out, {xs:?}, {y}) = out = {out:?}",
        );
    }
}

fn demo_limbs_neg_xor_limb_neg_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_neg_xor_limb_neg_in_place(&mut xs, y);
        println!("xs := {xs_old:?}; limbs_neg_xor_limb_neg_in_place(&mut xs, {y}); xs = {xs:?}");
    }
}

fn demo_limbs_xor_pos_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, config).take(limit) {
        println!(
            "limbs_xor_pos_neg({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_xor_pos_neg(&xs, &ys)
        );
    }
}

fn demo_limbs_xor_pos_neg_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_34().get(gm, config).take(limit) {
        let out_old = out.clone();
        let carry = limbs_xor_pos_neg_to_out(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_xor_pos_neg_to_out(&mut out, {xs:?}, {ys:?}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_xor_pos_neg_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_xor_pos_neg_in_place_left(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_xor_pos_neg_in_place_left(&mut xs, {ys:?}); xs = {xs:?}"
        );
    }
}

fn demo_limbs_xor_pos_neg_in_place_right(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys) in unsigned_vec_pair_gen_var_8().get(gm, config).take(limit) {
        let ys_old = ys.clone();
        limbs_xor_pos_neg_in_place_right(&xs, &mut ys);
        println!(
            "ys := {xs:?}; limbs_xor_pos_neg_in_place_right({ys_old:?}, &mut ys); ys = {ys:?}",
        );
    }
}

fn demo_limbs_xor_pos_neg_in_place_either(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen_var_8().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let b = limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {xs_old:?}; \
            ys := {ys_old:?}; limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys) = {b}; \
            xs = {xs:?}; ys = {ys:?}",
        );
    }
}

fn demo_limbs_xor_neg_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, config).take(limit) {
        println!(
            "limbs_xor_neg_neg({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_xor_neg_neg(&xs, &ys)
        );
    }
}

fn demo_limbs_xor_neg_neg_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_34().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_xor_neg_neg_to_out(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_xor_neg_neg_to_out(&mut out, {xs:?}, {ys:?}); out = {out:?}",
        );
    }
}

fn demo_limbs_xor_neg_neg_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_xor_neg_neg_in_place_left(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_xor_neg_neg_in_place_left(&mut xs, {ys:?}); xs = {xs:?}"
        );
    }
}

fn demo_limbs_xor_neg_neg_in_place_either(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen_var_8().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let b = limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {xs_old:?}; \
            ys := {ys_old:?}; limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys) = {b}; \
            xs = {xs:?}; ys = {ys:?}",
        );
    }
}

fn demo_integer_xor_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x ^= y.clone();
        println!("x := {x_old}; x ^= {y}; x = {x}");
    }
}

fn demo_integer_xor_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x ^= &y;
        println!("x := {x_old}; x ^= &{y}; x = {x}");
    }
}

fn demo_integer_xor(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} ^ {} = {}", x_old, y_old, x ^ y);
    }
}

fn demo_integer_xor_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} ^ &{} = {}", x_old, y, x ^ &y);
    }
}

fn demo_integer_xor_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} ^ {} = {}", x, y_old, &x ^ y);
    }
}

fn demo_integer_xor_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        println!("&{} ^ &{} = {}", x, y, &x ^ &y);
    }
}

fn benchmark_limbs_neg_xor_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_neg_xor_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_neg_xor_limb(&xs, y))
        })],
    );
}

fn benchmark_limbs_neg_xor_limb_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_xor_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            no_out!(limbs_neg_xor_limb_to_out(&mut out, &xs, y))
        })],
    );
}

fn benchmark_limbs_slice_neg_xor_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_slice_xor_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            no_out!(limbs_slice_neg_xor_limb_in_place(&mut xs, y))
        })],
    );
}

fn benchmark_limbs_vec_neg_xor_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_vec_xor_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            limbs_vec_neg_xor_limb_in_place(&mut xs, y)
        })],
    );
}

fn benchmark_limbs_pos_xor_limb_neg(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_pos_xor_limb_neg(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_pos_xor_limb_neg(&xs, y))
        })],
    );
}

fn benchmark_limbs_pos_xor_limb_neg_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_pos_xor_limb_neg_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            no_out!(limbs_pos_xor_limb_neg_to_out(&mut out, &xs, y))
        })],
    );
}

fn benchmark_limbs_slice_pos_xor_limb_neg_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_pos_xor_limb_neg_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            no_out!(limbs_slice_pos_xor_limb_neg_in_place(&mut xs, y))
        })],
    );
}

fn benchmark_limbs_vec_pos_xor_limb_neg_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_pos_xor_limb_neg_in_place(&Vec[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            limbs_vec_pos_xor_limb_neg_in_place(&mut xs, y)
        })],
    );
}

fn benchmark_limbs_neg_xor_limb_neg(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_xor_limb_neg(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_neg_xor_limb_neg(&xs, y))
        })],
    );
}

fn benchmark_limbs_neg_xor_limb_neg_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_xor_limb_neg_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            no_out!(limbs_neg_xor_limb_neg_to_out(&mut out, &xs, y))
        })],
    );
}

fn benchmark_limbs_neg_xor_limb_neg_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_xor_limb_neg_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            limbs_neg_xor_limb_neg_in_place(&mut xs, y)
        })],
    );
}

fn benchmark_limbs_xor_pos_neg(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_pos_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref xs, ref ys)| {
            no_out!(limbs_xor_pos_neg(xs, ys))
        })],
    );
}

fn benchmark_limbs_xor_pos_neg_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_xor_pos_neg_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_34().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref mut out, ref xs, ref ys)| {
            no_out!(limbs_xor_pos_neg_to_out(out, xs, ys))
        })],
    );
}

fn benchmark_limbs_xor_pos_neg_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_xor_pos_neg_in_place_left(&mut Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref mut xs, ref ys)| {
            no_out!(limbs_xor_pos_neg_in_place_left(xs, ys))
        })],
    );
}

fn benchmark_limbs_xor_pos_neg_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_xor_pos_neg_in_place_right(&[Limb], &mut Vec<Limb>)",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref xs, ref mut ys)| {
            no_out!(limbs_xor_pos_neg_in_place_right(xs, ys))
        })],
    );
}

fn benchmark_limbs_xor_pos_neg_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_xor_pos_neg_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref mut xs, ref mut ys)| {
            no_out!(limbs_xor_pos_neg_in_place_either(xs, ys))
        })],
    );
}

fn benchmark_limbs_xor_neg_neg(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_xor_neg_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref xs, ref ys)| {
            no_out!(limbs_xor_neg_neg(xs, ys))
        })],
    );
}

fn benchmark_limbs_xor_neg_neg_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_xor_neg_neg_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_34().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref mut out, ref xs, ref ys)| {
            limbs_xor_neg_neg_to_out(out, xs, ys)
        })],
    );
}

fn benchmark_limbs_xor_neg_neg_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_xor_neg_neg_in_place_left(&mut Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref mut xs, ref ys)| {
            no_out!(limbs_xor_neg_neg_in_place_left(xs, ys))
        })],
    );
}

fn benchmark_limbs_xor_neg_neg_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_xor_neg_neg_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(ref mut xs, ref mut ys)| {
            no_out!(limbs_xor_neg_neg_in_place_either(xs, ys))
        })],
    );
}

fn benchmark_integer_xor_assign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer ^= Integer",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(_, (mut x, y))| x ^= y), ("rug", &mut |((mut x, y), _)| x ^= y)],
    );
}

fn benchmark_integer_xor_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer ^= Integer",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("xs", "ys"),
        &mut [
            ("Integer ^= Integer", &mut |(mut x, y)| no_out!(x ^= y)),
            ("Integer ^= &Integer", &mut |(mut x, y)| no_out!(x ^= &y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_xor_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer ^ Integer",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("xs", "ys"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x ^ y)),
            ("rug", &mut |((x, y), _)| no_out!(x ^ y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_xor_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer ^ Integer",
        BenchmarkType::Algorithms,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("xs", "ys"),
        &mut [
            ("default", &mut |(ref x, ref y)| no_out!(x ^ y)),
            ("using bits explicitly", &mut |(ref x, ref y)| {
                no_out!(integer_xor_alt_1(x, y))
            }),
            ("using limbs explicitly", &mut |(ref x, ref y)| {
                no_out!(integer_xor_alt_2(x, y))
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_xor_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer ^ Integer",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("xs", "ys"),
        &mut [
            ("Integer ^ Integer", &mut |(x, y)| no_out!(x ^ y)),
            ("Integer ^ &Integer", &mut |(x, y)| no_out!(x ^ &y)),
            ("&Integer ^ Integer", &mut |(x, y)| no_out!(&x ^ y)),
            ("&Integer ^ &Integer", &mut |(x, y)| no_out!(&x ^ &y)),
        ],
    );
}
