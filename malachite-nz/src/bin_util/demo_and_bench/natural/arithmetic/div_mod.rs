// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem, DivRound,
    NegMod,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, quadruple_2_3_diff_vec_len_bucketer, quadruple_2_vec_len_bucketer,
    quadruple_3_vec_len_bucketer, quadruple_4_vec_len_bucketer, quintuple_1_vec_len_bucketer,
    triple_2_3_diff_vec_len_bucketer, triple_2_vec_len_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen_var_12, unsigned_pair_gen_var_35, unsigned_vec_pair_gen_var_11,
    unsigned_vec_triple_gen_var_50, unsigned_vec_triple_gen_var_51, unsigned_vec_triple_gen_var_52,
    unsigned_vec_triple_gen_var_53, unsigned_vec_unsigned_pair_gen_var_22,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::div_mod::{
    limbs_div_barrett_large_product, limbs_div_limb_in_place_mod, limbs_div_limb_mod,
    limbs_div_limb_to_out_mod, limbs_div_mod, limbs_div_mod_barrett, limbs_div_mod_barrett_helper,
    limbs_div_mod_barrett_large_helper, limbs_div_mod_barrett_scratch_len,
    limbs_div_mod_by_two_limb_normalized, limbs_div_mod_divide_and_conquer, limbs_div_mod_extra,
    limbs_div_mod_extra_in_place, limbs_div_mod_schoolbook, limbs_div_mod_three_limb_by_two_limb,
    limbs_div_mod_to_out, limbs_invert_approx, limbs_invert_basecase_approx, limbs_invert_limb,
    limbs_invert_newton_approx, limbs_two_limb_inverse_helper,
};
use malachite_nz::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::{
    limbs_div_mod_barrett_helper_bucketer, limbs_div_mod_barrett_product_bucketer,
    limbs_div_mod_extra_bucketer, pair_1_natural_bit_bucketer, pair_2_pair_1_natural_bit_bucketer,
    triple_3_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    large_type_gen_var_11, large_type_gen_var_12, large_type_gen_var_18, large_type_gen_var_19,
    large_type_gen_var_20, natural_pair_gen_var_5, natural_pair_gen_var_5_nrm,
    natural_pair_gen_var_5_rm, unsigned_sextuple_gen_var_2, unsigned_vec_quadruple_gen_var_1,
    unsigned_vec_quadruple_gen_var_4, unsigned_vec_quadruple_gen_var_5,
};
use malachite_nz::test_util::natural::arithmetic::div_mod::{
    limbs_div_limb_in_place_mod_alt, limbs_div_limb_in_place_mod_naive,
    limbs_div_limb_to_out_mod_alt, limbs_div_limb_to_out_mod_naive, rug_ceiling_div_neg_mod,
};
use num::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_invert_limb);
    register_demo!(runner, demo_limbs_div_limb_mod);
    register_demo!(runner, demo_limbs_div_limb_to_out_mod);
    register_demo!(runner, demo_limbs_div_limb_in_place_mod);
    register_demo!(runner, demo_limbs_div_mod_extra);
    register_demo!(runner, demo_limbs_div_mod_extra_in_place);
    register_demo!(runner, demo_limbs_two_limb_inverse_helper);
    register_demo!(runner, demo_limbs_div_mod_three_limb_by_two_limb);
    register_demo!(runner, demo_limbs_div_mod_by_two_limb_normalized);
    register_demo!(runner, demo_limbs_div_mod_schoolbook);
    register_demo!(runner, demo_limbs_div_mod_divide_and_conquer);
    register_demo!(runner, demo_limbs_invert_basecase_approx);
    register_demo!(runner, demo_limbs_invert_newton_approx);
    register_demo!(runner, demo_limbs_invert_approx);
    register_demo!(runner, demo_limbs_div_mod_barrett);
    register_demo!(runner, demo_limbs_div_mod);
    register_demo!(runner, demo_limbs_div_mod_to_out);
    register_demo!(runner, demo_natural_div_assign_mod);
    register_demo!(runner, demo_natural_div_assign_mod_ref);
    register_demo!(runner, demo_natural_div_mod);
    register_demo!(runner, demo_natural_div_mod_val_ref);
    register_demo!(runner, demo_natural_div_mod_ref_val);
    register_demo!(runner, demo_natural_div_mod_ref_ref);
    register_demo!(runner, demo_natural_div_assign_rem);
    register_demo!(runner, demo_natural_div_assign_rem_ref);
    register_demo!(runner, demo_natural_div_rem);
    register_demo!(runner, demo_natural_div_rem_val_ref);
    register_demo!(runner, demo_natural_div_rem_ref_val);
    register_demo!(runner, demo_natural_div_rem_ref_ref);
    register_demo!(runner, demo_natural_ceiling_div_assign_neg_mod);
    register_demo!(runner, demo_natural_ceiling_div_assign_neg_mod_ref);
    register_demo!(runner, demo_natural_ceiling_div_neg_mod);
    register_demo!(runner, demo_natural_ceiling_div_neg_mod_val_ref);
    register_demo!(runner, demo_natural_ceiling_div_neg_mod_ref_val);
    register_demo!(runner, demo_natural_ceiling_div_neg_mod_ref_ref);

    register_bench!(runner, benchmark_limbs_invert_limb);
    register_bench!(runner, benchmark_limbs_div_limb_mod);
    register_bench!(runner, benchmark_limbs_div_limb_to_out_mod_algorithms);
    register_bench!(runner, benchmark_limbs_div_limb_in_place_mod_algorithms);
    register_bench!(runner, benchmark_limbs_div_mod_extra);
    register_bench!(runner, benchmark_limbs_div_mod_extra_in_place);
    register_bench!(runner, benchmark_limbs_div_mod_by_two_limb_normalized);
    register_bench!(runner, benchmark_limbs_div_mod_schoolbook);
    register_bench!(
        runner,
        benchmark_limbs_div_mod_divide_and_conquer_algorithms
    );
    register_bench!(runner, benchmark_limbs_invert_basecase_approx);
    register_bench!(runner, benchmark_limbs_invert_newton_approx_algorithms);
    register_bench!(runner, benchmark_limbs_invert_approx_algorithms);
    register_bench!(runner, benchmark_limbs_div_mod_barrett);
    register_bench!(
        runner,
        benchmark_limbs_div_mod_divide_and_conquer_to_barrett_algorithms
    );
    register_bench!(runner, benchmark_limbs_div_mod_barrett_product_algorithms);
    register_bench!(runner, benchmark_limbs_div_mod_barrett_helper_algorithms);
    register_bench!(runner, benchmark_limbs_div_mod);
    register_bench!(runner, benchmark_limbs_div_mod_to_out);
    register_bench!(runner, benchmark_natural_div_assign_mod_evaluation_strategy);
    register_bench!(runner, benchmark_natural_div_mod_library_comparison);
    register_bench!(runner, benchmark_natural_div_mod_algorithms);
    register_bench!(runner, benchmark_natural_div_mod_evaluation_strategy);
    register_bench!(runner, benchmark_natural_div_assign_rem_evaluation_strategy);
    register_bench!(runner, benchmark_natural_div_rem_library_comparison);
    register_bench!(runner, benchmark_natural_div_rem_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_natural_ceiling_div_assign_neg_mod_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_ceiling_div_neg_mod_library_comparison
    );
    register_bench!(runner, benchmark_natural_ceiling_div_neg_mod_algorithms);
    register_bench!(
        runner,
        benchmark_natural_ceiling_div_neg_mod_evaluation_strategy
    );
}

