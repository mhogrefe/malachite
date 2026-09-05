// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::paths::{PathNS, lookup_path_str};
use clippy_utils::source::snippet;
use clippy_utils::ty::implements_trait;
use rustc_ast::LitKind;
use rustc_hir::def::DefKind;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags comparing the absolute value of a bignum (`Integer`, `Rational`, or `Float`), taken
    /// with `abs()`, against something that cannot be negative: a nonnegative literal, a `Natural`,
    /// an unsigned primitive, or another `abs()`. This covers the comparison operators and the
    /// `eq`/`ne`/`partial_cmp`/`cmp`/`lt`/`le`/`gt`/`ge` methods.
    ///
    /// ### Why is this bad?
    ///
    /// The `EqAbs` and `PartialOrdAbs` traits compare magnitudes directly: `x.le_abs(&y)` reads
    /// the sign bits and compares the magnitudes in place, whereas `(&x).abs() <= y` builds a
    /// whole new bignum (or, by value, consumes `x`) just to throw its sign away. The `*_abs`
    /// spelling also says what is meant. The rewrite is only offered when the other operand is
    /// known to be nonnegative, since the `*_abs` methods compare the magnitudes of *both* sides.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// assert!((&c).abs() <= 1u32);
    /// if (&x).abs() > (&y).abs() {}
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// assert!(c.le_abs(&1u32));
    /// if x.gt_abs(&y) {}
    /// ```
    pub USE_ABS_COMPARISON,
    Deny,
    "comparing a bignum's `abs()` instead of using an `*_abs` comparison"
}

declare_lint_pass!(UseAbsComparison => [USE_ABS_COMPARISON]);

fn peel_borrows<'tcx>(e: &'tcx Expr<'tcx>) -> &'tcx Expr<'tcx> {
    let mut e = e;
    while let ExprKind::AddrOf(_, _, inner) = e.kind {
        e = inner;
    }
    e
}

// If `e` (through `&`) is `recv.abs()` with a bignum receiver, returns the receiver (through `&`).
fn abs_call<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::MethodCall(seg, recv, [], _) = peel_borrows(e).kind else {
        return None;
    };
    if seg.ident.name.as_str() != "abs" {
        return None;
    }
    let recv = peel_borrows(recv);
    crate::bignum_name(cx, cx.typeck_results().expr_ty(recv).peel_refs())
        .is_some()
        .then_some(recv)
}

// If `e` is something known to be nonnegative -- a nonnegative integer literal, an unsigned
// primitive, a `Natural`, or an `abs()` of a bignum -- returns the expression to put on the
// right of the `*_abs` call (for an `abs()`, its receiver).
fn nonnegative<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    if let Some(recv) = abs_call(cx, e) {
        return Some(recv);
    }
    let e = peel_borrows(e);
    if let ExprKind::Lit(lit) = e.kind {
        return matches!(lit.node, LitKind::Int(..)).then_some(e);
    }
    let ty = cx.typeck_results().expr_ty(e).peel_refs();
    ((ty.is_integral() && !ty.is_signed()) || crate::bignum_name(cx, ty) == Some("Natural"))
        .then_some(e)
}

// Whether `ty` implements the `malachite_base` comparison trait `name` with `rhs` on the right.
fn implements<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>, name: &str, rhs: Ty<'tcx>) -> bool {
    let path = format!("malachite_base::num::comparison::traits::{name}");
    lookup_path_str(cx.tcx, PathNS::Type, &path)
        .into_iter()
        .filter(|did| cx.tcx.def_kind(*did) == DefKind::Trait)
        .any(|did| implements_trait(cx, ty, did, &[rhs.into()]))
}

