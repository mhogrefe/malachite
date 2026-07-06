// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::macros::{find_assert_eq_args, root_macro_call_first_node};
use clippy_utils::path_to_local_with_projections;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Block, Expr, ExprKind, HirId, PatKind, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags the pattern of binding a `(value, Ordering)` result and then asserting the ordering
    /// is `Equal`:
    ///
    /// ```rust,ignore
    /// let (x, o) = a.from_natural_prec(p);
    /// assert_eq!(o, Equal);
    /// ```
    ///
    /// when the called function has a `_round` sibling that accepts a rounding mode, so the same
    /// intent is expressed by passing `Exact`.
    ///
    /// ### Why is this bad?
    ///
    /// The `_round(.., Exact)` variant *is* the assertion: `Exact` panics if the result is not
    /// exactly representable, so the separate `assert_eq!` is redundant and the ordering binding
    /// only exists to be checked. Passing `Exact` also tends to be faster: the default `Nearest`
    /// generally does more work than the directed modes (it must decide the tie), so a call that
    /// is known to be exact should not pay for round-to-nearest.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let (x, o) = a.from_natural_prec(p);
    /// assert_eq!(o, Equal);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let x = a.from_natural_prec_round(p, Exact).0;
    /// ```
    pub ASSERT_ORDERING_EQUAL_PREFER_EXACT,
    Deny,
    "asserting a returned `Ordering` is `Equal` instead of using the `_round(.., Exact)` variant"
}

declare_lint_pass!(AssertOrderingEqualPreferExact => [ASSERT_ORDERING_EQUAL_PREFER_EXACT]);

// Whether `ty` is `core::cmp::Ordering`.
fn is_ordering_ty(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    if let rustc_middle::ty::Adt(adt, _) = ty.kind() {
        let path = cx.get_def_path(adt.did());
        path.len() == 3
            && path[0].as_str() == "core"
            && path[1].as_str() == "cmp"
            && path[2].as_str() == "Ordering"
    } else {
        false
    }
}

// Whether `e` is `Ordering::Equal` (a path to the `Equal` variant of `core::cmp::Ordering`).
fn is_ordering_equal(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let ExprKind::Path(qpath) = &e.kind else {
        return false;
    };
    let res = cx.qpath_res(qpath, e.hir_id);
    let Res::Def(DefKind::Ctor(..) | DefKind::Variant, did) = res else {
        return false;
    };
    cx.tcx.item_name(did).as_str() == "Equal" && is_ordering_ty(cx, cx.typeck_results().expr_ty(e))
}

// If `init` is a call to an inherent associated function of a Malachite bignum type, returns the
// function's name and the type's `DefId`.
fn callee_inherent_bignum_fn<'tcx>(
    cx: &LateContext<'tcx>,
    init: &Expr<'tcx>,
) -> Option<(String, rustc_hir::def_id::DefId)> {
    let def_id = match init.kind {
        ExprKind::MethodCall(..) => cx.typeck_results().type_dependent_def_id(init.hir_id)?,
        ExprKind::Call(callee, _) => {
            let ExprKind::Path(qpath) = &callee.kind else {
                return None;
            };
            let Res::Def(DefKind::AssocFn, did) = cx.qpath_res(qpath, callee.hir_id) else {
                return None;
            };
            did
        }
        _ => return None,
    };
    // Inherent impls only; a trait method has no bignum-specific `_round` sibling to point to.
    let impl_did = cx.tcx.inherent_impl_of_assoc(def_id)?;
    let self_ty = cx.tcx.type_of(impl_did).instantiate_identity();
    let adt_did = crate::bignum_adt_did(cx, self_ty)?;
    Some((cx.tcx.item_name(def_id).to_string(), adt_did))
}

// Whether `stmt_expr` is `assert_eq!`/`debug_assert_eq!` comparing the local `o_hir` with
// `Ordering::Equal` (in either argument order).
fn is_assert_eq_local_equal<'tcx>(
    cx: &LateContext<'tcx>,
    stmt_expr: &'tcx Expr<'tcx>,
    o_hir: HirId,
) -> bool {
    let Some(mac) = root_macro_call_first_node(cx, stmt_expr) else {
        return false;
    };
    let name = cx.tcx.item_name(mac.def_id);
    if name.as_str() != "assert_eq" && name.as_str() != "debug_assert_eq" {
        return false;
    }
    let Some((a, b, _)) = find_assert_eq_args(cx, stmt_expr, mac.expn) else {
        return false;
    };
    let is_o = |e: &Expr<'_>| path_to_local_with_projections(e) == Some(o_hir);
    (is_o(a) && is_ordering_equal(cx, b)) || (is_o(b) && is_ordering_equal(cx, a))
}

impl<'tcx> LateLintPass<'tcx> for AssertOrderingEqualPreferExact {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        for pair in block.stmts.windows(2) {
            let [s1, s2] = pair else {
                continue;
            };
            if s1.span.from_expansion() || crate::in_test_code(cx, s1.span) {
                continue;
            }
            // s1: `let (_x, o) = <call>;`, with `o` an `Ordering` binding.
            let StmtKind::Let(local) = s1.kind else {
                continue;
            };
            let Some(init) = local.init else {
                continue;
            };
            let PatKind::Tuple(pats, dot_dot) = local.pat.kind else {
                continue;
            };
            if dot_dot.as_opt_usize().is_some() || pats.len() != 2 {
                continue;
            }
            let PatKind::Binding(_, o_hir, _, None) = pats[1].kind else {
                continue;
            };
            if !is_ordering_ty(cx, cx.typeck_results().node_type(o_hir)) {
                continue;
            }
            // The initializer is an inherent bignum function with a `_round` sibling, and is not
            // itself the `_round` variant.
            let Some((name, adt_did)) = callee_inherent_bignum_fn(cx, init) else {
                continue;
            };
            if name.contains("round")
                || !crate::has_inherent_fn(cx, adt_did, &format!("{name}_round"))
            {
                continue;
            }
            // s2: `assert_eq!(o, Equal);`.
            let (StmtKind::Semi(e) | StmtKind::Expr(e)) = s2.kind else {
                continue;
            };
            if !is_assert_eq_local_equal(cx, e, o_hir) {
                continue;
            }
            span_lint_and_help(
                cx,
                ASSERT_ORDERING_EQUAL_PREFER_EXACT,
                s1.span,
                format!(
                    "asserting the `Ordering` from `{name}` is `Equal` instead of demanding \
                     exactness",
                ),
                None,
                format!(
                    "call `{name}_round(.., Exact)` and take `.0`; `Exact` panics if inexact and \
                     avoids the cost of round-to-nearest",
                ),
            );
        }
    }
}