fn demo_limbs_invert_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen_var_12().get(gm, config).take(limit) {
        println!("limbs_invert_limb({}) = {}", x, limbs_invert_limb(x));
    }
}

fn demo_limbs_div_limb_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_div_limb_mod({:?}, {}) = {:?}",
            xs,
            y,
            limbs_div_limb_mod(&xs, y)
        );
    }
}

fn demo_limbs_div_limb_to_out_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        let remainder = limbs_div_limb_to_out_mod(&mut out, &xs, y);
        println!(
            "out := {out_old:?}; limbs_div_limb_to_out_mod(&mut out, {xs:?}, {y}) = {remainder}; \
             out = {out:?}",
        );
    }
}

fn demo_limbs_div_limb_in_place_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let remainder = limbs_div_limb_in_place_mod(&mut xs, y);
        println!(
            "limbs := {xs_old:?}; limbs_div_limb_in_place_mod(&mut limbs, {y}) = {remainder}; \
            limbs = {xs:?}",
        );
    }
}

fn demo_limbs_div_mod_extra(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, fraction_len, ns, d, d_inv, shift) in
        large_type_gen_var_19().get(gm, config).take(limit)
    {
        let out_old = out.clone();
        let remainder = limbs_div_mod_extra(&mut out, fraction_len, &ns, d, d_inv, shift);
        println!(
            "out := {out_old:?}; \
            limbs_div_mod_extra(&mut out, {fraction_len}, {ns:?}, {d}, {d_inv}, {shift}) = \
            {remainder}; out = {out:?}",
        );
    }
}

