// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::visitors::for_each_expr;
use core::ops::ControlFlow;
use rustc_hir::def::Res;
use rustc_hir::{
    BindingMode, ByRef, Expr, ExprKind, HirId, LetStmt, Mutability, Node, Pat, PatKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_lint::{declare_lint, declare_lint_pass};
use rustc_middle::ty::Ty;

declare_lint! {
    /// ### What it does
    ///
    /// Flags taking a half of a concrete unsigned value with `upper_half()`, `lower_half()`, or
    /// `split_in_half()`, only to convert the half straight back to the original type: either
    /// directly (`u128::from(x.upper_half())`) or through a binding whose every use is such a
    /// conversion (`let (hi, lo) = x.split_in_half();` followed only by `u128::from(hi)` and
    /// `u128::from(lo)`).
    ///
    /// ### Why is this bad?
    ///
    /// The narrowing and widening cancel out; the half can stay in the wide type, which is shorter
    /// and avoids the conversions. In malachite-base, `wide_upper_half`, `wide_lower_half`, and
    /// `wide_split_in_half` return the halves in the wide type; elsewhere, `x >> W` and
    /// `x.mod_power_of_2(W)` do, where `W` is half the width.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let (a1, a0) = a.split_in_half();
    /// let s = u128::from(a0) * u128::from(a1);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let (a1, a0) = wide_split_in_half(a);
    /// let s = a0 * a1;
    /// ```
    pub WIDENED_HALF,
    Deny,
    "taking a half of a value only to convert it back to the value's type"
}

declare_lint_pass!(WidenedHalf => [WIDENED_HALF]);

const HELP: &str = "keep the half in the wide type: in malachite-base, use `wide_upper_half`, \
`wide_lower_half`, or `wide_split_in_half`; elsewhere, shift right by, or reduce mod 2 to the \
power of, half the width";

// If `e` is a call of `upper_half`, `lower_half`, or `split_in_half` from malachite-base's
// `SplitInHalf` on a concrete unsigned type other than `usize`, returns the method name and the
// receiver's type.
fn half_call<'tcx>(cx: &LateContext<'tcx>, e: &Expr<'_>) -> Option<(&'static str, Ty<'tcx>)> {
    let ExprKind::MethodCall(seg, recv, [], _) = e.kind else {
        return None;
    };
    let name = match seg.ident.name.as_str() {
        "upper_half" => "upper_half",
        "lower_half" => "lower_half",
        "split_in_half" => "split_in_half",
        _ => return None,
    };
    let fn_did = cx.typeck_results().type_dependent_def_id(e.hir_id)?;
    let trait_did = cx.tcx.trait_of_assoc(fn_did)?;
    let path = cx.get_def_path(trait_did);
    let paths: Vec<&str> = path.iter().map(|s| s.as_str()).collect();
    if paths
        != [
            "malachite_base",
            "num",
            "conversion",
            "traits",
            "SplitInHalf",
        ]
    {
        return None;
    }
    let ty = cx.typeck_results().expr_ty(recv).peel_refs();
    match ty.kind() {
        rustc_middle::ty::Uint(u) if u.bit_width().is_some() => Some((name, ty)),
        _ => None,
    }
}

// Whether the value of `e` is immediately converted to `wide`, by `T::from(e)` or `e.into()`.
fn is_widened<'tcx>(cx: &LateContext<'tcx>, e: &Expr<'_>, wide: Ty<'tcx>) -> bool {
    let Node::Expr(parent) = cx.tcx.parent_hir_node(e.hir_id) else {
        return false;
    };
    let converts = match parent.kind {
        ExprKind::Call(callee, [arg]) if arg.hir_id == e.hir_id => {
            matches!(&callee.kind, ExprKind::Path(qpath)
                if crate::qpath_last_segment_name(qpath) == Some("from"))
        }
        ExprKind::MethodCall(seg, recv, [], _) if recv.hir_id == e.hir_id => {
            seg.ident.name.as_str() == "into"
        }
        _ => false,
    };
    converts && cx.typeck_results().expr_ty(parent) == wide
}

// The bindings of an immutable by-value binding pattern, or of a tuple of such patterns and
// wildcards; `None` for any other pattern.
fn simple_bindings(pat: &Pat<'_>) -> Option<Vec<HirId>> {
    match pat.kind {
        PatKind::Binding(BindingMode(ByRef::No, Mutability::Not), id, _, None) => Some(vec![id]),
        PatKind::Tuple(pats, dotdot) if dotdot.as_opt_usize().is_none() => {
            let mut ids = Vec::new();
            for p in pats {
                match p.kind {
                    PatKind::Wild => {}
                    PatKind::Binding(BindingMode(ByRef::No, Mutability::Not), id, _, None) => {
                        ids.push(id);
                    }
                    _ => return None,
                }
            }
            Some(ids)
        }
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for WidenedHalf {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        let Some((name, ty)) = half_call(cx, expr) else {
            return;
        };
        if name == "split_in_half" || !is_widened(cx, expr, ty) {
            return;
        }
        if crate::in_cross_check_code(cx, expr.span) {
            return;
        }
        let Node::Expr(parent) = cx.tcx.parent_hir_node(expr.hir_id) else {
            return;
        };
        span_lint_and_help(
            cx,
            WIDENED_HALF,
            parent.span,
            format!("`{name}()` is converted straight back to `{ty}`"),
            None,
            HELP,
        );
    }

    fn check_local(&mut self, cx: &LateContext<'tcx>, local: &'tcx LetStmt<'tcx>) {
        if local.span.from_expansion() {
            return;
        }
        let Some(init) = local.init else {
            return;
        };
        let Some((name, ty)) = half_call(cx, init) else {
            return;
        };
        let Some(ids) = simple_bindings(local.pat) else {
            return;
        };
        if ids.is_empty() {
            return;
        }
        let owner = cx.tcx.hir_enclosing_body_owner(local.hir_id);
        let body = cx.tcx.hir_body_owned_by(owner);
        for id in ids {
            let mut uses = 0usize;
            let all_widened = for_each_expr(cx.tcx, body.value, |e| {
                if let ExprKind::Path(qpath) = &e.kind
                    && cx.qpath_res(qpath, e.hir_id) == Res::Local(id)
                {
                    if !is_widened(cx, e, ty) {
                        return ControlFlow::Break(());
                    }
                    uses += 1;
                }
                ControlFlow::Continue(())
            })
            .is_none();
            if !all_widened || uses == 0 {
                return;
            }
        }
        if crate::in_cross_check_code(cx, local.span) {
            return;
        }
        span_lint_and_help(
            cx,
            WIDENED_HALF,
            local.span,
            format!("every half taken by `{name}()` is converted straight back to `{ty}`"),
            None,
            HELP,
        );
    }
}
