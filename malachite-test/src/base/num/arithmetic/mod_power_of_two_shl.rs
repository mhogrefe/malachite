use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ModPowerOfTwoShl, ModPowerOfTwoShlAssign};
use malachite_base::num::conversion::traits::ExactFrom;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    triples_of_unsigned_small_signed_and_small_unsigned_var_1,
    triples_of_unsigned_small_unsigned_and_small_unsigned_var_3,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_u8);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_u16);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_u32);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_u64);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_usize);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_u8);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_u16);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_u32);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_u64);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_usize);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_u8);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_u16);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_u32);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_u64);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_usize);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_u8);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_u16);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_u32);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_u64);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_usize);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_u8);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_u16);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_u32);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_u64);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_usize);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_i8);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_i16);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_i32);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_i64);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_assign_isize);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_i8);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_i16);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_i32);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_i64);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_assign_isize);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_i8);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_i16);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_i32);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_i64);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_assign_isize);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_i8);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_i16);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_i32);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_i64);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_assign_isize);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_i8);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_i16);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_i32);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_i64);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_assign_isize);

    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_u8);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_u16);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_u32);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_u64);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_usize);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_u8);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_u16);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_u32);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_u64);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_usize);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_u8);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_u16);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_u32);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_u64);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_usize);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_u8);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_u16);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_u32);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_u64);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_usize);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_u8);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_u16);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_u32);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_u64);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_usize);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_i8);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_i16);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_i32);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_i64);
    register_ns_demo!(registry, demo_u8_mod_power_of_two_shl_isize);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_i8);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_i16);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_i32);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_i64);
    register_ns_demo!(registry, demo_u16_mod_power_of_two_shl_isize);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_i8);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_i16);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_i32);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_i64);
    register_ns_demo!(registry, demo_u32_mod_power_of_two_shl_isize);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_i8);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_i16);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_i32);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_i64);
    register_ns_demo!(registry, demo_u64_mod_power_of_two_shl_isize);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_i8);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_i16);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_i32);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_i64);
    register_ns_demo!(registry, demo_usize_mod_power_of_two_shl_isize);

    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_assign_u8);
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_two_shl_assign_u16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_two_shl_assign_u32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_two_shl_assign_u64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_two_shl_assign_usize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_u8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_u16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_u32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_u64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_usize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_u8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_u16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_u32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_u64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_usize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_u8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_u16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_u32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_u64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_usize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_u8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_u16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_u32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_u64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_usize
    );
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_assign_i8);
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_two_shl_assign_i16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_two_shl_assign_i32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_two_shl_assign_i64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_two_shl_assign_isize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_i8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_i16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_i32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_i64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_two_shl_assign_isize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_i8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_i16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_i32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_i64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_two_shl_assign_isize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_i8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_i16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_i32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_i64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_two_shl_assign_isize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_i8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_i16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_i32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_i64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_two_shl_assign_isize
    );

    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_u8);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_u16);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_u32);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_u64);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_usize);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_u8);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_u16);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_u32);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_u64);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_usize);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_u8);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_u16);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_u32);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_u64);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_usize);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_u8);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_u16);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_u32);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_u64);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_usize);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_u8);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_u16);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_u32);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_u64);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_usize);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_i8);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_i16);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_i32);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_i64);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_two_shl_isize);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_i8);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_i16);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_i32);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_i64);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_two_shl_isize);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_i8);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_i16);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_i32);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_i64);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_two_shl_isize);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_i8);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_i16);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_i32);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_i64);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_two_shl_isize);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_i8);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_i16);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_i32);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_i64);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_two_shl_isize);
}