fn demo_limbs_div_mod_extra_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ns, fraction_len, d, d_inv, shift) in
        large_type_gen_var_18().get(gm, config).take(limit)
    {
        let ns_old = ns.clone();
        let remainder = limbs_div_mod_extra_in_place(&mut ns, fraction_len, d, d_inv, shift);
        println!(
            "ns := {ns_old:?}; \
            limbs_div_mod_extra_in_place(&mut ns, {fraction_len}, {d}, {d_inv}, {shift}) = \
            {remainder}; ns = {ns:?}",
        );
    }
}

fn demo_limbs_two_limb_inverse_helper(gm: GenMode, config: &GenConfig, limit: usize) {
    for (hi, lo) in unsigned_pair_gen_var_35().get(gm, config).take(limit) {
        println!(
            "limbs_two_limb_inverse_helper({}, {}) = {}",
            hi,
            lo,
            limbs_two_limb_inverse_helper(hi, lo)
        );
    }
}

fn demo_limbs_div_mod_three_limb_by_two_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n2, n1, n0, d1, d0, inverse) in unsigned_sextuple_gen_var_2().get(gm, config).take(limit) {
        println!(
            "limbs_div_mod_three_limb_by_two_limb({}, {}, {}, {}, {}, {}) = {:?}",
            n2,
            n1,
            n0,
            d1,
            d0,
            inverse,
            limbs_div_mod_three_limb_by_two_limb(n2, n1, n0, d1, d0, inverse)
        );
    }
}

fn demo_limbs_div_mod_by_two_limb_normalized(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds) in unsigned_vec_triple_gen_var_53().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, &ds);
        println!(
            "qs := {old_qs:?}; ns := {old_ns:?}; \
             limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, {ds:?}) = {highest_q}; \
             qs = {qs:?}, ns = {ns:?}",
        );
    }
}

fn demo_limbs_div_mod_schoolbook(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_11().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {old_qs:?}; \
            ns := {old_ns:?}; \
            limbs_div_mod_schoolbook(&mut qs, &mut ns, {ds:?}, {inverse}) = {highest_q}; \
             qs = {qs:?}, ns = {ns:?}",
        );
    }
}

fn demo_limbs_div_mod_divide_and_conquer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in large_type_gen_var_12().get(gm, config).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {old_qs:?}; ns := {old_ns:?}; \
             limbs_div_mod_divide_and_conquer(&mut qs, &mut ns, {ds:?}, {inverse}) = {highest_q}; \
             qs = {qs:?}, ns = {ns:?}",
        );
    }
}

