use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_div, _limbs_modular_div_barrett, _limbs_modular_div_barrett_scratch_len,
    _limbs_modular_div_divide_and_conquer, _limbs_modular_div_mod_barrett,
    _limbs_modular_div_mod_barrett_scratch_len, _limbs_modular_div_mod_divide_and_conquer,
    _limbs_modular_div_mod_schoolbook, _limbs_modular_div_schoolbook,
    _limbs_modular_div_scratch_len, limbs_modular_invert, limbs_modular_invert_scratch_len,
};
use malachite_nz::natural::arithmetic::div_exact_limb::limbs_modular_invert_limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_var_12, quadruples_of_three_unsigned_vecs_and_unsigned_var_3,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_4,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_5,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_6, quadruples_of_unsigned_vec_var_4,
    quadruples_of_unsigned_vec_var_5, triples_of_unsigned_vec_var_50,
    triples_of_unsigned_vec_var_51,
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
    register_bench!(registry, Small, benchmark_limbs_modular_invert);
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
    register_bench!(registry, Small, benchmark_limbs_modular_div);
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
    for (mut qs, ns, ds) in triples_of_unsigned_vec_var_51(gm).take(limit) {
        let qs_old = qs.clone();
        let mut scratch = vec![0; _limbs_modular_div_scratch_len(ns.len(), ds.len())];
        _limbs_modular_div(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; _limbs_modular_div(&mut qs, {:?}, {:?} &mut scratch); qs = {:?}",
            qs_old, ns, ds, qs
        );
    }
}

fn benchmark_limbs_modular_invert(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_modular_invert(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_12(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ds)| ds.len()),
        "ds.len()",
        &mut [(
            "malachite",
            &mut (|(mut is, ds)| {
                let mut scratch = vec![0; limbs_modular_invert_scratch_len(ds.len())];
                limbs_modular_invert(&mut is, &ds, &mut scratch);
            }),
        )],
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
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
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
        quadruples_of_three_unsigned_vecs_and_unsigned_var_6(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
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

fn benchmark_limbs_modular_div(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_modular_div(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_51(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [(
            "Barrett",
            &mut (|(mut qs, ns, ds)| {
                let mut scratch = vec![0; _limbs_modular_div_scratch_len(ns.len(), ds.len())];
                _limbs_modular_div(&mut qs, &ns, &ds, &mut scratch)
            }),
        )],
    );
}
