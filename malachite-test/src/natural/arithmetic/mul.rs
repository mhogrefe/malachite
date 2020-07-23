use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::natural::arithmetic::mul::fft::{
    _limbs_mul_greater_to_out_fft, _limbs_mul_greater_to_out_fft_input_sizes_threshold,
};
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb, limbs_mul_limb_to_out, limbs_mul_limb_with_carry_to_out,
    limbs_slice_mul_limb_in_place, limbs_slice_mul_limb_with_carry_in_place,
    limbs_vec_mul_limb_in_place,
};
use malachite_nz::natural::arithmetic::mul::mul_low::{
    _limbs_mul_low_same_length_basecase, _limbs_mul_low_same_length_basecase_alt,
    _limbs_mul_low_same_length_divide_and_conquer,
    _limbs_mul_low_same_length_divide_and_conquer_scratch_len,
    _limbs_mul_low_same_length_divide_and_conquer_shared_scratch, _limbs_mul_low_same_length_large,
    limbs_mul_low_same_length,
};
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_22_scratch_len, _limbs_mul_greater_to_out_toom_32,
    _limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_32_scratch_len, _limbs_mul_greater_to_out_toom_33,
    _limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_33_scratch_len, _limbs_mul_greater_to_out_toom_42,
    _limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_42_scratch_len, _limbs_mul_greater_to_out_toom_43,
    _limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_43_scratch_len, _limbs_mul_greater_to_out_toom_44,
    _limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_44_scratch_len, _limbs_mul_greater_to_out_toom_52,
    _limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_52_scratch_len, _limbs_mul_greater_to_out_toom_53,
    _limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_53_scratch_len, _limbs_mul_greater_to_out_toom_54,
    _limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_54_scratch_len, _limbs_mul_greater_to_out_toom_62,
    _limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_62_scratch_len, _limbs_mul_greater_to_out_toom_63,
    _limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_63_scratch_len, _limbs_mul_greater_to_out_toom_6h,
    _limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_6h_scratch_len, _limbs_mul_greater_to_out_toom_8h,
    _limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_8h_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_greater_to_out_basecase, limbs_mul, limbs_mul_greater, limbs_mul_greater_to_out,
    limbs_mul_same_length_to_out, limbs_mul_to_out,
};
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::arithmetic::mul::_limbs_mul_greater_to_out_basecase_mem_opt;

