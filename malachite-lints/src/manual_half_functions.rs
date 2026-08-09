// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::consts::{ConstEvalCtxt, Constant};
use clippy_utils::diagnostics::span_lint;
use rustc_hir::{BinOpKind, Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags manual implementations of the half-conversion functions on concrete double-width
    /// unsigned types: assembling a value from two halves with a shift and a bitwise or
    /// (`(D::from(hi) << W) | D::from(lo)`), and extracting the upper half with a shift and a
    /// narrowing conversion (`H::wrapping_from(x >> W)`).
    ///
    /// ### Why is this bad?
    ///
    /// `join_halves(hi, lo)` and `x.upper_half()` say the same thing directly, and cannot get
    /// the shift amount wrong.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let d = (DoubleLimb::from(a1) << Limb::WIDTH) | DoubleLimb::from(a0);
    /// let hi = Limb::wrapping_from(p >> Limb::WIDTH);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let d = DoubleLimb::join_halves(a1, a0);
    /// let hi = p.upper_half();
    /// ```
    pub MANUAL_HALF_FUNCTIONS,
    Deny,
    "manually implementing `join_halves` or `upper_half` with shifts and conversions"
}

declare_lint_pass!(ManualHalfFunctions => [MANUAL_HALF_FUNCTIONS]);

// The bit width of `ty` if it is a concrete unsigned integer type other than `usize`.
fn unsigned_bits(ty: Ty<'_>) -> Option<u128> {
    match ty.kind() {
        rustc_middle::ty::Uint(u) => u.bit_width().map(u128::from),
        _ => None,
    }
}

// Whether `e` is a compile-time constant equal to `bits`.
fn is_const_shift(cx: &LateContext<'_>, e: &Expr<'_>, bits: u128) -> bool {
    matches!(
        ConstEvalCtxt::new(cx).eval(e),
        Some(Constant::Int(v)) if v == bits
    )
}

// Whether `e` is a conversion of a half-width value to the double-width type `d_bits` wide: a call
// like `D::from(h)` where `h` is unsigned and exactly half as wide.
fn is_conversion_from_half(cx: &LateContext<'_>, e: &Expr<'_>, d_bits: u128) -> bool {
    let ExprKind::Call(callee, [arg]) = e.kind else {
        return false;
    };
    let ExprKind::Path(qpath) = &callee.kind else {
        return false;
    };
    if crate::qpath_last_segment_name(qpath) != Some("from") {
        return false;
    }
    unsigned_bits(cx.typeck_results().expr_ty(arg)) == Some(d_bits >> 1)
}

const JOIN_HALVES: &str = "malachite_base::num::conversion::traits::JoinHalves";
const SPLIT_IN_HALF: &str = "malachite_base::num::conversion::traits::SplitInHalf";

impl<'tcx> LateLintPass<'tcx> for ManualHalfFunctions {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            // (D::from(hi) << half) | D::from(lo), in either operand order and also with `+`
            ExprKind::Binary(op, lhs, rhs)
                if matches!(op.node, BinOpKind::BitOr | BinOpKind::Add) =>
            {
                let d_ty = cx.typeck_results().expr_ty(expr);
                let Some(d_bits) = unsigned_bits(d_ty) else {
                    return;
                };
                if !crate::implements_trait_by_path(cx, d_ty, JOIN_HALVES) {
                    return;
                }
                for (shift_side, low_side) in [(lhs, rhs), (rhs, lhs)] {
                    if let ExprKind::Binary(shift_op, base, amount) = shift_side.kind
                        && shift_op.node == BinOpKind::Shl
                        && is_const_shift(cx, amount, d_bits >> 1)
                        && is_conversion_from_half(cx, base, d_bits)
                        && is_conversion_from_half(cx, low_side, d_bits)
                    {
                        span_lint(
                            cx,
                            MANUAL_HALF_FUNCTIONS,
                            expr.span,
                            "use `join_halves()` instead of assembling the halves with a shift",
                        );
                        return;
                    }
                }
            }
            // H::wrapping_from(x >> half) or H::exact_from(x >> half)
            ExprKind::Call(callee, [arg]) => {
                let ExprKind::Path(qpath) = &callee.kind else {
                    return;
                };
                if !matches!(
                    crate::qpath_last_segment_name(qpath),
                    Some("wrapping_from" | "exact_from")
                ) {
                    return;
                }
                let Some(h_bits) = unsigned_bits(cx.typeck_results().expr_ty(expr)) else {
                    return;
                };
                let ExprKind::Binary(shift_op, x, amount) = arg.kind else {
                    return;
                };
                if shift_op.node != BinOpKind::Shr {
                    return;
                }
                let x_ty = cx.typeck_results().expr_ty(x);
                if unsigned_bits(x_ty) != Some(h_bits << 1) {
                    return;
                }
                if !is_const_shift(cx, amount, h_bits) {
                    return;
                }
                if !crate::implements_trait_by_path(cx, x_ty, SPLIT_IN_HALF) {
                    return;
                }
                span_lint(
                    cx,
                    MANUAL_HALF_FUNCTIONS,
                    expr.span,
                    "use `upper_half()` instead of shifting and converting down",
                );
            }
            _ => {}
        }
    }
}
