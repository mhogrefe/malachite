use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned,
    pairs_of_unsigned_and_natural, rm_pairs_of_natural_and_unsigned,
    rm_pairs_of_unsigned_and_natural,
};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::logic::or_u32::{
    limbs_or_limb, limbs_or_limb_in_place, limbs_or_limb_to_out,
};
use malachite_nz::natural::Natural;
use natural::logic::or::{natural_or_alt_1, natural_or_alt_2};
use num::BigUint;

pub fn natural_or_u32_alt_1(n: &Natural, i: u32) -> Natural {
    natural_or_alt_1(n, &Natural::from(i))
}

pub fn natural_or_u32_alt_2(n: &Natural, i: u32) -> Natural {
    natural_or_alt_2(n, &Natural::from(i))
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_or_limb);
    register_demo!(registry, demo_limbs_or_limb_to_out);
    register_demo!(registry, demo_limbs_or_limb_in_place);
    register_demo!(registry, demo_natural_or_assign_u32);
    register_demo!(registry, demo_natural_or_u32);
    register_demo!(registry, demo_natural_or_u32_ref);
    register_demo!(registry, demo_u32_or_natural);
    register_demo!(registry, demo_u32_or_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_or_limb);
    register_bench!(registry, Small, benchmark_limbs_or_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_or_limb_in_place);
    register_bench!(
        registry,
        Large,
        benchmark_natural_or_assign_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_or_u32_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_natural_or_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_or_u32_algorithms);
    register_bench!(registry, Large, benchmark_u32_or_natural_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_u32_or_natural_evaluation_strategy
    );
}

pub fn num_or_u32(x: BigUint, u: u32) -> BigUint {
    x | BigUint::from(u)
}

fn demo_limbs_or_limb(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_or_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_or_limb(limbs, limb)
        );
    }
}

fn demo_limbs_or_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out_limbs, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        limbs_or_limb_to_out(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_or_limb_to_out(&mut out_limbs, {:?}, {}); out_limbs = {:?}",
            out_limbs_old, in_limbs, limb, out_limbs
        );
    }
}

fn demo_limbs_or_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_or_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_or_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_natural_or_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n |= u;
        println!("x := {}; x |= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_or_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} | {} = {}", n_old, u, n | u);
    }
}

fn demo_natural_or_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} | {} = {}", n, u, &n | u);
    }
}

fn demo_u32_or_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} | {} = {}", u, n_old, u | n);
    }
}

fn demo_u32_or_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        println!("{} | &{} = {}", u, n, u | &n);
    }
}

fn benchmark_limbs_or_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_or_limb(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_or_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_or_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_or_limb_to_out(&mut [u32], &[u32], u32)",
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
                no_out!(limbs_or_limb_to_out(&mut out_limbs, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_or_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_or_limb_in_place(&mut [u32], u32)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| no_out!(limbs_or_limb_in_place(&mut limbs, limb))),
        )],
    );
}

fn benchmark_natural_or_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural |= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x |= y)),
            ("rug", &mut (|((mut x, y), _)| x |= y)),
        ],
    );
}

fn benchmark_natural_or_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural | u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(&x | y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_or_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x | y))),
        ],
    );
}

fn benchmark_natural_or_u32_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural | u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural | u32", &mut (|(x, y)| no_out!(x | y))),
            ("&Natural | u32", &mut (|(x, y)| no_out!(&x | y))),
        ],
    );
}

fn benchmark_natural_or_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural | u32",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(&x | y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(natural_or_u32_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(natural_or_u32_alt_2(&x, y))),
            ),
        ],
    );
}

fn benchmark_u32_or_natural_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 | Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x | &y))),
            ("rug", &mut (|((x, y), _)| no_out!(x | y))),
        ],
    );
}

fn benchmark_u32_or_natural_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 | Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 | Natural", &mut (|(x, y)| no_out!(x | y))),
            ("u32 | &Natural", &mut (|(x, y)| no_out!(x | &y))),
        ],
    );
}
