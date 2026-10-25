// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::LitKind;
use rustc_hir::def::Res;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags computing the determinant of a matrix only to find its sign, where the matrix type
    /// implements `DeterminantSign`: `a.determinant().sign()`, and the determinant compared with
    /// zero, whether by an operator (`a.determinant() > 0u32`, `a.determinant() == Integer::ZERO`,
    /// with the zero on either side) or by `cmp`, `partial_cmp`, `eq`, `ne`, `lt`, `le`, `gt`, or
    /// `ge`. The method-call form `a.determinant()` and the path form `Determinant::determinant(a)`
    /// are both caught.
    ///
    /// ### Why is this bad?
    ///
    /// The determinant of an integer matrix can be enormous, and computing it exactly is costly.
    /// `determinant_sign` usually finds the sign without computing the determinant at all,
    /// certifying it with a preconditioned product instead; it falls back to the determinant only
    /// when the elements are small or the matrix is singular or ill-conditioned. Comparing the
    /// determinant with zero asks the same question: `a.determinant() > 0u32` is
    /// `a.determinant_sign() == Greater`, and `a.determinant() == 0u32` is
    /// `a.determinant_sign() == Equal`.
    ///
    /// A determinant that is stored and used for more than its sign is not flagged, and neither is
    /// one bound to a variable and compared with zero later. Test code is exempt, since tests
    /// cross-check `determinant_sign` against the sign of the determinant on purpose.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if a.determinant() > 0u32 { .. }
    /// let s = a.determinant().sign();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if a.determinant_sign() == Greater { .. }
    /// let s = a.determinant_sign();
    /// ```
    pub USE_DETERMINANT_SIGN,
    Deny,
    "computing a determinant only for its sign instead of using `determinant_sign`"
}

declare_lint_pass!(UseDeterminantSign => [USE_DETERMINANT_SIGN]);

// Whether `fn_did` is `malachite_base::num::arithmetic::traits::Determinant::determinant`.
fn is_determinant_fn(cx: &LateContext<'_>, fn_did: rustc_hir::def_id::DefId) -> bool {
    let Some(trait_did) = cx.tcx.trait_of_assoc(fn_did) else {
        return false;
    };
    let path = cx.get_def_path(trait_did);
    path.len() == 5
        && path[0].as_str() == "malachite_base"
        && path[1].as_str() == "num"
        && path[2].as_str() == "arithmetic"
        && path[3].as_str() == "traits"
        && path[4].as_str() == "Determinant"
}

// Whether `e`, after peeling `&` and `.clone()`, is a call of `Determinant::determinant` on a
// matrix whose type implements `DeterminantSign`. Other lints that would flag the same comparison
// with zero, such as `use_sign` and `compare_with_primitive`, defer to this one.
pub(crate) fn is_determinant_call<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> bool {
    let e = crate::peel_clone_and_borrows(e);
    let (fn_did, matrix) = match e.kind {
        ExprKind::MethodCall(_, receiver, [], _) => {
            let Some(fn_did) = cx.typeck_results().type_dependent_def_id(e.hir_id) else {
                return false;
            };
            (fn_did, receiver)
        }
        ExprKind::Call(callee, [arg]) => {
            let ExprKind::Path(ref qpath) = callee.kind else {
                return false;
            };
            let Res::Def(_, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
                return false;
            };
            (fn_did, arg)
        }
        _ => return false,
    };
    is_determinant_fn(cx, fn_did)
        && crate::implements_trait_by_path(
            cx,
            cx.typeck_results().expr_ty(matrix).peel_refs(),
            "malachite_base::num::arithmetic::traits::DeterminantSign",
        )
}

// Whether `e`, after peeling `&`, is the integer literal 0 or an associated constant named `ZERO`.
fn is_zero<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> bool {
    if let ExprKind::Lit(lit) = e.peel_borrows().kind
        && let LitKind::Int(v, _) = lit.node
    {
        v.get() == 0
    } else {
        crate::is_zero_assoc_const(cx, e)
    }
}

impl<'tcx> LateLintPass<'tcx> for UseDeterminantSign {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_cross_check_code(cx, expr.span) {
            return;
        }
        let (message, help) = match expr.kind {
            ExprKind::MethodCall(seg, receiver, [], _)
                if seg.ident.name.as_str() == "sign" && is_determinant_call(cx, receiver) =>
            {
                (
                    "this computes a determinant only for its sign",
                    "`a.determinant().sign()` is `a.determinant_sign()`, which usually finds the \
                     sign without computing the determinant",
                )
            }
            ExprKind::Binary(op, lhs, rhs)
                if matches!(
                    op.node,
                    BinOpKind::Eq
                        | BinOpKind::Ne
                        | BinOpKind::Lt
                        | BinOpKind::Le
                        | BinOpKind::Gt
                        | BinOpKind::Ge
                ) && ((is_determinant_call(cx, lhs) && is_zero(cx, rhs))
                    || (is_zero(cx, lhs) && is_determinant_call(cx, rhs))) =>
            {
                (
                    "this compares a determinant with zero",
                    "compare `a.determinant_sign()` with `Less`, `Equal`, or `Greater` instead; it \
                     usually finds the sign without computing the determinant",
                )
            }
            ExprKind::MethodCall(seg, receiver, [arg], _)
                if matches!(
                    seg.ident.name.as_str(),
                    "cmp" | "partial_cmp" | "eq" | "ne" | "lt" | "le" | "gt" | "ge"
                ) && ((is_determinant_call(cx, receiver) && is_zero(cx, arg))
                    || (is_zero(cx, receiver) && is_determinant_call(cx, arg))) =>
            {
                (
                    "this compares a determinant with zero",
                    "use `a.determinant_sign()`, comparing it with `Less`, `Equal`, or `Greater` \
                     if needed; it usually finds the sign without computing the determinant",
                )
            }
            _ => return,
        };
        span_lint_and_help(cx, USE_DETERMINANT_SIGN, expr.span, message, None, help);
    }
}
