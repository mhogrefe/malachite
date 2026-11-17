// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::eq_expr_value;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags multiplying a bignum by itself (`&x * &x`) and raising one to the power of 2
    /// (`x.pow(2)`, `x.pow_assign(2)`). For `Float` and `BigExponentFloat`, the rounding
    /// multiplication functions are flagged too, like `x.mul_prec_ref_ref(&x, prec)`,
    /// `x.mul_prec_round(x.clone(), prec, rm)`, or `x.mul_round_ref_ref(&x, rm)`.
    ///
    /// ### Why is this bad?
    ///
    /// Squaring has a dedicated, faster implementation: use `square()`, `(&x).square()`, or
    /// `square_assign()`, or the corresponding member of the `square_prec*`/`square_round*`
    /// families, like `x.square_prec_ref(prec)`.
    ///
    /// The rounding multiplication functions are not flagged inside a function with `square` in
    /// its name, which may be defining or checking squaring by multiplying.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = &x * &x;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = (&x).square();
    /// ```
    pub USE_SQUARE,
    Deny,
    "multiplying a bignum by itself or raising it to the power of 2 instead of squaring"
}

declare_lint_pass!(UseSquare => [USE_SQUARE]);

impl<'tcx> LateLintPass<'tcx> for UseSquare {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_cross_check_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) if op.node == BinOpKind::Mul => {
                let l = crate::peel_clone_and_borrows(lhs);
                let r = crate::peel_clone_and_borrows(rhs);
                if crate::bignum_name(cx, cx.typeck_results().expr_ty(l).peel_refs()).is_some()
                    && eq_expr_value(cx, l, r)
                {
                    span_lint(
                        cx,
                        USE_SQUARE,
                        expr.span,
                        "use `square()` (or `square_assign()`) instead of multiplying a value \
                        by itself",
                    );
                }
            }
            ExprKind::MethodCall(seg, recv, [arg, _, ..], _) => {
                // `x.mul_prec_ref_ref(&x, prec)` and the rest of the rounding multiplication
                // families, when the type has the corresponding squaring function.
                let name = seg.ident.name.as_str();
                let base = crate::strip_variant_suffixes(name);
                let Some(stem) = base.strip_prefix("mul_") else {
                    return;
                };
                if !matches!(
                    stem,
                    "prec_round"
                        | "prec"
                        | "round"
                        | "prec_round_assign"
                        | "prec_assign"
                        | "round_assign"
                ) {
                    return;
                }
                let l = crate::peel_clone_and_borrows(recv);
                let r = crate::peel_clone_and_borrows(arg);
                let Some(adt_did) = crate::bignum_adt_did(cx, cx.typeck_results().expr_ty(l))
                else {
                    return;
                };
                if !eq_expr_value(cx, l, r) {
                    return;
                }
                // The by-reference squaring functions end in a single `_ref`. A receiver taken by
                // reference is spelled `_ref_val` or `_ref_ref` in the multiplication's name; a
                // receiver cloned for the call is better squared by reference, too.
                let cloned = matches!(
                    recv.kind,
                    ExprKind::MethodCall(seg, _, [], _) if seg.ident.name.as_str() == "clone"
                );
                let by_ref =
                    !stem.ends_with("assign") && (cloned || name[base.len()..].starts_with("_ref"));
                let square = format!("square_{stem}{}", if by_ref { "_ref" } else { "" });
                if !crate::has_inherent_fn(cx, adt_did, &square)
                    || crate::in_fn_named_for(cx, expr, "square")
                {
                    return;
                }
                span_lint(
                    cx,
                    USE_SQUARE,
                    expr.span,
                    format!("use `{square}` instead of multiplying a value by itself"),
                );
            }
            ExprKind::MethodCall(seg, recv, [arg], _) => {
                let name = seg.ident.name.as_str();
                if !matches!(name, "pow" | "pow_assign") {
                    return;
                }
                if crate::literal_value(arg) != Some(2) {
                    return;
                }
                if crate::bignum_name(cx, cx.typeck_results().expr_ty(recv).peel_refs()).is_none() {
                    return;
                }
                let advice = if name == "pow" {
                    "use `square()` instead of `pow(2)`"
                } else {
                    "use `square_assign()` instead of `pow_assign(2)`"
                };
                span_lint(cx, USE_SQUARE, expr.span, advice);
            }
            _ => {}
        }
    }
}
