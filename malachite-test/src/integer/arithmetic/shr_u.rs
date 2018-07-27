use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    pairs_of_integer_and_small_unsigned, rm_pairs_of_integer_and_small_unsigned,
    triples_of_integer_small_unsigned_and_rounding_mode_var_1,
};
use malachite_base::misc::Named;
use malachite_base::num::{ShrRound, ShrRoundAssign};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_shr_assign_u8);
    register_demo!(registry, demo_integer_shr_assign_u16);
    register_demo!(registry, demo_integer_shr_assign_u32);
    register_demo!(registry, demo_integer_shr_assign_u64);

    register_demo!(registry, demo_integer_shr_u8);
    register_demo!(registry, demo_integer_shr_u16);
    register_demo!(registry, demo_integer_shr_u32);
    register_demo!(registry, demo_integer_shr_u64);

    register_demo!(registry, demo_integer_shr_u8_ref);
    register_demo!(registry, demo_integer_shr_u16_ref);
    register_demo!(registry, demo_integer_shr_u32_ref);
    register_demo!(registry, demo_integer_shr_u64_ref);

    register_demo!(registry, demo_integer_shr_round_assign_u8);
    register_demo!(registry, demo_integer_shr_round_assign_u16);
    register_demo!(registry, demo_integer_shr_round_assign_u32);
    register_demo!(registry, demo_integer_shr_round_assign_u64);

    register_demo!(registry, demo_integer_shr_round_u8);
    register_demo!(registry, demo_integer_shr_round_u16);
    register_demo!(registry, demo_integer_shr_round_u32);
    register_demo!(registry, demo_integer_shr_round_u64);

    register_demo!(registry, demo_integer_shr_round_u8_ref);
    register_demo!(registry, demo_integer_shr_round_u16_ref);
    register_demo!(registry, demo_integer_shr_round_u32_ref);
    register_demo!(registry, demo_integer_shr_round_u64_ref);

    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u64_evaluation_strategy
    );

    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_integer_shr_round_assign_u64);

    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_round_u64_evaluation_strategy
    );

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
}

macro_rules! demos_and_benches {
    (
        $t:ident,
        $demo_integer_shr_assign_u:ident,
        $demo_integer_shr_u:ident,
        $demo_integer_shr_u_ref:ident,
        $demo_integer_shr_round_assign_u:ident,
        $demo_integer_shr_round_u:ident,
        $demo_integer_shr_round_u_ref:ident,
        $benchmark_integer_shr_u_evaluation_strategy:ident,
        $benchmark_integer_shr_round_assign_u:ident,
        $benchmark_integer_shr_round_u_evaluation_strategy:ident
    ) => {
        fn $demo_integer_shr_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n >>= u;
                println!("x := {}; x >>= {}; x = {}", n_old, u, n);
            }
        }

        fn $demo_integer_shr_u(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} >> {} = {}", n_old, u, n >> u);
            }
        }

        fn $demo_integer_shr_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                println!("&{} >> {} = {}", n, u, &n >> u);
            }
        }

        fn $demo_integer_shr_round_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u, rm) in
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.shr_round_assign(u, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    n_old, u, rm, n
                );
            }
        }

        fn $demo_integer_shr_round_u(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
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

        fn $demo_integer_shr_round_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                println!(
                    "(&{}).shr_round({}, {}) = {}",
                    n,
                    u,
                    rm,
                    (&n).shr_round(u, rm)
                );
            }
        }

        fn $benchmark_integer_shr_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer >> {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_integer_and_small_unsigned::<$t>(gm),
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

        fn $benchmark_integer_shr_round_assign_u(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer.shr_round_assign({}, RoundingMode)", $t::NAME),
                BenchmarkType::Single,
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm),
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

        fn $benchmark_integer_shr_round_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer.shr_round({}, RoundingMode)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| other as usize),
                "other",
                &mut [
                    (
                        &format!("Integer.shr_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))),
                    ),
                    (
                        &format!("(&Integer).shr_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!((&x).shr_round(y, rm))),
                    ),
                ],
            );
        }
    };
}
demos_and_benches!(
    u8,
    demo_integer_shr_assign_u8,
    demo_integer_shr_u8,
    demo_integer_shr_u8_ref,
    demo_integer_shr_round_assign_u8,
    demo_integer_shr_round_u8,
    demo_integer_shr_round_u8_ref,
    benchmark_integer_shr_u8_evaluation_strategy,
    benchmark_integer_shr_round_assign_u8,
    benchmark_integer_shr_round_u8_evaluation_strategy
);
demos_and_benches!(
    u16,
    demo_integer_shr_assign_u16,
    demo_integer_shr_u16,
    demo_integer_shr_u16_ref,
    demo_integer_shr_round_assign_u16,
    demo_integer_shr_round_u16,
    demo_integer_shr_round_u16_ref,
    benchmark_integer_shr_u16_evaluation_strategy,
    benchmark_integer_shr_round_assign_u16,
    benchmark_integer_shr_round_u16_evaluation_strategy
);
demos_and_benches!(
    u32,
    demo_integer_shr_assign_u32,
    demo_integer_shr_u32,
    demo_integer_shr_u32_ref,
    demo_integer_shr_round_assign_u32,
    demo_integer_shr_round_u32,
    demo_integer_shr_round_u32_ref,
    benchmark_integer_shr_u32_evaluation_strategy,
    benchmark_integer_shr_round_assign_u32,
    benchmark_integer_shr_round_u32_evaluation_strategy
);
demos_and_benches!(
    u64,
    demo_integer_shr_assign_u64,
    demo_integer_shr_u64,
    demo_integer_shr_u64_ref,
    demo_integer_shr_round_assign_u64,
    demo_integer_shr_round_u64,
    demo_integer_shr_round_u64_ref,
    benchmark_integer_shr_u64_evaluation_strategy,
    benchmark_integer_shr_round_assign_u64,
    benchmark_integer_shr_round_u64_evaluation_strategy
);

fn benchmark_integer_shr_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer >>= u32",
        BenchmarkType::Single,
        rm_pairs_of_integer_and_small_unsigned::<u32>(gm),
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
        rm_pairs_of_integer_and_small_unsigned::<u32>(gm),
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
