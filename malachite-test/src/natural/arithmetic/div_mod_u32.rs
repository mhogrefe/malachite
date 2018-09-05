use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_unsigned, pairs_of_natural_and_positive_unsigned,
    pairs_of_unsigned_and_positive_natural,
};
use malachite_base::num::{DivAssignMod, DivAssignRem, DivMod, DivRem, SignificantBits};
use malachite_nz::natural::arithmetic::div_mod_u32::{
    limbs_div_limb_in_place_mod, limbs_div_limb_mod, limbs_div_limb_to_out_mod,
};
use num::{BigUint, Integer, ToPrimitive};
use rug;

// For `Natural`s, `div` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_div_limb_mod);
    register_demo!(registry, demo_limbs_div_limb_to_out_mod);
    register_demo!(registry, demo_limbs_div_limb_in_place_mod);
    register_demo!(registry, demo_natural_div_assign_mod_u32);
    register_demo!(registry, demo_natural_div_mod_u32);
    register_demo!(registry, demo_natural_div_mod_u32_ref);
    register_demo!(registry, demo_natural_div_assign_rem_u32);
    register_demo!(registry, demo_natural_div_rem_u32);
    register_demo!(registry, demo_natural_div_rem_u32_ref);
    register_demo!(registry, demo_u32_div_mod_natural);
    register_demo!(registry, demo_u32_div_mod_natural_ref);
    register_demo!(registry, demo_u32_div_assign_mod_natural);
    register_demo!(registry, demo_u32_div_assign_mod_natural_ref);
    register_demo!(registry, demo_u32_div_rem_natural);
    register_demo!(registry, demo_u32_div_rem_natural_ref);
    register_demo!(registry, demo_u32_div_assign_rem_natural);
    register_demo!(registry, demo_u32_div_assign_rem_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_div_limb_mod);
    register_bench!(registry, Small, benchmark_limbs_div_limb_to_out_mod);
    register_bench!(registry, Small, benchmark_limbs_div_limb_in_place_mod);
    register_bench!(registry, Large, benchmark_natural_div_assign_mod_u32);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_mod_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_div_mod_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_mod_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_div_assign_rem_u32);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_rem_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_rem_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_mod_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_assign_mod_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_rem_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_assign_rem_natural_evaluation_strategy
    );
}

pub fn num_div_mod_u32(x: BigUint, u: u32) -> (BigUint, u32) {
    let (quotient, remainder) = x.div_mod_floor(&BigUint::from(u));
    (quotient, remainder.to_u32().unwrap())
}

pub fn rug_div_mod_u32(x: rug::Integer, u: u32) -> (rug::Integer, u32) {
    let (quotient, remainder) = x.div_rem(rug::Integer::from(u));
    (quotient, remainder.to_u32_wrapping())
}

pub fn num_div_rem_u32(x: BigUint, u: u32) -> (BigUint, u32) {
    let (quotient, remainder) = x.div_rem(&BigUint::from(u));
    (quotient, remainder.to_u32().unwrap())
}

pub fn rug_div_rem_u32(x: rug::Integer, u: u32) -> (rug::Integer, u32) {
    let (quotient, remainder) = x.div_rem_floor(rug::Integer::from(u));
    (quotient, remainder.to_u32_wrapping())
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
    for (out_limbs, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        let remainder = limbs_div_limb_to_out_mod(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_div_limb_to_out_mod(&mut out_limbs, {:?}, {}) = {}; out_limbs = {:?}",
            out_limbs_old, in_limbs, limb, remainder, out_limbs
        );
    }
}

fn demo_limbs_div_limb_in_place_mod(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        let remainder = limbs_div_limb_in_place_mod(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_div_limb_in_place_mod(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, remainder, limbs
        );
    }
}

fn demo_natural_div_assign_mod_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_mod(u);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_natural_div_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", n_old, u, n.div_mod(u));
    }
}

