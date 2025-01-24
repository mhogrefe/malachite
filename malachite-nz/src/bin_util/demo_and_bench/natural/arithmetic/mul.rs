// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_sum_vec_len_bucketer, quadruple_2_vec_len_bucketer,
    triple_1_vec_len_bucketer, triple_2_3_sum_vec_len_bucketer, triple_2_vec_len_bucketer,
    triple_3_vec_len_bucketer, vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    large_type_gen_var_1, unsigned_pair_gen_var_27, unsigned_vec_gen_var_6,
    unsigned_vec_pair_gen_var_1, unsigned_vec_pair_gen_var_2, unsigned_vec_triple_gen_var_1,
    unsigned_vec_triple_gen_var_2, unsigned_vec_triple_gen_var_24, unsigned_vec_triple_gen_var_25,
    unsigned_vec_triple_gen_var_26, unsigned_vec_triple_gen_var_27, unsigned_vec_triple_gen_var_3,
    unsigned_vec_unsigned_pair_gen, unsigned_vec_unsigned_unsigned_triple_gen,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::fft::{
    limbs_mul_greater_to_out_fft, limbs_mul_greater_to_out_fft_scratch_len,
    limbs_square_to_out_fft, limbs_square_to_out_fft_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb, limbs_mul_limb_to_out, limbs_mul_limb_with_carry_to_out,
    limbs_slice_mul_limb_in_place, limbs_slice_mul_limb_with_carry_in_place,
    limbs_vec_mul_limb_in_place,
};
use malachite_nz::natural::arithmetic::mul::mul_low::{
    limbs_mul_low_same_length, limbs_mul_low_same_length_basecase,
    limbs_mul_low_same_length_basecase_alt, limbs_mul_low_same_length_divide_and_conquer,
    limbs_mul_low_same_length_divide_and_conquer_scratch_len,
    limbs_mul_low_same_length_divide_and_conquer_shared_scratch, limbs_mul_low_same_length_large,
};
use malachite_nz::natural::arithmetic::mul::product_of_limbs::limbs_product;
use malachite_nz::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_22, limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    limbs_mul_greater_to_out_toom_22_scratch_len, limbs_mul_greater_to_out_toom_32,
    limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    limbs_mul_greater_to_out_toom_32_scratch_len, limbs_mul_greater_to_out_toom_33,
    limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    limbs_mul_greater_to_out_toom_33_scratch_len, limbs_mul_greater_to_out_toom_42,
    limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    limbs_mul_greater_to_out_toom_42_scratch_len, limbs_mul_greater_to_out_toom_43,
    limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    limbs_mul_greater_to_out_toom_43_scratch_len, limbs_mul_greater_to_out_toom_44,
    limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    limbs_mul_greater_to_out_toom_44_scratch_len, limbs_mul_greater_to_out_toom_52,
    limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    limbs_mul_greater_to_out_toom_52_scratch_len, limbs_mul_greater_to_out_toom_53,
    limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    limbs_mul_greater_to_out_toom_53_scratch_len, limbs_mul_greater_to_out_toom_54,
    limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    limbs_mul_greater_to_out_toom_54_scratch_len, limbs_mul_greater_to_out_toom_62,
    limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    limbs_mul_greater_to_out_toom_62_scratch_len, limbs_mul_greater_to_out_toom_63,
    limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    limbs_mul_greater_to_out_toom_63_scratch_len, limbs_mul_greater_to_out_toom_6h,
    limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    limbs_mul_greater_to_out_toom_6h_scratch_len, limbs_mul_greater_to_out_toom_8h,
    limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
    limbs_mul_greater_to_out_toom_8h_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::{
    limbs_mul, limbs_mul_greater, limbs_mul_greater_to_out, limbs_mul_greater_to_out_basecase,
    limbs_mul_greater_to_out_scratch_len, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_natural_max_bit_bucketer, pair_natural_max_bit_bucketer,
    triple_3_pair_natural_max_bit_bucketer, triple_3_vec_natural_sum_bits_bucketer,
    vec_natural_sum_bits_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_pair_gen, natural_pair_gen_nrm, natural_pair_gen_rm, natural_vec_gen,
    natural_vec_gen_nrm, unsigned_vec_pair_gen_var_33, unsigned_vec_triple_gen_var_10,
    unsigned_vec_triple_gen_var_11, unsigned_vec_triple_gen_var_12, unsigned_vec_triple_gen_var_13,
    unsigned_vec_triple_gen_var_14, unsigned_vec_triple_gen_var_15, unsigned_vec_triple_gen_var_16,
    unsigned_vec_triple_gen_var_18, unsigned_vec_triple_gen_var_19, unsigned_vec_triple_gen_var_20,
    unsigned_vec_triple_gen_var_22, unsigned_vec_triple_gen_var_23, unsigned_vec_triple_gen_var_4,
    unsigned_vec_triple_gen_var_5, unsigned_vec_triple_gen_var_58, unsigned_vec_triple_gen_var_6,
    unsigned_vec_triple_gen_var_60, unsigned_vec_triple_gen_var_7, unsigned_vec_triple_gen_var_8,
    unsigned_vec_triple_gen_var_9,
};
use malachite_nz::test_util::natural::arithmetic::mul::natural_product_naive;
use malachite_nz::test_util::natural::arithmetic::mul::{
    limbs_mul_greater_to_out_basecase_mem_opt, limbs_product_naive,
};
use num::BigUint;
use std::iter::Product;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mul_greater_to_out_fft);
    register_demo!(runner, demo_limbs_square_to_out_fft);
    register_demo!(runner, demo_limbs_mul_limb);
    register_demo!(runner, demo_limbs_mul_limb_with_carry_to_out);
    register_demo!(runner, demo_limbs_mul_limb_to_out);
    register_demo!(runner, demo_limbs_slice_mul_limb_with_carry_in_place);
    register_demo!(runner, demo_limbs_slice_mul_limb_in_place);
    register_demo!(runner, demo_limbs_vec_mul_limb_in_place);
    register_demo!(runner, demo_limbs_mul_greater);
    register_demo!(runner, demo_limbs_mul);
    register_demo!(runner, demo_limbs_mul_same_length_to_out);
    register_demo!(runner, demo_limbs_mul_greater_to_out);
    register_demo!(runner, demo_limbs_mul_to_out);
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_22_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_32_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_33_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_42_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_43_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_44_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_52_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_53_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_54_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_62_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_63_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_6h_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_8h_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_33_and_toom_44_input_sizes_valid
    );
    register_demo!(runner, demo_limbs_mul_low_same_length_basecase);
    register_demo!(
        runner,
        demo_limbs_mul_low_same_length_divide_and_conquer_shared_scratch
    );
    register_demo!(runner, demo_limbs_mul_low_same_length_divide_and_conquer);
    register_demo!(runner, demo_limbs_mul_low_same_length);
    register_demo!(runner, demo_limbs_product);
    register_demo!(runner, demo_natural_mul);
    register_demo!(runner, demo_natural_mul_val_ref);
    register_demo!(runner, demo_natural_mul_ref_val);
    register_demo!(runner, demo_natural_mul_ref_ref);
    register_demo!(runner, demo_natural_mul_assign);
    register_demo!(runner, demo_natural_mul_assign_ref);
    register_demo!(runner, demo_natural_product);
    register_demo!(runner, demo_natural_ref_product);

    register_bench!(runner, benchmark_limbs_mul_limb);
    register_bench!(runner, benchmark_limbs_mul_limb_with_carry_to_out);
    register_bench!(runner, benchmark_limbs_mul_limb_to_out);
    register_bench!(runner, benchmark_limbs_slice_mul_limb_with_carry_in_place);
    register_bench!(runner, benchmark_limbs_slice_mul_limb_in_place);
    register_bench!(runner, benchmark_limbs_vec_mul_limb_in_place);
    register_bench!(runner, benchmark_limbs_mul_greater);
    register_bench!(runner, benchmark_limbs_mul);
    register_bench!(runner, benchmark_limbs_mul_same_length_to_out);
    register_bench!(runner, benchmark_limbs_mul_greater_to_out_algorithms);
    register_bench!(runner, benchmark_limbs_mul_to_out);
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_basecase_mem_opt_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_22_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_32_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_33_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_33_same_length_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_42_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_43_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_44_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_44_same_length_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_52_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_53_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_54_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_62_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_63_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_6h_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_6h_same_length_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_8h_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_8h_same_length_algorithms
    );
    register_bench!(runner, benchmark_limbs_mul_greater_to_out_fft_algorithms);
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_32_to_43_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_42_to_53_algorithms
    );
    register_bench!(runner, benchmark_limbs_mul_fft_alt_algorithms);
    register_bench!(
        runner,
        benchmark_limbs_mul_low_same_length_basecase_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_low_same_length_basecase_algorithms_2
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_low_same_length_divide_and_conquer_shared_scratch_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_low_same_length_divide_and_conquer_algorithms
    );
    register_bench!(runner, benchmark_limbs_mul_low_same_length_large_algorithms);
    register_bench!(runner, benchmark_limbs_mul_low_same_length_algorithms);
    register_bench!(runner, benchmark_limbs_product_algorithms);
    register_bench!(runner, benchmark_natural_mul_library_comparison);
    register_bench!(runner, benchmark_natural_mul_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mul_assign_library_comparison);
    register_bench!(runner, benchmark_natural_mul_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_product_algorithms);
    register_bench!(runner, benchmark_natural_product_library_comparison);
    register_bench!(runner, benchmark_natural_product_evaluation_strategy);
}

