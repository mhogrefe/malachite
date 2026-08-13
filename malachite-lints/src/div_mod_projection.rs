// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags projecting one component out of the pair returned by a combined
    /// division-and-remainder function, such as `x.div_mod(y).0`.
    ///
    /// ### Why is this bad?
    ///
    /// The combined functions exist for callers that need both results; taking only one computes
    /// and allocates the other for nothing, and hides which result is meant. A dedicated
    /// division or remainder function says it directly.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let q = (&i).div_mod(k).0;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let q = (&i).div_round(k, Floor).0;
    /// ```
    pub DIV_MOD_PROJECTION,
    Deny,
    "projecting one component out of a combined division-and-remainder result"
}

declare_lint_pass!(DivModProjection => [DIV_MOD_PROJECTION]);

// The tuple-returning quotient-and-remainder families, with the dedicated function to use for
// each component. The quotient suggestions name the rounding mode, since plain `/` truncates
// while `div_mod` floors.
const FAMILIES: [(&str, &str, &str); 4] = [
    (
        "div_mod",
        "`div_round` with `Floor` (or `/` on unsigned values)",
        "`mod_op`",
    ),
    ("div_rem", "`/`", "`%` or `rem_op`"),
    (
        "ceiling_div_mod",
        "`div_round` with `Ceiling`",
        "`ceiling_mod`",
    ),
    (
        "ceiling_div_neg_mod",
        "`div_round` with `Ceiling`",
        "`neg_mod`",
    ),
];

impl<'tcx> LateLintPass<'tcx> for DivModProjection {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        // Demo-and-bench comparison arms and cross-function consistency properties in tests
        // project these results on purpose, measuring or asserting one component against the
        // dedicated function; they are exempt. Test-utility reference implementations are not:
        // a projection there is just an unclear way to ask for one component.
        if crate::in_bin_util_or_tests(cx, expr.hir_id) {
            return;
        }
        let ExprKind::Field(base, field) = expr.kind else {
            return;
        };
        let component = match field.as_str() {
            "0" => 0,
            "1" => 1,
            _ => return,
        };
        let ExprKind::MethodCall(seg, ..) = base.kind else {
            return;
        };
        let name = seg.ident.as_str();
        for (family, div_suggestion, mod_suggestion) in FAMILIES {
            if name == family {
                let (which, suggestion) = if component == 0 {
                    ("quotient", div_suggestion)
                } else {
                    ("remainder", mod_suggestion)
                };
                span_lint(
                    cx,
                    DIV_MOD_PROJECTION,
                    expr.span,
                    format!(
                        "projecting the {which} out of `{family}` computes the other component \
                        for nothing; use {suggestion} instead"
                    ),
                );
                return;
            }
        }
    }
}
