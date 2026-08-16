// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::eq_expr_value;
use rustc_hir::{Expr, ExprKind, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags `if`/`else if`/`else` chains in which every branch consists of a single
    /// assignment to the same target, suggesting a single assignment of the conditional
    /// expression.
    ///
    /// ### Why is this bad?
    ///
    /// Repeating `x = ` in every branch obscures that the chain computes one value; assigning
    /// the `if` expression once names the target a single time and lets the compiler check
    /// that every branch produces a value.
    ///
    /// ### Known problems
    ///
    /// Only chains in which each branch's final statement is a plain assignment are
    /// recognized; compound assignments and differing targets are not flagged. Statements
    /// before the assignment are fine: they move into the corresponding arm of the factored
    /// expression. A chain with no final `else` cannot be rewritten and is not flagged.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if c {
    ///     x = a;
    /// } else {
    ///     x = b;
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// x = if c { a } else { b };
    /// ```
    pub FACTOR_OUT_ASSIGNMENT,
    Deny,
    "every branch of an if chain assigns to the same target"
}

declare_lint_pass!(FactorOutAssignment => [FACTOR_OUT_ASSIGNMENT]);

// If the block's final statement is a plain assignment (with no trailing tail expression),
// returns the assignment's target.
fn final_assignment_target<'tcx>(block: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Block(block, _) = block.kind else {
        return None;
    };
    if block.expr.is_some() {
        return None;
    }
    let StmtKind::Semi(stmt) = block.stmts.last()?.kind else {
        return None;
    };
    if let ExprKind::Assign(lhs, _, _) = stmt.kind {
        Some(lhs)
    } else {
        None
    }
}

impl<'tcx> LateLintPass<'tcx> for FactorOutAssignment {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // Only consider the head of a chain: if this `if` is itself an else branch of another
        // `if`, it is checked as part of the outer chain.
        if let Some(parent) = clippy_utils::get_parent_expr(cx, expr)
            && let ExprKind::If(_, _, Some(els)) = parent.kind
            && els.hir_id == expr.hir_id
        {
            return;
        }
        let ExprKind::If(_, then, Some(mut els)) = expr.kind else {
            return;
        };
        let Some(target) = final_assignment_target(then) else {
            return;
        };
        // walk the else-if chain; every arm must assign to the same target
        loop {
            match els.kind {
                ExprKind::If(_, arm, Some(next)) => {
                    let Some(t) = final_assignment_target(arm) else {
                        return;
                    };
                    if !eq_expr_value(cx, target, t) {
                        return;
                    }
                    els = next;
                }
                _ => {
                    let Some(t) = final_assignment_target(els) else {
                        return;
                    };
                    if !eq_expr_value(cx, target, t) {
                        return;
                    }
                    break;
                }
            }
        }
        clippy_utils::diagnostics::span_lint(
            cx,
            FACTOR_OUT_ASSIGNMENT,
            expr.span,
            "every branch of this `if` assigns to the same target; assign the conditional \
             expression instead",
        );
    }
}
