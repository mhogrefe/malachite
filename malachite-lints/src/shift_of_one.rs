// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use clippy_utils::{expr_or_init, get_parent_expr};
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags shifting `1` or `T::ONE` left by an amount, where a named `malachite-base` helper says
    /// the same thing more clearly:
    ///
    /// - `(1 << n) - 1` builds a mask of the low `n` bits — use `T::low_mask(n)`.
    /// - `x & (1 << n) != 0` / `== 0` tests bit `n` — use `x.get_bit(n)` / `!x.get_bit(n)`.
    /// - any other `1 << n` is the value two-to-the-`n` — use `T::power_of_2(n)`.
    ///
    /// `T` may be a primitive integer or one of the Malachite bignum types with a `PowerOf2`
    /// implementation (`Natural`, `Integer`, `Rational`, and `GaussianInteger`); the mask and bit
    /// forms are suggested only where `LowMask` and `BitAccess` exist (the primitives, `Natural`,
    /// and `Integer`). The bodies of `power_of_2` functions themselves, which define the operation
    /// as a shift, are exempt.
    ///
    /// ### Why is this bad?
    ///
    /// The raw shift obscures the intent (a mask, a bit test, or a power of two) and re-derives
    /// what `LowMask`, `BitAccess`, and `PowerOf2` already provide. The named helpers read at the
    /// level of the operation rather than its bit-twiddling implementation. For a bignum, `ONE <<
    /// n` also allocates a one-limb value and then a shifted copy of it, where `power_of_2` builds
    /// the result directly.
    ///
    /// A *constant* shift amount is left alone: `1 << 70` folds at compile time, but
    /// `power_of_2(70)` is an ordinary runtime call, so the rewrite would only pessimize it. Const
    /// contexts (a `const`/`static` item or a `const fn`) are likewise skipped — the helpers are
    /// not const fns, so the suggestion would not even compile there.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let mask = (Limb::ONE << k) - 1;
    /// let set = x & (Limb::ONE << k) != 0;
    /// let p = Limb::ONE << k;
    /// let q = Integer::ONE << k;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let mask = Limb::low_mask(k);
    /// let set = x.get_bit(k);
    /// let p = Limb::power_of_2(k);
    /// let q = Integer::power_of_2(k);
    /// ```
    pub SHIFT_OF_ONE,
    Deny,
    "`1 << n` bit tricks instead of `low_mask`, `get_bit`, or `power_of_2`"
}

declare_lint_pass!(ShiftOfOne => [SHIFT_OF_ONE]);

// Whether `e` is a compile-time-constant shift amount: an integer literal, a `const`/
// associated-const/const-parameter path, or arithmetic over such. Immutable locals are followed to
// their initializers (via `expr_or_init`), so a `let r = A::LOG_WIDTH - B::LOG_WIDTH;` counts too.
// A plain syntactic check is used rather than `ConstEvalCtxt` because the latter cannot evaluate a
// generic associated const like `B::LOG_WIDTH`, which nonetheless folds at each instantiation.
fn is_const_amount(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let e = expr_or_init(cx, e);
    match e.kind {
        ExprKind::Lit(_) => true,
        ExprKind::Path(ref qpath) => matches!(
            cx.qpath_res(qpath, e.hir_id),
            Res::Def(
                DefKind::Const { .. } | DefKind::AssocConst { .. } | DefKind::ConstParam,
                _,
            )
        ),
        ExprKind::Unary(_, inner) | ExprKind::Cast(inner, _) => is_const_amount(cx, inner),
        ExprKind::Binary(_, l, r) => is_const_amount(cx, l) && is_const_amount(cx, r),
        _ => false,
    }
}

// Whether `ty` is a type the lint rewrites shifts of one for: a primitive integer or a Malachite
// bignum with a `PowerOf2` implementation. Returns whether the type also has `LowMask` and
// `BitAccess`, so that the mask and bit-test forms apply (the primitives, `Natural`, and
// `Integer`; `Rational` and `GaussianInteger` have only `PowerOf2`).
fn shift_target<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<bool> {
    if ty.is_integral() {
        return Some(true);
    }
    match crate::bignum_name(cx, ty)? {
        "Natural" | "Integer" => Some(true),
        "Rational" | "GaussianInteger" => Some(false),
        _ => None,
    }
}

// Whether `e` is inside a function named `power_of_2`: the `PowerOf2` implementations define the
// operation as a shift, and must not be told to call themselves.
fn in_power_of_2_fn(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let owner = cx.tcx.hir_enclosing_body_owner(e.hir_id);
    cx.tcx
        .opt_item_name(owner.to_def_id())
        .is_some_and(|name| name.as_str() == "power_of_2")
}

// If `e` is `<one> << <n>` (`<one>` being the literal 1 or a `T::ONE` constant) with a primitive
// integer or bignum type (see `shift_target`), and neither the shift amount `<n>` nor the
// surrounding context is constant, returns `(<one>, <n>, has_mask_and_bit)`.
//
// Constant shifts are excluded: the raw shift folds at compile time (and in a const context the
// suggested `power_of_2`/`low_mask`/`get_bit` cannot even be called, as they are not const fns),
// whereas the helpers are ordinary runtime calls, so the rewrite would only pessimize them.
fn shift_of_one<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>, bool)> {
    let ExprKind::Binary(op, lhs, rhs) = e.kind else {
        return None;
    };
    if op.node != BinOpKind::Shl {
        return None;
    }
    let has_mask_and_bit = shift_target(cx, cx.typeck_results().expr_ty(e))?;
    if !crate::is_int_const(cx, lhs, 1, "ONE")
        || is_const_amount(cx, rhs)
        || crate::in_const_context(cx, e)
        || in_power_of_2_fn(cx, e)
    {
        return None;
    }
    Some((lhs, rhs, has_mask_and_bit))
}

