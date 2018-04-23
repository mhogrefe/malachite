use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_small_u32, rm_pairs_of_integer_and_small_u32,
                      triples_of_integer_small_u32_and_rounding_mode_var_1};
use malachite_base::num::{ShrRound, ShrRoundAssign};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_shr_assign_u32);
    register_demo!(registry, demo_integer_shr_u32);
    register_demo!(registry, demo_integer_shr_u32_ref);
    register_demo!(registry, demo_integer_shr_round_assign_u32);
    register_demo!(registry, demo_integer_shr_round_u32);
    register_demo!(registry, demo_integer_shr_round_u32_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u32_evaluation_strategy
    );
}

fn demo_integer_shr_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n >>= u;
        println!("x := {}; x >>= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_shr_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

fn demo_integer_shr_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

fn demo_integer_shr_round_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u, rm) in triples_of_integer_small_u32_and_rounding_mode_var_1(gm).take(limit) {
        let n_old = n.clone();
        n.shr_round_assign(u, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {}); x = {}",
            n_old, u, rm, n
        );
    }
}

fn demo_integer_shr_round_u32(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in triples_of_integer_small_u32_and_rounding_mode_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {}) = {}",
            n_old,
            u,
            rm,
            n.shr_round(u, rm)
        );
    }
}

fn demo_integer_shr_round_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in triples_of_integer_small_u32_and_rounding_mode_var_1(gm).take(limit) {
        println!(
            "(&{}).shr_round({}, {}) = {}",
            n,
            u,
            rm,
            (&n).shr_round(u, rm)
        );
    }
}

fn benchmark_integer_shr_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer >>= u32",
        BenchmarkType::Single,
        rm_pairs_of_integer_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| other as usize),
        "other",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x >>= y)),
            ("rug", &mut (|((mut x, y), _)| x >>= y)),
        ],
    );
}

fn benchmark_integer_shr_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer >> u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| other as usize),
        "other",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x >> y))),
            ("rug", &mut (|((x, y), _)| no_out!(x >> y))),
        ],
    );
}

fn benchmark_integer_shr_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer >> u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, other)| other as usize),
        "other",
        &mut [
            ("Integer >> u32", &mut (|(x, y)| no_out!(x >> y))),
            ("&Integer >> u32", &mut (|(x, y)| no_out!(&x >> y))),
        ],
    );
}

fn benchmark_integer_shr_round_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.shr_round_assign(u32, RoundingMode)",
        BenchmarkType::Single,
        triples_of_integer_small_u32_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, other, _)| other as usize),
        "other",
        &mut [(
            "malachite",
            &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
        )],
    );
}

fn benchmark_integer_shr_round_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.shr_round(u32, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_small_u32_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, other, _)| other as usize),
        "other",
        &mut [
            (
                "Integer.shr_round(u32, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))),
            ),
            (
                "(&Integer).shr_round(u32, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).shr_round(y, rm))),
            ),
        ],
    );
}
