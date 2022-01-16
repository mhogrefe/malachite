#[macro_use]
extern crate malachite_base_test_util;
extern crate malachite_nz;
extern crate malachite_nz_test_util;
extern crate malachite_q;
extern crate serde;
extern crate serde_json;

use crate::demo_and_bench::register;
use malachite_base_test_util::runner::cmd::read_command_line_arguments;
use malachite_base_test_util::runner::Runner;

// Examples:
//
// cargo run -- -l 10000 -m special_random -d demo_from_naturals -c "mean_bits_n 128 mean_bits_d 1"
fn main() {
    let args = read_command_line_arguments("malachite-q test utils");
    let mut runner = Runner::new();
    register(&mut runner);
    if let Some(demo_key) = args.demo_key {
        runner.run_demo(&demo_key, args.generation_mode, args.config, args.limit);
    } else if let Some(bench_key) = args.bench_key {
        runner.run_bench(
            &bench_key,
            args.generation_mode,
            args.config,
            args.limit,
            &args.out,
        );
    } else {
        panic!();
    }
}

mod demo_and_bench;