fn demo_limbs_mul_greater_to_out_fft(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_60().get(gm, config).take(limit) {
        let out_old = out.clone();
        let mut scratch = vec![0; limbs_mul_greater_to_out_fft_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_fft(&mut out, &xs, &ys, &mut scratch);
        println!(
            "out := {out_old:?}; \
            limbs_mul_greater_to_out_fft(&mut out, {xs:?}, {ys:?}); out = {out:?}",
        );
    }
}

fn demo_limbs_square_to_out_fft(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs) in unsigned_vec_pair_gen_var_33().get(gm, config).take(limit) {
        let out_old = out.clone();
        let mut scratch = vec![0; limbs_square_to_out_fft_scratch_len(xs.len())];
        limbs_square_to_out_fft(&mut out, &xs, &mut scratch);
        println!("out := {out_old:?}; limbs_square_to_out_fft(&mut out, {xs:?}); out = {out:?}");
    }
}

fn demo_limbs_mul_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen().get(gm, config).take(limit) {
        println!(
            "limbs_mul_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_mul_limb(&xs, y)
        );
    }
}

fn demo_limbs_mul_limb_with_carry_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y, carry) in large_type_gen_var_1().get(gm, config).take(limit) {
        let out_old = out.clone();
        let carry_out = limbs_mul_limb_with_carry_to_out(&mut out, &xs, y, carry);
        println!(
            "out := {out_old:?}; \
            limbs_mul_limb_with_carry_to_out(&mut out, {xs:?}, {y}, {carry}) = {carry_out}; \
             out = {out:?}",
        );
    }
}

