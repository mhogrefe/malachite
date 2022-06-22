use malachite_base::num::arithmetic::mod_inverse::mod_inverse_binary;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::bench::bucketers::pair_2_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_38;
use malachite_base::test_util::num::arithmetic::mod_inverse::mod_inverse_euclidean;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_inverse);
    register_unsigned_signed_match_benches!(runner, benchmark_mod_inverse_algorithms);
}

fn demo_mod_inverse<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in unsigned_pair_gen_var_38::<T>().get(gm, &config).take(limit) {
        if let Some(inverse) = n.mod_inverse(m) {
            println!("{}⁻¹ ≡ {} mod {}", n, inverse, m);
        } else {
            println!("{} is not invertible mod {}", n, m);
        }
    }
}

fn benchmark_mod_inverse_algorithms<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_inverse({})", U::NAME, U::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_38::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bit_bucketer("m"),
        &mut [
            ("default", &mut |(n, m)| no_out!(n.mod_inverse(m))),
            ("Euclidean", &mut |(n, m)| {
                no_out!(mod_inverse_euclidean::<U, S>(n, m))
            }),
            ("binary", &mut |(n, m)| {
                no_out!(mod_inverse_binary::<U, S>(n, m))
            }),
        ],
    );
}
