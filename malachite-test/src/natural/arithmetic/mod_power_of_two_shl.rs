use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoShl, ModPowerOfTwoShlAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::natural::{
    triples_of_natural_small_signed_and_u64_var_1, triples_of_natural_small_unsigned_and_u64_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_u8);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_u16);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_u32);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_u64);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_usize);

    register_demo!(registry, demo_natural_mod_power_of_two_shl_u8);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_u16);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_u32);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_u64);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_usize);

    register_demo!(registry, demo_natural_mod_power_of_two_shl_u8_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_u16_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_u32_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_u64_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_usize_ref);

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_u8
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_u16
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_u32
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_u64
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_usize
    );

    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_i8);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_i16);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_i32);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_i64);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_assign_isize);

    register_demo!(registry, demo_natural_mod_power_of_two_shl_i8);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_i16);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_i32);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_i64);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_isize);

    register_demo!(registry, demo_natural_mod_power_of_two_shl_i8_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_i16_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_i32_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_i64_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_shl_isize_ref);

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_usize_evaluation_strategy
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_u8_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_u16_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_u32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_u64_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_usize_algorithms
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_i8
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_i16
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_i32
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_i64
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_assign_isize
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_i64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_isize_evaluation_strategy
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_i8_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_i16_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_i32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_i64_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_shl_isize_algorithms
    );
}

