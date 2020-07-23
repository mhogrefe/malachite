use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_signeds_var_2, pairs_of_unsigned_and_positive_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_div_mod);
    register_demo!(registry, demo_u16_div_mod);
    register_demo!(registry, demo_u32_div_mod);
    register_demo!(registry, demo_u64_div_mod);
    register_demo!(registry, demo_usize_div_mod);
    register_demo!(registry, demo_i8_div_mod);
    register_demo!(registry, demo_i16_div_mod);
    register_demo!(registry, demo_i32_div_mod);
    register_demo!(registry, demo_i64_div_mod);
    register_demo!(registry, demo_isize_div_mod);

    register_demo!(registry, demo_u8_div_assign_mod);
    register_demo!(registry, demo_u16_div_assign_mod);
    register_demo!(registry, demo_u32_div_assign_mod);
    register_demo!(registry, demo_u64_div_assign_mod);
    register_demo!(registry, demo_usize_div_assign_mod);
    register_demo!(registry, demo_i8_div_assign_mod);
    register_demo!(registry, demo_i16_div_assign_mod);
    register_demo!(registry, demo_i32_div_assign_mod);
    register_demo!(registry, demo_i64_div_assign_mod);
    register_demo!(registry, demo_isize_div_assign_mod);

    register_demo!(registry, demo_u8_div_rem);
    register_demo!(registry, demo_u16_div_rem);
    register_demo!(registry, demo_u32_div_rem);
    register_demo!(registry, demo_u64_div_rem);
    register_demo!(registry, demo_usize_div_rem);
    register_demo!(registry, demo_i8_div_rem);
    register_demo!(registry, demo_i16_div_rem);
    register_demo!(registry, demo_i32_div_rem);
    register_demo!(registry, demo_i64_div_rem);
    register_demo!(registry, demo_isize_div_rem);

    register_demo!(registry, demo_u8_div_assign_rem);
    register_demo!(registry, demo_u16_div_assign_rem);
    register_demo!(registry, demo_u32_div_assign_rem);
    register_demo!(registry, demo_u64_div_assign_rem);
    register_demo!(registry, demo_usize_div_assign_rem);
    register_demo!(registry, demo_i8_div_assign_rem);
    register_demo!(registry, demo_i16_div_assign_rem);
    register_demo!(registry, demo_i32_div_assign_rem);
    register_demo!(registry, demo_i64_div_assign_rem);
    register_demo!(registry, demo_isize_div_assign_rem);

    register_demo!(registry, demo_u8_ceiling_div_neg_mod);
    register_demo!(registry, demo_u16_ceiling_div_neg_mod);
    register_demo!(registry, demo_u32_ceiling_div_neg_mod);
    register_demo!(registry, demo_u64_ceiling_div_neg_mod);
    register_demo!(registry, demo_usize_ceiling_div_neg_mod);

    register_demo!(registry, demo_u8_ceiling_div_assign_neg_mod);
    register_demo!(registry, demo_u16_ceiling_div_assign_neg_mod);
    register_demo!(registry, demo_u32_ceiling_div_assign_neg_mod);
    register_demo!(registry, demo_u64_ceiling_div_assign_neg_mod);
    register_demo!(registry, demo_usize_ceiling_div_assign_neg_mod);

    register_demo!(registry, demo_i8_ceiling_div_mod);
    register_demo!(registry, demo_i16_ceiling_div_mod);
    register_demo!(registry, demo_i32_ceiling_div_mod);
    register_demo!(registry, demo_i64_ceiling_div_mod);
    register_demo!(registry, demo_isize_ceiling_div_mod);

    register_demo!(registry, demo_i8_ceiling_div_assign_mod);
    register_demo!(registry, demo_i16_ceiling_div_assign_mod);
    register_demo!(registry, demo_i32_ceiling_div_assign_mod);
    register_demo!(registry, demo_i64_ceiling_div_assign_mod);
    register_demo!(registry, demo_isize_ceiling_div_assign_mod);

    register_bench!(registry, None, benchmark_u8_div_mod_algorithms);
    register_bench!(registry, None, benchmark_u16_div_mod_algorithms);
    register_bench!(registry, None, benchmark_u32_div_mod_algorithms);
    register_bench!(registry, None, benchmark_u64_div_mod_algorithms);
    register_bench!(registry, None, benchmark_usize_div_mod_algorithms);
    register_bench!(registry, None, benchmark_i8_div_mod_algorithms);
    register_bench!(registry, None, benchmark_i16_div_mod_algorithms);
    register_bench!(registry, None, benchmark_i32_div_mod_algorithms);
    register_bench!(registry, None, benchmark_i64_div_mod_algorithms);
    register_bench!(registry, None, benchmark_isize_div_mod_algorithms);

    register_bench!(registry, None, benchmark_u8_div_assign_mod);
    register_bench!(registry, None, benchmark_u16_div_assign_mod);
    register_bench!(registry, None, benchmark_u32_div_assign_mod);
    register_bench!(registry, None, benchmark_u64_div_assign_mod);
    register_bench!(registry, None, benchmark_usize_div_assign_mod);
    register_bench!(registry, None, benchmark_i8_div_assign_mod);
    register_bench!(registry, None, benchmark_i16_div_assign_mod);
    register_bench!(registry, None, benchmark_i32_div_assign_mod);
    register_bench!(registry, None, benchmark_i64_div_assign_mod);
    register_bench!(registry, None, benchmark_isize_div_assign_mod);

    register_bench!(registry, None, benchmark_u8_div_rem_algorithms);
    register_bench!(registry, None, benchmark_u16_div_rem_algorithms);
    register_bench!(registry, None, benchmark_u32_div_rem_algorithms);
    register_bench!(registry, None, benchmark_u64_div_rem_algorithms);
    register_bench!(registry, None, benchmark_usize_div_rem_algorithms);
    register_bench!(registry, None, benchmark_i8_div_rem_algorithms);
    register_bench!(registry, None, benchmark_i16_div_rem_algorithms);
    register_bench!(registry, None, benchmark_i32_div_rem_algorithms);
    register_bench!(registry, None, benchmark_i64_div_rem_algorithms);
    register_bench!(registry, None, benchmark_isize_div_rem_algorithms);

    register_bench!(registry, None, benchmark_u8_div_assign_rem);
    register_bench!(registry, None, benchmark_u16_div_assign_rem);
    register_bench!(registry, None, benchmark_u32_div_assign_rem);
    register_bench!(registry, None, benchmark_u64_div_assign_rem);
    register_bench!(registry, None, benchmark_usize_div_assign_rem);
    register_bench!(registry, None, benchmark_i8_div_assign_rem);
    register_bench!(registry, None, benchmark_i16_div_assign_rem);
    register_bench!(registry, None, benchmark_i32_div_assign_rem);
    register_bench!(registry, None, benchmark_i64_div_assign_rem);
    register_bench!(registry, None, benchmark_isize_div_assign_rem);

    register_bench!(registry, None, benchmark_u8_ceiling_div_neg_mod_algorithms);
    register_bench!(registry, None, benchmark_u16_ceiling_div_neg_mod_algorithms);
    register_bench!(registry, None, benchmark_u32_ceiling_div_neg_mod_algorithms);
    register_bench!(registry, None, benchmark_u64_ceiling_div_neg_mod_algorithms);
    register_bench!(
        registry,
        None,
        benchmark_usize_ceiling_div_neg_mod_algorithms
    );

    register_bench!(registry, None, benchmark_u8_ceiling_div_assign_neg_mod);
    register_bench!(registry, None, benchmark_u16_ceiling_div_assign_neg_mod);
    register_bench!(registry, None, benchmark_u32_ceiling_div_assign_neg_mod);
    register_bench!(registry, None, benchmark_u64_ceiling_div_assign_neg_mod);
    register_bench!(registry, None, benchmark_usize_ceiling_div_assign_neg_mod);

    register_bench!(registry, None, benchmark_i8_ceiling_div_mod_algorithms);
    register_bench!(registry, None, benchmark_i16_ceiling_div_mod_algorithms);
    register_bench!(registry, None, benchmark_i32_ceiling_div_mod_algorithms);
    register_bench!(registry, None, benchmark_i64_ceiling_div_mod_algorithms);
    register_bench!(registry, None, benchmark_isize_ceiling_div_mod_algorithms);

    register_bench!(registry, None, benchmark_i8_ceiling_div_assign_mod);
    register_bench!(registry, None, benchmark_i16_ceiling_div_assign_mod);
    register_bench!(registry, None, benchmark_i32_ceiling_div_assign_mod);
    register_bench!(registry, None, benchmark_i64_ceiling_div_assign_mod);
    register_bench!(registry, None, benchmark_isize_ceiling_div_assign_mod);
}