macro_rules! mod_power_of_two_shl_u_u {
    (
        $t:ident,
        $u:ident,
        $demo_mod_power_of_two_shl_assign:ident,
        $demo_mod_power_of_two_shl:ident,
        $benchmark_mod_power_of_two_shl_assign:ident,
        $benchmark_mod_power_of_two_shl:ident
    ) => {
        fn $demo_mod_power_of_two_shl_assign(gm: NoSpecialGenerationMode, limit: usize) {
            for (mut n, u, pow) in
                triples_of_unsigned_small_unsigned_and_small_unsigned_var_3::<$t, $u>(gm)
                    .take(limit)
            {
                let old_n = n;
                n.mod_power_of_two_shl_assign(u, pow);
                println!(
                    "x := {}; x.mod_power_of_two_shl_assign({}, {}); x = {}",
                    old_n, u, pow, n
                );
            }
        }

        fn $demo_mod_power_of_two_shl(gm: NoSpecialGenerationMode, limit: usize) {
            for (n, u, pow) in
                triples_of_unsigned_small_unsigned_and_small_unsigned_var_3::<$t, $u>(gm)
                    .take(limit)
            {
                println!(
                    "{}.mod_power_of_two_shl({}, {}) = {}",
                    n,
                    u,
                    pow,
                    n.mod_power_of_two_shl(u, pow)
                );
            }
        }

        fn $benchmark_mod_power_of_two_shl_assign(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!(
                    "{}.mod_power_of_two_shl_assign({}, u64)",
                    $t::NAME,
                    $u::NAME
                ),
                BenchmarkType::Single,
                triples_of_unsigned_small_unsigned_and_small_unsigned_var_3::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, pow)| x.mod_power_of_two_shl_assign(y, pow)),
                )],
            );
        }

        fn $benchmark_mod_power_of_two_shl(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("{}.mod_power_of_two_shl({}, u64)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_unsigned_and_small_unsigned_var_3::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_shl(y, pow))),
                )],
            );
        }
    };
}
mod_power_of_two_shl_u_u!(
    u8,
    u8,
    demo_u8_mod_power_of_two_shl_assign_u8,
    demo_u8_mod_power_of_two_shl_u8,
    benchmark_u8_mod_power_of_two_shl_assign_u8,
    benchmark_u8_mod_power_of_two_shl_u8
);
mod_power_of_two_shl_u_u!(
    u8,
    u16,
    demo_u8_mod_power_of_two_shl_assign_u16,
    demo_u8_mod_power_of_two_shl_u16,
    benchmark_u8_mod_power_of_two_shl_assign_u16,
    benchmark_u8_mod_power_of_two_shl_u16
);
mod_power_of_two_shl_u_u!(
    u8,
    u32,
    demo_u8_mod_power_of_two_shl_assign_u32,
    demo_u8_mod_power_of_two_shl_u32,
    benchmark_u8_mod_power_of_two_shl_assign_u32,
    benchmark_u8_mod_power_of_two_shl_u32
);
mod_power_of_two_shl_u_u!(
    u8,
    u64,
    demo_u8_mod_power_of_two_shl_assign_u64,
    demo_u8_mod_power_of_two_shl_u64,
    benchmark_u8_mod_power_of_two_shl_assign_u64,
    benchmark_u8_mod_power_of_two_shl_u64
);
mod_power_of_two_shl_u_u!(
    u8,
    usize,
    demo_u8_mod_power_of_two_shl_assign_usize,
    demo_u8_mod_power_of_two_shl_usize,
    benchmark_u8_mod_power_of_two_shl_assign_usize,
    benchmark_u8_mod_power_of_two_shl_usize
);

