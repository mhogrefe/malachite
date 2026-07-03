// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_hir;
use rustc_hir::attrs::AttributeKind;
use rustc_hir::{Attribute, CRATE_HIR_ID, HirId};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::{BytePos, FileName, SourceFile, Span, SyntaxContext};
use serde::Deserialize;

declare_lint! {
    /// ### What it does
    ///
    /// Flags source lines longer than `max_line_length` characters (default 100), ignoring
    /// trailing whitespace.
    ///
    /// ### Why is this bad?
    ///
    /// `rustfmt` keeps code within the limit, but cannot split long string literals or Markdown
    /// constructs in doc comments; this catches those.
    ///
    /// Each long line is attributed to the innermost item containing it (doc comments belong to
    /// the item they document), so a line that genuinely cannot be shortened (a long Markdown
    /// table row or link) is exempted by annotating that item with
    /// `#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]`. `expect` rather than
    /// `allow` keeps the exemptions from going stale: if the item no longer contains a long line,
    /// the unfulfilled expectation is itself reported. Crate-level `//!` doc lines have no
    /// containing item (an `#![expect]` at crate level would exempt the entire crate), so those
    /// few are listed in `dylint.toml` under `long_lines_exceptions` instead, with the same
    /// staleness guarantee.
    pub LONG_LINES,
    Deny,
    "source line exceeds the maximum length"
}

declare_lint_pass!(LongLines => [LONG_LINES]);

#[derive(Deserialize)]
struct Exception {
    file: String,
    line: usize,
}

#[derive(Deserialize)]
#[serde(default)]
struct Config {
    max_line_length: usize,
    long_lines_exceptions: Vec<Exception>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_line_length: 100,
            long_lines_exceptions: Vec::new(),
        }
    }
}

// Whether the exception path (repo-relative, e.g. `malachite-base/src/lib.rs`) refers to
// `path_str` (as rustc knows it, which may be absolute or relative to some other directory):
// require a whole-component suffix match.
fn path_matches(path_str: &str, exception_file: &str) -> bool {
    path_str == exception_file || path_str.ends_with(&format!("/{exception_file}"))
}

// The span of one source line, excluding the line terminator (`line_bounds` includes it, which
// would make the diagnostic span spill into the next line).
fn line_span(file: &SourceFile, line_index: usize, line: &str) -> Span {
    let range = file.line_bounds(line_index);
    let end = range.start + BytePos(u32::try_from(line.len()).unwrap());
    Span::new(range.start, end, SyntaxContext::root(), None)
}

// The innermost HIR owner whose extent contains `sp`, or the crate root if none does. Owner
// extents nest, so the smallest containing extent is the innermost.
fn innermost_owner(owners: &[(BytePos, BytePos, HirId)], sp: Span) -> HirId {
    owners
        .iter()
        .filter(|&&(lo, hi, _)| lo <= sp.lo() && sp.hi() <= hi)
        .min_by_key(|&&(lo, hi, _)| hi.0 - lo.0)
        .map_or(CRATE_HIR_ID, |&(_, _, hir_id)| hir_id)
}

impl<'tcx> LateLintPass<'tcx> for LongLines {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        let config: Config = dylint_linting::config_or_default(env!("CARGO_PKG_NAME"));
        // Collect every HIR owner's extent: its span joined with its attributes' spans, which
        // include doc comments. Each long line is then attributed to the innermost owner
        // containing it, which is where an `allow` or `expect` attribute takes effect.
        let mut owners: Vec<(BytePos, BytePos, HirId)> = Vec::new();
        for owner_id in cx.tcx.hir_crate_items(()).owners() {
            let hir_id = HirId::make_owner(owner_id.def_id);
            let mut span = cx.tcx.hir_span(hir_id);
            for attr in cx.tcx.hir_attrs(hir_id) {
                // Some synthetic parsed attributes have no span (`Attribute::span` panics on
                // them); only real source attributes -- unparsed ones and doc comments -- extend
                // the extent.
                match attr {
                    Attribute::Unparsed(u) => span = span.to(u.span),
                    Attribute::Parsed(AttributeKind::DocComment {
                        span: attr_span, ..
                    }) => {
                        span = span.to(*attr_span);
                    }
                    Attribute::Parsed(_) => {}
                }
            }
            owners.push((span.lo(), span.hi(), hir_id));
        }
        for file in cx.sess().source_map().files().iter() {
            if file.is_imported() {
                continue;
            }
            let FileName::Real(real) = &file.name else {
                continue;
            };
            let Some(path) = real.local_path() else {
                continue;
            };
            let path_str = path.to_string_lossy().replace('\\', "/");
            let Some(src) = file.src.as_ref() else {
                continue;
            };
            for (i, line) in src.lines().enumerate() {
                let len = line.trim_end().chars().count();
                let excepted = config
                    .long_lines_exceptions
                    .iter()
                    .any(|e| e.line == i + 1 && path_matches(&path_str, &e.file));
                if len > config.max_line_length {
                    if !excepted {
                        let sp = line_span(file, i, line);
                        span_lint_hir(
                            cx,
                            LONG_LINES,
                            innermost_owner(&owners, sp),
                            sp,
                            format!(
                                "line is {len} characters long (max {})",
                                config.max_line_length
                            ),
                        );
                    }
                } else if excepted {
                    let sp = line_span(file, i, line);
                    span_lint_hir(
                        cx,
                        LONG_LINES,
                        innermost_owner(&owners, sp),
                        sp,
                        "stale `long_lines` exception: this line is not too long",
                    );
                }
            }
        }
    }
}
