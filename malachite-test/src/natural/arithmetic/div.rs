use std::cmp::max;

use malachite_base::num::arithmetic::traits::DivMod;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
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
use malachite_nz_test_util::natural::arithmetic::div::{
    limbs_div_limb_in_place_alt, limbs_div_limb_to_out_alt,
};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_var_9, pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3, quadruples_of_limb_vec_var_2,
    quadruples_of_three_limb_vecs_and_limb_var_1, quadruples_of_three_limb_vecs_and_limb_var_2,
    triples_of_limb_vec_limb_and_limb_var_1, triples_of_limb_vec_var_41,
    triples_of_limb_vec_var_42, triples_of_limb_vec_var_43, triples_of_limb_vec_var_44,
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    nrm_pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_div_limb);
    register_demo!(registry, demo_limbs_div_limb_to_out);
    register_demo!(registry, demo_limbs_div_limb_in_place);
    register_demo!(
        registry,
        demo_limbs_div_divisor_of_limb_max_with_carry_to_out
    );
    register_demo!(
        registry,
        demo_limbs_div_divisor_of_limb_max_with_carry_in_place
    );
    register_demo!(registry, demo_limbs_div_schoolbook);
    register_demo!(registry, demo_limbs_div_divide_and_conquer);
    register_demo!(registry, demo_limbs_div_barrett);
    register_demo!(registry, demo_limbs_div_schoolbook_approx);
    register_demo!(registry, demo_limbs_div_divide_and_conquer_approx);
    register_demo!(registry, demo_limbs_div_barrett_approx);
    register_demo!(registry, demo_limbs_div);
    register_demo!(registry, demo_limbs_div_to_out);
    register_demo!(registry, demo_limbs_div_to_out_val_ref);
    register_demo!(registry, demo_limbs_div_to_out_ref_val);
    register_demo!(registry, demo_limbs_div_to_out_ref_ref);
    register_demo!(registry, demo_natural_div_assign);
    register_demo!(registry, demo_natural_div_assign_ref);
    register_demo!(registry, demo_natural_div);
    register_demo!(registry, demo_natural_div_val_ref);
    register_demo!(registry, demo_natural_div_ref_val);
    register_demo!(registry, demo_natural_div_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_div_limb);
    register_bench!(registry, Small, benchmark_limbs_div_limb_to_out_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_limb_in_place_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_divisor_of_limb_max_with_carry_to_out
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_divisor_of_limb_max_with_carry_in_place
    );
    register_bench!(registry, Small, benchmark_limbs_div_schoolbook_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_divide_and_conquer_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_div_barrett_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_schoolbook_approx_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_divide_and_conquer_approx_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_barrett_approx_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_div_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_to_out_balancing_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_to_out_evaluation_strategy
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_to_out_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_div_library_comparison);
    register_bench!(registry, Large, benchmark_natural_div_algorithms);
    register_bench!(registry, Large, benchmark_natural_div_evaluation_strategy);
}

fn demo_limbs_div_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_div_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_div_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_div_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_div_limb_to_out(&mut out, &in_limbs, limb);
        println!(
            "out := {:?}; limbs_div_limb_to_out(&mut out, {:?}, {}); out = {:?}",
            out_old, in_limbs, limb, out
        );
    }
}

fn demo_limbs_div_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_div_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_div_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_limbs_div_divisor_of_limb_max_with_carry_to_out(gm: GenerationMode, limit: usize) {
    for (out, xs, divisor, carry) in
        quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry_out =
            limbs_div_divisor_of_limb_max_with_carry_to_out(&mut out, &xs, divisor, carry);
        println!(
            "out := {:?}; limbs_div_divisor_of_limb_max_with_carry_to_out(&mut out, {:?}, {}, {}) \
             = {}; out = {:?}",
            out_old, xs, divisor, carry, carry_out, out
        );
    }
}

