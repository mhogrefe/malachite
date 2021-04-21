use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_u64_var_2, pairs_of_signed_and_small_u64_var_3,
    pairs_of_signed_and_small_u64_var_4, pairs_of_signed_and_small_unsigned,
    pairs_of_unsigned_and_small_u64_var_4, pairs_of_unsigned_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod_power_of_2);
    register_demo!(registry, demo_u16_mod_power_of_2);
    register_demo!(registry, demo_u32_mod_power_of_2);
    register_demo!(registry, demo_u64_mod_power_of_2);
    register_demo!(registry, demo_usize_mod_power_of_2);
    register_demo!(registry, demo_i8_mod_power_of_2);
    register_demo!(registry, demo_i16_mod_power_of_2);
    register_demo!(registry, demo_i32_mod_power_of_2);
    register_demo!(registry, demo_i64_mod_power_of_2);
    register_demo!(registry, demo_isize_mod_power_of_2);

    register_demo!(registry, demo_u8_mod_power_of_2_assign);
    register_demo!(registry, demo_u16_mod_power_of_2_assign);
    register_demo!(registry, demo_u32_mod_power_of_2_assign);
    register_demo!(registry, demo_u64_mod_power_of_2_assign);
    register_demo!(registry, demo_usize_mod_power_of_2_assign);
    register_demo!(registry, demo_i8_mod_power_of_2_assign);
    register_demo!(registry, demo_i16_mod_power_of_2_assign);
    register_demo!(registry, demo_i32_mod_power_of_2_assign);
    register_demo!(registry, demo_i64_mod_power_of_2_assign);
    register_demo!(registry, demo_isize_mod_power_of_2_assign);

    register_demo!(registry, demo_u8_rem_power_of_2);
    register_demo!(registry, demo_u16_rem_power_of_2);
    register_demo!(registry, demo_u32_rem_power_of_2);
    register_demo!(registry, demo_u64_rem_power_of_2);
    register_demo!(registry, demo_usize_rem_power_of_2);
    register_demo!(registry, demo_i8_rem_power_of_2);
    register_demo!(registry, demo_i16_rem_power_of_2);
    register_demo!(registry, demo_i32_rem_power_of_2);
    register_demo!(registry, demo_i64_rem_power_of_2);
    register_demo!(registry, demo_isize_rem_power_of_2);

    register_demo!(registry, demo_u8_rem_power_of_2_assign);
    register_demo!(registry, demo_u16_rem_power_of_2_assign);
    register_demo!(registry, demo_u32_rem_power_of_2_assign);
    register_demo!(registry, demo_u64_rem_power_of_2_assign);
    register_demo!(registry, demo_usize_rem_power_of_2_assign);
    register_demo!(registry, demo_i8_rem_power_of_2_assign);
    register_demo!(registry, demo_i16_rem_power_of_2_assign);
    register_demo!(registry, demo_i32_rem_power_of_2_assign);
    register_demo!(registry, demo_i64_rem_power_of_2_assign);
    register_demo!(registry, demo_isize_rem_power_of_2_assign);

    register_demo!(registry, demo_u8_neg_mod_power_of_2);
    register_demo!(registry, demo_u16_neg_mod_power_of_2);
    register_demo!(registry, demo_u32_neg_mod_power_of_2);
    register_demo!(registry, demo_u64_neg_mod_power_of_2);
    register_demo!(registry, demo_usize_neg_mod_power_of_2);

    register_demo!(registry, demo_u8_neg_mod_power_of_2_assign);
    register_demo!(registry, demo_u16_neg_mod_power_of_2_assign);
    register_demo!(registry, demo_u32_neg_mod_power_of_2_assign);
    register_demo!(registry, demo_u64_neg_mod_power_of_2_assign);
    register_demo!(registry, demo_usize_neg_mod_power_of_2_assign);

    register_demo!(registry, demo_i8_ceiling_mod_power_of_2);
    register_demo!(registry, demo_i16_ceiling_mod_power_of_2);
    register_demo!(registry, demo_i32_ceiling_mod_power_of_2);
    register_demo!(registry, demo_i64_ceiling_mod_power_of_2);
    register_demo!(registry, demo_isize_ceiling_mod_power_of_2);

    register_demo!(registry, demo_i8_ceiling_mod_power_of_2_assign);
    register_demo!(registry, demo_i16_ceiling_mod_power_of_2_assign);
    register_demo!(registry, demo_i32_ceiling_mod_power_of_2_assign);
    register_demo!(registry, demo_i64_ceiling_mod_power_of_2_assign);
    register_demo!(registry, demo_isize_ceiling_mod_power_of_2_assign);

    register_bench!(registry, None, benchmark_u8_mod_power_of_2);
    register_bench!(registry, None, benchmark_u16_mod_power_of_2);
    register_bench!(registry, None, benchmark_u32_mod_power_of_2);
    register_bench!(registry, None, benchmark_u64_mod_power_of_2);
    register_bench!(registry, None, benchmark_usize_mod_power_of_2);
    register_bench!(registry, None, benchmark_i8_mod_power_of_2);
    register_bench!(registry, None, benchmark_i16_mod_power_of_2);
    register_bench!(registry, None, benchmark_i32_mod_power_of_2);
    register_bench!(registry, None, benchmark_i64_mod_power_of_2);
    register_bench!(registry, None, benchmark_isize_mod_power_of_2);

    register_bench!(registry, None, benchmark_u8_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_u16_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_u32_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_u64_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_usize_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_i8_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_i16_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_i32_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_i64_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_isize_mod_power_of_2_assign);

    register_bench!(registry, None, benchmark_u8_rem_power_of_2);
    register_bench!(registry, None, benchmark_u16_rem_power_of_2);
    register_bench!(registry, None, benchmark_u32_rem_power_of_2);
    register_bench!(registry, None, benchmark_u64_rem_power_of_2);
    register_bench!(registry, None, benchmark_usize_rem_power_of_2);
    register_bench!(registry, None, benchmark_i8_rem_power_of_2);
    register_bench!(registry, None, benchmark_i16_rem_power_of_2);
    register_bench!(registry, None, benchmark_i32_rem_power_of_2);
    register_bench!(registry, None, benchmark_i64_rem_power_of_2);
    register_bench!(registry, None, benchmark_isize_rem_power_of_2);

    register_bench!(registry, None, benchmark_u8_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_u16_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_u32_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_u64_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_usize_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_i8_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_i16_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_i32_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_i64_rem_power_of_2_assign);
    register_bench!(registry, None, benchmark_isize_rem_power_of_2_assign);

    register_bench!(registry, None, benchmark_u8_neg_mod_power_of_2);
    register_bench!(registry, None, benchmark_u16_neg_mod_power_of_2);
    register_bench!(registry, None, benchmark_u32_neg_mod_power_of_2);
    register_bench!(registry, None, benchmark_u64_neg_mod_power_of_2);
    register_bench!(registry, None, benchmark_usize_neg_mod_power_of_2);

    register_bench!(registry, None, benchmark_u8_neg_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_u16_neg_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_u32_neg_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_u64_neg_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_usize_neg_mod_power_of_2_assign);

    register_bench!(registry, None, benchmark_i8_ceiling_mod_power_of_2);
    register_bench!(registry, None, benchmark_i16_ceiling_mod_power_of_2);
    register_bench!(registry, None, benchmark_i32_ceiling_mod_power_of_2);
    register_bench!(registry, None, benchmark_i64_ceiling_mod_power_of_2);
    register_bench!(registry, None, benchmark_isize_ceiling_mod_power_of_2);

    register_bench!(registry, None, benchmark_i8_ceiling_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_i16_ceiling_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_i32_ceiling_mod_power_of_2_assign);
    register_bench!(registry, None, benchmark_i64_ceiling_mod_power_of_2_assign);
    register_bench!(
        registry,
        None,
        benchmark_isize_ceiling_mod_power_of_2_assign
    );
}

