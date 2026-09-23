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

// For demos whose generators do not support the special_random mode, such as those built on
// primitive-integer generators declared with new_no_special.
fn check_demo_against_flint_no_special(
    oracle: &Path,
    crate_dir: &str,
    demo_name: &str,
    flint_mode: &str,
) {
    for mode in &MODES[..2] {
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
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_to_height",
        "fmpq_height",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_into_height",
        "fmpq_height",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_height_significant_bits",
        "fmpq_height_bits",
    );
    check_demo_against_flint(&oracle, "../malachite-q", "demo_rational_gcd", "fmpq_gcd");
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_extended_gcd",
        "fmpq_gcd_cofactors",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_farey_neighbors",
        "fmpq_farey_neighbors",
    );
    check_demo_against_flint_no_special(
        &oracle,
        "../malachite-q",
        "demo_rational_harmonic_number",
        "fmpq_harmonic",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_dedekind_sum",
        "fmpq_dedekind_sum",
    );
    check_demo_against_flint_no_special(
        &oracle,
        "../malachite-nz",
        "demo_natural_bell_number",
        "arith_bell_number",
    );
    check_demo_against_flint_no_special(
        &oracle,
        "../malachite-nz",
        "demo_natural_bell_numbers_prefix",
        "arith_bell_number_vec",
    );
    check_demo_against_flint_no_special(
        &oracle,
        "../malachite-nz",
        "demo_natural_landau_function_prefix",
        "arith_landau_function_vec",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_exhaustive_by_height",
        "fmpq_next_minimal",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_exhaustive_signed_by_height",
        "fmpq_next_signed_minimal",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct",
        "fmpq_reconstruct",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_ref",
        "fmpq_reconstruct",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_with_bounds",
        "fmpq_reconstruct_2",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_with_bounds_ref",
        "fmpq_reconstruct_2",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_tier_rows",
        "fmpq_reconstruct",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_tier_rows",
        "fmpq_reconstruct_2",
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

    // Every case from test_balanced_mod in malachite-nz's IntegerPolynomial tests, including the
    // negative moduli, which the oracle passes to FLINT as their absolute values.
    println!("testing IntegerPolynomial balanced_mod unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "(0).balanced_mod(1) = 0",
            "(0).balanced_mod(-7) = 0",
            "(x^2+27*x-23).balanced_mod(1) = 0",
            "(x^2+27*x-23).balanced_mod(-1) = 0",
            "(x^2+27*x-23).balanced_mod(10) = x^2-3*x-3",
            "(x^2+27*x-23).balanced_mod(-10) = x^2-3*x-3",
            "(5*x-5).balanced_mod(10) = 5*x+5",
            "(x^2+x+1).balanced_mod(2) = x^2+x+1",
            "(-x^2-x-1).balanced_mod(2) = x^2+x+1",
            "(2*x-2).balanced_mod(5) = 2*x-2",
            "(3*x-3).balanced_mod(5) = -2*x+2",
            "(10*x^2+7*x+5).balanced_mod(-10) = -3*x+5",
            "(-6*x^2-3*x-9).balanced_mod(3) = 0",
            "(x^3-6*x^2+2).balanced_mod(3) = x^3-1",
            "(1000000000000000000000000*x+1).balanced_mod(1234567890987) = 530068894399*x+1",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_poly_scalar_smod_fmpz", Some(TEST_OUT));
    // The generated cases from balanced_mod_properties: the (polynomial, nonzero modulus) pairs,
    // by value, by reference, and in place, and every polynomial against the moduli 1, -1, and 2.
    for demo_name in [
        "demo_integer_polynomial_balanced_mod",
        "demo_integer_polynomial_balanced_mod_ref",
        "demo_integer_polynomial_balanced_mod_assign",
        "demo_integer_polynomial_balanced_mod_small_moduli",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_scalar_smod_fmpz",
        );
    }

    // Every case from test_mod_op in malachite-nz's IntegerPolynomial tests, and the
    // u128 cases from test_mod_op_unsigned, whose modulus is too wide for an nmod_poly.
    println!("testing IntegerPolynomial mod_op unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "(0).mod_op(1) = 0",
            "(0).mod_op(7) = 0",
            "(x^2-4*x-5).mod_op(1) = 0",
            "(x^2-4*x-5).mod_op(3) = x^2+2*x+1",
            "(-1).mod_op(10) = 9",
            "(-x).mod_op(18446744073709551616) = 18446744073709551615*x",
            "(x^2+4*x+5).mod_op(3) = x^2+x+2",
            "(-6*x+1).mod_op(3) = 1",
            "(-6*x^2+3*x-1).mod_op(3) = 2",
            "(-6*x^2-3*x-9).mod_op(3) = 0",
            "(x^3-6*x^2+2).mod_op(3) = x^3+2",
            "(-1000000000000000000000000*x+1).mod_op(1234567890987) = 704498996588*x+1",
            "(x^2-4*x+5).mod_op(1000000000000000000000000) = x^2+999999999999999999999996*x+5",
            "(-x).mod_op(340282366920938463463374607431768211455) = 340282366920938463463374607431768211454*x",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_poly_scalar_mod_fmpz", Some(TEST_OUT));
    // The generated cases from mod_op_properties, and the u128 ones from
    // mod_op_unsigned_properties.
    for demo_name in [
        "demo_integer_polynomial_mod_op",
        "demo_integer_polynomial_mod_op_ref",
        "demo_integer_polynomial_mod_op_power_of_2_moduli",
        "demo_integer_polynomial_mod_op_unsigned_u128",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_scalar_mod_fmpz",
        );
    }

    // The cases from test_mod_op_unsigned whose modulus fits in a word.
    println!("testing IntegerPolynomial mod_op by a word unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "(0).mod_op(1) = 0",
            "(0).mod_op(7) = 0",
            "(x^2-4*x-5).mod_op(3) = x^2+2*x+1",
            "(x^2-4*x-5).mod_op(1) = 0",
            "(-1).mod_op(255) = 254",
            "(-1).mod_op(18446744073709551615) = 18446744073709551614",
            "(-1000000000001*x^2+2000000000003*x-5).mod_op(7) = 5*x^2+5*x+2",
            "(-100000000000000000000*x+1).mod_op(18446744073709551615) = 10680464442257309690*x+1",
            "(-1024*x^2-3).mod_op(4) = 1",
            "(-4294967296*x^2+4294967296).mod_op(65536) = 0",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_poly_get_nmod_poly", Some(TEST_OUT));
    // The generated cases from mod_op_unsigned_properties, for every word-sized type.
    for demo_name in [
        "demo_integer_polynomial_mod_op_unsigned_u8",
        "demo_integer_polynomial_mod_op_unsigned_u16",
        "demo_integer_polynomial_mod_op_unsigned_u32",
        "demo_integer_polynomial_mod_op_unsigned_u64",
        "demo_integer_polynomial_mod_op_unsigned_usize",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_get_nmod_poly",
        );
    }

    // Every case from test_rem, test_rem_lowers_the_degree, test_rem_unsigned, and test_mod_op
    // in malachite-nz's NaturalPolynomial tests.
    println!("testing NaturalPolynomial rem unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "(0) % 1 = 0",
            "(0) % 7 = 0",
            "(x^2+4*x+5) % 3 = x^2+x+2",
            "(x^2+4*x+5) % 7 = x^2+4*x+5",
            "(x^2+4*x+5) % 1 = 0",
            "(1000000000001*x^2+2000000000003*x+5) % 1000000000000 = x^2+3*x+5",
            "(1000000000000000000000000*x+1) % 1234567890987 = 530068894399*x+1",
            "(x^2+4*x+5) % 1000000000000000000000000 = x^2+4*x+5",
            "(4*x^2+3) % 4 = 3",
            "(6*x^2+3*x+2) % 3 = 2",
            "(6*x^2+3*x+9) % 3 = 0",
            "(x^3+6*x^2+2) % 3 = x^3+2",
            "(5*x^5+10*x^4+15*x^3+1) % 5 = 1",
            "(2000000000000*x^2+1000000000000*x+7) % 1000000000000 = 7",
            "(0) % 1 = 0",
            "(0) % 7 = 0",
            "(x^2+4*x+5) % 3 = x^2+x+2",
            "(x^2+4*x+5) % 1 = 0",
            "(1000000000001*x^2+2000000000003*x+5) % 1000 = x^2+3*x+5",
            "(1000000000001*x^2+2000000000003*x+5) % 7 = 2*x^2+5*x+5",
            "(256*x^2+257*x+3) % 255 = x^2+2*x+3",
            "(100000000000000000000*x+1) % 18446744073709551615 = 7766279631452241925*x+1",
            "(1000000000000000000000000000000000000000*x) % 340282366920938463463374607431768211455 = 319435266158123073073250785136463577090*x",
            "(1024*x^2+3) % 4 = 3",
            "(4294967296*x^2+4294967296) % 65536 = 0",
            "(0).mod_op(1) = 0",
            "(x^2+4*x+5).mod_op(3) = x^2+x+2",
            "(x^2+4*x+5).mod_op(1) = 0",
            "(4*x^2+3).mod_op(4) = 3",
            "(1000000000000000000000000*x+1).mod_op(1234567890987) = 530068894399*x+1",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_mod_poly_set_fmpz_poly", Some(TEST_OUT));
    // The generated cases from rem_properties and rem_unsigned_properties, in every form.
    for demo_name in [
        "demo_natural_polynomial_rem",
        "demo_natural_polynomial_rem_ref",
        "demo_natural_polynomial_rem_assign",
        "demo_natural_polynomial_mod_op",
        "demo_natural_polynomial_mod_assign",
        "demo_natural_polynomial_rem_special_moduli",
        "demo_natural_polynomial_rem_unsigned_u8",
        "demo_natural_polynomial_rem_unsigned_u16",
        "demo_natural_polynomial_rem_unsigned_u32",
        "demo_natural_polynomial_rem_unsigned_u64",
        "demo_natural_polynomial_rem_unsigned_u128",
        "demo_natural_polynomial_rem_unsigned_usize",
        "demo_natural_polynomial_rem_unsigned_ref_u8",
        "demo_natural_polynomial_rem_unsigned_ref_u16",
        "demo_natural_polynomial_rem_unsigned_ref_u32",
        "demo_natural_polynomial_rem_unsigned_ref_u64",
        "demo_natural_polynomial_rem_unsigned_ref_u128",
        "demo_natural_polynomial_rem_unsigned_ref_usize",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_mod_poly_set_fmpz_poly",
        );
    }

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
