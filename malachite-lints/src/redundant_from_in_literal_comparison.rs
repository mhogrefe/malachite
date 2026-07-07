// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::snippet;
use rustc_errors::Applicability;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags widening a primitive integer with `from` only to compare the result with an integer
    /// literal, like `i64::from(x) <= 32`.
    ///
    /// ### Why is this bad?
    ///
    /// `from` is an exact, value-preserving conversion, so as long as the literal is representable
    /// in the source type the comparison is unchanged by dropping the conversion: `x <= 32` means
    /// the same thing (and the literal takes the source type). This applies to every comparison
    /// operator.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if i64::from(x) <= 32 { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if x <= 32 { .. }
    /// ```
    pub REDUNDANT_FROM_IN_LITERAL_COMPARISON,
    Deny,
    "widening a primitive integer with `from` only to compare it with a literal"
}

declare_lint_pass!(RedundantFromInLiteralComparison => [REDUNDANT_FROM_IN_LITERAL_COMPARISON]);

// The inclusive value range of a fixed-width primitive integer type, as `i128`. `u128`, `usize`,
// and `isize` return `None`: `u128`'s range does not fit in `i128`, and the pointer-width types are
// target-dependent.
fn int_range(ty: Ty<'_>) -> Option<(i128, i128)> {
    match ty.kind() {
        TyKind::Int(IntTy::I8) => Some((i128::from(i8::MIN), i128::from(i8::MAX))),
        TyKind::Int(IntTy::I16) => Some((i128::from(i16::MIN), i128::from(i16::MAX))),
        TyKind::Int(IntTy::I32) => Some((i128::from(i32::MIN), i128::from(i32::MAX))),
        TyKind::Int(IntTy::I64) => Some((i128::from(i64::MIN), i128::from(i64::MAX))),
        TyKind::Int(IntTy::I128) => Some((i128::MIN, i128::MAX)),
        TyKind::Uint(UintTy::U8) => Some((0, i128::from(u8::MAX))),
        TyKind::Uint(UintTy::U16) => Some((0, i128::from(u16::MAX))),
        TyKind::Uint(UintTy::U32) => Some((0, i128::from(u32::MAX))),
        TyKind::Uint(UintTy::U64) => Some((0, i128::from(u64::MAX))),
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for RedundantFromInLiteralComparison {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
            return;
        };
        if !matches!(
            op.node,
            BinOpKind::Eq
                | BinOpKind::Ne
                | BinOpKind::Lt
                | BinOpKind::Le
                | BinOpKind::Gt
                | BinOpKind::Ge
        ) {
            return;
        }
        // Either operand may be the conversion; the other must be an integer literal.
        for (conv, other) in [(lhs, rhs), (rhs, lhs)] {
            let ExprKind::Call(callee, [arg]) = conv.kind else {
                continue;
            };
            let ExprKind::Path(qpath) = &callee.kind else {
                continue;
            };
            let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
                continue;
            };
            // The `from` must be `core::convert::From::from`.
            let Some(trait_did) = cx.tcx.trait_of_assoc(fn_did) else {
                continue;
            };
            let path = cx.get_def_path(trait_did);
            if path.len() != 3
                || path[0].as_str() != "core"
                || path[1].as_str() != "convert"
                || path[2].as_str() != "From"
            {
                continue;
            }
            // The result and the argument are both primitive integers, so the conversion is an
            // exact widening (`From` for integers is always value-preserving).
            if !cx.typeck_results().expr_ty(conv).is_integral() {
                continue;
            }
            let Some((lo, hi)) = int_range(cx.typeck_results().expr_ty(arg)) else {
                continue;
            };
            // Fire only when the literal is representable in the source type, so that comparing the
            // source value against it is well-defined and equivalent.
            let Some(lit) = crate::literal_value(other) else {
                continue;
            };
            if lit < lo || lit > hi {
                continue;
            }
            span_lint_and_sugg(
                cx,
                REDUNDANT_FROM_IN_LITERAL_COMPARISON,
                conv.span,
                "`from` is redundant in a comparison with a representable literal",
                "compare the source value directly",
                snippet(cx, arg.span, "..").to_string(),
                Applicability::MaybeIncorrect,
            );
            return;
        }
    }
}
