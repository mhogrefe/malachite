// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_vec_max_len_bucketer, triple_2_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen, unsigned_vec_pair_gen_var_1, unsigned_vec_pair_gen_var_6,
    unsigned_vec_triple_gen_var_31, unsigned_vec_triple_gen_var_32, unsigned_vec_triple_gen_var_40,
    unsigned_vec_unsigned_pair_gen, unsigned_vec_unsigned_pair_gen_var_15,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::add::{
    limbs_add, limbs_add_greater, limbs_add_greater_to_out, limbs_add_limb, limbs_add_limb_to_out,
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_add_to_out_aliased,
    limbs_slice_add_greater_in_place_left, limbs_slice_add_in_place_either,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
    limbs_vec_add_in_place_either, limbs_vec_add_in_place_left, limbs_vec_add_limb_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_natural_max_bit_bucketer, pair_natural_max_bit_bucketer,
    triple_3_pair_natural_max_bit_bucketer, triple_3_vec_natural_sum_bits_bucketer,
    vec_natural_sum_bits_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_pair_gen, natural_pair_gen_nrm, natural_pair_gen_rm, natural_vec_gen,
    natural_vec_gen_nrm,
};
use malachite_nz::test_util::natural::arithmetic::add::natural_sum_alt;
use num::BigUint;
use std::iter::Sum;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_add_limb);
    register_demo!(runner, demo_limbs_add_limb_to_out);
    register_demo!(runner, demo_limbs_slice_add_limb_in_place);
    register_demo!(runner, demo_limbs_vec_add_limb_in_place);
    register_demo!(runner, demo_limbs_add_greater);
    register_demo!(runner, demo_limbs_add);
    register_demo!(runner, demo_limbs_add_same_length_to_out);
    register_demo!(runner, demo_limbs_add_greater_to_out);
    register_demo!(runner, demo_limbs_add_to_out);
    register_demo!(runner, demo_limbs_add_to_out_aliased);
    register_demo!(runner, demo_limbs_slice_add_same_length_in_place_left);
    register_demo!(runner, demo_limbs_slice_add_greater_in_place_left);
    register_demo!(runner, demo_limbs_vec_add_in_place_left);
    register_demo!(runner, demo_limbs_slice_add_in_place_either);
    register_demo!(runner, demo_limbs_vec_add_in_place_either);
    register_demo!(runner, demo_natural_add_assign);
    register_demo!(runner, demo_natural_add_assign_ref);
    register_demo!(runner, demo_natural_add);
    register_demo!(runner, demo_natural_add_val_ref);
    register_demo!(runner, demo_natural_add_ref_val);
    register_demo!(runner, demo_natural_add_ref_ref);
    register_demo!(runner, demo_natural_sum);
    register_demo!(runner, demo_natural_ref_sum);

    register_bench!(runner, benchmark_limbs_add_limb);
    register_bench!(runner, benchmark_limbs_add_limb_to_out);
    register_bench!(runner, benchmark_limbs_slice_add_limb_in_place);
    register_bench!(runner, benchmark_limbs_vec_add_limb_in_place);
    register_bench!(runner, benchmark_limbs_add_greater);
    register_bench!(runner, benchmark_limbs_add);
    register_bench!(runner, benchmark_limbs_add_same_length_to_out);
    register_bench!(runner, benchmark_limbs_add_greater_to_out);
    register_bench!(runner, benchmark_limbs_add_to_out);
    register_bench!(runner, benchmark_limbs_add_to_out_aliased);
    register_bench!(runner, benchmark_limbs_slice_add_same_length_in_place_left);
    register_bench!(runner, benchmark_limbs_slice_add_greater_in_place_left);
    register_bench!(runner, benchmark_limbs_vec_add_in_place_left);
    register_bench!(runner, benchmark_limbs_slice_add_in_place_either);
    register_bench!(runner, benchmark_limbs_vec_add_in_place_either);
    register_bench!(runner, benchmark_natural_add_assign_library_comparison);
    register_bench!(runner, benchmark_natural_add_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_add_library_comparison);
    register_bench!(runner, benchmark_natural_add_evaluation_strategy);
    register_bench!(runner, benchmark_natural_sum_algorithms);
    register_bench!(runner, benchmark_natural_sum_library_comparison);
    register_bench!(runner, benchmark_natural_sum_evaluation_strategy);
}

fn demo_limbs_add_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen().get(gm, config).take(limit) {
        println!(
            "limbs_add_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_add_limb(&xs, y)
        );
    }
}

