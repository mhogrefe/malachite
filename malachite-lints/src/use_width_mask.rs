// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BinOpKind, Expr, ExprKind, Node};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a remainder by a type's bit width, `x % T::WIDTH`.
    ///
    /// ### Why is this bad?
    ///
    /// `T::WIDTH` is a power of two, so the remainder is a bit mask: `x & T::WIDTH_MASK` computes
    /// the same value with a single `and` instead of a division, and `WIDTH_MASK` exists precisely
    /// for this.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let bit = i % Limb::WIDTH;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let bit = i & Limb::WIDTH_MASK;
    /// ```
    pub USE_WIDTH_MASK,
    Deny,
    "taking a remainder by `T::WIDTH` instead of masking with `T::WIDTH_MASK`"
}

declare_lint_pass!(UseWidthMask => [USE_WIDTH_MASK]);

// Whether `e` is a path to an associated constant named `WIDTH`.
fn is_width(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let ExprKind::Path(qpath) = &e.kind else {
        return false;
    };
    matches!(cx.qpath_res(qpath, e.hir_id), Res::Def(DefKind::AssocConst { .. }, did)
        if cx.tcx.item_name(did).as_str() == "WIDTH")
}

impl<'tcx> LateLintPass<'tcx> for UseWidthMask {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Binary(op, x, divisor) = expr.kind else {
            return;
        };
        if op.node != BinOpKind::Rem || !is_width(cx, divisor) {
            return;
        }
        // The dividend is unsigned (`WIDTH` is a `u64`), so `%` and `&` agree.
        if !cx.typeck_results().expr_ty(x).peel_refs().is_integral() {
            return;
        }
        // `x % T::WIDTH == 0` is a divisibility test; leave it to `use_divisible_by`.
        if let Node::Expr(parent) = cx.tcx.parent_hir_node(expr.hir_id)
            && let ExprKind::Binary(pop, pl, pr) = parent.kind
            && matches!(pop.node, BinOpKind::Eq | BinOpKind::Ne)
            && (crate::literal_value(pl) == Some(0) || crate::literal_value(pr) == Some(0))
        {
            return;
        }
        span_lint_and_help(
            cx,
            USE_WIDTH_MASK,
            expr.span,
            "taking a remainder by `T::WIDTH`",
            None,
            format!(
                "use `{} & {}`",
                snippet(cx, x.span, ".."),
                snippet(cx, divisor.span, "..").replace("WIDTH", "WIDTH_MASK"),
            ),
        );
    }
}
