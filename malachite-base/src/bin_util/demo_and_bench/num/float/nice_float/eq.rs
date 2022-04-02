use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::pair_max_primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_nice_float_eq);
    register_primitive_float_benches!(runner, benchmark_nice_float_eq_algorithms);
}

fn demo_nice_float_eq<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in primitive_float_pair_gen::<T>().get(gm, &config).take(limit) {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_nice_float_eq_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("NiceFloat<{}> == NiceFloat<{}>", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_primitive_float_bucketer("f", "g"),
        &mut [
            ("Malachite", &mut |(x, y)| {
                no_out!(NiceFloat(x) == NiceFloat(y))
            }),
            ("Rust default", &mut |(x, y)| no_out!(x == y)),
        ],
    );
}
