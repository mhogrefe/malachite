use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::PowerOfTwoDigits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::natural::Natural;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{
    DemoBenchRegistry, GenerationMode, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    pairs_of_u64_and_unsigned_vec_var_3, pairs_of_u64_and_unsigned_vec_var_4,
};
use malachite_test::inputs::natural::pairs_of_u64_and_natural_vec_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_asc_u8);
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_asc_u16);
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_asc_u32);
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_asc_u64);
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_asc_usize);

    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_desc_u8);
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_desc_u16);
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_desc_u32);
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_desc_u64);
    register_ns_demo!(registry, demo_natural_from_power_of_two_digits_desc_usize);

    register_demo!(registry, demo_natural_from_power_of_two_digits_asc_natural);
    register_demo!(registry, demo_natural_from_power_of_two_digits_desc_natural);

    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_asc_u8_algorithms
    );
    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_asc_u16_algorithms
    );
    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_asc_u32_algorithms
    );
    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_asc_u64_algorithms
    );
    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_asc_usize_algorithms
    );

    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_desc_u8
    );
    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_desc_u16
    );
    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_desc_u32
    );
    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_desc_u64
    );
    register_ns_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_desc_usize
    );

    register_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_asc_natural_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_from_power_of_two_digits_desc_natural
    );
}

fn demo_from_power_of_two_digits_asc<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) where
    Natural: PowerOfTwoDigits<T>,
{
    for (log_base, digits) in pairs_of_u64_and_unsigned_vec_var_3::<T>(gm).take(limit) {
        println!(
            "Natural::from_power_of_two_digits_asc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_two_digits_asc(log_base, &digits)
        );
    }
}

fn demo_from_power_of_two_digits_desc<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) where
    Natural: PowerOfTwoDigits<T>,
{
    for (log_base, digits) in pairs_of_u64_and_unsigned_vec_var_4::<T>(gm).take(limit) {
        println!(
            "Natural::from_power_of_two_digits_desc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_two_digits_desc(log_base, &digits)
        );
    }
}

fn demo_natural_from_power_of_two_digits_asc_natural(gm: GenerationMode, limit: usize) {
    for (log_base, digits) in pairs_of_u64_and_natural_vec_var_1(gm).take(limit) {
        println!(
            "Natural.from_power_of_two_digits_asc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_two_digits_asc(log_base, &digits)
        );
    }
}

fn demo_natural_from_power_of_two_digits_desc_natural(gm: GenerationMode, limit: usize) {
    for (log_base, digits) in pairs_of_u64_and_natural_vec_var_1(gm).take(limit) {
        println!(
            "Natural.from_power_of_two_digits_desc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_two_digits_desc(log_base, &digits)
        );
    }
}

fn benchmark_from_power_of_two_digits_asc_algorithms<
    T: PrimitiveUnsigned + Rand + SampleRange,
    F: Fn(u64, &[T]) -> Natural,
