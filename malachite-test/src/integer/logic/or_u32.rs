use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_u32_vec_and_u32_var_1, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
};
use inputs::integer::{
    pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer, rm_pairs_of_integer_and_unsigned,
    rm_pairs_of_unsigned_and_integer,
};
use integer::logic::or::{integer_or_alt_1, integer_or_alt_2};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::logic::or_u32::{
    limbs_neg_or_limb, limbs_neg_or_limb_in_place, limbs_neg_or_limb_to_out,
};
use malachite_nz::integer::Integer;

pub fn integer_or_u32_alt_1(n: &Integer, i: u32) -> Integer {
    integer_or_alt_1(n, &Integer::from(i))
}

pub fn integer_or_u32_alt_2(n: &Integer, i: u32) -> Integer {
    integer_or_alt_2(n, &Integer::from(i))
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_neg_or_limb);
    register_demo!(registry, demo_limbs_neg_or_limb_to_out);
    register_demo!(registry, demo_limbs_neg_or_limb_in_place);
    register_demo!(registry, demo_integer_or_assign_u32);
    register_demo!(registry, demo_integer_or_u32);
    register_demo!(registry, demo_integer_or_u32_ref);
    register_demo!(registry, demo_u32_or_integer);
    register_demo!(registry, demo_u32_or_integer_ref);
    register_bench!(registry, Small, benchmark_limbs_neg_or_limb);
    register_bench!(registry, Small, benchmark_limbs_neg_or_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_neg_or_limb_in_place);
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_assign_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_or_u32_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_or_u32_algorithms);
    register_bench!(registry, Large, benchmark_u32_or_integer_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_u32_or_integer_evaluation_strategy
    );
}

fn demo_limbs_neg_or_limb(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_u32_vec_and_u32_var_1(gm).take(limit) {
        println!(
            "limbs_neg_or_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_neg_or_limb(limbs, limb)
        );
    }
}

fn demo_limbs_neg_or_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out_limbs, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        limbs_neg_or_limb_to_out(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_neg_or_limb_to_out(&mut out_limbs, {:?}, {}); out_limbs = {:?}",
            out_limbs_old, in_limbs, limb, out_limbs
        );
    }
}

fn demo_limbs_neg_or_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_u32_vec_and_u32_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_neg_or_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_neg_or_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_integer_or_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n |= u;
        println!("x := {}; x |= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_or_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} | {} = {}", n_old, u, n | u);
    }
}

fn demo_integer_or_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} | {} = {}", n, u, &n | u);
    }
}

fn demo_u32_or_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} | {} = {}", u, n_old, u | n);
    }
}

fn demo_u32_or_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        println!("{} | &{} = {}", u, n, u | &n);
    }
}

fn benchmark_limbs_neg_or_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_or_limb(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_neg_or_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_neg_or_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_or_limb_to_out(&mut [u32], &[u32], u32)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out_limbs, in_limbs, limb)| {
                limbs_neg_or_limb_to_out(&mut out_limbs, &in_limbs, limb)
            }),
        )],
    );
}

fn benchmark_limbs_neg_or_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_or_limb_in_place(&mut [u32], u32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| limbs_neg_or_limb_in_place(&mut limbs, limb)),
        )],
    );
}

fn benchmark_integer_or_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer |= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<u32>(gm),
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

fn benchmark_integer_or_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer | u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(&x | y))),
            ("rug", &mut (|((x, y), _)| no_out!(x | y))),
        ],
    );
}

fn benchmark_integer_or_u32_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer | u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer | u32", &mut (|(x, y)| no_out!(x | y))),
            ("&Integer | u32", &mut (|(x, y)| no_out!(&x | y))),
        ],
    );
}

fn benchmark_integer_or_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer | u32",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(&x | y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(integer_or_u32_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(integer_or_u32_alt_2(&x, y))),
            ),
        ],
    );
}

fn benchmark_u32_or_integer_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 | Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_integer::<u32>(gm),
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

fn benchmark_u32_or_integer_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 | Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 | Integer", &mut (|(x, y)| no_out!(x | y))),
            ("u32 | &Integer", &mut (|(x, y)| no_out!(x | &y))),
        ],
    );
}