// The type to name in a `T::low_mask`/`T::power_of_2` suggestion: the type-path prefix of a
// `T::ONE` operand (preserving aliases like `Limb`), otherwise the resolved integer type.
fn int_type_name(cx: &LateContext<'_>, one: &Expr<'_>, shift: &Expr<'_>) -> String {
    if matches!(one.kind, ExprKind::Path(_)) {
        let s = snippet(cx, one.span, "");
        if let Some(i) = s.rfind("::") {
            return s[..i].to_string();
        }
    }
    cx.typeck_results().expr_ty(shift).to_string()
}

// If exactly one of `a`, `b` is the literal 0, returns the other.
fn non_zero_side<'tcx>(a: &'tcx Expr<'tcx>, b: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    match (crate::literal_value(a), crate::literal_value(b)) {
        (Some(0), _) => Some(b),
        (_, Some(0)) => Some(a),
        _ => None,
    }
}

// Whether `and_expr` (a `&`) is directly compared to 0, i.e. its parent is `_ != 0` or `_ == 0`.
fn compared_to_zero<'tcx>(cx: &LateContext<'tcx>, and_expr: &'tcx Expr<'tcx>) -> bool {
    let Some(parent) = get_parent_expr(cx, and_expr) else {
        return false;
    };
    matches!(
        parent.kind,
        ExprKind::Binary(op, a, b)
            if matches!(op.node, BinOpKind::Ne | BinOpKind::Eq) && non_zero_side(a, b).is_some()
    )
}

// Whether a `<one> << <n>` at `expr` is the operand of a `- 1` or of an `& _` compared to 0, so
// that the low-mask or get-bit case already reports it and the power-of-two case must stay quiet.
fn in_mask_or_bit_context<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> bool {
    let Some(parent) = get_parent_expr(cx, expr) else {
        return false;
    };
    match parent.kind {
        ExprKind::Binary(op, lhs, rhs) => {
            (op.node == BinOpKind::Sub
                && lhs.hir_id == expr.hir_id
                && crate::is_int_const(cx, rhs, 1, "ONE"))
                || (op.node == BinOpKind::BitAnd && compared_to_zero(cx, parent))
        }
        _ => false,
    }
}

impl<'tcx> LateLintPass<'tcx> for ShiftOfOne {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }

        // (a) `(<one> << <n>) - 1` -> `T::low_mask(<n>)`.
        if let ExprKind::Binary(op, lhs, rhs) = expr.kind
            && op.node == BinOpKind::Sub
            && crate::is_int_const(cx, rhs, 1, "ONE")
            && let Some((one, n, true)) = shift_of_one(cx, lhs)
        {
            span_lint_and_help(
                cx,
                SHIFT_OF_ONE,
                expr.span,
                "`(1 << n) - 1` builds a mask of the low `n` bits",
                None,
                format!(
                    "use `{}::low_mask({})`",
                    int_type_name(cx, one, lhs),
                    snippet(cx, n.span, "..")
                ),
            );
            return;
        }

        // (b) `x & (<one> << <n>) != 0` (or `== 0`) -> `x.get_bit(<n>)` (or `!x.get_bit(<n>)`).
        if let ExprKind::Binary(cmp, a, b) = expr.kind
            && matches!(cmp.node, BinOpKind::Ne | BinOpKind::Eq)
            && let Some(and_expr) = non_zero_side(a, b)
            && let ExprKind::Binary(and_op, l, r) = and_expr.kind
            && and_op.node == BinOpKind::BitAnd
            && let Some((x, n)) = shift_of_one(cx, r)
                .filter(|&(_, _, mask_and_bit)| mask_and_bit)
                .map(|(_, n, _)| (l, n))
                .or_else(|| {
                    shift_of_one(cx, l)
                        .filter(|&(_, _, mask_and_bit)| mask_and_bit)
                        .map(|(_, n, _)| (r, n))
                })
        {
            let bang = if cmp.node == BinOpKind::Eq { "!" } else { "" };
            span_lint_and_help(
                cx,
                SHIFT_OF_ONE,
                expr.span,
                "`x & (1 << n)` tests a single bit",
                None,
                format!(
                    "use `{bang}{}.get_bit({})`",
                    snippet(cx, x.span, ".."),
                    snippet(cx, n.span, "..")
                ),
            );
            return;
        }

        // (c) any other `<one> << <n>` -> `T::power_of_2(<n>)`.
        if let Some((one, n, has_mask_and_bit)) = shift_of_one(cx, expr)
            && !(has_mask_and_bit && in_mask_or_bit_context(cx, expr))
        {
            span_lint_and_help(
                cx,
                SHIFT_OF_ONE,
                expr.span,
                "`1 << n` constructs a power of two",
                None,
                format!(
                    "use `{}::power_of_2({})`",
                    int_type_name(cx, one, expr),
                    snippet(cx, n.span, "..")
                ),
            );
        }
    }
}
