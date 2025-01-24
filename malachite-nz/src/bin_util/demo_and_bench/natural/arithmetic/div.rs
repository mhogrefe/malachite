// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedDiv, DivMod};
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, quadruple_2_3_diff_vec_len_bucketer, quadruple_2_vec_len_bucketer,
    quadruple_3_vec_len_bucketer, triple_1_vec_len_bucketer, triple_2_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_11, unsigned_vec_unsigned_pair_gen_var_22,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::div::{
    limbs_div, limbs_div_barrett, limbs_div_barrett_approx, limbs_div_barrett_approx_scratch_len,
    limbs_div_barrett_scratch_len, limbs_div_divide_and_conquer,
    limbs_div_divide_and_conquer_approx, limbs_div_divisor_of_limb_max_with_carry_in_place,
    limbs_div_divisor_of_limb_max_with_carry_to_out, limbs_div_limb, limbs_div_limb_in_place,
    limbs_div_limb_to_out, limbs_div_schoolbook, limbs_div_schoolbook_approx, limbs_div_to_out,
    limbs_div_to_out_balanced, limbs_div_to_out_ref_ref, limbs_div_to_out_ref_val,
    limbs_div_to_out_unbalanced, limbs_div_to_out_val_ref,
};
use malachite_nz::natural::arithmetic::div_mod::{
    limbs_div_mod, limbs_div_mod_barrett, limbs_div_mod_barrett_scratch_len,
    limbs_div_mod_divide_and_conquer, limbs_div_mod_schoolbook, limbs_div_mod_to_out,
    limbs_two_limb_inverse_helper,
};
use malachite_nz::test_util::bench::bucketers::{
    limbs_div_to_out_balancing_bucketer, pair_1_natural_bit_bucketer,
    pair_2_pair_1_natural_bit_bucketer, triple_3_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    large_type_gen_var_10, large_type_gen_var_11, large_type_gen_var_12, natural_pair_gen,
    natural_pair_gen_nm, natural_pair_gen_var_5, natural_pair_gen_var_5_nrm,
    unsigned_vec_quadruple_gen_var_1, unsigned_vec_triple_gen_var_42,
    unsigned_vec_triple_gen_var_43, unsigned_vec_triple_gen_var_44, unsigned_vec_triple_gen_var_45,
    unsigned_vec_unsigned_unsigned_triple_gen_var_9,
};
use malachite_nz::test_util::natural::arithmetic::div::{
    limbs_div_limb_in_place_alt, limbs_div_limb_to_out_alt,
};
use num::CheckedDiv as NumCheckedDiv;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_div_limb);
    register_demo!(runner, demo_limbs_div_limb_to_out);
    register_demo!(runner, demo_limbs_div_limb_in_place);
    register_demo!(runner, demo_limbs_div_divisor_of_limb_max_with_carry_to_out);
    register_demo!(
        runner,
        demo_limbs_div_divisor_of_limb_max_with_carry_in_place
    );
    register_demo!(runner, demo_limbs_div_schoolbook);
    register_demo!(runner, demo_limbs_div_divide_and_conquer);
    register_demo!(runner, demo_limbs_div_barrett);
    register_demo!(runner, demo_limbs_div_schoolbook_approx);
    register_demo!(runner, demo_limbs_div_divide_and_conquer_approx);
    register_demo!(runner, demo_limbs_div_barrett_approx);
    register_demo!(runner, demo_limbs_div);
    register_demo!(runner, demo_limbs_div_to_out);
    register_demo!(runner, demo_limbs_div_to_out_val_ref);
    register_demo!(runner, demo_limbs_div_to_out_ref_val);
    register_demo!(runner, demo_limbs_div_to_out_ref_ref);
    register_demo!(runner, demo_natural_div_assign);
    register_demo!(runner, demo_natural_div_assign_ref);
    register_demo!(runner, demo_natural_div);
    register_demo!(runner, demo_natural_div_val_ref);
    register_demo!(runner, demo_natural_div_ref_val);
    register_demo!(runner, demo_natural_div_ref_ref);
    register_demo!(runner, demo_natural_checked_div);
    register_demo!(runner, demo_natural_checked_div_val_ref);
    register_demo!(runner, demo_natural_checked_div_ref_val);
    register_demo!(runner, demo_natural_checked_div_ref_ref);

    register_bench!(runner, benchmark_limbs_div_limb);
    register_bench!(runner, benchmark_limbs_div_limb_to_out_algorithms);
    register_bench!(runner, benchmark_limbs_div_limb_in_place_algorithms);
    register_bench!(
        runner,
        benchmark_limbs_div_divisor_of_limb_max_with_carry_to_out
    );
    register_bench!(
        runner,
        benchmark_limbs_div_divisor_of_limb_max_with_carry_in_place
    );
    register_bench!(runner, benchmark_limbs_div_schoolbook_algorithms);
    register_bench!(runner, benchmark_limbs_div_divide_and_conquer_algorithms);
    register_bench!(runner, benchmark_limbs_div_barrett_algorithms);
    register_bench!(runner, benchmark_limbs_div_schoolbook_approx_algorithms);
    register_bench!(
        runner,
        benchmark_limbs_div_divide_and_conquer_approx_algorithms
    );
    register_bench!(runner, benchmark_limbs_div_barrett_approx_algorithms);
    register_bench!(runner, benchmark_limbs_div_algorithms);
    register_bench!(runner, benchmark_limbs_div_to_out_balancing_algorithms);
    register_bench!(runner, benchmark_limbs_div_to_out_evaluation_strategy);
    register_bench!(runner, benchmark_limbs_div_to_out_ref_ref_algorithms);
    register_bench!(runner, benchmark_natural_div_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_div_library_comparison);
    register_bench!(runner, benchmark_natural_div_algorithms);
    register_bench!(runner, benchmark_natural_div_evaluation_strategy);
    register_bench!(runner, benchmark_natural_checked_div_library_comparison);
    register_bench!(runner, benchmark_natural_checked_div_evaluation_strategy);
}

