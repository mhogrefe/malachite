use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem, DivRound,
    NegMod,
};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::div_mod_limb::{
    _limbs_div_limb_in_place_mod_alt, _limbs_div_limb_in_place_mod_naive,
    _limbs_div_limb_to_out_mod_alt, _limbs_div_limb_to_out_mod_naive, limbs_div_limb_in_place_mod,
    limbs_div_limb_mod, limbs_div_limb_to_out_mod, limbs_invert_limb,
};
use malachite_nz::platform::Limb;
use num::{BigUint, Integer, ToPrimitive};
use rug;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1, unsigneds_var_1,
};
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_unsigned, rm_pairs_of_natural_and_positive_unsigned,
};
use inputs::natural::{
    pairs_of_natural_and_positive_unsigned, pairs_of_unsigned_and_positive_natural,
};

// For `Natural`s, `mod` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_invert_limb);
    register_demo!(registry, demo_limbs_div_limb_mod);
    register_demo!(registry, demo_limbs_div_limb_to_out_mod);
    register_demo!(registry, demo_limbs_div_limb_in_place_mod);
    register_demo!(registry, demo_natural_div_assign_mod_limb);
    register_demo!(registry, demo_natural_div_mod_limb);
    register_demo!(registry, demo_natural_div_mod_limb_ref);
    register_demo!(registry, demo_natural_div_assign_rem_limb);
    register_demo!(registry, demo_natural_div_rem_limb);
    register_demo!(registry, demo_natural_div_rem_limb_ref);
    register_demo!(registry, demo_natural_ceiling_div_assign_neg_mod_limb);
    register_demo!(registry, demo_natural_ceiling_div_neg_mod_limb);
    register_demo!(registry, demo_natural_ceiling_div_neg_mod_limb_ref);
    register_demo!(registry, demo_limb_div_mod_natural);
    register_demo!(registry, demo_limb_div_mod_natural_ref);
    register_demo!(registry, demo_limb_div_assign_mod_natural);
    register_demo!(registry, demo_limb_div_assign_mod_natural_ref);
    register_demo!(registry, demo_limb_div_rem_natural);
    register_demo!(registry, demo_limb_div_rem_natural_ref);
    register_demo!(registry, demo_limb_div_assign_rem_natural);
    register_demo!(registry, demo_limb_div_assign_rem_natural_ref);
    register_demo!(registry, demo_limb_ceiling_div_neg_mod_natural);
    register_demo!(registry, demo_limb_ceiling_div_neg_mod_natural_ref);
    register_demo!(registry, demo_limb_ceiling_div_assign_neg_mod_natural);
    register_demo!(registry, demo_limb_ceiling_div_assign_neg_mod_natural_ref);
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
    register_bench!(registry, Large, benchmark_natural_div_assign_mod_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_mod_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_div_mod_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_mod_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_div_assign_rem_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_rem_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_rem_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_assign_neg_mod_limb
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_neg_mod_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_neg_mod_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_neg_mod_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_mod_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_assign_mod_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_rem_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_assign_rem_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_ceiling_div_neg_mod_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_ceiling_div_assign_neg_mod_natural_evaluation_strategy
    );
}

pub fn num_div_mod_u32(x: BigUint, u: u32) -> (BigUint, u32) {
    let (quotient, remainder) = x.div_mod_floor(&BigUint::from(u));
    (quotient, remainder.to_u32().unwrap())
}

pub fn rug_div_mod_u32(x: rug::Integer, u: u32) -> (rug::Integer, u32) {
    let (quotient, remainder) = x.div_rem_floor(rug::Integer::from(u));
    (quotient, remainder.to_u32_wrapping())
}

pub fn num_div_rem_u32(x: BigUint, u: u32) -> (BigUint, u32) {
    let (quotient, remainder) = x.div_rem(&BigUint::from(u));
    (quotient, remainder.to_u32().unwrap())
}

pub fn rug_div_rem_u32(x: rug::Integer, u: u32) -> (rug::Integer, u32) {
    let (quotient, remainder) = x.div_rem(rug::Integer::from(u));
    (quotient, remainder.to_u32_wrapping())
}

pub fn rug_ceiling_div_neg_mod_u32(x: rug::Integer, u: u32) -> (rug::Integer, u32) {
    let (quotient, remainder) = x.div_rem_ceil(rug::Integer::from(u));
    (quotient, (-remainder).to_u32_wrapping())
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

fn demo_natural_div_assign_mod_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_mod(u);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_natural_div_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", n_old, u, n.div_mod(u));
    }
}

fn demo_natural_div_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).div_mod({}) = {:?}", n, u, (&n).div_mod(u));
    }
}

fn demo_natural_div_assign_rem_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_rem(u);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_natural_div_rem_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", n_old, u, n.div_rem(u));
    }
}

fn demo_natural_div_rem_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).div_rem({}) = {:?}", n, u, (&n).div_rem(u));
    }
}

fn demo_natural_ceiling_div_assign_neg_mod_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.ceiling_div_assign_neg_mod(u);
        println!(
            "x := {}; x.ceiling_div_assign_neg_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_natural_ceiling_div_neg_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_neg_mod({}) = {:?}",
            n_old,
            u,
            n.ceiling_div_neg_mod(u)
        );
    }
}

fn demo_natural_ceiling_div_neg_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!(
            "(&{}).ceiling_div_neg_mod({}) = {:?}",
            n,
            u,
            (&n).ceiling_div_neg_mod(u)
        );
    }
}

