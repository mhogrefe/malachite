use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::string_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{string_gen, string_gen_var_10};
use malachite_base_test_util::runner::Runner;
use std::fmt::Debug;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_nice_float_from_str);
    register_primitive_float_demos!(runner, demo_nice_float_from_str_targeted);
    register_primitive_float_benches!(runner, benchmark_nice_float_from_str);
}

fn demo_nice_float_from_str<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    <T as FromStr>::Err: Debug,
{
    for s in string_gen().get(gm, &config).take(limit) {
        println!(
            "NiceFloat::from_str({:?}) = {:?}",
            s,
            NiceFloat::<T>::from_str(&s)
        );
    }
}

fn demo_nice_float_from_str_targeted<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    <T as FromStr>::Err: Debug,
{
    for s in string_gen_var_10().get(gm, &config).take(limit) {
        println!(
            "NiceFloat::from_str({:?}) = {:?}",
            s,
            NiceFloat::<T>::from_str(&s)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_nice_float_from_str<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("NiceFloat::<{}>::from_str(&str)", T::NAME),
        BenchmarkType::Single,
        string_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| no_out!(NiceFloat::<T>::from_str(&s)))],
    );
}
