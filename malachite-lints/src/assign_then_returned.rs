// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::ty::is_copy;
use rustc_hir::def::Res;
use rustc_hir::{Block, Expr, ExprKind, HirId, Path, QPath, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a block ending in an in-place `*_assign*` call on a variable immediately followed by
    /// that variable as the block's tail expression, like
    ///
    /// ```rust,ignore
    /// b.floor_sqrt_assign();
    /// b
    /// ```
    ///
    /// ### Why is this bad?
    ///
    /// The by-value variant produces the block's value directly: `b.floor_sqrt()`. The in-place
    /// call followed by the bare receiver says the same thing in two statements.
    ///
    /// This completes the family: `use_assign_variant` prefers the in-place form when a persisted
    /// variable is reassigned its own result, `assign_then_consumed_once` prefers a chain when a
    /// fresh binding is mutated once and moved once, and this lint prefers a by-value call when
    /// the mutation's only purpose is to produce the block's value.
    ///
    /// Only non-`Copy` receivers are flagged: for a `Copy` type the tail expression copies the
    /// mutated local, which may still be read afterwards, and the rewrite would change what those
    /// later reads see. Functions whose own name is in the same family as the assign method are
    /// also skipped, since the by-value variants are themselves implemented by exactly this
    /// shape.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// b.floor_sqrt_assign();
    /// b
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// b.floor_sqrt()
    /// ```
    pub ASSIGN_THEN_RETURNED,
    Deny,
    "an in-place `*_assign*` call followed by returning the receiver, instead of the by-value \
    variant"
}

declare_lint_pass!(AssignThenReturned => [ASSIGN_THEN_RETURNED]);

// The local a whole-local path expression refers to. Unlike
// `path_to_local_with_projections`, a field or index projection does not count: the receiver and
// the tail must be the same complete place for the rewrite to apply.
fn whole_local(e: &Expr<'_>) -> Option<HirId> {
    match e.kind {
        ExprKind::Path(QPath::Resolved(
            _,
            Path {
                res: Res::Local(id),
                ..
            },
        )) => Some(*id),
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for AssignThenReturned {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        let Some(tail) = block.expr else {
            return;
        };
        let Some(last) = block.stmts.last() else {
            return;
        };
        if last.span.from_expansion() || tail.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, last.span) {
            return;
        }
        let (StmtKind::Semi(e) | StmtKind::Expr(e)) = last.kind else {
            return;
        };
        let ExprKind::MethodCall(seg, recv, _, _) = e.kind else {
            return;
        };
        let name = seg.ident.as_str();
        if !name.contains("_assign") {
            return;
        }
        // The receiver must be a whole local, and the tail must be the same local.
        let Some(recv_local) = whole_local(recv) else {
            return;
        };
        if whole_local(tail) != Some(recv_local) {
            return;
        }
        if is_copy(cx, cx.typeck_results().expr_ty(tail)) {
            return;
        }
        // The by-value variants of the operations are themselves implemented by this very shape:
        // `fn gcd(self, mut other: Natural) -> Natural { other.gcd_assign(self); other }`. Inside
        // a function of the same family as the assign method, the rewrite would be a call to the
        // function being defined. Both names are normalized by dropping `_assign` and the
        // `_val`/`_ref` variant suffixes, and prefix matching covers helpers like
        // `add_assign_limb` inside `add`.
        let owner = cx.tcx.hir_enclosing_body_owner(e.hir_id);
        if let Some(f) = cx.tcx.opt_item_name(owner.to_def_id()) {
            let f = f.as_str();
            let f_family = crate::strip_variant_suffixes(f);
            let method_family_owned = name.replacen("_assign", "", 1);
            let method_family = crate::strip_variant_suffixes(&method_family_owned);
            if method_family.starts_with(f_family) || f_family.starts_with(method_family) {
                return;
            }
        }
        let msg = if let Some(base) = name.strip_suffix("_assign") {
            format!(
                "use `{base}()` as the block's value instead of `{name}` followed by the \
                receiver"
            )
        } else {
            format!(
                "use the by-value variant of `{name}` as the block's value instead of the \
                in-place call followed by the receiver"
            )
        };
        span_lint_and_help(
            cx,
            ASSIGN_THEN_RETURNED,
            last.span.to(tail.span),
            msg,
            None,
            "the by-value call produces the value directly",
        );
    }
}
