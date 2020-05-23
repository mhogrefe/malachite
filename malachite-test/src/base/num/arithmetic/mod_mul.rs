use std::cmp::max;

use malachite_base::num::arithmetic::mod_mul::{
    _fast_mod_mul, _limbs_invert_limb_u32, _limbs_invert_limb_u64, _limbs_mod_preinverted,
    _naive_mod_mul,
};
use malachite_base::num::arithmetic::traits::{ModMulPrecomputed, PowerOfTwo};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ExactFrom, HasHalf, JoinHalves, SplitInHalf,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::num::arithmetic::mod_mul::_limbs_invert_limb_naive;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{quadruples_of_unsigneds_var_2, triples_of_unsigneds_var_1, unsigneds_var_1};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_invert_limb_u32);
    register_demo!(registry, demo_limbs_invert_limb_u64);
    register_demo!(registry, demo_limbs_mod_preinverted_u8);
    register_demo!(registry, demo_limbs_mod_preinverted_u16);
    register_demo!(registry, demo_limbs_mod_preinverted_u32);
    register_demo!(registry, demo_limbs_mod_preinverted_u64);
    register_demo!(registry, demo_u8_mod_mul);
    register_demo!(registry, demo_u16_mod_mul);
    register_demo!(registry, demo_u32_mod_mul);
    register_demo!(registry, demo_u64_mod_mul);
    register_demo!(registry, demo_usize_mod_mul);
    register_demo!(registry, demo_u8_mod_mul_assign);
    register_demo!(registry, demo_u16_mod_mul_assign);
    register_demo!(registry, demo_u32_mod_mul_assign);
    register_demo!(registry, demo_u64_mod_mul_assign);
    register_demo!(registry, demo_usize_mod_mul_assign);

    register_bench!(registry, None, benchmark_limbs_invert_limb_u32_algorithms);
    register_bench!(registry, None, benchmark_limbs_invert_limb_u64_algorithms);
    register_bench!(
        registry,
        None,
        benchmark_limbs_mod_preinverted_u8_algorithms
    );
    register_bench!(
        registry,
        None,
        benchmark_limbs_mod_preinverted_u16_algorithms
    );
    register_bench!(
        registry,
        None,
        benchmark_limbs_mod_preinverted_u32_algorithms
    );
    register_bench!(
        registry,
        None,
        benchmark_limbs_mod_preinverted_u64_algorithms
    );
    register_bench!(registry, None, benchmark_u8_mod_mul_algorithms);
    register_bench!(registry, None, benchmark_u16_mod_mul_algorithms);
    register_bench!(registry, None, benchmark_u32_mod_mul_algorithms);
    register_bench!(registry, None, benchmark_u64_mod_mul_algorithms);
    register_bench!(registry, None, benchmark_usize_mod_mul_algorithms);
    register_bench!(registry, None, benchmark_u8_mod_mul_assign);
    register_bench!(registry, None, benchmark_u16_mod_mul_assign);
    register_bench!(registry, None, benchmark_u32_mod_mul_assign);
    register_bench!(registry, None, benchmark_u64_mod_mul_assign);
    register_bench!(registry, None, benchmark_usize_mod_mul_assign);

    register_bench!(registry, None, benchmark_u8_mod_mul_precomputed_algorithms);
    register_bench!(registry, None, benchmark_u16_mod_mul_precomputed_algorithms);
    register_bench!(registry, None, benchmark_u32_mod_mul_precomputed_algorithms);
    register_bench!(registry, None, benchmark_u64_mod_mul_precomputed_algorithms);
    register_bench!(
        registry,
        None,
        benchmark_usize_mod_mul_precomputed_algorithms
    );
}

fn demo_limbs_invert_limb_u32(gm: GenerationMode, limit: usize) {
    for x in unsigneds_var_1::<u32>(gm).take(limit) {
        println!(
            "_limbs_invert_limb_u32({}) = {}",
            x,
            _limbs_invert_limb_u32(x)
        );
    }
}

