// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivMod;
use malachite_base::num::arithmetic::traits::{
    CeilingDivNegMod, Mod, ModAssign, NegMod, NegModAssign,
};
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_1_vec_len_sub_1_bucketer, quadruple_2_3_diff_vec_len_bucketer,
    quadruple_2_vec_len_bucketer, quadruple_3_vec_len_bucketer, quadruple_4_vec_len_bucketer,
    triple_2_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_24;
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_11, unsigned_vec_pair_gen_var_18, unsigned_vec_triple_gen_var_57,
    unsigned_vec_unsigned_pair_gen_var_22, unsigned_vec_unsigned_pair_gen_var_25,
    unsigned_vec_unsigned_pair_gen_var_26, unsigned_vec_unsigned_pair_gen_var_27,
    unsigned_vec_unsigned_pair_gen_var_28,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::div_mod::{
    limbs_div_mod_barrett, limbs_div_mod_barrett_scratch_len, limbs_div_mod_by_two_limb_normalized,
    limbs_div_mod_divide_and_conquer, limbs_div_mod_schoolbook, limbs_div_mod_to_out,
};
use malachite_nz::natural::arithmetic::mod_op::{
    limbs_mod, limbs_mod_barrett, limbs_mod_by_two_limb_normalized, limbs_mod_divide_and_conquer,
    limbs_mod_limb, limbs_mod_limb_alt_1, limbs_mod_limb_alt_2, limbs_mod_limb_any_leading_zeros,
    limbs_mod_limb_any_leading_zeros_1, limbs_mod_limb_any_leading_zeros_2,
    limbs_mod_limb_at_least_1_leading_zero, limbs_mod_limb_at_least_2_leading_zeros,
    limbs_mod_limb_small_normalized, limbs_mod_limb_small_normalized_large,
    limbs_mod_limb_small_small, limbs_mod_limb_small_unnormalized,
    limbs_mod_limb_small_unnormalized_large, limbs_mod_schoolbook,
    limbs_mod_three_limb_by_two_limb, limbs_mod_to_out,
};
use malachite_nz::test_util::bench::bucketers::{
    limbs_mod_limb_small_unnormalized_bucketer, pair_1_natural_bit_bucketer,
    pair_2_pair_1_natural_bit_bucketer, triple_3_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    large_type_gen_var_11, large_type_gen_var_12, natural_pair_gen_var_5,
    natural_pair_gen_var_5_nrm, natural_pair_gen_var_5_rm, unsigned_sextuple_gen_var_2,
    unsigned_vec_quadruple_gen_var_1, unsigned_vec_quadruple_gen_var_5,
    unsigned_vec_triple_gen_var_56, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_17,
};
use malachite_nz::test_util::natural::arithmetic::mod_op::{limbs_mod_limb_alt_3, rug_neg_mod};
use num::Integer;
use rug::ops::RemRounding;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mod_limb);
    register_demo!(runner, demo_limbs_mod_limb_small_normalized);
    register_demo!(runner, demo_limbs_mod_limb_small_unnormalized);
    register_demo!(runner, demo_limbs_mod_limb_any_leading_zeros_1);
    register_demo!(runner, demo_limbs_mod_limb_any_leading_zeros_2);
    register_demo!(runner, demo_limbs_mod_limb_at_least_1_leading_zero);
    register_demo!(runner, demo_limbs_mod_limb_at_least_2_leading_zeros);
    register_demo!(runner, demo_limbs_mod_three_limb_by_two_limb);
    register_demo!(runner, demo_limbs_mod_by_two_limb_normalized);
    register_demo!(runner, demo_limbs_mod_schoolbook);
    register_demo!(runner, demo_limbs_mod_divide_and_conquer);
    register_demo!(runner, demo_limbs_mod_barrett);
    register_demo!(runner, demo_limbs_mod);
    register_demo!(runner, demo_limbs_mod_to_out);
    register_demo!(runner, demo_natural_mod_assign);
    register_demo!(runner, demo_natural_mod_assign_ref);
    register_demo!(runner, demo_natural_mod);
    register_demo!(runner, demo_natural_mod_val_ref);
    register_demo!(runner, demo_natural_mod_ref_val);
    register_demo!(runner, demo_natural_mod_ref_ref);
    register_demo!(runner, demo_natural_rem_assign);
    register_demo!(runner, demo_natural_rem_assign_ref);
    register_demo!(runner, demo_natural_rem);
    register_demo!(runner, demo_natural_rem_val_ref);
    register_demo!(runner, demo_natural_rem_ref_val);
    register_demo!(runner, demo_natural_rem_ref_ref);
    register_demo!(runner, demo_natural_neg_mod_assign);
    register_demo!(runner, demo_natural_neg_mod_assign_ref);
    register_demo!(runner, demo_natural_neg_mod);
    register_demo!(runner, demo_natural_neg_mod_val_ref);
    register_demo!(runner, demo_natural_neg_mod_ref_val);
    register_demo!(runner, demo_natural_neg_mod_ref_ref);

    register_bench!(runner, benchmark_limbs_mod_limb_algorithms);
    register_bench!(runner, benchmark_limbs_mod_limb_small_normalized_algorithms);
    register_bench!(
        runner,
        benchmark_limbs_mod_limb_small_unnormalized_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mod_limb_any_leading_zeros_from_normalized_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mod_limb_any_leading_zeros_from_unnormalized_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mod_limb_at_least_1_leading_zero_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mod_limb_at_least_2_leading_zeros_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mod_by_two_limb_normalized_algorithms
    );
    register_bench!(runner, benchmark_limbs_mod_schoolbook_algorithms);
    register_bench!(runner, benchmark_limbs_mod_divide_and_conquer_algorithms);
    register_bench!(runner, benchmark_limbs_mod_barrett_algorithms);
    register_bench!(runner, benchmark_limbs_mod);
    register_bench!(runner, benchmark_limbs_mod_to_out_algorithms);
    register_bench!(runner, benchmark_natural_mod_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_library_comparison);
    register_bench!(runner, benchmark_natural_mod_algorithms);
    register_bench!(runner, benchmark_natural_mod_evaluation_strategy);
    register_bench!(runner, benchmark_natural_rem_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_rem_library_comparison);
    register_bench!(runner, benchmark_natural_rem_evaluation_strategy);
    register_bench!(runner, benchmark_natural_neg_mod_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_neg_mod_library_comparison);
    register_bench!(runner, benchmark_natural_neg_mod_algorithms);
    register_bench!(runner, benchmark_natural_neg_mod_evaluation_strategy);
}

