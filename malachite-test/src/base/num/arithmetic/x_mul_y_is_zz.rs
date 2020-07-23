use std::cmp::max;

use malachite_base::num::arithmetic::x_mul_y_is_zz::_explicit_x_mul_y_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_x_mul_y_is_zz);
    register_demo!(registry, demo_u16_x_mul_y_is_zz);
    register_demo!(registry, demo_u32_x_mul_y_is_zz);
    register_demo!(registry, demo_u64_x_mul_y_is_zz);
    register_demo!(registry, demo_usize_x_mul_y_is_zz);

    register_bench!(registry, None, benchmark_u8_x_mul_y_is_zz_algorithms);
    register_bench!(registry, None, benchmark_u16_x_mul_y_is_zz_algorithms);
    register_bench!(registry, None, benchmark_u32_x_mul_y_is_zz_algorithms);
    register_bench!(registry, None, benchmark_u64_x_mul_y_is_zz_algorithms);
    register_bench!(registry, None, benchmark_usize_x_mul_y_is_zz_algorithms);
}

fn demo_x_mul_y_is_zz<T: PrimitiveUnsigned + Rand + SampleRange>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("{} * {} = {:?}", x, y, T::x_mul_y_is_zz(x, y));
    }
}

fn benchmark_x_mul_y_is_zz_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.x_mul_y_is_zz({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(x, y)| no_out!(T::x_mul_y_is_zz(x, y)))),
            (
                "explicit",
                &mut (|(x, y)| no_out!(_explicit_x_mul_y_is_zz(x, y))),
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
            demo_x_mul_y_is_zz::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_x_mul_y_is_zz_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_x_mul_y_is_zz,
    benchmark_u8_x_mul_y_is_zz_algorithms,
);
unsigned!(
    u16,
    demo_u16_x_mul_y_is_zz,
    benchmark_u16_x_mul_y_is_zz_algorithms,
);
unsigned!(
    u32,
    demo_u32_x_mul_y_is_zz,
    benchmark_u32_x_mul_y_is_zz_algorithms,
);
unsigned!(
    u64,
    demo_u64_x_mul_y_is_zz,
    benchmark_u64_x_mul_y_is_zz_algorithms,
);
unsigned!(
    usize,
    demo_usize_x_mul_y_is_zz,
    benchmark_usize_x_mul_y_is_zz_algorithms,
);
