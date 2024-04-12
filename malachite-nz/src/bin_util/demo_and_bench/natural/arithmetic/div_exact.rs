// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_2_vec_len_bucketer, quadruple_2_vec_len_bucketer,
    quadruple_3_vec_len_bucketer, triple_2_vec_len_bucketer, unsigned_bit_bucketer,
    vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen_var_22, unsigned_vec_pair_gen_var_12};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::div::{
    limbs_div, limbs_div_limb, limbs_div_limb_in_place, limbs_div_limb_to_out, limbs_div_to_out,
};
use malachite_nz::natural::arithmetic::div_exact::{
    limbs_div_exact, limbs_div_exact_3, limbs_div_exact_3_in_place, limbs_div_exact_3_to_out,
    limbs_div_exact_limb, limbs_div_exact_limb_in_place,
    limbs_div_exact_limb_in_place_no_special_3, limbs_div_exact_limb_no_special_3,
    limbs_div_exact_limb_to_out, limbs_div_exact_limb_to_out_no_special_3, limbs_div_exact_to_out,
    limbs_div_exact_to_out_ref_ref, limbs_div_exact_to_out_ref_val, limbs_div_exact_to_out_val_ref,
    limbs_modular_div, limbs_modular_div_barrett, limbs_modular_div_barrett_scratch_len,
    limbs_modular_div_divide_and_conquer, limbs_modular_div_mod_barrett,
    limbs_modular_div_mod_barrett_scratch_len, limbs_modular_div_mod_divide_and_conquer,
    limbs_modular_div_mod_schoolbook, limbs_modular_div_ref, limbs_modular_div_ref_scratch_len,
    limbs_modular_div_schoolbook, limbs_modular_div_scratch_len, limbs_modular_invert,
    limbs_modular_invert_limb, limbs_modular_invert_scratch_len, limbs_modular_invert_small,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_1_natural_bit_bucketer, triple_3_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    large_type_gen_var_13, large_type_gen_var_14, large_type_gen_var_15, large_type_gen_var_16,
    large_type_gen_var_17, natural_pair_gen_var_6, natural_pair_gen_var_6_nrm,
    unsigned_vec_gen_var_5, unsigned_vec_pair_gen_var_13, unsigned_vec_pair_gen_var_14,
    unsigned_vec_quadruple_gen_var_2, unsigned_vec_quadruple_gen_var_3,
    unsigned_vec_triple_gen_var_46, unsigned_vec_triple_gen_var_47, unsigned_vec_triple_gen_var_48,
    unsigned_vec_triple_gen_var_49, unsigned_vec_unsigned_pair_gen_var_29,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_14,
};
use malachite_nz::test_util::natural::arithmetic::div_exact::{
    limbs_div_exact_3_in_place_alt, limbs_div_exact_3_to_out_alt,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_modular_invert_limb);
    register_demo!(runner, demo_limbs_div_exact_limb);
    register_demo!(runner, demo_limbs_div_exact_limb_to_out);
    register_demo!(runner, demo_limbs_div_exact_limb_in_place);
    register_demo!(runner, demo_limbs_div_exact_3);
    register_demo!(runner, demo_limbs_div_exact_3_to_out);
    register_demo!(runner, demo_limbs_div_exact_3_in_place);
    register_demo!(runner, demo_limbs_modular_invert);
    register_demo!(runner, demo_limbs_modular_div_mod_schoolbook);
    register_demo!(runner, demo_limbs_modular_div_mod_divide_and_conquer);
    register_demo!(runner, demo_limbs_modular_div_mod_barrett);
    register_demo!(runner, demo_limbs_modular_div_schoolbook);
    register_demo!(runner, demo_limbs_modular_div_divide_and_conquer);
    register_demo!(runner, demo_limbs_modular_div_barrett);
    register_demo!(runner, demo_limbs_modular_div);
    register_demo!(runner, demo_limbs_modular_div_ref);
    register_demo!(runner, demo_limbs_div_exact);
    register_demo!(runner, demo_limbs_div_exact_to_out);
    register_demo!(runner, demo_limbs_div_exact_to_out_val_ref);
    register_demo!(runner, demo_limbs_div_exact_to_out_ref_val);
    register_demo!(runner, demo_limbs_div_exact_to_out_ref_ref);
    register_demo!(runner, demo_natural_div_exact_assign);
    register_demo!(runner, demo_natural_div_exact_assign_ref);
    register_demo!(runner, demo_natural_div_exact);
    register_demo!(runner, demo_natural_div_exact_val_ref);
    register_demo!(runner, demo_natural_div_exact_ref_val);
    register_demo!(runner, demo_natural_div_exact_ref_ref);

    register_bench!(runner, benchmark_limbs_modular_invert_limb);
    register_bench!(runner, benchmark_limbs_div_exact_limb_algorithms);
    register_bench!(runner, benchmark_limbs_div_exact_limb_to_out_algorithms);
    register_bench!(runner, benchmark_limbs_div_exact_limb_in_place_algorithms);
    register_bench!(runner, benchmark_limbs_div_exact_3_algorithms);
    register_bench!(runner, benchmark_limbs_div_exact_3_to_out_algorithms);
    register_bench!(runner, benchmark_limbs_div_exact_3_in_place_algorithms);
    register_bench!(runner, benchmark_limbs_modular_invert_algorithms);
    register_bench!(runner, benchmark_limbs_modular_div_mod_schoolbook);
    register_bench!(
        runner,
        benchmark_limbs_modular_div_mod_divide_and_conquer_algorithms
    );
    register_bench!(runner, benchmark_limbs_modular_div_mod_barrett_algorithms);
    register_bench!(runner, benchmark_limbs_modular_div_schoolbook);
    register_bench!(
        runner,
        benchmark_limbs_modular_div_divide_and_conquer_algorithms
    );
    register_bench!(runner, benchmark_limbs_modular_div_barrett_algorithms);
    register_bench!(runner, benchmark_limbs_modular_div_evaluation_strategy);
    register_bench!(runner, benchmark_limbs_div_exact_algorithms);
    register_bench!(runner, benchmark_limbs_div_exact_to_out_algorithms);
    register_bench!(runner, benchmark_limbs_div_exact_to_out_evaluation_strategy);
    register_bench!(runner, benchmark_natural_div_exact_assign_algorithms);
    register_bench!(
        runner,
        benchmark_natural_div_exact_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_div_exact_library_comparison);
    register_bench!(runner, benchmark_natural_div_exact_algorithms);
    register_bench!(runner, benchmark_natural_div_exact_evaluation_strategy);
}

