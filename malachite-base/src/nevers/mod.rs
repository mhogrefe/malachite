/// `Never` is a type that cannot be instantiated.
///
/// In other languages this type may be called `Nothing`, `Empty`, or `Void`.
///
/// # Examples
/// ```
/// use malachite_base::nevers::Never;
///
/// let x: Option<Never> = None;
/// ```
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Never {}
