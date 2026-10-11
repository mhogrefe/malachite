// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::use_fused_mul::{as_mul, camel, has_trait_impl, inside_own_definition, operand_ty};
use clippy_utils::diagnostics::span_lint;
use clippy_utils::eq_expr_value;
use rustc_hir::{AssignOpKind, BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a product that is shifted left as a separate step from the addition or subtraction
    /// that consumes it, where a fused operation exists:
    ///
    /// - `x + ((y * z) << k)`, `x - ((y * z) << k)`, and the `+=` and `-=` forms, which are
    ///   `add_mul_shl` and `sub_mul_shl`;
    /// - `x.add_mul(y << k, z)` and `x.sub_mul(y << k, z)` (with either factor shifted), and their
    ///   `_assign` forms, which are the same operations;
    /// - `x.mod_power_of_2_add(y.mod_power_of_2_mul(z, p).mod_power_of_2_shl(k, p), p)`, the same
    ///   with the shift applied to a factor, and `x.mod_power_of_2_add_mul(y.mod_power_of_2_shl(k,
    ///   p), z, p)`, which are `mod_power_of_2_add_mul_shl`, and likewise for subtraction;
    /// - the same chains built from `mod_add`, `mod_mul`, and `mod_shl`, which are
    ///   `mod_add_mul_shl` and `mod_sub_mul_shl`.
    ///
    /// ### Why is this bad?
    ///
    /// The fused operations do not materialize the shifted product, and the modular ones reduce
    /// the work further: `mod_power_of_2_add_mul_shl` multiplies only the bits that survive the
    /// shift, and the vector forms shift the scalar once instead of every element.
    ///
    /// ### Known problems
    ///
    /// The shift amount must be of an unsigned type, since a negative signed amount shifts the
    /// other way, and the modular chains must use the same modulus throughout. The lint applies
    /// only where the receiver's type implements the suggested trait, so `Natural`, which has no
    /// `SubMulShl`, is flagged only for addition, and primitive integers only for the modular
    /// forms, since their operator forms would wrap where `+` panics.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let a = &x + ((&y * &z) << 3);
    /// let b = x.mod_add(y.mod_mul(z, m).mod_shl(3, m), m);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let a = (&x).add_mul_shl(&y, &z, 3);
    /// let b = x.mod_add_mul_shl(y, z, 3, m);
    /// ```
    pub USE_FUSED_MUL_SHL,
    Deny,
    "shifting a product separately instead of using a fused operation"
}

declare_lint_pass!(UseFusedMulShl => [USE_FUSED_MUL_SHL]);

const TRAIT_ROOT: &str = "malachite_base::num::arithmetic::traits";

fn is_unsigned<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> bool {
    matches!(
        cx.typeck_results().expr_ty(e).peel_refs().kind(),
        rustc_middle::ty::Uint(_)
    )
}

// Whether `e` is `a << k` for an unsigned `k`.
fn is_unsigned_shl<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> bool {
    matches!(
        crate::peel_clone_and_borrows(e).kind,
        ExprKind::Binary(op, _, k) if op.node == BinOpKind::Shl && is_unsigned(cx, k)
    )
}

// Whether `e` is `(y * z) << k` for an unsigned `k`.
fn is_shifted_product<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> bool {
    match crate::peel_clone_and_borrows(e).kind {
        ExprKind::Binary(op, l, k) if op.node == BinOpKind::Shl => {
            is_unsigned(cx, k) && as_mul(l).is_some()
        }
        _ => false,
    }
}

fn same_value<'tcx>(cx: &LateContext<'tcx>, a: &'tcx Expr<'tcx>, b: &'tcx Expr<'tcx>) -> bool {
    eq_expr_value(
        cx,
        crate::peel_clone_and_borrows(a),
        crate::peel_clone_and_borrows(b),
    )
}

// The method names of one modular family: the multiplication and the shift that a fused
// add-mul-shl replaces.
struct Family {
    mul: &'static str,
    shl: &'static str,
}

const POWER_OF_2: Family = Family {
    mul: "mod_power_of_2_mul",
    shl: "mod_power_of_2_shl",
};

const GENERAL: Family = Family {
    mul: "mod_mul",
    shl: "mod_shl",
};

// Whether `e` is `w.<shl>(k, modulus)` for an unsigned `k`.
fn is_mod_shl<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
    family: &Family,
    modulus: &'tcx Expr<'tcx>,
) -> bool {
    matches!(
        crate::peel_clone_and_borrows(e).kind,
        ExprKind::MethodCall(seg, _, [k, m], _)
            if seg.ident.name.as_str() == family.shl
                && is_unsigned(cx, k)
                && same_value(cx, m, modulus)
    )
}

