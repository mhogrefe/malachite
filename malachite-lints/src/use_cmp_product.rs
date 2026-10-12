// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::use_fused_mul::{as_mul, operand_ty};
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{BinOpKind, Expr, ExprKind, UnOp};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags comparing a product of two values with a third value, or with another product, where
    /// the values' type has the product-comparison functions: `a * b` against `c` is
    /// `T::cmp_product(&a, &b, &c)`, `a * b` against `c * d` is `T::cmp_products(&a, &b, &c, &d)`,
    /// and against `-c` or `-(c * d)` they are `cmp_product_neg` and `cmp_products_neg`. Covers
    /// the comparison operators, `cmp`, `partial_cmp`, and `lt`/`le`/`gt`/`ge`/`eq`/`ne`, with the
    /// product on either side. All the factors and the compared value must have the same type,
    /// which must implement the suggested trait (`Natural`, `Integer`, and `Rational` do).
    ///
    /// ### Why is this bad?
    ///
    /// Forming a product only to compare it allocates a value that is thrown away, and for
    /// `Rational` the product is also reduced to lowest terms. The product-comparison functions
    /// usually decide the comparison from signs, bit lengths, and comparisons of the factors,
    /// multiplying only when those cannot. When they must multiply, they are a little slower than
    /// multiplying directly, so a site where the comparison is almost always that close can opt
    /// out with `#[cfg_attr(dylint_lib = "malachite_lints", allow(use_cmp_product))]`.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if &a * &b < c { .. }
    /// (&a * &b).cmp(&(&c * &d))
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if Natural::cmp_product(&a, &b, &c) == Less { .. }
    /// Natural::cmp_products(&a, &b, &c, &d)
    /// ```
    pub USE_CMP_PRODUCT,
    Deny,
    "comparing a product of bignums instead of using `cmp_product` or `cmp_products`"
}

declare_lint_pass!(UseCmpProduct => [USE_CMP_PRODUCT]);

const TRAITS: &str = "malachite_base::num::comparison::traits";

// If `e` is a product of two values of type `ty`, its factors.
fn product_of<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
    ty: Ty<'tcx>,
) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
    let (l, r) = as_mul(e)?;
    (operand_ty(cx, l) == ty && operand_ty(cx, r) == ty).then_some((l, r))
}

// If `e` is a negation, the negated value.
fn negation<'tcx>(e: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    match crate::peel_clone_and_borrows(e).kind {
        ExprKind::Unary(UnOp::Neg, inner) => Some(inner),
        _ => None,
    }
}

// The replacement for comparing `product` with `other`, if the type has it: the trait, the
// function, and the argument shape.
fn replacement<'tcx>(
    cx: &LateContext<'tcx>,
    other: &'tcx Expr<'tcx>,
    ty: Ty<'tcx>,
) -> Option<(&'static str, &'static str, &'static str)> {
    let implements = |t: &str| crate::implements_trait_by_path(cx, ty, &format!("{TRAITS}::{t}"));
    let choice = if let Some(inner) = negation(other) {
        if product_of(cx, inner, ty).is_some() {
            ("OrdProductsNeg", "cmp_products_neg", "(&a, &b, &c, &d)")
        } else if operand_ty(cx, inner) == ty {
            ("OrdProductNeg", "cmp_product_neg", "(&a, &b, &c)")
        } else {
            return None;
        }
    } else if product_of(cx, other, ty).is_some() {
        ("OrdProducts", "cmp_products", "(&a, &b, &c, &d)")
    } else if operand_ty(cx, other) == ty {
        ("OrdProduct", "cmp_product", "(&a, &b, &c)")
    } else {
        return None;
    };
    implements(choice.0).then_some(choice)
}

impl<'tcx> LateLintPass<'tcx> for UseCmpProduct {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_cross_check_code(cx, expr.span) {
            return;
        }
        let (a, b) = match expr.kind {
            ExprKind::Binary(op, lhs, rhs)
                if matches!(
                    op.node,
                    BinOpKind::Eq
                        | BinOpKind::Ne
                        | BinOpKind::Lt
                        | BinOpKind::Le
                        | BinOpKind::Gt
                        | BinOpKind::Ge
                ) =>
            {
                (lhs, rhs)
            }
            ExprKind::MethodCall(seg, receiver, [arg], _)
                if matches!(
                    seg.ident.name.as_str(),
                    "cmp" | "partial_cmp" | "eq" | "ne" | "lt" | "le" | "gt" | "ge"
                ) =>
            {
                (receiver, arg)
            }
            _ => return,
        };
        // The product-comparison functions' own definitions multiply as their last resort.
        let owner = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
        if cx
            .tcx
            .item_name(owner.to_def_id())
            .as_str()
            .contains("cmp_product")
        {
            return;
        }
        for (product, other, reversed) in [(a, b, false), (b, a, true)] {
            let Some((factor, _)) = as_mul(product) else {
                continue;
            };
            let ty = operand_ty(cx, factor);
            if crate::bignum_name(cx, ty).is_none() || product_of(cx, product, ty).is_none() {
                continue;
            }
            let Some((_, function, args)) = replacement(cx, other, ty) else {
                continue;
            };
            let help = if reversed {
                format!(
                    "the product is on the right, so the comparison is reversed: \
                     `T::{function}{args}` compares the product on the left"
                )
            } else {
                format!(
                    "`T::{function}{args}` decides most comparisons from signs, bit lengths, and \
                     the factors, multiplying only when it must"
                )
            };
            span_lint_and_help(
                cx,
                USE_CMP_PRODUCT,
                expr.span,
                format!("use `{function}` instead of comparing a product"),
                None,
                help,
            );
            return;
        }
    }
}
