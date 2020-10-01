use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{PowerOfTwoDigitIterable, PowerOfTwoDigits};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{
    DemoBenchRegistry, GenerationMode, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    pairs_of_u64_and_unsigned_vec_var_1, pairs_of_u64_and_unsigned_vec_var_2,
    pairs_of_unsigned_and_small_u64_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_to_power_of_two_digits_asc_u8);
    register_demo!(registry, demo_u8_to_power_of_two_digits_asc_u16);
    register_demo!(registry, demo_u8_to_power_of_two_digits_asc_u32);
    register_demo!(registry, demo_u8_to_power_of_two_digits_asc_u64);
    register_demo!(registry, demo_u8_to_power_of_two_digits_asc_usize);
    register_demo!(registry, demo_u16_to_power_of_two_digits_asc_u8);
    register_demo!(registry, demo_u16_to_power_of_two_digits_asc_u16);
    register_demo!(registry, demo_u16_to_power_of_two_digits_asc_u32);
    register_demo!(registry, demo_u16_to_power_of_two_digits_asc_u64);
    register_demo!(registry, demo_u16_to_power_of_two_digits_asc_usize);
    register_demo!(registry, demo_u32_to_power_of_two_digits_asc_u8);
    register_demo!(registry, demo_u32_to_power_of_two_digits_asc_u16);
    register_demo!(registry, demo_u32_to_power_of_two_digits_asc_u32);
    register_demo!(registry, demo_u32_to_power_of_two_digits_asc_u64);
    register_demo!(registry, demo_u32_to_power_of_two_digits_asc_usize);
    register_demo!(registry, demo_u64_to_power_of_two_digits_asc_u8);
    register_demo!(registry, demo_u64_to_power_of_two_digits_asc_u16);
    register_demo!(registry, demo_u64_to_power_of_two_digits_asc_u32);
    register_demo!(registry, demo_u64_to_power_of_two_digits_asc_u64);
    register_demo!(registry, demo_u64_to_power_of_two_digits_asc_usize);
    register_demo!(registry, demo_usize_to_power_of_two_digits_asc_u8);
    register_demo!(registry, demo_usize_to_power_of_two_digits_asc_u16);
    register_demo!(registry, demo_usize_to_power_of_two_digits_asc_u32);
    register_demo!(registry, demo_usize_to_power_of_two_digits_asc_u64);
    register_demo!(registry, demo_usize_to_power_of_two_digits_asc_usize);

    register_demo!(registry, demo_u8_to_power_of_two_digits_desc_u8);
    register_demo!(registry, demo_u8_to_power_of_two_digits_desc_u16);
    register_demo!(registry, demo_u8_to_power_of_two_digits_desc_u32);
    register_demo!(registry, demo_u8_to_power_of_two_digits_desc_u64);
    register_demo!(registry, demo_u8_to_power_of_two_digits_desc_usize);
    register_demo!(registry, demo_u16_to_power_of_two_digits_desc_u8);
    register_demo!(registry, demo_u16_to_power_of_two_digits_desc_u16);
    register_demo!(registry, demo_u16_to_power_of_two_digits_desc_u32);
    register_demo!(registry, demo_u16_to_power_of_two_digits_desc_u64);
    register_demo!(registry, demo_u16_to_power_of_two_digits_desc_usize);
    register_demo!(registry, demo_u32_to_power_of_two_digits_desc_u8);
    register_demo!(registry, demo_u32_to_power_of_two_digits_desc_u16);
    register_demo!(registry, demo_u32_to_power_of_two_digits_desc_u32);
    register_demo!(registry, demo_u32_to_power_of_two_digits_desc_u64);
    register_demo!(registry, demo_u32_to_power_of_two_digits_desc_usize);
    register_demo!(registry, demo_u64_to_power_of_two_digits_desc_u8);
    register_demo!(registry, demo_u64_to_power_of_two_digits_desc_u16);
    register_demo!(registry, demo_u64_to_power_of_two_digits_desc_u32);
    register_demo!(registry, demo_u64_to_power_of_two_digits_desc_u64);
    register_demo!(registry, demo_u64_to_power_of_two_digits_desc_usize);
    register_demo!(registry, demo_usize_to_power_of_two_digits_desc_u8);
    register_demo!(registry, demo_usize_to_power_of_two_digits_desc_u16);
    register_demo!(registry, demo_usize_to_power_of_two_digits_desc_u32);
    register_demo!(registry, demo_usize_to_power_of_two_digits_desc_u64);
    register_demo!(registry, demo_usize_to_power_of_two_digits_desc_usize);

    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_asc_u8);
    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_asc_u16);
    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_asc_u32);
    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_asc_u64);
    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_asc_usize);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_asc_u8);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_asc_u16);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_asc_u32);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_asc_u64);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_asc_usize);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_asc_u8);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_asc_u16);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_asc_u32);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_asc_u64);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_asc_usize);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_asc_u8);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_asc_u16);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_asc_u32);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_asc_u64);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_asc_usize);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_asc_u8);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_asc_u16);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_asc_u32);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_asc_u64);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_asc_usize);

    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_desc_u8);
    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_desc_u16);
    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_desc_u32);
    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_desc_u64);
    register_ns_demo!(registry, demo_u8_from_power_of_two_digits_desc_usize);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_desc_u8);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_desc_u16);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_desc_u32);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_desc_u64);
    register_ns_demo!(registry, demo_u16_from_power_of_two_digits_desc_usize);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_desc_u8);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_desc_u16);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_desc_u32);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_desc_u64);
    register_ns_demo!(registry, demo_u32_from_power_of_two_digits_desc_usize);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_desc_u8);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_desc_u16);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_desc_u32);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_desc_u64);
    register_ns_demo!(registry, demo_u64_from_power_of_two_digits_desc_usize);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_desc_u8);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_desc_u16);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_desc_u32);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_desc_u64);
    register_ns_demo!(registry, demo_usize_from_power_of_two_digits_desc_usize);

    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_asc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_asc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_asc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_asc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_asc_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_asc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_asc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_asc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_asc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_asc_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_asc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_asc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_asc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_asc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_asc_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_asc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_asc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_asc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_asc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_asc_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_asc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_asc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_asc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_asc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_asc_usize_evaluation_strategy
    );

    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_desc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_desc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_desc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_desc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u8_to_power_of_two_digits_desc_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_desc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_desc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_desc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_desc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u16_to_power_of_two_digits_desc_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_desc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_desc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_desc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_desc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u32_to_power_of_two_digits_desc_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_desc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_desc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_desc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_desc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_u64_to_power_of_two_digits_desc_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_desc_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_desc_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_desc_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_desc_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        None,
        benchmark_usize_to_power_of_two_digits_desc_usize_evaluation_strategy
    );

    register_ns_bench!(registry, None, benchmark_u8_from_power_of_two_digits_asc_u8);
    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_asc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_asc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_asc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_asc_usize
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_asc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_asc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_asc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_asc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_asc_usize
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_asc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_asc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_asc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_asc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_asc_usize
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_asc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_asc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_asc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_asc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_asc_usize
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_asc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_asc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_asc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_asc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_asc_usize
    );

    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_desc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_desc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_desc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_desc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u8_from_power_of_two_digits_desc_usize
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_desc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_desc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_desc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_desc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u16_from_power_of_two_digits_desc_usize
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_desc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_desc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_desc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_desc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u32_from_power_of_two_digits_desc_usize
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_desc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_desc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_desc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_desc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_u64_from_power_of_two_digits_desc_usize
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_desc_u8
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_desc_u16
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_desc_u32
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_desc_u64
    );
    register_ns_bench!(
        registry,
        None,
        benchmark_usize_from_power_of_two_digits_desc_usize
    );
}

