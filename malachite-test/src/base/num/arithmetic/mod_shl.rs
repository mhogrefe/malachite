use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_unsigned_signed_and_unsigned_var_1, triples_of_unsigned_unsigned_and_unsigned_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_shl_assign_u8);
    register_demo!(registry, demo_u8_mod_shl_assign_u16);
    register_demo!(registry, demo_u8_mod_shl_assign_u32);
    register_demo!(registry, demo_u8_mod_shl_assign_u64);
    register_demo!(registry, demo_u8_mod_shl_assign_usize);
    register_demo!(registry, demo_u16_mod_shl_assign_u8);
    register_demo!(registry, demo_u16_mod_shl_assign_u16);
    register_demo!(registry, demo_u16_mod_shl_assign_u32);
    register_demo!(registry, demo_u16_mod_shl_assign_u64);
    register_demo!(registry, demo_u16_mod_shl_assign_usize);
    register_demo!(registry, demo_u32_mod_shl_assign_u8);
    register_demo!(registry, demo_u32_mod_shl_assign_u16);
    register_demo!(registry, demo_u32_mod_shl_assign_u32);
    register_demo!(registry, demo_u32_mod_shl_assign_u64);
    register_demo!(registry, demo_u32_mod_shl_assign_usize);
    register_demo!(registry, demo_u64_mod_shl_assign_u8);
    register_demo!(registry, demo_u64_mod_shl_assign_u16);
    register_demo!(registry, demo_u64_mod_shl_assign_u32);
    register_demo!(registry, demo_u64_mod_shl_assign_u64);
    register_demo!(registry, demo_u64_mod_shl_assign_usize);
    register_demo!(registry, demo_usize_mod_shl_assign_u8);
    register_demo!(registry, demo_usize_mod_shl_assign_u16);
    register_demo!(registry, demo_usize_mod_shl_assign_u32);
    register_demo!(registry, demo_usize_mod_shl_assign_u64);
    register_demo!(registry, demo_usize_mod_shl_assign_usize);
    register_demo!(registry, demo_u8_mod_shl_assign_i8);
    register_demo!(registry, demo_u8_mod_shl_assign_i16);
    register_demo!(registry, demo_u8_mod_shl_assign_i32);
    register_demo!(registry, demo_u8_mod_shl_assign_i64);
    register_demo!(registry, demo_u8_mod_shl_assign_isize);
    register_demo!(registry, demo_u16_mod_shl_assign_i8);
    register_demo!(registry, demo_u16_mod_shl_assign_i16);
    register_demo!(registry, demo_u16_mod_shl_assign_i32);
    register_demo!(registry, demo_u16_mod_shl_assign_i64);
    register_demo!(registry, demo_u16_mod_shl_assign_isize);
    register_demo!(registry, demo_u32_mod_shl_assign_i8);
    register_demo!(registry, demo_u32_mod_shl_assign_i16);
    register_demo!(registry, demo_u32_mod_shl_assign_i32);
    register_demo!(registry, demo_u32_mod_shl_assign_i64);
    register_demo!(registry, demo_u32_mod_shl_assign_isize);
    register_demo!(registry, demo_u64_mod_shl_assign_i8);
    register_demo!(registry, demo_u64_mod_shl_assign_i16);
    register_demo!(registry, demo_u64_mod_shl_assign_i32);
    register_demo!(registry, demo_u64_mod_shl_assign_i64);
    register_demo!(registry, demo_u64_mod_shl_assign_isize);
    register_demo!(registry, demo_usize_mod_shl_assign_i8);
    register_demo!(registry, demo_usize_mod_shl_assign_i16);
    register_demo!(registry, demo_usize_mod_shl_assign_i32);
    register_demo!(registry, demo_usize_mod_shl_assign_i64);
    register_demo!(registry, demo_usize_mod_shl_assign_isize);

    register_demo!(registry, demo_u8_mod_shl_u8);
    register_demo!(registry, demo_u8_mod_shl_u16);
    register_demo!(registry, demo_u8_mod_shl_u32);
    register_demo!(registry, demo_u8_mod_shl_u64);
    register_demo!(registry, demo_u8_mod_shl_usize);
    register_demo!(registry, demo_u16_mod_shl_u8);
    register_demo!(registry, demo_u16_mod_shl_u16);
    register_demo!(registry, demo_u16_mod_shl_u32);
    register_demo!(registry, demo_u16_mod_shl_u64);
    register_demo!(registry, demo_u16_mod_shl_usize);
    register_demo!(registry, demo_u32_mod_shl_u8);
    register_demo!(registry, demo_u32_mod_shl_u16);
    register_demo!(registry, demo_u32_mod_shl_u32);
    register_demo!(registry, demo_u32_mod_shl_u64);
    register_demo!(registry, demo_u32_mod_shl_usize);
    register_demo!(registry, demo_u64_mod_shl_u8);
    register_demo!(registry, demo_u64_mod_shl_u16);
    register_demo!(registry, demo_u64_mod_shl_u32);
    register_demo!(registry, demo_u64_mod_shl_u64);
    register_demo!(registry, demo_u64_mod_shl_usize);
    register_demo!(registry, demo_usize_mod_shl_u8);
    register_demo!(registry, demo_usize_mod_shl_u16);
    register_demo!(registry, demo_usize_mod_shl_u32);
    register_demo!(registry, demo_usize_mod_shl_u64);
    register_demo!(registry, demo_usize_mod_shl_usize);
    register_demo!(registry, demo_u8_mod_shl_i8);
    register_demo!(registry, demo_u8_mod_shl_i16);
    register_demo!(registry, demo_u8_mod_shl_i32);
    register_demo!(registry, demo_u8_mod_shl_i64);
    register_demo!(registry, demo_u8_mod_shl_isize);
    register_demo!(registry, demo_u16_mod_shl_i8);
    register_demo!(registry, demo_u16_mod_shl_i16);
    register_demo!(registry, demo_u16_mod_shl_i32);
    register_demo!(registry, demo_u16_mod_shl_i64);
    register_demo!(registry, demo_u16_mod_shl_isize);
    register_demo!(registry, demo_u32_mod_shl_i8);
    register_demo!(registry, demo_u32_mod_shl_i16);
    register_demo!(registry, demo_u32_mod_shl_i32);
    register_demo!(registry, demo_u32_mod_shl_i64);
    register_demo!(registry, demo_u32_mod_shl_isize);
    register_demo!(registry, demo_u64_mod_shl_i8);
    register_demo!(registry, demo_u64_mod_shl_i16);
    register_demo!(registry, demo_u64_mod_shl_i32);
    register_demo!(registry, demo_u64_mod_shl_i64);
    register_demo!(registry, demo_u64_mod_shl_isize);
    register_demo!(registry, demo_usize_mod_shl_i8);
    register_demo!(registry, demo_usize_mod_shl_i16);
    register_demo!(registry, demo_usize_mod_shl_i32);
    register_demo!(registry, demo_usize_mod_shl_i64);
    register_demo!(registry, demo_usize_mod_shl_isize);

    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_u8);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_u16);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_u32);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_u64);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_usize);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_u8);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_u16);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_u32);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_u64);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_usize);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_u8);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_u16);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_u32);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_u64);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_usize);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_u8);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_u16);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_u32);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_u64);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_usize);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_u8);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_u16);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_u32);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_u64);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_usize);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_i8);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_i16);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_i32);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_i64);
    register_bench!(registry, Large, benchmark_u8_mod_shl_assign_isize);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_i8);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_i16);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_i32);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_i64);
    register_bench!(registry, Large, benchmark_u16_mod_shl_assign_isize);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_i8);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_i16);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_i32);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_i64);
    register_bench!(registry, Large, benchmark_u32_mod_shl_assign_isize);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_i8);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_i16);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_i32);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_i64);
    register_bench!(registry, Large, benchmark_u64_mod_shl_assign_isize);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_i8);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_i16);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_i32);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_i64);
    register_bench!(registry, Large, benchmark_usize_mod_shl_assign_isize);

    register_bench!(registry, Large, benchmark_u8_mod_shl_u8);
    register_bench!(registry, Large, benchmark_u8_mod_shl_u16);
    register_bench!(registry, Large, benchmark_u8_mod_shl_u32);
    register_bench!(registry, Large, benchmark_u8_mod_shl_u64);
    register_bench!(registry, Large, benchmark_u8_mod_shl_usize);
    register_bench!(registry, Large, benchmark_u16_mod_shl_u8);
    register_bench!(registry, Large, benchmark_u16_mod_shl_u16);
    register_bench!(registry, Large, benchmark_u16_mod_shl_u32);
    register_bench!(registry, Large, benchmark_u16_mod_shl_u64);
    register_bench!(registry, Large, benchmark_u16_mod_shl_usize);
    register_bench!(registry, Large, benchmark_u32_mod_shl_u8);
    register_bench!(registry, Large, benchmark_u32_mod_shl_u16);
    register_bench!(registry, Large, benchmark_u32_mod_shl_u32);
    register_bench!(registry, Large, benchmark_u32_mod_shl_u64);
    register_bench!(registry, Large, benchmark_u32_mod_shl_usize);
    register_bench!(registry, Large, benchmark_u64_mod_shl_u8);
    register_bench!(registry, Large, benchmark_u64_mod_shl_u16);
    register_bench!(registry, Large, benchmark_u64_mod_shl_u32);
    register_bench!(registry, Large, benchmark_u64_mod_shl_u64);
    register_bench!(registry, Large, benchmark_u64_mod_shl_usize);
    register_bench!(registry, Large, benchmark_usize_mod_shl_u8);
    register_bench!(registry, Large, benchmark_usize_mod_shl_u16);
    register_bench!(registry, Large, benchmark_usize_mod_shl_u32);
    register_bench!(registry, Large, benchmark_usize_mod_shl_u64);
    register_bench!(registry, Large, benchmark_usize_mod_shl_usize);
    register_bench!(registry, Large, benchmark_u8_mod_shl_i8);
    register_bench!(registry, Large, benchmark_u8_mod_shl_i16);
    register_bench!(registry, Large, benchmark_u8_mod_shl_i32);
    register_bench!(registry, Large, benchmark_u8_mod_shl_i64);
    register_bench!(registry, Large, benchmark_u8_mod_shl_isize);
    register_bench!(registry, Large, benchmark_u16_mod_shl_i8);
    register_bench!(registry, Large, benchmark_u16_mod_shl_i16);
    register_bench!(registry, Large, benchmark_u16_mod_shl_i32);
    register_bench!(registry, Large, benchmark_u16_mod_shl_i64);
    register_bench!(registry, Large, benchmark_u16_mod_shl_isize);
    register_bench!(registry, Large, benchmark_u32_mod_shl_i8);
    register_bench!(registry, Large, benchmark_u32_mod_shl_i16);
    register_bench!(registry, Large, benchmark_u32_mod_shl_i32);
    register_bench!(registry, Large, benchmark_u32_mod_shl_i64);
    register_bench!(registry, Large, benchmark_u32_mod_shl_isize);
    register_bench!(registry, Large, benchmark_u64_mod_shl_i8);
    register_bench!(registry, Large, benchmark_u64_mod_shl_i16);
    register_bench!(registry, Large, benchmark_u64_mod_shl_i32);
    register_bench!(registry, Large, benchmark_u64_mod_shl_i64);
    register_bench!(registry, Large, benchmark_u64_mod_shl_isize);
    register_bench!(registry, Large, benchmark_usize_mod_shl_i8);
    register_bench!(registry, Large, benchmark_usize_mod_shl_i16);
    register_bench!(registry, Large, benchmark_usize_mod_shl_i32);
    register_bench!(registry, Large, benchmark_usize_mod_shl_i64);
    register_bench!(registry, Large, benchmark_usize_mod_shl_isize);
}

