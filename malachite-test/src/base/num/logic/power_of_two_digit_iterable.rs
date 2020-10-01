use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{
    PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, PowerOfTwoDigits,
};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_1, triples_of_unsigned_small_u64_and_small_u64_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_power_of_two_digits_u8);
    register_demo!(registry, demo_u8_power_of_two_digits_u16);
    register_demo!(registry, demo_u8_power_of_two_digits_u32);
    register_demo!(registry, demo_u8_power_of_two_digits_u64);
    register_demo!(registry, demo_u8_power_of_two_digits_u128);
    register_demo!(registry, demo_u8_power_of_two_digits_usize);
    register_demo!(registry, demo_u16_power_of_two_digits_u8);
    register_demo!(registry, demo_u16_power_of_two_digits_u16);
    register_demo!(registry, demo_u16_power_of_two_digits_u32);
    register_demo!(registry, demo_u16_power_of_two_digits_u64);
    register_demo!(registry, demo_u16_power_of_two_digits_u128);
    register_demo!(registry, demo_u16_power_of_two_digits_usize);
    register_demo!(registry, demo_u32_power_of_two_digits_u8);
    register_demo!(registry, demo_u32_power_of_two_digits_u16);
    register_demo!(registry, demo_u32_power_of_two_digits_u32);
    register_demo!(registry, demo_u32_power_of_two_digits_u64);
    register_demo!(registry, demo_u32_power_of_two_digits_u128);
    register_demo!(registry, demo_u32_power_of_two_digits_usize);
    register_demo!(registry, demo_u64_power_of_two_digits_u8);
    register_demo!(registry, demo_u64_power_of_two_digits_u16);
    register_demo!(registry, demo_u64_power_of_two_digits_u32);
    register_demo!(registry, demo_u64_power_of_two_digits_u64);
    register_demo!(registry, demo_u64_power_of_two_digits_u128);
    register_demo!(registry, demo_u64_power_of_two_digits_usize);
    register_demo!(registry, demo_usize_power_of_two_digits_u8);
    register_demo!(registry, demo_usize_power_of_two_digits_u16);
    register_demo!(registry, demo_usize_power_of_two_digits_u32);
    register_demo!(registry, demo_usize_power_of_two_digits_u64);
    register_demo!(registry, demo_usize_power_of_two_digits_u128);
    register_demo!(registry, demo_usize_power_of_two_digits_usize);

    register_demo!(registry, demo_u8_power_of_two_digits_rev_u8);
    register_demo!(registry, demo_u8_power_of_two_digits_rev_u16);
    register_demo!(registry, demo_u8_power_of_two_digits_rev_u32);
    register_demo!(registry, demo_u8_power_of_two_digits_rev_u64);
    register_demo!(registry, demo_u8_power_of_two_digits_rev_u128);
    register_demo!(registry, demo_u8_power_of_two_digits_rev_usize);
    register_demo!(registry, demo_u16_power_of_two_digits_rev_u8);
    register_demo!(registry, demo_u16_power_of_two_digits_rev_u16);
    register_demo!(registry, demo_u16_power_of_two_digits_rev_u32);
    register_demo!(registry, demo_u16_power_of_two_digits_rev_u64);
    register_demo!(registry, demo_u16_power_of_two_digits_rev_u128);
    register_demo!(registry, demo_u16_power_of_two_digits_rev_usize);
    register_demo!(registry, demo_u32_power_of_two_digits_rev_u8);
    register_demo!(registry, demo_u32_power_of_two_digits_rev_u16);
    register_demo!(registry, demo_u32_power_of_two_digits_rev_u32);
    register_demo!(registry, demo_u32_power_of_two_digits_rev_u64);
    register_demo!(registry, demo_u32_power_of_two_digits_rev_u128);
    register_demo!(registry, demo_u32_power_of_two_digits_rev_usize);
    register_demo!(registry, demo_u64_power_of_two_digits_rev_u8);
    register_demo!(registry, demo_u64_power_of_two_digits_rev_u16);
    register_demo!(registry, demo_u64_power_of_two_digits_rev_u32);
    register_demo!(registry, demo_u64_power_of_two_digits_rev_u64);
    register_demo!(registry, demo_u64_power_of_two_digits_rev_u128);
    register_demo!(registry, demo_u64_power_of_two_digits_rev_usize);
    register_demo!(registry, demo_usize_power_of_two_digits_rev_u8);
    register_demo!(registry, demo_usize_power_of_two_digits_rev_u16);
    register_demo!(registry, demo_usize_power_of_two_digits_rev_u32);
    register_demo!(registry, demo_usize_power_of_two_digits_rev_u64);
    register_demo!(registry, demo_usize_power_of_two_digits_rev_u128);
    register_demo!(registry, demo_usize_power_of_two_digits_rev_usize);

    register_demo!(registry, demo_u8_power_of_two_digits_size_hint_u8);
    register_demo!(registry, demo_u8_power_of_two_digits_size_hint_u16);
    register_demo!(registry, demo_u8_power_of_two_digits_size_hint_u32);
    register_demo!(registry, demo_u8_power_of_two_digits_size_hint_u64);
    register_demo!(registry, demo_u8_power_of_two_digits_size_hint_u128);
    register_demo!(registry, demo_u8_power_of_two_digits_size_hint_usize);
    register_demo!(registry, demo_u16_power_of_two_digits_size_hint_u8);
    register_demo!(registry, demo_u16_power_of_two_digits_size_hint_u16);
    register_demo!(registry, demo_u16_power_of_two_digits_size_hint_u32);
    register_demo!(registry, demo_u16_power_of_two_digits_size_hint_u64);
    register_demo!(registry, demo_u16_power_of_two_digits_size_hint_u128);
    register_demo!(registry, demo_u16_power_of_two_digits_size_hint_usize);
    register_demo!(registry, demo_u32_power_of_two_digits_size_hint_u8);
    register_demo!(registry, demo_u32_power_of_two_digits_size_hint_u16);
    register_demo!(registry, demo_u32_power_of_two_digits_size_hint_u32);
    register_demo!(registry, demo_u32_power_of_two_digits_size_hint_u64);
    register_demo!(registry, demo_u32_power_of_two_digits_size_hint_u128);
    register_demo!(registry, demo_u32_power_of_two_digits_size_hint_usize);
    register_demo!(registry, demo_u64_power_of_two_digits_size_hint_u8);
    register_demo!(registry, demo_u64_power_of_two_digits_size_hint_u16);
    register_demo!(registry, demo_u64_power_of_two_digits_size_hint_u32);
    register_demo!(registry, demo_u64_power_of_two_digits_size_hint_u64);
    register_demo!(registry, demo_u64_power_of_two_digits_size_hint_u128);
    register_demo!(registry, demo_u64_power_of_two_digits_size_hint_usize);
    register_demo!(registry, demo_usize_power_of_two_digits_size_hint_u8);
    register_demo!(registry, demo_usize_power_of_two_digits_size_hint_u16);
    register_demo!(registry, demo_usize_power_of_two_digits_size_hint_u32);
    register_demo!(registry, demo_usize_power_of_two_digits_size_hint_u64);
    register_demo!(registry, demo_usize_power_of_two_digits_size_hint_u128);
    register_demo!(registry, demo_usize_power_of_two_digits_size_hint_usize);

    register_demo!(registry, demo_u8_power_of_two_digits_get_u8);
    register_demo!(registry, demo_u8_power_of_two_digits_get_u16);
    register_demo!(registry, demo_u8_power_of_two_digits_get_u32);
    register_demo!(registry, demo_u8_power_of_two_digits_get_u64);
    register_demo!(registry, demo_u8_power_of_two_digits_get_u128);
    register_demo!(registry, demo_u8_power_of_two_digits_get_usize);
    register_demo!(registry, demo_u16_power_of_two_digits_get_u8);
    register_demo!(registry, demo_u16_power_of_two_digits_get_u16);
    register_demo!(registry, demo_u16_power_of_two_digits_get_u32);
    register_demo!(registry, demo_u16_power_of_two_digits_get_u64);
    register_demo!(registry, demo_u16_power_of_two_digits_get_u128);
    register_demo!(registry, demo_u16_power_of_two_digits_get_usize);
    register_demo!(registry, demo_u32_power_of_two_digits_get_u8);
    register_demo!(registry, demo_u32_power_of_two_digits_get_u16);
    register_demo!(registry, demo_u32_power_of_two_digits_get_u32);
    register_demo!(registry, demo_u32_power_of_two_digits_get_u64);
    register_demo!(registry, demo_u32_power_of_two_digits_get_u128);
    register_demo!(registry, demo_u32_power_of_two_digits_get_usize);
    register_demo!(registry, demo_u64_power_of_two_digits_get_u8);
    register_demo!(registry, demo_u64_power_of_two_digits_get_u16);
    register_demo!(registry, demo_u64_power_of_two_digits_get_u32);
    register_demo!(registry, demo_u64_power_of_two_digits_get_u64);
    register_demo!(registry, demo_u64_power_of_two_digits_get_u128);
    register_demo!(registry, demo_u64_power_of_two_digits_get_usize);
    register_demo!(registry, demo_usize_power_of_two_digits_get_u8);
    register_demo!(registry, demo_usize_power_of_two_digits_get_u16);
    register_demo!(registry, demo_usize_power_of_two_digits_get_u32);
    register_demo!(registry, demo_usize_power_of_two_digits_get_u64);
    register_demo!(registry, demo_usize_power_of_two_digits_get_u128);
    register_demo!(registry, demo_usize_power_of_two_digits_get_usize);

    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_size_hint_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_size_hint_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_size_hint_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_size_hint_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_size_hint_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_size_hint_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_size_hint_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_size_hint_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_size_hint_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_size_hint_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_size_hint_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_size_hint_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_size_hint_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_size_hint_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_size_hint_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_size_hint_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_size_hint_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_size_hint_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_size_hint_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_size_hint_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_size_hint_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_size_hint_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_size_hint_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_size_hint_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_size_hint_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_size_hint_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_size_hint_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_size_hint_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_size_hint_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_size_hint_usize
    );

    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_get_algorithms_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_get_algorithms_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_get_algorithms_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_get_algorithms_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_get_algorithms_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_u8_power_of_two_digits_get_algorithms_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_get_algorithms_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_get_algorithms_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_get_algorithms_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_get_algorithms_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_get_algorithms_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_u16_power_of_two_digits_get_algorithms_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_get_algorithms_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_get_algorithms_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_get_algorithms_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_get_algorithms_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_get_algorithms_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_u32_power_of_two_digits_get_algorithms_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_get_algorithms_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_get_algorithms_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_get_algorithms_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_get_algorithms_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_get_algorithms_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_u64_power_of_two_digits_get_algorithms_usize
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_get_algorithms_u8
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_get_algorithms_u16
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_get_algorithms_u32
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_get_algorithms_u64
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_get_algorithms_u128
    );
    register_bench!(
        registry,
        Small,
        benchmark_usize_power_of_two_digits_get_algorithms_usize
    );
}