fn demo_limbs_mod_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, d) in unsigned_vec_unsigned_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_limb({:?}, {}) = {}",
            ns,
            d,
            limbs_mod_limb(&ns, d)
        );
    }
}

fn demo_limbs_mod_limb_small_normalized(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, d) in unsigned_vec_unsigned_pair_gen_var_26()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_limb_small_normalized({:?}, {}) = {}",
            ns,
            d,
            limbs_mod_limb_small_normalized(&ns, d)
        );
    }
}

fn demo_limbs_mod_limb_small_unnormalized(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, d) in unsigned_vec_unsigned_pair_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_limb_small_unnormalized({:?}, {}) = {}",
            ns,
            d,
            limbs_mod_limb_small_unnormalized(&ns, d)
        );
    }
}

fn demo_limbs_mod_limb_any_leading_zeros_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, d) in unsigned_vec_unsigned_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_limb_any_leading_zeros_1({:?}, {}) = {}",
            ns,
            d,
            limbs_mod_limb_any_leading_zeros_1(&ns, d)
        );
    }
}

fn demo_limbs_mod_limb_any_leading_zeros_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, d) in unsigned_vec_unsigned_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_limb_any_leading_zeros_2({:?}, {}) = {}",
            ns,
            d,
            limbs_mod_limb_any_leading_zeros_2(&ns, d)
        );
    }
}

fn demo_limbs_mod_limb_at_least_1_leading_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, d) in unsigned_vec_unsigned_pair_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_limb_at_least_1_leading_zero({:?}, {}) = {}",
            ns,
            d,
            limbs_mod_limb_at_least_1_leading_zero(&ns, d)
        );
    }
}

fn demo_limbs_mod_limb_at_least_2_leading_zeros(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, d) in unsigned_vec_unsigned_pair_gen_var_28()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_limb_at_least_2_leading_zeros({:?}, {}) = {}",
            ns,
            d,
            limbs_mod_limb_at_least_2_leading_zeros(&ns, d)
        );
    }
}

fn demo_limbs_mod_three_limb_by_two_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n2, n1, n0, d1, d0, inverse) in unsigned_sextuple_gen_var_2().get(gm, config).take(limit) {
        println!(
            "limbs_mod_three_limb_by_two_limb({}, {}, {}, {}, {}, {}) = {}",
            n2,
            n1,
            n0,
            d1,
            d0,
            inverse,
            limbs_mod_three_limb_by_two_limb(n2, n1, n0, d1, d0, inverse)
        );
    }
}

