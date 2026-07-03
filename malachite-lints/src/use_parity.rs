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
    /// Flags parity tests of a `Natural` or `Integer` spelled as `x % 2 == 0` (or `!= 0`, or
    /// compared with 1) or as `divisible_by(2)`.
    ///
    /// ### Why is this bad?
    ///
    /// `even()` and `odd()` say what is being asked and only inspect the lowest bit.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if &x % 2u32 == 0 { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if x.even() { .. }
    /// ```
    pub USE_PARITY,
    Deny,
    "testing the parity of a bignum with `% 2` or `divisible_by(2)` instead of `even()`/`odd()`"
}

declare_lint_pass!(UseParity => [USE_PARITY]);

// If `e` is `x % 2` (a literal 2 or the `TWO` constant) where `x` is a `Natural` or an
// `Integer`, returns `x`.
fn rem_2<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Binary(op, l, r) = e.kind else {
        return None;
    };
    if op.node != BinOpKind::Rem || !is_two(cx, r) {
        return None;
    }
    let x = crate::peel_clone_and_borrows(l);
    matches!(
        crate::bignum_name(cx, cx.typeck_results().expr_ty(x).peel_refs()),
        Some("Natural" | "Integer")
    )
    .then_some(x)
}

// Whether `e` is a literal `2` or a path to a constant named `TWO`.
fn is_two(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let e = crate::peel_clone_and_borrows(e);
    if crate::literal_value(e) == Some(2) {
        return true;
    }
    let ExprKind::Path(qpath) = &e.kind else {
        return false;
    };
    let Some(did) = cx.qpath_res(qpath, e.hir_id).opt_def_id() else {
        return false;
    };
    matches!(
        cx.tcx.def_kind(did),
        DefKind::Const { .. } | DefKind::AssocConst { .. }
    ) && cx.tcx.item_name(did).as_str() == "TWO"
}

impl<'tcx> LateLintPass<'tcx> for UseParity {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) if matches!(op.node, BinOpKind::Eq | BinOpKind::Ne) => {
                for (a, b) in [(lhs, rhs), (rhs, lhs)] {
                    if rem_2(cx, a).is_none() {
                        continue;
                    }
                    let Some(k) = crate::literal_value(b) else {
                        continue;
                    };
                    let advice = match (op.node, k) {
                        (BinOpKind::Eq, 0) | (BinOpKind::Ne, 1) => "even()",
                        (BinOpKind::Ne, 0) | (BinOpKind::Eq, 1) => "odd()",
                        _ => continue,
                    };
                    span_lint(
                        cx,
                        USE_PARITY,
                        expr.span,
                        format!("use `{advice}` instead of comparing `% 2` with 0 or 1"),
                    );
                    return;
                }
            }
            ExprKind::MethodCall(seg, recv, [arg], _) => {
                if seg.ident.name.as_str() != "divisible_by" {
                    return;
                }
                if !matches!(
                    crate::bignum_name(cx, cx.typeck_results().expr_ty(recv).peel_refs()),
                    Some("Natural" | "Integer")
                ) {
                    return;
                }
                if is_two(cx, arg) {
                    span_lint(
                        cx,
                        USE_PARITY,
                        expr.span,
                        "use `even()` instead of `divisible_by(2)`",
                    );
                }
            }
            _ => {}
        }
    }
}
