use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::div::limbs_div_to_out;
use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_div, _limbs_modular_div_barrett, _limbs_modular_div_barrett_scratch_len,
    _limbs_modular_div_divide_and_conquer, _limbs_modular_div_mod_barrett,
    _limbs_modular_div_mod_barrett_scratch_len, _limbs_modular_div_mod_divide_and_conquer,
    _limbs_modular_div_mod_schoolbook, _limbs_modular_div_ref, _limbs_modular_div_ref_scratch_len,
    _limbs_modular_div_schoolbook, _limbs_modular_div_scratch_len, _limbs_modular_invert_small,
    limbs_div_exact_to_out, limbs_div_exact_to_out_ref_ref, limbs_div_exact_to_out_ref_val,
    limbs_div_exact_to_out_val_ref, limbs_modular_invert, limbs_modular_invert_scratch_len,
};
use malachite_nz::natural::arithmetic::div_exact_limb::limbs_modular_invert_limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_var_12, quadruples_of_three_unsigned_vecs_and_unsigned_var_3,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_4,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_5,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_6,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_7, quadruples_of_unsigned_vec_var_4,
    quadruples_of_unsigned_vec_var_5, triples_of_unsigned_vec_var_50,
    triples_of_unsigned_vec_var_51, triples_of_unsigned_vec_var_53, triples_of_unsigned_vec_var_54,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_natural_var_1, pairs_of_natural_and_positive_natural_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_modular_invert);
    register_demo!(registry, demo_limbs_modular_div_mod_schoolbook);
    register_demo!(registry, demo_limbs_modular_div_mod_divide_and_conquer);
    register_demo!(registry, demo_limbs_modular_div_mod_barrett);
    register_demo!(registry, demo_limbs_modular_div_schoolbook);
    register_demo!(registry, demo_limbs_modular_div_divide_and_conquer);
    register_demo!(registry, demo_limbs_modular_div_barrett);
    register_demo!(registry, demo_limbs_modular_div);
    register_demo!(registry, demo_limbs_modular_div_ref);
    register_demo!(registry, demo_limbs_div_exact_to_out);
    register_demo!(registry, demo_limbs_div_exact_to_out_val_ref);
    register_demo!(registry, demo_limbs_div_exact_to_out_ref_val);
    register_demo!(registry, demo_limbs_div_exact_to_out_ref_ref);
    register_demo!(registry, demo_natural_div_exact_assign);
    register_demo!(registry, demo_natural_div_exact_assign_ref);
    register_demo!(registry, demo_natural_div_exact);
    register_demo!(registry, demo_natural_div_exact_val_ref);
    register_demo!(registry, demo_natural_div_exact_ref_val);
    register_demo!(registry, demo_natural_div_exact_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_modular_invert_algorithms);
    register_bench!(registry, Small, benchmark_limbs_modular_div_mod_schoolbook);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_modular_div_mod_divide_and_conquer_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_modular_div_mod_barrett_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_modular_div_schoolbook);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_modular_div_divide_and_conquer_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_modular_div_barrett_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_modular_div_evaluation_strategy
    );
    register_bench!(registry, Small, benchmark_limbs_div_exact_to_out_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_exact_to_out_evaluation_strategy
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_div_exact_assign_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_div_exact_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_div_exact_library_comparison
    );
    register_bench!(registry, Small, benchmark_natural_div_exact_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_natural_div_exact_evaluation_strategy
    );
}

fn demo_limbs_modular_invert(gm: GenerationMode, limit: usize) {
    for (mut is, ds) in pairs_of_unsigned_vec_var_12(gm).take(limit) {
        let old_is = is.clone();
        let mut scratch = vec![0; limbs_modular_invert_scratch_len(ds.len())];
        limbs_modular_invert(&mut is, &ds, &mut scratch);
        println!(
            "is := {:?}; _limbs_modular_invert(&mut is, {:?}, &mut scratch); is = {:?}, ",
            old_is, ds, is,
        );
    }
}

fn demo_limbs_modular_div_mod_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_4(gm).take(limit)
    {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        let borrow = _limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, {:?}, {}) = {}; qs = {:?}; \
             ns = {:?}",
            qs_old, ns_old, ds, inverse, borrow, qs, ns
        );
    }
}

