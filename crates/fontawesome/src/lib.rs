#![doc = include_str!("../README.md")]
mod icons;
pub use icons::*;

mod finder;
pub use finder::get_icon;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "pyo3")]
pub mod py_binding;

/// A Generic structure to describe a single icon.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "fontawesome_free_icons_pack", get_all, frozen)
)]
#[derive(Debug, PartialEq, Eq)]
pub struct Icon {
    /// The SVG data.
    pub svg: &'static str,

    /// The slug to identify the icon.
    pub slug: &'static str,

    /// The Unix timestamp of the icon's last modification.
    pub last_modified: u32,

    /// A flag to indicate family of the icon.
    pub family: &'static str,

    /// The width of the icon.
    pub width: u16,

    /// The height of the icon.
    pub height: u16,

    /// The label (human readable name) of the icon.
    pub label: &'static str,
    // The list of `aliases` would need (in a const context) either
    // - a lifetime boundary
    // - "lazy" static allocation
    // Both solutions do not work well for python bindings.
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl Icon {
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("< Icon object for slug {} >", self.slug))
    }
}
