// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::eq_expr_value;
use clippy_utils::source::snippet;
use rustc_errors::Applicability;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags constructing a [`Float`] from a primitive integer at exactly its own significant-bit
    /// precision and then discarding the ordering, like
    /// `Float::from_unsigned_prec(x, x.significant_bits()).0` or
    /// `Float::from_unsigned_prec(x, x.significant_bits().max(1)).0`.
    ///
    /// ### Why is this bad?
    ///
    /// Building a `Float` at its argument's significant-bit precision is an exact conversion --
    /// precisely what `Float::from` does -- and the discarded ordering is always `Equal`. Write
    /// `Float::from(x)`, which is shorter and also handles `x == 0` (where `significant_bits()` is
    /// 0, so `from_unsigned_prec(x, x.significant_bits())` panics on the zero precision).
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let f = Float::from_unsigned_prec(x, x.significant_bits().max(1)).0;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let f = Float::from(x);
    /// ```
    pub MANUAL_FLOAT_FROM_PRIMITIVE,
    Deny,
    "constructing a `Float` from a primitive integer at its significant-bit precision instead of \
    using `Float::from`"
}

declare_lint_pass!(ManualFloatFromPrimitive => [MANUAL_FLOAT_FROM_PRIMITIVE]);

// Whether `prec` is `x.significant_bits()`, optionally wrapped in `.max(1)`, for the same `x`.
fn is_min1_significant_bits<'tcx>(
    cx: &LateContext<'tcx>,
    prec: &'tcx Expr<'tcx>,
    x: &'tcx Expr<'tcx>,
) -> bool {
    let inner = match prec.kind {
        ExprKind::MethodCall(seg, recv, [arg], _)
            if seg.ident.name.as_str() == "max" && crate::literal_value(arg) == Some(1) =>
        {
            recv
        }
        _ => prec,
    };
    let ExprKind::MethodCall(seg, recv, [], _) = inner.kind else {
        return false;
    };
    seg.ident.name.as_str() == "significant_bits" && eq_expr_value(cx, recv, x)
}

impl<'tcx> LateLintPass<'tcx> for ManualFloatFromPrimitive {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // The whole expression is `<call>.0`, extracting the `Float` from the `(Float, Ordering)`
        // pair. Keeping the pair is a different value, so the `.0` is required.
        let ExprKind::Field(base, field) = expr.kind else {
            return;
        };
        if field.name.as_str() != "0" {
            return;
        }
        let ExprKind::Call(callee, [x, prec]) = base.kind else {
            return;
        };
        let ExprKind::Path(qpath) = &callee.kind else {
            return;
        };
        let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
            return;
        };
        if !matches!(
            cx.tcx.item_name(fn_did).as_str(),
            "from_unsigned_prec" | "from_signed_prec"
        ) {
            return;
        }
        // The extracted value (`<call>.0`) must be a `Float`.
        if crate::bignum_name(cx, cx.typeck_results().expr_ty(expr).peel_refs()) != Some("Float") {
            return;
        }
        if !is_min1_significant_bits(cx, prec, x) {
            return;
        }
        // Inside an `impl Float`, `Self` names the type and clippy's `use_self` prefers it, so
        // suggest `Self::from` there and `Float::from` elsewhere. The enclosing item's direct
        // parent is the `impl` exactly when the expression is in one of its methods (a nested `fn`
        // does not see `Self`).
        let owner = cx.tcx.hir_get_parent_item(expr.hir_id).to_def_id();
        let parent = cx.tcx.parent(owner);
        let ty_name = if matches!(cx.tcx.def_kind(parent), DefKind::Impl { .. })
            && crate::bignum_name(cx, cx.tcx.type_of(parent).instantiate_identity().peel_refs())
                == Some("Float")
        {
            "Self"
        } else {
            "Float"
        };
        span_lint_and_sugg(
            cx,
            MANUAL_FLOAT_FROM_PRIMITIVE,
            expr.span,
            "this is an exact conversion, which is what `Float::from` does",
            "use it directly",
            format!("{ty_name}::from({})", snippet(cx, x.span, "..")),
            Applicability::MachineApplicable,
        );
    }
}
