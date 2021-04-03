use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

#[derive(Debug, Default)]
struct DependencyInfo {
    constant_uses_constants_map: BTreeMap<String, BTreeSet<String>>,
    function_uses_constants_map: BTreeMap<String, BTreeSet<String>>,
    function_uses_functions_map: BTreeMap<String, BTreeSet<String>>,
    constant_dependencies_are_complete: HashSet<String>,
    constant_constants_are_complete: HashSet<String>,
    function_constants_are_complete: HashSet<String>,
    function_functions_are_complete: HashSet<String>,
}

fn read_from_file(name: &str) -> DependencyInfo {
    let mut info = DependencyInfo::default();
    let input = File::open(name).unwrap();
    let buffered = BufReader::new(input);
    let mut expecting_name = true;
    let mut expecting_constants = false;
    let mut expecting_fns = false;
    let mut current_name = String::new();
    let mut constants = BTreeSet::new();
    let mut fns = BTreeSet::new();
    let mut cs_complete = true;
    let mut fs_complete = true;
    let mut asterisk = false;
    let mut is_constant = true;
    for line in buffered.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if expecting_name {
            let trimmed = line.trim_end_matches('*');
            asterisk = trimmed != line;
            if info.constant_uses_constants_map.contains_key(trimmed)
                || info.function_uses_constants_map.contains_key(trimmed)
            {
                panic!("{} defined multiple times", trimmed);
            }
            current_name = trimmed.to_string();
            expecting_name = false;
        } else if line == "constants:" {
            expecting_fns = false;
            expecting_constants = true;
        } else if line == "functions:" {
            is_constant = false;
            expecting_constants = false;
            expecting_fns = true;
        } else if line.is_empty() {
            if is_constant {
                assert!(fs_complete);
                assert!(fns.is_empty());
                info.constant_uses_constants_map
                    .insert(current_name.clone(), constants);
                if cs_complete {
                    info.constant_constants_are_complete
                        .insert(current_name.clone());
                }
            } else {
                info.function_uses_constants_map
                    .insert(current_name.clone(), constants);
                info.function_uses_functions_map
                    .insert(current_name.clone(), fns);
                if cs_complete {
                    info.function_constants_are_complete
                        .insert(current_name.clone());
                }
                if fs_complete {
                    info.function_functions_are_complete
                        .insert(current_name.clone());
                }
            }
            if asterisk {
                assert!(is_constant);
                info.constant_dependencies_are_complete
                    .insert(current_name.clone());
            }
            constants = BTreeSet::new();
            fns = BTreeSet::new();
            is_constant = true;
            cs_complete = true;
            fs_complete = true;
            asterisk = false;
            current_name = String::new();
            expecting_name = true;
            expecting_constants = false;
            expecting_fns = false;
        } else if expecting_constants {
            if line == "..." {
                cs_complete = false;
            } else {
                constants.insert(line.to_string());
            }
        } else if expecting_fns {
            if line == "..." {
                fs_complete = false;
            } else {
                fns.insert(line.to_string());
            }
        }
    }
    if is_constant {
        assert!(fs_complete);
        assert!(fns.is_empty());
        info.constant_uses_constants_map
            .insert(current_name.clone(), constants);
        if cs_complete {
            info.constant_constants_are_complete
                .insert(current_name.clone());
        }
    } else {
        info.function_uses_constants_map
            .insert(current_name.clone(), constants);
        info.function_uses_functions_map
            .insert(current_name.clone(), fns);
        if cs_complete {
            info.function_constants_are_complete
                .insert(current_name.clone());
        }
        if fs_complete {
            info.function_functions_are_complete
                .insert(current_name.clone());
        }
    }
    if asterisk {
        assert!(is_constant);
        info.constant_dependencies_are_complete.insert(current_name);
    }
    info
}

fn write_to_file(name: &str, info: &DependencyInfo) -> Result<(), Error> {
    let mut output = File::create(name)?;
    for (constant, used_constants) in &info.constant_uses_constants_map {
        write!(output, "{}", constant)?;
        if info.constant_dependencies_are_complete.contains(constant) {
            writeln!(output, "*")?;
        } else {
            writeln!(output)?;
        }
        writeln!(output, "  constants:")?;
        for c in used_constants {
            writeln!(output, "    {}", c)?;
        }
        if !info.constant_constants_are_complete.contains(constant) {
            writeln!(output, "    ...")?;
        }
        writeln!(output)?;
    }

    for (function, used_functions) in &info.function_uses_functions_map {
        writeln!(output, "{}", function)?;
        writeln!(output, "  constants:")?;
        for c in &info.function_uses_constants_map[function] {
            writeln!(output, "    {}", c)?;
        }
        if !info.function_constants_are_complete.contains(function) {
            writeln!(output, "    ...")?;
        }
        writeln!(output, "  functions:")?;
        for f in used_functions {
            writeln!(output, "    {}", f)?;
        }
        if !info.function_functions_are_complete.contains(function) {
            writeln!(output, "    ...")?;
        }
        writeln!(output)?;
    }
    Ok(())
}

pub fn read_and_print() {
    let info = read_from_file("data/tuning_dependencies.txt");
    write_to_file("data/tuning_dependencies_out.txt", &info).unwrap();
}

pub fn print_constants_with_unfinished_deps() {
    let info = read_from_file("data/tuning_dependencies.txt");
    for constant in info.constant_uses_constants_map.keys() {
        if !info.constant_dependencies_are_complete.contains(constant) {
            println!("{}", constant);
        }
    }
}

pub fn print_undefined_constants() {
    let info = read_from_file("data/tuning_dependencies.txt");
    let mut undefined = BTreeSet::new();
    for constants in info.constant_uses_constants_map.values() {
        for constant in constants {
            if !info.constant_uses_constants_map.contains_key(constant) {
                undefined.insert(constant);
            }
        }
    }
    for constants in info.function_uses_constants_map.values() {
        for constant in constants {
            if !info.constant_uses_constants_map.contains_key(constant) {
                undefined.insert(constant);
            }
        }
    }
    for constant in undefined {
        println!("{}", constant);
    }
}

pub fn print_undefined_functions() {
    let info = read_from_file("data/tuning_dependencies.txt");
    let mut undefined = BTreeSet::new();
    for constants in info.function_uses_functions_map.values() {
        for constant in constants {
            if !info.constant_uses_constants_map.contains_key(constant) {
                undefined.insert(constant);
            }
        }
    }
    for constant in undefined {
        println!("{}", constant);
    }
}
