use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    pairs_of_integer_and_small_i32, rm_pairs_of_integer_and_small_i32,
    triples_of_integer_small_i32_and_rounding_mode_var_1,
};
use malachite_base::num::{ShlRound, ShlRoundAssign};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_shl_assign_i32);
    register_demo!(registry, demo_integer_shl_i32);
    register_demo!(registry, demo_integer_shl_i32_ref);
    register_demo!(registry, demo_integer_shl_round_assign_i32);
    register_demo!(registry, demo_integer_shl_round_i32);
    register_demo!(registry, demo_integer_shl_round_i32_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_assign_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_i32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_shl_round_assign_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_shl_round_i32_evaluation_strategy
    );
}

fn demo_integer_shl_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_small_i32(gm).take(limit) {
        let n_old = n.clone();
        n <<= i;
        println!("x := {}; x <<= {}; x = {}", n_old, i, n);
    }
}

fn demo_integer_shl_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_small_i32(gm).take(limit) {
        let n_old = n.clone();
        println!("{} << {} = {}", n_old, i, n << i);
    }
}

fn demo_integer_shl_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_small_i32(gm).take(limit) {
        println!("&{} << {} = {}", n, i, &n << i);
    }
}

fn demo_integer_shl_round_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i, rm) in triples_of_integer_small_i32_and_rounding_mode_var_1(gm).take(limit) {
        let n_old = n.clone();
        n.shl_round_assign(i, rm);
        println!(
            "x := {}; x.shl_round_assign({}, {}); x = {}",
            n_old, i, rm, n
        );
    }
}

fn demo_integer_shl_round_i32(gm: GenerationMode, limit: usize) {
    for (n, i, rm) in triples_of_integer_small_i32_and_rounding_mode_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.shl_round({}, {}) = {}",
            n_old,
            i,
            rm,
            n.shl_round(i, rm)
        );
    }
}

fn demo_integer_shl_round_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i, rm) in triples_of_integer_small_i32_and_rounding_mode_var_1(gm).take(limit) {
        println!(
            "(&{}).shl_round({}, {}) = {}",
            n,
            i,
            rm,
            (&n).shl_round(i, rm)
        );
    }
}

fn benchmark_integer_shl_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer <<= i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_i32(gm),
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

fn benchmark_integer_shl_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer << i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_i32(gm),
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

fn benchmark_integer_shl_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer << i32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_i32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, other)| other as usize),
        "other",
        &mut [
            ("Integer << i32", &mut (|(x, y)| no_out!(x << y))),
            ("&Integer << i32", &mut (|(x, y)| no_out!(&x << y))),
        ],
    );
}

fn benchmark_integer_shl_round_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.shl_round_assign(i32, RoundingMode)",
        BenchmarkType::Single,
        triples_of_integer_small_i32_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, other, _)| other as usize),
        "other",
        &mut [(
            "malachite",
            &mut (|(mut x, y, rm)| x.shl_round_assign(y, rm)),
        )],
    );
}

fn benchmark_integer_shl_round_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.shl_round(i32, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_small_i32_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, other, _)| other as usize),
        "other",
        &mut [
            (
                "Integer.shl_round(i32, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.shl_round(y, rm))),
            ),
            (
                "(&Integer).shl_round(i32, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).shl_round(y, rm))),
            ),
        ],
    );
}
