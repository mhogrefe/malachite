use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigneds_var_2, quadruples_of_three_unsigned_vecs_and_unsigned_var_1,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_2,
    quintuples_of_three_unsigned_vecs_unsigned_and_unsigned_vec_var_1, sextuples_of_limbs_var_1,
    triples_of_unsigned_vec_var_37,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural,
    rm_pairs_of_natural_and_positive_natural,
};
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_mod_divide_and_conquer, _limbs_div_mod_divide_and_conquer_helper,
    _limbs_div_mod_schoolbook, limbs_div_mod_by_two_limb, limbs_div_mod_three_limb_by_two_limb,
    limbs_two_limb_inverse_helper,
};
use num::Integer;

// For `Natural`s, `mod` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_two_limb_inverse_helper);
    register_demo!(registry, demo_limbs_div_mod_three_limb_by_two_limb);
    register_demo!(registry, demo_limbs_div_mod_by_two_limb);
    register_demo!(registry, demo_limbs_div_mod_schoolbook);
    register_demo!(registry, demo_limbs_div_mod_divide_and_conquer_helper);
    register_demo!(registry, demo_limbs_div_mod_divide_and_conquer);
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
    register_bench!(registry, Small, benchmark_limbs_div_mod_by_two_limb);
    register_bench!(registry, Small, benchmark_limbs_div_mod_schoolbook);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_mod_divide_and_conquer_helper
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_mod_divide_and_conquer_algorithms
    );
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
    //TODO register_bench!(registry, Large, benchmark_natural_div_mod_algorithms);
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
        benchmark_natural_ceiling_div_neg_mod_evaluation_strategy
    );
}

pub fn rug_ceiling_div_neg_mod(x: rug::Integer, y: rug::Integer) -> (rug::Integer, rug::Integer) {
    let (quotient, remainder) = x.div_rem_ceil(y);
    (quotient, -remainder)
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

fn demo_limbs_div_mod_by_two_limb(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds) in triples_of_unsigned_vec_var_37(gm).take(limit) {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let q_hi = limbs_div_mod_by_two_limb(&mut qs, &mut ns, &ds);
        println!(
            "qs := {:?}; ns := {:?}; limbs_div_mod_by_two_limb(&mut qs, &mut ns, {:?}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, q_hi, qs, ns
        );
    }
}

fn demo_limbs_div_mod_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_1(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let q_hi = _limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; _limbs_div_mod_schoolbook(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, q_hi, qs, ns
        );
    }
}

fn demo_limbs_div_mod_divide_and_conquer_helper(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse, mut scratch) in
        quintuples_of_three_unsigned_vecs_unsigned_and_unsigned_vec_var_1(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let old_scratch = scratch.clone();
        let q_hi =
            _limbs_div_mod_divide_and_conquer_helper(&mut qs, &mut ns, &ds, inverse, &mut scratch);
        println!(
            "qs := {:?}; ns := {:?}; scratch = {:?}; _limbs_div_mod_divide_and_conquer_helper(\
             &mut qs, &mut ns, {:?}, {}, &mut scratch) = {}; qs = {:?}, ns = {:?}, scratch = {:?}",
            old_qs, old_ns, old_scratch, ds, inverse, q_hi, qs, ns, scratch,
        );
    }
}

fn demo_limbs_div_mod_divide_and_conquer(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm).take(limit)
    {
        let old_qs = qs.clone();
        let old_ns = ns.clone();
        let q_hi = _limbs_div_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_div_mod_divide_and_conquer(&mut qs, &mut ns, {:?}, {}) = {}; \
             qs = {:?}, ns = {:?}",
            old_qs, old_ns, ds, inverse, q_hi, qs, ns
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

fn benchmark_limbs_div_mod_by_two_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_mod_by_two_limb(&mut [Limb], &mut [Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_37(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut ns, ds)| no_out!(limbs_div_mod_by_two_limb(&mut qs, &mut ns, &ds))),
        )],
    );
}

fn benchmark_limbs_div_mod_schoolbook(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "_limbs_div_mod_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_1(gm),
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

fn benchmark_limbs_div_mod_divide_and_conquer_helper(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_div_mod_divide_and_conquer_helper(&mut [Limb], &mut [Limb], &[Limb], Limb, \
         &mut [Limb])",
        BenchmarkType::Single,
        quintuples_of_three_unsigned_vecs_unsigned_and_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut ns, ds, inverse, mut scratch)| {
                no_out!(_limbs_div_mod_divide_and_conquer_helper(
                    &mut qs,
                    &mut ns,
                    &ds,
                    inverse,
                    &mut scratch
                ))
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
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
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
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_mod(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.div_mod_floor(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_rem_floor(y)))),
        ],
    );
}

//TODO
/*
fn benchmark_natural_div_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_mod(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.div_mod(y)))),
            ("naive", &mut (|(x, y)| no_out!(x._div_mod_limb_naive(y)))),
            (
                "using / and %",
                &mut (|(x, y)| {
                    let remainder = &x % y;
                    (x / y, remainder);
                }),
            ),
        ],
    );
}*/

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
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
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
