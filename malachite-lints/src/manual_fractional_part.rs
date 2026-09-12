// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use clippy_utils::{eq_expr_value, expr_or_init};
use rustc_hir::{AssignOpKind, BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags subtracting a [`Rational`]'s own integer part from it, as in `x -
    /// Rational::from(Integer::rounding_from(&x, Down).0)`, where a single remainder operation says
    /// the same thing: `x % Rational::ONE` for a part truncated toward zero (`Down`), and
    /// `x.mod_op(Rational::ONE)` for one truncated toward negative infinity (`Floor`).
    ///
    /// ### Why is this bad?
    ///
    /// `Rem` for `Rational` is defined as $x - y \operatorname{sgn}(xy) \lfloor |x/y| \rfloor$, so
    /// `x % 1` *is* the fractional part with the sign of `x`, and `mod_op` is the same with the
    /// sign of the divisor. The manual form spells that out as a rounding, a conversion back, and a
    /// subtraction, hiding the intent behind three type conversions and allocating an `Integer`
    /// that the remainder never needs.
    ///
    /// Only `Down` and `Floor` are flagged. Subtracting the *nearest* integer is a different
    /// operation, the symmetric remainder in $[-1/2, 1/2]$, which has no such shorthand.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let whole = Rational::from(Integer::rounding_from(&q, Down).0);
    /// let q = q - whole;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let q = q % Rational::ONE;
    /// ```
    pub MANUAL_FRACTIONAL_PART,
    Deny,
    "subtracting a `Rational`'s integer part from it instead of taking a remainder by one"
}

declare_lint_pass!(ManualFractionalPart => [MANUAL_FRACTIONAL_PART]);

// If `e` is the integer part of some `Rational`, converted back to a `Rational` — that is,
// `Rational::from(Integer::rounding_from(<x>, <rm>).0)`, with any borrow or `clone` around the
// parts — returns `<x>` and the name of `<rm>`. Immutable locals are followed to their
// initializers, so the usual two-statement form (a `let` for the integer part, then the
// subtraction) is matched too.
fn integer_part<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
) -> Option<(&'tcx Expr<'tcx>, &'tcx str)> {
    let e = crate::peel_clone_and_borrows(expr_or_init(cx, e));
    // the conversion back to a `Rational`
    let ExprKind::Call(callee, [inner]) = e.kind else {
        return None;
    };
    let ExprKind::Path(ref qpath) = callee.kind else {
        return None;
    };
    if !matches!(
        crate::qpath_last_segment_name(qpath),
        Some("from" | "exact_from")
    ) {
        return None;
    }
    // the ternary `.0` of the rounding
    let ExprKind::Field(rounded, field) =
        crate::peel_clone_and_borrows(expr_or_init(cx, inner)).kind
    else {
        return None;
    };
    if field.name.as_str() != "0" {
        return None;
    }
    let ExprKind::Call(callee, [x, rm]) = crate::peel_clone_and_borrows(rounded).kind else {
        return None;
    };
    let ExprKind::Path(ref qpath) = callee.kind else {
        return None;
    };
    if crate::qpath_last_segment_name(qpath) != Some("rounding_from") {
        return None;
    }
    let ExprKind::Path(ref rm_qpath) = rm.kind else {
        return None;
    };
    Some((
        crate::peel_clone_and_borrows(x),
        crate::qpath_last_segment_name(rm_qpath)?,
    ))
}

// The replacement for a subtraction of the integer part rounded with `rm`, as a method call on the
// receiver: `Down` truncates toward zero, which is what `%` does, and `Floor` toward negative
// infinity, which is what `mod_op` does.
fn replacement(rm: &str, assign: bool) -> Option<&'static str> {
    match (rm, assign) {
        ("Down", false) => Some("% Rational::ONE"),
        ("Down", true) => Some("%= Rational::ONE"),
        ("Floor", false) => Some(".mod_op(Rational::ONE)"),
        ("Floor", true) => Some(".mod_assign(Rational::ONE)"),
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for ManualFractionalPart {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let (lhs, rhs, assign) = match expr.kind {
            ExprKind::Binary(op, lhs, rhs) if op.node == BinOpKind::Sub => (lhs, rhs, false),
            ExprKind::AssignOp(op, lhs, rhs) if op.node == AssignOpKind::SubAssign => {
                (lhs, rhs, true)
            }
            _ => return,
        };
        // the receiver is named in the suggestion as written, but compared with the rounded
        // expression once both are peeled
        let peeled = crate::peel_clone_and_borrows(lhs);
        if crate::bignum_name(cx, cx.typeck_results().expr_ty(peeled).peel_refs())
            != Some("Rational")
        {
            return;
        }
        let Some((x, rm)) = integer_part(cx, rhs) else {
            return;
        };
        if !eq_expr_value(cx, peeled, x) {
            return;
        }
        let Some(replacement) = replacement(rm, assign) else {
            return;
        };
        // `&q` and other non-atomic receivers need parentheses, both before a method call and
        // before an operator that binds more tightly than they do
        let receiver = snippet(cx, lhs.span, "..");
        let receiver = if matches!(
            lhs.kind,
            ExprKind::Path(_) | ExprKind::MethodCall(..) | ExprKind::Call(..) | ExprKind::Field(..)
        ) {
            receiver.to_string()
        } else {
            format!("({receiver})")
        };
        let separator = if replacement.starts_with('.') {
            ""
        } else {
            " "
        };
        span_lint_and_help(
            cx,
            MANUAL_FRACTIONAL_PART,
            expr.span,
            "this subtracts a `Rational`'s integer part from it",
            None,
            format!("use `{receiver}{separator}{replacement}`"),
        );
    }
}
