// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::eq_expr_value;
use rustc_hir::{AssignOpKind, BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a multiplication written as a separate step from the addition or subtraction that
    /// consumes it, where a fused operation exists: `x + y * z`, `x - y * z`, `x * y + z * w`,
    /// `x * y - z * w`, and the `+=` and `-=` forms.
    ///
    /// ### Why is this bad?
    ///
    /// The fused operations do not materialize the product. For bignums that saves an
    /// allocation and a pass over the limbs, and for `x * y + z * w` the primitive
    /// implementations accumulate at double width, so the fused form is the faster spelling of
    /// the same value.
    ///
    /// ### Known problems
    ///
    /// Only the exact bignums are flagged for the operator forms. Primitive integers are
    /// excluded because the rewrite is not sound: `add_mul` and its relatives wrap on overflow,
    /// whereas `x + y * z` panics in a debug build, so it would silently trade an overflow check
    /// for wrapping; on those types the lint flags an explicitly wrapping composition instead,
    /// which the fused operation matches exactly. Primitive floats are excluded because their
    /// `add_mul` is defined as `self + y * z`, so it saves nothing. `Float` is excluded because
    /// its fused operations are not the same value spelled differently: they round once instead
    /// of twice, and pay for the exact product, so the choice between the spellings is a
    /// semantic one that the lint must not make.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let a = &x + &y * &z;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let a = (&x).add_mul(&y, &z);
    /// ```
    pub USE_FUSED_MUL,
    Deny,
    "adding or subtracting a product instead of using a fused operation"
}

declare_lint_pass!(UseFusedMul => [USE_FUSED_MUL]);

const TRAIT_ROOT: &str = "malachite_base::num::arithmetic::traits";

// Whether rewriting the operator form is both sound and worth doing for values of type `ty`.
//
// Only the exact bignums qualify. They cannot overflow, so the fused form computes the same
// value, and it avoids materializing the product -- an allocation and a pass over the limbs.
//
// Primitive integers are excluded because the rewrite is not sound: `add_mul` and its relatives
// wrap, while `x + y * z` panics in a debug build. They are covered instead by the `wrapping_*`
// compositions below, which the fused operations match exactly.
//
// Primitive floats are excluded because there is nothing to gain: their `add_mul` is defined as
// `self + y * z`, so it neither fuses the rounding nor saves any work, and insisting on it would
// only make expressions like a polynomial evaluation harder to read.
//
// `Float` is excluded even though it has the fused traits, for the opposite reasons on both
// axes: its fused operations compute a different value (the product enters the addition exactly,
// with a single rounding at the end), and they cost more, not less (the exact product must be
// computed in full, where the rounded `*` uses the short-product kernel). Rewriting would
// silently change numeric results while pessimizing the code; reaching for `Float`'s fused
// operations is an accuracy decision for the author to make explicitly.
fn operator_form_is_worthwhile<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> bool {
    crate::bignum_name(cx, ty).is_some_and(|name| name != "Float")
}

// Whether any impl of the trait named by `path` has `ty` as its self type, ignoring references and
// generic arguments.
//
// `implements_trait_by_path` cannot answer this for the bignums. It leaves the trait's parameters
// as inference variables, and a bignum has an impl for every by-value/by-reference permutation of
// them, so the solver finds the goal ambiguous and reports no impl at all -- while a primitive,
// with exactly one impl, resolves fine. Scanning the impl list sidesteps the ambiguity.
fn has_trait_impl<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>, path: &str) -> bool {
    use clippy_utils::paths::{PathNS, lookup_path_str};
    use rustc_middle::ty::TyKind;
    let target = ty.peel_refs();
    lookup_path_str(cx.tcx, PathNS::Type, path)
        .into_iter()
        .filter(|did| cx.tcx.def_kind(*did) == rustc_hir::def::DefKind::Trait)
        .any(|trait_did| {
            cx.tcx.all_impls(trait_did).any(|impl_did| {
                let self_ty = cx.tcx.type_of(impl_did).instantiate_identity().peel_refs();
                match (self_ty.kind(), target.kind()) {
                    (TyKind::Adt(a, _), TyKind::Adt(b, _)) => a.did() == b.did(),
                    (TyKind::Float(a), TyKind::Float(b)) => a == b,
                    (TyKind::Int(a), TyKind::Int(b)) => a == b,
                    (TyKind::Uint(a), TyKind::Uint(b)) => a == b,
                    _ => false,
                }
            })
        })
}

