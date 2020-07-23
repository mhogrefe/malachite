extern crate malachite_base;
#[macro_use]
extern crate malachite_base_test_util;

use demo_and_bench::register;
use malachite_base_test_util::runner::cmd::read_command_line_arguments;
use malachite_base_test_util::runner::Runner;

fn main() {
    let args = read_command_line_arguments();
    let mut runner = Runner::new();
    register(&mut runner);
    if let Some(demo_key) = args.demo_key {
        runner.run_demo(&demo_key, args.generation_mode, args.config, args.limit);
    } else {
        runner.run_bench(
            &args.bench_key.unwrap(),
            args.generation_mode,
            args.config,
            args.limit,
            &args.out,
        );
    }
}

pub mod demo_and_bench;
