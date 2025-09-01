// This file was generated. DO NOT EDIT.
mod part_00;
mod part_01;
mod part_02;
mod part_03;
mod part_04;
mod part_05;
mod part_06;
mod part_07;
mod part_08;
mod part_09;
mod part_10;
mod part_11;
mod part_12;
mod part_13;
mod part_14;
mod part_15;
mod part_16;
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
    if result.is_none() {
        result = part_04::find_part_4(slug);
    }
    if result.is_none() {
        result = part_05::find_part_5(slug);
    }
    if result.is_none() {
        result = part_06::find_part_6(slug);
    }
    if result.is_none() {
        result = part_07::find_part_7(slug);
    }
    if result.is_none() {
        result = part_08::find_part_8(slug);
    }
    if result.is_none() {
        result = part_09::find_part_9(slug);
    }
    if result.is_none() {
        result = part_10::find_part_10(slug);
    }
    if result.is_none() {
        result = part_11::find_part_11(slug);
    }
    if result.is_none() {
        result = part_12::find_part_12(slug);
    }
    if result.is_none() {
        result = part_13::find_part_13(slug);
    }
    if result.is_none() {
        result = part_14::find_part_14(slug);
    }
    if result.is_none() {
        result = part_15::find_part_15(slug);
    }
    if result.is_none() {
        result = part_16::find_part_16(slug);
    }
    result
}