fn demo_limbs_modular_div_mod_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_5(gm).take(limit)
    {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        let borrow = _limbs_modular_div_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_modular_div_mod_divide_and_conquer(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}; ns = {:?}",
            qs_old, ns_old, ds, inverse, borrow, qs, ns
        );
    }
}

fn demo_limbs_modular_div_mod_barrett(gm: GenerationMode, limit: usize) {
    for (mut qs, mut rs, ns, ds) in quadruples_of_unsigned_vec_var_4(gm).take(limit) {
        let qs_old = qs.clone();
        let rs_old = rs.clone();
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(ns.len(), ds.len())];
        let borrow = _limbs_modular_div_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; rs := {:?}; \
             _limbs_modular_div_mod_divide_and_conquer(\
             &mut qs, &mut rs, {:?}, {:?} &mut scratch) = {}; qs = {:?}; rs = {:?}",
            qs_old, rs_old, ns, ds, borrow, qs, rs
        );
    }
}

fn demo_limbs_modular_div_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_3(gm).take(limit)
    {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        _limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; _limbs_modular_div_schoolbook(&mut qs, &mut ns, {:?}, {}); \
             qs = {:?}",
            qs_old, ns_old, ds, inverse, qs
        );
    }
}

fn demo_limbs_modular_div_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_6(gm).take(limit)
    {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        _limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, {:?}, {}); qs = {:?}",
            qs_old, ns_old, ds, inverse, qs
        );
    }
}

fn demo_limbs_modular_div_barrett(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, ds) in triples_of_unsigned_vec_var_50(gm).take(limit) {
        let qs_old = qs.clone();
        let mut scratch = vec![0; _limbs_modular_div_barrett_scratch_len(ns.len(), ds.len())];
        _limbs_modular_div_barrett(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; _limbs_modular_div_barrett(&mut qs, {:?}, {:?} &mut scratch); qs = {:?}",
            qs_old, ns, ds, qs
        );
    }
}

fn demo_limbs_modular_div(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds) in triples_of_unsigned_vec_var_51(gm).take(limit) {
        let ns_old = ns.clone();
        let qs_old = qs.clone();
        let mut scratch = vec![0; _limbs_modular_div_scratch_len(ns.len(), ds.len())];
        _limbs_modular_div(&mut qs, &mut ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; _limbs_modular_div(&mut qs, {:?}, {:?} &mut scratch); qs = {:?}",
            qs_old, ns_old, ds, qs
        );
    }
}

fn demo_limbs_modular_div_ref(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, ds) in triples_of_unsigned_vec_var_51(gm).take(limit) {
        let qs_old = qs.clone();
        let mut scratch = vec![0; _limbs_modular_div_ref_scratch_len(ns.len(), ds.len())];
        _limbs_modular_div_ref(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; _limbs_modular_div_ref(&mut qs, {:?}, {:?} &mut scratch); qs = {:?}",
            qs_old, ns, ds, qs
        );
    }
}

fn demo_limbs_div_exact_to_out(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, mut ds) in triples_of_unsigned_vec_var_53(gm).take(limit) {
        let ns_old = ns.clone();
        let ds_old = ds.clone();
        let qs_old = qs.clone();
        limbs_div_exact_to_out(&mut qs, &mut ns, &mut ds);
        println!(
            "qs := {:?}; limbs_div_exact_to_out(&mut qs, {:?}, {:?}); qs = {:?}",
            qs_old, ns_old, ds_old, qs
        );
    }
}

fn demo_limbs_div_exact_to_out_val_ref(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds) in triples_of_unsigned_vec_var_53(gm).take(limit) {
        let ns_old = ns.clone();
        let qs_old = qs.clone();
        limbs_div_exact_to_out_val_ref(&mut qs, &mut ns, &ds);
        println!(
            "qs := {:?}; limbs_div_exact_to_out_val_ref(&mut qs, {:?}, {:?}); qs = {:?}",
            qs_old, ns_old, ds, qs
        );
    }
}