fn demo_to_power_of_two_digits_asc<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) where
    T: PowerOfTwoDigits<U>,
{
    for (n, log_base) in pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm).take(limit) {
        println!(
            "{}.to_power_of_two_digits_asc({}) = {:?}",
            n,
            log_base,
            PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&n, log_base)
        );
    }
}

fn demo_to_power_of_two_digits_desc<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) where
    T: PowerOfTwoDigits<U>,
{
    for (n, log_base) in pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm).take(limit) {
        println!(
            "{}.to_power_of_two_digits_desc({}) = {:?}",
            n,
            log_base,
            PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&n, log_base)
        );
    }
}

fn demo_from_power_of_two_digits_asc<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) where
    T: PowerOfTwoDigits<U>,
{
    for (log_base, digits) in pairs_of_u64_and_unsigned_vec_var_1::<T, U>(gm).take(limit) {
        println!(
            "{}::from_power_of_two_digits_asc({}, {:?}) = {}",
            T::NAME,
            log_base,
            digits,
            T::from_power_of_two_digits_asc(log_base, &digits)
        );
    }
}

fn demo_from_power_of_two_digits_desc<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) where
    T: PowerOfTwoDigits<U>,
{
    for (log_base, digits) in pairs_of_u64_and_unsigned_vec_var_2::<T, U>(gm).take(limit) {
        println!(
            "{}::from_power_of_two_digits_desc({}, {:?}) = {}",
            T::NAME,
            log_base,
            digits,
            T::from_power_of_two_digits_desc(log_base, &digits)
        );
    }
}