// How the addend of `x * y + a` relates to the product.
enum Alias {
    // `a` is not one of the factors, and does not occur inside them: a fused call works.
    None,
    // `a` is a factor, as in `&x * &y + x`: the repeated operand can be factored out instead.
    Factor,
    // `a` occurs somewhere inside a factor, as in `&x * (&a + 1) + a`. A fused call would have to
    // borrow `a` for the factor while consuming it as the addend, and there is no tidy factoring
    // either, so say nothing.
    Nested,
}

// Whether the addend of `product + addend` also appears in the product, and how.
fn classify_alias<'tcx>(
    cx: &LateContext<'tcx>,
    addend: &'tcx Expr<'tcx>,
    product: &'tcx Expr<'tcx>,
) -> Alias {
    let Some((l, r)) = as_mul(product) else {
        return Alias::None;
    };
    let addend = crate::peel_clone_and_borrows(addend);
    for factor in [l, r] {
        if eq_expr_value(cx, addend, crate::peel_clone_and_borrows(factor)) {
            return Alias::Factor;
        }
    }
    let mut nested = false;
    for factor in [l, r] {
        clippy_utils::visitors::for_each_expr_without_closures(factor, |e: &Expr<'_>| {
            if eq_expr_value(cx, addend, crate::peel_clone_and_borrows(e)) {
                nested = true;
            }
            core::ops::ControlFlow::<()>::Continue(())
        });
    }
    if nested { Alias::Nested } else { Alias::None }
}

// Whether `expr` sits inside the implementation of the very operation being suggested.
//
// The fallback arms of `add_mul_limb_ref_ref` and its relatives spell out `x + y * z` because that
// *is* the definition; suggesting `add_mul` there would be circular. Comparing against the
// enclosing function's name catches the whole family, including the `limbs_*` helpers and the
// `_assign`/`_ref` variants.
fn inside_own_definition<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>, fused: &str) -> bool {
    let base = fused
        .trim_start_matches("wrapping_")
        .trim_start_matches("checked_")
        .trim_start_matches("saturating_")
        .trim_start_matches("overflowing_")
        .trim_end_matches("_assign");
    let did = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
    cx.tcx.item_name(did.to_def_id()).as_str().contains(base)
}

// Peels borrows and `.clone()` so that `&x`, `x.clone()`, and `x` all report the same type.
fn operand_ty<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Ty<'tcx> {
    cx.typeck_results()
        .expr_ty(crate::peel_clone_and_borrows(e))
        .peel_refs()
}

// If `e` is a multiplication, returns its two operands.
fn as_mul<'tcx>(e: &'tcx Expr<'tcx>) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
    match crate::peel_clone_and_borrows(e).kind {
        ExprKind::Binary(op, l, r) if op.node == BinOpKind::Mul => Some((l, r)),
        _ => None,
    }
}