fn demo_mod_shl_unsigned<T: PrimitiveUnsigned + Rand + SampleRange, U: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: ModShl<U, T, Output = T>,
{
    for (x, pow, m) in triples_of_unsigned_unsigned_and_unsigned_var_1::<T, U>(gm).take(limit) {
        println!("{}.pow({}) === {} mod {}", x, pow, x.mod_shl(pow, m), m);
    }
}

fn demo_mod_shl_assign_unsigned<
    T: PrimitiveUnsigned + Rand + SampleRange,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
) where
    T: ModShlAssign<U, T>,
{
    for (mut x, pow, m) in triples_of_unsigned_unsigned_and_unsigned_var_1::<T, U>(gm).take(limit) {
        let old_x = x;
        x.mod_shl_assign(pow, m);
        println!(
            "x := {}; x.mod_shl_assign({}, {}); x = {}",
            old_x, pow, m, x
        );
    }
}

fn benchmark_mod_shl_unsigned<
    T: PrimitiveUnsigned + Rand + SampleRange,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: ModShl<U, T, Output = T>,
{
    run_benchmark_old(
        &format!("{}.mod_shl({}, {})", T::NAME, U::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_unsigned_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow, _)| usize::exact_from(pow.significant_bits())),
        "pow.significant_bits()",
        &mut [("Malachite", &mut (|(x, pow, m)| no_out!(x.mod_shl(pow, m))))],
    );
}

