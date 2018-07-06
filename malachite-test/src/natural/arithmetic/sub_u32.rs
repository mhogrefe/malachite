use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_unsigned, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_u32_var_1, pairs_of_natural_and_u32_var_1,
    pairs_of_u32_and_natural_var_1, rm_pairs_of_natural_and_u32_var_1,
    rm_pairs_of_u32_and_natural_var_1,
};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::arithmetic::sub_u32::{
    limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_sub_limb);
    register_demo!(registry, demo_limbs_sub_limb_to_out);
    register_demo!(registry, demo_limbs_sub_limb_in_place);
    register_demo!(registry, demo_natural_sub_assign_u32);
    register_demo!(registry, demo_natural_sub_u32);
    register_demo!(registry, demo_natural_sub_u32_ref);
    register_demo!(registry, demo_u32_sub_natural);
    register_bench!(registry, Small, benchmark_limbs_sub_limb);
    register_bench!(registry, Small, benchmark_limbs_sub_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_sub_limb_in_place);
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_sub_natural_library_comparison
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
    for (out_limbs, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        let borrow = limbs_sub_limb_to_out(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_sub_limb_to_out(&mut out_limbs, {:?}, {}) = {}; out_limbs = {:?}",
            out_limbs_old, in_limbs, limb, borrow, out_limbs
        );
    }
}

fn demo_limbs_sub_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        let borrow = limbs_sub_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_sub_limb_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, limb, borrow, limbs
        );
    }
}

fn demo_natural_sub_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_u32_var_1(gm).take(limit) {
        let n_old = n.clone();
        n -= u;
        println!("x := {}; x -= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_sub_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_u32_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", n_old, u, n - u);
    }
}

fn demo_natural_sub_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_u32_var_1(gm).take(limit) {
        println!("&{} - {} = {}", n, u, &n - u);
    }
}

fn demo_u32_sub_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_u32_and_natural_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", u, n_old, u - &n);
    }
}

fn benchmark_limbs_sub_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_limb(&[u32], u32)",
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
        "limbs_sub_limb_to_out(&mut [u32], &[u32], u32)",
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
                no_out!(limbs_sub_limb_to_out(&mut out_limbs, &in_limbs, limb))
            }),
        )],
    );
}

fn benchmark_limbs_sub_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_limb_in_place(&mut [u32], u32)",
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

fn benchmark_natural_sub_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural -= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

fn benchmark_natural_sub_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural - u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x - y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_natural_sub_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural - u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural - u32", &mut (|(x, y)| no_out!(x - y))),
            ("&Natural - u32", &mut (|(x, y)| no_out!(&x - y))),
        ],
    );
}

fn benchmark_u32_sub_natural_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 - Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_u32_and_natural_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x - &y))),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}
