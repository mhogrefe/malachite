// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::macros::root_macro_call_first_node;
use clippy_utils::source::snippet_opt;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags the long ways of writing what a `to_*_string` method already says: a `to_latex` or
    /// `to_typst` wrapper immediately turned into a `String`, and a `format!` whose whole format
    /// string is a single `{:?}`, `{:b}`, `{:o}`, `{:x}`, or `{:X}`.
    ///
    /// ### Why is this bad?
    ///
    /// These methods exist so that a caller who wants the string itself, rather than something to
    /// write into a formatter, can say so in one call. The long forms make the reader assemble the
    /// meaning from two pieces, and the `format!` ones hide it inside a format string.
    ///
    /// A method's own definition is left alone: `to_binary_string` is *written* with
    /// `format!("{self:b}")`, and telling it to call itself would be absurd.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// x.to_latex().to_string()
    /// format!("{x:b}")
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// x.to_latex_string()
    /// x.to_binary_string()
    /// ```
    pub USE_TO_STRING_VARIANT,
    Deny,
    "spelling out what a `to_*_string` method already says"
}

declare_lint_pass!(UseToStringVariant => [USE_TO_STRING_VARIANT]);

// Wrappers whose only purpose is to be written or turned into a `String`.
const WRAPPERS: [&str; 2] = ["to_latex", "to_typst"];

// The format specifiers that have a method of their own, and that method's name.
const SPECS: [(&str, &str); 5] = [
    ("?", "to_debug_string"),
    ("b", "to_binary_string"),
    ("o", "to_octal_string"),
    ("x", "to_lower_hex_string"),
    ("X", "to_upper_hex_string"),
];

// The name of the function an expression sits in, if it has one. A method written in terms of the
// very shape this lint flags is the one place the shape belongs.
fn enclosing_fn_name(cx: &LateContext<'_>, expr: &Expr<'_>) -> Option<String> {
    let owner = cx.tcx.hir_get_parent_item(expr.hir_id);
    Some(cx.tcx.opt_item_name(owner.to_def_id())?.to_string())
}

// Reads the format string out of a `format!` call's source, and gives the specifier if the whole
// string is one placeholder and nothing else. Anything more -- literal text, a width, a second
// placeholder -- has no method to replace it, so it is not this lint's business.
fn lone_spec(src: &str) -> Option<&'static str> {
    let open = src.find('(')?;
    let rest = src[open + 1..].trim_start();
    let quote = rest.strip_prefix('"')?;
    let close = quote.find('"')?;
    let format_string = &quote[..close];
    let after = quote[close + 1..].trim();
    // Either `format!("{x:b}")` or `format!("{:b}", x)`, and nothing more elaborate.
    let tail_ok = after == ")" || (after.starts_with(',') && after.ends_with(')'));
    if !tail_ok {
        return None;
    }
    let inner = format_string.strip_prefix('{')?.strip_suffix('}')?;
    let (name, spec) = inner.split_once(':')?;
    // The name, if there is one, is a captured identifier; a positional index is not a value this
    // lint can name in its suggestion.
    if !name.is_empty() && !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return None;
    }
    SPECS.iter().find(|(s, _)| *s == spec).map(|(_, m)| *m)
}

impl<'tcx> LateLintPass<'tcx> for UseToStringVariant {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let Some(mac) = root_macro_call_first_node(cx, expr)
            && cx.tcx.item_name(mac.def_id).as_str() == "format"
            && let Some(src) = snippet_opt(cx, mac.span)
            && let Some(method) = lone_spec(&src)
        {
            // The method's own body is written this way; see the lint docs.
            if enclosing_fn_name(cx, expr).as_deref() == Some(method) {
                return;
            }
            span_lint_and_help(
                cx,
                USE_TO_STRING_VARIANT,
                mac.span,
                format!("this is what `{method}` says"),
                None,
                format!("use `{method}` instead"),
            );
            return;
        }
        if expr.span.from_expansion() {
            return;
        }
        let ExprKind::MethodCall(seg, receiver, [], _) = expr.kind else {
            return;
        };
        if seg.ident.name.as_str() != "to_string" {
            return;
        }
        let ExprKind::MethodCall(inner, _, [], _) = receiver.kind else {
            return;
        };
        let name = inner.ident.name.as_str();
        if !WRAPPERS.contains(&name) {
            return;
        }
        let method = format!("{name}_string");
        if enclosing_fn_name(cx, expr).as_deref() == Some(method.as_str()) {
            return;
        }
        span_lint_and_help(
            cx,
            USE_TO_STRING_VARIANT,
            expr.span,
            format!("this is what `{method}` says"),
            None,
            format!("use `{method}` instead"),
        );
    }
}