fn demo_limbs_modular_invert_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen_var_22().get(gm, config).take(limit) {
        println!(
            "limbs_modular_invert_limb({}) = {}",
            x,
            limbs_modular_invert_limb(x)
        );
    }
}

fn demo_limbs_div_exact_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_29()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_div_exact_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_div_exact_limb(&xs, y)
        );
    }
}

fn demo_limbs_div_exact_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        limbs_div_exact_limb_to_out(&mut out, &xs, y);
        println!(
            "out := {out_old:?}; limbs_exact_div_limb_to_out(&mut out, {xs:?}, {y}); \
             out = {out:?}",
        );
    }
}

fn demo_limbs_div_exact_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_29()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_div_exact_limb_in_place(&mut xs, y);
        println!("xs := {xs_old:?}; limbs_div_exact_limb_in_place(&mut xs, {y}); xs = {xs:?}");
    }
}

fn demo_limbs_div_exact_3(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_5().get(gm, config).take(limit) {
        println!("limbs_div_exact_3({:?}) = {:?}", xs, limbs_div_exact_3(&xs));
    }
}

fn demo_limbs_div_exact_3_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs) in unsigned_vec_pair_gen_var_13().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_div_exact_3_to_out(&mut out, &xs);
        println!("out := {out_old:?}; limbs_exact_div_3_to_out(&mut out, {xs:?}); out = {out:?}");
    }
}

fn demo_limbs_div_exact_3_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut xs in unsigned_vec_gen_var_5().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_div_exact_3_in_place(&mut xs);
        println!("xs := {xs_old:?}; limbs_div_exact_3_in_place(&mut xs); xs = {xs:?}");
    }
}

fn demo_limbs_modular_invert(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut is, ds) in unsigned_vec_pair_gen_var_12().get(gm, config).take(limit) {
        let old_is = is.clone();
        let mut scratch = vec![0; limbs_modular_invert_scratch_len(ds.len())];
        limbs_modular_invert(&mut is, &ds, &mut scratch);
        println!(
            "is := {old_is:?}; limbs_modular_invert(&mut is, {ds:?}, &mut scratch); is = {is:?}, "
        );
    }
}

