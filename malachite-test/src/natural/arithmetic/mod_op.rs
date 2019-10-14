use malachite_base::num::arithmetic::traits::{DivMod, Mod, ModAssign, NegMod, NegModAssign};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_mod_barrett, _limbs_div_mod_barrett_scratch_len, _limbs_div_mod_divide_and_conquer,
    _limbs_div_mod_schoolbook, limbs_div_mod_by_two_limb_normalized, limbs_div_mod_to_out,
};
use malachite_nz::natural::arithmetic::mod_op::{
    _limbs_mod_barrett, _limbs_mod_divide_and_conquer, _limbs_mod_schoolbook, limbs_mod,
    limbs_mod_by_two_limb_normalized, limbs_mod_three_limb_by_two_limb, limbs_mod_to_out,
};
use num::Integer;
use rug::ops::RemRounding;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_var_10, pairs_of_unsigned_vec_var_9,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_1,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_2, quadruples_of_unsigned_vec_var_1,
    quadruples_of_unsigned_vec_var_2, sextuples_of_limbs_var_1,
    triples_of_two_unsigned_vecs_and_unsigned_var_1, triples_of_unsigned_vec_var_37,
    triples_of_unsigned_vec_var_45,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural,
    rm_pairs_of_natural_and_positive_natural,
};

// For `Natural`s, `mod` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_three_limb_by_two_limb);
    register_demo!(registry, demo_limbs_mod_by_two_limb_normalized);
    register_demo!(registry, demo_limbs_mod_schoolbook);
    register_demo!(registry, demo_limbs_mod_divide_and_conquer);
    register_demo!(registry, demo_limbs_mod_barrett);
    register_demo!(registry, demo_limbs_mod);
    register_demo!(registry, demo_limbs_mod_to_out);
    register_demo!(registry, demo_natural_mod_assign);
    register_demo!(registry, demo_natural_mod_assign_ref);
    register_demo!(registry, demo_natural_mod);
    register_demo!(registry, demo_natural_mod_val_ref);
    register_demo!(registry, demo_natural_mod_ref_val);
    register_demo!(registry, demo_natural_mod_ref_ref);
    register_demo!(registry, demo_natural_rem_assign);
    register_demo!(registry, demo_natural_rem_assign_ref);
    register_demo!(registry, demo_natural_rem);
    register_demo!(registry, demo_natural_rem_val_ref);
    register_demo!(registry, demo_natural_rem_ref_val);
    register_demo!(registry, demo_natural_rem_ref_ref);
    register_demo!(registry, demo_natural_neg_mod_assign);
    register_demo!(registry, demo_natural_neg_mod_assign_ref);
    register_demo!(registry, demo_natural_neg_mod);
    register_demo!(registry, demo_natural_neg_mod_val_ref);
    register_demo!(registry, demo_natural_neg_mod_ref_val);
    register_demo!(registry, demo_natural_neg_mod_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mod_by_two_limb_normalized_algorithms
    );
    register_bench!(registry, Large, benchmark_limbs_mod_schoolbook_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mod_divide_and_conquer_algorithms
    );
    register_bench!(registry, Large, benchmark_limbs_mod_barrett_algorithms);
    register_bench!(registry, Large, benchmark_limbs_mod);
    register_bench!(registry, Large, benchmark_limbs_mod_to_out_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_library_comparison);
    register_bench!(registry, Large, benchmark_natural_mod_algorithms);
    register_bench!(registry, Large, benchmark_natural_mod_evaluation_strategy);
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_rem_library_comparison);
    register_bench!(registry, Large, benchmark_natural_rem_evaluation_strategy);
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_library_comparison
    );
    //TODO
    /*
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_neg_mod_limb_algorithms
    );*/
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_evaluation_strategy
    );
}

pub fn rug_neg_mod(x: rug::Integer, y: rug::Integer) -> rug::Integer {
    -x.rem_ceil(y)
}

fn demo_limbs_mod_three_limb_by_two_limb(gm: GenerationMode, limit: usize) {
    for (n2, n1, n0, d1, d0, inverse) in sextuples_of_limbs_var_1(gm).take(limit) {
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

fn demo_limbs_mod_by_two_limb_normalized(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_unsigned_vec_var_10(gm).take(limit) {
        println!(
            "limbs_mod_by_two_limb_normalized({:?}, {:?}) = {:?}",
            ns,
            ds,
            limbs_mod_by_two_limb_normalized(&ns, &ds),
        );
    }
}

fn demo_limbs_mod_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut ns, ds, inverse) in triples_of_two_unsigned_vecs_and_unsigned_var_1(gm).take(limit) {
        let old_ns = ns.clone();
        _limbs_mod_schoolbook(&mut ns, &ds, inverse);
        println!(
            "ns := {:?}; _limbs_mod_schoolbook(&mut ns, {:?}, {}); ns = {:?}",
            old_ns, ds, inverse, ns
        );
    }
}