fn benchmark_to_power_of_two_digits_asc_evaluation_strategy<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: PowerOfTwoDigits<U>,
    T: PowerOfTwoDigitIterable<U>,
{
    run_benchmark_old(
        &format!(
            "PowerOfTwoDigits::<{}>::to_power_of_two_digits_asc(&{}, u64)",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Malachite",
                &mut (|(n, log_base)| {
                    no_out!(PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(
                        &n, log_base
                    ))
                }),
            ),
            (
                &format!(
                    "{}.power_of_two_digits(u64).collect::<Vec<{}>>()",
                    T::NAME,
                    U::NAME
                ),
                &mut (|(n, log_base)| {
                    no_out!(
                        PowerOfTwoDigitIterable::<U>::power_of_two_digits(n, log_base)
                            .collect::<Vec<U>>()
                    )
                }),
            ),
        ],
    );
}

fn benchmark_to_power_of_two_digits_desc_evaluation_strategy<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: PowerOfTwoDigits<U>,
    T: PowerOfTwoDigitIterable<U>,
{
    run_benchmark_old(
        &format!(
            "PowerOfTwoDigits::<{}>::to_power_of_two_digits_desc(&{}, u64)",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Malachite",
                &mut (|(n, log_base)| {
                    no_out!(PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(
                        &n, log_base
                    ))
                }),
            ),
            (
                &format!(
                    "{}.power_of_two_digits(u64).rev().collect::<Vec<{}>>()",
                    T::NAME,
                    U::NAME
                ),
                &mut (|(n, log_base)| {
                    no_out!(
                        PowerOfTwoDigitIterable::<U>::power_of_two_digits(n, log_base)
                            .rev()
                            .collect::<Vec<U>>()
                    )
                }),
            ),
        ],
    );
}

fn benchmark_from_power_of_two_digits_asc<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: PowerOfTwoDigits<U>,
{
    run_benchmark_old(
        &format!(
            "{}::from_power_of_two_digits_asc(u64, &[{}])",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::Single,
        pairs_of_u64_and_unsigned_vec_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, digits)| digits.len()),
        "bits.len()",
        &mut [(
            "Malachite",
            &mut (|(log_base, ref digits)| {
                no_out!(T::from_power_of_two_digits_asc(log_base, digits))
            }),
        )],
    );
}

