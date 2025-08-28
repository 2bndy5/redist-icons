// This file was generated. DO NOT EDIT.
mod part_00;
mod part_01;
mod part_02;
mod part_03;
use crate::Icon;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg_attr(feature = "pyo3", pyfunction)]
pub fn get_icon(slug: &str) -> Option<Icon> {
    let mut result = part_00::find_part_0(slug);
    if result.is_none() {
        result = part_01::find_part_1(slug);
    }
    if result.is_none() {
        result = part_02::find_part_2(slug);
    }
    if result.is_none() {
        result = part_03::find_part_3(slug);
    }
    result
}
