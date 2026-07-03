// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BodyOwnerKind, Constness, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags bignum conversions of integer literals that happen at runtime --
    /// `Natural::from(100u32)` and the like, and `Rational::from_unsigneds`/`from_signeds` of two
    /// literals -- as well as `const_from*` calls that are not inside a const context (a `const`
    /// block, a named `const`, a `static`, or a `const fn`). The latter matters most for the
    /// fraction constructors: `const_from_unsigneds`' naive Euclidean gcd is measurably slower at
    /// runtime than `from_unsigneds`.
    ///
    /// ### Why is this bad?
    ///
    /// A conversion of a literal can happen at compile time: `const_from*` is a const fn, and
    /// wrapping it in a `const` block (or binding it to a named `const`) makes the value free at
    /// runtime. A bare `const_from*` call outside a const context still runs at runtime, wasting
    /// the intent (measurements show it is not otherwise slower than `from` -- for `Float` it is
    /// actually faster -- but the convention is to make the compile-time evaluation explicit).
    ///
    /// Literals 0, 1, 2, and -1 are the named constants' territory (see `use_named_constant`) and
    /// are not flagged here. Literals that do not fit in a 32-bit limb are not flagged either,
    /// since `const_from*` takes a `Limb` and would not accept them under the `32_bit_limbs`
    /// feature.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let x = y % Natural::from(15u32);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let x = y % const { Natural::const_from(15) };
    /// ```
    pub RUNTIME_LITERAL_CONVERSION,
    Deny,
    "converting a literal to a bignum at runtime instead of in a const context"
}

declare_lint_pass!(RuntimeLiteralConversion => [RUNTIME_LITERAL_CONVERSION]);

const CONST_FROM_FNS: [&str; 7] = [
    "const_from",
    "const_from_unsigned",
    "const_from_signed",
    "const_from_unsigneds",
    "const_from_signeds",
    "const_from_unsigned_times_power_of_2",
    "const_from_signed_times_power_of_2",
];

// Whether the expression is inside a const context: the body of a `const` item, a `const` block,
// a `static` initializer, or a `const fn` (whose body is evaluated at compile time whenever its
// caller is).
fn in_const_context(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let owner = cx.tcx.hir_enclosing_body_owner(e.hir_id);
    match cx.tcx.hir_body_owner_kind(owner) {
        BodyOwnerKind::Const { .. } | BodyOwnerKind::Static(_) => true,
        BodyOwnerKind::Fn => cx.tcx.constness(owner) == Constness::Const,
        _ => false,
    }
}

impl<'tcx> LateLintPass<'tcx> for RuntimeLiteralConversion {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Call(callee, args) = expr.kind else {
            return;
        };
        let ExprKind::Path(qpath) = &callee.kind else {
            return;
        };
        let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
            return;
        };
        let fn_name = cx.tcx.item_name(fn_did);
        let fn_name = fn_name.as_str();
        let Some(t_name) = crate::bignum_name(cx, cx.typeck_results().expr_ty(expr)) else {
            return;
        };
        if CONST_FROM_FNS.contains(&fn_name) {
            // `const_from*` outside a const context still runs at runtime. (Literals 0, 1, 2, and
            // -1 are left to `use_named_constant`.)
            if in_const_context(cx, expr) {
                return;
            }
            if let [a] = args
                && let Some(v @ (0 | 1 | 2 | -1)) = crate::literal_value(a)
            {
                let _ = v;
                return;
            }
            span_lint(
                cx,
                RUNTIME_LITERAL_CONVERSION,
                expr.span,
                format!(
                    "this `{fn_name}` call is not evaluated at compile time; wrap it in a `const` \
                    block or bind it to a named `const` (or use the non-`const` equivalent if the \
                    arguments are not constants)"
                ),
            );
        } else if let ("Rational", "from_unsigneds" | "from_signeds", [n, d]) =
            (t_name, fn_name, args)
            && let Some(nv) = crate::literal_value(n)
            && let Some(dv) = crate::literal_value(d)
        {
            // (1, 2) is `Rational::ONE_HALF`, which `use_named_constant` handles; literals outside
            // the 32-bit limb range have no portable `const_from_*signeds`.
            let in_range = |v: i128| (i128::from(i32::MIN)..=i128::from(u32::MAX)).contains(&v);
            if (nv, dv) == (1, 2) || !in_range(nv) || !in_range(dv) {
                return;
            }
            let const_fn = if fn_name == "from_unsigneds" {
                "const_from_unsigneds"
            } else {
                "const_from_signeds"
            };
            span_lint(
                cx,
                RUNTIME_LITERAL_CONVERSION,
                expr.span,
                format!(
                    "evaluate this at compile time: use `Rational::{const_fn}(..)` in a `const` \
                    block or a named `const`"
                ),
            );
        } else if fn_name == "from"
            && let [a] = args
            && let Some(v) = crate::literal_value(a)
        {
            // 0, 1, 2, and -1 are the named constants' territory; literals outside the 32-bit
            // limb range have no portable `const_from*`.
            if matches!(v, 0 | 1 | 2 | -1) || v > i128::from(u32::MAX) || v < i128::from(i32::MIN) {
                return;
            }
            let const_fn = match (t_name, v < 0) {
                ("Natural", _) => "const_from",
                (_, true) => "const_from_signed",
                (_, false) => "const_from_unsigned",
            };
            span_lint(
                cx,
                RUNTIME_LITERAL_CONVERSION,
                expr.span,
                format!(
                    "evaluate this at compile time: use `{t_name}::{const_fn}(..)` in a `const` \
                    block or a named `const`"
                ),
            );
        }
    }
}
