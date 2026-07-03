// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags calls like `x.foo_prec_round(.., Nearest)` when a `foo_prec` shorthand exists.
    ///
    /// ### Why is this bad?
    ///
    /// The `*_prec*` shorthands are exactly the `*_prec_round*` functions with `Nearest`; house
    /// style is to use the shorthand.
    ///
    /// The lint exempts the shorthand's own defining delegation and everything inside trait
    /// impls, which delegate via the explicit form by convention.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let (f, o) = x.exp_prec_round(100, Nearest);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let (f, o) = x.exp_prec(100);
    /// ```
    pub REDUNDANT_NEAREST,
    Deny,
    "calling `*_prec_round*(.., Nearest)` where the `*_prec*` shorthand exists"
}

declare_lint_pass!(RedundantNearest => [REDUNDANT_NEAREST]);

// Whether `e` is a path expression denoting `malachite_base`'s `RoundingMode::Nearest`.
fn is_nearest(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let ExprKind::Path(qpath) = &e.kind else {
        return false;
    };
    let Some(did) = cx.qpath_res(qpath, e.hir_id).opt_def_id() else {
        return false;
    };
    let variant_did = if matches!(cx.tcx.def_kind(did), DefKind::Ctor(..)) {
        cx.tcx.parent(did)
    } else {
        did
    };
    cx.tcx.def_path_str(variant_did) == "malachite_base::rounding_modes::RoundingMode::Nearest"
}

impl<'tcx> LateLintPass<'tcx> for RedundantNearest {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        // Match both method calls (`x.foo_prec_round(..)`) and associated-function calls
        // (`Float::foo_rational_prec_round(..)`), extracting the function name, the self type
        // whose inherent impls define the shorthand, and the last argument.
        let (fn_name, self_ty_did, last_arg) = match expr.kind {
            ExprKind::MethodCall(seg, receiver, args, _) => {
                if !seg.ident.name.as_str().contains("_prec_round") {
                    return;
                }
                let recv_ty = cx.typeck_results().expr_ty_adjusted(receiver).peel_refs();
                let ty::Adt(adt, _) = recv_ty.kind() else {
                    return;
                };
                let Some(last_arg) = args.last() else {
                    return;
                };
                (seg.ident.name, adt.did(), last_arg)
            }
            ExprKind::Call(callee, args) => {
                let ExprKind::Path(qpath) = &callee.kind else {
                    return;
                };
                let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
                    return;
                };
                let name = cx.tcx.item_name(fn_did);
                if !name.as_str().contains("_prec_round") {
                    return;
                }
                let impl_did = cx.tcx.parent(fn_did);
                if !matches!(cx.tcx.def_kind(impl_did), DefKind::Impl { .. }) {
                    return;
                }
                let self_ty = cx.tcx.type_of(impl_did).instantiate_identity();
                let ty::Adt(adt, _) = self_ty.kind() else {
                    return;
                };
                let Some(last_arg) = args.last() else {
                    return;
                };
                (name, adt.did(), last_arg)
            }
            _ => return,
        };
        if !is_nearest(cx, last_arg) {
            return;
        }
        let shorthand = fn_name.as_str().replacen("_prec_round", "_prec", 1);
        if !crate::has_inherent_fn(cx, self_ty_did, &shorthand) {
            return;
        }
        // Exemptions. A shorthand's own definition delegates to a `_round` variant -- possibly of
        // a different by-value/by-reference variant, as in `exp_rational_prec` calling
        // `Self::exp_rational_prec_round_ref(&x, prec, Nearest)` -- so the name comparison ignores
        // `_val`/`_ref` suffixes. Trait impls (operators, `*Assign`, `LogBase`, etc.) delegate via
        // the explicit form by convention. Tests, demos, and test utilities exercise both
        // spellings on purpose.
        let owner_did = cx.tcx.hir_get_parent_item(expr.hir_id).to_def_id();
        if matches!(cx.tcx.def_kind(owner_did), DefKind::Fn | DefKind::AssocFn) {
            if crate::strip_variant_suffixes(cx.tcx.item_name(owner_did).as_str())
                == crate::strip_variant_suffixes(&shorthand)
            {
                return;
            }
            let parent = cx.tcx.parent(owner_did);
            if matches!(
                cx.tcx.def_kind(parent),
                DefKind::Impl { of_trait: true } | DefKind::Trait
            ) {
                return;
            }
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        span_lint(
            cx,
            REDUNDANT_NEAREST,
            expr.span,
            format!("use `{shorthand}` instead of `{fn_name}(.., Nearest)`"),
        );
    }
}
