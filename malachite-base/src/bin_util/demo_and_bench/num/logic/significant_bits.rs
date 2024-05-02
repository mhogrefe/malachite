// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_significant_bits_unsigned);
    register_signed_demos!(runner, demo_significant_bits_signed);
    register_unsigned_benches!(runner, benchmark_significant_bits_unsigned);
    register_signed_benches!(runner, benchmark_significant_bits_signed);
}

unsigned_single_arg_demo!(demo_significant_bits_unsigned, significant_bits);
signed_single_arg_demo!(demo_significant_bits_signed, significant_bits);

unsigned_single_arg_bench!(benchmark_significant_bits_unsigned, significant_bits);
signed_single_arg_bench!(benchmark_significant_bits_signed, significant_bits);