fn demo_mod_power_of_2_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (n, pow) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!("{} === {} mod 2^{}", n, n.mod_power_of_2(pow), pow);
    }
}

fn demo_mod_power_of_2_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as ModPowerOf2>::Output: PrimitiveUnsigned,
{
    for (n, pow) in pairs_of_signed_and_small_u64_var_2::<T>(gm).take(limit) {
        println!("{} === {} mod 2^{}", n, n.mod_power_of_2(pow), pow);
    }
}

fn demo_mod_power_of_2_assign_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut n, pow) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        let old_n = n;
        n.mod_power_of_2_assign(pow);
        println!(
            "n := {}; n.mod_power_of_2_assign({}); n = {}",
            old_n, pow, n
        );
    }
}

fn demo_mod_power_of_2_assign_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, pow) in pairs_of_signed_and_small_u64_var_3::<T>(gm).take(limit) {
        let old_n = n;
        n.mod_power_of_2_assign(pow);
        println!(
            "n := {}; n.mod_power_of_2_assign({}); n = {}",
            old_n, pow, n
        );
    }
}

fn demo_rem_power_of_2_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (n, pow) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!("{}.rem_power_of_2({}) = {}", n, pow, n.rem_power_of_2(pow));
    }
}

fn demo_rem_power_of_2_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, pow) in pairs_of_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!(
            "({}).rem_power_of_2({}) = {}",
            n,
            pow,
            n.rem_power_of_2(pow)
        );
    }
}