fn demo_limbs_invert_limb_u64(gm: GenerationMode, limit: usize) {
    for x in unsigneds_var_1::<u64>(gm).take(limit) {
        println!(
            "_limbs_invert_limb_u64({}) = {}",
            x,
            _limbs_invert_limb_u64(x)
        );
    }
}

fn demo_limbs_mod_preinverted<
    T: PrimitiveUnsigned + Rand,
    DT: JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    gm: GenerationMode,
    limit: usize,
) where
    DT: From<T> + HasHalf<Half = T>,
    T: CheckedFrom<DT>,
{
    for (x_1, x_0, m, inv) in quadruples_of_unsigneds_var_2::<T, DT>(gm).take(limit) {
        println!(
            "_limbs_mod_preinverted({}, {}, {}, {}) = {}",
            x_1,
            x_0,
            m,
            inv,
            _limbs_mod_preinverted::<T, DT>(x_1, x_0, m, inv)
        );
    }
}

fn demo_mod_mul<T: PrimitiveUnsigned + Rand + SampleRange>(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_unsigneds_var_1::<T>(gm).take(limit) {
        println!("{} * {} === {} mod {}", x, y, x.mod_mul(y, m), m);
    }
}

fn demo_mod_mul_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut x, y, m) in triples_of_unsigneds_var_1::<T>(gm).take(limit) {
        let old_x = x;
        x.mod_mul_assign(y, m);
        println!("x := {}; x.mod_mul_assign({}, {}); x = {}", old_x, y, m, x);
    }
}

fn benchmark_limbs_invert_limb_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_invert_limb_u32(u32)",
        BenchmarkType::Algorithms,
        unsigneds_var_1::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&x| usize::exact_from((x - u32::power_of_two(u32::WIDTH - 1)).significant_bits())),
        "(x - (1 << 31)).significant_bits()",
        &mut [
            ("default", &mut (|x| no_out!(_limbs_invert_limb_u32(x)))),
            (
                "naive",
                &mut (|x| no_out!(_limbs_invert_limb_naive::<u32, u64>(x))),
            ),
        ],
    );
}

fn benchmark_limbs_invert_limb_u64_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_invert_limb_u64(u64)",
        BenchmarkType::Algorithms,
        unsigneds_var_1::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&x| usize::exact_from((x - u64::power_of_two(u64::WIDTH - 1)).significant_bits())),
        "(x - (1 << 63)).significant_bits()",
        &mut [
            ("default", &mut (|x| no_out!(_limbs_invert_limb_u64(x)))),
            (
                "naive",
                &mut (|x| no_out!(_limbs_invert_limb_naive::<u64, u128>(x))),
            ),
        ],
    );
}

fn benchmark_limbs_mod_preinverted_algorithms<
    T: PrimitiveUnsigned + Rand,
    DT: JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    DT: From<T> + HasHalf<Half = T>,
    T: CheckedFrom<DT>,
{
    m_run_benchmark(
        &format!(
            "_limbs_mod_preinverted({}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        quadruples_of_unsigneds_var_2::<T, DT>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x_1, x_0, _, _)| usize::exact_from(limbs_significant_bits(&[x_0, x_1]))),
        "x.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(x_1, x_0, d, d_inv)| {
                    no_out!(_limbs_mod_preinverted::<T, DT>(x_1, x_0, d, d_inv))
                }),
            ),
            (
                "naive",
                &mut (|(x_1, x_0, d, _)| {
                    no_out!(T::exact_from(DT::join_halves(x_1, x_0) % DT::from(d)))
                }),
            ),
        ],
    );
}

fn benchmark_mod_mul_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_mul({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        triples_of_unsigneds_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(x, y, m)| no_out!(x.mod_mul(y, m)))),
            ("naive", &mut (|(x, y, m)| no_out!(_naive_mod_mul(x, y, m)))),
        ],
    );
}