fn benchmark_from_power_of_two_digits_desc<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: PowerOfTwoDigits<U>,
{
    run_benchmark_old(
        &format!(
            "{}::from_power_of_two_digits_desc(u64, &[{}])",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::Single,
        pairs_of_u64_and_unsigned_vec_var_2::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, digits)| digits.len()),
        "bits.len()",
        &mut [(
            "Malachite",
            &mut (|(log_base, ref digits)| {
                no_out!(T::from_power_of_two_digits_desc(log_base, digits))
            }),
        )],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $u:ident,
        $to_power_of_two_digits_asc_demo_name:ident,
        $to_power_of_two_digits_desc_demo_name:ident,
        $from_power_of_two_digits_asc_demo_name:ident,
        $from_power_of_two_digits_desc_demo_name:ident,
        $to_power_of_two_digits_asc_bench_name:ident,
        $to_power_of_two_digits_desc_bench_name:ident,
        $from_power_of_two_digits_asc_bench_name:ident,
        $from_power_of_two_digits_desc_bench_name:ident
    ) => {
        fn $to_power_of_two_digits_asc_demo_name(gm: GenerationMode, limit: usize) {
            demo_to_power_of_two_digits_asc::<$t, $u>(gm, limit);
        }

        fn $to_power_of_two_digits_desc_demo_name(gm: GenerationMode, limit: usize) {
            demo_to_power_of_two_digits_desc::<$t, $u>(gm, limit);
        }

        fn $from_power_of_two_digits_asc_demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_from_power_of_two_digits_asc::<$t, $u>(gm, limit);
        }

        fn $from_power_of_two_digits_desc_demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_from_power_of_two_digits_desc::<$t, $u>(gm, limit);
        }

        fn $to_power_of_two_digits_asc_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_to_power_of_two_digits_asc_evaluation_strategy::<$t, $u>(
                gm, limit, file_name,
            );
        }

        fn $to_power_of_two_digits_desc_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_to_power_of_two_digits_desc_evaluation_strategy::<$t, $u>(
                gm, limit, file_name,
            );
        }

        fn $from_power_of_two_digits_asc_bench_name(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_from_power_of_two_digits_asc::<$t, $u>(gm, limit, file_name);
        }

        fn $from_power_of_two_digits_desc_bench_name(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_from_power_of_two_digits_desc::<$t, $u>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    u8,
    demo_u8_to_power_of_two_digits_asc_u8,
    demo_u8_to_power_of_two_digits_desc_u8,
    demo_u8_from_power_of_two_digits_asc_u8,
    demo_u8_from_power_of_two_digits_desc_u8,
    benchmark_u8_to_power_of_two_digits_asc_u8_evaluation_strategy,
    benchmark_u8_to_power_of_two_digits_desc_u8_evaluation_strategy,
    benchmark_u8_from_power_of_two_digits_asc_u8,
    benchmark_u8_from_power_of_two_digits_desc_u8
);
demo_and_bench!(
    u8,
    u16,
    demo_u8_to_power_of_two_digits_asc_u16,
    demo_u8_to_power_of_two_digits_desc_u16,
    demo_u8_from_power_of_two_digits_asc_u16,
    demo_u8_from_power_of_two_digits_desc_u16,
    benchmark_u8_to_power_of_two_digits_asc_u16_evaluation_strategy,
    benchmark_u8_to_power_of_two_digits_desc_u16_evaluation_strategy,
    benchmark_u8_from_power_of_two_digits_asc_u16,
    benchmark_u8_from_power_of_two_digits_desc_u16
);
demo_and_bench!(
    u8,
    u32,
    demo_u8_to_power_of_two_digits_asc_u32,
    demo_u8_to_power_of_two_digits_desc_u32,
    demo_u8_from_power_of_two_digits_asc_u32,
    demo_u8_from_power_of_two_digits_desc_u32,
    benchmark_u8_to_power_of_two_digits_asc_u32_evaluation_strategy,
    benchmark_u8_to_power_of_two_digits_desc_u32_evaluation_strategy,
    benchmark_u8_from_power_of_two_digits_asc_u32,
    benchmark_u8_from_power_of_two_digits_desc_u32
);
demo_and_bench!(
    u8,
    u64,
    demo_u8_to_power_of_two_digits_asc_u64,
    demo_u8_to_power_of_two_digits_desc_u64,
    demo_u8_from_power_of_two_digits_asc_u64,
    demo_u8_from_power_of_two_digits_desc_u64,
    benchmark_u8_to_power_of_two_digits_asc_u64_evaluation_strategy,
    benchmark_u8_to_power_of_two_digits_desc_u64_evaluation_strategy,
    benchmark_u8_from_power_of_two_digits_asc_u64,
    benchmark_u8_from_power_of_two_digits_desc_u64
);
demo_and_bench!(
    u8,
    usize,
    demo_u8_to_power_of_two_digits_asc_usize,
    demo_u8_to_power_of_two_digits_desc_usize,
    demo_u8_from_power_of_two_digits_asc_usize,
    demo_u8_from_power_of_two_digits_desc_usize,
    benchmark_u8_to_power_of_two_digits_asc_usize_evaluation_strategy,
    benchmark_u8_to_power_of_two_digits_desc_usize_evaluation_strategy,
    benchmark_u8_from_power_of_two_digits_asc_usize,
    benchmark_u8_from_power_of_two_digits_desc_usize
);

