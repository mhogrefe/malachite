// Copyright © 2024 Mikhail Hogrefe
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
                &[Caption(&escape_label_string(options.name)), Color(options.color)],
            );
        }
    }
    fg.echo_to_file(&options.file_name);
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
