use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;

use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, unsigned_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_get_highest_bit);
    register_signed_demos!(runner, demo_signed_get_highest_bit);
    register_unsigned_benches!(runner, benchmark_unsigned_get_highest_bit);
    register_signed_benches!(runner, benchmark_signed_get_highest_bit);
}

unsigned_single_arg_demo!(demo_unsigned_get_highest_bit, get_highest_bit);
signed_single_arg_demo!(demo_signed_get_highest_bit, get_highest_bit);

unsigned_single_arg_bench!(benchmark_unsigned_get_highest_bit, get_highest_bit);
signed_single_arg_bench!(benchmark_signed_get_highest_bit, get_highest_bit);