fn demo_limbs_mul_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        let carry = limbs_mul_limb_to_out(&mut out, &xs, y);
        println!(
            "out := {out_old:?}; \
            limbs_mul_limb_to_out(&mut out, {xs:?}, {y}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_slice_mul_limb_with_carry_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y, carry) in unsigned_vec_unsigned_unsigned_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry_out = limbs_slice_mul_limb_with_carry_in_place(&mut xs, y, carry);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_mul_limb_with_carry_in_place(&mut xs, {y}, {carry}) = {carry_out}; \
            xs = {xs:?}",
        );
    }
}

fn demo_limbs_slice_mul_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let carry = limbs_slice_mul_limb_in_place(&mut xs, y);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_mul_limb_in_place(&mut xs, {y}) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_mul_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_vec_mul_limb_in_place(&mut xs, y);
        println!("xs := {xs_old:?}; limbs_vec_mul_limb_in_place(&mut xs, {y}); xs = {xs:?}");
    }
}

fn demo_limbs_mul_greater(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_mul_greater({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_mul_greater(&xs, &ys)
        );
    }
}

fn demo_limbs_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_2().get(gm, config).take(limit) {
        println!("limbs_mul({:?}, {:?}) = {:?}", xs, ys, limbs_mul(&xs, &ys));
    }
}