fn demo_limbs_div_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_div_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_div_limb(&xs, y)
        );
    }
}

fn demo_limbs_div_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        limbs_div_limb_to_out(&mut out, &xs, y);
        println!("out := {out_old:?}; limbs_div_limb_to_out(&mut out, {xs:?}, {y}); out = {out:?}");
    }
}

fn demo_limbs_div_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_div_limb_in_place(&mut xs, y);
        println!("limbs := {xs_old:?}; limbs_div_limb_in_place(&mut limbs, {y}); limbs = {xs:?}");
    }
}

fn demo_limbs_div_divisor_of_limb_max_with_carry_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut out, xs, divisor, carry) in large_type_gen_var_10().get(gm, config).take(limit) {
        let out_old = out.clone();
        let carry_out =
            limbs_div_divisor_of_limb_max_with_carry_to_out(&mut out, &xs, divisor, carry);
        println!(
            "out := {out_old:?}; \
            limbs_div_divisor_of_limb_max_with_carry_to_out(&mut out, {xs:?}, {divisor}, {carry}) \
             = {carry_out}; out = {out:?}",
        );
    }
}

fn demo_limbs_div_divisor_of_limb_max_with_carry_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut xs, divisor, carry) in unsigned_vec_unsigned_unsigned_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry_out = limbs_div_divisor_of_limb_max_with_carry_in_place(&mut xs, divisor, carry);
        println!(
            "xs := {xs_old:?}; \
            limbs_div_divisor_of_limb_max_with_carry_in_place(&mut xs, {divisor}, {carry}) = \
            {carry_out}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_div_schoolbook(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_11().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {old_qs:?}; \
            ns := {old_ns:?}; limbs_div_schoolbook(&mut qs, &mut ns, {ds:?}, {inverse}) = \
            {highest_q}; qs = {qs:?}, ns = {ns:?}",
        );
    }
}

fn demo_limbs_div_divide_and_conquer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, ds, inverse) in large_type_gen_var_12().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let highest_q = limbs_div_divide_and_conquer(&mut qs, &ns, &ds, inverse);
        println!(
            "qs := {old_qs:?}; \
            ns := {ns:?}; limbs_div_divide_and_conquer(&mut qs, &ns, {ds:?}, {inverse}) = \
            {highest_q}; qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_barrett(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, ds) in unsigned_vec_triple_gen_var_43().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let mut scratch = vec![0; limbs_div_barrett_scratch_len(ns.len(), ds.len())];
        let highest_q = limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {old_qs:?}; ns := {ns:?}; \
             limbs_div_barrett(&mut qs, ns, {ds:?}, &mut scratch) = {highest_q}; qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_schoolbook_approx(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_11().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {old_qs:?}; ns := {old_ns:?}; \
             limbs_div_schoolbook_approx(&mut qs, &mut ns, {ds:?}, {inverse}) = {highest_q}; \
             qs = {qs:?}, ns = {ns:?}",
        );
    }
}

