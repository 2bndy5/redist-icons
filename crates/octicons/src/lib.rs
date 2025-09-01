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
#[cfg_attr(feature = "pyo3", pyclass(module = "octicons_pack", get_all, frozen))]
#[derive(Debug)]
pub struct Icon {
    /// The SVG data.
    pub svg: &'static str,

    /// The slug to identify the icon.
    pub slug: &'static str,
    // The list of `keywords` and `heights` would need (in a const context) either
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