fn demo_natural_div_mod_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).div_mod({}) = {:?}", n, u, (&n).div_mod(u));
    }
}

fn demo_natural_div_assign_rem_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_rem(u);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_natural_div_rem_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", n_old, u, n.div_rem(u));
    }
}

fn demo_natural_div_rem_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).div_rem({}) = {:?}", n, u, (&n).div_rem(u));
    }
}

fn demo_u32_div_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", u, n_old, u.div_mod(n));
    }
}

fn demo_u32_div_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod(&{}) = {:?}", u, n_old, u.div_mod(&n));
    }
}

fn demo_u32_div_assign_mod_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        let remainder = u.div_assign_mod(n);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            u_old, n_old, remainder, u
        );
    }
}

fn demo_u32_div_assign_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        let remainder = u.div_assign_mod(&n);
        println!(
            "x := {}; x.div_assign_mod(&{}) = {}; x = {}",
            u_old, n, remainder, u
        );
    }
}

fn demo_u32_div_rem_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", u, n_old, u.div_rem(n));
    }
}

fn demo_u32_div_rem_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem(&{}) = {:?}", u, n_old, u.div_rem(&n));
    }
}

fn demo_u32_div_assign_rem_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        let remainder = u.div_assign_rem(n);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            u_old, n_old, remainder, u
        );
    }
}

fn demo_u32_div_assign_rem_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        let remainder = u.div_assign_rem(&n);
        println!(
            "x := {}; x.div_assign_rem(&{}) = {}; x = {}",
            u_old, n, remainder, u
        );
    }
}

fn benchmark_limbs_div_limb_mod(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb_mod(&[u32], u32)",
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

fn benchmark_limbs_div_limb_to_out_mod(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb_to_out_mod(&mut [u32], &[u32], u32)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out_limbs, in_limbs, limb)| {
                no_out!(limbs_div_limb_to_out_mod(&mut out_limbs, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_div_limb_in_place_mod(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb_in_place_mod(&mut [u32], u32)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| no_out!(limbs_div_limb_in_place_mod(&mut limbs, limb))),
        )],
    );
}

fn benchmark_natural_div_assign_mod_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_assign_mod(u32)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
        )],
    );
}

fn benchmark_natural_div_mod_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_mod(u32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
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

fn benchmark_natural_div_mod_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_mod(u32)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.div_mod(y)))),
            ("naive", &mut (|(x, y)| no_out!(x._div_mod_naive(y)))),
        ],
    );
}

fn benchmark_natural_div_mod_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_mod(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_mod(u32)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "(&Natural).div_mod(u32)",
                &mut (|(x, y)| no_out!((&x).div_mod(y))),
            ),
        ],
    );
}

fn benchmark_natural_div_assign_rem_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_assign_rem(u32)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
        )],
    );
}

fn benchmark_natural_div_rem_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_rem(u32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
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

fn benchmark_natural_div_rem_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_rem(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_rem(u32)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "(&Natural).div_rem(u32)",
                &mut (|(x, y)| no_out!((&x).div_rem(y))),
            ),
        ],
    );
}

fn benchmark_u32_div_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_mod(Natural)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "u32.div_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.div_mod(&y))),
            ),
        ],
    );
}

fn benchmark_u32_div_assign_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_assign_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_assign_mod(Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
            ),
            (
                "u32.div_assign_mod(&Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_mod(&y))),
            ),
        ],
    );
}

fn benchmark_u32_div_rem_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_rem(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_rem(Natural)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "u32.div_rem(&Natural)",
                &mut (|(x, y)| no_out!(x.div_rem(&y))),
            ),
        ],
    );
}

fn benchmark_u32_div_assign_rem_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_assign_rem(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_assign_rem(Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
            ),
            (
                "u32.div_assign_rem(&Natural)",
                &mut (|(mut x, y)| no_out!(x.div_assign_rem(&y))),
            ),
        ],
    );
}