fn demo_limbs_invert_basecase_approx(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut is, ds, mut scratch) in unsigned_vec_triple_gen_var_50().get(gm, config).take(limit) {
        let old_is = is.clone();
        let old_scratch = scratch.clone();
        let result_definitely_exact = limbs_invert_basecase_approx(&mut is, &ds, &mut scratch);
        println!(
            "is := {old_is:?}; scratch := {old_scratch:?}; \
             limbs_invert_basecase_approx(&mut is, {ds:?}, &mut scratch) = \
             {result_definitely_exact}; \
             is = {is:?}, scratch = {scratch:?}",
        );
    }
}

fn demo_limbs_invert_newton_approx(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut is, ds, mut scratch) in unsigned_vec_triple_gen_var_51().get(gm, config).take(limit) {
        let old_is = is.clone();
        let old_scratch = scratch.clone();
        let result_definitely_exact = limbs_invert_newton_approx(&mut is, &ds, &mut scratch);
        println!(
            "is := {old_is:?}; scratch := {old_scratch:?}; \
             limbs_invert_newton_approx(&mut is, {ds:?}, &mut scratch) = \
             {result_definitely_exact}; \
             is = {is:?}, scratch = {scratch:?}",
        );
    }
}

fn demo_limbs_invert_approx(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut is, ds, mut scratch) in unsigned_vec_triple_gen_var_50().get(gm, config).take(limit) {
        let old_is = is.clone();
        let old_scratch = scratch.clone();
        let result_definitely_exact = limbs_invert_approx(&mut is, &ds, &mut scratch);
        println!(
            "is := {old_is:?}; scratch := {old_scratch:?}; \
             limbs_invert_approx(&mut is, {ds:?}, &mut scratch) = {result_definitely_exact}; \
             is = {is:?}, scratch = {scratch:?}",
        );
    }
}

fn demo_limbs_div_mod_barrett(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut rs, ns, ds) in unsigned_vec_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let old_qs = qs.clone();
        let old_rs = rs.clone();
        let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
        let highest_q = limbs_div_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
        println!(
            "qs := {old_qs:?}; \
            rs := {old_rs:?}; \
            limbs_div_mod_barrett(&mut qs, &mut ns, {ns:?}, {ds:?}) = {highest_q}; \
             qs = {qs:?}, rs = {rs:?}",
        );
    }
}

fn demo_limbs_div_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, ds) in unsigned_vec_pair_gen_var_11().get(gm, config).take(limit) {
        println!(
            "limbs_div_mod({:?}, {:?}) = {:?}",
            ns,
            ds,
            limbs_div_mod(&ns, &ds)
        );
    }
}

fn demo_limbs_div_mod_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut qs, mut rs, ns, ds) in unsigned_vec_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_qs = qs.clone();
        let old_rs = rs.clone();
        limbs_div_mod_to_out(&mut qs, &mut rs, &ns, &ds);
        println!(
            "qs := {old_qs:?}; \
            rs := {old_rs:?}; limbs_div_mod_to_out(&mut qs, &mut ns, {ns:?}, {ds:?}); \
             qs = {qs:?}, rs = {rs:?}",
        );
    }
}

fn demo_natural_div_assign_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.div_assign_mod(y);
        println!("x := {x_old}; x.div_assign_mod({y_old}) = {remainder}; x = {x}");
    }
}

fn demo_natural_div_assign_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let remainder = x.div_assign_mod(&y);
        println!("x := {x_old}; x.div_assign_mod(&{y}) = {remainder}; x = {x}");
    }
}

fn demo_natural_div_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_mod({}) = {:?}", x_old, y_old, x.div_mod(y));
    }
}

fn demo_natural_div_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.div_mod(&{}) = {:?}", x_old, y, x.div_mod(&y));
    }
}

