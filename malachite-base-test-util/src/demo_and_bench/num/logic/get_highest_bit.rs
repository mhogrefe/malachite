use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, unsigned_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_get_highest_bit_unsigned);
    register_signed_demos!(runner, demo_get_highest_bit_signed);
    register_unsigned_benches!(runner, benchmark_get_highest_bit_unsigned);
    register_signed_benches!(runner, benchmark_get_highest_bit_signed);
}

unsigned_single_arg_demo!(demo_get_highest_bit_unsigned, get_highest_bit);
signed_single_arg_demo!(demo_get_highest_bit_signed, get_highest_bit);

unsigned_single_arg_bench!(benchmark_get_highest_bit_unsigned, get_highest_bit);
signed_single_arg_bench!(benchmark_get_highest_bit_signed, get_highest_bit);