fn demo_limbs_mul_same_length_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_1().get(gm, config).take(limit) {
        let out_old = out.clone();
        let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(xs.len())];
        limbs_mul_same_length_to_out(&mut out, &xs, &ys, &mut mul_scratch);
        println!(
            "out := {out_old:?}; \
            limbs_mul_same_length_to_out(&mut out, {xs:?}, {ys:?}); out = {out:?}",
        );
    }
}

fn demo_limbs_mul_greater_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_2().get(gm, config).take(limit) {
        let out_old = out.clone();
        let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xs.len(), ys.len())];
        let carry = limbs_mul_greater_to_out(&mut out, &xs, &ys, &mut mul_scratch);
        println!(
            "out := {out_old:?}; \
            limbs_mul_greater_to_out(&mut out, {xs:?}, {ys:?}) = {carry}; out = {out:?}",
        );
    }
}

fn demo_limbs_mul_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_3().get(gm, config).take(limit) {
        let out_old = out.clone();
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(xs.len(), ys.len())];
        let carry = limbs_mul_to_out(&mut out, &xs, &ys, &mut mul_scratch);
        println!(
            "out := {out_old:?}; \
            limbs_mul_to_out(&mut out, {xs:?}, {ys:?}) = {carry}; out = {out:?}",
        );
    }
}

macro_rules! mul_valid_helper {
    ($name: ident, $demo_name: ident) => {
        fn $demo_name(gm: GenMode, config: &GenConfig, limit: usize) {
            for (x, y) in unsigned_pair_gen_var_27().get(gm, config).take(limit) {
                println!(
                    concat!(stringify!($name), "({}, {}) = {}"),
                    x,
                    y,
                    $name(x, y)
                );
            }
        }
    };
}
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_22_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_32_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_33_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_42_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_43_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_44_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_52_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_53_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_54_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_62_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_63_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_6h_input_sizes_valid
);
mul_valid_helper!(
    limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_8h_input_sizes_valid
);

fn demo_limbs_mul_greater_to_out_toom_33_and_toom_44_input_sizes_valid(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in unsigned_pair_gen_var_27().get(gm, config).take(limit) {
        println!(
            concat!("Toom-33 and Toom-44 ({}, {}) = {}"),
            x,
            y,
            limbs_mul_greater_to_out_toom_33_input_sizes_valid(x, y)
                && limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_low_same_length_basecase(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_24().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_mul_low_same_length_basecase(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_mul_low_same_length_basecase(&mut out, {xs:?}, {ys:?}); out = {out:?}",
        );
    }
}

fn demo_limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_25().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
             limbs_mul_low_same_length_divide_and_conquer_shared_scratch\
             (&mut out, {xs:?}, {ys:?}); \
             out = {out:?}",
        );
    }
}

fn demo_limbs_mul_low_same_length_divide_and_conquer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_26().get(gm, config).take(limit) {
        let out_old = out.clone();
        let mut scratch = vec![0; xs.len() << 1];
        limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch);
        println!(
            "out := {out_old:?}; \
            limbs_mul_low_same_length_divide_and_conquer(&mut out, {xs:?}, {ys:?}); \
             out = {out:?}",
        );
    }
}

fn demo_limbs_mul_low_same_length(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_1().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_mul_low_same_length(&mut out, &xs, &ys);
        println!(
            "out := {out_old:?}; \
            limbs_mul_low_same_length(&mut out, {xs:?}, {ys:?}); out = {out:?}",
        );
    }
}