fn demo_natural_div_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_mod({}) = {:?}", x, y_old, (&x).div_mod(y));
    }
}

fn demo_natural_div_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        println!("(&{}).div_mod(&{}) = {:?}", x, y, (&x).div_mod(&y));
    }
}

fn demo_natural_div_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.div_assign_rem(y);
        println!("x := {x_old}; x.div_assign_rem({y_old}) = {remainder}; x = {x}");
    }
}

fn demo_natural_div_assign_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let remainder = x.div_assign_rem(&y);
        println!("x := {x_old}; x.div_assign_rem(&{y}) = {remainder}; x = {x}");
    }
}

fn demo_natural_div_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_rem({}) = {:?}", x_old, y_old, x.div_rem(y));
    }
}

fn demo_natural_div_rem_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.div_rem(&{}) = {:?}", x_old, y, x.div_rem(&y));
    }
}

fn demo_natural_div_rem_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_rem({}) = {:?}", x, y_old, (&x).div_rem(y));
    }
}

fn demo_natural_div_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        println!("(&{}).div_rem(&{}) = {:?}", x, y, (&x).div_rem(&y));
    }
}

fn demo_natural_ceiling_div_assign_neg_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.ceiling_div_assign_neg_mod(y);
        println!("x := {x_old}; x.ceiling_div_assign_neg_mod({y_old}) = {remainder}; x = {x}");
    }
}

fn demo_natural_ceiling_div_assign_neg_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let remainder = x.ceiling_div_assign_neg_mod(&y);
        println!("x := {x_old}; x.ceiling_div_assign_neg_mod(&{y}) = {remainder}; x = {x}");
    }
}

fn demo_natural_ceiling_div_neg_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.ceiling_div_neg_mod({}) = {:?}",
            x_old,
            y_old,
            x.ceiling_div_neg_mod(y)
        );
    }
}

fn demo_natural_ceiling_div_neg_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.ceiling_div_neg_mod(&{}) = {:?}",
            x_old,
            y,
            x.ceiling_div_neg_mod(&y)
        );
    }
}

fn demo_natural_ceiling_div_neg_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).ceiling_div_neg_mod({}) = {:?}",
            x,
            y_old,
            (&x).ceiling_div_neg_mod(y)
        );
    }
}

fn demo_natural_ceiling_div_neg_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "(&{}).ceiling_div_neg_mod(&{}) = {:?}",
            x,
            y,
            (&x).ceiling_div_neg_mod(&y)
        );
    }
}

fn benchmark_limbs_invert_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_invert_limb(Limb)",
        BenchmarkType::Single,
        unsigned_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(limbs_invert_limb(x)))],
    );
}

fn benchmark_limbs_div_limb_mod(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_div_limb_mod(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_div_limb_mod(&xs, y))
        })],
    );
}

fn benchmark_limbs_div_limb_to_out_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_limb_to_out_mod(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("standard", &mut |(mut out, xs, y)| {
                no_out!(limbs_div_limb_to_out_mod(&mut out, &xs, y))
            }),
            ("alt", &mut |(mut out, xs, y)| {
                no_out!(limbs_div_limb_to_out_mod_alt(&mut out, &xs, y))
            }),
            ("naive", &mut |(mut out, xs, y)| {
                no_out!(limbs_div_limb_to_out_mod_naive(&mut out, &xs, y))
            }),
        ],
    );
}

fn benchmark_limbs_div_limb_in_place_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_limb_in_place_mod(&mut [Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("standard", &mut |(mut xs, y)| {
                no_out!(limbs_div_limb_in_place_mod(&mut xs, y))
            }),
            ("alt", &mut |(mut xs, y)| {
                no_out!(limbs_div_limb_in_place_mod_alt(&mut xs, y))
            }),
            ("naive", &mut |(mut xs, y)| {
                no_out!(limbs_div_limb_in_place_mod_naive(&mut xs, y))
            }),
        ],
    );
}