// Whether `e` is a shifted product in `family`, all modulo `modulus`: the product shifted, as in
// `y.mod_mul(z, m).mod_shl(k, m)`, or a factor shifted, as in `y.mod_shl(k, m).mod_mul(z, m)`.
fn is_mod_shifted_product<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
    family: &Family,
    modulus: &'tcx Expr<'tcx>,
) -> bool {
    match crate::peel_clone_and_borrows(e).kind {
        ExprKind::MethodCall(seg, recv, [k, m], _)
            if seg.ident.name.as_str() == family.shl
                && is_unsigned(cx, k)
                && same_value(cx, m, modulus) =>
        {
            matches!(
                crate::peel_clone_and_borrows(recv).kind,
                ExprKind::MethodCall(inner, _, [_, m2], _)
                    if inner.ident.name.as_str() == family.mul && same_value(cx, m2, modulus)
            )
        }
        ExprKind::MethodCall(seg, recv, [z, m], _)
            if seg.ident.name.as_str() == family.mul && same_value(cx, m, modulus) =>
        {
            is_mod_shl(cx, recv, family, modulus) || is_mod_shl(cx, z, family, modulus)
        }
        _ => false,
    }
}

// Flags `expr` with advice to call `fused` if the receiver's type implements its trait, and the
// expression is not part of the definition of `fused` itself.
fn suggest<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
    receiver: &'tcx Expr<'tcx>,
    fused: &str,
) {
    let ty = operand_ty(cx, receiver);
    if !inside_own_definition(cx, expr, fused)
        && has_trait_impl(cx, ty, &format!("{TRAIT_ROOT}::{}", camel(fused)))
    {
        span_lint(
            cx,
            USE_FUSED_MUL_SHL,
            expr.span,
            format!("use `{fused}()` instead of shifting the product separately"),
        );
    }
}

// `add_mul` -> `add_mul_shl`, `mod_add_mul_assign` -> `mod_add_mul_shl_assign`.
fn with_shl(name: &str) -> String {
    match name.strip_suffix("_assign") {
        Some(stem) => format!("{stem}_shl_assign"),
        None => format!("{name}_shl"),
    }
}

impl<'tcx> LateLintPass<'tcx> for UseFusedMulShl {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_cross_check_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            // `x + ((y * z) << k)`, `((y * z) << k) + x`, and `x - ((y * z) << k)`.
            ExprKind::Binary(op, lhs, rhs) => {
                let name = match op.node {
                    BinOpKind::Add => "add",
                    BinOpKind::Sub => "sub",
                    _ => return,
                };
                let addend = if is_shifted_product(cx, rhs) {
                    lhs
                } else if name == "add" && is_shifted_product(cx, lhs) {
                    rhs
                } else {
                    return;
                };
                suggest(cx, expr, addend, &format!("{name}_mul_shl"));
            }
            ExprKind::AssignOp(op, lhs, rhs) => {
                let name = match op.node {
                    AssignOpKind::AddAssign => "add",
                    AssignOpKind::SubAssign => "sub",
                    _ => return,
                };
                if is_shifted_product(cx, rhs) {
                    suggest(cx, expr, lhs, &format!("{name}_mul_shl_assign"));
                }
            }
            ExprKind::MethodCall(seg, recv, args, _) => {
                let name = seg.ident.name.as_str();
                let base = name.strip_suffix("_assign").unwrap_or(name);
                match (base, args) {
                    // `x.add_mul(y << k, z)`, with either factor shifted.
                    ("add_mul" | "sub_mul", [y, z])
                        if is_unsigned_shl(cx, y) || is_unsigned_shl(cx, z) =>
                    {
                        suggest(cx, expr, recv, &with_shl(name));
                    }
                    // `x.mod_power_of_2_add(<shifted product>, p)`.
                    ("mod_power_of_2_add" | "mod_power_of_2_sub", [a, p])
                        if is_mod_shifted_product(cx, a, &POWER_OF_2, p) =>
                    {
                        suggest(
                            cx,
                            expr,
                            recv,
                            &with_shl(
                                &name
                                    .replacen("_add", "_add_mul", 1)
                                    .replacen("_sub", "_sub_mul", 1),
                            ),
                        );
                    }
                    // `x.mod_power_of_2_add_mul(y.mod_power_of_2_shl(k, p), z, p)`.
                    ("mod_power_of_2_add_mul" | "mod_power_of_2_sub_mul", [y, z, p])
                        if is_mod_shl(cx, y, &POWER_OF_2, p)
                            || is_mod_shl(cx, z, &POWER_OF_2, p) =>
                    {
                        suggest(cx, expr, recv, &with_shl(name));
                    }
                    // `x.mod_add(<shifted product>, m)`.
                    ("mod_add" | "mod_sub", [a, m])
                        if is_mod_shifted_product(cx, a, &GENERAL, m) =>
                    {
                        suggest(
                            cx,
                            expr,
                            recv,
                            &with_shl(
                                &name
                                    .replacen("_add", "_add_mul", 1)
                                    .replacen("_sub", "_sub_mul", 1),
                            ),
                        );
                    }
                    // `x.mod_add_mul(y.mod_shl(k, m), z, m)`.
                    ("mod_add_mul" | "mod_sub_mul", [y, z, m])
                        if is_mod_shl(cx, y, &GENERAL, m) || is_mod_shl(cx, z, &GENERAL, m) =>
                    {
                        suggest(cx, expr, recv, &with_shl(name));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