fn demo_limbs_div_divisor_of_limb_max_with_carry_in_place(gm: GenerationMode, limit: usize) {
    for (xs, divisor, carry) in triples_of_limb_vec_limb_and_limb_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry_out = limbs_div_divisor_of_limb_max_with_carry_in_place(&mut xs, divisor, carry);
        println!(
            "xs := {:?}; limbs_div_divisor_of_limb_max_with_carry_in_place(&mut xs, {}, {}) = {}; \
             xs = {:?}",
            xs_old, divisor, carry, carry_out, xs
        );
    }
}

fn demo_limbs_div_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_limb_vecs_and_limb_var_1(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; limbs_div_schoolbook(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_limb_vecs_and_limb_var_2(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; limbs_div_divide_and_conquer(&mut qs, &mut ns, {:?}, {}) = \
             {}; qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_barrett(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, ds) in triples_of_limb_vec_var_42(gm).take(limit) {
        let old_qs = qs.clone();
        let mut scratch = vec![0; limbs_div_barrett_scratch_len(ns.len(), ds.len())];
        let highest_q = limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; ns := {:?}; \
             limbs_div_barrett(&mut qs, ns, {:?}, &mut scratch) = {}; qs = {:?}",
            old_qs, ns, ds, highest_q, qs
        );
    }
}

fn demo_limbs_div_schoolbook_approx(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_limb_vecs_and_limb_var_1(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             limbs_div_schoolbook_approx(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_divide_and_conquer_approx(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_limb_vecs_and_limb_var_2(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_barrett_approx(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, ds) in triples_of_limb_vec_var_41(gm).take(limit) {
        let old_qs = qs.clone();
        let mut scratch = vec![0; limbs_div_barrett_approx_scratch_len(ns.len(), ds.len())];
        let highest_q = limbs_div_barrett_approx(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; ns := {:?}; \
             limbs_div_barrett_approx(&mut qs, ns, {:?}, &mut scratch) = {}; qs = {:?}",
            old_qs, ns, ds, highest_q, qs
        );
    }
}

fn demo_limbs_div(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_limb_vec_var_9(gm).take(limit) {
        println!("limbs_div({:?}, {:?}) = {:?}", ns, ds, limbs_div(&ns, &ds));
    }
}

fn demo_limbs_div_to_out(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, mut ds) in triples_of_limb_vec_var_43(gm).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let old_ds = ds.clone();
        limbs_div_to_out(&mut qs, &mut ns, &mut ds);
        println!(
            "qs := {:?}; ns := {:?}; ds := {:?}; limbs_div_to_out(&mut qs, &mut ns, &mut ds); \
             qs = {:?}",
            old_qs, old_ns, old_ds, qs,
        );
    }
}

fn demo_limbs_div_to_out_val_ref(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds) in triples_of_limb_vec_var_43(gm).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        limbs_div_to_out_val_ref(&mut qs, &mut ns, &ds);
        println!(
            "qs := {:?}; ns := {:?}; limbs_div_to_out_val_ref(&mut qs, &mut ns, {:?}); qs = {:?}",
            old_qs, old_ns, ds, qs,
        );
    }
}

fn demo_limbs_div_to_out_ref_val(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, mut ds) in triples_of_limb_vec_var_43(gm).take(limit) {
        let old_qs = qs.clone();
        let old_ds = ds.clone();
        limbs_div_to_out_ref_val(&mut qs, &ns, &mut ds);
        println!(
            "qs := {:?}; ds := {:?}; limbs_div_to_out_ref_val(&mut qs, {:?}, &mut ds); qs = {:?}",
            old_qs, old_ds, ns, qs,
        );
    }
}

fn demo_limbs_div_to_out_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, ds) in triples_of_limb_vec_var_43(gm).take(limit) {
        let old_qs = qs.clone();
        limbs_div_to_out_ref_ref(&mut qs, &ns, &ds);
        println!(
            "qs := {:?}; limbs_div_to_out_ref_ref(&mut qs, {:?}, {:?}); qs = {:?}",
            old_qs, ns, ds, qs,
        );
    }
}

fn demo_natural_div_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x /= y;
        println!("x := {}; x /= {}; x = {}", x_old, y_old, x);
    }
}

