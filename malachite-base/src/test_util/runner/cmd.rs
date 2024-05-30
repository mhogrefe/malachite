// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::generators::common::{GenConfig, GenMode};
use clap::{App, Arg};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct CommandLineArguments {
    pub codegen_key: Option<String>,
    pub demo_key: Option<String>,
    pub bench_key: Option<String>,
    pub generation_mode: GenMode,
    pub config: GenConfig,
    pub limit: usize,
    pub out: String,
}

pub fn read_command_line_arguments(name: &str) -> CommandLineArguments {
    let matches = App::new(name)
        .version("0.1.0")
        .author("Mikhail Hogrefe <mikhailhogrefe@gmail.com>")
        .about("Runs demos and benchmarks for malachite-base functions.")
        .arg(
            Arg::with_name("generation_mode")
                .short("m")
                .long("generation_mode")
                .help("May be 'exhaustive', 'random', or 'special_random'.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("e.g. 'mean_run_length_n 4 mean_run_length_d 1'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .help("Specifies the maximum number of elements to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .help("Specifies the file name to write a benchmark to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("demo")
                .short("d")
                .long("demo")
                .help("Specifies the demo name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bench")
                .short("b")
                .long("bench")
                .help("Specifies the benchmark name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("codegen")
                .short("g")
                .long("codegen")
                .help("Specifies the code to generate")
                .takes_value(true),
        )
        .get_matches();

    let generation_mode = match matches.value_of("generation_mode").unwrap_or("exhaustive") {
        "exhaustive" => GenMode::Exhaustive,
        "random" => GenMode::Random,
        "special_random" => GenMode::SpecialRandom,
        _ => panic!("Invalid generation mode"),
    };
    let config_string = matches.value_of("config").unwrap_or("");
    let mut config = GenConfig::new();
    if !config_string.is_empty() {
        for mut chunk in &config_string.split(' ').chunks(2) {
            let key = chunk.next().unwrap();
            let value =
                u64::from_str(chunk.next().expect("Bad config")).expect("Invalid config value");
            config.insert(key, value);
        }
    }
    let limit =
        usize::from_str(matches.value_of("limit").unwrap_or("10000")).expect("Invalid limit");
    let out = matches.value_of("out").unwrap_or("temp.gp").to_string();
    let demo_key = matches.value_of("demo").map(ToString::to_string);
    let bench_key = matches.value_of("bench").map(ToString::to_string);
    let codegen_key = matches.value_of("codegen").map(ToString::to_string);
    assert!(
        demo_key.is_some() || bench_key.is_some() || codegen_key.is_some(),
        "Must specify demo, bench, or codegen"
    );
    CommandLineArguments {
        codegen_key,
        demo_key,
        bench_key,
        generation_mode,
        config,
        limit,
        out,
    }
}
