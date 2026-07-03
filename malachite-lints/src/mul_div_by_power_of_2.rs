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
    /// Flags multiplying or dividing a bignum (`Natural`, `Integer`, `Rational`, or `Float`) by
    /// `power_of_2(..)`, including the `*=` and `/=` forms.
    ///
    /// ### Why is this bad?
    ///
    /// Shifting is more direct and cheaper: `x << k` instead of `x * T::power_of_2(k)`, and
    /// `x >> k` instead of `x / T::power_of_2(k)`. Note that malachite's signed shifts accept
    /// negative counts, which reverse the direction, so a signed `power_of_2` argument needs no
    /// special treatment. One case needs care: `Integer` division truncates while `>>` takes the
    /// floor, so dividing an `Integer` converts to `shr_round` with `Down` (or `>>` if the floor
    /// is really what's wanted).
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = x * Rational::power_of_2(k);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = x << k;
    /// ```
    pub MUL_DIV_BY_POWER_OF_2,
    Deny,
    "multiplying or dividing a bignum by `power_of_2` instead of shifting"
}

declare_lint_pass!(MulDivByPowerOf2 => [MUL_DIV_BY_POWER_OF_2]);

impl<'tcx> LateLintPass<'tcx> for MulDivByPowerOf2 {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        // Tests, demos, and test utilities multiply by `power_of_2` on purpose, to cross-check
        // the shift operators themselves.
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        // `mul` is true for `*` and `*=`, false for `/` and `/=`; `assign` distinguishes the
        // compound-assignment forms. For multiplication `power_of_2` may be either operand; for
        // division only the divisor is convertible (`2^k / x` is not a shift of `x`).
        let (mul, assign, operands) = match expr.kind {
            ExprKind::Binary(op, lhs, rhs) => match op.node {
                BinOpKind::Mul => (true, false, [Some(rhs), Some(lhs)]),
                BinOpKind::Div => (false, false, [Some(rhs), None]),
                _ => return,
            },
            ExprKind::AssignOp(op, _, rhs) => match op.node.into() {
                BinOpKind::Mul => (true, true, [Some(rhs), None]),
                BinOpKind::Div => (false, true, [Some(rhs), None]),
                _ => return,
            },
            _ => return,
        };
        for operand in operands.into_iter().flatten() {
            let Some(name) = crate::power_of_2_call(cx, operand) else {
                continue;
            };
            let advice = match (mul, assign, name) {
                (true, false, _) => "use `<<`",
                (true, true, _) => "use `<<=`",
                // `Integer` division truncates, but `>>` takes the floor; `shr_round` with `Down`
                // preserves the semantics.
                (false, false, "Integer") => {
                    "use `shr_round` with `Down` (or `>>`, which takes the floor)"
                }
                (false, true, "Integer") => {
                    "use `shr_round_assign` with `Down` (or `>>=`, which takes the floor)"
                }
                (false, false, _) => "use `>>`",
                (false, true, _) => "use `>>=`",
            };
            let verb = if mul { "multiplying" } else { "dividing" };
            span_lint(
                cx,
                MUL_DIV_BY_POWER_OF_2,
                expr.span,
                format!("{advice} instead of {verb} by `{name}::power_of_2(..)`"),
            );
            return;
        }
    }
}