fn benchmark_mod_shl_assign_unsigned<
    T: PrimitiveUnsigned + Rand + SampleRange,
    U: PrimitiveUnsigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: ModShlAssign<U, T>,
{
    run_benchmark_old(
        &format!("{}.mod_shl_assign({}, {})", T::NAME, U::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_unsigned_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow, _)| usize::exact_from(pow.significant_bits())),
        "pow.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut x, pow, m)| x.mod_shl_assign(pow, m)),
        )],
    );
}

macro_rules! mod_shl_u_u {
    (
        $t:ident,
        $u:ident,
        $demo_mod_shl_assign:ident,
        $demo_mod_shl:ident,
        $benchmark_mod_shl_assign:ident,
        $benchmark_mod_shl:ident
    ) => {
        fn $demo_mod_shl_assign(gm: GenerationMode, limit: usize) {
            demo_mod_shl_assign_unsigned::<$t, $u>(gm, limit);
        }

        fn $demo_mod_shl(gm: GenerationMode, limit: usize) {
            demo_mod_shl_unsigned::<$t, $u>(gm, limit);
        }

        fn $benchmark_mod_shl_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_shl_unsigned::<$t, $u>(gm, limit, file_name);
        }

        fn $benchmark_mod_shl(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_shl_assign_unsigned::<$t, $u>(gm, limit, file_name);
        }
    };
}
mod_shl_u_u!(
    u8,
    u8,
    demo_u8_mod_shl_assign_u8,
    demo_u8_mod_shl_u8,
    benchmark_u8_mod_shl_assign_u8,
    benchmark_u8_mod_shl_u8
);
mod_shl_u_u!(
    u8,
    u16,
    demo_u8_mod_shl_assign_u16,
    demo_u8_mod_shl_u16,
    benchmark_u8_mod_shl_assign_u16,
    benchmark_u8_mod_shl_u16
);
mod_shl_u_u!(
    u8,
    u32,
    demo_u8_mod_shl_assign_u32,
    demo_u8_mod_shl_u32,
    benchmark_u8_mod_shl_assign_u32,
    benchmark_u8_mod_shl_u32
);
mod_shl_u_u!(
    u8,
    u64,
    demo_u8_mod_shl_assign_u64,
    demo_u8_mod_shl_u64,
    benchmark_u8_mod_shl_assign_u64,
    benchmark_u8_mod_shl_u64
);
mod_shl_u_u!(
    u8,
    usize,
    demo_u8_mod_shl_assign_usize,
    demo_u8_mod_shl_usize,
    benchmark_u8_mod_shl_assign_usize,
    benchmark_u8_mod_shl_usize
);

