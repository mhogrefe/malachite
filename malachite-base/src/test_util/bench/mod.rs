// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::conversion::traits::ExactFrom;
use crate::test_util::bench::bucketers::Bucketer;
use gnuplot::{AxesCommon, Caption, Color, Figure};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::iter::Iterator;
use std::time::Instant;

fn escape_label_string(s: &str) -> String {
    let mut escaped = String::new();
    for c in s.chars() {
        if c == '_' {
            escaped.push_str("\\_");
        } else if c == '&' {
            escaped.push_str("\\&");
        } else {
            escaped.push(c);
        }
    }
    escaped
}

struct BenchmarkSeriesOptions<'a, T: 'a> {
    pub name: &'a str,
    pub function: &'a mut dyn FnMut(T),
    pub color: &'a str,
}

struct BenchmarkOptions<'a, I: Iterator>
where
    I::Item: 'a,
{
    pub generator: I,
    pub title: &'a str,
    pub limit: usize,
    pub bucketing_function: &'a dyn Fn(&I::Item) -> usize,
    pub x_axis_label: &'a str,
    pub y_axis_label: &'a str,
    pub file_name: String,
    pub series_options: Vec<BenchmarkSeriesOptions<'a, I::Item>>,
}

fn quick_median(xs: &mut [u64]) -> u64 {
    assert!(!xs.is_empty());
    xs.sort_unstable();
    let half_index = xs.len() >> 1;
    if xs.len().odd() {
        xs[half_index]
    } else {
        (xs[half_index - 1] + xs[half_index]) >> 1
    }
}

fn quick_mean(xs: &[u64]) -> u64 {
    assert!(!xs.is_empty());
    let sum: u64 = xs.iter().sum();
    sum / u64::exact_from(xs.len())
}

fn run_benchmark_internal<I: Iterator>(mut options: BenchmarkOptions<I>)
where
    I::Item: Clone,
{
    let reps = 5;
    let min_bucket_size = 2;

    let mut durations_maps = Vec::new();
    for _ in 0..options.series_options.len() {
        durations_maps.push(HashMap::new());
    }
    for x in options.generator.take(options.limit) {
        let size = (options.bucketing_function)(&x);
        for (i, series) in options.series_options.iter_mut().enumerate() {
            let mut durations_vec = Vec::new();
            for _ in 0..reps {
                let x = x.clone();
                let now = Instant::now();
                (series.function)(x);
                durations_vec.push(u64::exact_from(now.elapsed().as_nanos()));
            }
            let median_duration = quick_median(&mut durations_vec);
            durations_maps[i]
                .entry(size)
                .or_insert_with(Vec::new)
                .push(median_duration);
        }
    }
    let mut median_durations_maps = Vec::new();
    for durations_map in durations_maps {
        let mut median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
        for (&size, durations) in &durations_map {
            if durations.len() >= min_bucket_size {
                median_durations_map.insert(size, quick_mean(durations));
            }
        }
        median_durations_maps.push(median_durations_map);
    }

    // In addition to the gnuplot file, write the measured data as TSV (series, size, time-in-ns) so
    // that tooling can consume benchmark results. The .gp file alone doesn't suffice: its data
    // series are sidecar temp files that vanish when this process exits.
    let mut tsv = String::from("series\tsize\ttime_ns\n");
    for (median_durations_map, series) in median_durations_maps
        .iter()
        .zip(options.series_options.iter())
    {
        for (size, duration) in median_durations_map {
            tsv.push_str(&format!("{}\t{size}\t{duration}\n", series.name));
        }
    }
    std::fs::write(format!("{}.tsv", options.file_name), tsv).ok();

    let mut fg = Figure::new();
    {
        let axes = fg.axes2d();
        axes.set_title(&escape_label_string(options.title), &[]);
        axes.set_x_label(&escape_label_string(options.x_axis_label), &[]);
        axes.set_y_label(&escape_label_string(options.y_axis_label), &[]);
        for (median_durations_map, options) in median_durations_maps
            .iter()
            .zip(options.series_options.iter())
        {
            let sizes = median_durations_map
                .iter()
                .map(|entry| *entry.0)
                .collect_vec();
            let durations = median_durations_map
                .iter()
                .map(|entry| *entry.1)
                .collect_vec();
            axes.lines(
                &sizes,
                &durations,
                &[
                    Caption(&escape_label_string(options.name)),
                    Color(gnuplot::RGBString(options.color)),
                ],
            );
        }
    }
    fg.echo_to_file(&options.file_name);
    // `Figure` keeps its data in a `tempfile::TempDir` and writes the script with references to it,
    // so the `.gp` stops working the moment this process exits and its temp directory is removed.
    // Fold the data back into the script while it is still there, which is what the gnuplot crate
    // used to do and what makes an archived `.gp` replottable months later.
    inline_gp_data(&options.file_name);
}