fn demo_limbs_add_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        let carry = limbs_add_limb_to_out(&mut out, &xs, y);
        println!(
            "out := {out_old:?}; \
            limbs_add_limb_to_out(&mut out, {xs:?}, {y}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_slice_add_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen::<Limb, Limb>()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_slice_add_limb_in_place(&mut xs, y);
        println!(
            "xs := {xs_old:?}; limbs_slice_add_limb_in_place(&mut xs, {y}) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_add_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_add_limb_in_place(&mut xs, y);
        println!("xs := {xs_old:?}; limbs_vec_add_limb_in_place(&mut xs, {y}); xs = {xs:?}");
    }
}

fn demo_limbs_add_greater(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_add_greater({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_add_greater(&xs, &ys)
        );
    }
}

fn demo_limbs_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen().get(gm, config).take(limit) {
        println!("limbs_add({:?}, {:?}) = {:?}", xs, ys, limbs_add(&xs, &ys));
    }
}

fn demo_limbs_add_same_length_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_31().get(gm, config).take(limit) {
        let out_old = out.clone();
        let carry = limbs_add_same_length_to_out(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_add_same_length_to_out(&mut out, {xs:?}, {ys:?}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_add_greater_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_40().get(gm, config).take(limit) {
        let out_old = xs.clone();
        let carry = limbs_add_greater_to_out(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_add_greater_to_out(&mut out, {xs:?}, {ys:?}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_add_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_32().get(gm, config).take(limit) {
        let out_old = out.clone();
        let carry = limbs_add_to_out(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_add_to_out(&mut out, {xs:?}, {ys:?}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_add_to_out_aliased(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, xs_len) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_add_to_out_aliased(&mut xs, xs_len, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_add_to_out_aliased(&mut xs, {xs_len}, {ys:?}) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_slice_add_same_length_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_6().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let carry = limbs_slice_add_same_length_in_place_left(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_add_same_length_in_place_left(&mut xs, {ys:?}) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_slice_add_greater_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_1().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let carry = limbs_slice_add_greater_in_place_left(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_add_greater_in_place_left(&mut xs, {ys:?}) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_add_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_vec_add_in_place_left(&mut xs, &ys);
        println!("xs := {xs_old:?}; limbs_vec_add_in_place_left(&mut xs, {ys:?}); xs = {xs:?}");
    }
}

fn demo_limbs_slice_add_in_place_either(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let result = limbs_slice_add_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {xs_old:?}; \
            ys := {ys_old:?}; limbs_slice_add_in_place_either(&mut xs, &mut ys) = {result:?}; \
            xs = {xs:?}; ys = {ys:?}",
        );
    }
}

fn demo_limbs_vec_add_in_place_either(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_vec_add_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {xs_old:?}; \
            ys := {ys_old:?}; limbs_vec_add_in_place_either(&mut xs, &mut ys) = {right}; \
             xs = {xs:?}; ys = {ys:?}",
        );
    }
}

fn demo_natural_add_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!("x := {x_old}; x += {y}; x = {x}");
    }
}

fn demo_natural_add_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!("x := {x_old}; x += &{y}; x = {x}");
    }
}

fn demo_natural_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} + {} = {}", x_old, y_old, x + y);
    }
}

fn demo_natural_add_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} + &{} = {}", x_old, y, x + &y);
    }
}

fn demo_natural_add_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} + {} = {}", x, y_old, &x + y);
    }
}

fn demo_natural_add_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!("&{} + &{} = {}", x, y, &x + &y);
    }
}

fn demo_natural_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in natural_vec_gen().get(gm, config).take(limit) {
        println!("sum({:?}) = {}", xs.clone(), Natural::sum(xs.into_iter()));
    }
}

fn demo_natural_ref_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in natural_vec_gen().get(gm, config).take(limit) {
        println!("sum({:?}) = {}", xs, Natural::sum(xs.iter()));
    }
}

fn benchmark_limbs_add_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| no_out!(limbs_add_limb(&xs, y)))],
    );
}

fn benchmark_limbs_add_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            no_out!(limbs_add_limb_to_out(&mut out, &xs, y))
        })],
    );
}

fn benchmark_limbs_slice_add_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen::<Limb, Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            no_out!(limbs_slice_add_limb_in_place(&mut xs, y))
        })],
    );
}

fn benchmark_limbs_vec_add_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            limbs_vec_add_limb_in_place(&mut xs, y)
        })],
    );
}

