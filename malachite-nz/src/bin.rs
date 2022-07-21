#[cfg(feature = "bin_build")]
extern crate itertools;
#[cfg(feature = "bin_build")]
#[macro_use]
extern crate malachite_base;
#[cfg(feature = "bin_build")]
extern crate malachite_nz;
#[cfg(feature = "bin_build")]
extern crate num;
#[cfg(feature = "bin_build")]
extern crate rug;
#[cfg(feature = "bin_build")]
extern crate serde;
#[cfg(feature = "bin_build")]
extern crate serde_json;

#[cfg(feature = "bin_build")]
use crate::bin_util::demo_and_bench::register;
#[cfg(feature = "bin_build")]
use crate::bin_util::generate::digits_data::generate_string_data;
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::cmd::read_command_line_arguments;
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::Runner;

// Examples:
//
// cargo run --release --features bin_build -- -l 100000 -m special_random -d
//     demo_natural_from_unsigned_u128 -c "mean_run_length_n 4 mean_run_length_d 1"
//
// cargo run --release --features bin_build -- -l 100000 -m random -b
//     benchmark_limbs_to_digits_small_base_algorithms
//
// cargo run -- -g digits_data
#[cfg(feature = "bin_build")]
fn main() {
    let args = read_command_line_arguments("malachite-nz test utils");
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
            "digits_data" => generate_string_data(),
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