fn demo_limbs_div_exact_to_out_ref_val(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, mut ds) in triples_of_unsigned_vec_var_53(gm).take(limit) {
        let ds_old = ds.clone();
        let qs_old = qs.clone();
        limbs_div_exact_to_out_ref_val(&mut qs, &ns, &mut ds);
        println!(
            "qs := {:?}; limbs_div_exact_to_out_ref_val(&mut qs, {:?}, {:?}); qs = {:?}",
            qs_old, ns, ds_old, qs
        );
    }
}

fn demo_limbs_div_exact_to_out_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, ds) in triples_of_unsigned_vec_var_53(gm).take(limit) {
        let qs_old = qs.clone();
        limbs_div_exact_to_out_ref_ref(&mut qs, &ns, &ds);
        println!(
            "qs := {:?}; limbs_div_exact_to_out_ref_ref(&mut qs, {:?}, {:?}); qs = {:?}",
            qs_old, ns, ds, qs
        );
    }
}

fn demo_natural_div_exact_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.div_exact_assign(y);
        println!("x := {}; x.div_exact_assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_natural_div_exact_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural_var_1(gm).take(limit) {
        let x_old = x.clone();
        x.div_exact_assign(&y);
        println!("x := {}; x.div_exact_assign(&{}); x = {}", x_old, y, x);
    }
}

fn demo_natural_div_exact(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_exact({}) = {}", x_old, y_old, x.div_exact(y));
    }
}

fn demo_natural_div_exact_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural_var_1(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.div_exact(&{}) = {}", x_old, y, x.div_exact(&y));
    }
}

fn demo_natural_div_exact_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural_var_1(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_exact({}) = {}", x, y_old, (&x).div_exact(y));
    }
}

fn demo_natural_div_exact_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural_var_1(gm).take(limit) {
        println!("(&{}).div_exact(&{}) = {}", x, y, (&x).div_exact(&y));
    }
}

fn benchmark_limbs_modular_invert_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_modular_invert(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_7(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ds, _)| ds.len()),
        "ds.len()",
        &mut [
            (
                "modular invert small",
                &mut (|(mut is, mut scratch, ds, inverse)| {
                    let n = ds.len();
                    _limbs_modular_invert_small(n, &mut is, &mut scratch[..n], &ds, inverse);
                }),
            ),
            (
                "modular invert",
                &mut (|(mut is, mut scratch, ds, _)| {
                    limbs_modular_invert(&mut is, &ds, &mut scratch);
                }),
            ),
        ],
    );
}

fn benchmark_limbs_modular_div_mod_schoolbook(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_modular_div_mod_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut ns, ds, inverse)| {
                no_out!(_limbs_modular_div_mod_schoolbook(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
        )],
    );
}

