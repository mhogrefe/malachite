use malachite_base::num::arithmetic::x_mul_y_is_zz::explicit_x_mul_y_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_pair_gen_var_27;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_x_mul_y_is_zz);
    register_unsigned_benches!(runner, benchmark_x_mul_y_is_zz_algorithms);
}

fn demo_x_mul_y_is_zz<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_27::<T>().get(gm, &config).take(limit) {
        println!("{} * {} = {:?}", x, y, T::x_mul_y_is_zz(x, y));
    }
}

fn benchmark_x_mul_y_is_zz_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::x_mul_y_is_zz({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_27::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(T::x_mul_y_is_zz(x, y))),
            ("explicit", &mut |(x, y)| {
                no_out!(explicit_x_mul_y_is_zz(x, y))
            }),
        ],
    );
}