fn benchmark_limbs_div_mod_extra(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_div_mod_extra(&mut [Limb], usize, &[Limb], Limb, Limb, u64)",
        BenchmarkType::Single,
        large_type_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &limbs_div_mod_extra_bucketer(),
        &mut [("Malachite", &mut |(
            mut out,
            fraction_len,
            ns,
            d,
            d_inv,
            shift,
        )| {
            no_out!(limbs_div_mod_extra(
                &mut out,
                fraction_len,
                &ns,
                d,
                d_inv,
                shift
            ))
        })],
    );
}

fn benchmark_limbs_div_mod_extra_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_mod_extra(&mut [Limb], usize, Limb, Limb, u64)",
        BenchmarkType::Single,
        large_type_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_vec_len_bucketer("out"),
        &mut [("Malachite", &mut |(
            mut ns,
            fraction_len,
            d,
            d_inv,
            shift,
        )| {
            no_out!(limbs_div_mod_extra_in_place(
                &mut ns,
                fraction_len,
                d,
                d_inv,
                shift
            ))
        })],
    );
}

fn benchmark_limbs_div_mod_by_two_limb_normalized(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_mod_by_two_limb_normalized(&mut [Limb], &mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_53().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ns"),
        &mut [("Malachite", &mut |(mut qs, mut ns, ds)| {
            no_out!(limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, &ds))
        })],
    );
}

fn benchmark_limbs_div_mod_schoolbook(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_mod_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        large_type_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("ns"),
        &mut [("Malachite", &mut |(mut qs, mut ns, ds, inverse)| {
            no_out!(limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
        })],
    );
}

// use large params
fn benchmark_limbs_div_mod_divide_and_conquer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_mod_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        large_type_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("Schoolbook", &mut |(mut qs, mut ns, ds, inverse)| {
                no_out!(limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
            }),
            ("divide-and-conquer", &mut |(
                mut qs,
                mut ns,
                ds,
                inverse,
            )| {
                no_out!(limbs_div_mod_divide_and_conquer(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
        ],
    );
}

fn benchmark_limbs_invert_basecase_approx(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_invert_basecase_approx(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_50().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ds"),
        &mut [("Malachite", &mut |(mut is, ds, mut scratch)| {
            no_out!(limbs_invert_basecase_approx(&mut is, &ds, &mut scratch))
        })],
    );
}

// use very large params
fn benchmark_limbs_invert_newton_approx_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_invert_newton_approx(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_51().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ds"),
        &mut [
            ("basecase", &mut |(mut is, ds, mut scratch)| {
                no_out!(limbs_invert_basecase_approx(&mut is, &ds, &mut scratch))
            }),
            ("Newton", &mut |(mut is, ds, mut scratch)| {
                no_out!(limbs_invert_newton_approx(&mut is, &ds, &mut scratch))
            }),
        ],
    );
}

// use very large params
fn benchmark_limbs_invert_approx_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_invert_approx(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_50().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("ds"),
        &mut [
            ("basecase", &mut |(mut is, ds, mut scratch)| {
                no_out!(limbs_invert_basecase_approx(&mut is, &ds, &mut scratch))
            }),
            ("default", &mut |(mut is, ds, mut scratch)| {
                no_out!(limbs_invert_approx(&mut is, &ds, &mut scratch))
            }),
        ],
    );
}

fn benchmark_limbs_div_mod_barrett(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_div_mod_barrett(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Single,
        unsigned_vec_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_4_vec_len_bucketer("ds"),
        &mut [("Malachite", &mut |(mut qs, mut rs, ns, ds)| {
            let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
            no_out!(limbs_div_mod_barrett(
                &mut qs,
                &mut rs,
                &ns,
                &ds,
                &mut scratch
            ))
        })],
    );
}

