// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::source::snippet;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags multiplying or dividing a primitive integer by a power-of-two literal (`x * 8`, `x /
    /// 16`, and the `*=`/`/=` forms), where a shift says the same thing. Also flags multiplying or
    /// dividing by a type's bit width, `x * T::WIDTH` or `x / T::WIDTH`, where the shift amount is
    /// `T::LOG_WIDTH`. The rounding forms are covered too: `x.div_round(2, rm)` and
    /// `x.div_round_assign(2, rm)` become `x.shr_round(1, rm)` and `x.shr_round_assign(1, rm)`,
    /// with the same rounding mode.
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
    /// `shr_round(k, Down)` (or plain `>>` when the floor is really what is wanted). (`div_round`
    /// and `shr_round` round the exact quotient the same way, so that rewrite needs no such
    /// care.) And unlike
    /// `*`, a shift does not detect value overflow (`<<` silently drops the high bits where `*`
    /// would panic in a debug build), so only reach for `<<` where overflow is already ruled out.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = x * 8;
    /// let z = x / 16;
    /// let w = x / Limb::WIDTH;
    /// let v = x.div_round(2, Ceiling).0;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = x << 3;
    /// let z = x >> 4;
    /// let w = x >> Limb::LOG_WIDTH;
    /// let v = x.shr_round(1, Ceiling).0;
    /// ```
    pub MUL_DIV_BY_POWER_OF_2_LITERAL,
    Deny,
    "multiplying, dividing, or `div_round`ing a primitive integer by a power-of-two literal or by `T::WIDTH` instead of shifting"
}

declare_lint_pass!(MulDivByPowerOf2Literal => [MUL_DIV_BY_POWER_OF_2_LITERAL]);

// A recognized power-of-two operand: a literal, or a path to a `WIDTH` associated constant.
enum PowerOf2 {
    // the literal's value and its base-2 exponent (the shift amount)
    Literal(i128, u32),
    Width,
}

// If `e` is a power-of-two integer literal that is at least 2, or a `T::WIDTH` constant (whose
// value is always a power of two, with `T::LOG_WIDTH` as the shift amount), classifies it. A
// literal 1 is excluded: shifting by 0 is no clearer than the identity it already is.
fn power_of_2_operand(cx: &LateContext<'_>, e: &Expr<'_>) -> Option<PowerOf2> {
    if let Some(v) = crate::literal_value(e) {
        (v >= 2 && v & (v - 1) == 0).then(|| PowerOf2::Literal(v, v.trailing_zeros()))
    } else {
        crate::is_width_const(cx, e).then_some(PowerOf2::Width)
    }
}

// The shift amount as it should appear in the advice, and the operand as written.
fn shift_amount(cx: &LateContext<'_>, p: PowerOf2, power: &Expr<'_>) -> (String, String) {
    match p {
        PowerOf2::Literal(v, k) => (k.to_string(), v.to_string()),
        PowerOf2::Width => {
            let w = snippet(cx, power.span, "..").to_string();
            (w.replace("WIDTH", "LOG_WIDTH"), w)
        }
    }
}

impl<'tcx> LateLintPass<'tcx> for MulDivByPowerOf2Literal {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // `x.div_round(pow2, rm)` and `x.div_round_assign(pow2, rm)`: `shr_round` and
        // `shr_round_assign` take the same rounding mode and round the exact quotient the same way,
        // so the rewrite is direct for signed and unsigned integers alike.
        if let ExprKind::MethodCall(seg, receiver, [divisor, rm], _) = expr.kind {
            let name = seg.ident.name.as_str();
            let method = match name {
                "div_round" => "shr_round",
                "div_round_assign" => "shr_round_assign",
                _ => return,
            };
            let Some(p) = power_of_2_operand(cx, divisor) else {
                return;
            };
            let value_ty = cx.typeck_results().expr_ty_adjusted(receiver).peel_refs();
            if !value_ty.is_integral() || crate::literal_value(receiver).is_some() {
                return;
            }
            let (k, described) = shift_amount(cx, p, divisor);
            let rm = snippet(cx, rm.span, "..");
            span_lint(
                cx,
                MUL_DIV_BY_POWER_OF_2_LITERAL,
                expr.span,
                format!("use `{method}({k}, {rm})` instead of `{name}({described}, {rm})`"),
            );
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
            let Some(p) = power_of_2_operand(cx, power) else {
                continue;
            };
            // Only primitive integers; bignums go through `mul_div_by_power_of_2`. And a literal
            // times/over a literal or a `WIDTH` is a compile-time constant, not a runtime shift.
            let value_ty = cx.typeck_results().expr_ty(value);
            if !value_ty.is_integral() || crate::literal_value(value).is_some() {
                continue;
            }
            let (k, described) = shift_amount(cx, p, power);
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
                format!("{advice} instead of {verb} by `{described}`"),
            );
            return;
        }
    }
}