use malachite_test::common::{
    DemoBenchRegistry, GenerationMode, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    pairs_of_small_unsigneds_single, pairs_of_unsigned_vec_and_unsigned,
    pairs_of_unsigned_vec_var_4, pairs_of_unsigned_vec_var_5,
    quadruples_of_unsigned_vec_unsigned_vec_unsigned_and_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1, triples_of_unsigned_vec_var_10,
    triples_of_unsigned_vec_var_11, triples_of_unsigned_vec_var_12, triples_of_unsigned_vec_var_13,
    triples_of_unsigned_vec_var_14, triples_of_unsigned_vec_var_15, triples_of_unsigned_vec_var_16,
    triples_of_unsigned_vec_var_17, triples_of_unsigned_vec_var_18, triples_of_unsigned_vec_var_19,
    triples_of_unsigned_vec_var_20, triples_of_unsigned_vec_var_21, triples_of_unsigned_vec_var_22,
    triples_of_unsigned_vec_var_23, triples_of_unsigned_vec_var_25, triples_of_unsigned_vec_var_26,
    triples_of_unsigned_vec_var_30, triples_of_unsigned_vec_var_31, triples_of_unsigned_vec_var_32,
    triples_of_unsigned_vec_var_33, triples_of_unsigned_vec_var_34, triples_of_unsigned_vec_var_35,
    triples_of_unsigned_vec_var_36, triples_of_unsigned_vec_var_46, triples_of_unsigned_vec_var_47,
    triples_of_unsigned_vec_var_48, triples_of_unsigned_vec_var_49, triples_of_unsigned_vec_var_52,
};
use malachite_test::inputs::natural::{
    nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mul_limb);
    register_demo!(registry, demo_limbs_mul_limb_with_carry_to_out);
    register_demo!(registry, demo_limbs_mul_limb_to_out);
    register_demo!(registry, demo_limbs_slice_mul_limb_with_carry_in_place);
    register_demo!(registry, demo_limbs_slice_mul_limb_in_place);
    register_demo!(registry, demo_limbs_vec_mul_limb_in_place);
    register_demo!(registry, demo_limbs_mul_greater);
    register_demo!(registry, demo_limbs_mul);
    register_demo!(registry, demo_limbs_mul_same_length_to_out);
    register_demo!(registry, demo_limbs_mul_greater_to_out);
    register_demo!(registry, demo_limbs_mul_to_out);
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_22_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_32_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_33_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_42_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_43_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_44_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_52_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_53_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_54_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_62_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_63_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_6h_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_8h_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_fft_input_sizes_threshold
    );
    register_demo!(registry, demo_limbs_mul_low_same_length_basecase);
    register_demo!(
        registry,
        demo_limbs_mul_low_same_length_divide_and_conquer_shared_scratch
    );
    register_demo!(registry, demo_limbs_mul_low_same_length_divide_and_conquer);
    register_demo!(registry, demo_limbs_mul_low_same_length);
    register_demo!(registry, demo_natural_mul_assign);
    register_demo!(registry, demo_natural_mul_assign_ref);
    register_demo!(registry, demo_natural_mul);
    register_demo!(registry, demo_natural_mul_val_ref);
    register_demo!(registry, demo_natural_mul_ref_val);
    register_demo!(registry, demo_natural_mul_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_mul_limb);
    register_bench!(registry, Small, benchmark_limbs_mul_limb_with_carry_to_out);
    register_bench!(registry, Small, benchmark_limbs_mul_limb_to_out);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_mul_limb_with_carry_in_place
    );
    register_bench!(registry, Small, benchmark_limbs_slice_mul_limb_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_mul_limb_in_place);
    register_bench!(registry, Small, benchmark_limbs_mul_greater);
    register_bench!(registry, Small, benchmark_limbs_mul);
    register_bench!(registry, Small, benchmark_limbs_mul_same_length_to_out);
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_mul_to_out);
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_basecase_mem_opt_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_22_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_33_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_33_same_length_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_42_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_43_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_32_to_43_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_44_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_44_same_length_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_52_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_53_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_42_to_53_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_54_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_62_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_63_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_6h_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_6h_same_length_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_8h_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_8h_same_length_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_fft_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_fft_same_length_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_basecase_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_basecase_algorithms_2
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_divide_and_conquer_shared_scratch_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_divide_and_conquer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_large_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_low_same_length_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mul_library_comparison);
    register_bench!(registry, Large, benchmark_natural_mul_evaluation_strategy);
}

fn demo_limbs_mul_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_mul_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_mul_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_mul_limb_with_carry_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb, carry) in
        quadruples_of_unsigned_vec_unsigned_vec_unsigned_and_unsigned_var_1(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry_out = limbs_mul_limb_with_carry_to_out(&mut out, &in_limbs, limb, carry);
        println!(
            "out := {:?}; limbs_mul_limb_with_carry_to_out(&mut out, {:?}, {}, {}) = {}; \
             out = {:?}",
            out_old, in_limbs, limb, carry, carry_out, out
        );
    }
}

fn demo_limbs_mul_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry = limbs_mul_limb_to_out(&mut out, &in_limbs, limb);
        println!(
            "out := {:?}; limbs_mul_limb_to_out(&mut out, {:?}, {}) = {}; out = {:?}",
            out_old, in_limbs, limb, carry, out
        );
    }
}

fn demo_limbs_slice_mul_limb_with_carry_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb, carry) in triples_of_unsigned_vec_unsigned_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let carry_out = limbs_slice_mul_limb_with_carry_in_place(&mut limbs, limb, carry);
        println!(
            "limbs := {:?}; limbs_slice_mul_limb_with_carry_in_place(&mut limbs, {}, {}) = {}; \
             limbs = {:?}",
            limbs_old, limb, carry, carry_out, limbs
        );
    }
}

fn demo_limbs_slice_mul_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let carry = limbs_slice_mul_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_slice_mul_limb_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, carry, limbs
        );
    }
}

fn demo_limbs_vec_mul_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_mul_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_vec_mul_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_limbs_mul_greater(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_4(gm).take(limit) {
        println!(
            "limbs_mul_greater({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_mul_greater(&xs, &ys)
        );
    }
}

fn demo_limbs_mul(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_5(gm).take(limit) {
        println!("limbs_mul({:?}, {:?}) = {:?}", xs, ys, limbs_mul(&xs, &ys));
    }
}

fn demo_limbs_mul_same_length_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_25(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_mul_same_length_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_mul_same_length_to_out(&mut out, {:?}, {:?}); \
             out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_mul_greater_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_10(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_mul_greater_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_mul_greater_to_out(&mut out, {:?}, {:?}) = \
             {}; out = {:?}",
            xs_old, ys, zs, carry, xs
        );
    }
}