fn demo_div_mod_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        println!("{}.div_mod({}) = {:?}", x, y, x.div_mod(y));
    }
}

fn demo_div_mod_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds_var_2::<T>(gm).take(limit) {
        println!("({}).div_mod({}) = {:?}", x, y, x.div_mod(y));
    }
}

fn demo_div_assign_mod_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        let old_x = x;
        let r = x.div_assign_mod(y);
        println!("x := {}; x.div_assign_mod({}) = {}; x = {}", old_x, y, r, x);
    }
}

fn demo_div_assign_mod_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y) in pairs_of_signeds_var_2::<T>(gm).take(limit) {
        let old_x = x;
        let r = x.div_assign_mod(y);
        println!("x := {}; x.div_assign_mod({}) = {}; x = {}", old_x, y, r, x);
    }
}

fn demo_div_rem_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        println!("{}.div_rem({}) = {:?}", x, y, x.div_rem(y));
    }
}

fn demo_div_rem_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds_var_2::<T>(gm).take(limit) {
        println!("({}).div_rem({}) = {:?}", x, y, x.div_rem(y));
    }
}

fn demo_div_assign_rem_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        let old_x = x;
        let r = x.div_assign_rem(y);
        println!("x := {}; x.div_assign_rem({}) = {}; x = {}", old_x, y, r, x);
    }
}