fn demo_natural_div_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        x /= &y;
        println!("x := {}; x /= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_div(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} / {} = {}", x_old, y_old, x / y);
    }
}

fn demo_natural_div_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{} / &{} = {}", x_old, y, x / &y);
    }
}

fn demo_natural_div_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} / {} = {}", x, y_old, &x / y);
    }
}

fn demo_natural_div_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("&{} / &{} = {}", x, y, &x / &y);
    }
}

fn benchmark_limbs_div_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_div_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(limbs, limb)| no_out!(limbs_div_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_div_limb_to_out_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_div_limb_to_out(&mut [Limb], &[Limb], Limb)",
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
                &mut (|(mut out, in_limbs, limb)| limbs_div_limb_to_out(&mut out, &in_limbs, limb)),
            ),
            (
                "alt",
                &mut (|(mut out, in_limbs, limb)| {
                    limbs_div_limb_to_out_alt(&mut out, &in_limbs, limb)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_limb_in_place_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_div_limb_in_place(&mut [Limb], Limb)",
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
                &mut (|(mut limbs, limb)| limbs_div_limb_in_place(&mut limbs, limb)),
            ),
            (
                "alt",
                &mut (|(mut limbs, limb)| limbs_div_limb_in_place_alt(&mut limbs, limb)),
            ),
        ],
    );
}

fn benchmark_limbs_div_divisor_of_limb_max_with_carry_to_out(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_div_divisor_of_limb_max_with_carry_to_out(&mut [Limb], &[Limb], Limb, Limb)",
        BenchmarkType::Single,
        quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut out, xs, divisor, carry)| {
                no_out!(limbs_div_divisor_of_limb_max_with_carry_to_out(
                    &mut out, &xs, divisor, carry
                ))
            }),
        )],
    );
}

fn benchmark_limbs_div_divisor_of_limb_max_with_carry_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_div_divisor_of_limb_max_with_carry_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut xs, divisor, carry)| {
                no_out!(limbs_div_divisor_of_limb_max_with_carry_in_place(
                    &mut xs, divisor, carry
                ))
            }),
        )],
    );
}

