use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem, DivRound,
    NegMod,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_barrett_large_product, _limbs_div_mod_barrett, _limbs_div_mod_barrett_helper,
    _limbs_div_mod_barrett_large_helper, _limbs_div_mod_barrett_scratch_len,
    _limbs_div_mod_divide_and_conquer, _limbs_div_mod_schoolbook, _limbs_invert_approx,
    _limbs_invert_basecase_approx, _limbs_invert_newton_approx, limbs_div_limb_in_place_mod,
    limbs_div_limb_mod, limbs_div_limb_to_out_mod, limbs_div_mod,
    limbs_div_mod_by_two_limb_normalized, limbs_div_mod_three_limb_by_two_limb,
    limbs_div_mod_to_out, limbs_invert_limb, limbs_two_limb_inverse_helper,
};
use malachite_nz::natural::arithmetic::mul::limbs_mul_greater_to_out;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::arithmetic::div_mod::rug_ceiling_div_neg_mod;
use malachite_nz_test_util::natural::arithmetic::div_mod::{
    _limbs_div_limb_in_place_mod_alt, _limbs_div_limb_to_out_mod_alt,
};
use malachite_nz_test_util::natural::arithmetic::div_mod::{
    limbs_div_limb_in_place_mod_naive, limbs_div_limb_to_out_mod_naive,
};
use num::Integer;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_var_9, pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    pairs_of_unsigneds_var_2, quadruples_of_limb_vec_var_1, quadruples_of_limb_vec_var_2,
    quadruples_of_limb_vec_var_3, quadruples_of_three_limb_vecs_and_limb_var_1,
    quadruples_of_three_limb_vecs_and_limb_var_2, sextuples_of_four_limb_vecs_and_two_usizes_var_1,
    sextuples_of_limbs_var_1, triples_of_limb_vec_var_38, triples_of_limb_vec_var_39,
    triples_of_limb_vec_var_40, triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_var_37, unsigneds_var_1,
};
use malachite_test::inputs::natural::{
    nrm_pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural,
    rm_pairs_of_natural_and_positive_natural,
};

// For `Natural`s, `mod` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_invert_limb);
    register_demo!(registry, demo_limbs_div_limb_mod);
    register_demo!(registry, demo_limbs_div_limb_to_out_mod);
    register_demo!(registry, demo_limbs_div_limb_in_place_mod);
    register_demo!(registry, demo_limbs_two_limb_inverse_helper);
    register_demo!(registry, demo_limbs_div_mod_three_limb_by_two_limb);
    register_demo!(registry, demo_limbs_div_mod_by_two_limb_normalized);
    register_demo!(registry, demo_limbs_div_mod_schoolbook);
    register_demo!(registry, demo_limbs_div_mod_divide_and_conquer);
    register_demo!(registry, demo_limbs_invert_basecase_approx);
    register_demo!(registry, demo_limbs_invert_newton_approx);
    register_demo!(registry, demo_limbs_invert_approx);
    register_demo!(registry, demo_limbs_div_mod_barrett);
    register_demo!(registry, demo_limbs_div_mod);
    register_demo!(registry, demo_limbs_div_mod_to_out);
    register_demo!(registry, demo_natural_div_assign_mod);
    register_demo!(registry, demo_natural_div_assign_mod_ref);
    register_demo!(registry, demo_natural_div_mod);
    register_demo!(registry, demo_natural_div_mod_val_ref);
    register_demo!(registry, demo_natural_div_mod_ref_val);
    register_demo!(registry, demo_natural_div_mod_ref_ref);
    register_demo!(registry, demo_natural_div_assign_rem);
    register_demo!(registry, demo_natural_div_assign_rem_ref);
    register_demo!(registry, demo_natural_div_rem);
    register_demo!(registry, demo_natural_div_rem_val_ref);
    register_demo!(registry, demo_natural_div_rem_ref_val);
    register_demo!(registry, demo_natural_div_rem_ref_ref);
    register_demo!(registry, demo_natural_ceiling_div_assign_neg_mod);
    register_demo!(registry, demo_natural_ceiling_div_assign_neg_mod_ref);
    register_demo!(registry, demo_natural_ceiling_div_neg_mod);
    register_demo!(registry, demo_natural_ceiling_div_neg_mod_val_ref);
    register_demo!(registry, demo_natural_ceiling_div_neg_mod_ref_val);
    register_demo!(registry, demo_natural_ceiling_div_neg_mod_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_invert_limb);
    register_bench!(registry, Small, benchmark_limbs_div_limb_mod);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_limb_to_out_mod_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_limb_in_place_mod_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_mod_by_two_limb_normalized
    );
    register_bench!(registry, Small, benchmark_limbs_div_mod_schoolbook);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_mod_divide_and_conquer_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_invert_basecase_approx);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_invert_newton_approx_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_invert_approx_algorithms);
    register_bench!(registry, Small, benchmark_limbs_div_mod_barrett);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_mod_divide_and_conquer_to_barrett_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_mod_barrett_product_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_mod_barrett_helper_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_div_mod);
    register_bench!(registry, Small, benchmark_limbs_div_mod_to_out);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_assign_mod_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_mod_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_div_mod_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_mod_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_assign_rem_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_rem_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_rem_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_assign_neg_mod_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_neg_mod_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_neg_mod_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_neg_mod_evaluation_strategy
    );
}

