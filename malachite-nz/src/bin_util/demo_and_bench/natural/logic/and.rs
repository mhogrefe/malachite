// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_vec_min_len_bucketer, triple_2_3_vec_min_len_bucketer,
    triple_2_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen, unsigned_vec_pair_gen_var_1, unsigned_vec_pair_gen_var_6,
    unsigned_vec_triple_gen_var_31, unsigned_vec_triple_gen_var_32,
    unsigned_vec_unsigned_pair_gen_var_15,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::and::{
    limbs_and, limbs_and_in_place_either, limbs_and_limb, limbs_and_same_length_to_out,
    limbs_and_to_out, limbs_slice_and_in_place_left, limbs_slice_and_same_length_in_place_left,
    limbs_vec_and_in_place_left,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_natural_min_bit_bucketer, pair_natural_min_bit_bucketer,
    triple_3_pair_natural_min_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_pair_gen, natural_pair_gen_nrm, natural_pair_gen_rm,
};
use malachite_nz::test_util::natural::logic::and::{natural_and_alt_1, natural_and_alt_2};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_and_limb);
    register_demo!(runner, demo_limbs_and);
    register_demo!(runner, demo_limbs_and_same_length_to_out);
    register_demo!(runner, demo_limbs_and_to_out);
    register_demo!(runner, demo_limbs_slice_and_same_length_in_place_left);
    register_demo!(runner, demo_limbs_slice_and_in_place_left);
    register_demo!(runner, demo_limbs_vec_and_in_place_left);
    register_demo!(runner, demo_limbs_and_in_place_either);
    register_demo!(runner, demo_natural_and_assign);
    register_demo!(runner, demo_natural_and_assign_ref);
    register_demo!(runner, demo_natural_and);
    register_demo!(runner, demo_natural_and_val_ref);
    register_demo!(runner, demo_natural_and_ref_val);
    register_demo!(runner, demo_natural_and_ref_ref);

    register_bench!(runner, benchmark_limbs_and_limb);
    register_bench!(runner, benchmark_limbs_and);
    register_bench!(runner, benchmark_limbs_and_same_length_to_out);
    register_bench!(runner, benchmark_limbs_and_to_out);
    register_bench!(runner, benchmark_limbs_slice_and_same_length_in_place_left);
    register_bench!(runner, benchmark_limbs_slice_and_in_place_left);
    register_bench!(runner, benchmark_limbs_vec_and_in_place_left);
    register_bench!(runner, benchmark_limbs_and_in_place_either);
    register_bench!(runner, benchmark_natural_and_assign_library_comparison);
    register_bench!(runner, benchmark_natural_and_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_and_library_comparison);
    register_bench!(runner, benchmark_natural_and_algorithms);
    register_bench!(runner, benchmark_natural_and_evaluation_strategy);
}

fn demo_limbs_and_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_and_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_and_limb(&xs, y)
        );
    }
}

fn demo_limbs_and(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen().get(gm, config).take(limit) {
        println!("limbs_and({:?}, {:?}) = {:?}", xs, ys, limbs_and(&xs, &ys));
    }
}

fn demo_limbs_and_same_length_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_31().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_and_same_length_to_out(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_and_same_length_to_out(&mut out, {xs:?}, {ys:?}); out = {out:?}",
        );
    }
}

fn demo_limbs_and_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_32().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_and_to_out(&mut out, &xs, &ys);
        println!("out := {out_old:?}; limbs_and_to_out(&mut out, {xs:?}, {ys:?}); out = {out:?}");
    }
}

fn demo_limbs_slice_and_same_length_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_6().get(gm, config).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_slice_and_same_length_in_place_left(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_and_same_length_in_place_left(&mut xs, {ys:?}); xs = {xs:?}",
        );
    }
}

