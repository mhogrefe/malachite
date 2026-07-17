// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::eq_expr_value;
use clippy_utils::higher::While;
use rustc_hir::{AssignOpKind, BinOpKind, Expr, ExprKind, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a loop that strips the trailing zero bits off an integer one at a time, such as
    /// `while x.even() { x >>= 1; k -= 1; }`.
    ///
    /// ### Why is this bad?
    ///
    /// This re-implements `trailing_zeros`. Computing the shift amount once with
    /// `x.trailing_zeros()` and then shifting (and adjusting any step counter) by that amount is
    /// clearer and does the work in a single step instead of one iteration per bit.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// while x.even() {
    ///     x >>= 1;
    ///     k -= 1;
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let zeros = x.trailing_zeros();
    /// x >>= zeros;
    /// k -= zeros;
    /// ```
    pub USE_TRAILING_ZEROS,
    Deny,
    "a loop that strips trailing zero bits one at a time instead of using `trailing_zeros()`"
}

declare_lint_pass!(UseTrailingZeros => [USE_TRAILING_ZEROS]);

// If `cond` tests an integer `x` for evenness -- `x.even()`, `x & 1 == 0`, or `x % 2 == 0` (either
// operand order) -- returns the place expression `x`.
fn even_test_place<'tcx>(
    cx: &LateContext<'tcx>,
    cond: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    match cond.kind {
        ExprKind::MethodCall(seg, recv, [], _) if seg.ident.name.as_str() == "even" => Some(recv),
        ExprKind::Binary(op, l, r) if op.node == BinOpKind::Eq => {
            for (a, b) in [(l, r), (r, l)] {
                if crate::literal_value(b) == Some(0)
                    && let ExprKind::Binary(inner, x, k) = a.kind
                {
                    let is_low_bit = match inner.node {
                        BinOpKind::BitAnd => crate::is_int_const(cx, k, 1, "ONE"),
                        BinOpKind::Rem => crate::is_int_const(cx, k, 2, "TWO"),
                        _ => false,
                    };
                    if is_low_bit {
                        return Some(x);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for UseTrailingZeros {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let Some(while_loop) = While::hir(expr) else {
            return;
        };
        // The condition span is the user's source (the loop header itself is a desugaring); skip
        // macro-generated loops and test code.
        if while_loop.condition.span.from_expansion()
            || crate::in_test_code(cx, while_loop.condition.span)
        {
            return;
        }
        let Some(x) = even_test_place(cx, while_loop.condition) else {
            return;
        };
        let ExprKind::Block(block, _) = while_loop.body.kind else {
            return;
        };
        // Collect the body's effect expressions; bail on any `let`, item, or trailing control flow
        // that would make this more than a bit-stripping loop.
        let mut effects = Vec::new();
        for stmt in block.stmts {
            match stmt.kind {
                StmtKind::Semi(e) | StmtKind::Expr(e) => effects.push(e),
                _ => return,
            }
        }
        if let Some(e) = block.expr {
            effects.push(e);
        }
        if effects.is_empty() || effects.len() > 2 {
            return;
        }
        // The body must be exactly one `x >>= 1` (on the tested integer), plus at most one
        // `counter += 1` / `counter -= 1` on a *different* place.
        let mut shifts = 0;
        for e in effects {
            let ExprKind::AssignOp(op, lhs, rhs) = e.kind else {
                return;
            };
            match op.node {
                AssignOpKind::ShrAssign
                    if crate::is_int_const(cx, rhs, 1, "ONE") && eq_expr_value(cx, lhs, x) =>
                {
                    shifts += 1;
                }
                AssignOpKind::AddAssign | AssignOpKind::SubAssign
                    if crate::is_int_const(cx, rhs, 1, "ONE") && !eq_expr_value(cx, lhs, x) => {}
                _ => return,
            }
        }
        if shifts != 1 {
            return;
        }
        span_lint_and_help(
            cx,
            USE_TRAILING_ZEROS,
            while_loop.span,
            "this loop strips trailing zero bits one at a time",
            None,
            "compute the shift amount once with `trailing_zeros()`, then shift (and adjust any \
             counter) by that amount",
        );
    }
}