// Rewrites `file_name`, a gnuplot script whose `plot` command reads its series from separate binary
// files, into an equivalent self-contained one: each `"<path>" binary` reference becomes `"-"
// binary`, and the files' bytes are appended after the command, in the order the references appear.
// That is gnuplot's inline-data form, so the script plots with no sidecar files.
//
// Leaves the script alone if anything is missing or unexpected; a benchmark that has already run
// should not fail over its output formatting.
fn inline_gp_data(file_name: &str) {
    let Ok(script) = std::fs::read(file_name) else {
        return;
    };
    // The plot command is the last line, and the only one that names data files.
    let Some(plot_start) = script
        .windows(5)
        .rposition(|w| w == b"plot ")
        .filter(|&i| i == 0 || script[i - 1] == b'\n')
    else {
        return;
    };
    let Ok(plot) = std::str::from_utf8(&script[plot_start..]) else {
        return;
    };
    let mut rewritten = String::with_capacity(plot.len());
    let mut data = Vec::new();
    let mut rest = plot;
    // Each data reference is a quoted path immediately followed by ` binary`. Other quoted strings
    // on the line (colors, series names) are left alone.
    while let Some(quote) = rest.find('"') {
        let after = &rest[quote + 1..];
        let Some(close) = after.find('"') else {
            break;
        };
        let (path, tail) = (&after[..close], &after[close + 1..]);
        if tail.starts_with(" binary")
            && let Ok(bytes) = std::fs::read(path)
        {
            rewritten.push_str(&rest[..quote]);
            rewritten.push_str("\"-\"");
            data.push(bytes);
        } else {
            // Not a data reference; copy it through, up to where `tail` resumes.
            rewritten.push_str(&rest[..rest.len() - tail.len()]);
        }
        rest = tail;
    }
    if data.is_empty() {
        return;
    }
    rewritten.push_str(rest);
    let mut out = script[..plot_start].to_vec();
    out.extend_from_slice(rewritten.trim_end().as_bytes());
    out.push(b'\n');
    for series in data {
        out.extend_from_slice(&series);
    }
    std::fs::write(file_name, out).ok();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BenchmarkType {
    Single,
    LibraryComparison,
    EvaluationStrategy,
    Algorithms,
}

#[allow(clippy::print_stdout, clippy::type_complexity)]
pub fn run_benchmark<'a, I: Iterator>(
    title: &'a str,
    benchmark_type: BenchmarkType,
    generator: I,
    generation_mode_name: &'a str,
    limit: usize,
    file_name: &'a str,
    bucketer: &Bucketer<'a, I::Item>,
    series: &mut [(&'a str, &'a mut dyn FnMut(I::Item))],
) where
    I::Item: Clone,
{
    assert_eq!(
        benchmark_type == BenchmarkType::Single,
        series.len() == 1,
        "Bad benchmark: {title}. \
             Benchmarks should have type Single iff they have only one series.",
    );
    if limit == 0 {
        return;
    }
    let title = match benchmark_type {
        BenchmarkType::Single => title.to_string(),
        BenchmarkType::LibraryComparison => format!("{title} library comparison"),
        BenchmarkType::EvaluationStrategy => format!("{title} evaluation strategy"),
        BenchmarkType::Algorithms => format!("{title} algorithms"),
    };
    println!("benchmarking {generation_mode_name} {title}");
    let colors = ["green", "blue", "red", "black", "orange", "yellow", "gray", "purple"];
    assert!(series.len() <= colors.len(), "not enough available colors");
    let mut series_options = Vec::new();
    for (&mut (label, ref mut function), color) in series.iter_mut().zip(colors.iter()) {
        series_options.push(BenchmarkSeriesOptions {
            name: label,
            function,
            color,
        });
    }
    let options = BenchmarkOptions {
        generator,
        title: &title,
        limit,
        bucketing_function: bucketer.bucketing_function,
        x_axis_label: &bucketer.bucketing_label,
        y_axis_label: "time (ns)",
        file_name: file_name.to_string(),
        series_options,
    };
    run_benchmark_internal(options);
}

#[allow(clippy::print_stdout, clippy::type_complexity)]
pub fn run_benchmark_old<'a, I: Iterator>(
    title: &'a str,
    benchmark_type: BenchmarkType,
    generator: I,
    generation_mode_name: &'a str,
    limit: usize,
    file_name: &'a str,
    bucketing_function: &'a dyn Fn(&I::Item) -> usize,
    bucketing_label: &'a str,
    series: &mut [(&'a str, &'a mut dyn FnMut(I::Item))],
) where
    I::Item: Clone,
{
    assert_eq!(
        benchmark_type == BenchmarkType::Single,
        series.len() == 1,
        "Bad benchmark: {title}. \
             Benchmarks should have type Single iff they have only one series.",
    );
    if limit == 0 {
        return;
    }
    let title = match benchmark_type {
        BenchmarkType::Single => title.to_string(),
        BenchmarkType::LibraryComparison => format!("{title} library comparison"),
        BenchmarkType::EvaluationStrategy => format!("{title} evaluation strategy"),
        BenchmarkType::Algorithms => format!("{title} algorithms"),
    };
    println!("benchmarking {generation_mode_name} {title}");
    let colors = ["green", "blue", "red", "black", "orange", "yellow", "gray", "purple"];
    assert!(series.len() <= colors.len(), "not enough available colors");
    let mut series_options = Vec::new();
    for (&mut (label, ref mut function), color) in series.iter_mut().zip(colors.iter()) {
        series_options.push(BenchmarkSeriesOptions {
            name: label,
            function,
            color,
        });
    }
    let options = BenchmarkOptions {
        generator,
        title: &title,
        limit,
        bucketing_function,
        x_axis_label: bucketing_label,
        y_axis_label: "time (ns)",
        file_name: format!("benchmarks/{file_name}"),
        series_options,
    };
    run_benchmark_internal(options);
}

#[macro_export]
macro_rules! no_out {
    ($e:expr) => {{
        $e;
    }};
}

pub mod bucketers;
