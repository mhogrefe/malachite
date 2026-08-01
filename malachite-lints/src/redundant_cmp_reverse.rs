// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags `x.cmp(&y).reverse()`, and the same shape with `cmp_abs` or `total_cmp`, where
    /// swapping the operands says the same thing.
    ///
    /// ### Why is this bad?
    ///
    /// `y.cmp(&x)` is what the code means; reversing afterwards makes the reader carry the flip in
    /// their head, and the reversal is easy to lose track of when the result feeds a `match`.
    ///
    /// Only comparisons that are symmetric in their operands qualify. The `*_double` family is
    /// deliberately absent: `a.cmp_double(&b)` compares `a` with *twice* `b`, so
    /// `a.cmp_double(&b).reverse()` compares twice `b` with `a` — which is not
    /// `b.cmp_double(&a)`, a comparison of `b` with twice `a`. There, `.reverse()` is the correct
    /// way to flip the operands and must stay.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// x.cmp(&y).reverse()
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// y.cmp(&x)
    /// ```
    pub REDUNDANT_CMP_REVERSE,
    Deny,
    "reversing a symmetric comparison instead of swapping its operands"
}

declare_lint_pass!(RedundantCmpReverse => [REDUNDANT_CMP_REVERSE]);

// Comparisons for which `a.f(&b).reverse()` is `b.f(&a)`: those that compare the two operands
// against each other directly. See the lint docs on why `cmp_double` and `cmp_abs_double` are not
// here.
const SYMMETRIC: [&str; 3] = ["cmp", "cmp_abs", "total_cmp"];

impl<'tcx> LateLintPass<'tcx> for RedundantCmpReverse {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        let ExprKind::MethodCall(seg, receiver, [], _) = expr.kind else {
            return;
        };
        if seg.ident.name.as_str() != "reverse" {
            return;
        }
        // `Ordering::reverse` specifically, not some other type's `reverse`.
        if !crate::is_ordering_ty(cx, cx.typeck_results().expr_ty(receiver)) {
            return;
        }
        let ExprKind::MethodCall(inner, _, [_], _) = receiver.kind else {
            return;
        };
        let name = inner.ident.name.as_str();
        if !SYMMETRIC.contains(&name) {
            return;
        }
        span_lint_and_help(
            cx,
            REDUNDANT_CMP_REVERSE,
            expr.span,
            format!("swap the operands instead of reversing the result of `{name}`"),
            None,
            format!("`a.{name}(&b).reverse()` is `b.{name}(&a)`"),
        );
    }
}
