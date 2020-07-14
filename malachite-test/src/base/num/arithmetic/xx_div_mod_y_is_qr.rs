use malachite_base::num::arithmetic::xx_div_mod_y_is_qr::_explicit_xx_div_mod_y_is_qr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::triples_of_unsigneds_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_xx_div_mod_y_is_qr);
    register_demo!(registry, demo_u16_xx_div_mod_y_is_qr);
    register_demo!(registry, demo_u32_xx_div_mod_y_is_qr);
    register_demo!(registry, demo_u64_xx_div_mod_y_is_qr);
    register_demo!(registry, demo_usize_xx_div_mod_y_is_qr);

    register_bench!(registry, None, benchmark_u8_xx_div_mod_y_is_qr_algorithms);
    register_bench!(registry, None, benchmark_u16_xx_div_mod_y_is_qr_algorithms);
    register_bench!(registry, None, benchmark_u32_xx_div_mod_y_is_qr_algorithms);
    register_bench!(registry, None, benchmark_u64_xx_div_mod_y_is_qr_algorithms);
    register_bench!(
        registry,
        None,
        benchmark_usize_xx_div_mod_y_is_qr_algorithms
    );
}

fn demo_xx_div_mod_y_is_qr<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x_1, x_0, y) in triples_of_unsigneds_var_2::<T>(gm).take(limit) {
        println!(
            "xx_div_mod_y_is_qr({}, {}, {}) = {:?}",
            x_1,
            x_0,
            y,
            T::xx_div_mod_y_is_qr(x_1, x_0, y),
        );
    }
}

fn benchmark_xx_div_mod_y_is_qr_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!(
            "{}.xx_div_mod_y_is_qr({}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME,
        ),
        BenchmarkType::Algorithms,
        triples_of_unsigneds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x_1, x_0, _)| usize::exact_from(limbs_significant_bits(&[x_0, x_1]))),
        "x.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(x_1, x_0, y)| no_out!(T::xx_div_mod_y_is_qr(x_1, x_0, y))),
            ),
            (
                "explicit",
                &mut (|(x_1, x_0, y)| no_out!(_explicit_xx_div_mod_y_is_qr(x_1, x_0, y))),
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
            demo_xx_div_mod_y_is_qr::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_xx_div_mod_y_is_qr_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_xx_div_mod_y_is_qr,
    benchmark_u8_xx_div_mod_y_is_qr_algorithms,
);
unsigned!(
    u16,
    demo_u16_xx_div_mod_y_is_qr,
    benchmark_u16_xx_div_mod_y_is_qr_algorithms,
);
unsigned!(
    u32,
    demo_u32_xx_div_mod_y_is_qr,
    benchmark_u32_xx_div_mod_y_is_qr_algorithms,
);
unsigned!(
    u64,
    demo_u64_xx_div_mod_y_is_qr,
    benchmark_u64_xx_div_mod_y_is_qr_algorithms,
);
unsigned!(
    usize,
    demo_usize_xx_div_mod_y_is_qr,
    benchmark_usize_xx_div_mod_y_is_qr_algorithms,
);