fn demo_rem_power_of_2_assign_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut n, pow) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        let old_n = n;
        n.rem_power_of_2_assign(pow);
        println!(
            "n := {}; n.rem_power_of_2_assign({}); n = {}",
            old_n, pow, n
        );
    }
}

fn demo_rem_power_of_2_assign_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, pow) in pairs_of_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        let old_n = n;
        n.rem_power_of_2_assign(pow);
        println!(
            "n := {}; n.rem_power_of_2_assign({}); n = {}",
            old_n, pow, n
        );
    }
}

fn demo_neg_mod_power_of_2<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (n, pow) in pairs_of_unsigned_and_small_u64_var_4::<T>(gm).take(limit) {
        println!(
            "{}.neg_mod_power_of_2({}) = {}",
            n,
            pow,
            n.neg_mod_power_of_2(pow)
        );
    }
}

fn demo_neg_mod_power_of_2_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut n, pow) in pairs_of_unsigned_and_small_u64_var_4::<T>(gm).take(limit) {
        let old_n = n;
        n.neg_mod_power_of_2_assign(pow);
        println!(
            "n := {}; n.neg_mod_power_of_2_assign({}); n = {}",
            old_n, pow, n
        );
    }
}

fn demo_ceiling_mod_power_of_2<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (n, pow) in pairs_of_signed_and_small_u64_var_4::<T>(gm).take(limit) {
        println!(
            "({}).ceiling_mod_power_of_2({}) = {}",
            n,
            pow,
            n.ceiling_mod_power_of_2(pow)
        );
    }
}

fn demo_ceiling_mod_power_of_2_assign<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, pow) in pairs_of_signed_and_small_u64_var_4::<T>(gm).take(limit) {
        let old_n = n;
        n.ceiling_mod_power_of_2_assign(pow);
        println!(
            "n := {}; n.ceiling_mod_power_of_2_assign({}); n = {}",
            old_n, pow, n
        );
    }
}

fn benchmark_mod_power_of_2_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow)| no_out!(n.mod_power_of_2(pow))),
        )],
    );
}

fn benchmark_mod_power_of_2_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as ModPowerOf2>::Output: PrimitiveUnsigned,
{
    run_benchmark_old(
        &format!("{}.mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_u64_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow)| no_out!(n.mod_power_of_2(pow))),
        )],
    );
}

fn benchmark_mod_power_of_2_assign_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.mod_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow)| n.mod_power_of_2_assign(pow)),
        )],
    );
}

fn benchmark_mod_power_of_2_assign_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.mod_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_u64_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow)| n.mod_power_of_2_assign(pow)),
        )],
    );
}

fn benchmark_rem_power_of_2_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.rem_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow)| no_out!(n.rem_power_of_2(pow))),
        )],
    );
}

fn benchmark_rem_power_of_2_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.rem_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow)| no_out!(n.rem_power_of_2(pow))),
        )],
    );
}

fn benchmark_rem_power_of_2_assign_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.rem_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow)| n.rem_power_of_2_assign(pow)),
        )],
    );
}

fn benchmark_rem_power_of_2_assign_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.rem_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow)| n.rem_power_of_2_assign(pow)),
        )],
    );
}

fn benchmark_neg_mod_power_of_2<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.neg_mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_u64_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow)| no_out!(n.neg_mod_power_of_2(pow))),
        )],
    );
}

fn benchmark_neg_mod_power_of_2_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.neg_mod_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_u64_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow)| n.neg_mod_power_of_2_assign(pow)),
        )],
    );
}

fn benchmark_ceiling_mod_power_of_2<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.ceiling_mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_u64_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(n, pow)| no_out!(n.ceiling_mod_power_of_2(pow))),
        )],
    );
}

fn benchmark_ceiling_mod_power_of_2_assign<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.ceiling_mod_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_u64_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|(mut n, pow)| n.ceiling_mod_power_of_2_assign(pow)),
        )],
    );
}

macro_rules! mod_power_of_2_unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_power_of_2_unsigned::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_power_of_2_assign_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_2_unsigned::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_2_assign_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