fn demo_div_assign_rem_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y) in pairs_of_signeds_var_2::<T>(gm).take(limit) {
        let old_x = x;
        let r = x.div_assign_rem(y);
        println!("x := {}; x.div_assign_rem({}) = {}; x = {}", old_x, y, r, x);
    }
}

fn demo_ceiling_div_neg_mod<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        println!(
            "{}.ceiling_div_neg_mod({}) = {:?}",
            x,
            y,
            x.ceiling_div_neg_mod(y)
        );
    }
}

fn demo_ceiling_div_assign_neg_mod<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        let old_x = x;
        let r = x.ceiling_div_assign_neg_mod(y);
        println!(
            "x := {}; x.ceiling_div_assign_neg_mod({}) = {}; x = {}",
            old_x, y, r, x
        );
    }
}

fn demo_ceiling_div_mod<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds_var_2::<T>(gm).take(limit) {
        println!(
            "({}).ceiling_div_mod({}) = {:?}",
            x,
            y,
            x.ceiling_div_mod(y)
        );
    }
}

fn demo_ceiling_div_assign_mod<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y) in pairs_of_signeds_var_2::<T>(gm).take(limit) {
        let old_x = x;
        let r = x.ceiling_div_assign_mod(y);
        println!(
            "x := {}; x.ceiling_div_assign_mod({}) = {}; x = {}",
            old_x, y, r, x
        );
    }
}

fn benchmark_div_mod_unsigned_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y)))),
            ("using / and %", &mut (|(x, y)| no_out!((x / y, x % y)))),
        ],
    );
}

fn benchmark_div_mod_signed_algorithms<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.div_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y)))),
            (
                "using div_round and mod_op",
                &mut (|(x, y)| no_out!((x.div_round(y, RoundingMode::Floor), x.mod_op(y)))),
            ),
        ],
    );
}

fn benchmark_div_assign_mod_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_assign_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
        )],
    );
}

fn benchmark_div_assign_mod_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.div_assign_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
        )],
    );
}

fn benchmark_div_rem_unsigned_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_rem({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("using div_rem", &mut (|(x, y)| no_out!(x.div_rem(y)))),
            ("using / and %", &mut (|(x, y)| no_out!((x / y, x % y)))),
        ],
    );
}

fn benchmark_div_rem_signed_algorithms<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.div_rem({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("using div_rem", &mut (|(x, y)| no_out!(x.div_rem(y)))),
            ("using / and %", &mut (|(x, y)| no_out!((x / y, x % y)))),
        ],
    );
}

fn benchmark_div_assign_rem_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_assign_rem({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
        )],
    );
}

fn benchmark_div_assign_rem_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.div_assign_rem({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
        )],
    );
}

fn benchmark_ceiling_div_neg_mod_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_div_neg_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "using ceiling_div_neg_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "using div_round and neg_mod",
                &mut (|(x, y)| no_out!((x.div_round(y, RoundingMode::Ceiling), x.neg_mod(y)))),
            ),
        ],
    );
}