mod_shl_u_u!(
    u16,
    u8,
    demo_u16_mod_shl_assign_u8,
    demo_u16_mod_shl_u8,
    benchmark_u16_mod_shl_assign_u8,
    benchmark_u16_mod_shl_u8
);
mod_shl_u_u!(
    u16,
    u16,
    demo_u16_mod_shl_assign_u16,
    demo_u16_mod_shl_u16,
    benchmark_u16_mod_shl_assign_u16,
    benchmark_u16_mod_shl_u16
);
mod_shl_u_u!(
    u16,
    u32,
    demo_u16_mod_shl_assign_u32,
    demo_u16_mod_shl_u32,
    benchmark_u16_mod_shl_assign_u32,
    benchmark_u16_mod_shl_u32
);
mod_shl_u_u!(
    u16,
    u64,
    demo_u16_mod_shl_assign_u64,
    demo_u16_mod_shl_u64,
    benchmark_u16_mod_shl_assign_u64,
    benchmark_u16_mod_shl_u64
);
mod_shl_u_u!(
    u16,
    usize,
    demo_u16_mod_shl_assign_usize,
    demo_u16_mod_shl_usize,
    benchmark_u16_mod_shl_assign_usize,
    benchmark_u16_mod_shl_usize
);

mod_shl_u_u!(
    u32,
    u8,
    demo_u32_mod_shl_assign_u8,
    demo_u32_mod_shl_u8,
    benchmark_u32_mod_shl_assign_u8,
    benchmark_u32_mod_shl_u8
);
mod_shl_u_u!(
    u32,
    u16,
    demo_u32_mod_shl_assign_u16,
    demo_u32_mod_shl_u16,
    benchmark_u32_mod_shl_assign_u16,
    benchmark_u32_mod_shl_u16
);
mod_shl_u_u!(
    u32,
    u32,
    demo_u32_mod_shl_assign_u32,
    demo_u32_mod_shl_u32,
    benchmark_u32_mod_shl_assign_u32,
    benchmark_u32_mod_shl_u32
);
mod_shl_u_u!(
    u32,
    u64,
    demo_u32_mod_shl_assign_u64,
    demo_u32_mod_shl_u64,
    benchmark_u32_mod_shl_assign_u64,
    benchmark_u32_mod_shl_u64
);
mod_shl_u_u!(
    u32,
    usize,
    demo_u32_mod_shl_assign_usize,
    demo_u32_mod_shl_usize,
    benchmark_u32_mod_shl_assign_usize,
    benchmark_u32_mod_shl_usize
);