fn demo_limbs_mod_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        _limbs_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_mod_divide_and_conquer(&mut qs, &mut ns, {:?}, {}); ns = {:?}",
            old_qs, old_ns, ds, inverse, ns
        );
    }
}

fn demo_limbs_mod_barrett(gm: GenerationMode, limit: usize) {
    for (mut qs, mut rs, ns, ds) in quadruples_of_unsigned_vec_var_1(gm).take(limit) {
        let old_qs = qs.clone();
        let old_rs = rs.clone();
        let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
        _limbs_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
        println!(
            "qs := {:?}; rs := {:?}; _limbs_mod_barrett(&mut qs, &mut ns, {:?}, {:?}); rs = {:?}",
            old_qs, old_rs, ns, ds, rs
        );
    }
}

fn demo_limbs_mod(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_unsigned_vec_var_9(gm).take(limit) {
        println!("limbs_mod({:?}, {:?}) = {:?}", ns, ds, limbs_mod(&ns, &ds));
    }
}

fn demo_limbs_mod_to_out(gm: GenerationMode, limit: usize) {
    for (mut rs, ns, ds) in triples_of_unsigned_vec_var_45(gm).take(limit) {
        let old_rs = rs.clone();
        limbs_mod_to_out(&mut rs, &ns, &ds);
        println!(
            "rs := {:?}; limbs_mod_to_out(&mut rs, {:?}, {:?}); rs = {:?}",
            old_rs, ns, ds, rs
        );
    }
}

fn demo_natural_mod_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_assign(y);
        println!("x := {}; x.mod_assign({}); x = {}", x_old, y_old, x,);
    }
}

fn demo_natural_mod_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        x.mod_assign(&y);
        println!("x := {}; x.mod_assign(&{}); x = {}", x_old, y, x,);
    }
}

fn demo_natural_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.mod_op({}) = {}", x_old, y_old, x.mod_op(y));
    }
}

fn demo_natural_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.mod_op(&{}) = {}", x_old, y, x.mod_op(&y));
    }
}

fn demo_natural_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).mod_op({}) = {:?}", x, y_old, (&x).mod_op(y));
    }
}

fn demo_natural_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("(&{}).mod_op(&{}) = {:?}", x, y, (&x).mod_op(&y));
    }
}

fn demo_natural_rem_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x %= y;
        println!("x := {}; x %= {}; x = {}", x_old, y_old, x);
    }
}

fn demo_natural_rem_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        x %= &y;
        println!("x := {}; x %= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_rem(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} % {} = {:?}", x_old, y_old, x % y);
    }
}

fn demo_natural_rem_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{} % &{} = {:?}", x_old, y, x % &y);
    }
}

fn demo_natural_rem_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} % {} = {:?}", x, y_old, &x % y);
    }
}

fn demo_natural_rem_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("&{} % &{} = {:?}", x, y, &x % &y);
    }
}

fn demo_natural_neg_mod_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.neg_mod_assign(y);
        println!("x := {}; x.neg_mod_assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_natural_neg_mod_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        x.neg_mod_assign(&y);
        println!("x := {}; x.neg_mod_assign(&{}); x = {}", x_old, y, x);
    }
}

fn demo_natural_neg_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.neg_mod({}) = {}", x_old, y_old, x.neg_mod(y));
    }
}

fn demo_natural_neg_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.neg_mod(&{}) = {}", x_old, y, x.neg_mod(&y));
    }
}

fn demo_natural_neg_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).neg_mod({}) = {}", x, y_old, (&x).neg_mod(y));
    }
}

fn demo_natural_neg_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("(&{}).neg_mod(&{}) = {}", x, y, (&x).neg_mod(&y));
    }
}

fn benchmark_limbs_mod_by_two_limb_normalized_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_mod_by_two_limb_normalized(&[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_37(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "using div/mod",
                &mut (|(mut qs, mut ns, ds)| {
                    no_out!(limbs_div_mod_by_two_limb_normalized(&mut qs, &mut ns, &ds))
                }),
            ),
            (
                "standard",
                &mut (|(_, ns, ds)| no_out!(limbs_mod_by_two_limb_normalized(&ns, &ds))),
            ),
        ],
    );
}

fn benchmark_limbs_mod_schoolbook_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_mod_schoolbook(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "using div/mod",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse))
                }),
            ),
            (
                "standard",
                &mut (|(_, mut ns, ds, inverse)| _limbs_mod_schoolbook(&mut ns, &ds, inverse)),
            ),
        ],
    );
}