fn demo_limbs_mod_by_two_limb_normalized(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, ds) in unsigned_vec_pair_gen_var_18().get(gm, config).take(limit) {
        println!(
            "limbs_mod_by_two_limb_normalized({:?}, {:?}) = {:?}",
            ns,
            ds,
            limbs_mod_by_two_limb_normalized(&ns, &ds),
        );
    }
}

fn demo_limbs_mod_schoolbook(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ns, ds, inverse) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let old_ns = ns.clone();
        limbs_mod_schoolbook(&mut ns, &ds, inverse);
        println!("ns := {old_ns:?}; limbs_mod_schoolbook(&mut ns, {ds:?}, {inverse}); ns = {ns:?}");
    }
}

fn demo_limbs_mod_divide_and_conquer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_12().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        limbs_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {old_qs:?}; ns := {old_ns:?}; \
             limbs_mod_divide_and_conquer(&mut qs, &mut ns, {ds:?}, {inverse}); ns = {ns:?}",
        );
    }
}

fn demo_limbs_mod_barrett(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut rs, ns, ds) in unsigned_vec_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let old_qs = qs.clone();
        let old_rs = rs.clone();
        let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
        limbs_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
        println!(
            "qs := {old_qs:?}; \
            rs := {old_rs:?}; limbs_mod_barrett(&mut qs, &mut ns, {ns:?}, {ds:?}); rs = {rs:?}",
        );
    }
}

fn demo_limbs_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, ds) in unsigned_vec_pair_gen_var_11().get(gm, config).take(limit) {
        println!("limbs_mod({:?}, {:?}) = {:?}", ns, ds, limbs_mod(&ns, &ds));
    }
}

fn demo_limbs_mod_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut rs, ns, ds) in unsigned_vec_triple_gen_var_57().get(gm, config).take(limit) {
        let old_rs = rs.clone();
        limbs_mod_to_out(&mut rs, &ns, &ds);
        println!("rs := {old_rs:?}; limbs_mod_to_out(&mut rs, {ns:?}, {ds:?}); rs = {rs:?}");
    }
}

fn demo_natural_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_assign(y);
        println!("x := {x_old}; x.mod_assign({y_old}); x = {x}");
    }
}

fn demo_natural_mod_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mod_assign(&y);
        println!("x := {x_old}; x.mod_assign(&{y}); x = {x}");
    }
}

fn demo_natural_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.mod_op({}) = {}", x_old, y_old, x.mod_op(y));
    }
}

fn demo_natural_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.mod_op(&{}) = {}", x_old, y, x.mod_op(&y));
    }
}

fn demo_natural_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).mod_op({}) = {:?}", x, y_old, (&x).mod_op(y));
    }
}

fn demo_natural_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        println!("(&{}).mod_op(&{}) = {:?}", x, y, (&x).mod_op(&y));
    }
}

fn demo_natural_rem_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x %= y;
        println!("x := {x_old}; x %= {y_old}; x = {x}");
    }
}

fn demo_natural_rem_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        x %= &y;
        println!("x := {x_old}; x %= &{y}; x = {x}");
    }
}

fn demo_natural_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} % {} = {:?}", x_old, y_old, x % y);
    }
}

fn demo_natural_rem_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} % &{} = {:?}", x_old, y, x % &y);
    }
}

fn demo_natural_rem_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} % {} = {:?}", x, y_old, &x % y);
    }
}

fn demo_natural_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        println!("&{} % &{} = {:?}", x, y, &x % &y);
    }
}

fn demo_natural_neg_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.neg_mod_assign(y);
        println!("x := {x_old}; x.neg_mod_assign({y_old}); x = {x}");
    }
}

fn demo_natural_neg_mod_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.neg_mod_assign(&y);
        println!("x := {x_old}; x.neg_mod_assign(&{y}); x = {x}");
    }
}

fn demo_natural_neg_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.neg_mod({}) = {}", x_old, y_old, x.neg_mod(y));
    }
}

fn demo_natural_neg_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.neg_mod(&{}) = {}", x_old, y, x.neg_mod(&y));
    }
}

fn demo_natural_neg_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).neg_mod({}) = {}", x, y_old, (&x).neg_mod(y));
    }
}

fn demo_natural_neg_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        println!("(&{}).neg_mod(&{}) = {}", x, y, (&x).neg_mod(&y));
    }
}

fn benchmark_limbs_mod_limb_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_limb(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ns"),
        &mut [
            ("alt 1", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_alt_1(&ns, d))
            }),
            ("alt 2", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_alt_2(&ns, d))
            }),
            ("alt 3", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_alt_3(&ns, d))
            }),
            ("limbs_mod_limb_any_leading_zeros_1", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_any_leading_zeros_1(&ns, d))
            }),
            ("limbs_mod_limb_any_leading_zeros_2", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_any_leading_zeros_2(&ns, d))
            }),
        ],
    );
}

