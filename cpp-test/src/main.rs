// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Differential testing of Malachite against FLINT, without FFI: each Malachite demo's output is
// captured to a text file, and a small C oracle (the sources in `oracle/`) recomputes every
// line with FLINT and fails on the first disagreement. The FLINT source is not part of this repository; a
// built FLINT source tree is located through the `MALACHITE_FLINT_DIR` environment variable,
// defaulting to `../../flint-3.6.0`. See README.md.

use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const MODES: [&str; 3] = ["exhaustive", "random", "special_random"];
const LIMIT: usize = 10000;
const TEST_OUT: &str = "test-out.txt";

fn flint_dir() -> PathBuf {
    let dir = env::var("MALACHITE_FLINT_DIR").unwrap_or_else(|_| "../../flint-3.6.0".to_string());
    fs::canonicalize(&dir).unwrap_or_else(|_| {
        panic!(
            "no built FLINT tree at {dir}; build one (bootstrap.sh, configure, make) or set \
            MALACHITE_FLINT_DIR"
        )
    })
}

// Compiles the oracle sources in `oracle/` against the FLINT tree if the binary is missing or
// stale (judged against the newest source or header), returning
// the binary's path.
fn build_oracle() -> PathBuf {
    let flint = flint_dir();
    let binary = Path::new("target").join("flint-oracle");
    let mut sources = Vec::new();
    let mut newest = None;
    for entry in fs::read_dir("oracle").unwrap() {
        let path = entry.unwrap().path();
        match path.extension().and_then(|e| e.to_str()) {
            Some("c") => sources.push(path.clone()),
            Some("h") => {}
            _ => continue,
        }
        let modified = fs::metadata(&path).unwrap().modified().unwrap();
        newest = Some(newest.map_or(modified, |n: std::time::SystemTime| n.max(modified)));
    }
    sources.sort();
    let stale = match (fs::metadata(&binary), newest) {
        (Ok(bin), Some(newest)) => newest >= bin.modified().unwrap(),
        _ => true,
    };
    if stale {
        fs::create_dir_all("target").unwrap();
        let status = Command::new("cc")
            .arg("-O2")
            .arg("-Wall")
            .arg("-Wextra")
            .args(&sources)
            .arg("-o")
            .arg(&binary)
            .arg(format!(
                "-I{}",
                flint.join("build").join("include").display()
            ))
            .arg(format!("-L{}", flint.display()))
            .arg(format!("-Wl,-rpath,{}", flint.display()))
            .arg("-lflint")
            .status()
            .expect("failed to run cc");
        assert!(status.success(), "failed to build flint-oracle");
    }
    binary
}

fn run_oracle(oracle: &Path, mode: &str, input: Option<&str>) {
    let mut command = Command::new(oracle);
    command.arg(mode);
    if let Some(input) = input {
        command.arg(input);
    }
    let output = command.output().expect("failed to run the FLINT oracle");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(
        output.status.success(),
        "FLINT oracle failed in mode {mode}: {:?}",
        output.status
    );
}

// Runs a Malachite demo in the given generator mode, capturing its output to `TEST_OUT`.
fn run_demo(crate_dir: &str, demo_name: &str, mode: &str) {
    let output_file = File::create(TEST_OUT).unwrap();
    let mut command = Command::new("cargo");
    command
        .arg("run")
        .arg("--release")
        .arg("-j")
        .arg("4")
        .arg("--features")
        .arg("bin_build")
        .arg("--")
        .arg("-l")
        .arg(format!("{LIMIT}"))
        .arg("-m")
        .arg(mode)
        .arg("-d")
        .arg(demo_name);
    command.current_dir(crate_dir);
    command.stdout(Stdio::from(output_file));
    let output = command.output().expect("failed to run Malachite demo");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success(), "demo {demo_name} failed");
}

// Runs a Malachite demo in every generator mode, diffing each run's output against FLINT.
fn check_demo_against_flint(oracle: &Path, crate_dir: &str, demo_name: &str, flint_mode: &str) {
    for mode in MODES {
        println!("testing {demo_name} in mode {mode}");
        run_demo(crate_dir, demo_name, mode);
        run_oracle(oracle, flint_mode, Some(TEST_OUT));
    }
}