fn demo_limbs_modular_div_mod_schoolbook(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_14().get(gm, config).take(limit) {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        let borrow = limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {qs_old:?}; ns := {ns_old:?}; \
             limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, {ds:?}, {inverse}) = {borrow}; \
             qs = {qs:?}; ns = {ns:?}",
        );
    }
}

fn demo_limbs_modular_div_mod_divide_and_conquer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_15().get(gm, config).take(limit) {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        let borrow = limbs_modular_div_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {qs_old:?}; ns := {ns_old:?}; \
             limbs_modular_div_mod_divide_and_conquer(&mut qs, &mut ns, {ds:?}, {inverse}) = \
             {borrow}; qs = {qs:?}; ns = {ns:?}",
        );
    }
}

fn demo_limbs_modular_div_mod_barrett(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut rs, ns, ds) in unsigned_vec_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let qs_old = qs.clone();
        let rs_old = rs.clone();
        let mut scratch = vec![0; limbs_modular_div_mod_barrett_scratch_len(ns.len(), ds.len())];
        let borrow = limbs_modular_div_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
        println!(
            "qs := {qs_old:?}; rs := {rs_old:?}; limbs_modular_div_mod_divide_and_conquer(\
             &mut qs, &mut rs, {ns:?}, {ds:?} &mut scratch) = {borrow}; qs = {qs:?}; rs = {rs:?}",
        );
    }
}

fn demo_limbs_modular_div_schoolbook(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_13().get(gm, config).take(limit) {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {qs_old:?}; \
            ns := {ns_old:?}; limbs_modular_div_schoolbook(&mut qs, &mut ns, {ds:?}, {inverse}); \
             qs = {qs:?}",
        );
    }
}

fn demo_limbs_modular_div_divide_and_conquer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_16().get(gm, config).take(limit) {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {qs_old:?}; ns := {ns_old:?}; \
             limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, {ds:?}, {inverse}); \
             qs = {qs:?}",
        );
    }
}

fn demo_limbs_modular_div_barrett(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, ds) in unsigned_vec_triple_gen_var_46().get(gm, config).take(limit) {
        let qs_old = qs.clone();
        let mut scratch = vec![0; limbs_modular_div_barrett_scratch_len(ns.len(), ds.len())];
        limbs_modular_div_barrett(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {qs_old:?}; limbs_modular_div_barrett(&mut qs, {ns:?}, {ds:?} &mut scratch); \
            qs = {qs:?}",
        );
    }
}

fn demo_limbs_modular_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds) in unsigned_vec_triple_gen_var_47().get(gm, config).take(limit) {
        let ns_old = ns.clone();
        let qs_old = qs.clone();
        let mut scratch = vec![0; limbs_modular_div_scratch_len(ns.len(), ds.len())];
        limbs_modular_div(&mut qs, &mut ns, &ds, &mut scratch);
        println!(
            "qs := {qs_old:?}; limbs_modular_div(&mut qs, {ns_old:?}, {ds:?} &mut scratch); \
            qs = {qs:?}",
        );
    }
}

fn demo_limbs_modular_div_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, ds) in unsigned_vec_triple_gen_var_47().get(gm, config).take(limit) {
        let qs_old = qs.clone();
        let mut scratch = vec![0; limbs_modular_div_ref_scratch_len(ns.len(), ds.len())];
        limbs_modular_div_ref(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {qs_old:?}; limbs_modular_div_ref(&mut qs, {ns:?}, {ds:?} &mut scratch); \
            qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, ds) in unsigned_vec_pair_gen_var_14().get(gm, config).take(limit) {
        println!(
            "limbs_div_exact({:?}, {:?}) = {:?}",
            ns,
            ds,
            limbs_div_exact(&ns, &ds)
        );
    }
}

