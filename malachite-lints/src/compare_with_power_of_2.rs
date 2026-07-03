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
    /// Flags comparing a bignum (`Natural`, `Integer`, `Rational`, or `Float`) with
    /// `power_of_2(..)`, whether via the comparison operators or via `cmp`, `partial_cmp`, or the
    /// `*_abs` comparison methods.
    ///
    /// ### Why is this bad?
    ///
    /// Materializing a power of 2 just to compare against it is wasteful (for large powers it
    /// allocates a huge number); comparing the value's exponent with the power is direct and
    /// cheap. `Natural` has `floor_log_base_2`, `ceiling_log_base_2`, and `checked_log_base_2`;
    /// an `Integer` can use them through `unsigned_abs_ref()`; `Rational` additionally has the
    /// `_abs` variants; and for a `Float`, `get_exponent()` gives 1 more than the floor of the
    /// log (|x| lies in [2^(e-1), 2^e)).
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if x.lt_abs(&Rational::power_of_2(pow)) { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if x.floor_log_base_2_abs() < pow { .. }
    /// ```
    pub COMPARE_WITH_POWER_OF_2,
    Deny,
    "comparing a bignum with `power_of_2` instead of comparing exponents"
}

declare_lint_pass!(CompareWithPowerOf2 => [COMPARE_WITH_POWER_OF_2]);

const CMP_METHODS: [&str; 10] = [
    "cmp",
    "partial_cmp",
    "cmp_abs",
    "partial_cmp_abs",
    "eq_abs",
    "ne_abs",
    "lt_abs",
    "gt_abs",
    "le_abs",
    "ge_abs",
];

impl<'tcx> LateLintPass<'tcx> for CompareWithPowerOf2 {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        // Tests, demos, and test utilities compare against `power_of_2` on purpose, to cross-check
        // the log functions themselves.
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        // A comparison of `a` and `b`: an operator, or one of the comparison methods.
        let (a, b) = match expr.kind {
            ExprKind::Binary(op, lhs, rhs) => match op.node {
                BinOpKind::Eq
                | BinOpKind::Ne
                | BinOpKind::Lt
                | BinOpKind::Le
                | BinOpKind::Gt
                | BinOpKind::Ge => (lhs, rhs),
                _ => return,
            },
            ExprKind::MethodCall(seg, receiver, [arg], _) => {
                if !CMP_METHODS.contains(&seg.ident.name.as_str()) {
                    return;
                }
                (receiver, arg)
            }
            _ => return,
        };
        // One side is `T::power_of_2(..)`; the advice is keyed on the other side's type, whose
        // exponent would be compared instead. Both sides must be bignums (the comparison-method
        // names alone are not distinctive enough, and a primitive side would want
        // `significant_bits` instead).
        for (pow, other) in [(a, b), (b, a)] {
            let Some(pow_name) = crate::power_of_2_call(cx, pow) else {
                continue;
            };
            let other_ty = cx.typeck_results().expr_ty(other).peel_refs();
            let Some(other_name) = crate::bignum_name(cx, other_ty) else {
                continue;
            };
            let advice = match other_name {
                "Natural" => {
                    "compare `floor_log_base_2()`/`ceiling_log_base_2()`/`checked_log_base_2()` \
                    with the power"
                }
                "Integer" => {
                    "compare `floor_log_base_2()`/`ceiling_log_base_2()`/`checked_log_base_2()` \
                    of `unsigned_abs_ref()` with the power"
                }
                "Rational" => {
                    "compare `floor_log_base_2()`/`ceiling_log_base_2()` (or their `_abs` \
                    variants) with the power"
                }
                _ => "compare `get_exponent()` with the power",
            };
            span_lint(
                cx,
                COMPARE_WITH_POWER_OF_2,
                expr.span,
                format!("{advice} instead of comparing with `{pow_name}::power_of_2(..)`"),
            );
            return;
        }
    }
}