mod_shl_u_u!(
    u64,
    u8,
    demo_u64_mod_shl_assign_u8,
    demo_u64_mod_shl_u8,
    benchmark_u64_mod_shl_assign_u8,
    benchmark_u64_mod_shl_u8
);
mod_shl_u_u!(
    u64,
    u16,
    demo_u64_mod_shl_assign_u16,
    demo_u64_mod_shl_u16,
    benchmark_u64_mod_shl_assign_u16,
    benchmark_u64_mod_shl_u16
);
mod_shl_u_u!(
    u64,
    u32,
    demo_u64_mod_shl_assign_u32,
    demo_u64_mod_shl_u32,
    benchmark_u64_mod_shl_assign_u32,
    benchmark_u64_mod_shl_u32
);
mod_shl_u_u!(
    u64,
    u64,
    demo_u64_mod_shl_assign_u64,
    demo_u64_mod_shl_u64,
    benchmark_u64_mod_shl_assign_u64,
    benchmark_u64_mod_shl_u64
);
mod_shl_u_u!(
    u64,
    usize,
    demo_u64_mod_shl_assign_usize,
    demo_u64_mod_shl_usize,
    benchmark_u64_mod_shl_assign_usize,
    benchmark_u64_mod_shl_usize
);

mod_shl_u_u!(
    usize,
    u8,
    demo_usize_mod_shl_assign_u8,
    demo_usize_mod_shl_u8,
    benchmark_usize_mod_shl_assign_u8,
    benchmark_usize_mod_shl_u8
);
mod_shl_u_u!(
    usize,
    u16,
    demo_usize_mod_shl_assign_u16,
    demo_usize_mod_shl_u16,
    benchmark_usize_mod_shl_assign_u16,
    benchmark_usize_mod_shl_u16
);
mod_shl_u_u!(
    usize,
    u32,
    demo_usize_mod_shl_assign_u32,
    demo_usize_mod_shl_u32,
    benchmark_usize_mod_shl_assign_u32,
    benchmark_usize_mod_shl_u32
);
mod_shl_u_u!(
    usize,
    u64,
    demo_usize_mod_shl_assign_u64,
    demo_usize_mod_shl_u64,
    benchmark_usize_mod_shl_assign_u64,
    benchmark_usize_mod_shl_u64
);
mod_shl_u_u!(
    usize,
    usize,
    demo_usize_mod_shl_assign_usize,
    demo_usize_mod_shl_usize,
    benchmark_usize_mod_shl_assign_usize,
    benchmark_usize_mod_shl_usize
);

fn demo_mod_shl_signed<T: PrimitiveUnsigned + Rand + SampleRange, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T: ModShl<U, T, Output = T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, pow, m) in triples_of_unsigned_signed_and_unsigned_var_1::<T, U>(gm).take(limit) {
        println!("{}.pow({}) === {} mod {}", x, pow, x.mod_shl(pow, m), m);
    }
}

