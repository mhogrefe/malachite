// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::eq_expr_value;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags multiplying a bignum by itself (`&x * &x`) and raising one to the power of 2
    /// (`x.pow(2)`, `x.pow_assign(2)`).
    ///
    /// ### Why is this bad?
    ///
    /// Squaring has a dedicated, faster implementation: use `square()`, `(&x).square()`, or
    /// `square_assign()`.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = &x * &x;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = (&x).square();
    /// ```
    pub USE_SQUARE,
    Deny,
    "multiplying a bignum by itself or raising it to the power of 2 instead of squaring"
}

declare_lint_pass!(UseSquare => [USE_SQUARE]);

impl<'tcx> LateLintPass<'tcx> for UseSquare {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) if op.node == BinOpKind::Mul => {
                let l = crate::peel_clone_and_borrows(lhs);
                let r = crate::peel_clone_and_borrows(rhs);
                if crate::bignum_name(cx, cx.typeck_results().expr_ty(l).peel_refs()).is_some()
                    && eq_expr_value(cx, l, r)
                {
                    span_lint(
                        cx,
                        USE_SQUARE,
                        expr.span,
                        "use `square()` (or `square_assign()`) instead of multiplying a value \
                        by itself",
                    );
                }
            }
            ExprKind::MethodCall(seg, recv, [arg], _) => {
                let name = seg.ident.name.as_str();
                if !matches!(name, "pow" | "pow_assign") {
                    return;
                }
                if crate::literal_value(arg) != Some(2) {
                    return;
                }
                if crate::bignum_name(cx, cx.typeck_results().expr_ty(recv).peel_refs()).is_none() {
                    return;
                }
                let advice = if name == "pow" {
                    "use `square()` instead of `pow(2)`"
                } else {
                    "use `square_assign()` instead of `pow_assign(2)`"
                };
                span_lint(cx, USE_SQUARE, expr.span, advice);
            }
            _ => {}
        }
    }
}
