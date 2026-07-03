// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::ty::implements_trait;
use rustc_hir::{BinOpKind, Expr, ExprKind, LangItem};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags cloning a bignum where a by-reference alternative exists: `x.clone().op(..)` when the
    /// family has an `op_ref*` variant, `y.op(x.clone(), ..)` when it has an `op*_ref` variant,
    /// and `x.clone() * y` (or `x *= y.clone()`, etc.) when the operator is implemented for
    /// references.
    ///
    /// ### Why is this bad?
    ///
    /// Cloning a bignum can copy an arbitrarily large value; the `_val`/`_ref` families and the
    /// reference operator impls exist precisely to avoid that.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = x.clone().exp_prec(p);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = x.exp_prec_ref(p);
    /// ```
    pub CLONE_WITH_REF_VARIANT,
    Deny,
    "cloning a bignum where a by-reference variant exists"
}

declare_lint_pass!(CloneWithRefVariant => [CLONE_WITH_REF_VARIANT]);

// If `e` is `x.clone()` where `x` is a bignum, returns `x`.
fn bignum_clone<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::MethodCall(seg, recv, [], _) = e.kind else {
        return None;
    };
    if seg.ident.name.as_str() != "clone" {
        return None;
    }
    crate::bignum_adt_did(cx, cx.typeck_results().expr_ty(recv)).map(|_| recv)
}

// The lang item of the trait implementing a binary operator.
fn op_lang_item(op: BinOpKind, assign: bool) -> Option<LangItem> {
    Some(match (op, assign) {
        (BinOpKind::Add, false) => LangItem::Add,
        (BinOpKind::Sub, false) => LangItem::Sub,
        (BinOpKind::Mul, false) => LangItem::Mul,
        (BinOpKind::Div, false) => LangItem::Div,
        (BinOpKind::Rem, false) => LangItem::Rem,
        (BinOpKind::BitAnd, false) => LangItem::BitAnd,
        (BinOpKind::BitOr, false) => LangItem::BitOr,
        (BinOpKind::BitXor, false) => LangItem::BitXor,
        (BinOpKind::Shl, false) => LangItem::Shl,
        (BinOpKind::Shr, false) => LangItem::Shr,
        (BinOpKind::Add, true) => LangItem::AddAssign,
        (BinOpKind::Sub, true) => LangItem::SubAssign,
        (BinOpKind::Mul, true) => LangItem::MulAssign,
        (BinOpKind::Div, true) => LangItem::DivAssign,
        (BinOpKind::Rem, true) => LangItem::RemAssign,
        (BinOpKind::BitAnd, true) => LangItem::BitAndAssign,
        (BinOpKind::BitOr, true) => LangItem::BitOrAssign,
        (BinOpKind::BitXor, true) => LangItem::BitXorAssign,
        (BinOpKind::Shl, true) => LangItem::ShlAssign,
        (BinOpKind::Shr, true) => LangItem::ShrAssign,
        _ => return None,
    })
}

fn ref_of<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
    Ty::new_imm_ref(cx.tcx, cx.tcx.lifetimes.re_erased, ty)
}

impl<'tcx> LateLintPass<'tcx> for CloneWithRefVariant {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            ExprKind::MethodCall(seg, recv, args, _) => {
                let name = seg.ident.name.as_str();
                if name == "clone" {
                    return;
                }
                let base = crate::strip_variant_suffixes(name);
                // A cloned receiver, with a receiver-by-reference sibling available. The right
                // sibling depends on the current variant: a plain `foo` can move to `foo_ref`
                // (unary), `foo_ref_val`, or `foo_ref_ref`; a `foo_val_ref` moves to
                // `foo_ref_ref`.
                if let Some(x) = bignum_clone(cx, recv)
                    && let Some(adt_did) = crate::bignum_adt_did(cx, cx.typeck_results().expr_ty(x))
                {
                    let candidates: &[String] = if name == base {
                        &[
                            format!("{base}_ref"),
                            format!("{base}_ref_val"),
                            format!("{base}_ref_ref"),
                        ]
                    } else if name == format!("{base}_val_ref") {
                        &[format!("{base}_ref_ref")]
                    } else {
                        &[]
                    };
                    for cand in candidates {
                        if crate::has_inherent_fn(cx, adt_did, cand) {
                            span_lint(
                                cx,
                                CLONE_WITH_REF_VARIANT,
                                expr.span,
                                format!(
                                    "avoid cloning the receiver: use `{cand}` on a reference \
                                    instead"
                                ),
                            );
                            return;
                        }
                    }
                }
                // A cloned argument, with an argument-by-reference sibling available: a plain
                // `foo` moves to `foo_val_ref`, a `foo_ref_val` to `foo_ref_ref`, and an
                // `_assign`-family `foo` to `foo_ref`.
                for arg in args {
                    if let Some(x) = bignum_clone(cx, arg)
                        && let Some(adt_did) =
                            crate::bignum_adt_did(cx, cx.typeck_results().expr_ty(x))
                    {
                        let candidates: &[String] = if name == base {
                            &[format!("{base}_val_ref")]
                        } else if name == format!("{base}_ref_val") {
                            &[format!("{base}_ref_ref")]
                        } else if name.ends_with("_assign") {
                            &[format!("{name}_ref")]
                        } else {
                            &[]
                        };
                        for cand in candidates {
                            if crate::has_inherent_fn(cx, adt_did, cand) {
                                span_lint(
                                    cx,
                                    CLONE_WITH_REF_VARIANT,
                                    expr.span,
                                    format!(
                                        "avoid cloning the argument: use `{cand}` with a \
                                        reference instead"
                                    ),
                                );
                                return;
                            }
                        }
                    }
                }
            }
            ExprKind::Binary(_, lhs, rhs) | ExprKind::AssignOp(_, lhs, rhs) => {
                let (op_kind, assign) = match expr.kind {
                    ExprKind::Binary(o, ..) => (o.node, false),
                    ExprKind::AssignOp(o, ..) => (o.node.into(), true),
                    _ => unreachable!(),
                };
                let Some(item) = op_lang_item(op_kind, assign) else {
                    return;
                };
                let Some(trait_did) = cx.tcx.lang_items().get(item) else {
                    return;
                };
                // A cloned left operand (not for compound assignment, whose left side is a
                // place): the operator must be implemented for `&T op Rhs`.
                if !assign && let Some(x) = bignum_clone(cx, lhs) {
                    let x_ref = ref_of(cx, cx.typeck_results().expr_ty(x));
                    let rhs_ty = cx.typeck_results().expr_ty(rhs);
                    if implements_trait(cx, x_ref, trait_did, &[rhs_ty.into()]) {
                        span_lint(
                            cx,
                            CLONE_WITH_REF_VARIANT,
                            expr.span,
                            "borrow instead of cloning: this operator is implemented for \
                            references",
                        );
                        return;
                    }
                }
                // A cloned right operand: the operator must be implemented for `Lhs op &T`.
                if let Some(x) = bignum_clone(cx, rhs) {
                    let lhs_ty = cx.typeck_results().expr_ty(lhs);
                    let x_ref = ref_of(cx, cx.typeck_results().expr_ty(x));
                    if implements_trait(cx, lhs_ty, trait_did, &[x_ref.into()]) {
                        span_lint(
                            cx,
                            CLONE_WITH_REF_VARIANT,
                            expr.span,
                            "borrow instead of cloning: this operator is implemented for \
                            references",
                        );
                    }
                }
            }
            _ => {}
        }
    }
}
