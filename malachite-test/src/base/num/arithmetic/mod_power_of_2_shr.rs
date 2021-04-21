use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Shr, ModPowerOf2ShrAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::triples_of_unsigned_small_signed_and_small_unsigned_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_assign_i8);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_assign_i16);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_assign_i32);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_assign_i64);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_assign_isize);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_assign_i8);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_assign_i16);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_assign_i32);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_assign_i64);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_assign_isize);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_assign_i8);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_assign_i16);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_assign_i32);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_assign_i64);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_assign_isize);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_assign_i8);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_assign_i16);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_assign_i32);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_assign_i64);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_assign_isize);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_assign_i8);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_assign_i16);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_assign_i32);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_assign_i64);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_assign_isize);

    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_i8);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_i16);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_i32);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_i64);
    register_ns_demo!(registry, demo_u8_mod_power_of_2_shr_isize);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_i8);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_i16);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_i32);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_i64);
    register_ns_demo!(registry, demo_u16_mod_power_of_2_shr_isize);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_i8);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_i16);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_i32);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_i64);
    register_ns_demo!(registry, demo_u32_mod_power_of_2_shr_isize);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_i8);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_i16);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_i32);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_i64);
    register_ns_demo!(registry, demo_u64_mod_power_of_2_shr_isize);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_i8);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_i16);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_i32);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_i64);
    register_ns_demo!(registry, demo_usize_mod_power_of_2_shr_isize);

    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_assign_i8);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_assign_i16);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_assign_i32);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_assign_i64);
    register_ns_bench!(
        registry,
        Large,
        benchmark_u8_mod_power_of_2_shr_assign_isize
    );
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_assign_i8);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_assign_i16);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_assign_i32);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_assign_i64);
    register_ns_bench!(
        registry,
        Large,
        benchmark_u16_mod_power_of_2_shr_assign_isize
    );
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_assign_i8);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_assign_i16);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_assign_i32);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_assign_i64);
    register_ns_bench!(
        registry,
        Large,
        benchmark_u32_mod_power_of_2_shr_assign_isize
    );
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_assign_i8);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_assign_i16);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_assign_i32);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_assign_i64);
    register_ns_bench!(
        registry,
        Large,
        benchmark_u64_mod_power_of_2_shr_assign_isize
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_2_shr_assign_i8
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_2_shr_assign_i16
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_2_shr_assign_i32
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_2_shr_assign_i64
    );
    register_ns_bench!(
        registry,
        Large,
        benchmark_usize_mod_power_of_2_shr_assign_isize
    );

    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_i8);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_i16);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_i32);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_i64);
    register_ns_bench!(registry, Large, benchmark_u8_mod_power_of_2_shr_isize);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_i8);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_i16);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_i32);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_i64);
    register_ns_bench!(registry, Large, benchmark_u16_mod_power_of_2_shr_isize);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_i8);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_i16);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_i32);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_i64);
    register_ns_bench!(registry, Large, benchmark_u32_mod_power_of_2_shr_isize);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_i8);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_i16);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_i32);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_i64);
    register_ns_bench!(registry, Large, benchmark_u64_mod_power_of_2_shr_isize);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_2_shr_i8);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_2_shr_i16);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_2_shr_i32);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_2_shr_i64);
    register_ns_bench!(registry, Large, benchmark_usize_mod_power_of_2_shr_isize);
}