fn benchmark_limbs_modular_div_mod_divide_and_conquer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_modular_div_mod_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_5(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ds, _)| ds.len()),
        "ds.len()",
        &mut [
            (
                "schoolbook",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_modular_div_mod_schoolbook(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_modular_div_mod_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_modular_div_mod_barrett_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_modular_div_mod_barrett(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        quadruples_of_unsigned_vec_var_5(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "divide-and-conquer",
                &mut (|(mut qs, _, mut ns, ds)| {
                    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
                    no_out!(_limbs_modular_div_mod_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "Barrett",
                &mut (|(mut qs, mut rs, ns, ds)| {
                    let mut scratch =
                        vec![0; _limbs_modular_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                    no_out!(_limbs_modular_div_mod_barrett(
                        &mut qs,
                        &mut rs,
                        &ns,
                        &ds,
                        &mut scratch
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_modular_div_schoolbook(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_modular_div_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut ns, ds, inverse)| {
                _limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse)
            }),
        )],
    );
}

fn benchmark_limbs_modular_div_divide_and_conquer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_modular_div_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_6(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ds, _)| ds.len()),
        "ds.len()",
        &mut [
            (
                "schoolbook",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    _limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse)
                }),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    _limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_modular_div_barrett_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_modular_div_barrett(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_50(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "divide-and-conquer",
                &mut (|(mut qs, mut ns, ds)| {
                    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
                    _limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse)
                }),
            ),
            (
                "Barrett",
                &mut (|(mut qs, ns, ds)| {
                    let mut scratch =
                        vec![0; _limbs_modular_div_barrett_scratch_len(ns.len(), ds.len())];
                    _limbs_modular_div_barrett(&mut qs, &ns, &ds, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_modular_div_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_modular_div(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_vec_var_51(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "limbs_modular_div(&mut [Limb], &mut [Limb], &[Limb], &mut [Limb])",
                &mut (|(mut qs, mut ns, ds)| {
                    let mut scratch = vec![0; _limbs_modular_div_scratch_len(ns.len(), ds.len())];
                    _limbs_modular_div(&mut qs, &mut ns, &ds, &mut scratch)
                }),
            ),
            (
                "limbs_modular_div_ref(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
                &mut (|(mut qs, ns, ds)| {
                    let mut scratch =
                        vec![0; _limbs_modular_div_ref_scratch_len(ns.len(), ds.len())];
                    _limbs_modular_div_ref(&mut qs, &ns, &ds, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_exact_to_out_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_exact_to_out(&mut [Limb], &mut [Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_54(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "div",
                &mut (|(mut qs, mut ns, mut ds)| limbs_div_to_out(&mut qs, &mut ns, &mut ds)),
            ),
            (
                "div exact",
                &mut (|(mut qs, mut ns, mut ds)| limbs_div_exact_to_out(&mut qs, &mut ns, &mut ds)),
            ),
        ],
    );
}

fn benchmark_limbs_div_exact_to_out_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_div_exact_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_vec_var_54(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "limbs_div_exact_to_out(&mut [Limb], &mut [Limb], &mut [Limb])",
                &mut (|(mut qs, mut ns, mut ds)| limbs_div_exact_to_out(&mut qs, &mut ns, &mut ds)),
            ),
            (
                "limbs_div_exact_to_out_val_ref(&mut [Limb], &mut [Limb], &[Limb])",
                &mut (|(mut qs, mut ns, ds)| limbs_div_exact_to_out_val_ref(&mut qs, &mut ns, &ds)),
            ),
            (
                "limbs_div_exact_to_out_ref_val(&mut [Limb], &[Limb], &mut [Limb])",
                &mut (|(mut qs, ns, mut ds)| limbs_div_exact_to_out_ref_val(&mut qs, &ns, &mut ds)),
            ),
            (
                "limbs_div_exact_to_out_ref_ref(&mut [Limb], &[Limb], &[Limb])",
                &mut (|(mut qs, ns, ds)| limbs_div_exact_to_out_ref_ref(&mut qs, &ns, &ds)),
            ),
        ],
    );
}

fn benchmark_natural_div_exact_assign_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_exact_assign(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(mut x, y)| x /= y)),
            ("exact division", &mut (|(mut x, y)| x.div_exact_assign(y))),
        ],
    );
}

fn benchmark_natural_div_exact_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_exact_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_exact_assign(Natural)",
                &mut (|(mut x, y)| x.div_exact_assign(y)),
            ),
            (
                "Natural.div_exact_assign(&Natural)",
                &mut (|(mut x, y)| x.div_exact_assign(&y)),
            ),
        ],
    );
}

fn benchmark_natural_div_exact_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_exact(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("num", &mut (|((x, y), _, _)| no_out!(x / y))),
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_exact(y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_exact(&y)))),
        ],
    );
}

fn benchmark_natural_div_exact_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_exact(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(x, y)| no_out!(x / y))),
            ("exact division", &mut (|(x, y)| no_out!(x.div_exact(y)))),
        ],
    );
}

fn benchmark_natural_div_exact_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_exact(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_exact(Natural)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "Natural.div_exact(&Natural)",
                &mut (|(x, y)| no_out!(x.div_exact(&y))),
            ),
            (
                "(&Natural).div_exact(Natural)",
                &mut (|(x, y)| no_out!((&x).div_exact(y))),
            ),
            (
                "(&Natural).div_exact(&Natural)",
                &mut (|(x, y)| no_out!((&x).div_exact(&y))),
            ),
        ],
    );
}
