use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::sub_limb::{
    limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_unsigned, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
};
#[cfg(not(feature = "32_bit_limbs"))]
use inputs::natural::nm_pairs_of_natural_and_limb_var_1;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::{
    nrm_pairs_of_natural_and_limb_var_1, rm_pairs_of_limb_and_natural_var_1,
    rm_pairs_of_natural_and_limb_var_1,
};
use inputs::natural::{pairs_of_limb_and_natural_var_1, pairs_of_natural_and_limb_var_1};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_sub_limb);
    register_demo!(registry, demo_limbs_sub_limb_to_out);
    register_demo!(registry, demo_limbs_sub_limb_in_place);
    register_demo!(registry, demo_natural_sub_assign_limb);
    register_demo!(registry, demo_natural_sub_limb);
    register_demo!(registry, demo_natural_sub_limb_ref);
    register_demo!(registry, demo_limb_sub_assign_natural);
    register_demo!(registry, demo_limb_sub_assign_natural_ref);
    register_demo!(registry, demo_limb_sub_natural);
    register_demo!(registry, demo_limb_sub_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_sub_limb);
    register_bench!(registry, Small, benchmark_limbs_sub_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_sub_limb_in_place);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_assign_limb_library_comparison
    );
    #[cfg(not(feature = "32_bit_limbs"))]
    register_bench!(registry, Large, benchmark_natural_sub_assign_limb);
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_sub_assign_natural_evaluation_strategy
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_sub_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_sub_natural_evaluation_strategy
    );
}

fn demo_limbs_sub_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_sub_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_sub_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_sub_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let borrow = limbs_sub_limb_to_out(&mut out, &in_limbs, limb);
        println!(
            "out := {:?}; limbs_sub_limb_to_out(&mut out, {:?}, {}) = {}; \
             out = {:?}",
            out_old, in_limbs, limb, borrow, out
        );
    }
}

fn demo_limbs_sub_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let borrow = limbs_sub_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_sub_limb_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, borrow, limbs
        );
    }
}

fn demo_natural_sub_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_limb_var_1(gm).take(limit) {
        let n_old = n.clone();
        n -= u;
        println!("x := {}; x -= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_sub_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_limb_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", n_old, u, n - u);
    }
}

fn demo_natural_sub_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_limb_var_1(gm).take(limit) {
        println!("&{} - {} = {}", n, u, &n - u);
    }
}

fn demo_limb_sub_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_limb_and_natural_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", u, n_old, u - n);
    }
}

fn demo_limb_sub_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_limb_and_natural_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - &{} = {}", u, n_old, u - &n);
    }
}

fn demo_limb_sub_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_limb_and_natural_var_1(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u -= n;
        println!("x := {}; x -= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_limb_sub_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_limb_and_natural_var_1(gm).take(limit) {
        let u_old = u;
        u -= &n;
        println!("x := {}; x -= &{}; x = {}", u_old, n, u);
    }
}

fn benchmark_limbs_sub_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_sub_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_sub_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_limb_to_out(&mut [Limb], &[Limb], Limb)",
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
                no_out!(limbs_sub_limb_to_out(&mut out, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_sub_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| no_out!(limbs_sub_limb_in_place(&mut limbs, limb))),
        )],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_sub_assign_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural -= Limb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
fn benchmark_natural_sub_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural -= Limb",
        BenchmarkType::Single,
        pairs_of_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x -= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_sub_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural - Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x - y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
fn benchmark_natural_sub_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural - Limb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_natural_sub_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural - Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural - Limb", &mut (|(x, y)| no_out!(x - y))),
            ("&Natural - Limb", &mut (|(x, y)| no_out!(&x - y))),
        ],
    );
}

fn benchmark_limb_sub_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb -= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_limb_and_natural_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb -= Natural", &mut (|(mut x, y)| x -= y)),
            ("Limb -= &Natural", &mut (|(mut x, y)| x -= &y)),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_sub_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb - Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_limb_and_natural_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x - y))),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_limb_sub_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb - Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_limb_and_natural_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb - Natural", &mut (|(x, y)| no_out!(x - y))),
            ("Limb - &Natural", &mut (|(x, y)| no_out!(x - &y))),
        ],
    );
}