fn demo_limbs_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut xs in unsigned_vec_gen_var_6().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let mut out = vec![0; xs.len() + 1];
        let out_len = limbs_product(&mut out, &mut xs);
        out.truncate(out_len);
        println!("product_of_limbs({xs_old:?}) = {out:?}");
    }
}

fn demo_natural_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

fn demo_natural_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

fn demo_natural_mul_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

fn demo_natural_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

fn demo_natural_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {x_old}; x *= {y}; x = {x}");
    }
}

fn demo_natural_mul_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {x_old}; x *= &{y}; x = {x}");
    }
}

fn demo_natural_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in natural_vec_gen().get(gm, config).take(limit) {
        println!(
            "product({:?}) = {}",
            xs.clone(),
            Natural::product(xs.into_iter())
        );
    }
}

fn demo_natural_ref_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in natural_vec_gen().get(gm, config).take(limit) {
        println!("product({:?}) = {}", xs, Natural::product(xs.iter()));
    }
}

fn benchmark_limbs_mul_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| no_out!(limbs_mul_limb(&xs, y)))],
    );
}

fn benchmark_limbs_mul_limb_with_carry_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_limb_with_carry_to_out(&mut [Limb], &[Limb], Limb, Limb)",
        BenchmarkType::Single,
        large_type_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y, carry)| {
            no_out!(limbs_mul_limb_with_carry_to_out(&mut out, &xs, y, carry))
        })],
    );
}

fn benchmark_limbs_mul_limb_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            no_out!(limbs_mul_limb_to_out(&mut out, &xs, y))
        })],
    );
}

fn benchmark_limbs_slice_mul_limb_with_carry_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mul_limb_with_carry_in_place(&mut [Limb], Limb, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y, carry)| {
            no_out!(limbs_slice_mul_limb_with_carry_in_place(&mut xs, y, carry))
        })],
    );
}

fn benchmark_limbs_slice_mul_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mul_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            no_out!(limbs_slice_mul_limb_in_place(&mut xs, y))
        })],
    );
}

fn benchmark_limbs_vec_mul_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_mul_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            limbs_vec_mul_limb_in_place(&mut xs, y)
        })],
    );
}

fn benchmark_limbs_mul_greater(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_greater(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| {
            no_out!(limbs_mul_greater(&xs, &ys))
        })],
    );
}

fn benchmark_limbs_mul(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| no_out!(limbs_mul(&xs, &ys)))],
    );
}

fn benchmark_limbs_mul_same_length_to_out(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(xs.len())];
            limbs_mul_same_length_to_out(&mut out, &xs, &ys, &mut mul_scratch)
        })],
    );
}

fn benchmark_limbs_mul_greater_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("full", &mut |(mut out, xs, ys)| {
                let mut mul_scratch =
                    vec![0; limbs_mul_greater_to_out_scratch_len(xs.len(), ys.len())];
                no_out!(limbs_mul_greater_to_out(
                    &mut out,
                    &xs,
                    &ys,
                    &mut mul_scratch
                ))
            }),
        ],
    );
}

fn benchmark_limbs_mul_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(xs.len(), ys.len())];
            no_out!(limbs_mul_to_out(&mut out, &xs, &ys, &mut mul_scratch))
        })],
    );
}

fn benchmark_limbs_mul_greater_to_out_basecase_mem_opt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_basecase_mem_opt(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("limbs_mul_greater_to_out_basecase", &mut |(
                mut out,
                xs,
                ys,
            )| {
                limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            (
                "limbs_mul_greater_to_out_basecase_mem_opt",
                &mut |(mut out, xs, ys)| {
                    limbs_mul_greater_to_out_basecase_mem_opt(&mut out, &xs, &ys)
                },
            ),
        ],
    );
}

