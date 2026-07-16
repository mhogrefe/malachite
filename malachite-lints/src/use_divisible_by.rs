// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags divisibility tests of an integer (a primitive, `Natural`, or `Integer`) spelled as
    /// `x % b == 0` or `x % b != 0`.
    ///
    /// ### Why is this bad?
    ///
    /// `divisible_by(b)` says what is being asked, and for the bignum types it avoids computing and
    /// allocating the full remainder. (The `b == 2` case has its own `even()`/`odd()` spelling; see
    /// `use_parity`.)
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if x % b == 0 { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if x.divisible_by(b) { .. }
    /// ```
    pub USE_DIVISIBLE_BY,
    Deny,
    "testing divisibility with `% b == 0` instead of `divisible_by(b)`"
}

declare_lint_pass!(UseDivisibleBy => [USE_DIVISIBLE_BY]);

// Whether `ty` (references already peeled) has `divisible_by`: a primitive integer, or a `Natural`
// or `Integer`.
fn has_divisible_by<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> bool {
    ty.is_integral() || matches!(crate::bignum_name(cx, ty), Some("Natural" | "Integer"))
}

impl<'tcx> LateLintPass<'tcx> for UseDivisibleBy {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // `x % b == 0` or `x % b != 0`.
        let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
            return;
        };
        if !matches!(op.node, BinOpKind::Eq | BinOpKind::Ne) {
            return;
        }
        for (rem, zero) in [(lhs, rhs), (rhs, lhs)] {
            if crate::literal_value(zero) != Some(0) {
                continue;
            }
            let ExprKind::Binary(rem_op, x, b) = rem.kind else {
                continue;
            };
            if rem_op.node != BinOpKind::Rem {
                continue;
            }
            // `% 2` (including `% T::TWO`) is `use_parity`'s job.
            if crate::is_int_const(cx, b, 2, "TWO") {
                continue;
            }
            let x_inner = crate::peel_clone_and_borrows(x);
            if !has_divisible_by(cx, cx.typeck_results().expr_ty(x_inner).peel_refs()) {
                continue;
            }
            let bang = if op.node == BinOpKind::Eq { "" } else { "!" };
            span_lint_and_help(
                cx,
                USE_DIVISIBLE_BY,
                expr.span,
                "testing divisibility with `% b == 0`",
                None,
                format!(
                    "use `{bang}{}.divisible_by({})`",
                    snippet(cx, x_inner.span, ".."),
                    snippet(cx, b.span, ".."),
                ),
            );
            return;
        }
    }
}