fn demo_limbs_div_divide_and_conquer_approx(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_12().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {old_qs:?}; ns := {old_ns:?}; \
             limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, {ds:?}, {inverse}) = \
             {highest_q}; qs = {qs:?}, ns = {ns:?}",
        );
    }
}

fn demo_limbs_div_barrett_approx(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, ds) in unsigned_vec_triple_gen_var_42().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let mut scratch = vec![0; limbs_div_barrett_approx_scratch_len(ns.len(), ds.len())];
        let highest_q = limbs_div_barrett_approx(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {old_qs:?}; ns := {ns:?}; \
             limbs_div_barrett_approx(&mut qs, ns, {ds:?}, &mut scratch) = {highest_q}; \
             qs = {qs:?}",
        );
    }
}

fn demo_limbs_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, ds) in unsigned_vec_pair_gen_var_11().get(gm, config).take(limit) {
        println!("limbs_div({:?}, {:?}) = {:?}", ns, ds, limbs_div(&ns, &ds));
    }
}

fn demo_limbs_div_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, mut ds) in unsigned_vec_triple_gen_var_44().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let old_ds = ds.clone();
        limbs_div_to_out(&mut qs, &mut ns, &mut ds);
        println!(
            "qs := {old_qs:?}; \
            ns := {old_ns:?}; ds := {old_ds:?}; limbs_div_to_out(&mut qs, &mut ns, &mut ds); \
             qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_to_out_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds) in unsigned_vec_triple_gen_var_44().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        limbs_div_to_out_val_ref(&mut qs, &mut ns, &ds);
        println!(
            "qs := {old_qs:?}; \
            ns := {old_ns:?}; limbs_div_to_out_val_ref(&mut qs, &mut ns, {ds:?}); qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_to_out_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, mut ds) in unsigned_vec_triple_gen_var_44().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ds = ds.clone();
        limbs_div_to_out_ref_val(&mut qs, &ns, &mut ds);
        println!(
            "qs := {old_qs:?}; \
            ds := {old_ds:?}; limbs_div_to_out_ref_val(&mut qs, {ns:?}, &mut ds); qs = {qs:?}",
        );
    }
}

fn demo_limbs_div_to_out_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, ns, ds) in unsigned_vec_triple_gen_var_44().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        limbs_div_to_out_ref_ref(&mut qs, &ns, &ds);
        println!(
            "qs := {old_qs:?}; limbs_div_to_out_ref_ref(&mut qs, {ns:?}, {ds:?}); qs = {qs:?}",
        );
    }
}

fn demo_natural_div_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x /= y;
        println!("x := {x_old}; x /= {y_old}; x = {x}");
    }
}

fn demo_natural_div_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        x /= &y;
        println!("x := {x_old}; x /= &{y}; x = {x}");
    }
}

fn demo_natural_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} / {} = {}", x_old, y_old, x / y);
    }
}

fn demo_natural_div_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} / &{} = {}", x_old, y, x / &y);
    }
}

fn demo_natural_div_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} / {} = {}", x, y_old, &x / y);
    }
}

fn demo_natural_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        println!("&{} / &{} = {}", x, y, &x / &y);
    }
}

fn demo_natural_checked_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).checked_div({}) = {:?}",
            x_old,
            y_old,
            x.checked_div(y)
        );
    }
}

fn demo_natural_checked_div_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).checked_div(&{}) = {:?}", x_old, y, x.checked_div(&y));
    }
}

fn demo_natural_checked_div_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).checked_div({}) = {:?}",
            x,
            y_old,
            (&x).checked_div(y)
        );
    }
}

fn demo_natural_checked_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!("(&{}).checked_div(&{}) = {:?}", x, y, (&x).checked_div(&y));
    }
}

fn benchmark_limbs_div_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_div_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| no_out!(limbs_div_limb(&xs, y)))],
    );
}

fn benchmark_limbs_div_limb_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("standard", &mut |(mut out, xs, y)| {
                limbs_div_limb_to_out(&mut out, &xs, y)
            }),
            ("alt", &mut |(mut out, xs, y)| {
                limbs_div_limb_to_out_alt(&mut out, &xs, y)
            }),
        ],
    );
}

