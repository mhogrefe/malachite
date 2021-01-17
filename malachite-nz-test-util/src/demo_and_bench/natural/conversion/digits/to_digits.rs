use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, SaturatingFrom};
use malachite_base_test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, quadruple_3_vec_len_bucketer, triple_3_vec_len_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::conversion::digits::general_digits::{
    _limbs_to_digits_basecase, _limbs_to_digits_small_base, _limbs_to_digits_small_base_basecase,
    _to_digits_asc_naive,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_to_digits_small_base_basecase);
    register_demo!(runner, demo_limbs_to_digits_small_base);
    register_unsigned_demos!(runner, demo_limbs_to_digits_basecase);

    register_bench!(
        runner,
        benchmark_limbs_to_digits_small_base_basecase_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_to_digits_small_base_basecase_algorithms_2
    );
    register_bench!(runner, benchmark_limbs_to_digits_small_base_algorithms);
    register_unsigned_benches!(runner, benchmark_limbs_to_digits_basecase);
}

fn demo_limbs_to_digits_small_base_basecase(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, len, xs, base) in
        unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1()
            .get(gm, &config)
            .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = _limbs_to_digits_small_base_basecase(&mut out, len, &xs, base);
        println!(
            "out := {:?}; _limbs_to_digits_small_base_basecase(&mut out, {}, {:?}, {}) = {}; \
            out = {:?}",
            old_out, len, xs, base, out_len, out
        );
    }
}

fn demo_limbs_to_digits_small_base(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, base, mut xs) in unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = _limbs_to_digits_small_base(&mut out, base, &mut xs, None);
        println!(
            "out := {:?}; _limbs_to_digits_small_base(&mut out, {}, {:?}) = {}; out = {:?}",
            old_out, base, xs, out_len, out
        );
    }
}

fn demo_limbs_to_digits_basecase<T: ConvertibleFrom<u64> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    u64: SaturatingFrom<T>,
{
    for (mut xs, base) in unsigned_vec_unsigned_pair_gen_var_1::<Limb, T>()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        println!(
            "_limbs_to_digits_basecase(&{:?}, {}) = {:?}",
            xs_old,
            base,
            _limbs_to_digits_basecase::<T>(&mut xs, base)
        );
    }
}

fn benchmark_limbs_to_digits_small_base_basecase_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_to_digits_small_base_basecase(&mut [u8], usize, &[Limb], u64)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, len, xs, base)| {
                no_out!(_limbs_to_digits_small_base_basecase(
                    &mut out, len, &xs, base
                ))
            }),
            ("naive", &mut |(_, _, xs, base)| {
                no_out!(_to_digits_asc_naive::<u8, _>(
                    &Natural::from_owned_limbs_asc(xs),
                    base
                ))
            }),
        ],
    );
}

fn benchmark_limbs_to_digits_small_base_basecase_algorithms_2(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_to_digits_small_base_basecase(&mut [u8], usize, &[Limb], u64)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("xs"),
        &mut [
            ("_limbs_to_digits_small_base_basecase", &mut |(
                mut out,
                _,
                xs,
                base,
            )| {
                no_out!(_limbs_to_digits_small_base_basecase(&mut out, 0, &xs, base))
            }),
            ("_limbs_to_digits_small_base", &mut |(
                mut out,
                _,
                mut xs,
                base,
            )| {
                no_out!(_limbs_to_digits_small_base(&mut out, base, &mut xs, None))
            }),
        ],
    );
}

fn benchmark_limbs_to_digits_small_base_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_to_digits_small_base(&mut [u8], u64, &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(mut out, base, mut xs)| {
                no_out!(_limbs_to_digits_small_base(&mut out, base, &mut xs, None))
            }),
            ("naive", &mut |(_, base, xs)| {
                no_out!(_to_digits_asc_naive::<u8, _>(
                    &Natural::from_owned_limbs_asc(xs),
                    base
                ))
            }),
        ],
    );
}

fn benchmark_limbs_to_digits_basecase<T: ConvertibleFrom<u64> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    u64: SaturatingFrom<T>,
{
    run_benchmark(
        "_limbs_to_digits_basecase(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_1::<Limb, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, base)| {
            no_out!(_limbs_to_digits_basecase::<T>(&mut xs, base))
        })],
    );
}
