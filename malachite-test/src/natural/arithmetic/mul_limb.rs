use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_unsigned, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
};
#[cfg(feature = "64_bit_limbs")]
use inputs::natural::nm_pairs_of_natural_and_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::{
    nrm_pairs_of_natural_and_unsigned, rm_pairs_of_natural_and_unsigned,
    rm_pairs_of_unsigned_and_natural,
};
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::arithmetic::mul_limb::{
    limbs_mul_limb, limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place,
    limbs_vec_mul_limb_in_place,
};
use malachite_nz::platform::Limb;
use num::BigUint;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mul_limb);
    register_demo!(registry, demo_limbs_mul_limb_to_out);
    register_demo!(registry, demo_limbs_slice_mul_limb_in_place);
    register_demo!(registry, demo_limbs_vec_mul_limb_in_place);
    register_demo!(registry, demo_natural_mul_assign_limb);
    register_demo!(registry, demo_natural_mul_limb);
    register_demo!(registry, demo_natural_mul_limb_ref);
    register_demo!(registry, demo_limb_mul_natural);
    register_demo!(registry, demo_limb_mul_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_mul_limb);
    register_bench!(registry, Small, benchmark_limbs_mul_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_slice_mul_limb_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_mul_limb_in_place);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_limb_library_comparison
    );
    #[cfg(feature = "64_bit_limbs")]
    register_bench!(registry, Large, benchmark_natural_mul_assign_limb);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_limb_evaluation_strategy
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_mul_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_mul_natural_evaluation_strategy
    );
}

fn demo_limbs_mul_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_mul_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_mul_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_mul_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out_limbs, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        let carry = limbs_mul_limb_to_out(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_mul_limb_to_out(&mut out_limbs, {:?}, {}) = {}; out_limbs = \
             {:?}",
            out_limbs_old, in_limbs, limb, carry, out_limbs
        );
    }
}

fn demo_limbs_slice_mul_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        let carry = limbs_slice_mul_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_slice_mul_limb_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, carry, limbs
        );
    }
}

fn demo_limbs_vec_mul_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_vec_mul_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_vec_mul_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

pub fn num_mul_limb(x: BigUint, u: Limb) -> BigUint {
    x * BigUint::from(u)
}

fn demo_natural_mul_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n *= u;
        println!("x := {}; x *= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_mul_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", n_old, u, n * u);
    }
}

fn demo_natural_mul_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        println!("&{} * {} = {}", n, u, &n * u);
    }
}

fn demo_limb_mul_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", u, n_old, u * n);
    }
}

fn demo_limb_mul_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * &{} = {}", u, n_old, u * &n);
    }
}

fn benchmark_limbs_mul_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mul_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_mul_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_mul_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mul_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out_limbs, in_limbs, limb)| {
                no_out!(limbs_mul_limb_to_out(&mut out_limbs, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_slice_mul_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_slice_mul_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| no_out!(limbs_slice_mul_limb_in_place(&mut limbs, limb))),
        )],
    );
}

fn benchmark_limbs_vec_mul_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_mul_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| limbs_vec_mul_limb_in_place(&mut limbs, limb)),
        )],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_mul_assign_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural *= Limb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x *= y)),
            ("rug", &mut (|((mut x, y), _)| x *= y)),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_natural_mul_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural *= Limb",
        BenchmarkType::Single,
        pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x *= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_mul_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural * Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x * y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_mul_limb(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x * y))),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_natural_mul_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural * Limb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x * y))),
            ("num", &mut (|((x, y), _)| no_out!(num_mul_limb(x, y)))),
        ],
    );
}

fn benchmark_natural_mul_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural * Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural * Limb", &mut (|(x, y)| no_out!(x * y))),
            ("&Natural * Limb", &mut (|(x, y)| no_out!(&x * y))),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_mul_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb * Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x * y))),
            ("rug", &mut (|((x, y), _)| no_out!(x * y))),
        ],
    );
}

fn benchmark_limb_mul_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb * Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Limb * Natural", &mut (|(x, y)| no_out!(x * y))),
            ("Limb * &Natural", &mut (|(x, y)| no_out!(x * &y))),
        ],
    );
}