fn demo_limbs_invert_limb(gm: GenerationMode, limit: usize) {
    for limb in unsigneds_var_1(gm).take(limit) {
        println!("limbs_invert_limb({}) = {}", limb, limbs_invert_limb(limb));
    }
}

fn demo_limbs_div_limb_mod(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_div_limb_mod({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_div_limb_mod(&limbs, limb)
        );
    }
}

fn demo_limbs_div_limb_to_out_mod(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let remainder = limbs_div_limb_to_out_mod(&mut out, &in_limbs, limb);
        println!(
            "out := {:?}; limbs_div_limb_to_out_mod(&mut out, {:?}, {}) = {}; \
             out = {:?}",
            out_old, in_limbs, limb, remainder, out
        );
    }
}

fn demo_limbs_div_limb_in_place_mod(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let remainder = limbs_div_limb_in_place_mod(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_div_limb_in_place_mod(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, remainder, limbs
        );
    }
}

fn demo_limbs_two_limb_inverse_helper(gm: GenerationMode, limit: usize) {
    for (hi, lo) in pairs_of_unsigneds_var_2(gm).take(limit) {
        println!(
            "limbs_two_limb_inverse_helper({}, {}) = {}",
            hi,
            lo,
            limbs_two_limb_inverse_helper(hi, lo)
        );
    }
}

