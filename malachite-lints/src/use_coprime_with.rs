// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::{BinOpKind, Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags comparing the result of a `gcd` call with 1 (`x.gcd(y) == 1`, `x.gcd(y) != 1`) when
    /// the receiver's type implements `CoprimeWith`.
    ///
    /// ### Why is this bad?
    ///
    /// `coprime_with` says what is being asked directly, and it can be much faster: it
    /// short-circuits on cheap divisibility checks, such as both operands being even, before
    /// falling back to a gcd.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if x.gcd(y) != 1 {
    ///     return None;
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if !x.coprime_with(y) {
    ///     return None;
    /// }
    /// ```
    pub USE_COPRIME_WITH,
    Deny,
    "comparing a gcd with 1 instead of using `coprime_with`"
}

declare_lint_pass!(UseCoprimeWith => [USE_COPRIME_WITH]);

// Whether the expression is the constant one: an integer literal `1` or a path ending in `ONE`.
fn is_one(e: &Expr<'_>) -> bool {
    if crate::literal_value(e) == Some(1) {
        return true;
    }
    match &e.kind {
        ExprKind::Path(QPath::Resolved(_, path)) => path
            .segments
            .last()
            .is_some_and(|seg| seg.ident.as_str() == "ONE"),
        ExprKind::Path(QPath::TypeRelative(_, seg)) => seg.ident.as_str() == "ONE",
        _ => false,
    }
}

impl<'tcx> LateLintPass<'tcx> for UseCoprimeWith {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
            return;
        };
        let negated = match op.node {
            BinOpKind::Eq => false,
            BinOpKind::Ne => true,
            _ => return,
        };
        let gcd_call = if is_one(rhs) {
            lhs
        } else if is_one(lhs) {
            rhs
        } else {
            return;
        };
        let ExprKind::MethodCall(seg, recv, [_], _) = gcd_call.kind else {
            return;
        };
        if seg.ident.as_str() != "gcd" {
            return;
        }
        // Only suggest when the receiver's type actually implements `CoprimeWith`.
        let ty = cx.typeck_results().expr_ty(recv).peel_refs();
        if !crate::implements_trait_with_self_rhs(
            cx,
            ty,
            "malachite_base::num::arithmetic::traits::CoprimeWith",
        ) {
            return;
        }
        // The `coprime_with` implementations and their `coprime_with_check_*` test helpers compare
        // a gcd with 1 by definition; the suggestion cannot apply to them.
        let owner = cx.tcx.hir_enclosing_body_owner(expr.hir_id);
        if cx
            .tcx
            .opt_item_name(owner.to_def_id())
            .is_some_and(|name| name.as_str().starts_with("coprime_with"))
        {
            return;
        }
        let advice = if negated {
            "use `!coprime_with()` instead of comparing a gcd with 1"
        } else {
            "use `coprime_with()` instead of comparing a gcd with 1"
        };
        span_lint(cx, USE_COPRIME_WITH, expr.span, advice);
    }
}
