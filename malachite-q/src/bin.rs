#[cfg(feature = "bin_build")]
extern crate itertools;
#[cfg(feature = "bin_build")]
#[macro_use]
extern crate malachite_base;
#[cfg(feature = "bin_build")]
extern crate malachite_nz;
#[cfg(feature = "bin_build")]
extern crate malachite_q;
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
use malachite_base::test_util::runner::cmd::read_command_line_arguments;
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::Runner;

// Examples:
//
// cargo run --release --features bin_build -- -l 10000 -m special_random -d demo_from_naturals
//      -c "mean_bits_n 128 mean_bits_d 1"
#[cfg(feature = "bin_build")]
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

#[cfg(not(feature = "bin_build"))]
fn main() {}

#[cfg(feature = "bin_build")]
pub mod bin_util {
    pub mod demo_and_bench;
}
