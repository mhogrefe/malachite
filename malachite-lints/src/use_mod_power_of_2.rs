// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use rustc_ast::ast::LitKind;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BinOpKind, Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyKind;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags `x & m` and `x &= m` where `m` is a constant one less than a power of 2 -- a literal
    /// like `7` or `0xffff` for unsigned primitives, or a named constant defined as
    /// `Natural::const_from(<such a literal>)` for `Natural` -- suggesting `mod_power_of_2(k)`.
    ///
    /// ### Why is this bad?
    ///
    /// `x.mod_power_of_2(k)` says what the mask means: the remainder modulo $2^k$. The mask
    /// spelling makes the reader count bits to recover $k$.
    ///
    /// ### Known problems
    ///
    /// Only unsigned primitives and `Natural` are flagged: for signed types and `Integer`,
    /// `mod_power_of_2` returns the unsigned/`Natural` remainder, so the rewrite changes the
    /// type. Masks that are named constants of primitive type, like `WIDTH_MASK`, are not
    /// flagged; the name already carries the meaning, and `use_width_mask` deliberately steers
    /// toward one of them. Masks of 1 belong to `use_parity`. `limbs_*` functions are skipped:
    /// in limb-level kernels the mask is the idiom.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if m & 7 == 5 { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if m.mod_power_of_2(3) == 5 { .. }
    /// ```
    pub USE_MOD_POWER_OF_2,
    Deny,
    "masking with a constant one less than a power of 2 instead of using mod_power_of_2"
}

declare_lint_pass!(UseModPowerOf2 => [USE_MOD_POWER_OF_2]);

// Whether `expr` is in a function whose name marks it as exempt: the `mod_power_of_2`
// implementations themselves, and `limbs_*` kernels, where masks are the idiom.
fn in_exempt_fn<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> bool {
    let did = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
    let name = cx.tcx.item_name(did.to_def_id());
    let name = name.as_str();
    name.contains("mod_power_of_2") || name.starts_with("limbs_")
}

// If `v` is one less than a power of 2 and at least 3, returns the exponent.
fn mask_exponent(v: u128) -> Option<u32> {
    if v >= 3 && (v & v.wrapping_add(1)) == 0 {
        Some(v.count_ones())
    } else {
        None
    }
}

// If `e` is an integer literal that is one less than a power of 2 (and at least 3), returns the
// exponent.
fn literal_mask_exponent(e: &Expr<'_>) -> Option<u32> {
    if let ExprKind::Lit(lit) = e.kind
        && let LitKind::Int(v, _) = lit.node
    {
        mask_exponent(v.get())
    } else {
        None
    }
}

// If `e` is a path to a local constant whose definition is `Natural::const_from(<literal>)` or
// `Integer::const_from(<literal>)` with a mask literal, returns the exponent.
fn const_mask_exponent<'tcx>(cx: &LateContext<'tcx>, e: &Expr<'tcx>) -> Option<u32> {
    let ExprKind::Path(QPath::Resolved(_, path)) = &e.kind else {
        return None;
    };
    let Res::Def(DefKind::Const { .. }, did) = path.res else {
        return None;
    };
    let did = did.as_local()?;
    let body = cx.tcx.hir_body_owned_by(did);
    if let ExprKind::Call(callee, [arg]) = body.value.kind
        && let ExprKind::Path(qpath) = &callee.kind
        && let Some(seg) = crate::qpath_last_segment_name(qpath)
        && seg.starts_with("const_from")
    {
        literal_mask_exponent(arg)
    } else {
        None
    }
}

impl<'tcx> LateLintPass<'tcx> for UseModPowerOf2 {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || crate::in_test_code(cx, expr.span)
            || in_exempt_fn(cx, expr)
        {
            return;
        }
        let (l, r, assign) = match expr.kind {
            ExprKind::Binary(op, l, r) if op.node == BinOpKind::BitAnd => (l, r, false),
            ExprKind::AssignOp(op, l, r) if op.node == rustc_hir::AssignOpKind::BitAndAssign => {
                (l, r, true)
            }
            _ => return,
        };
        // For `&`, the mask may be on either side; for `&=`, only on the right.
        let candidates: &[(&Expr<'_>, &Expr<'_>)] =
            if assign { &[(r, l)] } else { &[(r, l), (l, r)] };
        for &(mask, value) in candidates {
            let mask = crate::peel_clone_and_borrows(mask);
            let value_ty = cx
                .typeck_results()
                .expr_ty(crate::peel_clone_and_borrows(value))
                .peel_refs();
            let pow = if matches!(value_ty.kind(), TyKind::Uint(_)) {
                literal_mask_exponent(mask)
            } else if crate::bignum_name(cx, value_ty) == Some("Natural") {
                const_mask_exponent(cx, mask)
            } else {
                None
            };
            if let Some(pow) = pow {
                let method = if assign {
                    "mod_power_of_2_assign"
                } else {
                    "mod_power_of_2"
                };
                clippy_utils::diagnostics::span_lint(
                    cx,
                    USE_MOD_POWER_OF_2,
                    expr.span,
                    format!(
                        "this mask is one less than a power of 2; use `{method}({pow})` instead"
                    ),
                );
                return;
            }
        }
    }
}