fn benchmark_ceiling_div_mod_algorithms<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.ceiling_div_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "using ceiling_div_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "using div_round and ceiling_mod",
                &mut (|(x, y)| no_out!((x.div_round(y, RoundingMode::Ceiling), x.ceiling_mod(y)))),
            ),
        ],
    );
}

fn benchmark_ceiling_div_assign_neg_mod<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_div_assign_neg_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(y))),
        )],
    );
}

fn benchmark_ceiling_div_assign_mod<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.ceiling_div_assign_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_mod(y))),
        )],
    );
}

macro_rules! div_mod_unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_div_mod_unsigned::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_div_assign_mod_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_div_mod_unsigned_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_div_assign_mod_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

div_mod_unsigned!(
    u8,
    demo_u8_div_mod,
    demo_u8_div_assign_mod,
    benchmark_u8_div_mod_algorithms,
    benchmark_u8_div_assign_mod
);
div_mod_unsigned!(
    u16,
    demo_u16_div_mod,
    demo_u16_div_assign_mod,
    benchmark_u16_div_mod_algorithms,
    benchmark_u16_div_assign_mod
);
div_mod_unsigned!(
    u32,
    demo_u32_div_mod,
    demo_u32_div_assign_mod,
    benchmark_u32_div_mod_algorithms,
    benchmark_u32_div_assign_mod
);
div_mod_unsigned!(
    u64,
    demo_u64_div_mod,
    demo_u64_div_assign_mod,
    benchmark_u64_div_mod_algorithms,
    benchmark_u64_div_assign_mod
);
div_mod_unsigned!(
    usize,
    demo_usize_div_mod,
    demo_usize_div_assign_mod,
    benchmark_usize_div_mod_algorithms,
    benchmark_usize_div_assign_mod
);

macro_rules! div_mod_signed {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_div_mod_signed::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_div_assign_mod_signed::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_div_mod_signed_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_div_assign_mod_signed::<$t>(gm, limit, file_name);
        }
    };
}

div_mod_signed!(
    i8,
    demo_i8_div_mod,
    demo_i8_div_assign_mod,
    benchmark_i8_div_mod_algorithms,
    benchmark_i8_div_assign_mod
);
div_mod_signed!(
    i16,
    demo_i16_div_mod,
    demo_i16_div_assign_mod,
    benchmark_i16_div_mod_algorithms,
    benchmark_i16_div_assign_mod
);
div_mod_signed!(
    i32,
    demo_i32_div_mod,
    demo_i32_div_assign_mod,
    benchmark_i32_div_mod_algorithms,
    benchmark_i32_div_assign_mod
);
div_mod_signed!(
    i64,
    demo_i64_div_mod,
    demo_i64_div_assign_mod,
    benchmark_i64_div_mod_algorithms,
    benchmark_i64_div_assign_mod
);
div_mod_signed!(
    isize,
    demo_isize_div_mod,
    demo_isize_div_assign_mod,
    benchmark_isize_div_mod_algorithms,
    benchmark_isize_div_assign_mod
);

macro_rules! div_rem_unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_div_rem_unsigned::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_div_assign_rem_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_div_rem_unsigned_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_div_assign_rem_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

div_rem_unsigned!(
    u8,
    demo_u8_div_rem,
    demo_u8_div_assign_rem,
    benchmark_u8_div_rem_algorithms,
    benchmark_u8_div_assign_rem
);
div_rem_unsigned!(
    u16,
    demo_u16_div_rem,
    demo_u16_div_assign_rem,
    benchmark_u16_div_rem_algorithms,
    benchmark_u16_div_assign_rem
);
div_rem_unsigned!(
    u32,
    demo_u32_div_rem,
    demo_u32_div_assign_rem,
    benchmark_u32_div_rem_algorithms,
    benchmark_u32_div_assign_rem
);
div_rem_unsigned!(
    u64,
    demo_u64_div_rem,
    demo_u64_div_assign_rem,
    benchmark_u64_div_rem_algorithms,
    benchmark_u64_div_assign_rem
);
div_rem_unsigned!(
    usize,
    demo_usize_div_rem,
    demo_usize_div_assign_rem,
    benchmark_usize_div_rem_algorithms,
    benchmark_usize_div_assign_rem
);

macro_rules! div_rem_signed {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_div_rem_signed::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_div_assign_rem_signed::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_div_rem_signed_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_div_assign_rem_signed::<$t>(gm, limit, file_name);
        }
    };
}

