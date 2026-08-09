// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::eq_expr_value;
use rustc_hir::{BinOpKind, Block, Expr, ExprKind, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags computing a quotient and a remainder of the same two operands in adjacent `let`
    /// statements, like
    ///
    /// ```rust,ignore
    /// let q = a / b;
    /// let r = a % b;
    /// ```
    ///
    /// ### Why is this bad?
    ///
    /// One `div_mod` or `div_rem` call produces both, and a single division instruction computes
    /// both on most targets: writing them separately asks for the work twice.
    ///
    /// For unsigned types the two traits agree, and `div_mod` is the house spelling. For signed
    /// types they differ, and `/` and `%` truncate toward zero, so `div_rem` is the equivalent —
    /// `div_mod` would floor instead, changing the results for negative operands.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let q = a / b;
    /// let r = a % b;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let (q, r) = a.div_mod(b);
    /// ```
    pub USE_DIV_MOD,
    Deny,
    "computing a quotient and a remainder of the same operands separately"
}

declare_lint_pass!(UseDivMod => [USE_DIV_MOD]);

// If `e` is a division or a remainder, returns its operator, left operand, and right operand.
fn div_or_rem<'tcx>(
    e: &'tcx Expr<'tcx>,
) -> Option<(BinOpKind, &'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
    let ExprKind::Binary(op, lhs, rhs) = e.kind else {
        return None;
    };
    matches!(op.node, BinOpKind::Div | BinOpKind::Rem).then_some((op.node, lhs, rhs))
}

// The companion to suggest for operands of type `ty`. `/` and `%` truncate toward zero, which is
// what `div_rem` does; `div_mod` agrees only when the operands cannot be negative.
fn suggestion<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<&'static str> {
    let unsigned = match ty.kind() {
        rustc_middle::ty::Uint(_) => true,
        rustc_middle::ty::Int(_) => false,
        rustc_middle::ty::Adt(..) => match crate::bignum_name(cx, ty) {
            Some("Natural") => true,
            Some(_) => false,
            None => return None,
        },
        _ => return None,
    };
    let name = if unsigned { "div_mod" } else { "div_rem" };
    let path = format!(
        "malachite_base::num::arithmetic::traits::{}",
        if unsigned { "DivMod" } else { "DivRem" }
    );
    crate::implements_trait_with_self_rhs(cx, ty, &path).then_some(name)
}

impl<'tcx> LateLintPass<'tcx> for UseDivMod {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        for (s1, s2) in block.stmts.iter().zip(block.stmts.iter().skip(1)) {
            if s1.span.from_expansion() || crate::in_test_code(cx, s1.span) {
                continue;
            }
            let (StmtKind::Let(l1), StmtKind::Let(l2)) = (&s1.kind, &s2.kind) else {
                continue;
            };
            let (Some(e1), Some(e2)) = (l1.init, l2.init) else {
                continue;
            };
            let (Some((op1, lhs1, rhs1)), Some((op2, lhs2, rhs2))) =
                (div_or_rem(e1), div_or_rem(e2))
            else {
                continue;
            };
            // One quotient and one remainder, in either order, of the same two operands. Operand
            // equality is checked with `eq_expr_value`, which also rules out side effects.
            if op1 == op2 || !eq_expr_value(cx, lhs1, lhs2) || !eq_expr_value(cx, rhs1, rhs2) {
                continue;
            }
            let Some(name) = suggestion(cx, cx.typeck_results().expr_ty(lhs1).peel_refs()) else {
                continue;
            };
            span_lint(
                cx,
                USE_DIV_MOD,
                s1.span.to(s2.span),
                format!(
                    "use `{name}()` instead of computing the quotient and remainder separately"
                ),
            );
        }
    }
}
