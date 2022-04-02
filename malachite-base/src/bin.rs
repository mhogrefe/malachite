extern crate itertools;
#[macro_use]
extern crate malachite_base;
extern crate walkdir;

use bin_util::demo_and_bench::register;
use bin_util::generate::max_base::generate_max_base;
use bin_util::generate::tuning_manager::{build_reference_data, test};
use malachite_base::test_util::runner::cmd::read_command_line_arguments;
use malachite_base::test_util::runner::Runner;

// Examples:
//
// cargo run --features test_build -- -g max_base
// cargo run --features test_build --release -- -l 10000 -m exhaustive -b demo_mod_pow_u32
fn main() {
    let args = read_command_line_arguments("malachite-base");
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
            "tm_build_reference_data" => build_reference_data(),
            "tm_test" => test(),
            _ => panic!("Invalid codegen key: {}", codegen_key),
        }
    }
}

pub mod bin_util {
    pub mod demo_and_bench;
    pub mod generate;
}