// use very large params
fn benchmark_limbs_div_mod_divide_and_conquer_to_barrett_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_mod_barrett(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_52::<Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_diff_vec_len_bucketer("ns", "ds"),
        &mut [
            ("divide-and-conquer", &mut |(mut qs, mut ns, mut ds)| {
                let q_len = ns.len() - ds.len() + 1;
                ds[q_len - 1].set_bit(Limb::WIDTH - 1);
                let inverse = limbs_two_limb_inverse_helper(ds[q_len - 1], ds[q_len - 2]);
                no_out!(limbs_div_mod_divide_and_conquer(
                    &mut qs,
                    &mut ns[..q_len << 1],
                    &ds[..q_len],
                    inverse
                ))
            }),
            ("Barrett", &mut |(mut qs, mut ns, mut ds)| {
                let d_len = ds.len();
                let mut rs = vec![0; d_len];
                let q_len = ns.len() - d_len + 1;
                let q_len_2 = q_len << 1;
                ds[q_len - 1].set_bit(Limb::WIDTH - 1);
                limbs_two_limb_inverse_helper(ds[q_len - 1], ds[q_len - 2]);
                let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(q_len_2, q_len)];
                limbs_div_mod_barrett(&mut qs, &mut rs, &ns[..q_len_2], &ds[..q_len], &mut scratch);
                ns[..q_len].copy_from_slice(&rs[..q_len]);
            }),
        ],
    );
}

// use large params
fn benchmark_limbs_div_mod_barrett_product_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_barrett_large_product(&mut [Limb], &[Limb], &[Limb], &[Limb], usize, usize)",
        BenchmarkType::Algorithms,
        large_type_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &limbs_div_mod_barrett_product_bucketer(),
        &mut [
            ("limbs_mul_greater_to_out", &mut |(
                mut scratch,
                ds,
                qs,
                _,
                _,
                _,
            )| {
                let mut mul_scratch =
                    vec![0; limbs_mul_greater_to_out_scratch_len(ds.len(), qs.len())];
                no_out!(limbs_mul_greater_to_out(
                    &mut scratch,
                    &ds,
                    &qs,
                    &mut mul_scratch
                ))
            }),
            ("limbs_div_barrett_large_product", &mut |(
                mut scratch,
                ds,
                qs,
                rs_hi,
                scratch_len,
                i_len,
            )| {
                limbs_div_barrett_large_product(&mut scratch, &ds, &qs, &rs_hi, scratch_len, i_len)
            }),
        ],
    );
}

fn benchmark_limbs_div_mod_barrett_helper_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_div_mod_barrett_helper(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &limbs_div_mod_barrett_helper_bucketer(),
        &mut [
            ("limbs_div_mod_barrett_helper", &mut |(
                mut qs,
                mut rs,
                ns,
                ds,
            )| {
                let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                let q_len = ns.len() - ds.len();
                no_out!(limbs_div_mod_barrett_helper(
                    &mut qs[..q_len],
                    &mut rs[..ds.len()],
                    &ns,
                    &ds,
                    &mut scratch
                ))
            }),
            ("limbs_div_mod_barrett_large_helper", &mut |(
                mut qs,
                mut rs,
                ns,
                ds,
            )| {
                let mut scratch = vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                let q_len = ns.len() - ds.len();
                no_out!(limbs_div_mod_barrett_large_helper(
                    &mut qs[..q_len],
                    &mut rs[..ds.len()],
                    &ns,
                    &ds,
                    &mut scratch
                ))
            }),
        ],
    );
}

fn benchmark_limbs_div_mod(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_div_mod(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("ns"),
        &mut [("Malachite", &mut |(ns, ds)| {
            no_out!(limbs_div_mod(&ns, &ds))
        })],
    );
}

fn benchmark_limbs_div_mod_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_div_mod_to_out(&mut [Limb], &mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("ns"),
        &mut [("Malachite", &mut |(mut qs, mut rs, ns, ds)| {
            limbs_div_mod_to_out(&mut qs, &mut rs, &ns, &ds)
        })],
    );
}