fn demo_limbs_div_exact_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, mut ds) in unsigned_vec_triple_gen_var_48().get(gm, config).take(limit) {
        let ns_old = ns.clone();
        let ds_old = ds.clone();
        let qs_old = qs.clone();
        limbs_div_exact_to_out(&mut qs, &mut ns, &mut ds);
        println!(
            "qs := {qs_old:?}; limbs_div_exact_to_out(&mut qs, {ns_old:?}, {ds_old:?}); \
            qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_exact_to_out_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds) in unsigned_vec_triple_gen_var_48().get(gm, config).take(limit) {
        let ns_old = ns.clone();
        let qs_old = qs.clone();
        limbs_div_exact_to_out_val_ref(&mut qs, &mut ns, &ds);
        println!(
            "qs := {qs_old:?}; limbs_div_exact_to_out_val_ref(&mut qs, {ns_old:?}, {ds:?}); \
            qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_exact_to_out_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, mut ds) in unsigned_vec_triple_gen_var_48().get(gm, config).take(limit) {
        let ds_old = ds.clone();
        let qs_old = qs.clone();
        limbs_div_exact_to_out_ref_val(&mut qs, &ns, &mut ds);
        println!(
            "qs := {qs_old:?}; limbs_div_exact_to_out_ref_val(&mut qs, {ns:?}, {ds_old:?}); \
            qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_exact_to_out_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, ds) in unsigned_vec_triple_gen_var_48().get(gm, config).take(limit) {
        let qs_old = qs.clone();
        limbs_div_exact_to_out_ref_ref(&mut qs, &ns, &ds);
        println!(
            "qs := {qs_old:?}; limbs_div_exact_to_out_ref_ref(&mut qs, {ns:?}, {ds:?}); \
            qs = {qs:?}",
        );
    }
}

fn demo_natural_div_exact_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_6().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.div_exact_assign(y);
        println!("x := {x_old}; x.div_exact_assign({y_old}); x = {x}");
    }
}

fn demo_natural_div_exact_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_6().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.div_exact_assign(&y);
        println!("x := {x_old}; x.div_exact_assign(&{y}); x = {x}");
    }
}

fn demo_natural_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_6().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_exact({}) = {}", x_old, y_old, x.div_exact(y));
    }
}

fn demo_natural_div_exact_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_6().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.div_exact(&{}) = {}", x_old, y, x.div_exact(&y));
    }
}

fn demo_natural_div_exact_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_6().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_exact({}) = {}", x, y_old, (&x).div_exact(y));
    }
}

fn demo_natural_div_exact_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_6().get(gm, config).take(limit) {
        println!("(&{}).div_exact(&{}) = {}", x, y, (&x).div_exact(&y));
    }
}

fn benchmark_limbs_modular_invert_limb(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_invert_limb(Limb)",
        BenchmarkType::Single,
        unsigned_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(limbs_modular_invert_limb(x)))],
    );
}

fn benchmark_limbs_div_exact_limb_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact_limb(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_29().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("div_exact", &mut |(xs, y)| {
                no_out!(limbs_div_exact_limb(&xs, y))
            }),
            ("div", &mut |(xs, y)| no_out!(limbs_div_limb(&xs, y))),
        ],
    );
}

fn benchmark_limbs_div_exact_limb_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("div_exact", &mut |(mut out, xs, y)| {
                limbs_div_exact_limb_to_out(&mut out, &xs, y)
            }),
            ("div", &mut |(mut out, xs, y)| {
                limbs_div_limb_to_out(&mut out, &xs, y)
            }),
        ],
    );
}

fn benchmark_limbs_div_exact_limb_in_place_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_29().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("div_exact", &mut |(mut xs, y)| {
                limbs_div_exact_limb_in_place(&mut xs, y)
            }),
            ("div", &mut |(mut xs, y)| {
                limbs_div_limb_in_place(&mut xs, y)
            }),
        ],
    );
}

fn benchmark_limbs_div_exact_3_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact_3(&[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("limbs_div_exact_3", &mut |xs| {
                no_out!(limbs_div_exact_3(&xs))
            }),
            ("limbs_div_exact_limb_no_special_3", &mut |xs| {
                no_out!(limbs_div_exact_limb_no_special_3(&xs, 3))
            }),
        ],
    );
}

fn benchmark_limbs_div_exact_3_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact_limb_to_out(&mut [Limb], 3)",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            (
                "limbs_div_exact_limb_to_out_no_special_3",
                &mut |(mut out, xs)| limbs_div_exact_limb_to_out_no_special_3(&mut out, &xs, 3),
            ),
            ("limbs_div_exact_3_to_out", &mut |(mut out, xs)| {
                limbs_div_exact_3_to_out(&mut out, &xs)
            }),
            ("limbs_div_exact_3_to_out_alt", &mut |(mut out, xs)| {
                limbs_div_exact_3_to_out_alt(&mut out, &xs)
            }),
        ],
    );
}

