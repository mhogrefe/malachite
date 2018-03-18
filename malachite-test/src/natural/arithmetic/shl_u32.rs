use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::{pairs_of_natural_and_small_u32, rm_pairs_of_natural_and_small_u32};

pub fn demo_natural_shl_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n <<= u;
        println!("x := {}; x <<= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_natural_shl_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!("{} << {} = {}", n_old, u, n << u);
    }
}

pub fn demo_natural_shl_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        println!("&{} << {} = {}", n, u, &n << u);
    }
}

pub fn benchmark_natural_shl_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural <<= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_u32(gm),
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

pub fn benchmark_natural_shl_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural << u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_u32(gm),
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

pub fn benchmark_natural_shl_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural << u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, other)| other as usize),
        "other",
        &mut [
            ("Natural << u32", &mut (|(x, y)| no_out!(x << y))),
            ("&Natural << u32", &mut (|(x, y)| no_out!(&x << y))),
        ],
    );
}