fn demo_limbs_div_mod_three_limb_by_two_limb(gm: GenerationMode, limit: usize) {
    for (n2, n1, n0, d1, d0, inverse) in sextuples_of_limbs_var_1(gm).take(limit) {
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

fn demo_limbs_div_mod_by_two_limb_normalized(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds) in triples_of_unsigned_vec_var_37(gm).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, &ds);
        println!(
            "qs := {:?}; ns := {:?}; \
             limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, {:?}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_mod_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_limb_vecs_and_limb_var_1(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = _limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; _limbs_div_mod_schoolbook(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_mod_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_limb_vecs_and_limb_var_2(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = _limbs_div_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_div_mod_divide_and_conquer(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_invert_basecase_approx(gm: GenerationMode, limit: usize) {
    for (mut is, ds, mut scratch) in triples_of_limb_vec_var_38(gm).take(limit) {
        let old_is = is.clone();
        let old_scratch = scratch.clone();
        let result_definitely_exact = _limbs_invert_basecase_approx(&mut is, &ds, &mut scratch);
        println!(
            "is := {:?}; scratch := {:?}; \
             _limbs_invert_basecase_approx(&mut is, {:?}, &mut scratch) = {}; \
             is = {:?}, scratch = {:?}",
            old_is, old_scratch, ds, result_definitely_exact, is, scratch
        );
    }
}

fn demo_limbs_invert_newton_approx(gm: GenerationMode, limit: usize) {
    for (mut is, ds, mut scratch) in triples_of_limb_vec_var_39(gm).take(limit) {
        let old_is = is.clone();
        let old_scratch = scratch.clone();
        let result_definitely_exact = _limbs_invert_newton_approx(&mut is, &ds, &mut scratch);
        println!(
            "is := {:?}; scratch := {:?}; \
             _limbs_invert_newton_approx(&mut is, {:?}, &mut scratch) = {}; \
             is = {:?}, scratch = {:?}",
            old_is, old_scratch, ds, result_definitely_exact, is, scratch
        );
    }
}

fn demo_limbs_invert_approx(gm: GenerationMode, limit: usize) {
    for (mut is, ds, mut scratch) in triples_of_limb_vec_var_38(gm).take(limit) {
        let old_is = is.clone();
        let old_scratch = scratch.clone();
        let result_definitely_exact = _limbs_invert_approx(&mut is, &ds, &mut scratch);
        println!(
            "is := {:?}; scratch := {:?}; \
             _limbs_invert_approx(&mut is, {:?}, &mut scratch) = {}; \
             is = {:?}, scratch = {:?}",
            old_is, old_scratch, ds, result_definitely_exact, is, scratch
        );
    }
}

fn demo_limbs_div_mod_barrett(gm: GenerationMode, limit: usize) {
    for (mut qs, mut rs, ns, ds) in quadruples_of_limb_vec_var_1(gm).take(limit) {
        let old_qs = qs.clone();
        let old_rs = rs.clone();
        let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
        let highest_q = _limbs_div_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; rs := {:?}; _limbs_div_mod_barrett(&mut qs, &mut ns, {:?}, {:?}) = {}; \
             qs = {:?}, rs = {:?}",
            old_qs, old_rs, ns, ds, highest_q, qs, rs
        );
    }
}

fn demo_limbs_div_mod(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_limb_vec_var_9(gm).take(limit) {
        println!(
            "limbs_div_mod({:?}, {:?}) = {:?}",
            ns,
            ds,
            limbs_div_mod(&ns, &ds)
        );
    }
}

fn demo_limbs_div_mod_to_out(gm: GenerationMode, limit: usize) {
    for (mut qs, mut rs, ns, ds) in quadruples_of_limb_vec_var_2(gm).take(limit) {
        let old_qs = qs.clone();
        let old_rs = rs.clone();
        limbs_div_mod_to_out(&mut qs, &mut rs, &ns, &ds);
        println!(
            "qs := {:?}; rs := {:?}; limbs_div_mod_to_out(&mut qs, &mut ns, {:?}, {:?}); \
             qs = {:?}, rs = {:?}",
            old_qs, old_rs, ns, ds, qs, rs
        );
    }
}

fn demo_natural_div_assign_mod(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.div_assign_mod(y);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            x_old, y_old, remainder, x
        );
    }
}

fn demo_natural_div_assign_mod_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let remainder = x.div_assign_mod(&y);
        println!(
            "x := {}; x.div_assign_mod(&{}) = {}; x = {}",
            x_old, y, remainder, x
        );
    }
}

fn demo_natural_div_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_mod({}) = {:?}", x_old, y_old, x.div_mod(y));
    }
}

fn demo_natural_div_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.div_mod(&{}) = {:?}", x_old, y, x.div_mod(&y));
    }
}

fn demo_natural_div_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_mod({}) = {:?}", x, y_old, (&x).div_mod(y));
    }
}

fn demo_natural_div_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("(&{}).div_mod(&{}) = {:?}", x, y, (&x).div_mod(&y));
    }
}

fn demo_natural_div_assign_rem(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.div_assign_rem(y);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            x_old, y_old, remainder, x
        );
    }
}

fn demo_natural_div_assign_rem_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let remainder = x.div_assign_rem(&y);
        println!(
            "x := {}; x.div_assign_rem(&{}) = {}; x = {}",
            x_old, y, remainder, x
        );
    }
}

fn demo_natural_div_rem(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_rem({}) = {:?}", x_old, y_old, x.div_rem(y));
    }
}

fn demo_natural_div_rem_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.div_rem(&{}) = {:?}", x_old, y, x.div_rem(&y));
    }
}

fn demo_natural_div_rem_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_rem({}) = {:?}", x, y_old, (&x).div_rem(y));
    }
}

