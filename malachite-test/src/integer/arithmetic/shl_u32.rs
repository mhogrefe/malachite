use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_small_u32, rm_pairs_of_integer_and_small_u32};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_shl_assign_u32);
    register_demo!(registry, demo_integer_shl_u32);
    register_demo!(registry, demo_integer_shl_u32_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_u32_evaluation_strategy
    );
}

fn demo_integer_shl_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n <<= u;
        println!("x := {}; x <<= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_shl_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!("{} << {} = {}", n_old, u, n << u);
    }
}

fn demo_integer_shl_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        println!("&{} << {} = {}", n, u, &n << u);
    }
}

fn benchmark_integer_shl_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer <<= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| other as usize),
        "other",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x <<= y)),
            ("rug", &mut (|((mut x, y), _)| x <<= y)),
        ],
    );
}

fn benchmark_integer_shl_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer << u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| other as usize),
        "other",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x << y))),
            ("rug", &mut (|((x, y), _)| no_out!(x << y))),
        ],
    );
}

fn benchmark_integer_shl_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer << u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, other)| other as usize),
        "other",
        &mut [
            ("Integer << u32", &mut (|(x, y)| no_out!(x << y))),
            ("&Integer << u32", &mut (|(x, y)| no_out!(&x << y))),
        ],
    );
}