macro_rules! mod_power_of_2_shr_u_i {
    (
        $t:ident,
        $u:ident,
        $demo_mod_power_of_2_shr_assign:ident,
        $demo_mod_power_of_2_shr:ident,
        $benchmark_mod_power_of_2_shr_assign:ident,
        $benchmark_mod_power_of_2_shr:ident
    ) => {
        fn $demo_mod_power_of_2_shr_assign(gm: NoSpecialGenerationMode, limit: usize) {
            for (mut n, i, pow) in
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<$t, $u>(gm).take(limit)
            {
                let old_n = n;
                n.mod_power_of_2_shr_assign(i, pow);
                println!(
                    "x := {}; x.mod_power_of_2_shr_assign({}, {}); x = {}",
                    old_n, i, pow, n
                );
            }
        }

        fn $demo_mod_power_of_2_shr(gm: NoSpecialGenerationMode, limit: usize) {
            for (n, i, pow) in
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<$t, $u>(gm).take(limit)
            {
                println!(
                    "{}.mod_power_of_2_shr({}, {}) = {}",
                    n,
                    i,
                    pow,
                    n.mod_power_of_2_shr(i, pow)
                );
            }
        }

        fn $benchmark_mod_power_of_2_shr_assign(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("{}.mod_power_of_2_shr_assign({}, u64)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "other",
                &mut [(
                    "Malachite",
                    &mut (|(mut x, y, pow)| x.mod_power_of_2_shr_assign(y, pow)),
                )],
            );
        }

        fn $benchmark_mod_power_of_2_shr(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark_old(
                &format!("{}.mod_power_of_2_shr({}, u64)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, _, pow)| usize::exact_from(pow)),
                "other",
                &mut [(
                    "Malachite",
                    &mut (|(x, y, pow)| no_out!(x.mod_power_of_2_shr(y, pow))),
                )],
            );
        }
    };
}
mod_power_of_2_shr_u_i!(
    u8,
    i8,
    demo_u8_mod_power_of_2_shr_assign_i8,
    demo_u8_mod_power_of_2_shr_i8,
    benchmark_u8_mod_power_of_2_shr_assign_i8,
    benchmark_u8_mod_power_of_2_shr_i8
);
mod_power_of_2_shr_u_i!(
    u8,
    i16,
    demo_u8_mod_power_of_2_shr_assign_i16,
    demo_u8_mod_power_of_2_shr_i16,
    benchmark_u8_mod_power_of_2_shr_assign_i16,
    benchmark_u8_mod_power_of_2_shr_i16
);
mod_power_of_2_shr_u_i!(
    u8,
    i32,
    demo_u8_mod_power_of_2_shr_assign_i32,
    demo_u8_mod_power_of_2_shr_i32,
    benchmark_u8_mod_power_of_2_shr_assign_i32,
    benchmark_u8_mod_power_of_2_shr_i32
);
mod_power_of_2_shr_u_i!(
    u8,
    i64,
    demo_u8_mod_power_of_2_shr_assign_i64,
    demo_u8_mod_power_of_2_shr_i64,
    benchmark_u8_mod_power_of_2_shr_assign_i64,
    benchmark_u8_mod_power_of_2_shr_i64
);
mod_power_of_2_shr_u_i!(
    u8,
    isize,
    demo_u8_mod_power_of_2_shr_assign_isize,
    demo_u8_mod_power_of_2_shr_isize,
    benchmark_u8_mod_power_of_2_shr_assign_isize,
    benchmark_u8_mod_power_of_2_shr_isize
);

mod_power_of_2_shr_u_i!(
    u16,
    i8,
    demo_u16_mod_power_of_2_shr_assign_i8,
    demo_u16_mod_power_of_2_shr_i8,
    benchmark_u16_mod_power_of_2_shr_assign_i8,
    benchmark_u16_mod_power_of_2_shr_i8
);
mod_power_of_2_shr_u_i!(
    u16,
    i16,
    demo_u16_mod_power_of_2_shr_assign_i16,
    demo_u16_mod_power_of_2_shr_i16,
    benchmark_u16_mod_power_of_2_shr_assign_i16,
    benchmark_u16_mod_power_of_2_shr_i16
);
mod_power_of_2_shr_u_i!(
    u16,
    i32,
    demo_u16_mod_power_of_2_shr_assign_i32,
    demo_u16_mod_power_of_2_shr_i32,
    benchmark_u16_mod_power_of_2_shr_assign_i32,
    benchmark_u16_mod_power_of_2_shr_i32
);
mod_power_of_2_shr_u_i!(
    u16,
    i64,
    demo_u16_mod_power_of_2_shr_assign_i64,
    demo_u16_mod_power_of_2_shr_i64,
    benchmark_u16_mod_power_of_2_shr_assign_i64,
    benchmark_u16_mod_power_of_2_shr_i64
);
mod_power_of_2_shr_u_i!(
    u16,
    isize,
    demo_u16_mod_power_of_2_shr_assign_isize,
    demo_u16_mod_power_of_2_shr_isize,
    benchmark_u16_mod_power_of_2_shr_assign_isize,
    benchmark_u16_mod_power_of_2_shr_isize
);

mod_power_of_2_shr_u_i!(
    u32,
    i8,
    demo_u32_mod_power_of_2_shr_assign_i8,
    demo_u32_mod_power_of_2_shr_i8,
    benchmark_u32_mod_power_of_2_shr_assign_i8,
    benchmark_u32_mod_power_of_2_shr_i8
);
mod_power_of_2_shr_u_i!(
    u32,
    i16,
    demo_u32_mod_power_of_2_shr_assign_i16,
    demo_u32_mod_power_of_2_shr_i16,
    benchmark_u32_mod_power_of_2_shr_assign_i16,
    benchmark_u32_mod_power_of_2_shr_i16
);
mod_power_of_2_shr_u_i!(
    u32,
    i32,
    demo_u32_mod_power_of_2_shr_assign_i32,
    demo_u32_mod_power_of_2_shr_i32,
    benchmark_u32_mod_power_of_2_shr_assign_i32,
    benchmark_u32_mod_power_of_2_shr_i32
);
mod_power_of_2_shr_u_i!(
    u32,
    i64,
    demo_u32_mod_power_of_2_shr_assign_i64,
    demo_u32_mod_power_of_2_shr_i64,
    benchmark_u32_mod_power_of_2_shr_assign_i64,
    benchmark_u32_mod_power_of_2_shr_i64
);
mod_power_of_2_shr_u_i!(
    u32,
    isize,
    demo_u32_mod_power_of_2_shr_assign_isize,
    demo_u32_mod_power_of_2_shr_isize,
    benchmark_u32_mod_power_of_2_shr_assign_isize,
    benchmark_u32_mod_power_of_2_shr_isize
);

mod_power_of_2_shr_u_i!(
    u64,
    i8,
    demo_u64_mod_power_of_2_shr_assign_i8,
    demo_u64_mod_power_of_2_shr_i8,
    benchmark_u64_mod_power_of_2_shr_assign_i8,
    benchmark_u64_mod_power_of_2_shr_i8
);
mod_power_of_2_shr_u_i!(
    u64,
    i16,
    demo_u64_mod_power_of_2_shr_assign_i16,
    demo_u64_mod_power_of_2_shr_i16,
    benchmark_u64_mod_power_of_2_shr_assign_i16,
    benchmark_u64_mod_power_of_2_shr_i16
);
mod_power_of_2_shr_u_i!(
    u64,
    i32,
    demo_u64_mod_power_of_2_shr_assign_i32,
    demo_u64_mod_power_of_2_shr_i32,
    benchmark_u64_mod_power_of_2_shr_assign_i32,
    benchmark_u64_mod_power_of_2_shr_i32
);
mod_power_of_2_shr_u_i!(
    u64,
    i64,
    demo_u64_mod_power_of_2_shr_assign_i64,
    demo_u64_mod_power_of_2_shr_i64,
    benchmark_u64_mod_power_of_2_shr_assign_i64,
    benchmark_u64_mod_power_of_2_shr_i64
);
mod_power_of_2_shr_u_i!(
    u64,
    isize,
    demo_u64_mod_power_of_2_shr_assign_isize,
    demo_u64_mod_power_of_2_shr_isize,
    benchmark_u64_mod_power_of_2_shr_assign_isize,
    benchmark_u64_mod_power_of_2_shr_isize
);

mod_power_of_2_shr_u_i!(
    usize,
    i8,
    demo_usize_mod_power_of_2_shr_assign_i8,
    demo_usize_mod_power_of_2_shr_i8,
    benchmark_usize_mod_power_of_2_shr_assign_i8,
    benchmark_usize_mod_power_of_2_shr_i8
);
mod_power_of_2_shr_u_i!(
    usize,
    i16,
    demo_usize_mod_power_of_2_shr_assign_i16,
    demo_usize_mod_power_of_2_shr_i16,
    benchmark_usize_mod_power_of_2_shr_assign_i16,
    benchmark_usize_mod_power_of_2_shr_i16
);
mod_power_of_2_shr_u_i!(
    usize,
    i32,
    demo_usize_mod_power_of_2_shr_assign_i32,
    demo_usize_mod_power_of_2_shr_i32,
    benchmark_usize_mod_power_of_2_shr_assign_i32,
    benchmark_usize_mod_power_of_2_shr_i32
);
mod_power_of_2_shr_u_i!(
    usize,
    i64,
    demo_usize_mod_power_of_2_shr_assign_i64,
    demo_usize_mod_power_of_2_shr_i64,
    benchmark_usize_mod_power_of_2_shr_assign_i64,
    benchmark_usize_mod_power_of_2_shr_i64
);
mod_power_of_2_shr_u_i!(
    usize,
    isize,
    demo_usize_mod_power_of_2_shr_assign_isize,
    demo_usize_mod_power_of_2_shr_isize,
    benchmark_usize_mod_power_of_2_shr_assign_isize,
    benchmark_usize_mod_power_of_2_shr_isize
);