mod_power_of_two_shl_u_u!(
    u16,
    u8,
    demo_u16_mod_power_of_two_shl_assign_u8,
    demo_u16_mod_power_of_two_shl_u8,
    benchmark_u16_mod_power_of_two_shl_assign_u8,
    benchmark_u16_mod_power_of_two_shl_u8
);
mod_power_of_two_shl_u_u!(
    u16,
    u16,
    demo_u16_mod_power_of_two_shl_assign_u16,
    demo_u16_mod_power_of_two_shl_u16,
    benchmark_u16_mod_power_of_two_shl_assign_u16,
    benchmark_u16_mod_power_of_two_shl_u16
);
mod_power_of_two_shl_u_u!(
    u16,
    u32,
    demo_u16_mod_power_of_two_shl_assign_u32,
    demo_u16_mod_power_of_two_shl_u32,
    benchmark_u16_mod_power_of_two_shl_assign_u32,
    benchmark_u16_mod_power_of_two_shl_u32
);
mod_power_of_two_shl_u_u!(
    u16,
    u64,
    demo_u16_mod_power_of_two_shl_assign_u64,
    demo_u16_mod_power_of_two_shl_u64,
    benchmark_u16_mod_power_of_two_shl_assign_u64,
    benchmark_u16_mod_power_of_two_shl_u64
);
mod_power_of_two_shl_u_u!(
    u16,
    usize,
    demo_u16_mod_power_of_two_shl_assign_usize,
    demo_u16_mod_power_of_two_shl_usize,
    benchmark_u16_mod_power_of_two_shl_assign_usize,
    benchmark_u16_mod_power_of_two_shl_usize
);

mod_power_of_two_shl_u_u!(
    u32,
    u8,
    demo_u32_mod_power_of_two_shl_assign_u8,
    demo_u32_mod_power_of_two_shl_u8,
    benchmark_u32_mod_power_of_two_shl_assign_u8,
    benchmark_u32_mod_power_of_two_shl_u8
);
mod_power_of_two_shl_u_u!(
    u32,
    u16,
    demo_u32_mod_power_of_two_shl_assign_u16,
    demo_u32_mod_power_of_two_shl_u16,
    benchmark_u32_mod_power_of_two_shl_assign_u16,
    benchmark_u32_mod_power_of_two_shl_u16
);
mod_power_of_two_shl_u_u!(
    u32,
    u32,
    demo_u32_mod_power_of_two_shl_assign_u32,
    demo_u32_mod_power_of_two_shl_u32,
    benchmark_u32_mod_power_of_two_shl_assign_u32,
    benchmark_u32_mod_power_of_two_shl_u32
);
mod_power_of_two_shl_u_u!(
    u32,
    u64,
    demo_u32_mod_power_of_two_shl_assign_u64,
    demo_u32_mod_power_of_two_shl_u64,
    benchmark_u32_mod_power_of_two_shl_assign_u64,
    benchmark_u32_mod_power_of_two_shl_u64
);
mod_power_of_two_shl_u_u!(
    u32,
    usize,
    demo_u32_mod_power_of_two_shl_assign_usize,
    demo_u32_mod_power_of_two_shl_usize,
    benchmark_u32_mod_power_of_two_shl_assign_usize,
    benchmark_u32_mod_power_of_two_shl_usize
);

mod_power_of_two_shl_u_u!(
    u64,
    u8,
    demo_u64_mod_power_of_two_shl_assign_u8,
    demo_u64_mod_power_of_two_shl_u8,
    benchmark_u64_mod_power_of_two_shl_assign_u8,
    benchmark_u64_mod_power_of_two_shl_u8
);
mod_power_of_two_shl_u_u!(
    u64,
    u16,
    demo_u64_mod_power_of_two_shl_assign_u16,
    demo_u64_mod_power_of_two_shl_u16,
    benchmark_u64_mod_power_of_two_shl_assign_u16,
    benchmark_u64_mod_power_of_two_shl_u16
);
mod_power_of_two_shl_u_u!(
    u64,
    u32,
    demo_u64_mod_power_of_two_shl_assign_u32,
    demo_u64_mod_power_of_two_shl_u32,
    benchmark_u64_mod_power_of_two_shl_assign_u32,
    benchmark_u64_mod_power_of_two_shl_u32
);
mod_power_of_two_shl_u_u!(
    u64,
    u64,
    demo_u64_mod_power_of_two_shl_assign_u64,
    demo_u64_mod_power_of_two_shl_u64,
    benchmark_u64_mod_power_of_two_shl_assign_u64,
    benchmark_u64_mod_power_of_two_shl_u64
);
mod_power_of_two_shl_u_u!(
    u64,
    usize,
    demo_u64_mod_power_of_two_shl_assign_usize,
    demo_u64_mod_power_of_two_shl_usize,
    benchmark_u64_mod_power_of_two_shl_assign_usize,
    benchmark_u64_mod_power_of_two_shl_usize
);

