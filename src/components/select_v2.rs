/// SelectV2 — API alias for [`Select`](super::select::Select).
///
/// Currently re-exports `Select`, `SelectProps`, `SelectOption`, and `SelectSize`
/// from the `select` module. This provides API compatibility for users who want
/// to use the `SelectV2` name.
///
/// # Future Plans
///
/// A virtualized version of Select (with support for large option lists via
/// windowing/virtual scrolling) will be implemented in a future version.
/// Until then, `SelectV2` behaves identically to `Select`.
pub use super::select::{Select, SelectOption, SelectProps, SelectSize};