macro_rules! bench_mul_helper {
    ($bench: ident, $mul: ident, $scratch: ident, $gen: ident, $name: expr) => {
        fn $bench(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
            run_benchmark(
                concat!(stringify!($mul), "(&mut [Limb], &[Limb], &[Limb])"),
                BenchmarkType::Algorithms,
                $gen().get(gm, config),
                gm.name(),
                limit,
                file_name,
                &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
                &mut [
                    ("basecase", &mut |(mut out, xs, ys)| {
                        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
                    }),
                    ($name, &mut |(mut out, xs, ys)| {
                        let mut scratch = vec![0; $scratch(xs.len(), ys.len())];
                        $mul(&mut out, &xs, &ys, &mut scratch)
                    }),
                ],
            );
        }
    };
}
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_22_algorithms,
    limbs_mul_greater_to_out_toom_22,
    limbs_mul_greater_to_out_toom_22_scratch_len,
    unsigned_vec_triple_gen_var_4,
    "Toom22"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_32_algorithms,
    limbs_mul_greater_to_out_toom_32,
    limbs_mul_greater_to_out_toom_32_scratch_len,
    unsigned_vec_triple_gen_var_5,
    "Toom32"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_33_algorithms,
    limbs_mul_greater_to_out_toom_33,
    limbs_mul_greater_to_out_toom_33_scratch_len,
    unsigned_vec_triple_gen_var_6,
    "Toom33"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_42_algorithms,
    limbs_mul_greater_to_out_toom_42,
    limbs_mul_greater_to_out_toom_42_scratch_len,
    unsigned_vec_triple_gen_var_7,
    "Toom42"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_43_algorithms,
    limbs_mul_greater_to_out_toom_43,
    limbs_mul_greater_to_out_toom_43_scratch_len,
    unsigned_vec_triple_gen_var_8,
    "Toom43"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_44_algorithms,
    limbs_mul_greater_to_out_toom_44,
    limbs_mul_greater_to_out_toom_44_scratch_len,
    unsigned_vec_triple_gen_var_9,
    "Toom44"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_52_algorithms,
    limbs_mul_greater_to_out_toom_52,
    limbs_mul_greater_to_out_toom_52_scratch_len,
    unsigned_vec_triple_gen_var_10,
    "Toom52"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_54_algorithms,
    limbs_mul_greater_to_out_toom_54,
    limbs_mul_greater_to_out_toom_54_scratch_len,
    unsigned_vec_triple_gen_var_12,
    "Toom54"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_62_algorithms,
    limbs_mul_greater_to_out_toom_62,
    limbs_mul_greater_to_out_toom_62_scratch_len,
    unsigned_vec_triple_gen_var_13,
    "Toom62"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_6h_algorithms,
    limbs_mul_greater_to_out_toom_6h,
    limbs_mul_greater_to_out_toom_6h_scratch_len,
    unsigned_vec_triple_gen_var_15,
    "Toom6h"
);
bench_mul_helper!(
    benchmark_limbs_mul_greater_to_out_toom_8h_algorithms,
    limbs_mul_greater_to_out_toom_8h,
    limbs_mul_greater_to_out_toom_8h_scratch_len,
    unsigned_vec_triple_gen_var_16,
    "Toom8h"
);

fn benchmark_limbs_mul_greater_to_out_toom_53_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_toom_53(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom32", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom42", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom53", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_63_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_toom_63(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom42", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom63", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_fft_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_fft(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("FFT", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_fft_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_fft(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_33_same_length_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_toom_33(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom22", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_22_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom33", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_44_same_length_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_toom_44(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_58().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom33", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom44", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_6h_same_length_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_toom_6h(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom33", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom44", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom6h", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_8h_same_length_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_toom_8h(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom6h", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom8h", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_32_to_43_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Toom32 to Toom43",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_vec_len_bucketer("ys"),
        &mut [
            ("Toom32", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom43", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_43_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_42_to_53_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Toom42 to Toom53",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_23().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_vec_len_bucketer("ys"),
        &mut [
            ("Toom42", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom53", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_fft_alt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_fft_alt(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom8h", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch)
            }),
            ("FFT", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_greater_to_out_fft_scratch_len(xs.len(), ys.len())];
                limbs_mul_greater_to_out_fft(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_basecase_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_basecase(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("standard", &mut |(mut out, xs, ys)| {
                limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)
            }),
            ("alt", &mut |(mut out, xs, ys)| {
                limbs_mul_low_same_length_basecase_alt(&mut out, &xs, &ys)
            }),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_basecase_algorithms_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_basecase(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase mul", &mut |(mut out, xs, ys)| {
                limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("basecase mul low", &mut |(mut out, xs, ys)| {
                limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)
            }),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_divide_and_conquer_shared_scratch_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_divide_and_conquer_shared_scratch\
         (&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_25().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)
            }),
            ("divide-and-conquer", &mut |(mut out, xs, ys)| {
                limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &xs, &ys)
            }),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_divide_and_conquer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_divide_and_conquer(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_26().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)
            }),
            ("divide-and-conquer", &mut |(mut out, xs, ys)| {
                let mut scratch = vec![0; ys.len() << 1];
                limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_large_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_large(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("mul low divide-and-conquer", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())];
                limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch)
            }),
            ("mul low large", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())];
                limbs_mul_low_same_length_large(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("mul low", &mut |(mut out, xs, ys)| {
                limbs_mul_low_same_length(&mut out, &xs, &ys)
            }),
            ("mul", &mut |(mut out, xs, ys)| {
                let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(xs.len())];
                limbs_mul_same_length_to_out(&mut out, &xs, &ys, &mut mul_scratch)
            }),
        ],
    );
}