fn demo_limbs_mul_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_26(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry = limbs_mul_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_mul_to_out(&mut out, {:?}, {:?}) = {}; out = {:?}",
            xs_old, ys, zs, carry, xs
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_22_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_22_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_22_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_32_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_32_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_32_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_33_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_33_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_33_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_42_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_42_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_42_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_43_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_43_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_43_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_44_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_44_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_52_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_52_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_52_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_53_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_53_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_53_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_54_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_54_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_54_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_62_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_62_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_62_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_63_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_63_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_63_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_6h_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_6h_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_6h_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_8h_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_8h_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_8h_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_fft_input_sizes_threshold(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_unsigneds_single(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_fft_input_sizes_threshold({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_fft_input_sizes_threshold(x, y)
        );
    }
}

fn demo_limbs_mul_low_same_length_basecase(gm: GenerationMode, limit: usize) {
    for (mut out, xs, ys) in triples_of_unsigned_vec_var_46(gm).take(limit) {
        let out_old = out.clone();
        _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys);
        println!(
            "out := {:?}; _limbs_mul_low_same_length_basecase(&mut out, {:?}, {:?}); out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
    gm: GenerationMode,
    limit: usize,
) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_48(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; \
             _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, {:?}, {:?}); \
             out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_mul_low_same_length_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_49(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut scratch = vec![0; ys.len() << 1];
        _limbs_mul_low_same_length_divide_and_conquer(&mut xs, &ys, &zs, &mut scratch);
        println!(
            "out := {:?}; _limbs_mul_low_same_length_divide_and_conquer(&mut out, {:?}, {:?}); \
             out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_limbs_mul_low_same_length(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_25(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_mul_low_same_length(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_mul_low_same_length(&mut out, {:?}, {:?}); out = {:?}",
            xs_old, ys, zs, xs
        );
    }
}

fn demo_natural_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {}; x *= {}; x = {}", x_old, y, x);
    }
}

fn demo_natural_mul_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {}; x *= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_mul(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

fn demo_natural_mul_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

fn demo_natural_mul_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

fn demo_natural_mul_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

fn benchmark_limbs_mul_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_mul_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_mul_limb_with_carry_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb_with_carry_to_out(&mut [Limb], &[Limb], Limb, Limb)",
        BenchmarkType::Single,
        quadruples_of_unsigned_vec_unsigned_vec_unsigned_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, in_limbs, limb, carry)| {
                no_out!(limbs_mul_limb_with_carry_to_out(
                    &mut out, &in_limbs, limb, carry
                ))
            }),
        )],
    );
}

fn benchmark_limbs_mul_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, in_limbs, limb)| {
                no_out!(limbs_mul_limb_to_out(&mut out, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_slice_mul_limb_with_carry_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mul_limb_with_carry_in_place(&mut [Limb], Limb, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb, carry)| {
                no_out!(limbs_slice_mul_limb_with_carry_in_place(
                    &mut limbs, limb, carry
                ))
            }),
        )],
    );
}

fn benchmark_limbs_slice_mul_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_slice_mul_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| no_out!(limbs_slice_mul_limb_in_place(&mut limbs, limb))),
        )],
    );
}

fn benchmark_limbs_vec_mul_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_vec_mul_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| limbs_vec_mul_limb_in_place(&mut limbs, limb)),
        )],
    );
}

fn benchmark_limbs_mul_greater(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_greater(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [(
            "malachite",
            &mut (|(xs, ys)| no_out!(limbs_mul_greater(&xs, &ys))),
        )],
    );
}

fn benchmark_limbs_mul(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_5(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [("malachite", &mut (|(xs, ys)| no_out!(limbs_mul(&xs, &ys))))],
    );
}

fn benchmark_limbs_mul_same_length_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_25(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| limbs_mul_same_length_to_out(&mut xs, &ys, &zs)),
        )],
    );
}

fn benchmark_limbs_mul_greater_to_out_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_10(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "full",
                &mut (|(mut out, xs, ys)| no_out!(limbs_mul_greater_to_out(&mut out, &xs, &ys))),
            ),
        ],
    );
}

fn benchmark_limbs_mul_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_26(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, xs, ys)| no_out!(limbs_mul_to_out(&mut out, &xs, &ys))),
        )],
    );
}