demo_and_bench!(
    u16,
    u8,
    demo_u16_to_power_of_two_digits_asc_u8,
    demo_u16_to_power_of_two_digits_desc_u8,
    demo_u16_from_power_of_two_digits_asc_u8,
    demo_u16_from_power_of_two_digits_desc_u8,
    benchmark_u16_to_power_of_two_digits_asc_u8_evaluation_strategy,
    benchmark_u16_to_power_of_two_digits_desc_u8_evaluation_strategy,
    benchmark_u16_from_power_of_two_digits_asc_u8,
    benchmark_u16_from_power_of_two_digits_desc_u8
);
demo_and_bench!(
    u16,
    u16,
    demo_u16_to_power_of_two_digits_asc_u16,
    demo_u16_to_power_of_two_digits_desc_u16,
    demo_u16_from_power_of_two_digits_asc_u16,
    demo_u16_from_power_of_two_digits_desc_u16,
    benchmark_u16_to_power_of_two_digits_asc_u16_evaluation_strategy,
    benchmark_u16_to_power_of_two_digits_desc_u16_evaluation_strategy,
    benchmark_u16_from_power_of_two_digits_asc_u16,
    benchmark_u16_from_power_of_two_digits_desc_u16
);
demo_and_bench!(
    u16,
    u32,
    demo_u16_to_power_of_two_digits_asc_u32,
    demo_u16_to_power_of_two_digits_desc_u32,
    demo_u16_from_power_of_two_digits_asc_u32,
    demo_u16_from_power_of_two_digits_desc_u32,
    benchmark_u16_to_power_of_two_digits_asc_u32_evaluation_strategy,
    benchmark_u16_to_power_of_two_digits_desc_u32_evaluation_strategy,
    benchmark_u16_from_power_of_two_digits_asc_u32,
    benchmark_u16_from_power_of_two_digits_desc_u32
);
demo_and_bench!(
    u16,
    u64,
    demo_u16_to_power_of_two_digits_asc_u64,
    demo_u16_to_power_of_two_digits_desc_u64,
    demo_u16_from_power_of_two_digits_asc_u64,
    demo_u16_from_power_of_two_digits_desc_u64,
    benchmark_u16_to_power_of_two_digits_asc_u64_evaluation_strategy,
    benchmark_u16_to_power_of_two_digits_desc_u64_evaluation_strategy,
    benchmark_u16_from_power_of_two_digits_asc_u64,
    benchmark_u16_from_power_of_two_digits_desc_u64
);
demo_and_bench!(
    u16,
    usize,
    demo_u16_to_power_of_two_digits_asc_usize,
    demo_u16_to_power_of_two_digits_desc_usize,
    demo_u16_from_power_of_two_digits_asc_usize,
    demo_u16_from_power_of_two_digits_desc_usize,
    benchmark_u16_to_power_of_two_digits_asc_usize_evaluation_strategy,
    benchmark_u16_to_power_of_two_digits_desc_usize_evaluation_strategy,
    benchmark_u16_from_power_of_two_digits_asc_usize,
    benchmark_u16_from_power_of_two_digits_desc_usize
);