fn benchmark_limbs_div_limb_in_place_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("standard", &mut |(mut xs, y)| {
                limbs_div_limb_in_place(&mut xs, y)
            }),
            ("alt", &mut |(mut xs, y)| {
                limbs_div_limb_in_place_alt(&mut xs, y)
            }),
        ],
    );
}

fn benchmark_limbs_div_divisor_of_limb_max_with_carry_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_divisor_of_limb_max_with_carry_to_out(&mut [Limb], &[Limb], Limb, Limb)",
        BenchmarkType::Single,
        large_type_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, divisor, carry)| {
            no_out!(limbs_div_divisor_of_limb_max_with_carry_to_out(
                &mut out, &xs, divisor, carry
            ))
        })],
    );
}

fn benchmark_limbs_div_divisor_of_limb_max_with_carry_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_divisor_of_limb_max_with_carry_in_place(&mut [Limb], Limb, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, divisor, carry)| {
            no_out!(limbs_div_divisor_of_limb_max_with_carry_in_place(
                &mut xs, divisor, carry
            ))
        })],
    );
}

fn benchmark_limbs_div_schoolbook_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("Schoolbook div/mod", &mut |(
                mut qs,
                mut ns,
                ds,
                inverse,
            )| {
                no_out!(limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
            }),
            ("Schoolbook div", &mut |(mut qs, mut ns, ds, inverse)| {
                no_out!(limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
            }),
        ],
    );
}

fn benchmark_limbs_div_divide_and_conquer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("Schoolbook div", &mut |(mut qs, mut ns, ds, inverse)| {
                no_out!(limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
            }),
            ("divide-and-conquer div/mod", &mut |(
                mut qs,
                mut ns,
                ds,
                inverse,
            )| {
                no_out!(limbs_div_mod_divide_and_conquer(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
            ("divide-and-conquer div", &mut |(
                mut qs,
                ns,
                ds,
                inverse,
            )| {
                no_out!(limbs_div_divide_and_conquer(&mut qs, &ns, &ds, inverse))
            }),
        ],
    );
}

fn benchmark_limbs_div_barrett_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_barrett(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        large_type_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("divide-and-conquer div", &mut |(
                mut qs,
                ns,
                ds,
                inverse,
            )| {
                no_out!(limbs_div_divide_and_conquer(&mut qs, &ns, &ds, inverse))
            }),
            ("Barrett div/mod", &mut |(mut qs, ns, ds, _)| {
                let mut rs = vec![0; ds.len()];
                let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                no_out!(limbs_div_mod_barrett(
                    &mut qs,
                    &mut rs,
                    &ns,
                    &ds,
                    &mut scratch
                ))
            }),
            ("Barrett div", &mut |(mut qs, ns, ds, _)| {
                let mut scratch = vec![0; limbs_div_barrett_scratch_len(ns.len(), ds.len())];
                no_out!(limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch))
            }),
        ],
    );
}

fn benchmark_limbs_div_schoolbook_approx_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_schoolbook_approx(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("Schoolbook", &mut |(mut qs, mut ns, ds, inverse)| {
                no_out!(limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
            }),
            ("Schoolbook approx", &mut |(mut qs, mut ns, ds, inverse)| {
                no_out!(limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse))
            }),
        ],
    );
}

fn benchmark_limbs_div_divide_and_conquer_approx_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_divide_and_conquer_approx(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("Schoolbook approx", &mut |(mut qs, mut ns, ds, inverse)| {
                no_out!(limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse))
            }),
            ("divide-and-conquer", &mut |(mut qs, ns, ds, inverse)| {
                no_out!(limbs_div_divide_and_conquer(&mut qs, &ns, &ds, inverse))
            }),
            ("divide-and-conquer approx", &mut |(
                mut qs,
                mut ns,
                ds,
                inverse,
            )| {
                no_out!(limbs_div_divide_and_conquer_approx(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
        ],
    );
}

fn benchmark_limbs_div_barrett_approx_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_barrett_approx(&mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("divide-and-conquer approx", &mut |(
                mut qs,
                mut ns,
                ds,
                _,
            )| {
                // recompute inverse to make benchmark fair
                let inverse = limbs_two_limb_inverse_helper(ds[ds.len() - 1], ds[ds.len() - 2]);
                no_out!(limbs_div_divide_and_conquer_approx(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
            ("Barrett", &mut |(mut qs, ns, ds, _)| {
                let mut scratch = vec![0; limbs_div_barrett_scratch_len(ns.len(), ds.len())];
                no_out!(limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch))
            }),
            ("Barrett approx", &mut |(mut qs, ns, ds, _)| {
                let mut scratch = vec![0; limbs_div_barrett_approx_scratch_len(ns.len(), ds.len())];
                no_out!(limbs_div_barrett_approx(&mut qs, &ns, &ds, &mut scratch))
            }),
        ],
    );
}

fn benchmark_limbs_div_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_div(&[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ns"),
        &mut [
            ("div_mod", &mut |(ns, ds)| no_out!(limbs_div_mod(&ns, &ds))),
            ("div", &mut |(ns, ds)| no_out!(limbs_div(&ns, &ds))),
        ],
    );
}

fn benchmark_limbs_div_to_out_balancing_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_to_out(&mut [Limb], &mut [Limb], &mut [Limb]) balancing",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_45().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &limbs_div_to_out_balancing_bucketer(),
        &mut [
            ("unbalanced", &mut |(mut qs, mut ns, mut ds)| {
                limbs_div_to_out_unbalanced(&mut qs, &mut ns, &mut ds)
            }),
            ("balanced", &mut |(mut qs, ns, ds)| {
                limbs_div_to_out_balanced(&mut qs, &ns, &ds)
            }),
        ],
    );
}