fn benchmark_limbs_product_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_product(&mut [Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |mut xs| {
                let mut out = vec![0; xs.len()];
                let out_len = limbs_product(&mut out, &mut xs);
                out.truncate(out_len);
            }),
            ("naive", &mut |xs| {
                let mut out = vec![0; xs.len()];
                let out_len = limbs_product_naive(&mut out, &xs);
                out.truncate(out_len);
            }),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_mul_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural * Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x * y)),
            ("num", &mut |((x, y), _, _)| no_out!(x * y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x * y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural * Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural * Natural", &mut |(x, y)| no_out!(x * y)),
            ("Natural * &Natural", &mut |(x, y)| no_out!(x * &y)),
            ("&Natural * Natural", &mut |(x, y)| no_out!(&x * y)),
            ("&Natural * &Natural", &mut |(x, y)| no_out!(&x * &y)),
        ],
    );
}

fn benchmark_natural_mul_assign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural *= Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(_, (mut x, y))| x *= y), ("rug", &mut |((mut x, y), _)| x *= y)],
    );
}

fn benchmark_natural_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural *= Natural",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural *= Natural", &mut |(mut x, y)| no_out!(x *= y)),
            ("Natural *= &Natural", &mut |(mut x, y)| no_out!(x *= &y)),
        ],
    );
}

fn benchmark_natural_product_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::product(Iterator<Item=Natural>)",
        BenchmarkType::LibraryComparison,
        natural_vec_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_vec_natural_sum_bits_bucketer(),
        &mut [
            ("Malachite", &mut |(_, _, xs)| {
                no_out!(Natural::product(xs.into_iter()))
            }),
            ("num", &mut |(xs, _, _)| {
                no_out!(BigUint::product(xs.into_iter()))
            }),
            ("rug", &mut |(_, xs, _)| {
                no_out!(rug::Integer::product(xs.iter()))
            }),
        ],
    );
}

fn benchmark_natural_product_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::product(Iterator<Item=Natural>)",
        BenchmarkType::Algorithms,
        natural_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_natural_sum_bits_bucketer(),
        &mut [
            ("default", &mut |xs| {
                no_out!(Natural::product(xs.into_iter()))
            }),
            ("naive", &mut |xs| {
                no_out!(natural_product_naive(xs.into_iter()))
            }),
        ],
    );
}

fn benchmark_natural_product_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::product(Iterator<Item=Natural>)",
        BenchmarkType::EvaluationStrategy,
        natural_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_natural_sum_bits_bucketer(),
        &mut [
            ("Natural::product(Iterator<Item=Natural>)", &mut |xs| {
                no_out!(Natural::product(xs.into_iter()))
            }),
            ("Natural::product(Iterator<Item=&Natural>)", &mut |xs| {
                no_out!(Natural::product(xs.iter()))
            }),
        ],
    );
}
