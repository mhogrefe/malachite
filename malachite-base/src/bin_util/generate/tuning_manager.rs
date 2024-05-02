// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

const PARENT_PATHS: [&str; 2] = ["../malachite-base/src", "../malachite-nz/src"];

fn find_all_file_paths() -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    for &parent_path in &PARENT_PATHS {
        for entry in WalkDir::new(parent_path) {
            let name = entry.unwrap().path().display().to_string();
            // Exclude platform_64 as we'll get all the same info from _32
            if name.ends_with(".rs") && !name.ends_with("/platform_64.rs") {
                paths.insert(name);
            }
        }
    }
    paths
}

fn primitive_tokenize(s: &str) -> Vec<String> {
    let chars = s.chars().collect_vec();
    let mut tokens = Vec::new();
    let mut token = String::new();
    for &c in &chars {
        if c.is_alphanumeric() || c == '_' {
            token.push(c);
        } else if !token.is_empty() {
            tokens.push(token);
            token = String::new();
        }
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

const FN_PREFIXES: [&str; 6] =
    ["pub fn ", "pub(crate) fn ", "fn ", "pub const fn ", "pub(crate) const fn ", "const fn "];

const ALLOWED_DOUBLE_NAMES: [&str; 3] = ["limbs_mod_limb", "oz_fmt", "fail_on_untested_path"];

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct FunctionRecord {
    name: String,
    file_path: String,
    line_number: usize,
}

fn get_all_top_level_functions() -> BTreeMap<String, FunctionRecord> {
    let allowed_double_names: HashSet<&str> = ALLOWED_DOUBLE_NAMES.iter().cloned().collect();
    let mut fns = BTreeMap::new();
    let file_paths = find_all_file_paths();
    for path in &file_paths {
        let input = File::open(path).unwrap();
        let buffered = BufReader::new(input);
        for (i, line) in buffered.lines().enumerate() {
            let line = line.unwrap();
            for &prefix in &FN_PREFIXES {
                if let Some(fn_string) = line.strip_prefix(prefix) {
                    let fn_name = primitive_tokenize(fn_string).swap_remove(0);
                    if !allowed_double_names.contains(&fn_name.as_str())
                        && fns.contains_key(&fn_name)
                    {
                        panic!("Duplicate top-level function name: {fn_name}");
                    }
                    fns.insert(
                        fn_name.clone(),
                        FunctionRecord {
                            name: fn_name,
                            file_path: path.to_string(),
                            line_number: i,
                        },
                    );
                }
            }
        }
    }
    fns
}

const CONST_PREFIXES: [&str; 3] = ["pub const ", "pub(crate) const ", "const "];

fn get_all_tuneable_constants() -> BTreeMap<String, FunctionRecord> {
    let file_paths = find_all_file_paths();
    let mut constants = BTreeMap::new();
    let mut expecting_constant = false;
    for path in &file_paths {
        let input = File::open(path).unwrap();
        let buffered = BufReader::new(input);
        for (i, line) in buffered.lines().enumerate() {
            let line = line.unwrap();
            if expecting_constant {
                if line.starts_with("//") || line.starts_with("#[") {
                    // do nothing
                } else {
                    let mut p = None;
                    for &prefix in &CONST_PREFIXES {
                        if line.starts_with(prefix) {
                            p = Some(prefix);
                            break;
                        }
                    }
                    if p.is_none() {
                        panic!("Bad const. line {i} in {path}");
                    }
                    let p = p.unwrap();
                    let name = &line[p.len()..];
                    let colon_index = name.chars().position(|c| c == ':').unwrap();
                    let name = &name[..colon_index];
                    if constants.contains_key(name) {
                        panic!("Duplicate constant name: {name}");
                    }
                    constants.insert(
                        name.to_string(),
                        FunctionRecord {
                            name: name.to_string(),
                            file_path: path.to_string(),
                            line_number: i,
                        },
                    );
                    expecting_constant = false;
                }
            } else if line == "//TODO tune" {
                expecting_constant = true;
            }
        }
    }
    assert!(!expecting_constant);
    constants
}

fn extract_functions_and_constants(
    name: &str,
    lines: &[String],
    fns: &BTreeMap<String, FunctionRecord>,
    cs: &BTreeMap<String, FunctionRecord>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut matched_fns = BTreeSet::new();
    let mut matched_cs = BTreeSet::new();
    let mut first = true;
    for line in lines {
        for token in primitive_tokenize(line) {
            if fns.contains_key(&token) {
                if first {
                    assert_eq!(token, name);
                    first = false;
                } else {
                    matched_fns.insert(token.clone());
                }
            } else if cs.contains_key(&token) {
                if first {
                    assert_eq!(token, name);
                    first = false;
                } else {
                    matched_cs.insert(token.clone());
                }
            }
        }
    }
    (matched_fns, matched_cs)
}

fn get_referenced_items_for_constant(
    constant: &FunctionRecord,
    fns: &BTreeMap<String, FunctionRecord>,
    cs: &BTreeMap<String, FunctionRecord>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let input = File::open(&constant.file_path).unwrap();
    let buffered = BufReader::new(input);
    let mut lines = Vec::new();
    let mut brace_index = 0;
    for (i, line) in buffered.lines().enumerate() {
        if i < constant.line_number {
            continue;
        }
        let line = line.unwrap();
        let mut done = false;
        for c in line.chars() {
            match c {
                '{' => brace_index += 1,
                '}' => {
                    assert_ne!(brace_index, 0);
                    brace_index -= 1;
                }
                ';' => {
                    if brace_index == 0 {
                        done = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        lines.push(line);
        if done {
            break;
        }
    }
    extract_functions_and_constants(&constant.name, &lines, fns, cs)
}

fn get_referenced_items_for_function(
    function: &FunctionRecord,
    fns: &BTreeMap<String, FunctionRecord>,
    cs: &BTreeMap<String, FunctionRecord>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let input = File::open(&function.file_path).unwrap();
    let buffered = BufReader::new(input);
    let mut lines = Vec::new();
    let mut brace_index = 0;
    for (i, line) in buffered.lines().enumerate() {
        if i < function.line_number {
            continue;
        }
        let line = line.unwrap();
        let mut done = false;
        for c in line.chars() {
            match c {
                '{' => brace_index += 1,
                '}' => {
                    assert_ne!(brace_index, 0);
                    brace_index -= 1;
                    if brace_index == 0 {
                        done = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        lines.push(line);
        if done {
            break;
        }
    }
    extract_functions_and_constants(&function.name, &lines, fns, cs)
}

pub struct ReferenceData {
    functions: BTreeMap<String, FunctionRecord>,
    constants: BTreeMap<String, FunctionRecord>,
    pub constants_referencing_constants: BTreeMap<String, BTreeSet<String>>,
    pub constants_referencing_functions: BTreeMap<String, BTreeSet<String>>,
    functions_referencing_constants: BTreeMap<String, BTreeSet<String>>,
    pub functions_referencing_functions: BTreeMap<String, BTreeSet<String>>,
}

fn parse_and_get_references() -> ReferenceData {
    let fns = get_all_top_level_functions();
    let cs = get_all_tuneable_constants();
    let mut crc = BTreeMap::new();
    let mut crf = BTreeMap::new();
    for c in cs.values() {
        let (rf, rc) = get_referenced_items_for_constant(c, &fns, &cs);
        crc.insert(c.name.clone(), rc);
        crf.insert(c.name.clone(), rf);
    }
    let mut frc = BTreeMap::new();
    let mut frf = BTreeMap::new();
    for f in fns.values() {
        let (rf, rc) = get_referenced_items_for_function(f, &fns, &cs);
        frc.insert(f.name.clone(), rc);
        frf.insert(f.name.clone(), rf);
    }
    ReferenceData {
        functions: fns,
        constants: cs,
        constants_referencing_constants: crc,
        constants_referencing_functions: crf,
        functions_referencing_constants: frc,
        functions_referencing_functions: frf,
    }
}

fn invert_map(m: &BTreeMap<String, BTreeSet<String>>) -> BTreeMap<String, BTreeSet<String>> {
    let mut inverse = BTreeMap::new();
    for (k, vs) in m {
        for v in vs {
            inverse
                .entry(v.clone())
                .or_insert_with(BTreeSet::new)
                .insert(k.clone());
        }
    }
    inverse
}

pub fn build_reference_data() {
    parse_and_get_references();
}

fn hfdm_helper(map: &mut BTreeMap<String, String>, k: &str, v: &str) {
    map.insert(k.to_string(), v.to_string());
}

fn hardcoded_defining_function_map() -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    hfdm_helper(&mut m, "BMOD_1_TO_MOD_1_THRESHOLD", "limbs_mod_limb_helper");
    hfdm_helper(
        &mut m,
        "DC_BDIV_QR_THRESHOLD",
        "limbs_modular_div_mod_helper",
    );
    hfdm_helper(&mut m, "DC_BDIV_Q_THRESHOLD", "limbs_modular_invert_small");
    hfdm_helper(&mut m, "DC_DIVAPPR_Q_THRESHOLD", "limbs_div_approx_helper");
    hfdm_helper(&mut m, "DC_DIV_QR_THRESHOLD", "limbs_div_dc_helper");
    hfdm_helper(&mut m, "DC_DIV_Q_THRESHOLD", "limbs_div_q_dc_helper");
    m
}

fn generate_defining_function_map(data: &ReferenceData) -> BTreeMap<String, String> {
    let mut defining_functions = hardcoded_defining_function_map();
    for (k, v) in &defining_functions {
        assert!(data.constants.contains_key(k));
        assert!(data.functions.contains_key(v));
        assert!(data.functions_referencing_constants[v].contains(k));
    }
    for (c, fs) in invert_map(&data.functions_referencing_constants) {
        if defining_functions.contains_key(&c) {
            continue;
        }
        if fs.len() == 1 {
            defining_functions.insert(c.clone(), fs.iter().next().unwrap().clone());
        } else {
            panic!("Must specify defining function for {c}. Possibilities are {fs:?}");
        }
    }
    defining_functions
}

pub fn test() {
    let data = parse_and_get_references();
    generate_defining_function_map(&data);
}
