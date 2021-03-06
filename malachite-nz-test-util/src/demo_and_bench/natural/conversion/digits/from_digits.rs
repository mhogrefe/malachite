use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base_test_util::bench::bucketers::triple_2_vec_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::conversion::digits::general_digits::{
    _from_digits_desc_naive_primitive, _limbs_from_digits_small_base_basecase,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::generators::unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_limbs_from_digits_small_base_basecase);
    register_unsigned_benches!(
        runner,
        benchmark_limbs_from_digits_small_base_basecase_algorithms
    );
}

fn demo_limbs_from_digits_small_base_basecase<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Limb: WrappingFrom<T>,
{
    for (mut out, xs, base) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2::<T, Limb>()
        .get(gm, &config)
        .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = _limbs_from_digits_small_base_basecase(&mut out, &xs, base);
        println!(
            "out := {:?}; _limbs_from_digits_small_base_basecase(&mut out, {:?}, {}) = {}; \
            out = {:?}",
            old_out, xs, base, out_len, out
        );
    }
}

fn benchmark_limbs_from_digits_small_base_basecase_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: WrappingFrom<T>,
    Natural: From<T>,
{
    run_benchmark(
        &format!(
            "_limbs_from_digits_small_base_basecase(&mut [Limb], &[{}], u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2::<T, Limb>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, base)| {
                no_out!(_limbs_from_digits_small_base_basecase(&mut out, &xs, base))
            }),
            ("naive", &mut |(_, xs, base)| {
                _from_digits_desc_naive_primitive(&xs, T::exact_from(base)).into_limbs_asc();
            }),
        ],
    );
}