fn benchmark_limbs_mul_greater_to_out_basecase_mem_opt_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_basecase_mem_opt(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_10(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "limbs_mul_greater_to_out_basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "limbs_mul_greater_to_out_basecase_mem_opt",
                &mut (|(mut out, xs, ys)| {
                    _limbs_mul_greater_to_out_basecase_mem_opt(&mut out, &xs, &ys)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_22_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_22(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_11(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom22",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_22_scratch_len(xs.len())];
                    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_32(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_12(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom32",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_33_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_33(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_13(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom33",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_33_scratch_len(xs.len())];
                    _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_33_same_length_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_33(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_30(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom22",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_22_scratch_len(xs.len())];
                    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom33",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_33_scratch_len(xs.len())];
                    _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_42_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_42(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_14(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom42",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_43_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_43(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_15(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom43",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_43_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_32_to_43_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Toom32 to Toom43",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_35(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "Toom32",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom43",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_43_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_44_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_44(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_16(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom44",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_44_scratch_len(xs.len())];
                    _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_44_same_length_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_44(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_31(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom33",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_33_scratch_len(xs.len())];
                    _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom44",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_44_scratch_len(xs.len())];
                    _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_52_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_52(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_17(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom52",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_52_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_53_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_53(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_18(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom32",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom42",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom53",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_42_to_53_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Toom42 to Toom53",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_36(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "Toom42",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom53",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_54_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_54(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_19(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom54",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_54_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_62_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_62(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_20(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom62",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_62_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_62(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_63_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_63(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_21(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom42",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom63",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_63_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_6h_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_6h(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_22(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom6h",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_6h_same_length_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_6h(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom44",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_44_scratch_len(xs.len())];
                    _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom6h",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_8h_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_8h(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_23(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom8h",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_8h_same_length_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_8h(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_33(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom6h",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "Toom8h",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_fft_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_fft(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_10(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "xs.len() + ys.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "FFT",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_fft(&mut out, &xs, &ys)),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_fft_same_length_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_fft(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_34(gm.with_scale(8_192)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [
            (
                "Toom8h",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "FFT",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_fft(&mut out, &xs, &ys)),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_basecase_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_basecase(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_46(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut out, xs, ys)| _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)),
            ),
            (
                "alt",
                &mut (|(mut out, xs, ys)| {
                    _limbs_mul_low_same_length_basecase_alt(&mut out, &xs, &ys)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_basecase_algorithms_2(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_basecase(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_47(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "basecase mul",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "basecase mul low",
                &mut (|(mut out, xs, ys)| _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_divide_and_conquer_shared_scratch_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_divide_and_conquer_shared_scratch\
         (&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_48(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut out, xs, ys)| {
                    _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &xs, &ys)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_divide_and_conquer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_divide_and_conquer(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_49(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys)),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch = vec![0; ys.len() << 1];
                    _limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_large_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length_large(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_52(gm.with_scale(u32::power_of_two(15))),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "mul low divide-and-conquer",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![
                            0;
                            _limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())
                        ];
                    _limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
            (
                "mul low large",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![
                            0;
                            _limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())
                        ];
                    _limbs_mul_low_same_length_large(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_low_same_length_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_low_same_length(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_25(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "mul low",
                &mut (|(mut out, xs, ys)| limbs_mul_low_same_length(&mut out, &xs, &ys)),
            ),
            (
                "mul",
                &mut (|(mut out, xs, ys)| limbs_mul_same_length_to_out(&mut out, &xs, &ys)),
            ),
        ],
    );
}

fn benchmark_natural_mul_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural *= Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals(gm.with_scale(u32::exact_from(16 * Limb::WIDTH))),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x *= y)),
            ("rug", &mut (|((mut x, y), _)| x *= y)),
        ],
    );
}

fn benchmark_natural_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural *= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Natural *= Natural", &mut (|(mut x, y)| no_out!(x *= y))),
            ("Natural *= &Natural", &mut (|(mut x, y)| no_out!(x *= &y))),
        ],
    );
}

fn benchmark_natural_mul_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural * Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm.with_scale(u32::exact_from(16 * Limb::WIDTH))),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x * y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x * y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x * y))),
        ],
    );
}

fn benchmark_natural_mul_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural * Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Natural * Natural", &mut (|(x, y)| no_out!(x * y))),
            ("Natural * &Natural", &mut (|(x, y)| no_out!(x * &y))),
            ("&Natural * Natural", &mut (|(x, y)| no_out!(&x * y))),
            ("&Natural * &Natural", &mut (|(x, y)| no_out!(&x * &y))),
        ],
    );
}
