// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! Recording unit-test rows for the oracles in `oracle-test`.
//!
//! A unit test calls [`oracle_row!`](crate::oracle_row) on each of its rows, with the oracle
//! backend (`"azurite"` or `"flint"`), the oracle mode, and the row in the format that the mode
//! reads, such as
//!
//! ```text
//! (0x1.0#1).atan2_prec_round(0x1.0#1, 10, Floor) = (0x0.c90#10, Less)
//! ```
//!
//! Without the `oracle` feature the macro expands to nothing. With it, and with the
//! `MALACHITE_ORACLE_DUMP` environment variable naming a directory, each row is appended to
//! `<directory>/<mode>.txt`. `oracle-test` finds the tests that call the macro, runs them that way,
//! and hands each file to its oracle mode, so the rows exist only in the tests.
//!
//! The functions `not`, `eq_symbol`, `cmp_symbol`, `partial_cmp_symbol`, `partial_cmp_row`, and
//! `option` help with rows whose wording depends on the expected result. Since the macro drops its
//! arguments without the `oracle` feature, they are called inside it by their full paths, as in
//!
//! ```text
//! oracle_row!(
//!     "azurite",
//!     "az_nat_parity",
//!     "{n} is {}even",
//!     malachite_base::test_util::oracle_rows::not(out)
//! );
//! ```

/// Records a unit-test row for an oracle; see [`test_util::oracle_rows`](self).
#[cfg(feature = "oracle")]
#[macro_export]
macro_rules! oracle_row {
    ($backend: expr, $mode: expr, $($format: tt)*) => {
        $crate::test_util::oracle_rows::record($backend, $mode, &format!($($format)*))
    };
}

/// Records a unit-test row for an oracle; see [`test_util::oracle_rows`](self). Without the
/// `oracle` feature, it does nothing.
#[cfg(not(feature = "oracle"))]
#[macro_export]
macro_rules! oracle_row {
    ($backend: expr, $mode: expr, $($format: tt)*) => {};
}

#[cfg(feature = "oracle")]
use core::cmp::Ordering::{self, *};
#[cfg(feature = "oracle")]
use core::fmt::Display;
#[cfg(feature = "oracle")]
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Mutex, PoisonError},
};

/// `""` if `b` is true and `"not "` otherwise, for rows such as `3 is not even`.
#[cfg(feature = "oracle")]
pub const fn not(b: bool) -> &'static str {
    if b { "" } else { "not " }
}

/// `"="` if `b` is true and `"≠"` otherwise, for rows such as `3 ≠ 4`.
#[cfg(feature = "oracle")]
pub const fn eq_symbol(b: bool) -> &'static str {
    if b { "=" } else { "≠" }
}

/// `"<"`, `"="`, or `">"`, for rows such as `3 < 4`.
#[cfg(feature = "oracle")]
pub const fn cmp_symbol(o: Ordering) -> &'static str {
    match o {
        Less => "<",
        Equal => "=",
        Greater => ">",
    }
}

/// `"<"`, `"="`, or `">"`, for rows such as `3 < 4`.
///
/// # Panics
/// Panics if `o` is `None`.
#[cfg(feature = "oracle")]
pub const fn partial_cmp_symbol(o: Option<Ordering>) -> &'static str {
    cmp_symbol(o.unwrap())
}

/// `x < y`, `x = y`, or `x > y`, or `x and y are incomparable` if `o` is `None`.
#[cfg(feature = "oracle")]
pub fn partial_cmp_row<T: Display, U: Display>(x: T, y: U, o: Option<Ordering>) -> String {
    match o {
        Some(o) => format!("{x} {} {y}", cmp_symbol(o)),
        None => format!("{x} and {y} are incomparable"),
    }
}

/// `Some(x)` or `None`, with `x` in its `Display` form.
#[cfg(feature = "oracle")]
pub fn option<T: Display>(x: Option<T>) -> String {
    match x {
        Some(x) => format!("Some({x})"),
        None => "None".to_string(),
    }
}

#[cfg(feature = "oracle")]
static FILES: Mutex<Option<HashMap<String, File>>> = Mutex::new(None);

/// Appends `row` to `<MALACHITE_ORACLE_DUMP>/<mode>.txt`; does nothing when `MALACHITE_ORACLE_DUMP`
/// is unset. Called by [`oracle_row!`](crate::oracle_row).
///
/// # Panics
/// Panics if `backend` is not `"azurite"` or `"flint"`, or if the file cannot be written.
#[cfg(feature = "oracle")]
pub fn record(backend: &str, mode: &str, row: &str) {
    assert!(
        backend == "azurite" || backend == "flint",
        "unknown oracle backend {backend:?}"
    );
    let Some(dir) = std::env::var_os("MALACHITE_ORACLE_DUMP") else {
        return;
    };
    let mut files = FILES.lock().unwrap_or_else(PoisonError::into_inner);
    let file = files
        .get_or_insert_with(HashMap::new)
        .entry(mode.to_string())
        .or_insert_with(|| {
            let path = PathBuf::from(dir).join(format!("{mode}.txt"));
            File::options()
                .create(true)
                .append(true)
                .open(&path)
                .unwrap_or_else(|e| panic!("cannot open {path:?}: {e}"))
        });
    writeln!(file, "{row}").unwrap();
}