fn benchmark_limbs_mod_divide_and_conquer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mod_divide_and_conquer(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
        "ns.len()",
        &mut [
            (
                "Schoolbook",
                &mut (|(_, mut ns, ds, inverse)| _limbs_mod_schoolbook(&mut ns, &ds, inverse)),
            ),
            (
                "divide-and-conquer using div/mod",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    no_out!(_limbs_div_mod_divide_and_conquer(
                        &mut qs, &mut ns, &ds, inverse
                    ))
                }),
            ),
            (
                "divide-and-conquer",
                &mut (|(mut qs, mut ns, ds, inverse)| {
                    _limbs_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mod_barrett_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_mod_barrett(&mut [Limb], &mut [Limb], &[Limb], &[Limb], &mut Limb)",
        BenchmarkType::Algorithms,
        quadruples_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, _, ref ds)| ds.len()),
        "ds.len()",
        &mut [
            (
                "Barrett using div/mod",
                &mut (|(mut qs, mut rs, ns, ds)| {
                    let mut scratch =
                        vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                    _limbs_div_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
                }),
            ),
            (
                "Barrett",
                &mut (|(mut qs, mut rs, ns, ds)| {
                    let mut scratch =
                        vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
                    _limbs_mod_barrett(&mut qs, &mut rs, &ns, &ds, &mut scratch);
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mod(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_9(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref ns, _)| ns.len()),
        "ns.len()",
        &mut [("malachite", &mut (|(ns, ds)| no_out!(limbs_mod(&ns, &ds))))],
    );
}

fn benchmark_limbs_mod_to_out_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        quadruples_of_unsigned_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [
            (
                "using div/mod",
                &mut (|(qs, mut rs, ns, ds)| {
                    // Allocate again to make benchmark fair
                    let mut qs = vec![0; qs.len()];
                    limbs_div_mod_to_out(&mut qs, &mut rs, &ns, &ds);
                }),
            ),
            (
                "standard",
                &mut (|(_, mut rs, ns, ds)| limbs_mod_to_out(&mut rs, &ns, &ds)),
            ),
        ],
    );
}

fn benchmark_natural_mod_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.mod_assign(Natural)",
                &mut (|(mut x, y)| no_out!(x.mod_assign(y))),
            ),
            (
                "Natural.mod_assign(&Natural)",
                &mut (|(mut x, y)| no_out!(x.mod_assign(&y))),
            ),
        ],
    );
}

fn benchmark_natural_mod_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.mod_floor(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.rem_floor(y)))),
        ],
    );
}

fn benchmark_natural_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_natural_mod_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.mod_op(Natural)",
                &mut (|(x, y)| no_out!(x.mod_op(y))),
            ),
            (
                "Natural.mod_op(&Natural)",
                &mut (|(x, y)| no_out!(x.mod_op(&y))),
            ),
            (
                "(&Natural).mod_op(Natural)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
            (
                "(&Natural).mod_op(&Natural)",
                &mut (|(x, y)| no_out!((&x).mod_op(&y))),
            ),
        ],
    );
}

fn benchmark_natural_rem_assign_evaluation_strategy(
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
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural %= Natural", &mut (|(mut x, y)| x %= y)),
            ("Natural %= &Natural", &mut (|(mut x, y)| x %= &y)),
        ],
    );
}

fn benchmark_natural_rem_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural % Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x % y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_natural_rem_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural % Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural % Natural", &mut (|(x, y)| no_out!(x % y))),
            ("Natural % &Natural", &mut (|(x, y)| no_out!(x % &y))),
            ("&Natural % Natural", &mut (|(x, y)| no_out!(&x % y))),
            ("&Natural % &Natural", &mut (|(x, y)| no_out!(&x % &y))),
        ],
    );
}

fn benchmark_natural_neg_mod_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.neg_mod_assign(Natural)",
                &mut (|(mut x, y)| no_out!(x.neg_mod_assign(y))),
            ),
            (
                "Natural.neg_mod_assign(&Natural)",
                &mut (|(mut x, y)| no_out!(x.neg_mod_assign(&y))),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.neg_mod(Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.neg_mod(y)))),
            ("rug", &mut (|((x, y), _)| no_out!(rug_neg_mod(x, y)))),
        ],
    );
}

//TODO
/*
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
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "using div_round and %",
                &mut (|(x, y)| {
                    let remainder = (&x).neg_mod(y);
                    (x.div_round(y, RoundingMode::Ceiling), remainder);
                }),
            ),
        ],
    );
}*/

fn benchmark_natural_neg_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.neg_mod(Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "Natural.neg_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(&y))),
            ),
            (
                "(&Natural).neg_mod(Natural)",
                &mut (|(x, y)| no_out!((&x).neg_mod(y))),
            ),
            (
                "(&Natural).neg_mod(&Natural)",
                &mut (|(x, y)| no_out!((&x).neg_mod(&y))),
            ),
        ],
    );
}