fn write_primitive_root_prime_unit_test(output_file: &mut File, n: u64, out: u64) {
    writeln!(output_file, "primitive_root_prime({n}) = {out}").unwrap();
}

fn main() {
    let oracle = build_oracle();

    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_mod_sqrt",
        "fmpz_sqrtmod",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-base",
        "demo_mod_sqrt_u64",
        "n_sqrtmod",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_mod_div",
        "fmpz_mod_divides",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-base",
        "demo_mod_div_u64",
        "fmpz_mod_divides",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_mod_div_list",
        "fmpz_divides_mod_list",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-base",
        "demo_mod_div_list_u64",
        "fmpz_divides_mod_list",
    );
    check_demo_against_flint(&oracle, "../malachite-nz", "demo_natural_crt", "fmpz_CRT");
    check_demo_against_flint(&oracle, "../malachite-base", "demo_crt_u64", "fmpz_CRT");
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integer_balanced_crt",
        "fmpz_CRT_balanced",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_multi_crt",
        "fmpz_multi_CRT",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integer_multi_balanced_crt",
        "fmpz_multi_CRT_balanced",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_crt_comb_reduce",
        "fmpz_multi_mod_ui",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_crt_comb_combine",
        "fmpz_multi_CRT_ui",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_crt_comb_combine_balanced",
        "fmpz_multi_CRT_ui_balanced",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_rising_factorial",
        "fmpz_rfac",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integer_rising_factorial",
        "fmpz_rfac",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_extended_gcd_partial",
        "fmpz_xgcd_partial",
    );

    println!("testing primitive_root_prime unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        write_primitive_root_prime_unit_test(&mut output_file, 2, 1);
        write_primitive_root_prime_unit_test(&mut output_file, 3, 2);
        write_primitive_root_prime_unit_test(&mut output_file, 5, 2);
        write_primitive_root_prime_unit_test(&mut output_file, 7, 3);
        write_primitive_root_prime_unit_test(&mut output_file, 11, 2);
        write_primitive_root_prime_unit_test(&mut output_file, 191, 19);
        write_primitive_root_prime_unit_test(&mut output_file, 9223372036854775807, 2);
        write_primitive_root_prime_unit_test(&mut output_file, 8760810010780182161, 3);
    }
    run_oracle(&oracle, "n_primitive_root_prime", Some(TEST_OUT));

    // Edge rows for the multi-modulus CRT quirks: a single modulus of 1 is usable, but a 1 or a
    // 0 among two or more moduli is not, and unusable lists must report None.
    println!("testing multi_crt unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "multi_crt([5], [3]) = Some(3)",
            "multi_crt([1], [0]) = Some(0)",
            "multi_crt([0], [0]) = None",
            "multi_crt([3, 5, 7], [2, 3, 2]) = Some(23)",
            "multi_crt([4, 6], [1, 3]) = None",
            "multi_crt([5, 5], [1, 1]) = None",
            "multi_crt([3, 1], [2, 0]) = None",
            "multi_crt([0, 3], [0, 2]) = None",
            "multi_crt_balanced([5], [3]) = Some(-2)",
            "multi_crt_balanced([3, 5], [2, 3]) = Some(-7)",
            "multi_crt_balanced([2, 7], [1, 0]) = Some(7)",
            "multi_crt_balanced([4, 6], [1, 3]) = None",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_multi_CRT", Some(TEST_OUT));
    run_oracle(&oracle, "fmpz_multi_CRT_balanced", Some(TEST_OUT));

    for demo_name in [
        "demo_u8_primitive_root_prime",
        "demo_u16_primitive_root_prime",
        "demo_u32_primitive_root_prime",
        "demo_u64_primitive_root_prime",
        "demo_usize_primitive_root_prime",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-base",
            demo_name,
            "n_primitive_root_prime",
        );
    }
}
