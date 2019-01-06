use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_unsigned_vec_var_10, triples_of_unsigned_vec_var_11, triples_of_unsigned_vec_var_12,
    triples_of_unsigned_vec_var_13, triples_of_unsigned_vec_var_14,
};
use inputs::natural::{nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_to_out_basecase, _limbs_mul_to_out_toom_22, _limbs_mul_to_out_toom_22_scratch_size,
    _limbs_mul_to_out_toom_32, _limbs_mul_to_out_toom_32_scratch_size, _limbs_mul_to_out_toom_33,
    _limbs_mul_to_out_toom_33_scratch_size, _limbs_mul_to_out_toom_42,
    _limbs_mul_to_out_toom_42_scratch_size, mpn_mul,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mul_assign);
    register_demo!(registry, demo_natural_mul_assign_ref);
    register_demo!(registry, demo_natural_mul);
    register_demo!(registry, demo_natural_mul_val_ref);
    register_demo!(registry, demo_natural_mul_ref_val);
    register_demo!(registry, demo_natural_mul_ref_ref);
    register_bench!(registry, Large, benchmark_limbs_mul_to_out_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_to_out_toom_22_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_to_out_toom_32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_to_out_toom_33_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_to_out_toom_42_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_mul_assign_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mul_library_comparison);
    register_bench!(registry, Large, benchmark_natural_mul_evaluation_strategy);
}

fn demo_natural_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {}; x *= {}; x = {}", x_old, y, x);
    }
}

fn demo_natural_mul_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {}; x *= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_mul(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

fn demo_natural_mul_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

fn demo_natural_mul_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

fn demo_natural_mul_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

fn benchmark_limbs_mul_to_out_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mul_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_10(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out_limbs, xs, ys)| {
                    _limbs_mul_to_out_basecase(&mut out_limbs, &xs, &ys)
                }),
            ),
            (
                "full",
                &mut (|(mut out_limbs, xs, ys)| no_out!(mpn_mul(&mut out_limbs, &xs, &ys))),
            ),
        ],
    );
}

fn benchmark_limbs_mul_to_out_toom_22_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_mul_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_11(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out_limbs, xs, ys)| {
                    _limbs_mul_to_out_basecase(&mut out_limbs, &xs, &ys)
                }),
            ),
            (
                "Toom22",
                &mut (|(mut out_limbs, xs, ys)| {
                    let mut scratch = vec![0; _limbs_mul_to_out_toom_22_scratch_size(xs.len())];
                    _limbs_mul_to_out_toom_22(&mut out_limbs, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_to_out_toom_32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_mul_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_12(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out_limbs, xs, ys)| {
                    _limbs_mul_to_out_basecase(&mut out_limbs, &xs, &ys)
                }),
            ),
            (
                "Toom32",
                &mut (|(mut out_limbs, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_to_out_toom_32_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_to_out_toom_32(&mut out_limbs, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_to_out_toom_33_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_mul_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_13(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out_limbs, xs, ys)| {
                    _limbs_mul_to_out_basecase(&mut out_limbs, &xs, &ys)
                }),
            ),
            (
                "Toom33",
                &mut (|(mut out_limbs, xs, ys)| {
                    let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(xs.len())];
                    _limbs_mul_to_out_toom_33(&mut out_limbs, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_to_out_toom_42_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_mul_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_14(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out_limbs, xs, ys)| {
                    _limbs_mul_to_out_basecase(&mut out_limbs, &xs, &ys)
                }),
            ),
            (
                "Toom42",
                &mut (|(mut out_limbs, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_to_out_toom_42_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_to_out_toom_42(&mut out_limbs, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_natural_mul_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural *= Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x *= y)),
            ("rug", &mut (|((mut x, y), _)| x *= y)),
        ],
    );
}

fn benchmark_natural_mul_assign_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural *= Natural",
        BenchmarkType::Algorithms,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("basecase", &mut (|(mut x, y)| no_out!(x *= y))),
            (
                "basecase memory-optimized",
                &mut (|(mut x, y)| no_out!(x._mul_assign_basecase_mem_opt(y))),
            ),
        ],
    );
}

fn benchmark_natural_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural *= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Natural *= Natural", &mut (|(mut x, y)| no_out!(x *= y))),
            ("Natural *= &Natural", &mut (|(mut x, y)| no_out!(x *= &y))),
        ],
    );
}

fn benchmark_natural_mul_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural * Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x * y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x * y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x * y))),
        ],
    );
}

fn benchmark_natural_mul_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural * Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Natural * Natural", &mut (|(x, y)| no_out!(x * y))),
            ("Natural * &Natural", &mut (|(x, y)| no_out!(x * &y))),
            ("&Natural * Natural", &mut (|(x, y)| no_out!(&x * y))),
            ("&Natural * &Natural", &mut (|(x, y)| no_out!(&x * &y))),
        ],
    );
}