fn demo_power_of_two_digits<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) where
    T: PowerOfTwoDigitIterable<U>,
{
    for (u, log_base) in pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm).take(limit) {
        println!(
            "power_of_two_digits({}, {}) = {:?}",
            u,
            log_base,
            PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base).collect::<Vec<U>>()
        );
    }
}

fn demo_power_of_two_digits_rev<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) where
    T: PowerOfTwoDigitIterable<U>,
{
    for (u, log_base) in pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm).take(limit) {
        println!(
            "power_of_two_digits({}, {}).rev() = {:?}",
            u,
            log_base,
            PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base)
                .rev()
                .collect::<Vec<U>>()
        );
    }
}

fn demo_power_of_two_digits_size_hint<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) where
    T: PowerOfTwoDigitIterable<U>,
{
    for (u, log_base) in pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm).take(limit) {
        println!(
            "power_of_two_digits({}, {}).size_hint() = {:?}",
            u,
            log_base,
            PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base).size_hint()
        );
    }
}

fn demo_power_of_two_digits_get<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) where
    T: PowerOfTwoDigitIterable<U>,
{
    for (u, log_base, i) in
        triples_of_unsigned_small_u64_and_small_u64_var_1::<T, U>(gm).take(limit)
    {
        println!(
            "power_of_two_digits({}, {}).get({}) = {:?}",
            u,
            log_base,
            i,
            PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base).get(i)
        );
    }
}