fn benchmark_natural_div_assign_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_assign_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.div_assign_mod(Natural)", &mut |(mut x, y)| {
                no_out!(x.div_assign_mod(y))
            }),
            ("Natural.div_mod(&Natural)", &mut |(mut x, y)| {
                no_out!(x.div_assign_mod(&y))
            }),
        ],
    );
}

fn benchmark_natural_div_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_mod(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.div_mod(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.div_mod_floor(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.div_rem_floor(y))),
        ],
    );
}

#[allow(clippy::no_effect)]
fn benchmark_natural_div_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_mod(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.div_mod(y))),
            ("using / and %", &mut |(x, y)| no_out!((&x / &y, x % y))),
        ],
    );
}

fn benchmark_natural_div_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.div_mod(Natural)", &mut |(x, y)| {
                no_out!(x.div_mod(y))
            }),
            ("Natural.div_mod(&Natural)", &mut |(x, y)| {
                no_out!(x.div_mod(&y))
            }),
            ("(&Natural).div_mod(Natural)", &mut |(x, y)| {
                no_out!((&x).div_mod(y))
            }),
            ("(&Natural).div_mod(&Natural)", &mut |(x, y)| {
                no_out!((&x).div_mod(&y))
            }),
        ],
    );
}

fn benchmark_natural_div_assign_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_assign_rem(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.div_assign_rem(Natural)", &mut |(mut x, y)| {
                no_out!(x.div_assign_rem(y))
            }),
            ("Natural.div_assign_rem(&Natural)", &mut |(mut x, y)| {
                no_out!(x.div_assign_rem(&y))
            }),
        ],
    );
}

fn benchmark_natural_div_rem_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_rem(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.div_rem(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.div_rem(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.div_rem(y))),
        ],
    );
}

fn benchmark_natural_div_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.div_rem(Natural)", &mut |(x, y)| {
                no_out!(x.div_rem(y))
            }),
            ("Natural.div_rem(&Natural)", &mut |(x, y)| {
                no_out!(x.div_rem(&y))
            }),
            ("(&Natural).div_rem(Natural)", &mut |(x, y)| {
                no_out!((&x).div_rem(y))
            }),
            ("(&Natural).div_rem(&Natural)", &mut |(x, y)| {
                no_out!((&x).div_rem(&y))
            }),
        ],
    );
}

fn benchmark_natural_ceiling_div_assign_neg_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_div_assign_neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            (
                "Natural.ceiling_div_assign_neg_mod(Natural)",
                &mut |(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(y)),
            ),
            (
                "Natural.ceiling_div_assign_neg_mod(&Natural)",
                &mut |(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(&y)),
            ),
        ],
    );
}

fn benchmark_natural_ceiling_div_neg_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_div_neg_mod(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.ceiling_div_neg_mod(y))
            }),
            ("rug", &mut |((x, y), _)| {
                no_out!(rug_ceiling_div_neg_mod(x, y))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_natural_ceiling_div_neg_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_div_neg_mod(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ("using div_round and %", &mut |(x, y)| {
                ((&x).div_round(&y, Ceiling), x.neg_mod(y));
            }),
        ],
    );
}

fn benchmark_natural_ceiling_div_neg_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_div_neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.ceiling_div_neg_mod(Natural)", &mut |(x, y)| {
                no_out!(x.ceiling_div_neg_mod(y))
            }),
            ("Natural.ceiling_div_neg_mod(&Natural)", &mut |(x, y)| {
                no_out!(x.ceiling_div_neg_mod(&y))
            }),
            ("(&Natural).ceiling_div_neg_mod(Natural)", &mut |(x, y)| {
                no_out!((&x).ceiling_div_neg_mod(y))
            }),
            ("(&Natural).ceiling_div_neg_mod(&Natural)", &mut |(x, y)| {
                no_out!((&x).ceiling_div_neg_mod(&y))
            }),
        ],
    );
}
