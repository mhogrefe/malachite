use malachite_base::num::arithmetic::xx_div_mod_y_is_qr::explicit_xx_div_mod_y_is_qr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::triple_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_triple_gen_var_15;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_xx_div_mod_y_is_qr);
    register_unsigned_benches!(runner, benchmark_xx_div_mod_y_is_qr_algorithms);
}

fn demo_xx_div_mod_y_is_qr<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x_1, x_0, y) in unsigned_triple_gen_var_15::<T, T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "[{}, {}].div_mod({}) = {:?}",
            x_1,
            x_0,
            y,
            T::xx_div_mod_y_is_qr(x_1, x_0, y)
        );
    }
}

fn benchmark_xx_div_mod_y_is_qr_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::xx_div_mod_y_is_qr({}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_15::<T, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x_1", "x_0", "y"),
        &mut [
            ("default", &mut |(x_1, x_0, y)| {
                no_out!(T::xx_div_mod_y_is_qr(x_1, x_0, y))
            }),
            ("explicit", &mut |(x_1, x_0, y)| {
                no_out!(explicit_xx_div_mod_y_is_qr(x_1, x_0, y))
            }),
        ],
    );
}
