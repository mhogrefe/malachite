use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_signed_small_u64_and_rounding_mode_var_2,
    triples_of_unsigned_small_u64_and_rounding_mode_var_2,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_u16_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_u32_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_u64_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_usize_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_i8_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_i16_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_i32_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_i64_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_isize_round_to_multiple_of_power_of_two);

    register_demo!(registry, demo_u8_round_to_multiple_of_power_of_two_assign);
    register_demo!(registry, demo_u16_round_to_multiple_of_power_of_two_assign);
    register_demo!(registry, demo_u32_round_to_multiple_of_power_of_two_assign);
    register_demo!(registry, demo_u64_round_to_multiple_of_power_of_two_assign);
    register_demo!(
        registry,
        demo_usize_round_to_multiple_of_power_of_two_assign
    );
    register_demo!(registry, demo_i8_round_to_multiple_of_power_of_two_assign);
    register_demo!(registry, demo_i16_round_to_multiple_of_power_of_two_assign);
    register_demo!(registry, demo_i32_round_to_multiple_of_power_of_two_assign);
    register_demo!(registry, demo_i64_round_to_multiple_of_power_of_two_assign);
    register_demo!(
        registry,
        demo_isize_round_to_multiple_of_power_of_two_assign
    );

    register_bench!(
        registry,
        None,
        benchmark_u8_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_i8_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_i16_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_i32_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_i64_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        None,
        benchmark_isize_round_to_multiple_of_power_of_two
    );

    register_bench!(
        registry,
        None,
        benchmark_u8_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_i8_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_i16_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_i32_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_i64_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        None,
        benchmark_isize_round_to_multiple_of_power_of_two_assign
    );
}

fn demo_unsigned_round_to_multiple_of_power_of_two<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (n, pow, rm) in triples_of_unsigned_small_u64_and_rounding_mode_var_2::<T>(gm).take(limit) {
        println!(
            "{}.round_to_multiple_of_power_of_two({}, {}) = {}",
            n,
            pow,
            rm,
            n.round_to_multiple_of_power_of_two(pow, rm)
        );
    }
}

fn demo_unsigned_round_to_multiple_of_power_of_two_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut n, pow, rm) in
        triples_of_unsigned_small_u64_and_rounding_mode_var_2::<T>(gm).take(limit)
    {
        let old_n = n;
        n.round_to_multiple_of_power_of_two_assign(pow, rm);
        println!(
            "n := {}; n.round_to_multiple_of_power_of_two({}, {}); n = {}",
            old_n, pow, rm, n
        );
    }
}

fn demo_signed_round_to_multiple_of_power_of_two<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, pow, rm) in triples_of_signed_small_u64_and_rounding_mode_var_2::<T>(gm).take(limit) {
        println!(
            "{}.round_to_multiple_of_power_of_two({}, {}) = {}",
            n,
            pow,
            rm,
            n.round_to_multiple_of_power_of_two(pow, rm)
        );
    }
}

fn demo_signed_round_to_multiple_of_power_of_two_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, pow, rm) in triples_of_signed_small_u64_and_rounding_mode_var_2::<T>(gm).take(limit)
    {
        let old_n = n;
        n.round_to_multiple_of_power_of_two_assign(pow, rm);
        println!(
            "n := {}; n.round_to_multiple_of_power_of_two({}, {}); n = {}",
            old_n, pow, rm, n
        );
    }
}

fn benchmark_unsigned_round_to_multiple_of_power_of_two<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!(
            "{}.round_to_multiple_of_power_of_two(u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::Single,
        triples_of_unsigned_small_u64_and_rounding_mode_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow, rm)| no_out!(n.round_to_multiple_of_power_of_two(pow, rm))),
        )],
    );
}

fn benchmark_unsigned_round_to_multiple_of_power_of_two_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!(
            "{}.round_to_multiple_of_power_of_two_assign(u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::Single,
        triples_of_unsigned_small_u64_and_rounding_mode_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow, rm)| n.round_to_multiple_of_power_of_two_assign(pow, rm)),
        )],
    );
}

fn benchmark_signed_round_to_multiple_of_power_of_two<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!(
            "{}.round_to_multiple_of_power_of_two(u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::Single,
        triples_of_signed_small_u64_and_rounding_mode_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow, rm)| no_out!(n.round_to_multiple_of_power_of_two(pow, rm))),
        )],
    );
}

