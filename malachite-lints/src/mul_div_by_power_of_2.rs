// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::source::snippet;
use rustc_ast::{LitIntType, LitKind};
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags multiplying or dividing a bignum (`Natural`, `Integer`, `Rational`, `Float`,
    /// `GaussianInteger`, or `GaussianRational`) by `power_of_2(..)`, including the `*=` and `/=`
    /// forms, and `div_round`/`div_round_assign` of a `Natural` or `Integer` by `power_of_2(..)`,
    /// where `shr_round`/`shr_round_assign` with the same rounding mode says the same thing.
    ///
    /// ### Why is this bad?
    ///
    /// Shifting is more direct and cheaper: `x << k` instead of `x * T::power_of_2(k)`, and `x >>
    /// k` instead of `x / T::power_of_2(k)`. Note that malachite's signed shifts accept negative
    /// counts, which reverse the direction, so a signed `power_of_2` argument needs no special
    /// treatment. One case needs care: `Integer` division truncates while `>>` takes the floor, so
    /// dividing an `Integer` converts to `shr_round` with `Down` (or `>>` if the floor is really
    /// what's wanted).
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = x * Rational::power_of_2(k);
    /// let z = n.div_round(Natural::power_of_2(k), Ceiling).0;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = x << k;
    /// let z = n.shr_round(k, Ceiling).0;
    /// ```
    pub MUL_DIV_BY_POWER_OF_2,
    Deny,
    "multiplying, dividing, or `div_round`ing a bignum by `power_of_2` instead of shifting"
}

declare_lint_pass!(MulDivByPowerOf2 => [MUL_DIV_BY_POWER_OF_2]);

impl<'tcx> LateLintPass<'tcx> for MulDivByPowerOf2 {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        // Tests, demos, and test utilities multiply by `power_of_2` on purpose, to cross-check the
        // shift operators themselves.
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        // `x.div_round(T::power_of_2(k), rm)` and the assign form: `shr_round` and
        // `shr_round_assign` take the same rounding mode, so the rewrite is direct. Only `Natural`
        // and `Integer` have `shr_round`.
        if let ExprKind::MethodCall(seg, _, [divisor, rm], _) = expr.kind {
            let method = seg.ident.name.as_str();
            let advice = match method {
                "div_round" => "shr_round",
                "div_round_assign" => "shr_round_assign",
                _ => return,
            };
            let Some(name) = crate::power_of_2_call(cx, divisor) else {
                return;
            };
            if name != "Natural" && name != "Integer" {
                return;
            }
            let ExprKind::Call(_, [k]) = divisor.peel_borrows().kind else {
                return;
            };
            // An unsuffixed count literal in `power_of_2` is a `u64` by its signature; as a shift
            // count it follows the `u32` literal convention (`bignum_literal_suffix`).
            let k = match k.kind {
                ExprKind::Lit(lit)
                    if matches!(lit.node, LitKind::Int(_, LitIntType::Unsuffixed)) =>
                {
                    format!("{}u32", snippet(cx, k.span, ".."))
                }
                _ => snippet(cx, k.span, "..").to_string(),
            };
            let rm = snippet(cx, rm.span, "..");
            span_lint(
                cx,
                MUL_DIV_BY_POWER_OF_2,
                expr.span,
                format!(
                    "use `{advice}({k}, {rm})` instead of `{method}({name}::power_of_2({k}), {rm})`"
                ),
            );
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
                // `GaussianInteger` division rounds to the nearest Gaussian integer and there is
                // no right shift, so there is nothing cheaper to suggest.
                (false, _, "GaussianInteger") => return,
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
