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
    /// Flags multiplying or dividing a primitive integer by a power-of-two literal (`x * 8`,
    /// `x / 16`, and the `*=`/`/=` forms), where a shift says the same thing.
    ///
    /// This is the primitive-integer companion of `mul_div_by_power_of_2`, which covers the bignum
    /// `x * T::power_of_2(k)` spelling.
    ///
    /// ### Why is this bad?
    ///
    /// Shifting names the operation directly: `x << 3` rather than `x * 8`. There is no measurable
    /// speed difference for primitives (the compiler strength-reduces either way) -- this is a
    /// stylistic preference for the explicit form.
    ///
    /// Two cases need care. Division of a *signed* integer truncates toward zero, whereas `>>`
    /// takes the floor, so the two disagree for negative values; the faithful rewrite is
    /// `shr_round(k, Down)` (or plain `>>` when the floor is really what is wanted). And unlike
    /// `*`, a shift does not detect value overflow (`<<` silently drops the high bits where `*`
    /// would panic in a debug build), so only reach for `<<` where overflow is already ruled out.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = x * 8;
    /// let z = x / 16;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = x << 3;
    /// let z = x >> 4;
    /// ```
    pub MUL_DIV_BY_POWER_OF_2_LITERAL,
    Deny,
    "multiplying or dividing a primitive integer by a power-of-two literal instead of shifting"
}

declare_lint_pass!(MulDivByPowerOf2Literal => [MUL_DIV_BY_POWER_OF_2_LITERAL]);

// If `e` is a power-of-two integer literal that is at least 2, returns its base-2 exponent (the
// shift amount). 1 is excluded: shifting by 0 is no clearer than the identity it already is.
fn power_of_2_literal(e: &Expr<'_>) -> Option<u32> {
    let v = crate::literal_value(e)?;
    (v >= 2 && v & (v - 1) == 0).then(|| v.trailing_zeros())
}

impl<'tcx> LateLintPass<'tcx> for MulDivByPowerOf2Literal {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // `mul` is true for `*`/`*=`, false for `/`/`/=`; `assign` marks the compound forms. Each
        // candidate is `(power_of_two_operand, value_operand)`: for `*` the literal may be either
        // side, for `/` only the divisor (`8 / x` is not a shift of `x`), and for the compound
        // forms the value is the assignee. The value operand fixes the integer type (an `AssignOp`
        // expression itself has type `()`) and is checked against being a literal too.
        let (mul, assign, candidates): (bool, bool, [Option<(&Expr<'_>, &Expr<'_>)>; 2]) =
            match expr.kind {
                ExprKind::Binary(op, lhs, rhs) => match op.node {
                    BinOpKind::Mul => (true, false, [Some((rhs, lhs)), Some((lhs, rhs))]),
                    BinOpKind::Div => (false, false, [Some((rhs, lhs)), None]),
                    _ => return,
                },
                ExprKind::AssignOp(op, lhs, rhs) => match op.node.into() {
                    BinOpKind::Mul => (true, true, [Some((rhs, lhs)), None]),
                    BinOpKind::Div => (false, true, [Some((rhs, lhs)), None]),
                    _ => return,
                },
                _ => return,
            };
        for (power, value) in candidates.into_iter().flatten() {
            let Some(k) = power_of_2_literal(power) else {
                continue;
            };
            // Only primitive integers; bignums go through `mul_div_by_power_of_2`. And a literal
            // times/over a literal is a compile-time constant, not a runtime shift.
            let value_ty = cx.typeck_results().expr_ty(value);
            if !value_ty.is_integral() || crate::literal_value(value).is_some() {
                continue;
            }
            let v = crate::literal_value(power).unwrap();
            let advice = match (mul, assign, value_ty.is_signed()) {
                (true, false, _) => format!("use `<< {k}`"),
                (true, true, _) => format!("use `<<= {k}`"),
                // Signed division truncates toward zero, but `>>` takes the floor; `shr_round` with
                // `Down` preserves the truncating semantics.
                (false, false, true) => {
                    format!("use `shr_round({k}, Down)` (or `>> {k}`, which takes the floor)")
                }
                (false, true, true) => format!(
                    "use `shr_round_assign({k}, Down)` (or `>>= {k}`, which takes the floor)"
                ),
                (false, false, false) => format!("use `>> {k}`"),
                (false, true, false) => format!("use `>>= {k}`"),
            };
            let verb = if mul { "multiplying" } else { "dividing" };
            span_lint(
                cx,
                MUL_DIV_BY_POWER_OF_2_LITERAL,
                expr.span,
                format!("{advice} instead of {verb} by `{v}`"),
            );
            return;
        }
    }
}
