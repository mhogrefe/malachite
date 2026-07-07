// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::eq_expr_value;
use clippy_utils::source::snippet;
use rustc_errors::Applicability;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags adding the significant bits of a [`Rational`]'s numerator and denominator, like
    /// `x.numerator_ref().significant_bits() + x.denominator_ref().significant_bits()`.
    ///
    /// ### Why is this bad?
    ///
    /// This is exactly what `Rational::significant_bits` returns (in constant time, and without
    /// accessing the numerator and denominator separately). Write `x.significant_bits()`.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let bits = x.numerator_ref().significant_bits() + x.denominator_ref().significant_bits();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let bits = x.significant_bits();
    /// ```
    pub MANUAL_RATIONAL_SIGNIFICANT_BITS,
    Deny,
    "summing the significant bits of a `Rational`'s numerator and denominator instead of using \
    `Rational::significant_bits`"
}

declare_lint_pass!(ManualRationalSignificantBits => [MANUAL_RATIONAL_SIGNIFICANT_BITS]);

// If `e` is `<recv>.significant_bits()` with `<recv>` in turn a call to one of `methods` on a
// `Rational`, returns that `Rational` receiver.
fn part_of<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
    methods: &[&str],
) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::MethodCall(sig_seg, sig_recv, [], _) = e.kind else {
        return None;
    };
    if sig_seg.ident.name.as_str() != "significant_bits" {
        return None;
    }
    let ExprKind::MethodCall(part_seg, x, [], _) = sig_recv.kind else {
        return None;
    };
    if !methods.contains(&part_seg.ident.name.as_str()) {
        return None;
    }
    (crate::bignum_name(cx, cx.typeck_results().expr_ty(x).peel_refs()) == Some("Rational"))
        .then_some(x)
}

impl<'tcx> LateLintPass<'tcx> for ManualRationalSignificantBits {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
            return;
        };
        if op.node != BinOpKind::Add {
            return;
        }
        const NUM: &[&str] = &["numerator_ref", "to_numerator"];
        const DEN: &[&str] = &["denominator_ref", "to_denominator"];
        // The two `significant_bits()` calls, in either order, must be the numerator and the
        // denominator of the same `Rational`.
        let same = |a, b| {
            part_of(cx, a, NUM)
                .zip(part_of(cx, b, DEN))
                .filter(|(xa, xb)| eq_expr_value(cx, xa, xb))
                .map(|(xa, _)| xa)
        };
        let Some(x) = same(lhs, rhs).or_else(|| same(rhs, lhs)) else {
            return;
        };
        span_lint_and_sugg(
            cx,
            MANUAL_RATIONAL_SIGNIFICANT_BITS,
            expr.span,
            "this is `Rational::significant_bits`",
            "use it directly",
            format!("{}.significant_bits()", snippet(cx, x.span, "..")),
            Applicability::MachineApplicable,
        );
    }
}
