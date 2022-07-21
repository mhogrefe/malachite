#[cfg(feature = "bin_build")]
extern crate itertools;
#[cfg(feature = "bin_build")]
#[macro_use]
extern crate malachite_base;
#[cfg(feature = "bin_build")]
extern crate walkdir;

#[cfg(feature = "bin_build")]
use crate::bin_util::demo_and_bench::register;
#[cfg(feature = "bin_build")]
use crate::bin_util::generate::max_base::generate_max_base;
#[cfg(feature = "bin_build")]
use crate::bin_util::generate::tuning_manager::{build_reference_data, test};
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::cmd::read_command_line_arguments;
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::Runner;
#[cfg(feature = "bin_build")]
// Examples:
//
// cargo run --features bin_build -- -g max_base
// cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_mod_pow_u32
#[cfg(feature = "bin_build")]
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

#[cfg(not(feature = "bin_build"))]
fn main() {}

#[cfg(feature = "bin_build")]
pub mod bin_util {
    pub mod demo_and_bench;
    pub mod generate;
}
