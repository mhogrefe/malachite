// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use clippy_utils::ty::implements_trait;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{AssignOpKind, BinOpKind, Expr, ExprKind, LangItem};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a shift whose amount is converted to another integer type first, as in
    /// `x << i64::exact_from(n)`, when the shift is already implemented for the type of `n`.
    ///
    /// ### Why is this bad?
    ///
    /// Shifting is implemented for every primitive integer on the right-hand side, so the
    /// conversion buys nothing and only obscures the shift amount.
    ///
    /// The conversion is *not* redundant when it is load-bearing, and those cases are left alone:
    /// converting a signed amount to an unsigned type asserts that the amount is non-negative, and
    /// dropping it would silently reverse the direction of the shift, since Malachite's shifts by a
    /// signed amount shift the other way when the amount is negative. Only conversions that cannot
    /// change the shift's direction are flagged; among those, the conversion still panics on an
    /// out-of-range amount, so keep it where that panic is the intent.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let root = root << i64::exact_from(e / k);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let root = root << (e / k);
    /// ```
    pub REDUNDANT_SHIFT_CONVERSION,
    Deny,
    "converting a shift amount to another integer type that the shift also accepts"
}

declare_lint_pass!(RedundantShiftConversion => [REDUNDANT_SHIFT_CONVERSION]);

const CONVERSIONS: [&str; 4] = ["exact_from", "wrapping_from", "saturating_from", "from"];

// The operand of a `T::exact_from(..)`-style integer conversion, if `e` is one.
fn conversion_operand<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Call(callee, [operand]) = e.kind else {
        return None;
    };
    let ExprKind::Path(qpath) = &callee.kind else {
        return None;
    };
    let Res::Def(DefKind::AssocFn, did) = cx.qpath_res(qpath, callee.hir_id) else {
        return None;
    };
    CONVERSIONS
        .contains(&cx.tcx.item_name(did).as_str())
        .then_some(operand)
}

// The lang item of the trait implementing a shift.
fn shift_lang_item(expr: &Expr<'_>) -> Option<(LangItem, &'static str)> {
    match expr.kind {
        ExprKind::Binary(op, ..) => match op.node {
            BinOpKind::Shl => Some((LangItem::Shl, "<<")),
            BinOpKind::Shr => Some((LangItem::Shr, ">>")),
            _ => None,
        },
        ExprKind::AssignOp(op, ..) => match op.node {
            AssignOpKind::ShlAssign => Some((LangItem::ShlAssign, "<<=")),
            AssignOpKind::ShrAssign => Some((LangItem::ShrAssign, ">>=")),
            _ => None,
        },
        _ => None,
    }
}

// Whether dropping a conversion from `from` to `to` can change which way a shift goes. Malachite
// shifts by a signed amount in the opposite direction when the amount is negative, so a signed
// amount widened to an unsigned type is really an assertion that it is non-negative.
fn changes_direction(from: Ty<'_>, to: Ty<'_>) -> bool {
    from.is_signed() && !to.is_signed()
}

impl<'tcx> LateLintPass<'tcx> for RedundantShiftConversion {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let Some((lang_item, operator)) = shift_lang_item(expr) else {
            return;
        };
        let (ExprKind::Binary(_, lhs, rhs) | ExprKind::AssignOp(_, lhs, rhs)) = expr.kind else {
            return;
        };
        let Some(operand) = conversion_operand(cx, rhs) else {
            return;
        };
        let converted = cx.typeck_results().expr_ty(rhs);
        let original = cx.typeck_results().expr_ty(operand);
        if !converted.is_integral()
            || !original.is_integral()
            || converted == original
            || changes_direction(original, converted)
        {
            return;
        }
        let Some(trait_did) = cx.tcx.lang_items().get(lang_item) else {
            return;
        };
        // The shift must already accept the unconverted amount.
        if !implements_trait(
            cx,
            cx.typeck_results().expr_ty(lhs),
            trait_did,
            &[original.into()],
        ) {
            return;
        }
        span_lint_and_help(
            cx,
            REDUNDANT_SHIFT_CONVERSION,
            rhs.span,
            "converting a shift amount to a type the shift already accepts",
            None,
            format!(
                "shift by the amount itself: `{operator} {}`",
                snippet(cx, operand.span, ".."),
            ),
        );
    }
}