fn benchmark_limbs_mod_limb_small_normalized_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_limb_small_normalized(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_26().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_sub_1_bucketer("ns"),
        &mut [
            ("small", &mut |(ns, d)| {
                let mut len = ns.len();
                let mut r = ns[len - 1];
                if r >= d {
                    r -= d;
                }
                len -= 1;
                if len == 0 {
                    return;
                }
                limbs_mod_limb_small_small(&ns[..len], d, r);
            }),
            ("large", &mut |(ns, d)| {
                let mut len = ns.len();
                let mut r = ns[len - 1];
                if r >= d {
                    r -= d;
                }
                len -= 1;
                if len == 0 {
                    return;
                }
                limbs_mod_limb_small_normalized_large(&ns[..len], d, r);
            }),
        ],
    );
}

fn benchmark_limbs_mod_limb_small_unnormalized_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_limb_small_unnormalized(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &limbs_mod_limb_small_unnormalized_bucketer(),
        &mut [
            ("small", &mut |(ns, d)| {
                let mut len = ns.len();
                let mut r = ns[len - 1];
                if r < d {
                    len -= 1;
                    if len == 0 {
                        return;
                    }
                } else {
                    r = 0;
                }
                limbs_mod_limb_small_small(&ns[..len], d, r);
            }),
            ("large", &mut |(ns, d)| {
                let mut len = ns.len();
                let mut r = ns[len - 1];
                if r < d {
                    len -= 1;
                    if len == 0 {
                        return;
                    }
                } else {
                    r = 0;
                }
                limbs_mod_limb_small_unnormalized_large(&ns[..len], d, r);
            }),
        ],
    );
}

fn benchmark_limbs_mod_limb_any_leading_zeros_from_normalized_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_limb_any_leading_zeros(&[Limb], Limb) from normalized",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ns"),
        &mut [
            ("limbs_mod_limb_small_normalized", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_small_normalized(&ns, d))
            }),
            ("limbs_mod_limb_any_leading_zeros", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_any_leading_zeros(&ns, d))
            }),
        ],
    );
}

fn benchmark_limbs_mod_limb_any_leading_zeros_from_unnormalized_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_limb_any_leading_zeros(&[Limb], Limb) from unnormalized",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_25().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ns"),
        &mut [
            ("limbs_mod_limb_small_unnormalized", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_small_unnormalized(&ns, d))
            }),
            ("limbs_mod_limb_any_leading_zeros", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_any_leading_zeros(&ns, d))
            }),
        ],
    );
}

fn benchmark_limbs_mod_limb_at_least_1_leading_zero_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_limb_at_least_1_leading_zero(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_25().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ns"),
        &mut [
            ("limbs_mod_limb_any_leading_zeros", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_any_leading_zeros(&ns, d))
            }),
            ("limbs_mod_limb_at_least_1_leading_zero", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_at_least_1_leading_zero(&ns, d))
            }),
        ],
    );
}

fn benchmark_limbs_mod_limb_at_least_2_leading_zeros_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_limb_at_least_2_leading_zeros(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_28().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ns"),
        &mut [
            ("Malachite", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_at_least_1_leading_zero(&ns, d))
            }),
            ("limbs_mod_limb_at_least_2_leading_zeros", &mut |(ns, d)| {
                no_out!(limbs_mod_limb_at_least_2_leading_zeros(&ns, d))
            }),
        ],
    );
}

fn benchmark_limbs_mod_by_two_limb_normalized_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_by_two_limb_normalized(&[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_56().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ns"),
        &mut [
            ("using div/mod", &mut |(mut qs, mut ns, ds)| {
                no_out!(limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, &ds))
            }),
            ("standard", &mut |(_, ns, ds)| {
                no_out!(limbs_mod_by_two_limb_normalized(&ns, &ds))
            }),
        ],
    );
}

fn benchmark_limbs_mod_schoolbook_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_schoolbook(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("ns"),
        &mut [
            ("using div/mod", &mut |(mut qs, mut ns, ds, inverse)| {
                no_out!(limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
            }),
            ("standard", &mut |(_, mut ns, ds, inverse)| {
                limbs_mod_schoolbook(&mut ns, &ds, inverse)
            }),
        ],
    );
}

