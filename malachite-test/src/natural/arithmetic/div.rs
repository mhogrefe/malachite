use malachite_base::num::arithmetic::traits::DivMod;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::div::{
    _limbs_div_barrett, _limbs_div_barrett_approx, _limbs_div_barrett_approx_scratch_len,
    _limbs_div_barrett_scratch_len, _limbs_div_divide_and_conquer,
    _limbs_div_divide_and_conquer_approx, _limbs_div_schoolbook, _limbs_div_schoolbook_approx,
};
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_mod_barrett, _limbs_div_mod_barrett_scratch_len, _limbs_div_mod_divide_and_conquer,
    _limbs_div_mod_schoolbook,
};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    quadruples_of_three_unsigned_vecs_and_unsigned_var_1,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_2, triples_of_unsigned_vec_var_41,
    triples_of_unsigned_vec_var_42,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_div_schoolbook);
    register_demo!(registry, demo_limbs_div_divide_and_conquer);
    register_demo!(registry, demo_limbs_div_barrett);
    register_demo!(registry, demo_limbs_div_schoolbook_approx);
    register_demo!(registry, demo_limbs_div_divide_and_conquer_approx);
    register_demo!(registry, demo_limbs_div_barrett_approx);
    register_demo!(registry, demo_natural_div_assign);
    register_demo!(registry, demo_natural_div_assign_ref);
    register_demo!(registry, demo_natural_div);
    register_demo!(registry, demo_natural_div_val_ref);
    register_demo!(registry, demo_natural_div_ref_val);
    register_demo!(registry, demo_natural_div_ref_ref);
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
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_div_library_comparison);
    register_bench!(registry, Large, benchmark_natural_div_algorithms);
    register_bench!(registry, Large, benchmark_natural_div_evaluation_strategy);
}

fn demo_limbs_div_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_1(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = _limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; _limbs_div_schoolbook(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = _limbs_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; _limbs_div_divide_and_conquer(&mut qs, &mut ns, {:?}, {}) = \
             {}; qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_barrett(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, ds) in triples_of_unsigned_vec_var_42(gm).take(limit) {
        let old_qs = qs.clone();
        let mut scratch = vec![0; _limbs_div_barrett_scratch_len(ns.len(), ds.len())];
        let highest_q = _limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_div_barrett(&mut qs, ns, {:?}, &mut scratch) = {}; qs = {:?}",
            old_qs, ns, ds, highest_q, qs
        );
    }
}

fn demo_limbs_div_schoolbook_approx(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_1(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = _limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_div_schoolbook_approx(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_divide_and_conquer_approx(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let highest_q = _limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, highest_q, qs, ns
        );
    }
}

fn demo_limbs_div_barrett_approx(gm: GenerationMode, limit: usize) {
    for (mut qs, ns, ds) in triples_of_unsigned_vec_var_41(gm).take(limit) {
        let old_qs = qs.clone();
        let mut scratch = vec![0; _limbs_div_barrett_approx_scratch_len(ns.len(), ds.len())];
        let highest_q = _limbs_div_barrett_approx(&mut qs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_div_barrett_approx(&mut qs, ns, {:?}, &mut scratch) = {}; qs = {:?}",
            old_qs, ns, ds, highest_q, qs
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

fn benchmark_limbs_div_schoolbook_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_div_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "Schoolbook div/mod",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "Schoolbook div",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
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
    m_run_benchmark(
        "_limbs_div_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "Schoolbook div",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "divide-and-conquer div/mod",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_mod_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "divide-and-conquer div",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_div_barrett_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_div_barrett(&mut [Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "divide-and-conquer div",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "Barrett div/mod",
                &mut (|(mut qs, ns, ds, _)| {
                    let mut rs = vec![0; ds.len()];
                    let mut scratch =
                        vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                    no_out!(_limbs_div_mod_barrett(
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
                    let mut scratch = vec![0; _limbs_div_barrett_scratch_len(ns.len(), ds.len())];
                    no_out!(_limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch))
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
    m_run_benchmark(
        "_limbs_div_schoolbook_approx(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len() - ds.len()",
        &mut [
            (
                "Schoolbook",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "Schoolbook approx",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse))
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
    m_run_benchmark(
        "_limbs_div_divide_and_conquer_approx(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ds, _)| ds.len()),
        "ns.len()",
        &mut [
            (
                "Schoolbook approx",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "divide-and-conquer approx",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_divide_and_conquer_approx(
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
    m_run_benchmark(
        "_limbs_div_barrett_approx(&mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm.with_scale(2_048)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ds, _)| ds.len()),
        "ns.len()",
        &mut [
            (
                "divide-and-conquer approx",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_divide_and_conquer_approx(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "Barrett",
                &mut (|(mut qs, ns, ds, _)| {
                    let mut scratch = vec![0; _limbs_div_barrett_scratch_len(ns.len(), ds.len())];
                    no_out!(_limbs_div_barrett(&mut qs, &ns, &ds, &mut scratch))
                }),
            ),
            (
                "Barrett approx",
                &mut (|(mut qs, ns, ds, _)| {
                    let mut scratch =
                        vec![0; _limbs_div_barrett_approx_scratch_len(ns.len(), ds.len())];
                    no_out!(_limbs_div_barrett_approx(&mut qs, &ns, &ds, &mut scratch))
                }),
            ),
        ],
    );
}

fn benchmark_natural_div_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural /= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural /= Natural", &mut (|(mut x, y)| x /= y)),
            ("Natural /= &Natural", &mut (|(mut x, y)| x /= &y)),
        ],
    );
}

fn benchmark_natural_div_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural / Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x / y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x / &y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x / y))),
        ],
    );
}

fn benchmark_natural_div_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural / Natural",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x / y))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).0))),
        ],
    );
}

fn benchmark_natural_div_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural / Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural / Natural", &mut (|(x, y)| no_out!(x / y))),
            ("Natural / &Natural", &mut (|(x, y)| no_out!(x / &y))),
            ("&Natural / Natural", &mut (|(x, y)| no_out!(&x / y))),
            ("&Natural / &Natural", &mut (|(x, y)| no_out!(&x / &y))),
        ],
    );
}