fn benchmark_limbs_div_to_out_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_to_out(&mut [Limb], &mut [Limb], &mut [Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_triple_gen_var_44().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ns"),
        &mut [
            (
                "limbs_div_to_out(&mut [Limb], &mut [Limb], &mut [Limb])",
                &mut |(mut qs, mut ns, mut ds)| limbs_div_to_out(&mut qs, &mut ns, &mut ds),
            ),
            (
                "limbs_div_to_out_val_ref(&mut [Limb], &mut [Limb], &[Limb])",
                &mut |(mut qs, mut ns, ds)| limbs_div_to_out_val_ref(&mut qs, &mut ns, &ds),
            ),
            (
                "limbs_div_to_out_ref_val(&mut [Limb], &[Limb], &mut [Limb])",
                &mut |(mut qs, ns, mut ds)| limbs_div_to_out_ref_val(&mut qs, &ns, &mut ds),
            ),
            (
                "limbs_div_to_out_ref_ref(&mut [Limb], &[Limb], &[Limb])",
                &mut |(mut qs, ns, ds)| limbs_div_to_out_ref_ref(&mut qs, &ns, &ds),
            ),
        ],
    );
}

fn benchmark_limbs_div_to_out_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_to_out_ref_ref(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("ns"),
        &mut [
            ("div_mod", &mut |(mut qs, mut rs, ns, ds)| {
                limbs_div_mod_to_out(&mut qs, &mut rs, &ns, &ds)
            }),
            ("div", &mut |(mut qs, _, ns, ds)| {
                limbs_div_to_out_ref_ref(&mut qs, &ns, &ds)
            }),
        ],
    );
}

fn benchmark_natural_div_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural /= Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural /= Natural", &mut |(mut x, y)| x /= y),
            ("Natural /= &Natural", &mut |(mut x, y)| x /= &y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_div_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural / Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x / y)),
            ("num", &mut |((x, y), _, _)| no_out!(x / &y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x / y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_div_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural / Natural",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x / y)),
            ("using div_mod", &mut |(x, y)| no_out!(x.div_mod(y).0)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural / Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural / Natural", &mut |(x, y)| no_out!(x / y)),
            ("Natural / &Natural", &mut |(x, y)| no_out!(x / &y)),
            ("&Natural / Natural", &mut |(x, y)| no_out!(&x / y)),
            ("&Natural / &Natural", &mut |(x, y)| no_out!(&x / &y)),
        ],
    );
}

fn benchmark_natural_checked_div_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_div(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.checked_div(&y))),
            ("num", &mut |((x, y), _)| no_out!(x.checked_div(&y))),
        ],
    );
}

fn benchmark_natural_checked_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_div(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.checked_div(Natural)", &mut |(x, y)| {
                no_out!(x.checked_div(y))
            }),
            ("Natural.checked_div(&Natural)", &mut |(x, y)| {
                no_out!(x.checked_div(&y))
            }),
            ("(&Natural).checked_div(Natural)", &mut |(x, y)| {
                no_out!((&x).checked_div(y))
            }),
            ("(&Natural).checked_div(&Natural)", &mut |(x, y)| {
                no_out!((&x).checked_div(&y))
            }),
        ],
    );
}