// If `e` is `a.wrapping_mul(b)`, returns the receiver and argument.
fn as_wrapping_mul<'tcx>(e: &'tcx Expr<'tcx>) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
    match crate::peel_clone_and_borrows(e).kind {
        ExprKind::MethodCall(seg, recv, [arg], _) if seg.ident.name.as_str() == "wrapping_mul" => {
            Some((recv, arg))
        }
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for UseFusedMul {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) => {
                let (name, verb) = match op.node {
                    BinOpKind::Add => ("add", "adding"),
                    BinOpKind::Sub => ("sub", "subtracting"),
                    _ => return,
                };
                let add = name == "add";
                let ty = operand_ty(cx, lhs);
                if !operator_form_is_worthwhile(cx, ty) {
                    return;
                }
                // Check the four-operand form first: `x * y + z * w` is one `mul_add_mul`, not a
                // nested `add_mul`.
                if as_mul(lhs).is_some() && as_mul(rhs).is_some() {
                    let fused = format!("mul_{name}_mul");
                    if !inside_own_definition(cx, expr, &fused)
                        && has_trait_impl(cx, ty, &format!("{TRAIT_ROOT}::{}", camel(&fused)))
                    {
                        span_lint(
                            cx,
                            USE_FUSED_MUL,
                            expr.span,
                            format!("use `{fused}()` instead of {verb} the products separately"),
                        );
                    }
                    return;
                }
                // `x + y * z` and, for addition only, `y * z + x`. Subtraction is not
                // commutative, so `y * z - x` is not a `sub_mul`.
                let (addend, product) = if as_mul(rhs).is_some() {
                    (lhs, rhs)
                } else if as_mul(lhs).is_some() {
                    // `y * z + x` is an `add_mul`, since addition commutes. `y * z - x` is not a
                    // `sub_mul`, but if `x` is one of the factors it still factors out, so let it
                    // through to the alias classification below and stop there.
                    if !add && !matches!(classify_alias(cx, rhs, lhs), Alias::Factor) {
                        return;
                    }
                    (rhs, lhs)
                } else {
                    return;
                };
                match classify_alias(cx, addend, product) {
                    Alias::Nested => return,
                    Alias::Factor => {
                        // `x * y + x` is `x * (y + 1)`: one multiplication instead of a
                        // multiplication and an addition of a full-width value.
                        if crate::bignum_name(cx, ty).is_some() {
                            span_lint(
                                cx,
                                USE_FUSED_MUL,
                                expr.span,
                                format!(
                                    "the addend is also a factor: multiply by {} operand \
                                     instead, as in `x * (y {} 1)`",
                                    if add {
                                        "an incremented"
                                    } else {
                                        "a decremented"
                                    },
                                    if add { "+" } else { "-" },
                                ),
                            );
                        }
                        return;
                    }
                    Alias::None => {}
                }
                let fused = format!("{name}_mul");
                if !inside_own_definition(cx, expr, &fused)
                    && has_trait_impl(cx, ty, &format!("{TRAIT_ROOT}::{}", camel(&fused)))
                {
                    span_lint(
                        cx,
                        USE_FUSED_MUL,
                        expr.span,
                        format!("use `{fused}()` instead of forming the product separately"),
                    );
                }
            }
            ExprKind::AssignOp(op, lhs, rhs) => {
                let name = match op.node {
                    AssignOpKind::AddAssign => "add",
                    AssignOpKind::SubAssign => "sub",
                    _ => return,
                };
                if as_mul(rhs).is_none() {
                    return;
                }
                let ty = operand_ty(cx, lhs);
                if !operator_form_is_worthwhile(cx, ty) {
                    return;
                }
                let fused = format!("{name}_mul_assign");
                if !inside_own_definition(cx, expr, &fused)
                    && has_trait_impl(cx, ty, &format!("{TRAIT_ROOT}::{}", camel(&fused)))
                {
                    span_lint(
                        cx,
                        USE_FUSED_MUL,
                        expr.span,
                        format!("use `{fused}()` instead of forming the product separately"),
                    );
                }
            }
            // `x.wrapping_add(y.wrapping_mul(z))` -- the primitive-integer case, where the fused
            // operation wraps in exactly the same way.
            ExprKind::MethodCall(seg, recv, [arg], _) => {
                let outer = seg.ident.name.as_str();
                let name = match outer {
                    "wrapping_add" => "add",
                    "wrapping_sub" => "sub",
                    _ => return,
                };
                let ty = operand_ty(cx, recv);
                if !matches!(
                    ty.kind(),
                    rustc_middle::ty::Int(_) | rustc_middle::ty::Uint(_)
                ) {
                    return;
                }
                // `x.wrapping_mul(y).wrapping_add(z.wrapping_mul(w))` is a `mul_add_mul`.
                let verb = if name == "add" {
                    "adding"
                } else {
                    "subtracting"
                };
                let (fused, advice) =
                    if as_wrapping_mul(recv).is_some() && as_wrapping_mul(arg).is_some() {
                        (
                            format!("wrapping_mul_{name}_mul"),
                            format!("{verb} the products separately"),
                        )
                    } else if as_wrapping_mul(arg).is_some() {
                        (
                            format!("wrapping_{name}_mul"),
                            "forming the product separately".to_string(),
                        )
                    } else {
                        return;
                    };
                if !inside_own_definition(cx, expr, &fused)
                    && has_trait_impl(cx, ty, &format!("{TRAIT_ROOT}::{}", camel(&fused)))
                {
                    span_lint(
                        cx,
                        USE_FUSED_MUL,
                        expr.span,
                        format!("use `{fused}()` instead of {advice}"),
                    );
                }
            }
            _ => {}
        }
    }
}

// `add_mul_assign` -> `AddMulAssign`, so that a method name can name its trait.
fn camel(snake: &str) -> String {
    snake
        .split('_')
        .map(|w| {
            let mut cs = w.chars();
            match cs.next() {
                Some(c) => c.to_uppercase().collect::<String>() + cs.as_str(),
                None => String::new(),
            }
        })
        .collect()
}
