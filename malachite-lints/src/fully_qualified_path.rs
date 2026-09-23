// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{HirId, ItemKind, Node, Path};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a path that names an item of a Malachite crate starting from the crate root, such as
    /// `malachite_nz::natural::Natural::from(2u32)` or a type written as
    /// `malachite_base::unsigned_polynomial::UnsignedPolynomial<u8>`, anywhere but in a `use` item.
    ///
    /// ### Why is this bad?
    ///
    /// House style is to import items and refer to them by name. A full path buries the expression
    /// it appears in, and hides a dependency that the imports at the top of the file would
    /// otherwise show.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let m = malachite_nz::natural::Natural::from(3u32);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use malachite_nz::natural::Natural;
    ///
    /// let m = Natural::from(3u32);
    /// ```
    pub FULLY_QUALIFIED_PATH,
    Deny,
    "naming a Malachite item by its full path instead of importing it"
}

declare_lint_pass!(FullyQualifiedPath => [FULLY_QUALIFIED_PATH]);

impl<'tcx> LateLintPass<'tcx> for FullyQualifiedPath {
    fn check_path(&mut self, cx: &LateContext<'tcx>, path: &Path<'tcx>, hir_id: HirId) {
        // Paths written by macros (including `$crate::` paths) are not the author's to change.
        if path.span.from_expansion() || path.segments.len() < 2 {
            return;
        }
        // A `use` item is where full paths belong.
        if let Node::Item(item) = cx.tcx.hir_node(hir_id)
            && matches!(item.kind, ItemKind::Use(..))
        {
            return;
        }
        let first = &path.segments[0];
        let Res::Def(DefKind::Mod, did) = first.res else {
            return;
        };
        if !did.is_crate_root() || did.is_local() {
            return;
        }
        let crate_name = cx.tcx.crate_name(did.krate);
        if !crate_name.as_str().starts_with("malachite") {
            return;
        }
        // The item to import is the first segment past the modules: the type in
        // `malachite_nz::natural::Natural::from`, the trait in `...::traits::Mod::mod_op`.
        let item = path.segments[1..]
            .iter()
            .find(|segment| !matches!(segment.res, Res::Def(DefKind::Mod, _)))
            .unwrap_or_else(|| path.segments.last().unwrap())
            .ident;
        span_lint_and_help(
            cx,
            FULLY_QUALIFIED_PATH,
            path.span,
            format!("`{item}` is named by its full path"),
            None,
            format!("import it with `use` and refer to it as `{item}`"),
        );
    }
}
