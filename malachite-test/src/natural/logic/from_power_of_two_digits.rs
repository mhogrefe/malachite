use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::PowerOfTwoDigits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
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
            Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned())
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
            Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned())
        );
    }
}

fn demo_natural_from_power_of_two_digits_asc_natural(gm: GenerationMode, limit: usize) {
    for (log_base, digits) in pairs_of_u64_and_natural_vec_var_1(gm).take(limit) {
        println!(
            "Natural.from_power_of_two_digits_asc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned())
        );
    }
}

fn demo_natural_from_power_of_two_digits_desc_natural(gm: GenerationMode, limit: usize) {
    for (log_base, digits) in pairs_of_u64_and_natural_vec_var_1(gm).take(limit) {
        println!(
            "Natural.from_power_of_two_digits_desc({}, {:?}) = {}",
            log_base,
            digits,
            Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned())
        );
    }
}

fn benchmark_from_power_of_two_digits_asc_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T> + PowerOfTwoDigits<T>,
{
    run_benchmark_old(
        &format!(
            "PowerOfTwoDigits::<Natural>::from_power_of_two_digits_asc\
                <I: Iterator<Item={}>>(I, u64)",
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
                    no_out!(Natural::from_power_of_two_digits_asc(
                        log_base,
                        digits.iter().cloned()
                    ))
                }),
            ),
            (
                "naive",
                &mut (|(log_base, ref digits)| {
                    no_out!(Natural::_from_power_of_two_digits_asc_naive(
                        log_base,
                        digits.iter().cloned()
                    ))
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
    run_benchmark_old(
        &format!(
            "PowerOfTwoDigits::<Natural>::from_power_of_two_digits_desc\
                <I: Iterator<Item={}>>(I, u64)",
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
            "Malachite",
            &mut (|(log_base, ref digits)| {
                no_out!(Natural::from_power_of_two_digits_desc(
                    log_base,
                    digits.iter().cloned()
                ))
            }),
        )],
    );
}

fn benchmark_natural_from_power_of_two_digits_asc_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural::from_power_of_two_digits_asc<I: Iterator<Item=Natural>>(u64, I)",
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
                    no_out!(Natural::from_power_of_two_digits_asc(
                        log_base,
                        digits.iter().cloned()
                    ))
                }),
            ),
            (
                "naive",
                &mut (|(log_base, ref digits)| {
                    no_out!(Natural::_from_power_of_two_digits_asc_natural_naive(
                        log_base,
                        digits.iter().cloned()
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
    run_benchmark_old(
        "Natural::from_power_of_two_digits_desc<I: Iterator<Item=Natural>>(u64, I)",
        BenchmarkType::Single,
        pairs_of_u64_and_natural_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(log_base, ref digits)| digits.len() * usize::exact_from(log_base)),
        "digits.len() * log_base",
        &mut [(
            "Malachite",
            &mut (|(log_base, ref digits)| {
                no_out!(Natural::from_power_of_two_digits_desc(
                    log_base,
                    digits.iter().cloned()
                ))
            }),
        )],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
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
            benchmark_from_power_of_two_digits_asc_algorithms::<$t>(gm, limit, file_name);
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
    demo_natural_from_power_of_two_digits_asc_u8,
    demo_natural_from_power_of_two_digits_desc_u8,
    benchmark_natural_from_power_of_two_digits_asc_u8_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_u8
);
demo_and_bench!(
    u16,
    demo_natural_from_power_of_two_digits_asc_u16,
    demo_natural_from_power_of_two_digits_desc_u16,
    benchmark_natural_from_power_of_two_digits_asc_u16_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_u16
);
demo_and_bench!(
    u32,
    demo_natural_from_power_of_two_digits_asc_u32,
    demo_natural_from_power_of_two_digits_desc_u32,
    benchmark_natural_from_power_of_two_digits_asc_u32_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_u32
);
demo_and_bench!(
    u64,
    demo_natural_from_power_of_two_digits_asc_u64,
    demo_natural_from_power_of_two_digits_desc_u64,
    benchmark_natural_from_power_of_two_digits_asc_u64_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_u64
);
demo_and_bench!(
    usize,
    demo_natural_from_power_of_two_digits_asc_usize,
    demo_natural_from_power_of_two_digits_desc_usize,
    benchmark_natural_from_power_of_two_digits_asc_usize_algorithms,
    benchmark_natural_from_power_of_two_digits_desc_usize
);
