// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::def::DefKind;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags dividing `ONE` by a `Rational` or a `Float`, like `Float::ONE / x`.
    ///
    /// ### Why is this bad?
    ///
    /// Taking the reciprocal has a dedicated implementation: use `reciprocal()`,
    /// `reciprocal_assign()`, or (for `Float`) the `reciprocal_prec*` family when a specific
    /// output precision is wanted.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = Float::ONE / x;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = x.reciprocal();
    /// ```
    pub USE_RECIPROCAL,
    Deny,
    "dividing `ONE` by a value instead of taking its reciprocal"
}

declare_lint_pass!(UseReciprocal => [USE_RECIPROCAL]);

impl<'tcx> LateLintPass<'tcx> for UseReciprocal {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Binary(op, num, _) = expr.kind else {
            return;
        };
        if op.node != BinOpKind::Div {
            return;
        }
        let num = crate::peel_clone_and_borrows(num);
        let ExprKind::Path(qpath) = &num.kind else {
            return;
        };
        let Some(did) = cx.qpath_res(qpath, num.hir_id).opt_def_id() else {
            return;
        };
        if !matches!(
            cx.tcx.def_kind(did),
            DefKind::Const { .. } | DefKind::AssocConst { .. }
        ) || cx.tcx.item_name(did).as_str() != "ONE"
        {
            return;
        }
        // Only `Rational` and `Float` have `Reciprocal`.
        let Some(name @ ("Rational" | "Float")) =
            crate::bignum_name(cx, cx.typeck_results().expr_ty(num).peel_refs())
        else {
            return;
        };
        let extra = if name == "Float" {
            " (or `reciprocal_prec*` for a specific precision)"
        } else {
            ""
        };
        span_lint(
            cx,
            USE_RECIPROCAL,
            expr.span,
            format!("use `reciprocal()`{extra} instead of dividing `{name}::ONE` by a value"),
        );
    }
}