mod_power_of_2_unsigned!(
    u8,
    demo_u8_mod_power_of_2,
    demo_u8_mod_power_of_2_assign,
    benchmark_u8_mod_power_of_2,
    benchmark_u8_mod_power_of_2_assign
);
mod_power_of_2_unsigned!(
    u16,
    demo_u16_mod_power_of_2,
    demo_u16_mod_power_of_2_assign,
    benchmark_u16_mod_power_of_2,
    benchmark_u16_mod_power_of_2_assign
);
mod_power_of_2_unsigned!(
    u32,
    demo_u32_mod_power_of_2,
    demo_u32_mod_power_of_2_assign,
    benchmark_u32_mod_power_of_2,
    benchmark_u32_mod_power_of_2_assign
);
mod_power_of_2_unsigned!(
    u64,
    demo_u64_mod_power_of_2,
    demo_u64_mod_power_of_2_assign,
    benchmark_u64_mod_power_of_2,
    benchmark_u64_mod_power_of_2_assign
);
mod_power_of_2_unsigned!(
    usize,
    demo_usize_mod_power_of_2,
    demo_usize_mod_power_of_2_assign,
    benchmark_usize_mod_power_of_2,
    benchmark_usize_mod_power_of_2_assign
);

macro_rules! mod_power_of_2_signed {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_power_of_2_signed::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_power_of_2_assign_signed::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_2_signed::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_power_of_2_assign_signed::<$t>(gm, limit, file_name);
        }
    };
}

mod_power_of_2_signed!(
    i8,
    demo_i8_mod_power_of_2,
    demo_i8_mod_power_of_2_assign,
    benchmark_i8_mod_power_of_2,
    benchmark_i8_mod_power_of_2_assign
);
mod_power_of_2_signed!(
    i16,
    demo_i16_mod_power_of_2,
    demo_i16_mod_power_of_2_assign,
    benchmark_i16_mod_power_of_2,
    benchmark_i16_mod_power_of_2_assign
);
mod_power_of_2_signed!(
    i32,
    demo_i32_mod_power_of_2,
    demo_i32_mod_power_of_2_assign,
    benchmark_i32_mod_power_of_2,
    benchmark_i32_mod_power_of_2_assign
);
mod_power_of_2_signed!(
    i64,
    demo_i64_mod_power_of_2,
    demo_i64_mod_power_of_2_assign,
    benchmark_i64_mod_power_of_2,
    benchmark_i64_mod_power_of_2_assign
);
mod_power_of_2_signed!(
    isize,
    demo_isize_mod_power_of_2,
    demo_isize_mod_power_of_2_assign,
    benchmark_isize_mod_power_of_2,
    benchmark_isize_mod_power_of_2_assign
);

macro_rules! rem_power_of_2_unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_rem_power_of_2_unsigned::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_rem_power_of_2_assign_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_rem_power_of_2_unsigned::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_rem_power_of_2_assign_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

rem_power_of_2_unsigned!(
    u8,
    demo_u8_rem_power_of_2,
    demo_u8_rem_power_of_2_assign,
    benchmark_u8_rem_power_of_2,
    benchmark_u8_rem_power_of_2_assign
);
rem_power_of_2_unsigned!(
    u16,
    demo_u16_rem_power_of_2,
    demo_u16_rem_power_of_2_assign,
    benchmark_u16_rem_power_of_2,
    benchmark_u16_rem_power_of_2_assign
);
rem_power_of_2_unsigned!(
    u32,
    demo_u32_rem_power_of_2,
    demo_u32_rem_power_of_2_assign,
    benchmark_u32_rem_power_of_2,
    benchmark_u32_rem_power_of_2_assign
);
rem_power_of_2_unsigned!(
    u64,
    demo_u64_rem_power_of_2,
    demo_u64_rem_power_of_2_assign,
    benchmark_u64_rem_power_of_2,
    benchmark_u64_rem_power_of_2_assign
);
rem_power_of_2_unsigned!(
    usize,
    demo_usize_rem_power_of_2,
    demo_usize_rem_power_of_2_assign,
    benchmark_usize_rem_power_of_2,
    benchmark_usize_rem_power_of_2_assign
);

macro_rules! rem_power_of_2_signed {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_rem_power_of_2_signed::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_rem_power_of_2_assign_signed::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_rem_power_of_2_signed::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_rem_power_of_2_assign_signed::<$t>(gm, limit, file_name);
        }
    };
}