div_rem_signed!(
    i8,
    demo_i8_div_rem,
    demo_i8_div_assign_rem,
    benchmark_i8_div_rem_algorithms,
    benchmark_i8_div_assign_rem
);
div_rem_signed!(
    i16,
    demo_i16_div_rem,
    demo_i16_div_assign_rem,
    benchmark_i16_div_rem_algorithms,
    benchmark_i16_div_assign_rem
);
div_rem_signed!(
    i32,
    demo_i32_div_rem,
    demo_i32_div_assign_rem,
    benchmark_i32_div_rem_algorithms,
    benchmark_i32_div_assign_rem
);
div_rem_signed!(
    i64,
    demo_i64_div_rem,
    demo_i64_div_assign_rem,
    benchmark_i64_div_rem_algorithms,
    benchmark_i64_div_assign_rem
);
div_rem_signed!(
    isize,
    demo_isize_div_rem,
    demo_isize_div_assign_rem,
    benchmark_isize_div_rem_algorithms,
    benchmark_isize_div_assign_rem
);

macro_rules! ceiling_div_neg_mod {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_ceiling_div_neg_mod::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_ceiling_div_assign_neg_mod::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_div_neg_mod_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_div_assign_neg_mod::<$t>(gm, limit, file_name);
        }
    };
}

ceiling_div_neg_mod!(
    u8,
    demo_u8_ceiling_div_neg_mod,
    demo_u8_ceiling_div_assign_neg_mod,
    benchmark_u8_ceiling_div_neg_mod_algorithms,
    benchmark_u8_ceiling_div_assign_neg_mod
);
ceiling_div_neg_mod!(
    u16,
    demo_u16_ceiling_div_neg_mod,
    demo_u16_ceiling_div_assign_neg_mod,
    benchmark_u16_ceiling_div_neg_mod_algorithms,
    benchmark_u16_ceiling_div_assign_neg_mod
);
ceiling_div_neg_mod!(
    u32,
    demo_u32_ceiling_div_neg_mod,
    demo_u32_ceiling_div_assign_neg_mod,
    benchmark_u32_ceiling_div_neg_mod_algorithms,
    benchmark_u32_ceiling_div_assign_neg_mod
);
ceiling_div_neg_mod!(
    u64,
    demo_u64_ceiling_div_neg_mod,
    demo_u64_ceiling_div_assign_neg_mod,
    benchmark_u64_ceiling_div_neg_mod_algorithms,
    benchmark_u64_ceiling_div_assign_neg_mod
);
ceiling_div_neg_mod!(
    usize,
    demo_usize_ceiling_div_neg_mod,
    demo_usize_ceiling_div_assign_neg_mod,
    benchmark_usize_ceiling_div_neg_mod_algorithms,
    benchmark_usize_ceiling_div_assign_neg_mod
);

macro_rules! ceiling_div_mod {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_ceiling_div_mod::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_ceiling_div_assign_mod::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_div_mod_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_div_assign_mod::<$t>(gm, limit, file_name);
        }
    };
}

ceiling_div_mod!(
    i8,
    demo_i8_ceiling_div_mod,
    demo_i8_ceiling_div_assign_mod,
    benchmark_i8_ceiling_div_mod_algorithms,
    benchmark_i8_ceiling_div_assign_mod
);
ceiling_div_mod!(
    i16,
    demo_i16_ceiling_div_mod,
    demo_i16_ceiling_div_assign_mod,
    benchmark_i16_ceiling_div_mod_algorithms,
    benchmark_i16_ceiling_div_assign_mod
);
ceiling_div_mod!(
    i32,
    demo_i32_ceiling_div_mod,
    demo_i32_ceiling_div_assign_mod,
    benchmark_i32_ceiling_div_mod_algorithms,
    benchmark_i32_ceiling_div_assign_mod
);
ceiling_div_mod!(
    i64,
    demo_i64_ceiling_div_mod,
    demo_i64_ceiling_div_assign_mod,
    benchmark_i64_ceiling_div_mod_algorithms,
    benchmark_i64_ceiling_div_assign_mod
);
ceiling_div_mod!(
    isize,
    demo_isize_ceiling_div_mod,
    demo_isize_ceiling_div_assign_mod,
    benchmark_isize_ceiling_div_mod_algorithms,
    benchmark_isize_ceiling_div_assign_mod
);
