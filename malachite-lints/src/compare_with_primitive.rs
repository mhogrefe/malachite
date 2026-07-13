// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::ty::implements_trait;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags comparing a bignum (`Natural`, `Integer`, `Rational`, or `Float`) with a small named
    /// bignum constant, or with `from(primitive)` in a comparison method, when the bignum could be
    /// compared with the primitive directly: `x == Rational::ONE`, `x.cmp(&Rational::ONE)`, or
    /// `x.partial_cmp(&Natural::from(10u32))`.
    ///
    /// ### Why is this bad?
    ///
    /// The bignum types implement `PartialEq` and `PartialOrd` directly against the primitive
    /// types, so materializing a bignum comparand is unnecessary: `*x == 1u32` and
    /// `x.partial_cmp(&1u32).unwrap()` mean the same thing. (Comparing with an unsigned literal is
    /// preferred when the value is nonnegative.) The operator-with-`from` form is covered by
    /// `redundant_from_in_comparison`; this lint covers named constants and the comparison
    /// methods.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// match x.cmp(&Rational::ONE) { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// match x.partial_cmp(&1u32).unwrap() { .. }
    /// ```
    pub COMPARE_WITH_PRIMITIVE,
    Deny,
    "comparing a bignum with a bignum constant or conversion instead of with a primitive"
}

declare_lint_pass!(CompareWithPrimitive => [COMPARE_WITH_PRIMITIVE]);

// The named constants with primitive equivalents, and the literals to suggest. Unsigned literals
// are preferred for nonnegative values.
const CONSTANTS: [(&str, &str, bool); 4] = [
    ("ZERO", "0u32", false),
    ("ONE", "1u32", false),
    ("TWO", "2u32", false),
    ("NEGATIVE_ONE", "-1i32", true),
];

// If `e` (after peeling `&`) is a named bignum constant with a primitive equivalent, or (when
// `allow_from` is set) a bignum `from(primitive-literal-or-value)` call, returns the primitive
// spelling to suggest and whether it is signed.
fn primitive_equivalent<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
    allow_from: bool,
) -> Option<(String, bool)> {
    let mut e = e;
    while let ExprKind::AddrOf(_, _, inner) = e.kind {
        e = inner;
    }
    if crate::bignum_name(cx, cx.typeck_results().expr_ty(e)).is_none() {
        return None;
    }
    match e.kind {
        ExprKind::Path(ref qpath) => {
            let Res::Def(DefKind::AssocConst { .. }, did) = cx.qpath_res(qpath, e.hir_id) else {
                return None;
            };
            let name = cx.tcx.item_name(did);
            CONSTANTS
                .iter()
                .find(|(c, _, _)| name.as_str() == *c)
                .map(|&(_, lit, signed)| (lit.to_string(), signed))
        }
        ExprKind::Call(callee, [arg]) if allow_from => {
            let ExprKind::Path(ref qpath) = callee.kind else {
                return None;
            };
            let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
                return None;
            };
            if cx.tcx.item_name(fn_did).as_str() != "from" {
                return None;
            }
            let arg_ty = cx.typeck_results().expr_ty(arg);
            if !arg_ty.is_integral() {
                return None;
            }
            Some((
                clippy_utils::source::snippet(cx, arg.span, "..").to_string(),
                arg_ty.is_signed(),
            ))
        }
        _ => None,
    }
}

// Whether the bignum type implements the required comparison trait against `u32` (or `i32` for
// signed suggestions), so that dropping the bignum comparand still compiles.
fn primitive_comparable<'tcx>(
    cx: &LateContext<'tcx>,
    bignum_ty: Ty<'tcx>,
    signed: bool,
    eq_only: bool,
) -> bool {
    let prim_ty = if signed {
        cx.tcx.types.i32
    } else {
        cx.tcx.types.u32
    };
    let trait_did = if eq_only {
        cx.tcx.lang_items().eq_trait()
    } else {
        cx.tcx.lang_items().partial_ord_trait()
    };
    trait_did.is_some_and(|did| implements_trait(cx, bignum_ty, did, &[prim_ty.into()]))
}

impl<'tcx> LateLintPass<'tcx> for CompareWithPrimitive {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            // Operator comparisons against a named constant. (The `from(primitive)` operator form
            // is `redundant_from_in_comparison`'s.)
            ExprKind::Binary(op, lhs, rhs) => {
                let is_eq = matches!(op.node, BinOpKind::Eq | BinOpKind::Ne);
                if !is_eq
                    && !matches!(
                        op.node,
                        BinOpKind::Lt | BinOpKind::Le | BinOpKind::Gt | BinOpKind::Ge
                    )
                {
                    return;
                }
                for (c, other) in [(lhs, rhs), (rhs, lhs)] {
                    let Some((lit, signed)) = primitive_equivalent(cx, c, false) else {
                        continue;
                    };
                    let other_ty = cx.typeck_results().expr_ty(other).peel_refs();
                    if crate::bignum_name(cx, other_ty).is_none() {
                        continue;
                    }
                    if !primitive_comparable(cx, other_ty, signed, is_eq) {
                        continue;
                    }
                    span_lint(
                        cx,
                        COMPARE_WITH_PRIMITIVE,
                        c.span,
                        format!("compare with `{lit}` directly instead of with a bignum constant"),
                    );
                    return;
                }
            }
            // `cmp`, `partial_cmp`, `eq`, and `ne` against a named constant or `from(primitive)`.
            ExprKind::MethodCall(seg, recv, [arg], _) => {
                let name = seg.ident.name.as_str();
                let eq_only = match name {
                    "cmp" | "partial_cmp" => false,
                    "eq" | "ne" => true,
                    _ => return,
                };
                let recv_ty = cx.typeck_results().expr_ty(recv).peel_refs();
                if crate::bignum_name(cx, recv_ty).is_none() {
                    return;
                }
                let Some((lit, signed)) = primitive_equivalent(cx, arg, true) else {
                    return;
                };
                if !primitive_comparable(cx, recv_ty, signed, eq_only) {
                    return;
                }
                let suggestion = match name {
                    // Cross-type `Ord` does not exist, but the primitive `PartialOrd` is total for
                    // these types.
                    "cmp" => format!("partial_cmp(&{lit}).unwrap()"),
                    "partial_cmp" => format!("partial_cmp(&{lit})"),
                    "eq" => format!("== {lit}"),
                    _ => format!("!= {lit}"),
                };
                span_lint(
                    cx,
                    COMPARE_WITH_PRIMITIVE,
                    expr.span,
                    format!(
                        "use `{suggestion}` instead of comparing with a bignum constant or \
                         conversion"
                    ),
                );
            }
            _ => {}
        }
    }
}