fn demo_limb_div_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", u, n_old, u.div_mod(n));
    }
}

fn demo_limb_div_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod(&{}) = {:?}", u, n_old, u.div_mod(&n));
    }
}

fn demo_limb_div_assign_mod_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        let remainder = u.div_assign_mod(n);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            u_old, n_old, remainder, u
        );
    }
}

fn demo_limb_div_assign_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let remainder = u.div_assign_mod(&n);
        println!(
            "x := {}; x.div_assign_mod(&{}) = {}; x = {}",
            u_old, n, remainder, u
        );
    }
}

fn demo_limb_div_rem_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", u, n_old, u.div_rem(n));
    }
}

fn demo_limb_div_rem_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem(&{}) = {:?}", u, n_old, u.div_rem(&n));
    }
}

fn demo_limb_div_assign_rem_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        let remainder = u.div_assign_rem(n);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            u_old, n_old, remainder, u
        );
    }
}

fn demo_limb_div_assign_rem_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let remainder = u.div_assign_rem(&n);
        println!(
            "x := {}; x.div_assign_rem(&{}) = {}; x = {}",
            u_old, n, remainder, u
        );
    }
}

fn demo_limb_ceiling_div_neg_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_neg_mod({}) = {:?}",
            u,
            n_old,
            u.ceiling_div_neg_mod(n)
        );
    }
}

fn demo_limb_ceiling_div_neg_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_neg_mod(&{}) = {:?}",
            u,
            n_old,
            u.ceiling_div_neg_mod(&n)
        );
    }
}

fn demo_limb_ceiling_div_assign_neg_mod_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        let remainder = u.ceiling_div_assign_neg_mod(n);
        println!(
            "x := {}; x.ceiling_div_assign_neg_mod({}) = {:?}; x = {}",
            u_old, n_old, remainder, u
        );
    }
}

fn demo_limb_ceiling_div_assign_neg_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let remainder = u.ceiling_div_assign_neg_mod(&n);
        println!(
            "x := {}; x.ceiling_div_assign_neg_mod(&{}) = {:?}; x = {}",
            u_old, n, remainder, u
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
        &(|limb| usize::checked_from(limb.significant_bits()).unwrap()),
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
                    no_out!(_limbs_div_limb_to_out_mod_naive(&mut out, &in_limbs, limb))
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
                    no_out!(_limbs_div_limb_in_place_mod_naive(&mut limbs, limb))
                }),
            ),
        ],
    );
}

fn benchmark_natural_div_assign_mod_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_assign_mod(Limb)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
        )],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_div_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_mod(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_mod(y)))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_div_mod_u32(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_mod_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_div_mod_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_mod(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.div_mod(y)))),
            ("using / and %", &mut (|(x, y)| no_out!((&x / y, x % y)))),
        ],
    );
}

fn benchmark_natural_div_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_mod(Limb)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "(&Natural).div_mod(Limb)",
                &mut (|(x, y)| no_out!((&x).div_mod(y))),
            ),
        ],
    );
}

fn benchmark_natural_div_assign_rem_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_assign_rem(Limb)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
        )],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_div_rem_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_rem(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_rem(y)))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_div_rem_u32(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_rem_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_div_rem_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_rem(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_rem(Limb)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "(&Natural).div_rem(Limb)",
                &mut (|(x, y)| no_out!((&x).div_rem(y))),
            ),
        ],
    );
}

fn benchmark_natural_ceiling_div_assign_neg_mod_limb(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_assign_neg_mod(Limb)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(y))),
        )],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_ceiling_div_neg_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_neg_mod(Limb)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_unsigned(gm),
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
                &mut (|((x, y), _)| no_out!(rug_ceiling_div_neg_mod_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_ceiling_div_neg_mod_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_neg_mod(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
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
}

fn benchmark_natural_ceiling_div_neg_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_neg_mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.ceiling_div_neg_mod(Limb)",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "(&Natural).ceiling_div_neg_mod(Limb)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_neg_mod(y))),
            ),
        ],
    );
}

fn benchmark_limb_div_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_mod(Natural)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "Limb.div_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.div_mod(&y))),
            ),
        ],
    );
}

fn benchmark_limb_div_assign_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_assign_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_assign_mod(Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
            ),
            (
                "Limb.div_assign_mod(&Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_mod(&y))),
            ),
        ],
    );
}

fn benchmark_limb_div_rem_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_rem(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_rem(Natural)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "Limb.div_rem(&Natural)",
                &mut (|(x, y)| no_out!(x.div_rem(&y))),
            ),
        ],
    );
}

fn benchmark_limb_div_assign_rem_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_assign_rem(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_assign_rem(Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
            ),
            (
                "Limb.div_assign_rem(&Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_rem(&y))),
            ),
        ],
    );
}

fn benchmark_limb_ceiling_div_neg_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.ceiling_div_neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.ceiling_div_neg_mod(Natural)",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "Limb.ceiling_div_neg_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(&y))),
            ),
        ],
    );
}

fn benchmark_limb_ceiling_div_assign_neg_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.ceiling_div_assign_neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.ceiling_div_assign_neg_mod(Natural)",
                &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(y))),
            ),
            (
                "Limb.ceiling_div_assign_neg_mod(&Natural)",
                &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(&y))),
            ),
        ],
    );
}
