// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::snippet;
use clippy_utils::ty::implements_trait;
use rustc_errors::Applicability;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags comparisons between a bignum type (`Natural`, `Integer`, `Rational`, or `Float`) and
    /// a value converted from a primitive with `from`, such as `x >= Integer::from(prec)`.
    ///
    /// ### Why is this bad?
    ///
    /// The bignum types implement `PartialEq` and `PartialOrd` directly against the primitive
    /// types, so the conversion (which may allocate) is unnecessary: `x >= prec` means the same
    /// thing.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if Integer::from(prec) >= bits_needed { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if bits_needed <= prec { .. }
    /// ```
    pub REDUNDANT_FROM_IN_COMPARISON,
    Deny,
    "comparing a bignum with `from(primitive)` instead of comparing with the primitive directly"
}

declare_lint_pass!(RedundantFromInComparison => [REDUNDANT_FROM_IN_COMPARISON]);

impl<'tcx> LateLintPass<'tcx> for RedundantFromInComparison {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        // Comparison tests (and demos and test utilities) intentionally compare converted values:
        // `Integer::from(x) == Integer::from(y)` there is testing `Integer`'s own comparison, not
        // comparing against a primitive.
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
            return;
        };
        let is_eq = matches!(op.node, BinOpKind::Eq | BinOpKind::Ne);
        if !is_eq
            && !matches!(
                op.node,
                BinOpKind::Lt | BinOpKind::Le | BinOpKind::Gt | BinOpKind::Ge
            )
        {
            return;
        }
        for (conv, other, conv_is_lhs) in [(lhs, rhs, true), (rhs, lhs, false)] {
            let ExprKind::Call(callee, [arg]) = conv.kind else {
                continue;
            };
            let ExprKind::Path(qpath) = &callee.kind else {
                continue;
            };
            let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
                continue;
            };
            if cx.tcx.item_name(fn_did).as_str() != "from" {
                continue;
            }
            let conv_ty = cx.typeck_results().expr_ty(conv);
            let Some(name) = crate::bignum_name(cx, conv_ty) else {
                continue;
            };
            let arg_ty = cx.typeck_results().expr_ty(arg);
            if !arg_ty.is_integral() && !arg_ty.is_floating_point() {
                continue;
            }
            if cx.typeck_results().expr_ty(other) != conv_ty {
                continue;
            }
            // Only fire when dropping the conversion still compiles: the appropriate
            // `PartialEq`/`PartialOrd` impl between the primitive and the bignum (in the operands'
            // order) must exist.
            let trait_did = if is_eq {
                cx.tcx.lang_items().eq_trait()
            } else {
                cx.tcx.lang_items().partial_ord_trait()
            };
            let Some(trait_did) = trait_did else {
                continue;
            };
            let (l_ty, r_ty) = if conv_is_lhs {
                (arg_ty, conv_ty)
            } else {
                (conv_ty, arg_ty)
            };
            if !implements_trait(cx, l_ty, trait_did, &[r_ty.into()]) {
                continue;
            }
            span_lint_and_sugg(
                cx,
                REDUNDANT_FROM_IN_COMPARISON,
                conv.span,
                format!("`{name}::from` is redundant in a comparison with `{name}`"),
                "compare with the primitive directly",
                snippet(cx, arg.span, "..").to_string(),
                Applicability::MaybeIncorrect,
            );
        }
    }
}