fn demo_natural_div_rem_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("(&{}).div_rem(&{}) = {:?}", x, y, (&x).div_rem(&y));
    }
}

fn demo_natural_ceiling_div_assign_neg_mod(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.ceiling_div_assign_neg_mod(y);
        println!(
            "x := {}; x.ceiling_div_assign_neg_mod({}) = {}; x = {}",
            x_old, y_old, remainder, x
        );
    }
}

fn demo_natural_ceiling_div_assign_neg_mod_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let remainder = x.ceiling_div_assign_neg_mod(&y);
        println!(
            "x := {}; x.ceiling_div_assign_neg_mod(&{}) = {}; x = {}",
            x_old, y, remainder, x
        );
    }
}

fn demo_natural_ceiling_div_neg_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
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

fn demo_natural_ceiling_div_neg_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.ceiling_div_neg_mod(&{}) = {:?}",
            x_old,
            y,
            x.ceiling_div_neg_mod(&y)
        );
    }
}

fn demo_natural_ceiling_div_neg_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).ceiling_div_neg_mod({}) = {:?}",
            x,
            y_old,
            (&x).ceiling_div_neg_mod(y)
        );
    }
}

fn demo_natural_ceiling_div_neg_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!(
            "(&{}).ceiling_div_neg_mod(&{}) = {:?}",
            x,
            y,
            (&x).ceiling_div_neg_mod(&y)
        );
    }
}

fn benchmark_limbs_invert_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_invert_limb(Limb)",
        BenchmarkType::Single,
        unsigneds_var_1::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|limb| usize::exact_from(limb.significant_bits())),
        "limb.significant_bits()",
        &mut [("malachite", &mut (|limb| no_out!(limbs_invert_limb(limb))))],
    );
}

fn benchmark_limbs_div_limb_mod(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb_mod(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_div_limb_mod(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_div_limb_to_out_mod_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_div_limb_to_out_mod(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut out, in_limbs, limb)| {
                    no_out!(limbs_div_limb_to_out_mod(&mut out, &in_limbs, limb))
                }),
            ),
            (
                "alt",
                &mut (|(mut out, in_limbs, limb)| {
                    no_out!(_limbs_div_limb_to_out_mod_alt(&mut out, &in_limbs, limb))
                }),
            ),
            (
                "naive",
                &mut (|(mut out, in_limbs, limb)| {
                    no_out!(limbs_div_limb_to_out_mod_naive(&mut out, &in_limbs, limb))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_limb_in_place_mod_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_div_limb_in_place_mod(&mut [Limb], Limb)",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut limbs, limb)| no_out!(limbs_div_limb_in_place_mod(&mut limbs, limb))),
            ),
            (
                "alt",
                &mut (|(mut limbs, limb)| {
                    no_out!(_limbs_div_limb_in_place_mod_alt(&mut limbs, limb))
                }),
            ),
            (
                "naive",
                &mut (|(mut limbs, limb)| {
                    no_out!(limbs_div_limb_in_place_mod_naive(&mut limbs, limb))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_mod_by_two_limb_normalized(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_div_mod_by_two_limb_normalized(&mut [Limb], &mut [Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_37(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut ns, ds)| {
                no_out!(limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, &ds))
            }),
        )],
    );
}

fn benchmark_limbs_div_mod_schoolbook(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_div_mod_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        quadruples_of_three_limb_vecs_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut ns, ds, inverse)| {
                no_out!(_limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
            }),
        )],
    );
}

