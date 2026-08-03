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
use rustc_middle::ty::{Ty, TyKind};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a product that is immediately right-shifted, where the fused `mul_shr_round` exists:
    /// `(x * y).shr_round(bits, rm)` and `(x * y) >> bits` on `Natural` or `Integer`, and the
    /// widening idiom `(u128::from(x) * u128::from(y)) >> bits` (or via `as`) on primitives.
    ///
    /// ### Why is this bad?
    ///
    /// For bignums, `mul_shr_round` never computes the part of the product that the shift
    /// discards: when most of it is discarded, the fused operation is faster by an unbounded
    /// factor (measured up to ~48x at 33-kilobit products). For primitives, the fused operation
    /// says directly what the widening spelling encodes indirectly, and handles the rounding
    /// mode and exactness uniformly.
    ///
    /// ### Known problems
    ///
    /// A plain `(x * y) >> bits` on a *primitive* type is deliberately not flagged: the in-type
    /// product has already discarded overflow, so the fused operation, which computes the exact
    /// double-width product, is not an equivalent rewrite. In width-critical kernel code the
    /// widening spelling may also be deliberate, to avoid the fused operation's rounding
    /// bookkeeping; such sites can carry an `expect` with a justifying comment.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let (hi, o) = (&x * &y).shr_round(1000, Floor);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let (hi, o) = (&x).mul_shr_round(&y, 1000, Floor);
    /// ```
    pub USE_MUL_SHR_ROUND,
    Deny,
    "shifting a product right instead of using the fused mul_shr_round"
}

declare_lint_pass!(UseMulShrRound => [USE_MUL_SHR_ROUND]);

// Whether `expr` sits inside the implementation of the operation itself, whose full-product
// fallback legitimately spells out the composition.
fn inside_own_definition<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> bool {
    let did = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
    cx.tcx
        .item_name(did.to_def_id())
        .as_str()
        .contains("mul_shr_round")
}

// If `e` is a multiplication, returns its two operands.
fn as_mul<'tcx>(e: &'tcx Expr<'tcx>) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
    match crate::peel_clone_and_borrows(e).kind {
        ExprKind::Binary(op, l, r) if op.node == BinOpKind::Mul => Some((l, r)),
        _ => None,
    }
}

// If `e` widens a narrower expression -- `W::from(x)`, `From::from(x)`, or `x as W` -- returns
// the type of the narrower expression.
fn widened_from<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<Ty<'tcx>> {
    match crate::peel_clone_and_borrows(e).kind {
        ExprKind::Call(callee, [arg]) => {
            if let ExprKind::Path(qpath) = &callee.kind
                && let Some(seg) = qpath_last_segment_name(qpath)
                && seg == "from"
            {
                Some(cx.typeck_results().expr_ty(arg))
            } else {
                None
            }
        }
        ExprKind::Cast(inner, _) => Some(cx.typeck_results().expr_ty(inner)),
        _ => None,
    }
}

fn qpath_last_segment_name<'a>(qpath: &'a rustc_hir::QPath<'a>) -> Option<&'a str> {
    match qpath {
        rustc_hir::QPath::Resolved(_, path) => path.segments.last().map(|s| s.ident.name.as_str()),
        rustc_hir::QPath::TypeRelative(_, seg) => Some(seg.ident.name.as_str()),
    }
}

// Whether the product `l * r`, of type `prod_ty`, is a doubled-width product of two equal
// narrower primitive-integer expressions -- the manual spelling of what `mul_shr_round` does
// internally.
fn is_widening_product<'tcx>(
    cx: &LateContext<'tcx>,
    prod_ty: Ty<'tcx>,
    l: &'tcx Expr<'tcx>,
    r: &'tcx Expr<'tcx>,
) -> bool {
    let (Some(lt), Some(rt)) = (widened_from(cx, l), widened_from(cx, r)) else {
        return false;
    };
    if lt != rt {
        return false;
    }
    let width = |ty: Ty<'tcx>| match ty.kind() {
        TyKind::Uint(u) => u.bit_width(),
        TyKind::Int(i) => i.bit_width(),
        _ => None,
    };
    match (width(prod_ty), width(lt)) {
        (Some(w), Some(n)) => w == n * 2,
        _ => false,
    }
}

impl<'tcx> LateLintPass<'tcx> for UseMulShrRound {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || crate::in_test_code(cx, expr.span)
            || inside_own_definition(cx, expr)
        {
            return;
        }
        let product = match expr.kind {
            // `(x * y).shr_round(bits, rm)`
            ExprKind::MethodCall(seg, recv, [_, _], _) if seg.ident.name.as_str() == "shr_round" => {
                recv
            }
            // `(x * y) >> bits`
            ExprKind::Binary(op, lhs, _) if op.node == BinOpKind::Shr => lhs,
            _ => return,
        };
        let Some((l, r)) = as_mul(product) else {
            return;
        };
        let prod_ty = cx
            .typeck_results()
            .expr_ty(crate::peel_clone_and_borrows(product))
            .peel_refs();
        if crate::bignum_name(cx, prod_ty).is_some() {
            span_lint(
                cx,
                USE_MUL_SHR_ROUND,
                expr.span,
                "use `mul_shr_round()` instead of shifting the product: it never computes the \
                part of the product that the shift discards",
            );
        } else if is_widening_product(cx, prod_ty, l, r) {
            span_lint(
                cx,
                USE_MUL_SHR_ROUND,
                expr.span,
                "use `mul_shr_round()` on the narrower type instead of widening, multiplying, \
                and shifting",
            );
        }
    }
}