fn demo_mod_shl_assign_signed<
    T: PrimitiveUnsigned + Rand + SampleRange,
    U: PrimitiveSigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
) where
    T: ModShlAssign<U, T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, pow, m) in triples_of_unsigned_signed_and_unsigned_var_1::<T, U>(gm).take(limit) {
        let old_x = x;
        x.mod_shl_assign(pow, m);
        println!(
            "x := {}; x.mod_shl_assign({}, {}); x = {}",
            old_x, pow, m, x
        );
    }
}

fn benchmark_mod_shl_signed<T: PrimitiveUnsigned + Rand + SampleRange, U: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: ModShl<U, T, Output = T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.mod_shl({}, {})", T::NAME, U::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_signed_and_unsigned_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow, _)| usize::exact_from(pow.significant_bits())),
        "pow.significant_bits()",
        &mut [("Malachite", &mut (|(x, pow, m)| no_out!(x.mod_shl(pow, m))))],
    );
}

fn benchmark_mod_shl_assign_signed<
    T: PrimitiveUnsigned + Rand + SampleRange,
    U: PrimitiveSigned + Rand,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: ModShlAssign<U, T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.mod_shl_assign({}, {})", T::NAME, U::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_signed_and_unsigned_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow, _)| usize::exact_from(pow.significant_bits())),
        "pow.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut x, pow, m)| x.mod_shl_assign(pow, m)),
        )],
    );
}

macro_rules! mod_shl_u_i {
    (
        $t:ident,
        $u:ident,
        $demo_mod_shl_assign:ident,
        $demo_mod_shl:ident,
        $benchmark_mod_shl_assign:ident,
        $benchmark_mod_shl:ident
    ) => {
        fn $demo_mod_shl_assign(gm: GenerationMode, limit: usize) {
            demo_mod_shl_assign_signed::<$t, $u>(gm, limit);
        }

        fn $demo_mod_shl(gm: GenerationMode, limit: usize) {
            demo_mod_shl_signed::<$t, $u>(gm, limit);
        }

        fn $benchmark_mod_shl_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_shl_signed::<$t, $u>(gm, limit, file_name);
        }

        fn $benchmark_mod_shl(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_shl_assign_signed::<$t, $u>(gm, limit, file_name);
        }
    };
}
mod_shl_u_i!(
    u8,
    i8,
    demo_u8_mod_shl_assign_i8,
    demo_u8_mod_shl_i8,
    benchmark_u8_mod_shl_assign_i8,
    benchmark_u8_mod_shl_i8
);
mod_shl_u_i!(
    u8,
    i16,
    demo_u8_mod_shl_assign_i16,
    demo_u8_mod_shl_i16,
    benchmark_u8_mod_shl_assign_i16,
    benchmark_u8_mod_shl_i16
);
mod_shl_u_i!(
    u8,
    i32,
    demo_u8_mod_shl_assign_i32,
    demo_u8_mod_shl_i32,
    benchmark_u8_mod_shl_assign_i32,
    benchmark_u8_mod_shl_i32
);
mod_shl_u_i!(
    u8,
    i64,
    demo_u8_mod_shl_assign_i64,
    demo_u8_mod_shl_i64,
    benchmark_u8_mod_shl_assign_i64,
    benchmark_u8_mod_shl_i64
);
mod_shl_u_i!(
    u8,
    isize,
    demo_u8_mod_shl_assign_isize,
    demo_u8_mod_shl_isize,
    benchmark_u8_mod_shl_assign_isize,
    benchmark_u8_mod_shl_isize
);

mod_shl_u_i!(
    u16,
    i8,
    demo_u16_mod_shl_assign_i8,
    demo_u16_mod_shl_i8,
    benchmark_u16_mod_shl_assign_i8,
    benchmark_u16_mod_shl_i8
);
mod_shl_u_i!(
    u16,
    i16,
    demo_u16_mod_shl_assign_i16,
    demo_u16_mod_shl_i16,
    benchmark_u16_mod_shl_assign_i16,
    benchmark_u16_mod_shl_i16
);
mod_shl_u_i!(
    u16,
    i32,
    demo_u16_mod_shl_assign_i32,
    demo_u16_mod_shl_i32,
    benchmark_u16_mod_shl_assign_i32,
    benchmark_u16_mod_shl_i32
);
mod_shl_u_i!(
    u16,
    i64,
    demo_u16_mod_shl_assign_i64,
    demo_u16_mod_shl_i64,
    benchmark_u16_mod_shl_assign_i64,
    benchmark_u16_mod_shl_i64
);
mod_shl_u_i!(
    u16,
    isize,
    demo_u16_mod_shl_assign_isize,
    demo_u16_mod_shl_isize,
    benchmark_u16_mod_shl_assign_isize,
    benchmark_u16_mod_shl_isize
);