fn benchmark_limbs_div_mod_divide_and_conquer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_div_mod_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_limb_vecs_and_limb_var_2(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len()",
        &mut [
            (
                "Schoolbook",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_mod_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_invert_basecase_approx(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_invert_basecase_approx(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        triples_of_limb_vec_var_38(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ds, _)| ds.len()),
        "ds.len()",
        &mut [(
            "malachite",
            &mut (|(mut is, ds, mut scratch)| {
                no_out!(_limbs_invert_basecase_approx(&mut is, &ds, &mut scratch))
            }),
        )],
    );
}

fn benchmark_limbs_invert_newton_approx_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_invert_newton_approx(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        triples_of_limb_vec_var_39(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ds, _)| ds.len()),
        "ds.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut is, ds, mut scratch)| {
                    no_out!(_limbs_invert_basecase_approx(&mut is, &ds, &mut scratch))
                }),
            ),
            (
                "Newton",
                &mut (|(mut is, ds, mut scratch)| {
                    no_out!(_limbs_invert_newton_approx(&mut is, &ds, &mut scratch))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_invert_approx_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_invert_approx(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        triples_of_limb_vec_var_38(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ds, _)| ds.len()),
        "ds.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut is, ds, mut scratch)| {
                    no_out!(_limbs_invert_basecase_approx(&mut is, &ds, &mut scratch))
                }),
            ),
            (
                "default",
                &mut (|(mut is, ds, mut scratch)| {
                    no_out!(_limbs_invert_approx(&mut is, &ds, &mut scratch))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_mod_barrett(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_div_mod_barrett(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Single,
        quadruples_of_limb_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, _, ref ds)| ds.len()),
        "ds.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut rs, ns, ds)| {
                let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                no_out!(_limbs_div_mod_barrett(
                    &mut qs,
                    &mut rs,
                    &ns,
                    &ds,
                    &mut scratch
                ))
            }),
        )],
    );
}

fn benchmark_limbs_div_mod_divide_and_conquer_to_barrett_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_div_mod_barrett(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Algorithms,
        triples_of_limb_vec_var_40(gm.with_scale(4_096)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds)| ns.len() - ds.len()),
        "ns.len()",
        &mut [
            (
                "divide-and-conquer",
                &mut (|(mut qs, mut ns, mut ds)| {
                    let q_len = ns.len() - ds.len() + 1;
                    ds[q_len - 1].set_bit(Limb::WIDTH - 1);
                    let inverse = limbs_two_limb_inverse_helper(ds[q_len - 1], ds[q_len - 2]);
                    no_out!(_limbs_div_mod_divide_and_conquer(
                        &mut qs,
                        &mut ns[..q_len << 1],
                        &ds[..q_len],
                        inverse
                    ))
                }),
            ),
            (
                "Barrett",
                &mut (|(mut qs, mut ns, mut ds)| {
                    let d_len = ds.len();
                    let mut rs = vec![0; d_len];
                    let q_len = ns.len() - d_len + 1;
                    let q_len_2 = q_len << 1;
                    ds[q_len - 1].set_bit(Limb::WIDTH - 1);
                    limbs_two_limb_inverse_helper(ds[q_len - 1], ds[q_len - 2]);
                    let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(q_len_2, q_len)];
                    _limbs_div_mod_barrett(
                        &mut qs,
                        &mut rs,
                        &ns[..q_len_2],
                        &ds[..q_len],
                        &mut scratch,
                    );
                    ns[..q_len].copy_from_slice(&rs[..q_len]);
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_mod_barrett_product_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_div_barrett_large_product(\
         &mut [Limb], &[Limb], &[Limb], &[Limb], usize, usize)",
        BenchmarkType::Algorithms,
        sextuples_of_four_limb_vecs_and_two_usizes_var_1(gm.with_scale(128)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, _, _, _, i_len)| i_len << 1),
        "2 * i_len",
        &mut [
            (
                "_limbs_mul_greater_to_out",
                &mut (|(mut scratch, ds, qs, _, _, _)| {
                    no_out!(limbs_mul_greater_to_out(&mut scratch, &ds, &qs))
                }),
            ),
            (
                "_limbs_div_barrett_large_product",
                &mut (|(mut scratch, ds, qs, rs_hi, scratch_len, i_len)| {
                    _limbs_div_barrett_large_product(
                        &mut scratch,
                        &ds,
                        &qs,
                        &rs_hi,
                        scratch_len,
                        i_len,
                    )
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_mod_barrett_helper_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_div_mod_barrett_helper(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        quadruples_of_limb_vec_var_3(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ns, ref ds)| (ds.len() << 1).saturating_sub(ns.len())),
        "max(0, 2 * ds.len() - ns.len())",
        &mut [
            (
                "_limbs_div_mod_barrett_helper",
                &mut (|(mut qs, mut rs, ns, ds)| {
                    let mut scratch =
                        vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                    let q_len = ns.len() - ds.len();
                    no_out!(_limbs_div_mod_barrett_helper(
                        &mut qs[..q_len],
                        &mut rs[..ds.len()],
                        &ns,
                        &ds,
                        &mut scratch
                    ))
                }),
            ),
            (
                "_limbs_div_mod_barrett_large_helper",
                &mut (|(mut qs, mut rs, ns, ds)| {
                    let mut scratch =
                        vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                    let q_len = ns.len() - ds.len();
                    no_out!(_limbs_div_mod_barrett_large_helper(
                        &mut qs[..q_len],
                        &mut rs[..ds.len()],
                        &ns,
                        &ds,
                        &mut scratch
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_mod(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_mod(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_limb_vec_var_9(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref ns, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(ns, ds)| no_out!(limbs_div_mod(&ns, &ds))),
        )],
    );
}

fn benchmark_limbs_div_mod_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_mod_to_out(&mut [Limb], &mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        quadruples_of_limb_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut rs, ns, ds)| limbs_div_mod_to_out(&mut qs, &mut rs, &ns, &ds)),
        )],
    );
}

fn benchmark_natural_div_assign_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_assign_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_assign_mod(Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
            ),
            (
                "Natural.div_mod(&Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_mod(&y))),
            ),
        ],
    );
}

fn benchmark_natural_div_mod_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_mod(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_mod(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.div_mod_floor(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_rem_floor(y)))),
        ],
    );
}

