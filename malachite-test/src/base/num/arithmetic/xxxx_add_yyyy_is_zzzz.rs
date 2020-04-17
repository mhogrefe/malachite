use std::cmp::max;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::octuples_of_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_xxxx_add_yyyy_is_zzzz);
    register_demo!(registry, demo_u16_xxxx_add_yyyy_is_zzzz);
    register_demo!(registry, demo_u32_xxxx_add_yyyy_is_zzzz);
    register_demo!(registry, demo_u64_xxxx_add_yyyy_is_zzzz);
    register_demo!(registry, demo_usize_xxxx_add_yyyy_is_zzzz);

    register_bench!(registry, None, benchmark_u8_xxxx_add_yyyy_is_zzzz);
    register_bench!(registry, None, benchmark_u16_xxxx_add_yyyy_is_zzzz);
    register_bench!(registry, None, benchmark_u32_xxxx_add_yyyy_is_zzzz);
    register_bench!(registry, None, benchmark_u64_xxxx_add_yyyy_is_zzzz);
    register_bench!(registry, None, benchmark_usize_xxxx_add_yyyy_is_zzzz);
}

fn demo_xxxx_add_yyyy_is_zzzz<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0) in octuples_of_unsigneds::<T>(gm).take(limit) {
        println!(
            "({}, {}, {}, {}) + ({}, {}, {}, {}) = {:?}",
            x_3,
            x_2,
            x_1,
            x_0,
            y_3,
            y_2,
            y_1,
            y_0,
            T::xxxx_add_yyyy_is_zzzz(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0),
        );
    }
}

fn benchmark_xxxx_add_yyyy_is_zzzz<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!(
            "{}.xxxx_add_yyyy_is_zzzz({}, {}, {}, {}, {}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        octuples_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0)| {
            usize::exact_from(max(
                limbs_significant_bits(&[x_0, x_1, x_2, x_3]),
                limbs_significant_bits(&[y_0, y_1, y_2, y_3]),
            ))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0)| {
                no_out!(T::xxxx_add_yyyy_is_zzzz(
                    x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0
                ))
            }),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident,
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_xxxx_add_yyyy_is_zzzz::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_xxxx_add_yyyy_is_zzzz::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_xxxx_add_yyyy_is_zzzz,
    benchmark_u8_xxxx_add_yyyy_is_zzzz,
);
unsigned!(
    u16,
    demo_u16_xxxx_add_yyyy_is_zzzz,
    benchmark_u16_xxxx_add_yyyy_is_zzzz,
);
unsigned!(
    u32,
    demo_u32_xxxx_add_yyyy_is_zzzz,
    benchmark_u32_xxxx_add_yyyy_is_zzzz,
);
unsigned!(
    u64,
    demo_u64_xxxx_add_yyyy_is_zzzz,
    benchmark_u64_xxxx_add_yyyy_is_zzzz,
);
unsigned!(
    usize,
    demo_usize_xxxx_add_yyyy_is_zzzz,
    benchmark_usize_xxxx_add_yyyy_is_zzzz,
);
