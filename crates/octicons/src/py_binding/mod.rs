// This file was generated. DO NOT EDIT.
mod part_00;
mod part_01;
mod part_02;
mod part_03;
use crate::{Icon, finder::get_icon};
use pyo3::prelude::*;

#[pymodule]
pub fn octicons_pack(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_icon, m)?)?;
    m.add_class::<Icon>()?;
    part_00::bind_part_0(m)?;
    part_01::bind_part_1(m)?;
    part_02::bind_part_2(m)?;
    part_03::bind_part_3(m)?;
    Ok(())
}