demo_and_bench!(
    u32,
    u8,
    demo_u32_to_power_of_two_digits_asc_u8,
    demo_u32_to_power_of_two_digits_desc_u8,
    demo_u32_from_power_of_two_digits_asc_u8,
    demo_u32_from_power_of_two_digits_desc_u8,
    benchmark_u32_to_power_of_two_digits_asc_u8_evaluation_strategy,
    benchmark_u32_to_power_of_two_digits_desc_u8_evaluation_strategy,
    benchmark_u32_from_power_of_two_digits_asc_u8,
    benchmark_u32_from_power_of_two_digits_desc_u8
);
demo_and_bench!(
    u32,
    u16,
    demo_u32_to_power_of_two_digits_asc_u16,
    demo_u32_to_power_of_two_digits_desc_u16,
    demo_u32_from_power_of_two_digits_asc_u16,
    demo_u32_from_power_of_two_digits_desc_u16,
    benchmark_u32_to_power_of_two_digits_asc_u16_evaluation_strategy,
    benchmark_u32_to_power_of_two_digits_desc_u16_evaluation_strategy,
    benchmark_u32_from_power_of_two_digits_asc_u16,
    benchmark_u32_from_power_of_two_digits_desc_u16
);
demo_and_bench!(
    u32,
    u32,
    demo_u32_to_power_of_two_digits_asc_u32,
    demo_u32_to_power_of_two_digits_desc_u32,
    demo_u32_from_power_of_two_digits_asc_u32,
    demo_u32_from_power_of_two_digits_desc_u32,
    benchmark_u32_to_power_of_two_digits_asc_u32_evaluation_strategy,
    benchmark_u32_to_power_of_two_digits_desc_u32_evaluation_strategy,
    benchmark_u32_from_power_of_two_digits_asc_u32,
    benchmark_u32_from_power_of_two_digits_desc_u32
);
demo_and_bench!(
    u32,
    u64,
    demo_u32_to_power_of_two_digits_asc_u64,
    demo_u32_to_power_of_two_digits_desc_u64,
    demo_u32_from_power_of_two_digits_asc_u64,
    demo_u32_from_power_of_two_digits_desc_u64,
    benchmark_u32_to_power_of_two_digits_asc_u64_evaluation_strategy,
    benchmark_u32_to_power_of_two_digits_desc_u64_evaluation_strategy,
    benchmark_u32_from_power_of_two_digits_asc_u64,
    benchmark_u32_from_power_of_two_digits_desc_u64
);
demo_and_bench!(
    u32,
    usize,
    demo_u32_to_power_of_two_digits_asc_usize,
    demo_u32_to_power_of_two_digits_desc_usize,
    demo_u32_from_power_of_two_digits_asc_usize,
    demo_u32_from_power_of_two_digits_desc_usize,
    benchmark_u32_to_power_of_two_digits_asc_usize_evaluation_strategy,
    benchmark_u32_to_power_of_two_digits_desc_usize_evaluation_strategy,
    benchmark_u32_from_power_of_two_digits_asc_usize,
    benchmark_u32_from_power_of_two_digits_desc_usize
);

demo_and_bench!(
    u64,
    u8,
    demo_u64_to_power_of_two_digits_asc_u8,
    demo_u64_to_power_of_two_digits_desc_u8,
    demo_u64_from_power_of_two_digits_asc_u8,
    demo_u64_from_power_of_two_digits_desc_u8,
    benchmark_u64_to_power_of_two_digits_asc_u8_evaluation_strategy,
    benchmark_u64_to_power_of_two_digits_desc_u8_evaluation_strategy,
    benchmark_u64_from_power_of_two_digits_asc_u8,
    benchmark_u64_from_power_of_two_digits_desc_u8
);
demo_and_bench!(
    u64,
    u16,
    demo_u64_to_power_of_two_digits_asc_u16,
    demo_u64_to_power_of_two_digits_desc_u16,
    demo_u64_from_power_of_two_digits_asc_u16,
    demo_u64_from_power_of_two_digits_desc_u16,
    benchmark_u64_to_power_of_two_digits_asc_u16_evaluation_strategy,
    benchmark_u64_to_power_of_two_digits_desc_u16_evaluation_strategy,
    benchmark_u64_from_power_of_two_digits_asc_u16,
    benchmark_u64_from_power_of_two_digits_desc_u16
);
demo_and_bench!(
    u64,
    u32,
    demo_u64_to_power_of_two_digits_asc_u32,
    demo_u64_to_power_of_two_digits_desc_u32,
    demo_u64_from_power_of_two_digits_asc_u32,
    demo_u64_from_power_of_two_digits_desc_u32,
    benchmark_u64_to_power_of_two_digits_asc_u32_evaluation_strategy,
    benchmark_u64_to_power_of_two_digits_desc_u32_evaluation_strategy,
    benchmark_u64_from_power_of_two_digits_asc_u32,
    benchmark_u64_from_power_of_two_digits_desc_u32
);
demo_and_bench!(
    u64,
    u64,
    demo_u64_to_power_of_two_digits_asc_u64,
    demo_u64_to_power_of_two_digits_desc_u64,
    demo_u64_from_power_of_two_digits_asc_u64,
    demo_u64_from_power_of_two_digits_desc_u64,
    benchmark_u64_to_power_of_two_digits_asc_u64_evaluation_strategy,
    benchmark_u64_to_power_of_two_digits_desc_u64_evaluation_strategy,
    benchmark_u64_from_power_of_two_digits_asc_u64,
    benchmark_u64_from_power_of_two_digits_desc_u64
);
demo_and_bench!(
    u64,
    usize,
    demo_u64_to_power_of_two_digits_asc_usize,
    demo_u64_to_power_of_two_digits_desc_usize,
    demo_u64_from_power_of_two_digits_asc_usize,
    demo_u64_from_power_of_two_digits_desc_usize,
    benchmark_u64_to_power_of_two_digits_asc_usize_evaluation_strategy,
    benchmark_u64_to_power_of_two_digits_desc_usize_evaluation_strategy,
    benchmark_u64_from_power_of_two_digits_asc_usize,
    benchmark_u64_from_power_of_two_digits_desc_usize
);