mod_shl_u_i!(
    u32,
    i8,
    demo_u32_mod_shl_assign_i8,
    demo_u32_mod_shl_i8,
    benchmark_u32_mod_shl_assign_i8,
    benchmark_u32_mod_shl_i8
);
mod_shl_u_i!(
    u32,
    i16,
    demo_u32_mod_shl_assign_i16,
    demo_u32_mod_shl_i16,
    benchmark_u32_mod_shl_assign_i16,
    benchmark_u32_mod_shl_i16
);
mod_shl_u_i!(
    u32,
    i32,
    demo_u32_mod_shl_assign_i32,
    demo_u32_mod_shl_i32,
    benchmark_u32_mod_shl_assign_i32,
    benchmark_u32_mod_shl_i32
);
mod_shl_u_i!(
    u32,
    i64,
    demo_u32_mod_shl_assign_i64,
    demo_u32_mod_shl_i64,
    benchmark_u32_mod_shl_assign_i64,
    benchmark_u32_mod_shl_i64
);
mod_shl_u_i!(
    u32,
    isize,
    demo_u32_mod_shl_assign_isize,
    demo_u32_mod_shl_isize,
    benchmark_u32_mod_shl_assign_isize,
    benchmark_u32_mod_shl_isize
);

mod_shl_u_i!(
    u64,
    i8,
    demo_u64_mod_shl_assign_i8,
    demo_u64_mod_shl_i8,
    benchmark_u64_mod_shl_assign_i8,
    benchmark_u64_mod_shl_i8
);
mod_shl_u_i!(
    u64,
    i16,
    demo_u64_mod_shl_assign_i16,
    demo_u64_mod_shl_i16,
    benchmark_u64_mod_shl_assign_i16,
    benchmark_u64_mod_shl_i16
);
mod_shl_u_i!(
    u64,
    i32,
    demo_u64_mod_shl_assign_i32,
    demo_u64_mod_shl_i32,
    benchmark_u64_mod_shl_assign_i32,
    benchmark_u64_mod_shl_i32
);
mod_shl_u_i!(
    u64,
    i64,
    demo_u64_mod_shl_assign_i64,
    demo_u64_mod_shl_i64,
    benchmark_u64_mod_shl_assign_i64,
    benchmark_u64_mod_shl_i64
);
mod_shl_u_i!(
    u64,
    isize,
    demo_u64_mod_shl_assign_isize,
    demo_u64_mod_shl_isize,
    benchmark_u64_mod_shl_assign_isize,
    benchmark_u64_mod_shl_isize
);

mod_shl_u_i!(
    usize,
    i8,
    demo_usize_mod_shl_assign_i8,
    demo_usize_mod_shl_i8,
    benchmark_usize_mod_shl_assign_i8,
    benchmark_usize_mod_shl_i8
);
mod_shl_u_i!(
    usize,
    i16,
    demo_usize_mod_shl_assign_i16,
    demo_usize_mod_shl_i16,
    benchmark_usize_mod_shl_assign_i16,
    benchmark_usize_mod_shl_i16
);
mod_shl_u_i!(
    usize,
    i32,
    demo_usize_mod_shl_assign_i32,
    demo_usize_mod_shl_i32,
    benchmark_usize_mod_shl_assign_i32,
    benchmark_usize_mod_shl_i32
);
mod_shl_u_i!(
    usize,
    i64,
    demo_usize_mod_shl_assign_i64,
    demo_usize_mod_shl_i64,
    benchmark_usize_mod_shl_assign_i64,
    benchmark_usize_mod_shl_i64
);
mod_shl_u_i!(
    usize,
    isize,
    demo_usize_mod_shl_assign_isize,
    demo_usize_mod_shl_isize,
    benchmark_usize_mod_shl_assign_isize,
    benchmark_usize_mod_shl_isize
);