macro_rules! demos_and_benches_u {
    (
        $t:ident,
        $demo_natural_mod_power_of_two_shl_assign_u:ident,
        $demo_natural_mod_power_of_two_shl_u:ident,
        $demo_natural_mod_power_of_two_shl_u_ref:ident,
        $benchmark_natural_mod_power_of_two_shl_assign_u:ident,
        $benchmark_natural_mod_power_of_two_shl_u_evaluation_strategy:ident,
        $benchmark_natural_mod_power_of_two_shl_u_algorithms:ident
    ) => {
        fn $demo_natural_mod_power_of_two_shl_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u, pow) in
                triples_of_natural_small_unsigned_and_u64_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.mod_power_of_two_shl_assign(u, pow);
                println!(
                    "x := {}; x.mod_power_of_two_shl_assign({}, {}); x = {}",
                    n_old, u, pow, n
                );
            }
        }

        fn $demo_natural_mod_power_of_two_shl_u(gm: GenerationMode, limit: usize) {
            for (n, u, pow) in triples_of_natural_small_unsigned_and_u64_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                println!(
                    "{}.mod_power_of_two_shl({}, {}) = {}",
                    n_old,
                    u,
                    pow,
                    n.mod_power_of_two_shl(u, pow)
                );
            }
        }

        fn $demo_natural_mod_power_of_two_shl_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u, pow) in triples_of_natural_small_unsigned_and_u64_var_1::<$t>(gm).take(limit)
            {
                println!(
                    "(&{}).mod_power_of_two_shl({}, {}) = {}",
                    n,
                    u,
                    pow,
                    (&n).mod_power_of_two_shl(u, pow)
                );
            }
        }

        fn $benchmark_natural_mod_power_of_two_shl_assign_u(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.mod_power_of_two_shl_assign({}, u64)", $t::NAME),
                BenchmarkType::Single,
                triples_of_natural_small_unsigned_and_u64_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "pow",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, pow)| x.mod_power_of_two_shl_assign(y, pow)),
                )],
            );
        }

        fn $benchmark_natural_mod_power_of_two_shl_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.mod_power_of_two_shl({}, u64)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_natural_small_unsigned_and_u64_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "pow",
                &mut [
                    (
                        &format!("Natural.mod_power_of_two_shl({}, u64)", $t::NAME),
                        &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_shl(y, pow))),
                    ),
                    (
                        &format!("(&Natural).mod_power_of_two_shl({}, u64)", $t::NAME),
                        &mut (|(x, y, pow)| no_out!((&x).mod_power_of_two_shl(y, pow))),
                    ),
                ],
            );
        }

        fn $benchmark_natural_mod_power_of_two_shl_u_algorithms(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.mod_power_of_two_shl({}, u64)", $t::NAME),
                BenchmarkType::Algorithms,
                triples_of_natural_small_unsigned_and_u64_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "pow",
                &mut [
                    (
                        "default",
                        &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_shl(y, pow))),
                    ),
                    (
                        &format!("(Natural << {}).mod_power_of_two(u64)", $t::NAME),
                        &mut (|(x, y, pow)| no_out!((x << y).mod_power_of_two(pow))),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_u!(
    u8,
    demo_natural_mod_power_of_two_shl_assign_u8,
    demo_natural_mod_power_of_two_shl_u8,
    demo_natural_mod_power_of_two_shl_u8_ref,
    benchmark_natural_mod_power_of_two_shl_assign_u8,
    benchmark_natural_mod_power_of_two_shl_u8_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_u8_algorithms
);
demos_and_benches_u!(
    u16,
    demo_natural_mod_power_of_two_shl_assign_u16,
    demo_natural_mod_power_of_two_shl_u16,
    demo_natural_mod_power_of_two_shl_u16_ref,
    benchmark_natural_mod_power_of_two_shl_assign_u16,
    benchmark_natural_mod_power_of_two_shl_u16_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_u16_algorithms
);
demos_and_benches_u!(
    u32,
    demo_natural_mod_power_of_two_shl_assign_u32,
    demo_natural_mod_power_of_two_shl_u32,
    demo_natural_mod_power_of_two_shl_u32_ref,
    benchmark_natural_mod_power_of_two_shl_assign_u32,
    benchmark_natural_mod_power_of_two_shl_u32_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_u32_algorithms
);
demos_and_benches_u!(
    u64,
    demo_natural_mod_power_of_two_shl_assign_u64,
    demo_natural_mod_power_of_two_shl_u64,
    demo_natural_mod_power_of_two_shl_u64_ref,
    benchmark_natural_mod_power_of_two_shl_assign_u64,
    benchmark_natural_mod_power_of_two_shl_u64_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_u64_algorithms
);
demos_and_benches_u!(
    usize,
    demo_natural_mod_power_of_two_shl_assign_usize,
    demo_natural_mod_power_of_two_shl_usize,
    demo_natural_mod_power_of_two_shl_usize_ref,
    benchmark_natural_mod_power_of_two_shl_assign_usize,
    benchmark_natural_mod_power_of_two_shl_usize_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_usize_algorithms
);

macro_rules! demos_and_benches_i {
    (
        $t:ident,
        $demo_natural_mod_power_of_two_shl_assign_i:ident,
        $demo_natural_mod_power_of_two_shl_i:ident,
        $demo_natural_mod_power_of_two_shl_i_ref:ident,
        $benchmark_natural_mod_power_of_two_shl_assign_i:ident,
        $benchmark_natural_mod_power_of_two_shl_i_evaluation_strategy:ident,
        $benchmark_natural_mod_power_of_two_shl_i_algorithms:ident
    ) => {
        fn $demo_natural_mod_power_of_two_shl_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i, pow) in
                triples_of_natural_small_signed_and_u64_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.mod_power_of_two_shl_assign(i, pow);
                println!(
                    "x := {}; x.mod_power_of_two_shl_assign({}, {}); x = {}",
                    n_old, i, pow, n
                );
            }
        }

        fn $demo_natural_mod_power_of_two_shl_i(gm: GenerationMode, limit: usize) {
            for (n, i, pow) in triples_of_natural_small_signed_and_u64_var_1::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!(
                    "{}.mod_power_of_two_shl({}, {}) = {}",
                    n_old,
                    i,
                    pow,
                    n.mod_power_of_two_shl(i, pow)
                );
            }
        }

        fn $demo_natural_mod_power_of_two_shl_i_ref(gm: GenerationMode, limit: usize) {
            for (n, i, pow) in triples_of_natural_small_signed_and_u64_var_1::<$t>(gm).take(limit) {
                println!(
                    "(&{}).mod_power_of_two_shl({}, {}) = {}",
                    n,
                    i,
                    pow,
                    (&n).mod_power_of_two_shl(i, pow)
                );
            }
        }

        fn $benchmark_natural_mod_power_of_two_shl_assign_i(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.mod_power_of_two_shl_assign({}, u64)", $t::NAME),
                BenchmarkType::Single,
                triples_of_natural_small_signed_and_u64_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "pow",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, pow)| x.mod_power_of_two_shl_assign(y, pow)),
                )],
            );
        }

        fn $benchmark_natural_mod_power_of_two_shl_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.mod_power_of_two_shl({}, u64)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_natural_small_signed_and_u64_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "pow",
                &mut [
                    (
                        &format!("Natural.mod_power_of_two_shl({}, u64)", $t::NAME),
                        &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_shl(y, pow))),
                    ),
                    (
                        &format!("(&Natural).mod_power_of_two_shl({}, u64)", $t::NAME),
                        &mut (|(x, y, pow)| no_out!((&x).mod_power_of_two_shl(y, pow))),
                    ),
                ],
            );
        }

        fn $benchmark_natural_mod_power_of_two_shl_i_algorithms(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.mod_power_of_two_shl({}, u64)", $t::NAME),
                BenchmarkType::Algorithms,
                triples_of_natural_small_signed_and_u64_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "pow",
                &mut [
                    (
                        "default",
                        &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_shl(y, pow))),
                    ),
                    (
                        &format!("(Natural << {}).mod_power_of_two(u64)", $t::NAME),
                        &mut (|(x, y, pow)| no_out!((x << y).mod_power_of_two(pow))),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_i!(
    i8,
    demo_natural_mod_power_of_two_shl_assign_i8,
    demo_natural_mod_power_of_two_shl_i8,
    demo_natural_mod_power_of_two_shl_i8_ref,
    benchmark_natural_mod_power_of_two_shl_assign_i8,
    benchmark_natural_mod_power_of_two_shl_i8_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_i8_algorithms
);
demos_and_benches_i!(
    i16,
    demo_natural_mod_power_of_two_shl_assign_i16,
    demo_natural_mod_power_of_two_shl_i16,
    demo_natural_mod_power_of_two_shl_i16_ref,
    benchmark_natural_mod_power_of_two_shl_assign_i16,
    benchmark_natural_mod_power_of_two_shl_i16_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_i16_algorithms
);
demos_and_benches_i!(
    i32,
    demo_natural_mod_power_of_two_shl_assign_i32,
    demo_natural_mod_power_of_two_shl_i32,
    demo_natural_mod_power_of_two_shl_i32_ref,
    benchmark_natural_mod_power_of_two_shl_assign_i32,
    benchmark_natural_mod_power_of_two_shl_i32_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_i32_algorithms
);
demos_and_benches_i!(
    i64,
    demo_natural_mod_power_of_two_shl_assign_i64,
    demo_natural_mod_power_of_two_shl_i64,
    demo_natural_mod_power_of_two_shl_i64_ref,
    benchmark_natural_mod_power_of_two_shl_assign_i64,
    benchmark_natural_mod_power_of_two_shl_i64_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_i64_algorithms
);
demos_and_benches_i!(
    isize,
    demo_natural_mod_power_of_two_shl_assign_isize,
    demo_natural_mod_power_of_two_shl_isize,
    demo_natural_mod_power_of_two_shl_isize_ref,
    benchmark_natural_mod_power_of_two_shl_assign_isize,
    benchmark_natural_mod_power_of_two_shl_isize_evaluation_strategy,
    benchmark_natural_mod_power_of_two_shl_isize_algorithms
);