demo_and_bench!(
    usize,
    u8,
    demo_usize_to_power_of_two_digits_asc_u8,
    demo_usize_to_power_of_two_digits_desc_u8,
    demo_usize_from_power_of_two_digits_asc_u8,
    demo_usize_from_power_of_two_digits_desc_u8,
    benchmark_usize_to_power_of_two_digits_asc_u8_evaluation_strategy,
    benchmark_usize_to_power_of_two_digits_desc_u8_evaluation_strategy,
    benchmark_usize_from_power_of_two_digits_asc_u8,
    benchmark_usize_from_power_of_two_digits_desc_u8
);
demo_and_bench!(
    usize,
    u16,
    demo_usize_to_power_of_two_digits_asc_u16,
    demo_usize_to_power_of_two_digits_desc_u16,
    demo_usize_from_power_of_two_digits_asc_u16,
    demo_usize_from_power_of_two_digits_desc_u16,
    benchmark_usize_to_power_of_two_digits_asc_u16_evaluation_strategy,
    benchmark_usize_to_power_of_two_digits_desc_u16_evaluation_strategy,
    benchmark_usize_from_power_of_two_digits_asc_u16,
    benchmark_usize_from_power_of_two_digits_desc_u16
);
demo_and_bench!(
    usize,
    u32,
    demo_usize_to_power_of_two_digits_asc_u32,
    demo_usize_to_power_of_two_digits_desc_u32,
    demo_usize_from_power_of_two_digits_asc_u32,
    demo_usize_from_power_of_two_digits_desc_u32,
    benchmark_usize_to_power_of_two_digits_asc_u32_evaluation_strategy,
    benchmark_usize_to_power_of_two_digits_desc_u32_evaluation_strategy,
    benchmark_usize_from_power_of_two_digits_asc_u32,
    benchmark_usize_from_power_of_two_digits_desc_u32
);
demo_and_bench!(
    usize,
    u64,
    demo_usize_to_power_of_two_digits_asc_u64,
    demo_usize_to_power_of_two_digits_desc_u64,
    demo_usize_from_power_of_two_digits_asc_u64,
    demo_usize_from_power_of_two_digits_desc_u64,
    benchmark_usize_to_power_of_two_digits_asc_u64_evaluation_strategy,
    benchmark_usize_to_power_of_two_digits_desc_u64_evaluation_strategy,
    benchmark_usize_from_power_of_two_digits_asc_u64,
    benchmark_usize_from_power_of_two_digits_desc_u64
);
demo_and_bench!(
    usize,
    usize,
    demo_usize_to_power_of_two_digits_asc_usize,
    demo_usize_to_power_of_two_digits_desc_usize,
    demo_usize_from_power_of_two_digits_asc_usize,
    demo_usize_from_power_of_two_digits_desc_usize,
    benchmark_usize_to_power_of_two_digits_asc_usize_evaluation_strategy,
    benchmark_usize_to_power_of_two_digits_desc_usize_evaluation_strategy,
    benchmark_usize_from_power_of_two_digits_asc_usize,
    benchmark_usize_from_power_of_two_digits_desc_usize
);