fn demo_limbs_slice_and_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let truncate_size = limbs_slice_and_in_place_left(&mut xs, &ys);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_and_in_place_left(&mut xs, {ys:?}) = {truncate_size:?}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_and_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_vec_and_in_place_left(&mut xs, &ys);
        println!("xs := {xs_old:?}; limbs_vec_and_in_place_left(&mut xs, {ys:?}); xs = {xs:?}");
    }
}

fn demo_limbs_and_in_place_either(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_and_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {xs_old:?}; \
            ys := {ys_old:?}; limbs_and_in_place_either(&mut xs, &mut ys) = {right}; \
             xs = {xs:?}; ys = {ys:?}",
        );
    }
}

fn demo_natural_and_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x &= y.clone();
        println!("x := {x_old}; x &= {y}; x = {x}");
    }
}

fn demo_natural_and_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x &= &y;
        println!("x := {x_old}; x &= &{y}; x = {x}");
    }
}

fn demo_natural_and(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} & {} = {}", x_old, y_old, x & y);
    }
}

fn demo_natural_and_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} & &{} = {}", x_old, y, x & &y);
    }
}

fn demo_natural_and_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} & {} = {}", x, y_old, &x & y);
    }
}

fn demo_natural_and_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!("&{} & &{} = {}", x, y, &x & &y);
    }
}

fn benchmark_limbs_and_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_and_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| no_out!(limbs_and_limb(&xs, y)))],
    );
}

fn benchmark_limbs_and(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_and(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_min_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| no_out!(limbs_and(&xs, &ys)))],
    );
}

fn benchmark_limbs_and_same_length_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_and_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            limbs_and_same_length_to_out(&mut out, &xs, &ys)
        })],
    );
}

fn benchmark_limbs_and_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_and_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_vec_min_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            limbs_and_to_out(&mut out, &xs, &ys)
        })],
    );
}

fn benchmark_limbs_slice_and_same_length_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_and_same_length_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, ys)| {
            limbs_slice_and_same_length_in_place_left(&mut xs, &ys)
        })],
    );
}

fn benchmark_limbs_slice_and_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_and_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_min_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, ys)| {
            no_out!(limbs_slice_and_in_place_left(&mut xs, &ys))
        })],
    );
}

fn benchmark_limbs_vec_and_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_and_in_place_left(&Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_min_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, ys)| {
            limbs_vec_and_in_place_left(&mut xs, &ys)
        })],
    );
}

fn benchmark_limbs_and_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_and_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_min_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, mut ys)| {
            no_out!(limbs_and_in_place_either(&mut xs, &mut ys))
        })],
    );
}

fn benchmark_natural_and_assign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural &= Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_min_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(_, (mut x, y))| x &= y), ("rug", &mut |((mut x, y), _)| x &= y)],
    );
}

fn benchmark_natural_and_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural &= Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_min_bit_bucketer("x", "y"),
        &mut [
            ("Natural &= Natural", &mut |(mut x, y)| no_out!(x &= y)),
            ("Natural &= &Natural", &mut |(mut x, y)| no_out!(x &= &y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_and_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural & Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_min_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x & y)),
            ("num", &mut |((x, y), _, _)| no_out!(x & y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x & y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_and_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural & Natural",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_min_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(ref x, ref y)| no_out!(x & y)),
            ("using bits explicitly", &mut |(ref x, ref y)| {
                no_out!(natural_and_alt_1(x, y))
            }),
            ("using limbs explicitly", &mut |(ref x, ref y)| {
                no_out!(natural_and_alt_2(x, y))
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_and_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural & Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_min_bit_bucketer("x", "y"),
        &mut [
            ("Natural & Natural", &mut |(x, y)| no_out!(x & y)),
            ("Natural & &Natural", &mut |(x, y)| no_out!(x & &y)),
            ("&Natural & Natural", &mut |(x, y)| no_out!(&x & y)),
            ("&Natural & &Natural", &mut |(x, y)| no_out!(&x & &y)),
        ],
    );
}
