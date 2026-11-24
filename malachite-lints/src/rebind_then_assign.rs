// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use rustc_hir::def::Res;
use rustc_hir::{
    BindingMode, Block, ByRef, Expr, ExprKind, HirId, Mutability, PatKind, QPath, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_lint::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags rebinding a local as mutable only to mutate it in place on the very next line, like
    ///
    /// ```rust,ignore
    /// let mut t = t;
    /// t.abs_assign();
    /// ```
    ///
    /// when the `*_assign*` method has a by-value companion.
    ///
    /// ### Why is this bad?
    ///
    /// The rebind exists only to make the value mutable for the in-place call. Binding the result
    /// of the by-value variant does the same in one statement, and the by-value variants consume
    /// their receiver, so no allocation is lost. If the value is not mutated again afterwards,
    /// declaring the original binding `mut` and keeping the in-place call is the other fix.
    ///
    /// `use_assign_variant` does not flag the suggested `let mut t = t.abs();`: when the shadowed
    /// binding is immutable, the in-place form would need exactly this rebind.
    ///
    /// Tests and demos are exempt, since they call the in-place variants they exercise.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let mut t = t;
    /// t.abs_assign();
    /// let mut u = u;
    /// u.atan_prec_assign(w);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let mut t = t.abs();
    /// let mut u = u.atan_prec(w).0;
    /// ```
    pub REBIND_THEN_ASSIGN,
    Deny,
    "rebinding a local as mutable just to call an in-place `*_assign*` method on it"
}

declare_lint_pass!(RebindThenAssign => [REBIND_THEN_ASSIGN]);

impl<'tcx> LateLintPass<'tcx> for RebindThenAssign {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        for pair in block.stmts.windows(2) {
            let [s1, s2] = pair else {
                continue;
            };
            if s1.span.from_expansion() || crate::in_cross_check_code(cx, s1.span) {
                continue;
            }
            // s1: `let mut NAME = LOCAL;`, a by-value mutable binding initialized by a bare local.
            let StmtKind::Let(local) = s1.kind else {
                continue;
            };
            if local.els.is_some() || local.ty.is_some() {
                continue;
            }
            let PatKind::Binding(BindingMode(ByRef::No, Mutability::Mut), name_hir, name, None) =
                local.pat.kind
            else {
                continue;
            };
            let Some(init) = local.init else {
                continue;
            };
            if path_to_local(init).is_none() {
                continue;
            }
            // s2: `NAME.<base>_assign<..>(..);`, with NAME itself as the receiver.
            let (StmtKind::Semi(e) | StmtKind::Expr(e)) = s2.kind else {
                continue;
            };
            let ExprKind::MethodCall(seg, recv, _, _) = e.kind else {
                continue;
            };
            let method = seg.ident.as_str();
            let Some((head, tail)) = method.split_once("_assign") else {
                continue;
            };
            if path_to_local(recv) != Some(name_hir) {
                continue;
            }
            let base = crate::strip_variant_suffixes(head);
            let ty = cx.typeck_results().expr_ty(init);
            if !tail.chars().all(|c| c == '_' || c.is_ascii_lowercase())
                || !by_value_variant_exists(cx, ty, base)
            {
                continue;
            }
            // The by-value variant's own defining delegation (`fn abs(self) -> Self { let mut x =
            // self; x.abs_assign(); x }`) is exempt: the suggested rewrite would recurse.
            let owner_did = cx.tcx.hir_get_parent_item(local.hir_id).to_def_id();
            if matches!(
                cx.tcx.def_kind(owner_did),
                rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::AssocFn
            ) && crate::strip_variant_suffixes(cx.tcx.item_name(owner_did).as_str()) == base
            {
                continue;
            }
            // An in-place method that reports something (an `Ordering`, say) has a by-value
            // companion that returns it alongside the value.
            let projection = if cx.typeck_results().expr_ty(e).is_unit() {
                ""
            } else {
                ".0"
            };
            // Declaring the original binding `mut` instead is an option only for a same-name
            // rebind; a renaming one may copy a value that is still used afterwards.
            let init_name = snippet(cx, init.span, "..");
            let alternative = if init_name == name.as_str() {
                ", or declare the original binding `mut` and drop the rebind"
            } else {
                ""
            };
            span_lint_and_help(
                cx,
                REBIND_THEN_ASSIGN,
                s1.span.to(s2.span),
                format!("`{name}` is rebound as mutable only to call `{method}` on it"),
                None,
                format!(
                    "bind the result of the by-value variant instead (`let mut {name} = \
                     {init_name}.{base}(..){projection};`){alternative}"
                ),
            );
        }
    }
}

// The local that `e` names, if it is a bare path to one.
fn path_to_local(e: &Expr<'_>) -> Option<HirId> {
    if let ExprKind::Path(QPath::Resolved(None, path)) = e.kind
        && let Res::Local(id) = path.res
    {
        Some(id)
    } else {
        None
    }
}

// Whether a by-value companion of the function family `base` exists for `ty`: for a bignum, an
// inherent `{base}`/`{base}_val_ref` or a `{CamelCase(base)}` trait in `malachite_base`; for any
// other type, an implementation of that trait.
fn by_value_variant_exists<'tcx>(
    cx: &LateContext<'tcx>,
    ty: rustc_middle::ty::Ty<'tcx>,
    base: &str,
) -> bool {
    let bignum = crate::bignum_adt_did(cx, ty);
    if let Some(did) = bignum
        && (crate::has_inherent_fn(cx, did, base)
            || crate::has_inherent_fn(cx, did, &format!("{base}_val_ref")))
    {
        return true;
    }
    let trait_name = crate::camel_case(base);
    [
        "arithmetic",
        "modular",
        "number_theory",
        "exponential",
        "trigonometric",
        "hyperbolic",
        "linear_algebra",
        "elliptic",
        "gamma",
        "factorization",
        "logic",
    ]
    .iter()
    .any(|module| {
        let path = format!("malachite_base::num::{module}::traits::{trait_name}");
        if bignum.is_some() {
            !clippy_utils::paths::lookup_path_str(
                cx.tcx,
                clippy_utils::paths::PathNS::Type,
                &path,
            )
            .is_empty()
        } else {
            crate::implements_trait_by_path(cx, ty, &path)
        }
    })
}