fn benchmark_signed_round_to_multiple_of_power_of_two_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!(
            "{}.round_to_multiple_of_power_of_two_assign(u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::Single,
        triples_of_signed_small_u64_and_rounding_mode_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow, rm)| n.round_to_multiple_of_power_of_two_assign(pow, rm)),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name_floor:ident,
        $demo_name_ceiling:ident,
        $bench_name_floor:ident,
        $bench_name_ceiling:ident
    ) => {
        fn $demo_name_floor(gm: GenerationMode, limit: usize) {
            demo_unsigned_round_to_multiple_of_power_of_two::<$t>(gm, limit);
        }

        fn $demo_name_ceiling(gm: GenerationMode, limit: usize) {
            demo_unsigned_round_to_multiple_of_power_of_two_assign::<$t>(gm, limit);
        }

        fn $bench_name_floor(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_round_to_multiple_of_power_of_two::<$t>(gm, limit, file_name);
        }

        fn $bench_name_ceiling(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_round_to_multiple_of_power_of_two_assign::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name_floor:ident,
        $demo_name_ceiling:ident,
        $bench_name_floor:ident,
        $bench_name_ceiling:ident
    ) => {
        fn $demo_name_floor(gm: GenerationMode, limit: usize) {
            demo_signed_round_to_multiple_of_power_of_two::<$t>(gm, limit);
        }

        fn $demo_name_ceiling(gm: GenerationMode, limit: usize) {
            demo_signed_round_to_multiple_of_power_of_two_assign::<$t>(gm, limit);
        }

        fn $bench_name_floor(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_round_to_multiple_of_power_of_two::<$t>(gm, limit, file_name);
        }

        fn $bench_name_ceiling(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_round_to_multiple_of_power_of_two_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_round_to_multiple_of_power_of_two,
    demo_u8_round_to_multiple_of_power_of_two_assign,
    benchmark_u8_round_to_multiple_of_power_of_two,
    benchmark_u8_round_to_multiple_of_power_of_two_assign
);
unsigned!(
    u16,
    demo_u16_round_to_multiple_of_power_of_two,
    demo_u16_round_to_multiple_of_power_of_two_assign,
    benchmark_u16_round_to_multiple_of_power_of_two,
    benchmark_u16_round_to_multiple_of_power_of_two_assign
);
unsigned!(
    u32,
    demo_u32_round_to_multiple_of_power_of_two,
    demo_u32_round_to_multiple_of_power_of_two_assign,
    benchmark_u32_round_to_multiple_of_power_of_two,
    benchmark_u32_round_to_multiple_of_power_of_two_assign
);
unsigned!(
    u64,
    demo_u64_round_to_multiple_of_power_of_two,
    demo_u64_round_to_multiple_of_power_of_two_assign,
    benchmark_u64_round_to_multiple_of_power_of_two,
    benchmark_u64_round_to_multiple_of_power_of_two_assign
);
unsigned!(
    usize,
    demo_usize_round_to_multiple_of_power_of_two,
    demo_usize_round_to_multiple_of_power_of_two_assign,
    benchmark_usize_round_to_multiple_of_power_of_two,
    benchmark_usize_round_to_multiple_of_power_of_two_assign
);

signed!(
    i8,
    demo_i8_round_to_multiple_of_power_of_two,
    demo_i8_round_to_multiple_of_power_of_two_assign,
    benchmark_i8_round_to_multiple_of_power_of_two,
    benchmark_i8_round_to_multiple_of_power_of_two_assign
);
signed!(
    i16,
    demo_i16_round_to_multiple_of_power_of_two,
    demo_i16_round_to_multiple_of_power_of_two_assign,
    benchmark_i16_round_to_multiple_of_power_of_two,
    benchmark_i16_round_to_multiple_of_power_of_two_assign
);
signed!(
    i32,
    demo_i32_round_to_multiple_of_power_of_two,
    demo_i32_round_to_multiple_of_power_of_two_assign,
    benchmark_i32_round_to_multiple_of_power_of_two,
    benchmark_i32_round_to_multiple_of_power_of_two_assign
);
signed!(
    i64,
    demo_i64_round_to_multiple_of_power_of_two,
    demo_i64_round_to_multiple_of_power_of_two_assign,
    benchmark_i64_round_to_multiple_of_power_of_two,
    benchmark_i64_round_to_multiple_of_power_of_two_assign
);
signed!(
    isize,
    demo_isize_round_to_multiple_of_power_of_two,
    demo_isize_round_to_multiple_of_power_of_two_assign,
    benchmark_isize_round_to_multiple_of_power_of_two,
    benchmark_isize_round_to_multiple_of_power_of_two_assign
);
