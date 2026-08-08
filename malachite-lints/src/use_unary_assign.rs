// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::eq_expr_value;
use rustc_hir::{Expr, ExprKind, UnOp};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags assigning a value's own negation or complement back to itself (`x = -x`, `x = !x`)
    /// when the type has the in-place variant (`NegAssign`, `NotAssign`).
    ///
    /// ### Why is this bad?
    ///
    /// The in-place variants say what is happening directly and avoid spelling the place twice;
    /// for bignums they also reuse the existing allocation.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// mdet_pos = !mdet_pos;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// mdet_pos.not_assign();
    /// ```
    pub USE_UNARY_ASSIGN,
    Deny,
    "assigning a value's negation or complement back to itself instead of using the in-place \
    variant"
}

declare_lint_pass!(UseUnaryAssign => [USE_UNARY_ASSIGN]);

impl<'tcx> LateLintPass<'tcx> for UseUnaryAssign {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Assign(lhs, rhs, _) = expr.kind else {
            return;
        };
        let ExprKind::Unary(op, operand) = rhs.kind else {
            return;
        };
        let (base, symbol) = match op {
            UnOp::Not => ("not", "!"),
            UnOp::Neg => ("neg", "-"),
            UnOp::Deref => return,
        };
        let operand = crate::peel_clone_and_borrows(operand);
        if !eq_expr_value(cx, lhs, operand) {
            return;
        }
        // `!` and `-` also work on types with no in-place companion; only suggest one when the
        // left-hand side's type actually implements it.
        let Some(method) = crate::assign_trait_impl(cx, cx.typeck_results().expr_ty(lhs), base)
        else {
            return;
        };
        // `*self = !*self` inside `not_assign` itself is that method's definition, not a place to
        // call it.
        let owner = cx.tcx.hir_enclosing_body_owner(expr.hir_id);
        if cx
            .tcx
            .opt_item_name(owner.to_def_id())
            .is_some_and(|name| name.as_str() == method)
        {
            return;
        }
        span_lint(
            cx,
            USE_UNARY_ASSIGN,
            expr.span,
            format!("use `{method}()` (in place) instead of `x = {symbol}x`"),
        );
    }
}