mod_power_of_two_shl_u_u!(
    usize,
    u8,
    demo_usize_mod_power_of_two_shl_assign_u8,
    demo_usize_mod_power_of_two_shl_u8,
    benchmark_usize_mod_power_of_two_shl_assign_u8,
    benchmark_usize_mod_power_of_two_shl_u8
);
mod_power_of_two_shl_u_u!(
    usize,
    u16,
    demo_usize_mod_power_of_two_shl_assign_u16,
    demo_usize_mod_power_of_two_shl_u16,
    benchmark_usize_mod_power_of_two_shl_assign_u16,
    benchmark_usize_mod_power_of_two_shl_u16
);
mod_power_of_two_shl_u_u!(
    usize,
    u32,
    demo_usize_mod_power_of_two_shl_assign_u32,
    demo_usize_mod_power_of_two_shl_u32,
    benchmark_usize_mod_power_of_two_shl_assign_u32,
    benchmark_usize_mod_power_of_two_shl_u32
);
mod_power_of_two_shl_u_u!(
    usize,
    u64,
    demo_usize_mod_power_of_two_shl_assign_u64,
    demo_usize_mod_power_of_two_shl_u64,
    benchmark_usize_mod_power_of_two_shl_assign_u64,
    benchmark_usize_mod_power_of_two_shl_u64
);
mod_power_of_two_shl_u_u!(
    usize,
    usize,
    demo_usize_mod_power_of_two_shl_assign_usize,
    demo_usize_mod_power_of_two_shl_usize,
    benchmark_usize_mod_power_of_two_shl_assign_usize,
    benchmark_usize_mod_power_of_two_shl_usize
);

macro_rules! mod_power_of_two_shl_u_i {
    (
        $t:ident,
        $u:ident,
        $demo_mod_power_of_two_shl_assign:ident,
        $demo_mod_power_of_two_shl:ident,
        $benchmark_mod_power_of_two_shl_assign:ident,
        $benchmark_mod_power_of_two_shl:ident
    ) => {
        fn $demo_mod_power_of_two_shl_assign(gm: NoSpecialGenerationMode, limit: usize) {
            for (mut n, i, pow) in
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<$t, $u>(gm).take(limit)
            {
                let old_n = n;
                n.mod_power_of_two_shl_assign(i, pow);
                println!(
                    "x := {}; x.mod_power_of_two_shl_assign({}, {}); x = {}",
                    old_n, i, pow, n
                );
            }
        }

        fn $demo_mod_power_of_two_shl(gm: NoSpecialGenerationMode, limit: usize) {
            for (n, i, pow) in
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<$t, $u>(gm).take(limit)
            {
                println!(
                    "{}.mod_power_of_two_shl({}, {}) = {}",
                    n,
                    i,
                    pow,
                    n.mod_power_of_two_shl(i, pow)
                );
            }
        }

        fn $benchmark_mod_power_of_two_shl_assign(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!(
                    "{}.mod_power_of_two_shl_assign({}, u64)",
                    $t::NAME,
                    $u::NAME
                ),
                BenchmarkType::Single,
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, pow)| x.mod_power_of_two_shl_assign(y, pow)),
                )],
            );
        }

        fn $benchmark_mod_power_of_two_shl(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("{}.mod_power_of_two_shl({}, u64)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(x, y, pow)| no_out!(x.mod_power_of_two_shl(y, pow))),
                )],
            );
        }
    };
}
mod_power_of_two_shl_u_i!(
    u8,
    i8,
    demo_u8_mod_power_of_two_shl_assign_i8,
    demo_u8_mod_power_of_two_shl_i8,
    benchmark_u8_mod_power_of_two_shl_assign_i8,
    benchmark_u8_mod_power_of_two_shl_i8
);
mod_power_of_two_shl_u_i!(
    u8,
    i16,
    demo_u8_mod_power_of_two_shl_assign_i16,
    demo_u8_mod_power_of_two_shl_i16,
    benchmark_u8_mod_power_of_two_shl_assign_i16,
    benchmark_u8_mod_power_of_two_shl_i16
);
mod_power_of_two_shl_u_i!(
    u8,
    i32,
    demo_u8_mod_power_of_two_shl_assign_i32,
    demo_u8_mod_power_of_two_shl_i32,
    benchmark_u8_mod_power_of_two_shl_assign_i32,
    benchmark_u8_mod_power_of_two_shl_i32
);
mod_power_of_two_shl_u_i!(
    u8,
    i64,
    demo_u8_mod_power_of_two_shl_assign_i64,
    demo_u8_mod_power_of_two_shl_i64,
    benchmark_u8_mod_power_of_two_shl_assign_i64,
    benchmark_u8_mod_power_of_two_shl_i64
);
mod_power_of_two_shl_u_i!(
    u8,
    isize,
    demo_u8_mod_power_of_two_shl_assign_isize,
    demo_u8_mod_power_of_two_shl_isize,
    benchmark_u8_mod_power_of_two_shl_assign_isize,
    benchmark_u8_mod_power_of_two_shl_isize
);

