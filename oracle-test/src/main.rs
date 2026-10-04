// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Differential testing of Malachite against FLINT and Azurite, without FFI: each Malachite demo's
// output is captured to a text file, and an oracle recomputes every line and fails on the first
// disagreement. The FLINT oracle is a small C program (the sources in `oracle/`) built against a
// FLINT source tree located through the `MALACHITE_FLINT_DIR` environment variable, defaulting to
// `../../flint-3.6.0`. The Azurite oracle is the `oracle` executable of the Azurite repository
// (formally verified Lean arithmetic), located through `MALACHITE_AZURITE_DIR`, defaulting to
// `../../../azurite`. `cargo run --release -- flint` or `-- azurite` runs one backend; with no
// argument both run. See README.md.

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
    run_demo_with_limit(crate_dir, demo_name, mode, LIMIT);
}

// Like `run_demo`, but with a line limit other than `LIMIT`, for demos whose lines are long.
fn run_demo_with_limit(crate_dir: &str, demo_name: &str, mode: &str, limit: usize) {
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
        .arg(format!("{limit}"))
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

// Like `check_demo_against_flint`, but with a line limit other than `LIMIT`, for demos whose
// lines are long.
fn check_demo_against_flint_with_limit(
    oracle: &Path,
    crate_dir: &str,
    demo_name: &str,
    flint_mode: &str,
    limit: usize,
) {
    for mode in MODES {
        println!("testing {demo_name} in mode {mode}");
        run_demo_with_limit(crate_dir, demo_name, mode, limit);
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

// Checks every unit-row file under `unit/<backend>/`: each `<mode>/<name>.txt` holds lines in the
// format the demos print, mostly the rows of a `test_*` function in Malachite's own tests, and is
// handed to the oracle's mode `<mode>` as it is. Directories and files are visited in sorted order.
fn run_unit_files(backend: &str, mut run: impl FnMut(&str, &str)) {
    let root = Path::new("unit").join(backend);
    let mut mode_dirs: Vec<PathBuf> = fs::read_dir(&root)
        .unwrap_or_else(|_| panic!("no unit-row directory {}", root.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect();
    mode_dirs.sort();
    for mode_dir in mode_dirs {
        let mode = mode_dir.file_name().unwrap().to_str().unwrap().to_string();
        let mut files: Vec<PathBuf> = fs::read_dir(&mode_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("txt"))
            .collect();
        files.sort();
        for file in files {
            println!("testing unit rows {} in mode {mode}", file.display());
            run(&mode, file.to_str().unwrap());
        }
    }
}

fn azurite_dir() -> PathBuf {
    let dir = env::var("MALACHITE_AZURITE_DIR").unwrap_or_else(|_| "../../../azurite".to_string());
    fs::canonicalize(&dir).unwrap_or_else(|_| {
        panic!("no Azurite checkout at {dir}; clone github.com/mhogrefe/azurite or set MALACHITE_AZURITE_DIR")
    })
}

// Builds Azurite's `oracle` executable with `lake` (a no-op when it is up to date), returning its
// path.
fn build_azurite_oracle() -> PathBuf {
    let azurite = azurite_dir();
    let status = Command::new("lake")
        .arg("build")
        .arg("oracle")
        .env("LEAN_NUM_THREADS", "4")
        .current_dir(&azurite)
        .status()
        .expect("failed to run lake; is the Lean toolchain (elan) installed?");
    assert!(status.success(), "failed to build the Azurite oracle");
    let binary = azurite
        .join(".lake")
        .join("build")
        .join("bin")
        .join("oracle");
    assert!(
        binary.exists(),
        "lake built the Azurite oracle but {binary:?} is missing"
    );
    binary
}

fn run_azurite_oracle(oracle: &Path, mode: &str, input: &str) {
    let output = Command::new(oracle)
        .arg(mode)
        .arg(input)
        .output()
        .expect("failed to run the Azurite oracle");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(
        output.status.success(),
        "Azurite oracle failed in mode {mode}: {:?}",
        output.status
    );
}

// Runs a Malachite demo in the given generator modes, checking each run's output against Azurite.
fn check_demo_against_azurite_in_modes(
    oracle: &Path,
    crate_dir: &str,
    demo_name: &str,
    azurite_mode: &str,
    modes: &[&str],
) {
    for mode in modes {
        println!("testing {demo_name} in mode {mode} against Azurite");
        run_demo(crate_dir, demo_name, mode);
        run_azurite_oracle(oracle, azurite_mode, TEST_OUT);
    }
}

// Runs a Malachite demo in every generator mode, checking each run's output against Azurite.
fn check_demo_against_azurite(oracle: &Path, crate_dir: &str, demo_name: &str, azurite_mode: &str) {
    check_demo_against_azurite_in_modes(oracle, crate_dir, demo_name, azurite_mode, &MODES);
}

// The Natural demos checked against Azurite, with the oracle mode that reads each one. The modes
// follow the rows of the "Malachite for Azurite Users: Naturals" mapping page.
const AZURITE_NATURAL_STAGES: &[(&str, &str)] = &[
    ("demo_natural_add", "az_nat_add"),
    ("demo_natural_add_val_ref", "az_nat_add"),
    ("demo_natural_add_ref_val", "az_nat_add"),
    ("demo_natural_add_ref_ref", "az_nat_add"),
    ("demo_natural_sub", "az_nat_sub"),
    ("demo_natural_sub_val_ref", "az_nat_sub"),
    ("demo_natural_sub_ref_val", "az_nat_sub"),
    ("demo_natural_sub_ref_ref", "az_nat_sub"),
    ("demo_natural_saturating_sub", "az_nat_saturating_sub"),
    (
        "demo_natural_saturating_sub_val_ref",
        "az_nat_saturating_sub",
    ),
    (
        "demo_natural_saturating_sub_ref_val",
        "az_nat_saturating_sub",
    ),
    (
        "demo_natural_saturating_sub_ref_ref",
        "az_nat_saturating_sub",
    ),
    ("demo_natural_mul", "az_nat_mul"),
    ("demo_natural_mul_val_ref", "az_nat_mul"),
    ("demo_natural_mul_ref_val", "az_nat_mul"),
    ("demo_natural_mul_ref_ref", "az_nat_mul"),
    ("demo_natural_square", "az_nat_square"),
    ("demo_natural_square_ref", "az_nat_square"),
    ("demo_natural_pow", "az_nat_pow"),
    ("demo_natural_pow_ref", "az_nat_pow"),
    ("demo_natural_div", "az_nat_div"),
    ("demo_natural_div_val_ref", "az_nat_div"),
    ("demo_natural_div_ref_val", "az_nat_div"),
    ("demo_natural_div_ref_ref", "az_nat_div"),
    ("demo_natural_mod", "az_nat_mod"),
    ("demo_natural_mod_val_ref", "az_nat_mod"),
    ("demo_natural_mod_ref_val", "az_nat_mod"),
    ("demo_natural_mod_ref_ref", "az_nat_mod"),
    ("demo_natural_rem", "az_nat_mod"),
    ("demo_natural_rem_ref_ref", "az_nat_mod"),
    ("demo_natural_div_mod", "az_nat_div_mod"),
    ("demo_natural_div_mod_val_ref", "az_nat_div_mod"),
    ("demo_natural_div_mod_ref_val", "az_nat_div_mod"),
    ("demo_natural_div_mod_ref_ref", "az_nat_div_mod"),
    ("demo_natural_div_rem", "az_nat_div_mod"),
    ("demo_natural_div_rem_ref_ref", "az_nat_div_mod"),
    ("demo_natural_div_round", "az_nat_div_round"),
    ("demo_natural_div_round_val_ref", "az_nat_div_round"),
    ("demo_natural_div_round_ref_val", "az_nat_div_round"),
    ("demo_natural_div_round_ref_ref", "az_nat_div_round"),
    ("demo_natural_shr_round_unsigned_u8", "az_nat_shr_round"),
    ("demo_natural_shr_round_unsigned_u64", "az_nat_shr_round"),
    (
        "demo_natural_shr_round_unsigned_ref_u64",
        "az_nat_shr_round",
    ),
    ("demo_natural_shr_round_signed_i8", "az_nat_shr_round"),
    ("demo_natural_shr_round_signed_i64", "az_nat_shr_round"),
    ("demo_natural_shr_round_signed_ref_i64", "az_nat_shr_round"),
    ("demo_natural_gcd", "az_nat_gcd"),
    ("demo_natural_gcd_val_ref", "az_nat_gcd"),
    ("demo_natural_gcd_ref_val", "az_nat_gcd"),
    ("demo_natural_gcd_ref_ref", "az_nat_gcd"),
    ("demo_natural_coprime_with", "az_nat_coprime_with"),
    ("demo_natural_coprime_with_ref_ref", "az_nat_coprime_with"),
    ("demo_natural_mod_inverse", "az_nat_mod_inverse"),
    ("demo_natural_mod_inverse_ref_ref", "az_nat_mod_inverse"),
    ("demo_natural_jacobi_symbol", "az_nat_jacobi_symbol"),
    ("demo_natural_jacobi_symbol_ref_ref", "az_nat_jacobi_symbol"),
    ("demo_natural_multi_crt", "az_nat_multi_crt"),
    ("demo_natural_mod_power_of_2", "az_nat_mod_power_of_2"),
    ("demo_natural_mod_power_of_2_ref", "az_nat_mod_power_of_2"),
    (
        "demo_natural_mod_power_of_2_add",
        "az_nat_mod_power_of_2_add",
    ),
    (
        "demo_natural_mod_power_of_2_add_ref_ref",
        "az_nat_mod_power_of_2_add",
    ),
    (
        "demo_natural_mod_power_of_2_sub",
        "az_nat_mod_power_of_2_sub",
    ),
    (
        "demo_natural_mod_power_of_2_sub_ref_ref",
        "az_nat_mod_power_of_2_sub",
    ),
    (
        "demo_natural_mod_power_of_2_mul",
        "az_nat_mod_power_of_2_mul",
    ),
    (
        "demo_natural_mod_power_of_2_mul_ref_ref",
        "az_nat_mod_power_of_2_mul",
    ),
    (
        "demo_natural_mod_power_of_2_square",
        "az_nat_mod_power_of_2_square",
    ),
    (
        "demo_natural_mod_power_of_2_square_ref",
        "az_nat_mod_power_of_2_square",
    ),
    ("demo_natural_floor_sqrt", "az_nat_floor_sqrt"),
    ("demo_natural_floor_sqrt_ref", "az_nat_floor_sqrt"),
    ("demo_natural_sqrt_rem", "az_nat_sqrt_rem"),
    ("demo_natural_sqrt_rem_ref", "az_nat_sqrt_rem"),
    ("demo_natural_floor_root", "az_nat_floor_root"),
    ("demo_natural_floor_root_ref", "az_nat_floor_root"),
    ("demo_natural_floor_cbrt", "az_nat_floor_root"),
    ("demo_natural_checked_root", "az_nat_checked_root"),
    ("demo_natural_checked_root_ref", "az_nat_checked_root"),
    ("demo_natural_checked_cbrt", "az_nat_checked_root"),
    ("demo_natural_is_power_of_2", "az_nat_is_power_of_2"),
    ("demo_natural_even", "az_nat_parity"),
    ("demo_natural_odd", "az_nat_parity"),
    (
        "demo_natural_divisible_by_power_of_2",
        "az_nat_divisible_by_power_of_2",
    ),
    ("demo_natural_significant_bits", "az_nat_significant_bits"),
    ("demo_natural_trailing_zeros", "az_nat_trailing_zeros"),
    ("demo_natural_limbs", "az_nat_limbs"),
    ("demo_natural_from_limbs_asc", "az_nat_from_limbs_asc"),
    ("demo_to_digits_asc_u8", "az_nat_to_digits_asc"),
    ("demo_to_digits_asc_u64", "az_nat_to_digits_asc"),
    ("demo_natural_from_string_base", "az_nat_from_string_base"),
    ("demo_natural_from_str", "az_nat_from_str"),
    ("demo_natural_to_string_base", "az_nat_to_string_base"),
    ("demo_natural_cmp", "az_nat_cmp"),
    ("demo_natural_cmp_normalized", "az_nat_cmp_normalized"),
    ("demo_natural_from_unsigned_u8", "az_nat_from_unsigned"),
    ("demo_natural_from_unsigned_u64", "az_nat_from_unsigned"),
    (
        "demo_natural_saturating_from_signed_i8",
        "az_nat_saturating_from_signed",
    ),
    (
        "demo_natural_saturating_from_signed_i64",
        "az_nat_saturating_from_signed",
    ),
    (
        "demo_primitive_int_wrapping_from_natural_u8",
        "az_nat_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_natural_u64",
        "az_nat_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_natural_usize",
        "az_nat_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_natural_i8",
        "az_nat_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_natural_i64",
        "az_nat_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_natural_isize",
        "az_nat_wrapping_from",
    ),
];

// The demos whose generators have no `special_random` mode.
const AZURITE_NATURAL_STAGES_NO_SPECIAL: &[(&str, &str)] = &[
    ("demo_natural_power_of_2", "az_nat_power_of_2"),
    ("demo_natural_low_mask", "az_nat_low_mask"),
    (
        "demo_natural_from_string_base_targeted",
        "az_nat_from_string_base",
    ),
    ("demo_natural_from_str_targeted", "az_nat_from_str"),
];

// The Integer demos checked against Azurite, with the oracle mode that reads each one. The modes
// follow the rows of the "Malachite for Azurite Users: Integers" mapping page; `extended_gcd` is a
// Natural demo whose Bézout coefficients are Integers.
const AZURITE_INTEGER_STAGES: &[(&str, &str)] = &[
    ("demo_integer_add", "az_int_add"),
    ("demo_integer_add_val_ref", "az_int_add"),
    ("demo_integer_add_ref_val", "az_int_add"),
    ("demo_integer_add_ref_ref", "az_int_add"),
    ("demo_integer_sub", "az_int_sub"),
    ("demo_integer_sub_val_ref", "az_int_sub"),
    ("demo_integer_sub_ref_val", "az_int_sub"),
    ("demo_integer_sub_ref_ref", "az_int_sub"),
    ("demo_integer_mul", "az_int_mul"),
    ("demo_integer_mul_val_ref", "az_int_mul"),
    ("demo_integer_mul_ref_val", "az_int_mul"),
    ("demo_integer_mul_ref_ref", "az_int_mul"),
    ("demo_integer_neg", "az_int_neg"),
    ("demo_integer_neg_ref", "az_int_neg"),
    ("demo_integer_div_euclidean", "az_int_div_euclidean"),
    ("demo_integer_mod_euclidean", "az_int_mod_euclidean"),
    ("demo_integer_div_mod_euclidean", "az_int_div_mod_euclidean"),
    ("demo_integer_div_mod", "az_int_div_mod"),
    ("demo_integer_div_mod_val_ref", "az_int_div_mod"),
    ("demo_integer_div_mod_ref_val", "az_int_div_mod"),
    ("demo_integer_div_mod_ref_ref", "az_int_div_mod"),
    ("demo_integer_mod", "az_int_mod"),
    ("demo_integer_mod_val_ref", "az_int_mod"),
    ("demo_integer_mod_ref_val", "az_int_mod"),
    ("demo_integer_mod_ref_ref", "az_int_mod"),
    ("demo_integer_div_exact", "az_int_div_exact"),
    ("demo_integer_div_exact_val_ref", "az_int_div_exact"),
    ("demo_integer_div_exact_ref_val", "az_int_div_exact"),
    ("demo_integer_div_exact_ref_ref", "az_int_div_exact"),
    ("demo_integer_div_round", "az_int_div_round"),
    ("demo_integer_div_round_val_ref", "az_int_div_round"),
    ("demo_integer_div_round_ref_val", "az_int_div_round"),
    ("demo_integer_div_round_ref_ref", "az_int_div_round"),
    ("demo_integer_shl_unsigned_u8", "az_int_shl"),
    ("demo_integer_shl_unsigned_u64", "az_int_shl"),
    ("demo_integer_shl_unsigned_ref_u64", "az_int_shl"),
    ("demo_integer_shl_signed_i8", "az_int_shl"),
    ("demo_integer_shl_signed_i64", "az_int_shl"),
    ("demo_integer_shl_signed_ref_i64", "az_int_shl"),
    ("demo_integer_shr_unsigned_u8", "az_int_shr"),
    ("demo_integer_shr_unsigned_u64", "az_int_shr"),
    ("demo_integer_shr_unsigned_ref_u64", "az_int_shr"),
    ("demo_integer_shr_signed_i8", "az_int_shr"),
    ("demo_integer_shr_signed_i64", "az_int_shr"),
    ("demo_integer_shr_signed_ref_i64", "az_int_shr"),
    ("demo_integer_shr_round_unsigned_u8", "az_int_shr_round"),
    ("demo_integer_shr_round_unsigned_u64", "az_int_shr_round"),
    (
        "demo_integer_shr_round_ref_unsigned_u64",
        "az_int_shr_round",
    ),
    ("demo_integer_shr_round_signed_i8", "az_int_shr_round"),
    ("demo_integer_shr_round_signed_i64", "az_int_shr_round"),
    ("demo_integer_shr_round_ref_signed_i64", "az_int_shr_round"),
    ("demo_integer_pow", "az_int_pow"),
    ("demo_integer_pow_ref", "az_int_pow"),
    ("demo_integer_gcd", "az_int_gcd"),
    ("demo_integer_gcd_val_ref", "az_int_gcd"),
    ("demo_integer_gcd_ref_val", "az_int_gcd"),
    ("demo_integer_gcd_ref_ref", "az_int_gcd"),
    ("demo_natural_extended_gcd", "az_int_extended_gcd"),
    ("demo_natural_extended_gcd_val_ref", "az_int_extended_gcd"),
    ("demo_natural_extended_gcd_ref_val", "az_int_extended_gcd"),
    ("demo_natural_extended_gcd_ref_ref", "az_int_extended_gcd"),
    ("demo_integer_is_power_of_2", "az_int_is_power_of_2"),
    ("demo_integer_even", "az_int_parity"),
    ("demo_integer_odd", "az_int_parity"),
    ("demo_integer_sign", "az_int_sign"),
    ("demo_integer_significant_bits", "az_int_significant_bits"),
    ("demo_integer_trailing_zeros", "az_int_trailing_zeros"),
    ("demo_integer_from_string_base", "az_int_from_string_base"),
    ("demo_integer_from_str", "az_int_from_str"),
    ("demo_integer_to_string", "az_int_to_string"),
    ("demo_integer_cmp", "az_int_cmp"),
    ("demo_integer_partial_cmp_natural", "az_int_cmp_natural"),
    ("demo_integer_partial_cmp_unsigned_u8", "az_int_cmp_unsigned"),
    ("demo_integer_partial_cmp_unsigned_u64", "az_int_cmp_unsigned"),
    ("demo_integer_partial_cmp_signed_i8", "az_int_cmp_signed"),
    ("demo_integer_partial_cmp_signed_i64", "az_int_cmp_signed"),
    ("demo_integer_eq", "az_int_eq"),
    ("demo_integer_partial_eq_natural", "az_int_eq_natural"),
    ("demo_integer_partial_eq_unsigned_u8", "az_int_eq_unsigned"),
    ("demo_integer_partial_eq_unsigned_u64", "az_int_eq_unsigned"),
    ("demo_integer_partial_eq_signed_i8", "az_int_eq_signed"),
    ("demo_integer_partial_eq_signed_i64", "az_int_eq_signed"),
    ("demo_integer_from_natural", "az_int_from_natural"),
    ("demo_integer_from_natural_ref", "az_int_from_natural"),
    ("demo_integer_from_unsigned_u8", "az_int_from_unsigned"),
    ("demo_integer_from_unsigned_u64", "az_int_from_unsigned"),
    ("demo_integer_from_signed_i8", "az_int_from_signed"),
    ("demo_integer_from_signed_i64", "az_int_from_signed"),
    ("demo_from_sign_and_abs", "az_int_from_sign_and_abs"),
    ("demo_from_sign_and_abs_ref", "az_int_from_sign_and_abs"),
    ("demo_integer_unsigned_abs", "az_int_unsigned_abs"),
    ("demo_integer_unsigned_abs_ref", "az_int_unsigned_abs"),
    (
        "demo_primitive_int_wrapping_from_integer_u8",
        "az_int_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_integer_u64",
        "az_int_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_integer_usize",
        "az_int_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_integer_i8",
        "az_int_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_integer_i64",
        "az_int_wrapping_from",
    ),
    (
        "demo_primitive_int_wrapping_from_integer_isize",
        "az_int_wrapping_from",
    ),
];

// The Integer demos whose generators have no `special_random` mode.
const AZURITE_INTEGER_STAGES_NO_SPECIAL: &[(&str, &str)] = &[
    ("demo_integer_power_of_2", "az_int_power_of_2"),
    ("demo_integer_low_mask", "az_int_low_mask"),
    (
        "demo_integer_from_string_base_targeted",
        "az_int_from_string_base",
    ),
    ("demo_integer_from_str_targeted", "az_int_from_str"),
];

fn test_azurite_units(oracle: &Path) {
    run_unit_files("azurite", |mode, file| {
        run_azurite_oracle(oracle, mode, file)
    });
}

fn test_against_azurite() {
    let oracle = build_azurite_oracle();
    test_azurite_units(&oracle);
    for (demo, mode) in AZURITE_NATURAL_STAGES {
        check_demo_against_azurite(&oracle, "../malachite-nz", demo, mode);
    }
    for (demo, mode) in AZURITE_NATURAL_STAGES_NO_SPECIAL {
        check_demo_against_azurite_in_modes(&oracle, "../malachite-nz", demo, mode, &MODES[..2]);
    }
    for (demo, mode) in AZURITE_INTEGER_STAGES {
        check_demo_against_azurite(&oracle, "../malachite-nz", demo, mode);
    }
    for (demo, mode) in AZURITE_INTEGER_STAGES_NO_SPECIAL {
        check_demo_against_azurite_in_modes(&oracle, "../malachite-nz", demo, mode, &MODES[..2]);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (flint, azurite) = match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [] => (true, true),
        ["flint"] => (true, false),
        ["azurite"] => (false, true),
        // The unit-row files alone, for both backends: a quick check while adding rows.
        ["units"] => {
            test_flint_units(&build_oracle());
            test_azurite_units(&build_azurite_oracle());
            return;
        }
        _ => panic!("usage: cargo run --release [-- flint | azurite | units]"),
    };
    if flint {
        test_against_flint();
    }
    if azurite {
        test_against_azurite();
    }
}

fn test_flint_units(oracle: &Path) {
    run_unit_files("flint", |mode, file| run_oracle(oracle, mode, Some(file)));
}

fn test_against_flint() {
    let oracle = build_oracle();
    test_flint_units(&oracle);

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

    // Edge rows for the multi-modulus CRT quirks: a single modulus of 1 is usable, but a 1 or a

    // Every case from test_balanced_mod in malachite-nz's IntegerPolynomial tests, including the
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

    // Every case from test_evaluate in malachite-nz's IntegerPolynomial and NaturalPolynomial tests,
    // checked against
    // FLINT's evaluation and against each of the two algorithms it chooses between.
    // The generated cases from evaluate_properties: by value and by reference, polynomials of
    // degree at least 50 (which take the divide-and-conquer path), and each algorithm on its own.
    // The generated cases from evaluate_integer_polynomial_rational_properties.
    for (demo_name, flint_mode) in [
        (
            "demo_integer_polynomial_evaluate_rational",
            "fmpz_poly_evaluate_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_ref",
            "fmpz_poly_evaluate_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_long",
            "fmpz_poly_evaluate_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_horner",
            "fmpz_poly_evaluate_horner_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_divide_and_conquer",
            "fmpz_poly_evaluate_divconquer_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_divide_and_conquer_long",
            "fmpz_poly_evaluate_divconquer_fmpq",
        ),
    ] {
        check_demo_against_flint(&oracle, "../malachite-q", demo_name, flint_mode);
    }

    // Every case from test_evaluate_rational_polynomial in malachite-q's tests, and the generated
    for demo_name in [
        "demo_rational_polynomial_evaluate",
        "demo_rational_polynomial_evaluate_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-q",
            demo_name,
            "fmpq_poly_evaluate_fmpq",
        );
    }

    // Every case from test_evaluate_rational_polynomial_integer in malachite-q's tests, and the
    for demo_name in [
        "demo_rational_polynomial_evaluate_integer",
        "demo_rational_polynomial_evaluate_integer_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-q",
            demo_name,
            "fmpq_poly_evaluate_fmpz",
        );
    }

    // Every case from test_mod_power_of_2_evaluate in malachite-nz's NaturalPolynomial tests, and the
    for demo_name in [
        "demo_natural_polynomial_mod_power_of_2_evaluate",
        "demo_natural_polynomial_mod_power_of_2_evaluate_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_mod_poly_evaluate_fmpz",
        );
    }

    // Every case from test_mod_power_of_2_evaluate in malachite-base's UnsignedPolynomial tests,
    for demo_name in [
        "demo_unsigned_polynomial_mod_power_of_2_evaluate",
        "demo_unsigned_polynomial_mod_power_of_2_evaluate_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-base",
            demo_name,
            "fmpz_mod_poly_evaluate_fmpz",
        );
    }

    // Every case from test_mod_evaluate_u64 in malachite-nz's IntegerPolynomial tests, and the
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integer_polynomial_mod_evaluate_u64",
        "fmpz_poly_evaluate_mod",
    );

    // Every case from test_mod_evaluate in malachite-base's UnsignedPolynomial tests, and the
    for demo_name in [
        "demo_unsigned_polynomial_mod_evaluate",
        "demo_unsigned_polynomial_mod_evaluate_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-base",
            demo_name,
            "fmpz_mod_poly_evaluate_fmpz",
        );
    }

    // Every case from test_mod_evaluate in malachite-nz's NaturalPolynomial tests, and the generated
    for demo_name in [
        "demo_natural_polynomial_mod_evaluate",
        "demo_natural_polynomial_mod_evaluate_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_mod_poly_evaluate_fmpz",
        );
    }

    // Every case from test_add and test_add_self in malachite-q's RationalPolynomial tests, and the
    for demo_name in [
        "demo_rational_polynomial_add",
        "demo_rational_polynomial_add_val_ref",
        "demo_rational_polynomial_add_ref_val",
        "demo_rational_polynomial_add_ref_ref",
        "demo_rational_polynomial_add_assign",
        "demo_rational_polynomial_add_assign_ref",
    ] {
        check_demo_against_flint(&oracle, "../malachite-q", demo_name, "fmpq_poly_add");
    }

    // Every case from test_sub and test_sub_self in malachite-q's RationalPolynomial tests, and the
    for demo_name in [
        "demo_rational_polynomial_sub",
        "demo_rational_polynomial_sub_val_ref",
        "demo_rational_polynomial_sub_ref_val",
        "demo_rational_polynomial_sub_ref_ref",
        "demo_rational_polynomial_sub_assign",
        "demo_rational_polynomial_sub_assign_ref",
    ] {
        check_demo_against_flint(&oracle, "../malachite-q", demo_name, "fmpq_poly_sub");
    }

    // Every case from test_add_truncated in malachite-nz's IntegerPolynomial tests, and the
    for demo_name in [
        "demo_integer_polynomial_add_truncated",
        "demo_integer_polynomial_add_truncated_val_ref",
        "demo_integer_polynomial_add_truncated_ref_val",
        "demo_integer_polynomial_add_truncated_ref_ref",
        "demo_integer_polynomial_add_truncated_assign",
        "demo_integer_polynomial_add_truncated_assign_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_add_series",
        );
    }

    // Every case from test_add_truncated in malachite-nz's NaturalPolynomial tests, and the
    for demo_name in [
        "demo_natural_polynomial_add_truncated",
        "demo_natural_polynomial_add_truncated_val_ref",
        "demo_natural_polynomial_add_truncated_ref_val",
        "demo_natural_polynomial_add_truncated_ref_ref",
        "demo_natural_polynomial_add_truncated_assign",
        "demo_natural_polynomial_add_truncated_assign_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_add_series",
        );
    }

    // Every case from test_sub_truncated in malachite-nz's IntegerPolynomial tests, and the
    for demo_name in [
        "demo_integer_polynomial_sub_truncated",
        "demo_integer_polynomial_sub_truncated_val_ref",
        "demo_integer_polynomial_sub_truncated_ref_val",
        "demo_integer_polynomial_sub_truncated_ref_ref",
        "demo_integer_polynomial_sub_truncated_assign",
        "demo_integer_polynomial_sub_truncated_assign_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_sub_series",
        );
    }

    // Every case from test_add_truncated and test_add_truncated_self in malachite-q's
    for demo_name in [
        "demo_rational_polynomial_add_truncated",
        "demo_rational_polynomial_add_truncated_val_ref",
        "demo_rational_polynomial_add_truncated_ref_val",
        "demo_rational_polynomial_add_truncated_ref_ref",
        "demo_rational_polynomial_add_truncated_assign",
        "demo_rational_polynomial_add_truncated_assign_ref",
    ] {
        check_demo_against_flint(&oracle, "../malachite-q", demo_name, "fmpq_poly_add_series");
    }

    // Every case from test_sub_truncated and test_sub_truncated_self in malachite-q's
    for demo_name in [
        "demo_rational_polynomial_sub_truncated",
        "demo_rational_polynomial_sub_truncated_val_ref",
        "demo_rational_polynomial_sub_truncated_ref_val",
        "demo_rational_polynomial_sub_truncated_ref_ref",
        "demo_rational_polynomial_sub_truncated_assign",
        "demo_rational_polynomial_sub_truncated_assign_ref",
    ] {
        check_demo_against_flint(&oracle, "../malachite-q", demo_name, "fmpq_poly_sub_series");
    }

    // Every case from test_bit_pack in malachite-nz's IntegerPolynomial tests, and the generated
    for demo_name in [
        "demo_integer_polynomial_bit_pack",
        "demo_integer_polynomial_bit_pack_ref",
        "demo_integer_polynomial_bit_pack_small_bits",
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, "fmpz_poly_bit_pack");
    }

    // Every case from test_bit_pack in malachite-nz's NaturalPolynomial tests, and the generated
    for demo_name in [
        "demo_natural_polynomial_bit_pack",
        "demo_natural_polynomial_bit_pack_ref",
        "demo_natural_polynomial_bit_pack_small_bits",
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, "fmpz_poly_bit_pack");
    }

    // Every case from test_bit_unpack in malachite-nz's NaturalPolynomial tests, and the generated
    for demo_name in [
        "demo_natural_polynomial_bit_unpack",
        "demo_natural_polynomial_bit_unpack_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_bit_unpack_unsigned",
        );
    }

    // Every case from test_bit_unpack in malachite-nz's IntegerPolynomial tests, and the generated
    for demo_name in [
        "demo_integer_polynomial_bit_unpack",
        "demo_integer_polynomial_bit_unpack_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_bit_unpack",
        );
    }

    // Every case from test_mul_to_out_classical in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mul_classical.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_to_out_classical",
        "_fmpz_poly_mul_classical",
    );

    // Every case from test_mul_to_out_tiny_1 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mul.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_to_out_tiny_1",
        "_fmpz_poly_mul",
    );

    // Every case from test_mul_to_out_tiny_2 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mul.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_to_out_tiny_2",
        "_fmpz_poly_mul",
    );

    // Every case from test_mul_greater_to_out in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mul.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_greater_to_out",
        "_fmpz_poly_mul",
    );

    // Every case from test_mul in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against fmpz_poly_mul.
    for demo_name in [
        "demo_integer_polynomial_mul",
        "demo_integer_polynomial_mul_val_ref",
        "demo_integer_polynomial_mul_ref_val",
        "demo_integer_polynomial_mul_ref_ref",
        "demo_integer_polynomial_mul_assign",
        "demo_integer_polynomial_mul_assign_ref",
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, "fmpz_poly_mul");
    }

    // Every case from test_mul_truncated_to_out_classical in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mullow_classical.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_truncated_to_out_classical",
        "_fmpz_poly_mullow_classical",
    );

    // Every case from test_mul_truncated_to_out_tiny_1 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mullow.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_truncated_to_out_tiny_1",
        "_fmpz_poly_mullow",
    );

    // Every case from test_mul_truncated_to_out_tiny_2 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mullow.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_truncated_to_out_tiny_2",
        "_fmpz_poly_mullow",
    );

    // Every case from test_mul_truncated_to_out in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mullow.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_truncated_to_out",
        "_fmpz_poly_mullow",
    );

    // Every case from test_mul_truncated in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against fmpz_poly_mullow.
    for demo_name in [
        "demo_integer_polynomial_mul_truncated",
        "demo_integer_polynomial_mul_truncated_val_ref",
        "demo_integer_polynomial_mul_truncated_ref_val",
        "demo_integer_polynomial_mul_truncated_ref_ref",
        "demo_integer_polynomial_mul_truncated_assign",
        "demo_integer_polynomial_mul_truncated_assign_ref",
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, "fmpz_poly_mullow");
    }

    // Every case from test_mul_high_to_out_classical in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mulhigh_classical.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_high_to_out_classical",
        "_fmpz_poly_mulhigh_classical",
    );

    // Every case from test_mul_middle_to_out_classical in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mulmid_classical.
    for demo_name in [
        "demo_mul_middle_to_out_classical",
        "demo_mul_middle_to_out_classical_square",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "_fmpz_poly_mulmid_classical",
        );
    }

    // Every case from test_mul_middle_to_out_tiny_1 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mulmid.
    for demo_name in [
        "demo_mul_middle_to_out_tiny_1",
        "demo_mul_middle_to_out_tiny_1_square",
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, "_fmpz_poly_mulmid");
    }

    // Every case from test_mul_middle_to_out_tiny_2 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mulmid.
    for demo_name in [
        "demo_mul_middle_to_out_tiny_2",
        "demo_mul_middle_to_out_tiny_2_square",
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, "_fmpz_poly_mulmid");
    }

    // Every case from test_mul_middle_to_out in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mulmid.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_middle_to_out",
        "_fmpz_poly_mulmid",
    );

    // Every case from test_square_to_out_classical in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqr_classical.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_to_out_classical",
        "_fmpz_poly_sqr_classical",
    );

    // Every case from test_square_to_out_tiny_1 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqr.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_to_out_tiny_1",
        "_fmpz_poly_sqr",
    );

    // Every case from test_square_to_out_tiny_2 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqr.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_to_out_tiny_2",
        "_fmpz_poly_sqr",
    );

    // Every case from test_square_to_out in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqr.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_to_out",
        "_fmpz_poly_sqr",
    );

    // Every case from test_square in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against fmpz_poly_sqr.
    for demo_name in [
        "demo_integer_polynomial_square",
        "demo_integer_polynomial_square_ref",
        "demo_integer_polynomial_square_assign",
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, "fmpz_poly_sqr");
    }

    // Every case from test_square_truncated_to_out_classical in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqrlow_classical.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_truncated_to_out_classical",
        "_fmpz_poly_sqrlow_classical",
    );

    // Every case from test_square_truncated_to_out_tiny_1 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqrlow.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_truncated_to_out_tiny_1",
        "_fmpz_poly_sqrlow",
    );

    // Every case from test_square_truncated_to_out_tiny_2 in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqrlow.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_truncated_to_out_tiny_2",
        "_fmpz_poly_sqrlow",
    );

    // Every case from test_square_truncated_to_out in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqrlow.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_truncated_to_out",
        "_fmpz_poly_sqrlow",
    );

    // Every case from test_square_truncated in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against fmpz_poly_sqrlow.
    for demo_name in [
        "demo_integer_polynomial_square_truncated",
        "demo_integer_polynomial_square_truncated_ref",
        "demo_integer_polynomial_square_truncated_assign",
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, "fmpz_poly_sqrlow");
    }

    // Every case from test_mul_to_out_karatsuba in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demo, against _fmpz_poly_mul_karatsuba.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_to_out_karatsuba",
        "_fmpz_poly_mul_karatsuba",
    );

    // Every case from test_mul_truncated_to_out_karatsuba_n in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demo, against _fmpz_poly_mullow_karatsuba_n.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_truncated_to_out_karatsuba_n",
        "_fmpz_poly_mullow_karatsuba_n",
    );

    // Every case from test_mul_truncated_to_out_karatsuba in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demo, against _fmpz_poly_mullow_karatsuba.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_truncated_to_out_karatsuba",
        "_fmpz_poly_mullow_karatsuba",
    );

    // Every case from test_mul_high_to_out_karatsuba_n in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demo, against _fmpz_poly_mulhigh_karatsuba_n.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_high_to_out_karatsuba_n",
        "_fmpz_poly_mulhigh_karatsuba_n",
    );

    // Every case from test_mul_high_to_out in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demo, against _fmpz_poly_mulhigh.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_high_to_out",
        "_fmpz_poly_mulhigh",
    );

    // Every case from test_square_to_out_karatsuba in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demo, against _fmpz_poly_sqr_karatsuba.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_to_out_karatsuba",
        "_fmpz_poly_sqr_karatsuba",
    );

    // Every case from test_square_truncated_to_out_karatsuba_n in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demo, against _fmpz_poly_sqrlow_karatsuba_n.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_truncated_to_out_karatsuba_n",
        "_fmpz_poly_sqrlow_karatsuba_n",
    );

    // Every case from test_square_truncated_to_out_karatsuba in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demo, against _fmpz_poly_sqrlow_karatsuba.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_truncated_to_out_karatsuba",
        "_fmpz_poly_sqrlow_karatsuba",
    );

    // Every case from test_mul_to_out_kronecker in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mul_KS.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_to_out_kronecker",
        "_fmpz_poly_mul_KS",
    );

    // Every case from test_mul_truncated_to_out_kronecker in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mullow_KS.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_mul_truncated_to_out_kronecker",
        "_fmpz_poly_mullow_KS",
    );

    // Every case from test_mul_middle_to_out_kronecker in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_mulmid_KS.
    for demo_name in [
        "demo_mul_middle_to_out_kronecker",
        "demo_mul_middle_to_out_kronecker_square",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "_fmpz_poly_mulmid_KS",
        );
    }

    // Every case from test_square_to_out_kronecker in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqr_KS.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_to_out_kronecker",
        "_fmpz_poly_sqr_KS",
    );

    // Every case from test_square_truncated_to_out_kronecker in malachite-nz's IntegerPolynomial tests, and the generated cases
    // from its demos, against _fmpz_poly_sqrlow_KS.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_square_truncated_to_out_kronecker",
        "_fmpz_poly_sqrlow_KS",
    );

    // Every literal case from test_mul_middle_to_out_fft in malachite-nz's IntegerPolynomial tests,
    // and the generated cases from its demos, against _fmpz_poly_mul_mid_default_mpn_ctx, including
    // whether it declines.
    for demo_name in [
        "demo_mul_middle_to_out_fft",
        "demo_mul_middle_to_out_fft_square",
        "demo_mul_middle_to_out_fft_long",
        "demo_mul_middle_to_out_fft_long_square",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "_fmpz_poly_mul_mid_default_mpn_ctx",
        );
    }

    // The generated cases from the demos of the port of FLINT's Schönhage–Strassen code in
    // malachite-nz (`natural::arithmetic::mul::schonhage_strassen`), against the FLINT functions
    // they port. Demos whose lines are long run fewer of them.
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_add_signed_limb_mod_2expp1",
        "mpn_addmod_2expp1_1",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_sum_diff",
        "flint_mpn_sumdiff_n",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_norm_mod_2expp1",
        "mpn_normmod_2expp1",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_neg_mod_2expp1_to_out",
        "mpn_negmod_2expp1",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_mul_2exp_mod_2expp1_in_place",
        "mpn_mul_2expmod_2expp1",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_mul_2exp_mod_2expp1_to_out",
        "mpn_mul_2expmod_2expp1",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_div_2exp_mod_2expp1_in_place",
        "mpn_div_2expmod_2expp1",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_div_2exp_mod_2expp1_to_out",
        "mpn_div_2expmod_2expp1",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_fft_adjust",
        "fft_adjust",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_fft_adjust_sqrt2",
        "fft_adjust_sqrt2",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_butterfly_lsh_b",
        "butterfly_lshB",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_butterfly_rsh_b",
        "butterfly_rshB",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_fft_butterfly",
        "fft_butterfly",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_ifft_butterfly",
        "ifft_butterfly",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_fft_butterfly_sqrt2",
        "fft_butterfly_sqrt2",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_ifft_butterfly_sqrt2",
        "ifft_butterfly_sqrt2",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_fft_butterfly_twiddle",
        "fft_butterfly_twiddle",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_ifft_butterfly_twiddle",
        "ifft_butterfly_twiddle",
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_radix2",
        "fft_radix2",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_ifft_radix2",
        "ifft_radix2",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_truncate1",
        "fft_truncate1",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_truncate",
        "fft_truncate",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_ifft_truncate1",
        "ifft_truncate1",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_ifft_truncate",
        "ifft_truncate",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_truncate_sqrt2",
        "fft_truncate_sqrt2",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_ifft_truncate_sqrt2",
        "ifft_truncate_sqrt2",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_negacyclic",
        "fft_negacyclic",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_ifft_negacyclic",
        "ifft_negacyclic",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_radix2_twiddle",
        "fft_radix2_twiddle",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_ifft_radix2_twiddle",
        "ifft_radix2_twiddle",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_truncate1_twiddle",
        "fft_truncate1_twiddle",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_ifft_truncate1_twiddle",
        "ifft_truncate1_twiddle",
        3000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_mfa_truncate_sqrt2_outer",
        "fft_mfa_truncate_sqrt2_outer",
        1500,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_ifft_mfa_truncate_sqrt2_outer",
        "ifft_mfa_truncate_sqrt2_outer",
        1500,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_mfa_truncate_sqrt2_inner",
        "fft_mfa_truncate_sqrt2_inner",
        1000,
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_limbs_mul_mod_2expp1_basecase",
        "flint_mpn_mulmod_2expp1_basecase",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_fft_naive_convolution_1",
        "fft_naive_convolution_1",
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_mulmod_2expp1_negacyclic",
        "_fft_mulmod_2expp1",
        3000,
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_fft_mulmod_2expp1",
        "fft_mulmod_2expp1",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_fft_adjust_limbs",
        "fft_adjust_limbs",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_fft_split_limbs",
        "fft_split_limbs",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_fft_split_bits",
        "fft_split_bits",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_fft_combine_limbs",
        "fft_combine_limbs",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_fft_combine_bits",
        "fft_combine_bits",
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_convolution",
        "fft_convolution",
        1000,
    );
    check_demo_against_flint_with_limit(
        &oracle,
        "../malachite-nz",
        "demo_fft_convolution_matrix_fourier",
        "fft_convolution",
        200,
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integers_to_fermat_residues",
        "_fmpz_vec_get_fft",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integers_from_fermat_residues",
        "_fmpz_vec_set_fft",
    );

    // The generated cases from the demos of IntegerPolynomial's slice-level Schönhage–Strassen
    // multiplication, against _fmpz_poly_*_SS.
    for (demo_name, flint_mode) in [
        ("demo_mul_to_out_schonhage_strassen", "_fmpz_poly_mul_SS"),
        (
            "demo_mul_truncated_to_out_schonhage_strassen",
            "_fmpz_poly_mullow_SS",
        ),
        (
            "demo_mul_middle_to_out_schonhage_strassen",
            "_fmpz_poly_mulmid_SS",
        ),
        (
            "demo_mul_middle_to_out_schonhage_strassen_square",
            "_fmpz_poly_mulmid_SS",
        ),
        (
            "demo_mul_middle_to_out_schonhage_strassen_wide",
            "_fmpz_poly_mulmid_SS",
        ),
        (
            "demo_mul_middle_to_out_schonhage_strassen_wide_square",
            "_fmpz_poly_mulmid_SS",
        ),
        ("demo_square_to_out_schonhage_strassen", "_fmpz_poly_sqr_SS"),
        (
            "demo_square_truncated_to_out_schonhage_strassen",
            "_fmpz_poly_sqrlow_SS",
        ),
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, flint_mode);
    }
}