fn benchmark_limbs_div_schoolbook_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "_limbs_div_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_limb_vecs_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "Schoolbook div/mod",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "Schoolbook div",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_divide_and_conquer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_div_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_limb_vecs_and_limb_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "Schoolbook div",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "divide-and-conquer div/mod",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_mod_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "divide-and-conquer div",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_barrett_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "_limbs_div_barrett(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        quadruples_of_three_limb_vecs_and_limb_var_2(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "divide-and-conquer div",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "Barrett div/mod",
                &mut (|(mut qs, ns, ds, _)| {
                    let mut rs = vec![0; ds.len()];
                    let mut scratch =
                        vec![0; limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                    no_out!(limbs_div_mod_barrett(
                        &mut qs,
                        &mut rs,
                        &ns,
                        &ds,
                        &mut scratch
                    ))
                }),
            ),
            (
                "Barrett div",
                &mut (|(mut qs, ns, ds, _)| {
                    let mut scratch = vec![0; limbs_div_barrett_scratch_len(ns.len(), ds.len())];
                    no_out!(limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_schoolbook_approx_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_div_schoolbook_approx(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_limb_vecs_and_limb_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "Schoolbook",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "Schoolbook approx",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_divide_and_conquer_approx_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_div_divide_and_conquer_approx(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_limb_vecs_and_limb_var_2(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "Schoolbook approx",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "divide-and-conquer approx",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(limbs_div_divide_and_conquer_approx(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_barrett_approx_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_div_barrett_approx(&mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_limb_vecs_and_limb_var_2(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "divide-and-conquer approx",
                &mut (|(mut qs, mut ns, ds, _)| {
                    // recompute inverse to make benchmark fair
                    let inverse = limbs_two_limb_inverse_helper(ds[ds.len() - 1], ds[ds.len() - 2]);
                    no_out!(limbs_div_divide_and_conquer_approx(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "Barrett",
                &mut (|(mut qs, ns, ds, _)| {
                    let mut scratch = vec![0; limbs_div_barrett_scratch_len(ns.len(), ds.len())];
                    no_out!(limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch))
                }),
            ),
            (
                "Barrett approx",
                &mut (|(mut qs, ns, ds, _)| {
                    let mut scratch =
                        vec![0; limbs_div_barrett_approx_scratch_len(ns.len(), ds.len())];
                    no_out!(limbs_div_barrett_approx(&mut qs, &ns, &ds, &mut scratch))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_div(&[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        pairs_of_limb_vec_var_9(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "div_mod",
                &mut (|(ns, ds)| no_out!(limbs_div_mod(&ns, &ds))),
            ),
            ("div", &mut (|(ns, ds)| no_out!(limbs_div(&ns, &ds)))),
        ],
    );
}

fn benchmark_limbs_div_to_out_balancing_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_div_to_out(&mut [Limb], &mut [Limb], &mut [Limb]) balancing",
        BenchmarkType::Algorithms,
        triples_of_limb_vec_var_44(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds)| max(2, (ds.len() << 1).saturating_sub(ns.len()))),
        "max(2, 2 * ds.len() - ns.len())",
        &mut [
            (
                "unbalanced",
                &mut (|(mut qs, mut ns, mut ds)| {
                    limbs_div_to_out_unbalanced(&mut qs, &mut ns, &mut ds)
                }),
            ),
            (
                "balanced",
                &mut (|(mut qs, ns, ds)| limbs_div_to_out_balanced(&mut qs, &ns, &ds)),
            ),
        ],
    );
}

fn benchmark_limbs_div_to_out_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_div_to_out(&mut [Limb], &mut [Limb], &mut [Limb])",
        BenchmarkType::EvaluationStrategy,
        triples_of_limb_vec_var_43(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "limbs_div_to_out(&mut [Limb], &mut [Limb], &mut [Limb])",
                &mut (|(mut qs, mut ns, mut ds)| limbs_div_to_out(&mut qs, &mut ns, &mut ds)),
            ),
            (
                "limbs_div_to_out_val_ref(&mut [Limb], &mut [Limb], &[Limb])",
                &mut (|(mut qs, mut ns, ds)| limbs_div_to_out_val_ref(&mut qs, &mut ns, &ds)),
            ),
            (
                "limbs_div_to_out_ref_val(&mut [Limb], &[Limb], &mut [Limb])",
                &mut (|(mut qs, ns, mut ds)| limbs_div_to_out_ref_val(&mut qs, &ns, &mut ds)),
            ),
            (
                "limbs_div_to_out_ref_ref(&mut [Limb], &[Limb], &[Limb])",
                &mut (|(mut qs, ns, ds)| limbs_div_to_out_ref_ref(&mut qs, &ns, &ds)),
            ),
        ],
    );
}

fn benchmark_limbs_div_to_out_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_div_to_out_ref_ref(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        quadruples_of_limb_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "div_mod",
                &mut (|(mut qs, mut rs, ns, ds)| limbs_div_mod_to_out(&mut qs, &mut rs, &ns, &ds)),
            ),
            (
                "div",
                &mut (|(mut qs, _, ns, ds)| limbs_div_to_out_ref_ref(&mut qs, &ns, &ds)),
            ),
        ],
    );
}

fn benchmark_natural_div_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural /= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Natural /= Natural", &mut (|(mut x, y)| x /= y)),
            ("Natural /= &Natural", &mut (|(mut x, y)| x /= &y)),
        ],
    );
}

fn benchmark_natural_div_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural / Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, _, (x, y))| no_out!(x / y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x / &y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x / y))),
        ],
    );
}

fn benchmark_natural_div_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural / Natural",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x / y))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).0))),
        ],
    );
}

fn benchmark_natural_div_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural / Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Natural / Natural", &mut (|(x, y)| no_out!(x / y))),
            ("Natural / &Natural", &mut (|(x, y)| no_out!(x / &y))),
            ("&Natural / Natural", &mut (|(x, y)| no_out!(&x / y))),
            ("&Natural / &Natural", &mut (|(x, y)| no_out!(&x / &y))),
        ],
    );
}