fn benchmark_mod_mul_algorithms_with_fast<
    T: PrimitiveUnsigned + Rand + SampleRange,
    DT: JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T: ModMulPrecomputed<Data = T>,
    DT: From<T> + HasHalf<Half = T>,
{
    m_run_benchmark(
        &format!("{}.mod_mul({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        triples_of_unsigneds_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(x, y, m)| no_out!(x.mod_mul(y, m)))),
            ("naive", &mut (|(x, y, m)| no_out!(_naive_mod_mul(x, y, m)))),
            (
                "fast",
                &mut (|(x, y, m)| {
                    no_out!(_fast_mod_mul::<T, DT>(
                        x,
                        y,
                        m,
                        T::precompute_mod_mul_data(&m)
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_mod_mul_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_mul_assign({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y, m)| x.mod_mul_assign(y, m)))],
    );
}

fn benchmark_mod_mul_precomputed_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_mul({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        triples_of_unsigneds_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "default",
                &mut (|(x, y, m)| {
                    for _ in 0..10 {
                        x.mod_mul(y, m);
                    }
                }),
            ),
            (
                "precomputed",
                &mut (|(x, y, m)| {
                    let data = T::precompute_mod_mul_data(&m);
                    for _ in 0..10 {
                        x.mod_mul_precomputed(y, m, &data);
                    }
                }),
            ),
        ],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_assign_name:ident,
        $bench_precomputed_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_mul::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_mul_assign::<$t>(gm, limit);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_mul_assign::<$t>(gm, limit, file_name);
        }

        fn $bench_precomputed_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_mul_precomputed_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_mod_mul,
    demo_u8_mod_mul_assign,
    benchmark_u8_mod_mul_assign,
    benchmark_u8_mod_mul_precomputed_algorithms
);
unsigned!(
    u16,
    demo_u16_mod_mul,
    demo_u16_mod_mul_assign,
    benchmark_u16_mod_mul_assign,
    benchmark_u16_mod_mul_precomputed_algorithms
);
unsigned!(
    u32,
    demo_u32_mod_mul,
    demo_u32_mod_mul_assign,
    benchmark_u32_mod_mul_assign,
    benchmark_u32_mod_mul_precomputed_algorithms
);
unsigned!(
    u64,
    demo_u64_mod_mul,
    demo_u64_mod_mul_assign,
    benchmark_u64_mod_mul_assign,
    benchmark_u64_mod_mul_precomputed_algorithms
);
unsigned!(
    usize,
    demo_usize_mod_mul,
    demo_usize_mod_mul_assign,
    benchmark_usize_mod_mul_assign,
    benchmark_usize_mod_mul_precomputed_algorithms
);

macro_rules! bench_fast {
    (
        $t:ident,
        $dt:ident,
        $bench_name:ident
    ) => {
        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_mul_algorithms_with_fast::<$t, $dt>(gm, limit, file_name);
        }
    };
}

macro_rules! bench_default {
    (
        $t:ident,
        $bench_name:ident
    ) => {
        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_mul_algorithms::<$t>(gm, limit, file_name);
        }
    };
}

bench_default!(u8, benchmark_u8_mod_mul_algorithms);
bench_default!(u16, benchmark_u16_mod_mul_algorithms);
bench_fast!(u32, u64, benchmark_u32_mod_mul_algorithms);
bench_fast!(u64, u128, benchmark_u64_mod_mul_algorithms);
bench_default!(u64, benchmark_usize_mod_mul_algorithms);

macro_rules! special {
    (
        $t:ident,
        $dt:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_limbs_mod_preinverted::<$t, $dt>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_limbs_mod_preinverted_algorithms::<$t, $dt>(gm, limit, file_name);
        }
    };
}

special!(
    u8,
    u16,
    demo_limbs_mod_preinverted_u8,
    benchmark_limbs_mod_preinverted_u8_algorithms
);
special!(
    u16,
    u32,
    demo_limbs_mod_preinverted_u16,
    benchmark_limbs_mod_preinverted_u16_algorithms
);
special!(
    u32,
    u64,
    demo_limbs_mod_preinverted_u32,
    benchmark_limbs_mod_preinverted_u32_algorithms
);
special!(
    u64,
    u128,
    demo_limbs_mod_preinverted_u64,
    benchmark_limbs_mod_preinverted_u64_algorithms
);
