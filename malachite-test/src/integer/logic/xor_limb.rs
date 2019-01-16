use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_limb_vec_and_limb_var_1, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::{rm_pairs_of_integer_and_unsigned, rm_pairs_of_unsigned_and_integer};
use integer::logic::xor::{integer_xor_alt_1, integer_xor_alt_2};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::logic::xor_limb::{
    limbs_neg_xor_limb, limbs_neg_xor_limb_to_out, limbs_slice_neg_xor_limb_in_place,
    limbs_vec_neg_xor_limb_in_place,
};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

pub fn integer_xor_limb_alt_1(n: &Integer, i: Limb) -> Integer {
    integer_xor_alt_1(n, &Integer::from(i))
}

pub fn integer_xor_limb_alt_2(n: &Integer, i: Limb) -> Integer {
    integer_xor_alt_2(n, &Integer::from(i))
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_neg_xor_limb);
    register_demo!(registry, demo_limbs_neg_xor_limb_to_out);
    register_demo!(registry, demo_limbs_slice_neg_xor_limb_in_place);
    register_demo!(registry, demo_limbs_vec_neg_xor_limb_in_place);
    register_demo!(registry, demo_integer_xor_assign_limb);
    register_demo!(registry, demo_integer_xor_limb);
    register_demo!(registry, demo_integer_xor_limb_ref);
    register_demo!(registry, demo_limb_xor_integer);
    register_demo!(registry, demo_limb_xor_integer_ref);
    register_bench!(registry, Small, benchmark_limbs_neg_xor_limb);
    register_bench!(registry, Small, benchmark_limbs_neg_xor_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_slice_neg_xor_limb_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_neg_xor_limb_in_place);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_xor_assign_limb_library_comparison
    );
    #[cfg(feature = "64_bit_limbs")]
    register_bench!(registry, Large, benchmark_integer_xor_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_xor_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_xor_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_xor_limb_algorithms);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_xor_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_xor_integer_evaluation_strategy
    );
}

fn demo_limbs_neg_xor_limb(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_limb_vec_and_limb_var_1(gm).take(limit) {
        println!(
            "limbs_neg_xor_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_neg_xor_limb(limbs, limb)
        );
    }
}

fn demo_limbs_neg_xor_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out_limbs, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        let carry = limbs_neg_xor_limb_to_out(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_neg_xor_limb_to_out(&mut out_limbs, {:?}, {}) = {}; \
             out_limbs = {:?}",
            out_limbs_old, in_limbs, limb, carry, out_limbs
        );
    }
}

fn demo_limbs_slice_neg_xor_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_limb_vec_and_limb_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        let carry = limbs_slice_neg_xor_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_slice_neg_xor_limb_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, carry, limbs
        );
    }
}

fn demo_limbs_vec_neg_xor_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_limb_vec_and_limb_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_vec_neg_xor_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_vec_neg_xor_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_integer_xor_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n ^= u;
        println!("x := {}; x ^= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_xor_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} ^ {} = {}", n_old, u, n ^ u);
    }
}

fn demo_integer_xor_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        println!("&{} ^ {} = {}", n, u, &n ^ u);
    }
}

fn demo_limb_xor_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} ^ {} = {}", u, n_old, u ^ n);
    }
}

fn demo_limb_xor_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        println!("{} ^ &{} = {}", u, n, u ^ &n);
    }
}

fn benchmark_limbs_neg_xor_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_xor_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_limb_vec_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_neg_xor_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_neg_xor_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_xor_limb_to_out(&mut [Limb], &[Limb], Limb)",
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
                no_out!(limbs_neg_xor_limb_to_out(&mut out_limbs, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_slice_neg_xor_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_slice_xor_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_limb_vec_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| no_out!(limbs_slice_neg_xor_limb_in_place(&mut limbs, limb))),
        )],
    );
}

fn benchmark_limbs_vec_neg_xor_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_vec_xor_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_limb_vec_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| limbs_vec_neg_xor_limb_in_place(&mut limbs, limb)),
        )],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_xor_assign_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer ^= Limb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x ^= y)),
            ("rug", &mut (|((mut x, y), _)| x ^= y)),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_integer_xor_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer ^= Limb",
        BenchmarkType::Single,
        pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x ^= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_xor_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer ^ Limb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(&x ^ y))),
            ("rug", &mut (|((x, y), _)| no_out!(x ^ y))),
        ],
    );
}

fn benchmark_integer_xor_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer ^ Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer ^ Limb", &mut (|(x, y)| no_out!(x ^ y))),
            ("&Integer ^ Limb", &mut (|(x, y)| no_out!(&x ^ y))),
        ],
    );
}

fn benchmark_integer_xor_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer ^ Limb",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(&x ^ y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(integer_xor_limb_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(integer_xor_limb_alt_2(&x, y))),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_xor_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb ^ Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x ^ &y))),
            ("rug", &mut (|((x, y), _)| no_out!(x ^ y))),
        ],
    );
}

fn benchmark_limb_xor_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb ^ Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Limb ^ Integer", &mut (|(x, y)| no_out!(x ^ y))),
            ("Limb ^ &Integer", &mut (|(x, y)| no_out!(x ^ &y))),
        ],
    );
}