fn benchmark_limbs_add_greater(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_greater(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, ys)| {
            no_out!(limbs_add_greater(&xs, &ys))
        })],
    );
}

fn benchmark_limbs_add(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| no_out!(limbs_add(&xs, &ys)))],
    );
}

fn benchmark_limbs_add_same_length_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_add_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            no_out!(limbs_add_same_length_to_out(&mut out, &xs, &ys))
        })],
    );
}

fn benchmark_limbs_add_greater_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_add_greater_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_40().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            no_out!(limbs_add_greater_to_out(&mut out, &xs, &ys))
        })],
    );
}

fn benchmark_limbs_add_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_32().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            no_out!(limbs_add_to_out(&mut out, &xs, &ys))
        })],
    );
}

fn benchmark_limbs_add_to_out_aliased(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_add_to_out_aliased(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ys"),
        &mut [("Malachite", &mut |(mut xs, ys, xs_len)| {
            no_out!(limbs_add_to_out_aliased(&mut xs, xs_len, &ys))
        })],
    );
}

fn benchmark_limbs_slice_add_same_length_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_same_length_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, ys)| {
            no_out!(limbs_slice_add_same_length_in_place_left(&mut xs, &ys))
        })],
    );
}

fn benchmark_limbs_slice_add_greater_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_greater_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, ys)| {
            no_out!(limbs_slice_add_greater_in_place_left(&mut xs, &ys))
        })],
    );
}

fn benchmark_limbs_vec_add_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_in_place_left(&mut Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, ys)| {
            no_out!(limbs_vec_add_in_place_left(&mut xs, &ys))
        })],
    );
}

fn benchmark_limbs_slice_add_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, mut ys)| {
            no_out!(limbs_slice_add_in_place_either(&mut xs, &mut ys))
        })],
    );
}

fn benchmark_limbs_vec_add_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_in_place_either(&mut Vec<Limb>, &mut Vec<Limb>)",
        BenchmarkType::Single,
        unsigned_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, mut ys)| {
            no_out!(limbs_vec_add_in_place_either(&mut xs, &mut ys))
        })],
    );
}

fn benchmark_natural_add_assign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural += Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(_, (mut x, y))| x += y), ("rug", &mut |((mut x, y), _)| x += y)],
    );
}

fn benchmark_natural_add_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural += Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural += Natural", &mut |(mut x, y)| no_out!(x += y)),
            ("Natural += &Natural", &mut |(mut x, y)| no_out!(x += &y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_add_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural + Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x + y)),
            ("num", &mut |((x, y), _, _)| no_out!(x + y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x + y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_add_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural + Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural + Natural", &mut |(x, y)| no_out!(x + y)),
            ("Natural + &Natural", &mut |(x, y)| no_out!(x + &y)),
            ("&Natural + Natural", &mut |(x, y)| no_out!(&x + y)),
            ("&Natural + &Natural", &mut |(x, y)| no_out!(&x + &y)),
        ],
    );
}

fn benchmark_natural_sum_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::sum(Iterator<Item=Natural>)",
        BenchmarkType::LibraryComparison,
        natural_vec_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_vec_natural_sum_bits_bucketer(),
        &mut [
            ("Malachite", &mut |(_, _, xs)| {
                no_out!(Natural::sum(xs.into_iter()))
            }),
            ("num", &mut |(xs, _, _)| {
                no_out!(BigUint::sum(xs.into_iter()))
            }),
            ("rug", &mut |(_, xs, _)| {
                no_out!(rug::Integer::sum(xs.iter()))
            }),
        ],
    );
}

fn benchmark_natural_sum_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::sum(Iterator<Item=Natural>)",
        BenchmarkType::Algorithms,
        natural_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_natural_sum_bits_bucketer(),
        &mut [
            ("default", &mut |xs| no_out!(Natural::sum(xs.into_iter()))),
            ("alt", &mut |xs| no_out!(natural_sum_alt(xs.into_iter()))),
        ],
    );
}

fn benchmark_natural_sum_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::sum(Iterator<Item=Natural>)",
        BenchmarkType::EvaluationStrategy,
        natural_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_natural_sum_bits_bucketer(),
        &mut [
            ("Natural::sum(Iterator<Item=Natural>)", &mut |xs| {
                no_out!(Natural::sum(xs.into_iter()))
            }),
            ("Natural::sum(Iterator<Item=&Natural>)", &mut |xs| {
                no_out!(Natural::sum(xs.iter()))
            }),
        ],
    );
}
