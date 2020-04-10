use std::cmp::max;

use malachite_base::num::arithmetic::unsigneds::_explicit_xx_add_yy_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::quadruples_of_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_xx_add_yy_is_zz);
    register_demo!(registry, demo_u16_xx_add_yy_is_zz);
    register_demo!(registry, demo_u32_xx_add_yy_is_zz);
    register_demo!(registry, demo_u64_xx_add_yy_is_zz);
    register_demo!(registry, demo_usize_xx_add_yy_is_zz);

    register_bench!(registry, None, benchmark_u8_xx_add_yy_is_zz_algorithms);
    register_bench!(registry, None, benchmark_u16_xx_add_yy_is_zz_algorithms);
    register_bench!(registry, None, benchmark_u32_xx_add_yy_is_zz_algorithms);
    register_bench!(registry, None, benchmark_u64_xx_add_yy_is_zz_algorithms);
    register_bench!(registry, None, benchmark_usize_xx_add_yy_is_zz_algorithms);
}

fn demo_xx_add_yy_is_zz<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x_1, x_0, y_1, y_0) in quadruples_of_unsigneds::<T>(gm).take(limit) {
        println!(
            "({}, {}) + ({}, {}) = {:?}",
            x_1,
            x_0,
            y_1,
            y_0,
            T::xx_add_yy_is_zz(x_1, x_0, y_1, y_0),
        );
    }
}

fn benchmark_xx_add_yy_is_zz_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!(
            "{}.xx_add_yy_is_zz({}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        quadruples_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x_1, x_0, y_1, y_0)| {
            usize::exact_from(max(
                limbs_significant_bits(&[x_1, x_0]),
                limbs_significant_bits(&[y_1, y_0]),
            ))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "default",
                &mut (|(x_1, x_0, y_1, y_0)| no_out!(T::xx_add_yy_is_zz(x_1, x_0, y_1, y_0))),
            ),
            (
                "explicit",
                &mut (|(x_1, x_0, y_1, y_0)| {
                    no_out!(_explicit_xx_add_yy_is_zz(x_1, x_0, y_1, y_0))
                }),
            ),
        ],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident,
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_xx_add_yy_is_zz::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_xx_add_yy_is_zz_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_xx_add_yy_is_zz,
    benchmark_u8_xx_add_yy_is_zz_algorithms,
);
unsigned!(
    u16,
    demo_u16_xx_add_yy_is_zz,
    benchmark_u16_xx_add_yy_is_zz_algorithms,
);
unsigned!(
    u32,
    demo_u32_xx_add_yy_is_zz,
    benchmark_u32_xx_add_yy_is_zz_algorithms,
);
unsigned!(
    u64,
    demo_u64_xx_add_yy_is_zz,
    benchmark_u64_xx_add_yy_is_zz_algorithms,
);
unsigned!(
    usize,
    demo_usize_xx_add_yy_is_zz,
    benchmark_usize_xx_add_yy_is_zz_algorithms,
);