// The `*_abs` method for a comparison, and the trait it needs. `flip` reverses the direction, for
// when the `abs()` is the right operand.
fn abs_method(op: BinOpKind, flip: bool) -> Option<(&'static str, &'static str)> {
    Some(match (op, flip) {
        (BinOpKind::Eq, _) => ("eq_abs", "EqAbs"),
        (BinOpKind::Ne, _) => ("ne_abs", "EqAbs"),
        (BinOpKind::Lt, false) | (BinOpKind::Gt, true) => ("lt_abs", "PartialOrdAbs"),
        (BinOpKind::Le, false) | (BinOpKind::Ge, true) => ("le_abs", "PartialOrdAbs"),
        (BinOpKind::Gt, false) | (BinOpKind::Lt, true) => ("gt_abs", "PartialOrdAbs"),
        (BinOpKind::Ge, false) | (BinOpKind::Le, true) => ("ge_abs", "PartialOrdAbs"),
        _ => return None,
    })
}

fn lint<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
    x: &'tcx Expr<'tcx>,
    y: &'tcx Expr<'tcx>,
    method: &str,
    trait_name: &str,
) {
    let x_ty = cx.typeck_results().expr_ty(x).peel_refs();
    let y_ty = cx.typeck_results().expr_ty(y).peel_refs();
    if trait_name == "OrdAbs" {
        // `cmp_abs` is same-type only
        if x_ty != y_ty
            || !crate::implements_trait_by_path(
                cx,
                x_ty,
                "malachite_base::num::comparison::traits::OrdAbs",
            )
        {
            return;
        }
    } else if !implements(cx, x_ty, trait_name, y_ty) {
        return;
    }
    let x = snippet(cx, x.span, "..");
    let y = snippet(cx, y.span, "..");
    span_lint(
        cx,
        USE_ABS_COMPARISON,
        expr.span,
        format!("use `{x}.{method}(&{y})` instead of comparing an absolute value"),
    );
}

// The comparison modules and their tests cross-check the `*_abs` comparisons against `abs()`
// on purpose.
fn in_comparison_module(cx: &LateContext<'_>, span: rustc_span::Span) -> bool {
    let rustc_span::FileName::Real(real) = cx.sess().source_map().span_to_filename(span) else {
        return false;
    };
    real.local_path().is_some_and(|path| {
        let path = path.to_string_lossy().replace('\\', "/");
        path.contains("/comparison/") || path.contains("_comparison_")
    })
}

impl<'tcx> LateLintPass<'tcx> for UseAbsComparison {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || in_comparison_module(cx, expr.span) {
            return;
        }
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) => {
                let Some((method, trait_name)) = abs_method(op.node, false) else {
                    return;
                };
                if let Some(x) = abs_call(cx, lhs)
                    && let Some(y) = nonnegative(cx, rhs)
                {
                    lint(cx, expr, x, y, method, trait_name);
                } else if let Some(x) = abs_call(cx, rhs)
                    && let Some(y) = nonnegative(cx, lhs)
                {
                    let (method, trait_name) = abs_method(op.node, true).unwrap();
                    lint(cx, expr, x, y, method, trait_name);
                }
            }
            ExprKind::MethodCall(seg, recv, [arg], _) => {
                let (method, trait_name) = match seg.ident.name.as_str() {
                    "eq" => ("eq_abs", "EqAbs"),
                    "ne" => ("ne_abs", "EqAbs"),
                    "partial_cmp" => ("partial_cmp_abs", "PartialOrdAbs"),
                    "lt" => ("lt_abs", "PartialOrdAbs"),
                    "le" => ("le_abs", "PartialOrdAbs"),
                    "gt" => ("gt_abs", "PartialOrdAbs"),
                    "ge" => ("ge_abs", "PartialOrdAbs"),
                    "cmp" => ("cmp_abs", "OrdAbs"),
                    _ => return,
                };
                if let Some(x) = abs_call(cx, recv)
                    && let Some(y) = nonnegative(cx, arg)
                {
                    lint(cx, expr, x, y, method, trait_name);
                }
            }
            _ => {}
        }
    }
}
