extern crate itertools;
#[macro_use]
extern crate malachite_base;
#[macro_use]
extern crate malachite_base_test_util;

use demo_and_bench::register;
use generate::max_base::generate_max_base;
use malachite_base_test_util::runner::cmd::read_command_line_arguments;
use malachite_base_test_util::runner::Runner;

// Examples:
//
// cargo run -- -g max_base
fn main() {
    let args = read_command_line_arguments("malachite-base test utils");
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
        let codegen_key = args.codegen_key.unwrap();
        match codegen_key.as_str() {
            "max_base" => generate_max_base(),
            _ => panic!("Invalid codegen key: {}", codegen_key),
        }
    }
}

mod demo_and_bench;
mod generate;