fn benchmark_limbs_div_exact_3_in_place_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact_limb_in_place(&mut [Limb], 3)",
        BenchmarkType::Algorithms,
        unsigned_vec_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            (
                "limbs_div_exact_limb_in_place_no_special_3",
                &mut |mut xs| limbs_div_exact_limb_in_place_no_special_3(&mut xs, 3),
            ),
            ("limbs_div_exact_3_in_place", &mut |mut xs| {
                limbs_div_exact_3_in_place(&mut xs)
            }),
            ("limbs_div_exact_3_in_place_alt", &mut |mut xs| {
                limbs_div_exact_3_in_place_alt(&mut xs)
            }),
        ],
    );
}

// use large params
fn benchmark_limbs_modular_invert_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_invert(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        large_type_gen_var_17().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("ds"),
        &mut [
            ("modular invert small", &mut |(
                mut is,
                mut scratch,
                ds,
                inverse,
            )| {
                let n = ds.len();
                limbs_modular_invert_small(n, &mut is, &mut scratch[..n], &ds, inverse);
            }),
            ("modular invert", &mut |(mut is, mut scratch, ds, _)| {
                limbs_modular_invert(&mut is, &ds, &mut scratch);
            }),
        ],
    );
}

fn benchmark_limbs_modular_div_mod_schoolbook(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_div_mod_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        large_type_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("ns"),
        &mut [("Malachite", &mut |(mut qs, mut ns, ds, inverse)| {
            no_out!(limbs_modular_div_mod_schoolbook(
                &mut qs, &mut ns, &ds, inverse
            ))
        })],
    );
}

// use large params
fn benchmark_limbs_modular_div_mod_divide_and_conquer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_div_mod_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("ds"),
        &mut [
            ("schoolbook", &mut |(mut qs, mut ns, ds, inverse)| {
                no_out!(limbs_modular_div_mod_schoolbook(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
            ("divide-and-conquer", &mut |(
                mut qs,
                mut ns,
                ds,
                inverse,
            )| {
                no_out!(limbs_modular_div_mod_divide_and_conquer(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
        ],
    );
}

fn benchmark_limbs_modular_div_mod_barrett_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_div_mod_barrett(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("ns"),
        &mut [
            ("divide-and-conquer", &mut |(mut qs, _, mut ns, ds)| {
                let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
                no_out!(limbs_modular_div_mod_divide_and_conquer(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
            ("Barrett", &mut |(mut qs, mut rs, ns, ds)| {
                let mut scratch =
                    vec![0; limbs_modular_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                no_out!(limbs_modular_div_mod_barrett(
                    &mut qs,
                    &mut rs,
                    &ns,
                    &ds,
                    &mut scratch
                ))
            }),
        ],
    );
}

fn benchmark_limbs_modular_div_schoolbook(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_div_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        large_type_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("ns"),
        &mut [("Malachite", &mut |(mut qs, mut ns, ds, inverse)| {
            limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse)
        })],
    );
}

// use large params
fn benchmark_limbs_modular_div_divide_and_conquer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_div_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("ns"),
        &mut [
            ("schoolbook", &mut |(mut qs, mut ns, ds, inverse)| {
                limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse)
            }),
            ("divide-and-conquer", &mut |(
                mut qs,
                mut ns,
                ds,
                inverse,
            )| {
                limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse)
            }),
        ],
    );
}

fn benchmark_limbs_modular_div_barrett_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_div_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_46().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ns"),
        &mut [
            ("divide-and-conquer", &mut |(mut qs, mut ns, ds)| {
                let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
                limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse)
            }),
            ("Barrett", &mut |(mut qs, ns, ds)| {
                let mut scratch =
                    vec![0; limbs_modular_div_barrett_scratch_len(ns.len(), ds.len())];
                limbs_modular_div_barrett(&mut qs, &ns, &ds, &mut scratch)
            }),
        ],
    );
}

// use large params
fn benchmark_limbs_modular_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_modular_div(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_triple_gen_var_47().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ns"),
        &mut [
            (
                "limbs_modular_div(&mut [Limb], &mut [Limb], &[Limb], &mut [Limb])",
                &mut |(mut qs, mut ns, ds)| {
                    let mut scratch = vec![0; limbs_modular_div_scratch_len(ns.len(), ds.len())];
                    limbs_modular_div(&mut qs, &mut ns, &ds, &mut scratch)
                },
            ),
            (
                "limbs_modular_div_ref(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
                &mut |(mut qs, ns, ds)| {
                    let mut scratch =
                        vec![0; limbs_modular_div_ref_scratch_len(ns.len(), ds.len())];
                    limbs_modular_div_ref(&mut qs, &ns, &ds, &mut scratch)
                },
            ),
        ],
    );
}