mod_power_of_two_shl_u_i!(
    u16,
    i8,
    demo_u16_mod_power_of_two_shl_assign_i8,
    demo_u16_mod_power_of_two_shl_i8,
    benchmark_u16_mod_power_of_two_shl_assign_i8,
    benchmark_u16_mod_power_of_two_shl_i8
);
mod_power_of_two_shl_u_i!(
    u16,
    i16,
    demo_u16_mod_power_of_two_shl_assign_i16,
    demo_u16_mod_power_of_two_shl_i16,
    benchmark_u16_mod_power_of_two_shl_assign_i16,
    benchmark_u16_mod_power_of_two_shl_i16
);
mod_power_of_two_shl_u_i!(
    u16,
    i32,
    demo_u16_mod_power_of_two_shl_assign_i32,
    demo_u16_mod_power_of_two_shl_i32,
    benchmark_u16_mod_power_of_two_shl_assign_i32,
    benchmark_u16_mod_power_of_two_shl_i32
);
mod_power_of_two_shl_u_i!(
    u16,
    i64,
    demo_u16_mod_power_of_two_shl_assign_i64,
    demo_u16_mod_power_of_two_shl_i64,
    benchmark_u16_mod_power_of_two_shl_assign_i64,
    benchmark_u16_mod_power_of_two_shl_i64
);
mod_power_of_two_shl_u_i!(
    u16,
    isize,
    demo_u16_mod_power_of_two_shl_assign_isize,
    demo_u16_mod_power_of_two_shl_isize,
    benchmark_u16_mod_power_of_two_shl_assign_isize,
    benchmark_u16_mod_power_of_two_shl_isize
);

mod_power_of_two_shl_u_i!(
    u32,
    i8,
    demo_u32_mod_power_of_two_shl_assign_i8,
    demo_u32_mod_power_of_two_shl_i8,
    benchmark_u32_mod_power_of_two_shl_assign_i8,
    benchmark_u32_mod_power_of_two_shl_i8
);
mod_power_of_two_shl_u_i!(
    u32,
    i16,
    demo_u32_mod_power_of_two_shl_assign_i16,
    demo_u32_mod_power_of_two_shl_i16,
    benchmark_u32_mod_power_of_two_shl_assign_i16,
    benchmark_u32_mod_power_of_two_shl_i16
);
mod_power_of_two_shl_u_i!(
    u32,
    i32,
    demo_u32_mod_power_of_two_shl_assign_i32,
    demo_u32_mod_power_of_two_shl_i32,
    benchmark_u32_mod_power_of_two_shl_assign_i32,
    benchmark_u32_mod_power_of_two_shl_i32
);
mod_power_of_two_shl_u_i!(
    u32,
    i64,
    demo_u32_mod_power_of_two_shl_assign_i64,
    demo_u32_mod_power_of_two_shl_i64,
    benchmark_u32_mod_power_of_two_shl_assign_i64,
    benchmark_u32_mod_power_of_two_shl_i64
);
mod_power_of_two_shl_u_i!(
    u32,
    isize,
    demo_u32_mod_power_of_two_shl_assign_isize,
    demo_u32_mod_power_of_two_shl_isize,
    benchmark_u32_mod_power_of_two_shl_assign_isize,
    benchmark_u32_mod_power_of_two_shl_isize
);