// use large params
fn benchmark_limbs_mod_divide_and_conquer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("Schoolbook", &mut |(_, mut ns, ds, inverse)| {
                limbs_mod_schoolbook(&mut ns, &ds, inverse)
            }),
            ("divide-and-conquer using div/mod", &mut |(
                mut qs,
                mut ns,
                ds,
                inverse,
            )| {
                no_out!(limbs_div_mod_divide_and_conquer(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
            ("divide-and-conquer", &mut |(
                mut qs,
                mut ns,
                ds,
                inverse,
            )| {
                limbs_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse)
            }),
        ],
    );
}

fn benchmark_limbs_mod_barrett_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_barrett(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_4_vec_len_bucketer("ds"),
        &mut [
            ("Barrett using div/mod", &mut |(mut qs, mut rs, ns, ds)| {
                let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                limbs_div_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
            }),
            ("Barrett", &mut |(mut qs, mut rs, ns, ds)| {
                let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                limbs_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
            }),
        ],
    );
}

fn benchmark_limbs_mod(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mod(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ds"),
        &mut [("Malachite", &mut |(ns, ds)| no_out!(limbs_mod(&ns, &ds)))],
    );
}

fn benchmark_limbs_mod_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("ns"),
        &mut [
            ("using div/mod", &mut |(qs, mut rs, ns, ds)| {
                // Allocate again to make benchmark fair
                let mut qs = vec![0; qs.len()];
                limbs_div_mod_to_out(&mut qs, &mut rs, &ns, &ds);
            }),
            ("standard", &mut |(_, mut rs, ns, ds)| {
                limbs_mod_to_out(&mut rs, &ns, &ds)
            }),
        ],
    );
}

fn benchmark_natural_mod_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural.mod_assign(Natural)", &mut |(mut x, y)| {
                no_out!(x.mod_assign(y))
            }),
            ("Natural.mod_assign(&Natural)", &mut |(mut x, y)| {
                no_out!(x.mod_assign(&y))
            }),
        ],
    );
}

fn benchmark_natural_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.mod_op(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.mod_floor(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.rem_floor(y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_natural_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.mod_op(y))),
            ("using div_mod", &mut |(x, y)| no_out!(x.div_mod(y).1)),
        ],
    );
}

fn benchmark_natural_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            (
                "Natural.mod_op(Natural)",
                &mut |(x, y)| no_out!(x.mod_op(y)),
            ),
            ("Natural.mod_op(&Natural)", &mut |(x, y)| {
                no_out!(x.mod_op(&y))
            }),
            ("(&Natural).mod_op(Natural)", &mut |(x, y)| {
                no_out!((&x).mod_op(y))
            }),
            ("(&Natural).mod_op(&Natural)", &mut |(x, y)| {
                no_out!((&x).mod_op(&y))
            }),
        ],
    );
}

fn benchmark_natural_rem_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural %= Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural %= Natural", &mut |(mut x, y)| x %= y),
            ("Natural %= &Natural", &mut |(mut x, y)| x %= &y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_rem_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural % Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x % y)),
            ("num", &mut |((x, y), _, _)| no_out!(x % y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x % y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural % Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural % Natural", &mut |(x, y)| no_out!(x % y)),
            ("Natural % &Natural", &mut |(x, y)| no_out!(x % &y)),
            ("&Natural % Natural", &mut |(x, y)| no_out!(&x % y)),
            ("&Natural % &Natural", &mut |(x, y)| no_out!(&x % &y)),
        ],
    );
}

fn benchmark_natural_neg_mod_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.neg_mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural.neg_mod_assign(Natural)", &mut |(mut x, y)| {
                no_out!(x.neg_mod_assign(y))
            }),
            ("Natural.neg_mod_assign(&Natural)", &mut |(mut x, y)| {
                no_out!(x.neg_mod_assign(&y))
            }),
        ],
    );
}

fn benchmark_natural_neg_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.neg_mod(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.neg_mod(y))),
            ("rug", &mut |((x, y), _)| no_out!(rug_neg_mod(x, y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_natural_neg_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.neg_mod(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.neg_mod(y))),
            ("using ceiling_div_neg_mod", &mut |(x, y)| {
                no_out!(x.ceiling_div_neg_mod(y).1)
            }),
        ],
    );
}

fn benchmark_natural_neg_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("Natural.neg_mod(Natural)", &mut |(x, y)| {
                no_out!(x.neg_mod(y))
            }),
            ("Natural.neg_mod(&Natural)", &mut |(x, y)| {
                no_out!(x.neg_mod(&y))
            }),
            ("(&Natural).neg_mod(Natural)", &mut |(x, y)| {
                no_out!((&x).neg_mod(y))
            }),
            ("(&Natural).neg_mod(&Natural)", &mut |(x, y)| {
                no_out!((&x).neg_mod(&y))
            }),
        ],
    );
}