fn benchmark_limbs_div_exact_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact(&[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ns"),
        &mut [
            ("div_exact", &mut |(ns, ds)| {
                no_out!(limbs_div_exact(&ns, &ds))
            }),
            ("div", &mut |(ns, ds)| no_out!(limbs_div(&ns, &ds))),
        ],
    );
}

fn benchmark_limbs_div_exact_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact_to_out(&mut [Limb], &mut [Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_49().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ns"),
        &mut [
            ("div", &mut |(mut qs, mut ns, mut ds)| {
                limbs_div_to_out(&mut qs, &mut ns, &mut ds)
            }),
            ("div exact", &mut |(mut qs, mut ns, mut ds)| {
                limbs_div_exact_to_out(&mut qs, &mut ns, &mut ds)
            }),
        ],
    );
}

fn benchmark_limbs_div_exact_to_out_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_exact_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_triple_gen_var_49().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ns"),
        &mut [
            (
                "limbs_div_exact_to_out(&mut [Limb], &mut [Limb], &mut [Limb])",
                &mut |(mut qs, mut ns, mut ds)| limbs_div_exact_to_out(&mut qs, &mut ns, &mut ds),
            ),
            (
                "limbs_div_exact_to_out_val_ref(&mut [Limb], &mut [Limb], &[Limb])",
                &mut |(mut qs, mut ns, ds)| limbs_div_exact_to_out_val_ref(&mut qs, &mut ns, &ds),
            ),
            (
                "limbs_div_exact_to_out_ref_val(&mut [Limb], &[Limb], &mut [Limb])",
                &mut |(mut qs, ns, mut ds)| limbs_div_exact_to_out_ref_val(&mut qs, &ns, &mut ds),
            ),
            (
                "limbs_div_exact_to_out_ref_ref(&mut [Limb], &[Limb], &[Limb])",
                &mut |(mut qs, ns, ds)| limbs_div_exact_to_out_ref_ref(&mut qs, &ns, &ds),
            ),
        ],
    );
}

// use large params
fn benchmark_natural_div_exact_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_exact_assign(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("ordinary division", &mut |(mut x, y)| x /= y),
            ("exact division", &mut |(mut x, y)| x.div_exact_assign(y)),
        ],
    );
}

// use large params
fn benchmark_natural_div_exact_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_exact_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural.div_exact_assign(Natural)", &mut |(mut x, y)| {
                x.div_exact_assign(y)
            }),
            ("Natural.div_exact_assign(&Natural)", &mut |(mut x, y)| {
                x.div_exact_assign(&y)
            }),
        ],
    );
}

// use large params
#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_div_exact_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_exact(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_6_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("num", &mut |((x, y), _, _)| no_out!(x / y)),
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.div_exact(y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.div_exact(&y))),
        ],
    );
}

// use large params
#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_div_exact_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_exact(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("ordinary division", &mut |(x, y)| no_out!(x / y)),
            ("exact division", &mut |(x, y)| no_out!(x.div_exact(y))),
        ],
    );
}

// use large params
fn benchmark_natural_div_exact_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_exact(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural.div_exact(Natural)", &mut |(x, y)| {
                no_out!(x.div_exact(y))
            }),
            ("Natural.div_exact(&Natural)", &mut |(x, y)| {
                no_out!(x.div_exact(&y))
            }),
            ("(&Natural).div_exact(Natural)", &mut |(x, y)| {
                no_out!((&x).div_exact(y))
            }),
            ("(&Natural).div_exact(&Natural)", &mut |(x, y)| {
                no_out!((&x).div_exact(&y))
            }),
        ],
    );
}
