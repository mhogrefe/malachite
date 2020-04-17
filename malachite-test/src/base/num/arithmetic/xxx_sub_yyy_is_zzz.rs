use std::cmp::max;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::sextuples_of_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_xxx_sub_yyy_is_zzz);
    register_demo!(registry, demo_u16_xxx_sub_yyy_is_zzz);
    register_demo!(registry, demo_u32_xxx_sub_yyy_is_zzz);
    register_demo!(registry, demo_u64_xxx_sub_yyy_is_zzz);
    register_demo!(registry, demo_usize_xxx_sub_yyy_is_zzz);

    register_bench!(registry, None, benchmark_u8_xxx_sub_yyy_is_zzz);
    register_bench!(registry, None, benchmark_u16_xxx_sub_yyy_is_zzz);
    register_bench!(registry, None, benchmark_u32_xxx_sub_yyy_is_zzz);
    register_bench!(registry, None, benchmark_u64_xxx_sub_yyy_is_zzz);
    register_bench!(registry, None, benchmark_usize_xxx_sub_yyy_is_zzz);
}

fn demo_xxx_sub_yyy_is_zzz<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x_2, x_1, x_0, y_2, y_1, y_0) in sextuples_of_unsigneds::<T>(gm).take(limit) {
        println!(
            "({}, {}, {}) - ({}, {}, {}) = {:?}",
            x_2,
            x_1,
            x_0,
            y_2,
            y_1,
            y_0,
            T::xxx_sub_yyy_is_zzz(x_2, x_1, x_0, y_2, y_1, y_0),
        );
    }
}

fn benchmark_xxx_sub_yyy_is_zzz<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!(
            "{}.xxx_sub_yyy_is_zzz({}, {}, {}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        sextuples_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x_2, x_1, x_0, y_2, y_1, y_0)| {
            usize::exact_from(max(
                limbs_significant_bits(&[x_0, x_1, x_2]),
                limbs_significant_bits(&[y_0, y_1, y_2]),
            ))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(x_2, x_1, x_0, y_2, y_1, y_0)| {
                no_out!(T::xxx_sub_yyy_is_zzz(x_2, x_1, x_0, y_2, y_1, y_0))
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
            demo_xxx_sub_yyy_is_zzz::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_xxx_sub_yyy_is_zzz::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_xxx_sub_yyy_is_zzz,
    benchmark_u8_xxx_sub_yyy_is_zzz,
);
unsigned!(
    u16,
    demo_u16_xxx_sub_yyy_is_zzz,
    benchmark_u16_xxx_sub_yyy_is_zzz,
);
unsigned!(
    u32,
    demo_u32_xxx_sub_yyy_is_zzz,
    benchmark_u32_xxx_sub_yyy_is_zzz,
);
unsigned!(
    u64,
    demo_u64_xxx_sub_yyy_is_zzz,
    benchmark_u64_xxx_sub_yyy_is_zzz,
);
unsigned!(
    usize,
    demo_usize_xxx_sub_yyy_is_zzz,
    benchmark_usize_xxx_sub_yyy_is_zzz,
);