mod_power_of_two_shl_u_i!(
    u64,
    i8,
    demo_u64_mod_power_of_two_shl_assign_i8,
    demo_u64_mod_power_of_two_shl_i8,
    benchmark_u64_mod_power_of_two_shl_assign_i8,
    benchmark_u64_mod_power_of_two_shl_i8
);
mod_power_of_two_shl_u_i!(
    u64,
    i16,
    demo_u64_mod_power_of_two_shl_assign_i16,
    demo_u64_mod_power_of_two_shl_i16,
    benchmark_u64_mod_power_of_two_shl_assign_i16,
    benchmark_u64_mod_power_of_two_shl_i16
);
mod_power_of_two_shl_u_i!(
    u64,
    i32,
    demo_u64_mod_power_of_two_shl_assign_i32,
    demo_u64_mod_power_of_two_shl_i32,
    benchmark_u64_mod_power_of_two_shl_assign_i32,
    benchmark_u64_mod_power_of_two_shl_i32
);
mod_power_of_two_shl_u_i!(
    u64,
    i64,
    demo_u64_mod_power_of_two_shl_assign_i64,
    demo_u64_mod_power_of_two_shl_i64,
    benchmark_u64_mod_power_of_two_shl_assign_i64,
    benchmark_u64_mod_power_of_two_shl_i64
);
mod_power_of_two_shl_u_i!(
    u64,
    isize,
    demo_u64_mod_power_of_two_shl_assign_isize,
    demo_u64_mod_power_of_two_shl_isize,
    benchmark_u64_mod_power_of_two_shl_assign_isize,
    benchmark_u64_mod_power_of_two_shl_isize
);

mod_power_of_two_shl_u_i!(
    usize,
    i8,
    demo_usize_mod_power_of_two_shl_assign_i8,
    demo_usize_mod_power_of_two_shl_i8,
    benchmark_usize_mod_power_of_two_shl_assign_i8,
    benchmark_usize_mod_power_of_two_shl_i8
);
mod_power_of_two_shl_u_i!(
    usize,
    i16,
    demo_usize_mod_power_of_two_shl_assign_i16,
    demo_usize_mod_power_of_two_shl_i16,
    benchmark_usize_mod_power_of_two_shl_assign_i16,
    benchmark_usize_mod_power_of_two_shl_i16
);
mod_power_of_two_shl_u_i!(
    usize,
    i32,
    demo_usize_mod_power_of_two_shl_assign_i32,
    demo_usize_mod_power_of_two_shl_i32,
    benchmark_usize_mod_power_of_two_shl_assign_i32,
    benchmark_usize_mod_power_of_two_shl_i32
);
mod_power_of_two_shl_u_i!(
    usize,
    i64,
    demo_usize_mod_power_of_two_shl_assign_i64,
    demo_usize_mod_power_of_two_shl_i64,
    benchmark_usize_mod_power_of_two_shl_assign_i64,
    benchmark_usize_mod_power_of_two_shl_i64
);
mod_power_of_two_shl_u_i!(
    usize,
    isize,
    demo_usize_mod_power_of_two_shl_assign_isize,
    demo_usize_mod_power_of_two_shl_isize,
    benchmark_usize_mod_power_of_two_shl_assign_isize,
    benchmark_usize_mod_power_of_two_shl_isize
);