fn benchmark_natural_div_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_mod(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.div_mod(y)))),
            ("using / and %", &mut (|(x, y)| no_out!((&x / &y, x % y)))),
        ],
    );
}

fn benchmark_natural_div_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_mod(Natural)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "Natural.div_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.div_mod(&y))),
            ),
            (
                "(&Natural).div_mod(Natural)",
                &mut (|(x, y)| no_out!((&x).div_mod(y))),
            ),
            (
                "(&Natural).div_mod(&Natural)",
                &mut (|(x, y)| no_out!((&x).div_mod(&y))),
            ),
        ],
    );
}

fn benchmark_natural_div_assign_rem_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_assign_rem(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_assign_rem(Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
            ),
            (
                "Natural.div_assign_rem(&Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_rem(&y))),
            ),
        ],
    );
}

fn benchmark_natural_div_rem_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_rem(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_rem(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.div_rem(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_rem(y)))),
        ],
    );
}

fn benchmark_natural_div_rem_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_rem(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_rem(Natural)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "Natural.div_rem(&Natural)",
                &mut (|(x, y)| no_out!(x.div_rem(&y))),
            ),
            (
                "(&Natural).div_rem(Natural)",
                &mut (|(x, y)| no_out!((&x).div_rem(y))),
            ),
            (
                "(&Natural).div_rem(&Natural)",
                &mut (|(x, y)| no_out!((&x).div_rem(&y))),
            ),
        ],
    );
}

fn benchmark_natural_ceiling_div_assign_neg_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_assign_neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.ceiling_div_assign_neg_mod(Natural)",
                &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(y))),
            ),
            (
                "Natural.ceiling_div_assign_neg_mod(&Natural)",
                &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(&y))),
            ),
        ],
    );
}

fn benchmark_natural_ceiling_div_neg_mod_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_neg_mod(Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "rug",
                &mut (|((x, y), _)| no_out!(rug_ceiling_div_neg_mod(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_ceiling_div_neg_mod_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_neg_mod(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "using div_round and %",
                &mut (|(x, y)| {
                    ((&x).div_round(&y, RoundingMode::Ceiling), x.neg_mod(y));
                }),
            ),
        ],
    );
}

fn benchmark_natural_ceiling_div_neg_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.ceiling_div_neg_mod(Natural)",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "Natural.ceiling_div_neg_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(&y))),
            ),
            (
                "(&Natural).ceiling_div_neg_mod(Natural)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_neg_mod(y))),
            ),
            (
                "(&Natural).ceiling_div_neg_mod(&Natural)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_neg_mod(&y))),
            ),
        ],
    );
}