>(
    from_power_of_two_digits_asc_naive: F,
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PowerOfTwoDigits<T>,
{
    run_benchmark(
        &format!(
            "PowerOfTwoDigits::<Natural>::from_power_of_two_digits_asc(&[{}], u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        pairs_of_u64_and_unsigned_vec_var_3::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, digits)| digits.len()),
        "digits.len()",
        &mut [
            (
                "default",
                &mut (|(log_base, ref digits)| {
                    no_out!(Natural::from_power_of_two_digits_asc(log_base, digits))
                }),
            ),
            (
                "naive",
                &mut (|(log_base, ref digits)| {
                    no_out!(from_power_of_two_digits_asc_naive(log_base, digits))
                }),
            ),
        ],
    );
}

fn benchmark_from_power_of_two_digits_desc<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PowerOfTwoDigits<T>,
{
    run_benchmark(
        &format!(
            "PowerOfTwoDigits::<Natural>::from_power_of_two_digits_desc(&[{}], u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        pairs_of_u64_and_unsigned_vec_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, digits)| digits.len()),
        "digits.len()",
        &mut [(
            "malachite",
            &mut (|(log_base, ref digits)| {
                no_out!(Natural::from_power_of_two_digits_desc(log_base, digits))
            }),
        )],
    );
}

fn benchmark_natural_from_power_of_two_digits_asc_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_power_of_two_digits_asc(u64, &[Natural])",
        BenchmarkType::Algorithms,
        pairs_of_u64_and_natural_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(log_base, ref digits)| digits.len() * usize::exact_from(log_base)),
        "digits.len() * log_base",
        &mut [
            (
                "default",
                &mut (|(log_base, ref digits)| {
                    no_out!(Natural::from_power_of_two_digits_asc(log_base, digits))
                }),
            ),
            (
                "naive",
                &mut (|(log_base, ref digits)| {
                    no_out!(Natural::_from_power_of_two_digits_asc_natural_naive(
                        log_base, digits
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_natural_from_power_of_two_digits_desc_natural(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_power_of_two_digits_desc(u64, &[Natural])",
        BenchmarkType::Single,
        pairs_of_u64_and_natural_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(log_base, ref digits)| digits.len() * usize::exact_from(log_base)),
        "digits.len() * log_base",
        &mut [(
            "malachite",
            &mut (|(log_base, ref digits)| {
                no_out!(Natural::from_power_of_two_digits_desc(log_base, digits))
            }),
        )],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $from_power_of_two_digits_asc_naive:ident,
        $from_power_of_two_digits_asc_demo_name:ident,
        $from_power_of_two_digits_desc_demo_name:ident,
        $from_power_of_two_digits_asc_bench_name:ident,
        $from_power_of_two_digits_desc_bench_name:ident
    ) => {
        fn $from_power_of_two_digits_asc_demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_from_power_of_two_digits_asc::<$t>(gm, limit);
        }

        fn $from_power_of_two_digits_desc_demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_from_power_of_two_digits_desc::<$t>(gm, limit);
        }

        fn $from_power_of_two_digits_asc_bench_name(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_from_power_of_two_digits_asc_algorithms::<$t, _>(
                Natural::$from_power_of_two_digits_asc_naive,
                gm,
                limit,
                file_name,
            );
        }

        fn $from_power_of_two_digits_desc_bench_name(
            gm: NoSpecialGenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_from_power_of_two_digits_desc::<$t>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    _from_power_of_two_digits_asc_u8_naive,
    demo_natural_from_power_of_two_digits_asc_u8,
    demo_natural_from_power_of_two_digits_desc_u8,
    benchmark_natural_from_power_of_two_digits_asc_u8_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_u8
);
demo_and_bench!(
    u16,
    _from_power_of_two_digits_asc_u16_naive,
    demo_natural_from_power_of_two_digits_asc_u16,
    demo_natural_from_power_of_two_digits_desc_u16,
    benchmark_natural_from_power_of_two_digits_asc_u16_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_u16
);
demo_and_bench!(
    u32,
    _from_power_of_two_digits_asc_u32_naive,
    demo_natural_from_power_of_two_digits_asc_u32,
    demo_natural_from_power_of_two_digits_desc_u32,
    benchmark_natural_from_power_of_two_digits_asc_u32_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_u32
);
demo_and_bench!(
    u64,
    _from_power_of_two_digits_asc_u64_naive,
    demo_natural_from_power_of_two_digits_asc_u64,
    demo_natural_from_power_of_two_digits_desc_u64,
    benchmark_natural_from_power_of_two_digits_asc_u64_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_u64
);
demo_and_bench!(
    usize,
    _from_power_of_two_digits_asc_usize_naive,
    demo_natural_from_power_of_two_digits_asc_usize,
    demo_natural_from_power_of_two_digits_desc_usize,
    benchmark_natural_from_power_of_two_digits_asc_usize_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_usize
);
