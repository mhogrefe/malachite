use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_u32_vec_and_negative_signed_var_1;
use inputs::integer::{
    pairs_of_integer_and_signed, pairs_of_signed_and_integer, rm_pairs_of_integer_and_signed,
    rm_pairs_of_signed_and_integer,
};
use integer::logic::or::{integer_or_alt_1, integer_or_alt_2};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::logic::or_i32::{limbs_neg_or_neg_i32, limbs_pos_or_neg_i32};
use malachite_nz::integer::Integer;

pub fn integer_or_i32_alt_1(n: &Integer, i: i32) -> Integer {
    integer_or_alt_1(n, &Integer::from(i))
}

pub fn integer_or_i32_alt_2(n: &Integer, i: i32) -> Integer {
    integer_or_alt_2(n, &Integer::from(i))
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_pos_or_neg_i32);
    register_demo!(registry, demo_limbs_neg_or_neg_i32);
    register_demo!(registry, demo_integer_or_assign_i32);
    register_demo!(registry, demo_integer_or_i32);
    register_demo!(registry, demo_integer_or_i32_ref);
    register_demo!(registry, demo_i32_or_integer);
    register_demo!(registry, demo_i32_or_integer_ref);
    register_bench!(registry, Small, benchmark_limbs_pos_or_neg_i32);
    register_bench!(registry, Small, benchmark_limbs_neg_or_neg_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_assign_i32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_or_i32_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_integer_or_i32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_or_i32_algorithms);
    register_bench!(registry, Large, benchmark_i32_or_integer_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_i32_or_integer_evaluation_strategy
    );
}

fn demo_limbs_pos_or_neg_i32(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_u32_vec_and_negative_signed_var_1(gm).take(limit) {
        println!(
            "limbs_pos_or_neg_i32({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_pos_or_neg_i32(&limbs, limb)
        );
    }
}

fn demo_limbs_neg_or_neg_i32(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_u32_vec_and_negative_signed_var_1(gm).take(limit) {
        println!(
            "limbs_neg_or_neg_i32({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_neg_or_neg_i32(&limbs, limb)
        );
    }
}

fn demo_integer_or_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n |= u;
        println!("x := {}; x |= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_or_i32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} | {} = {}", n_old, u, n | u);
    }
}

fn demo_integer_or_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        println!("&{} | {} = {}", n, u, &n | u);
    }
}

fn demo_i32_or_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} | {} = {}", u, n_old, u | n);
    }
}

fn demo_i32_or_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        println!("{} | &{} = {}", u, n, u | &n);
    }
}

fn benchmark_limbs_pos_or_neg_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_pos_or_neg_i32(&[u32], i32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_negative_signed_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, i)| no_out!(limbs_pos_or_neg_i32(&limbs, i))),
        )],
    );
}

fn benchmark_limbs_neg_or_neg_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_or_neg_i32(&[u32], i32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_negative_signed_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, i)| no_out!(limbs_neg_or_neg_i32(&limbs, i))),
        )],
    );
}

fn benchmark_integer_or_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer |= i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_signed::<i32>(gm),
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

fn benchmark_integer_or_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer | i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_signed::<i32>(gm),
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

fn benchmark_integer_or_i32_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer | i32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer | i32", &mut (|(x, y)| no_out!(x | y))),
            ("&Integer | i32", &mut (|(x, y)| no_out!(&x | y))),
        ],
    );
}

fn benchmark_integer_or_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer | i32",
        BenchmarkType::LibraryComparison,
        pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(&x | y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(integer_or_i32_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(integer_or_i32_alt_2(&x, y))),
            ),
        ],
    );
}

fn benchmark_i32_or_integer_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i32 | Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_integer::<i32>(gm),
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

fn benchmark_i32_or_integer_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i32 | Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("i32 | Integer", &mut (|(x, y)| no_out!(x | y))),
            ("i32 | &Integer", &mut (|(x, y)| no_out!(x | &y))),
        ],
    );
}