rem_power_of_2_signed!(
    i8,
    demo_i8_rem_power_of_2,
    demo_i8_rem_power_of_2_assign,
    benchmark_i8_rem_power_of_2,
    benchmark_i8_rem_power_of_2_assign
);
rem_power_of_2_signed!(
    i16,
    demo_i16_rem_power_of_2,
    demo_i16_rem_power_of_2_assign,
    benchmark_i16_rem_power_of_2,
    benchmark_i16_rem_power_of_2_assign
);
rem_power_of_2_signed!(
    i32,
    demo_i32_rem_power_of_2,
    demo_i32_rem_power_of_2_assign,
    benchmark_i32_rem_power_of_2,
    benchmark_i32_rem_power_of_2_assign
);
rem_power_of_2_signed!(
    i64,
    demo_i64_rem_power_of_2,
    demo_i64_rem_power_of_2_assign,
    benchmark_i64_rem_power_of_2,
    benchmark_i64_rem_power_of_2_assign
);
rem_power_of_2_signed!(
    isize,
    demo_isize_rem_power_of_2,
    demo_isize_rem_power_of_2_assign,
    benchmark_isize_rem_power_of_2,
    benchmark_isize_rem_power_of_2_assign
);

macro_rules! neg_mod_power_of_2 {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_neg_mod_power_of_2::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_neg_mod_power_of_2_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_neg_mod_power_of_2::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_neg_mod_power_of_2_assign::<$t>(gm, limit, file_name);
        }
    };
}

neg_mod_power_of_2!(
    u8,
    demo_u8_neg_mod_power_of_2,
    demo_u8_neg_mod_power_of_2_assign,
    benchmark_u8_neg_mod_power_of_2,
    benchmark_u8_neg_mod_power_of_2_assign
);
neg_mod_power_of_2!(
    u16,
    demo_u16_neg_mod_power_of_2,
    demo_u16_neg_mod_power_of_2_assign,
    benchmark_u16_neg_mod_power_of_2,
    benchmark_u16_neg_mod_power_of_2_assign
);
neg_mod_power_of_2!(
    u32,
    demo_u32_neg_mod_power_of_2,
    demo_u32_neg_mod_power_of_2_assign,
    benchmark_u32_neg_mod_power_of_2,
    benchmark_u32_neg_mod_power_of_2_assign
);
neg_mod_power_of_2!(
    u64,
    demo_u64_neg_mod_power_of_2,
    demo_u64_neg_mod_power_of_2_assign,
    benchmark_u64_neg_mod_power_of_2,
    benchmark_u64_neg_mod_power_of_2_assign
);
neg_mod_power_of_2!(
    usize,
    demo_usize_neg_mod_power_of_2,
    demo_usize_neg_mod_power_of_2_assign,
    benchmark_usize_neg_mod_power_of_2,
    benchmark_usize_neg_mod_power_of_2_assign
);

macro_rules! ceiling_mod_power_of_2 {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_ceiling_mod_power_of_2::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_ceiling_mod_power_of_2_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_mod_power_of_2::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_mod_power_of_2_assign::<$t>(gm, limit, file_name);
        }
    };
}

ceiling_mod_power_of_2!(
    i8,
    demo_i8_ceiling_mod_power_of_2,
    demo_i8_ceiling_mod_power_of_2_assign,
    benchmark_i8_ceiling_mod_power_of_2,
    benchmark_i8_ceiling_mod_power_of_2_assign
);
ceiling_mod_power_of_2!(
    i16,
    demo_i16_ceiling_mod_power_of_2,
    demo_i16_ceiling_mod_power_of_2_assign,
    benchmark_i16_ceiling_mod_power_of_2,
    benchmark_i16_ceiling_mod_power_of_2_assign
);
ceiling_mod_power_of_2!(
    i32,
    demo_i32_ceiling_mod_power_of_2,
    demo_i32_ceiling_mod_power_of_2_assign,
    benchmark_i32_ceiling_mod_power_of_2,
    benchmark_i32_ceiling_mod_power_of_2_assign
);
ceiling_mod_power_of_2!(
    i64,
    demo_i64_ceiling_mod_power_of_2,
    demo_i64_ceiling_mod_power_of_2_assign,
    benchmark_i64_ceiling_mod_power_of_2,
    benchmark_i64_ceiling_mod_power_of_2_assign
);
ceiling_mod_power_of_2!(
    isize,
    demo_isize_ceiling_mod_power_of_2,
    demo_isize_ceiling_mod_power_of_2_assign,
    benchmark_isize_ceiling_mod_power_of_2,
    benchmark_isize_ceiling_mod_power_of_2_assign
);
