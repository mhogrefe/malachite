// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a shifted operand inside a multiplication or (for `Rational` and `GaussianRational`) a division when the
    /// shift can be hoisted out of the operation: `(a << s) * b` computes the same value as
    /// `(a * b) << s`.
    ///
    /// ### Why is this bad?
    ///
    /// A left shift makes a bignum `s` bits longer, so `(a << s) * b` multiplies a longer number
    /// than `(a * b) << s` does. Hoisting the shift performs the same multiplication on smaller
    /// operands and shifts once at the end. For `Rational`s, an inner shift also drags a power
    /// of 2 through the reduction to lowest terms before the operation throws it away or
    /// restores it.
    ///
    /// ### Known problems
    ///
    /// Only exact rewrites are suggested. `Natural` and `Integer` right shifts are floor
    /// divisions, so `(a >> s) * b` is not `(a * b) >> s`, and `/` on those types truncates, so
    /// no shift commutes with it; only `<<` inside `*` is flagged for them. For `Rational`, both
    /// shift directions commute with both `*` and `/` in either operand, and all combinations
    /// are flagged, with the direction reversed when the shifted operand is a divisor. `Float`
    /// is excluded entirely: its shifts saturate at the exponent-range boundaries, so
    /// `(a << s) * b` can overflow to infinity where `(a * b) << s` is finite, and there is
    /// nothing to gain, a `Float` multiplication's cost not depending on the exponents.
    /// Primitive integers are excluded because the rewrite moves the point at which an overflow
    /// occurs. The compound-assignment forms (`x *= &a << s`) are not flagged, since the rewrite
    /// would need two statements.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let x = (a << 5u64) * b;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let x = (a * b) << 5u64;
    /// ```
    pub HOIST_SHIFTS,
    Deny,
    "shifting an operand of a multiplication or division instead of shifting the result"
}

declare_lint_pass!(HoistShifts => [HOIST_SHIFTS]);

// If `e` is a shift, returns true for `<<` and false for `>>`.
fn shift_direction<'tcx>(e: &'tcx Expr<'tcx>) -> Option<bool> {
    match crate::peel_clone_and_borrows(e).kind {
        ExprKind::Binary(op, _, _) => match op.node {
            BinOpKind::Shl => Some(true),
            BinOpKind::Shr => Some(false),
            _ => None,
        },
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for HoistShifts {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
            return;
        };
        let mul = match op.node {
            BinOpKind::Mul => true,
            BinOpKind::Div => false,
            _ => return,
        };
        let Some(name) = crate::bignum_name(cx, cx.typeck_results().expr_ty(expr).peel_refs())
        else {
            return;
        };
        let rational = match name {
            "Rational" | "GaussianRational" => true,
            // Float shifts saturate at the exponent-range boundaries, so hoisting is not
            // value-preserving there; see the lint documentation.
            "Natural" | "Integer" | "GaussianInteger" => false,
            _ => return,
        };
        for (operand, operand_is_lhs) in [(lhs, true), (rhs, false)] {
            let Some(left_shift) = shift_direction(operand) else {
                continue;
            };
            // For Natural and Integer, only a left shift inside a multiplication is exact: right
            // shifts are floor divisions, and `/` truncates.
            if !rational && !(mul && left_shift) {
                continue;
            }
            // A shifted divisor hoists with the direction reversed: a / (b << s) = (a / b) >> s.
            let hoisted_left = if mul || operand_is_lhs {
                left_shift
            } else {
                !left_shift
            };
            let op_sym = if mul { "*" } else { "/" };
            let shift_in = if left_shift { "<<" } else { ">>" };
            let shift_out = if hoisted_left { "<<" } else { ">>" };
            let pattern = if operand_is_lhs {
                format!("(a {shift_in} s) {op_sym} b")
            } else {
                format!("a {op_sym} (b {shift_in} s)")
            };
            span_lint(
                cx,
                HOIST_SHIFTS,
                expr.span,
                format!(
                    "hoist the shift out of the {}: use `(a {op_sym} b) {shift_out} s` instead \
                     of `{pattern}`",
                    if mul { "multiplication" } else { "division" },
                ),
            );
            return;
        }
    }
}