fn benchmark_power_of_two_digits_size_hint<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: PowerOfTwoDigitIterable<U>,
{
    run_benchmark_old(
        &format!(
            "PowerOfTwoDigitIterable::<{}>::power_of_two_digits(&{}, u64).size_hint()",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_u64_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(u, _)| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [(
            &format!(
                "PowerOfTwoDigitIterable::<{}>::power_of_two_digits(&{}, u64).size_hint()",
                U::NAME,
                T::NAME
            ),
            &mut (|(n, log_base)| {
                no_out!(PowerOfTwoDigitIterable::<U>::power_of_two_digits(n, log_base).size_hint())
            }),
        )],
    );
}

fn benchmark_power_of_two_digits_get_algorithms<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: PowerOfTwoDigits<U> + PowerOfTwoDigitIterable<U>,
{
    run_benchmark_old(
        &format!(
            "PowerOfTwoDigitIterable::<{}>::power_of_two_digits(&{}, u64).size_hint()",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        triples_of_unsigned_small_u64_and_small_u64_var_1::<T, U>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref u, _, _)| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            (
                &format!("power_of_two_digits({}, u64).get(u64)", T::NAME),
                &mut (|(u, log_base, i)| {
                    no_out!(PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base).get(i))
                }),
            ),
            (
                &format!("{}.to_power_of_two_digits_asc(u64)[u64]", T::NAME),
                &mut (|(u, log_base, i)| {
                    let digits = PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&u, log_base);
                    let i = usize::exact_from(i);
                    if i >= digits.len() {
                        U::ZERO
                    } else {
                        digits[i]
                    };
                }),
            ),
        ],
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $u:ident,
        $power_of_two_digits_demo_name:ident,
        $power_of_two_digits_rev_demo_name:ident,
        $power_of_two_digits_size_hint_demo_name:ident,
        $power_of_two_digits_get_demo_name:ident,
        $power_of_two_digits_size_hint_bench_name:ident,
        $power_of_two_digits_get_bench_name:ident
    ) => {
        fn $power_of_two_digits_demo_name(gm: GenerationMode, limit: usize) {
            demo_power_of_two_digits::<$t, $u>(gm, limit);
        }

        fn $power_of_two_digits_rev_demo_name(gm: GenerationMode, limit: usize) {
            demo_power_of_two_digits_rev::<$t, $u>(gm, limit);
        }

        fn $power_of_two_digits_size_hint_demo_name(gm: GenerationMode, limit: usize) {
            demo_power_of_two_digits_size_hint::<$t, $u>(gm, limit);
        }

        fn $power_of_two_digits_get_demo_name(gm: GenerationMode, limit: usize) {
            demo_power_of_two_digits_get::<$t, $u>(gm, limit);
        }

        fn $power_of_two_digits_size_hint_bench_name(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            benchmark_power_of_two_digits_size_hint::<$t, $u>(gm, limit, file_name);
        }

        fn $power_of_two_digits_get_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_power_of_two_digits_get_algorithms::<$t, $u>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    u8,
    demo_u8_power_of_two_digits_u8,
    demo_u8_power_of_two_digits_rev_u8,
    demo_u8_power_of_two_digits_size_hint_u8,
    demo_u8_power_of_two_digits_get_u8,
    benchmark_u8_power_of_two_digits_size_hint_u8,
    benchmark_u8_power_of_two_digits_get_algorithms_u8
);
demo_and_bench!(
    u8,
    u16,
    demo_u8_power_of_two_digits_u16,
    demo_u8_power_of_two_digits_rev_u16,
    demo_u8_power_of_two_digits_size_hint_u16,
    demo_u8_power_of_two_digits_get_u16,
    benchmark_u8_power_of_two_digits_size_hint_u16,
    benchmark_u8_power_of_two_digits_get_algorithms_u16
);
demo_and_bench!(
    u8,
    u32,
    demo_u8_power_of_two_digits_u32,
    demo_u8_power_of_two_digits_rev_u32,
    demo_u8_power_of_two_digits_size_hint_u32,
    demo_u8_power_of_two_digits_get_u32,
    benchmark_u8_power_of_two_digits_size_hint_u32,
    benchmark_u8_power_of_two_digits_get_algorithms_u32
);
demo_and_bench!(
    u8,
    u64,
    demo_u8_power_of_two_digits_u64,
    demo_u8_power_of_two_digits_rev_u64,
    demo_u8_power_of_two_digits_size_hint_u64,
    demo_u8_power_of_two_digits_get_u64,
    benchmark_u8_power_of_two_digits_size_hint_u64,
    benchmark_u8_power_of_two_digits_get_algorithms_u64
);
demo_and_bench!(
    u8,
    u128,
    demo_u8_power_of_two_digits_u128,
    demo_u8_power_of_two_digits_rev_u128,
    demo_u8_power_of_two_digits_size_hint_u128,
    demo_u8_power_of_two_digits_get_u128,
    benchmark_u8_power_of_two_digits_size_hint_u128,
    benchmark_u8_power_of_two_digits_get_algorithms_u128
);
demo_and_bench!(
    u8,
    usize,
    demo_u8_power_of_two_digits_usize,
    demo_u8_power_of_two_digits_rev_usize,
    demo_u8_power_of_two_digits_size_hint_usize,
    demo_u8_power_of_two_digits_get_usize,
    benchmark_u8_power_of_two_digits_size_hint_usize,
    benchmark_u8_power_of_two_digits_get_algorithms_usize
);
demo_and_bench!(
    u16,
    u8,
    demo_u16_power_of_two_digits_u8,
    demo_u16_power_of_two_digits_rev_u8,
    demo_u16_power_of_two_digits_size_hint_u8,
    demo_u16_power_of_two_digits_get_u8,
    benchmark_u16_power_of_two_digits_size_hint_u8,
    benchmark_u16_power_of_two_digits_get_algorithms_u8
);
demo_and_bench!(
    u16,
    u16,
    demo_u16_power_of_two_digits_u16,
    demo_u16_power_of_two_digits_rev_u16,
    demo_u16_power_of_two_digits_size_hint_u16,
    demo_u16_power_of_two_digits_get_u16,
    benchmark_u16_power_of_two_digits_size_hint_u16,
    benchmark_u16_power_of_two_digits_get_algorithms_u16
);
demo_and_bench!(
    u16,
    u32,
    demo_u16_power_of_two_digits_u32,
    demo_u16_power_of_two_digits_rev_u32,
    demo_u16_power_of_two_digits_size_hint_u32,
    demo_u16_power_of_two_digits_get_u32,
    benchmark_u16_power_of_two_digits_size_hint_u32,
    benchmark_u16_power_of_two_digits_get_algorithms_u32
);
demo_and_bench!(
    u16,
    u64,
    demo_u16_power_of_two_digits_u64,
    demo_u16_power_of_two_digits_rev_u64,
    demo_u16_power_of_two_digits_size_hint_u64,
    demo_u16_power_of_two_digits_get_u64,
    benchmark_u16_power_of_two_digits_size_hint_u64,
    benchmark_u16_power_of_two_digits_get_algorithms_u64
);
demo_and_bench!(
    u16,
    u128,
    demo_u16_power_of_two_digits_u128,
    demo_u16_power_of_two_digits_rev_u128,
    demo_u16_power_of_two_digits_size_hint_u128,
    demo_u16_power_of_two_digits_get_u128,
    benchmark_u16_power_of_two_digits_size_hint_u128,
    benchmark_u16_power_of_two_digits_get_algorithms_u128
);
demo_and_bench!(
    u16,
    usize,
    demo_u16_power_of_two_digits_usize,
    demo_u16_power_of_two_digits_rev_usize,
    demo_u16_power_of_two_digits_size_hint_usize,
    demo_u16_power_of_two_digits_get_usize,
    benchmark_u16_power_of_two_digits_size_hint_usize,
    benchmark_u16_power_of_two_digits_get_algorithms_usize
);
demo_and_bench!(
    u32,
    u8,
    demo_u32_power_of_two_digits_u8,
    demo_u32_power_of_two_digits_rev_u8,
    demo_u32_power_of_two_digits_size_hint_u8,
    demo_u32_power_of_two_digits_get_u8,
    benchmark_u32_power_of_two_digits_size_hint_u8,
    benchmark_u32_power_of_two_digits_get_algorithms_u8
);
demo_and_bench!(
    u32,
    u16,
    demo_u32_power_of_two_digits_u16,
    demo_u32_power_of_two_digits_rev_u16,
    demo_u32_power_of_two_digits_size_hint_u16,
    demo_u32_power_of_two_digits_get_u16,
    benchmark_u32_power_of_two_digits_size_hint_u16,
    benchmark_u32_power_of_two_digits_get_algorithms_u16
);
demo_and_bench!(
    u32,
    u32,
    demo_u32_power_of_two_digits_u32,
    demo_u32_power_of_two_digits_rev_u32,
    demo_u32_power_of_two_digits_size_hint_u32,
    demo_u32_power_of_two_digits_get_u32,
    benchmark_u32_power_of_two_digits_size_hint_u32,
    benchmark_u32_power_of_two_digits_get_algorithms_u32
);
demo_and_bench!(
    u32,
    u64,
    demo_u32_power_of_two_digits_u64,
    demo_u32_power_of_two_digits_rev_u64,
    demo_u32_power_of_two_digits_size_hint_u64,
    demo_u32_power_of_two_digits_get_u64,
    benchmark_u32_power_of_two_digits_size_hint_u64,
    benchmark_u32_power_of_two_digits_get_algorithms_u64
);
demo_and_bench!(
    u32,
    u128,
    demo_u32_power_of_two_digits_u128,
    demo_u32_power_of_two_digits_rev_u128,
    demo_u32_power_of_two_digits_size_hint_u128,
    demo_u32_power_of_two_digits_get_u128,
    benchmark_u32_power_of_two_digits_size_hint_u128,
    benchmark_u32_power_of_two_digits_get_algorithms_u128
);
demo_and_bench!(
    u32,
    usize,
    demo_u32_power_of_two_digits_usize,
    demo_u32_power_of_two_digits_rev_usize,
    demo_u32_power_of_two_digits_size_hint_usize,
    demo_u32_power_of_two_digits_get_usize,
    benchmark_u32_power_of_two_digits_size_hint_usize,
    benchmark_u32_power_of_two_digits_get_algorithms_usize
);
demo_and_bench!(
    u64,
    u8,
    demo_u64_power_of_two_digits_u8,
    demo_u64_power_of_two_digits_rev_u8,
    demo_u64_power_of_two_digits_size_hint_u8,
    demo_u64_power_of_two_digits_get_u8,
    benchmark_u64_power_of_two_digits_size_hint_u8,
    benchmark_u64_power_of_two_digits_get_algorithms_u8
);
demo_and_bench!(
    u64,
    u16,
    demo_u64_power_of_two_digits_u16,
    demo_u64_power_of_two_digits_rev_u16,
    demo_u64_power_of_two_digits_size_hint_u16,
    demo_u64_power_of_two_digits_get_u16,
    benchmark_u64_power_of_two_digits_size_hint_u16,
    benchmark_u64_power_of_two_digits_get_algorithms_u16
);
demo_and_bench!(
    u64,
    u32,
    demo_u64_power_of_two_digits_u32,
    demo_u64_power_of_two_digits_rev_u32,
    demo_u64_power_of_two_digits_size_hint_u32,
    demo_u64_power_of_two_digits_get_u32,
    benchmark_u64_power_of_two_digits_size_hint_u32,
    benchmark_u64_power_of_two_digits_get_algorithms_u32
);
demo_and_bench!(
    u64,
    u64,
    demo_u64_power_of_two_digits_u64,
    demo_u64_power_of_two_digits_rev_u64,
    demo_u64_power_of_two_digits_size_hint_u64,
    demo_u64_power_of_two_digits_get_u64,
    benchmark_u64_power_of_two_digits_size_hint_u64,
    benchmark_u64_power_of_two_digits_get_algorithms_u64
);
demo_and_bench!(
    u64,
    u128,
    demo_u64_power_of_two_digits_u128,
    demo_u64_power_of_two_digits_rev_u128,
    demo_u64_power_of_two_digits_size_hint_u128,
    demo_u64_power_of_two_digits_get_u128,
    benchmark_u64_power_of_two_digits_size_hint_u128,
    benchmark_u64_power_of_two_digits_get_algorithms_u128
);
demo_and_bench!(
    u64,
    usize,
    demo_u64_power_of_two_digits_usize,
    demo_u64_power_of_two_digits_rev_usize,
    demo_u64_power_of_two_digits_size_hint_usize,
    demo_u64_power_of_two_digits_get_usize,
    benchmark_u64_power_of_two_digits_size_hint_usize,
    benchmark_u64_power_of_two_digits_get_algorithms_usize
);
demo_and_bench!(
    usize,
    u8,
    demo_usize_power_of_two_digits_u8,
    demo_usize_power_of_two_digits_rev_u8,
    demo_usize_power_of_two_digits_size_hint_u8,
    demo_usize_power_of_two_digits_get_u8,
    benchmark_usize_power_of_two_digits_size_hint_u8,
    benchmark_usize_power_of_two_digits_get_algorithms_u8
);
demo_and_bench!(
    usize,
    u16,
    demo_usize_power_of_two_digits_u16,
    demo_usize_power_of_two_digits_rev_u16,
    demo_usize_power_of_two_digits_size_hint_u16,
    demo_usize_power_of_two_digits_get_u16,
    benchmark_usize_power_of_two_digits_size_hint_u16,
    benchmark_usize_power_of_two_digits_get_algorithms_u16
);
demo_and_bench!(
    usize,
    u32,
    demo_usize_power_of_two_digits_u32,
    demo_usize_power_of_two_digits_rev_u32,
    demo_usize_power_of_two_digits_size_hint_u32,
    demo_usize_power_of_two_digits_get_u32,
    benchmark_usize_power_of_two_digits_size_hint_u32,
    benchmark_usize_power_of_two_digits_get_algorithms_u32
);
demo_and_bench!(
    usize,
    u64,
    demo_usize_power_of_two_digits_u64,
    demo_usize_power_of_two_digits_rev_u64,
    demo_usize_power_of_two_digits_size_hint_u64,
    demo_usize_power_of_two_digits_get_u64,
    benchmark_usize_power_of_two_digits_size_hint_u64,
    benchmark_usize_power_of_two_digits_get_algorithms_u64
);
demo_and_bench!(
    usize,
    u128,
    demo_usize_power_of_two_digits_u128,
    demo_usize_power_of_two_digits_rev_u128,
    demo_usize_power_of_two_digits_size_hint_u128,
    demo_usize_power_of_two_digits_get_u128,
    benchmark_usize_power_of_two_digits_size_hint_u128,
    benchmark_usize_power_of_two_digits_get_algorithms_u128
);
demo_and_bench!(
    usize,
    usize,
    demo_usize_power_of_two_digits_usize,
    demo_usize_power_of_two_digits_rev_usize,
    demo_usize_power_of_two_digits_size_hint_usize,
    demo_usize_power_of_two_digits_get_usize,
    benchmark_usize_power_of_two_digits_size_hint_usize,
    benchmark_usize_power_of_two_digits_get_algorithms_usize
);
