// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_25(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "octagram-outline" => Some(icons::OCTAGRAM_OUTLINE),
        "octagram-plus-outline" => Some(icons::OCTAGRAM_PLUS_OUTLINE),
        "octagram-plus" => Some(icons::OCTAGRAM_PLUS),
        "octagram" => Some(icons::OCTAGRAM),
        "octahedron-off" => Some(icons::OCTAHEDRON_OFF),
        "octahedron" => Some(icons::OCTAHEDRON),
        #[allow(deprecated)]
        "odnoklassniki" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'odnoklassniki' is deprecated.").print(py);
            }
            Some(icons::ODNOKLASSNIKI)
        }
        "offer" => Some(icons::OFFER),
        "office-building-cog-outline" => Some(icons::OFFICE_BUILDING_COG_OUTLINE),
        "office-building-cog" => Some(icons::OFFICE_BUILDING_COG),
        "office-building-marker-outline" => Some(icons::OFFICE_BUILDING_MARKER_OUTLINE),
        "office-building-marker" => Some(icons::OFFICE_BUILDING_MARKER),
        "office-building-minus-outline" => Some(icons::OFFICE_BUILDING_MINUS_OUTLINE),
        "office-building-minus" => Some(icons::OFFICE_BUILDING_MINUS),
        "office-building-outline" => Some(icons::OFFICE_BUILDING_OUTLINE),
        "office-building-plus-outline" => Some(icons::OFFICE_BUILDING_PLUS_OUTLINE),
        "office-building-plus" => Some(icons::OFFICE_BUILDING_PLUS),
        "office-building-remove-outline" => Some(icons::OFFICE_BUILDING_REMOVE_OUTLINE),
        "office-building-remove" => Some(icons::OFFICE_BUILDING_REMOVE),
        "office-building" => Some(icons::OFFICE_BUILDING),
        "oil-lamp" => Some(icons::OIL_LAMP),
        "oil-level" => Some(icons::OIL_LEVEL),
        "oil-temperature" => Some(icons::OIL_TEMPERATURE),
        "oil" => Some(icons::OIL),
        "om" => Some(icons::OM),
        "omega" => Some(icons::OMEGA),
        "one-up" => Some(icons::ONE_UP),
        #[allow(deprecated)]
        "onepassword" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'onepassword' is deprecated.").print(py);
            }
            Some(icons::ONEPASSWORD)
        }
        "opacity" => Some(icons::OPACITY),
        "open-in-app" => Some(icons::OPEN_IN_APP),
        "open-in-new" => Some(icons::OPEN_IN_NEW),
        #[allow(deprecated)]
        "open-source-initiative" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'open-source-initiative' is deprecated.")
                    .print(py);
            }
            Some(icons::OPEN_SOURCE_INITIATIVE)
        }
        #[allow(deprecated)]
        "openid" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'openid' is deprecated.").print(py);
            }
            Some(icons::OPENID)
        }
        #[allow(deprecated)]
        "opera" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'opera' is deprecated.").print(py);
            }
            Some(icons::OPERA)
        }
        "orbit-variant" => Some(icons::ORBIT_VARIANT),
        "orbit" => Some(icons::ORBIT),
        "order-alphabetical-ascending" => Some(icons::ORDER_ALPHABETICAL_ASCENDING),
        "order-alphabetical-descending" => Some(icons::ORDER_ALPHABETICAL_DESCENDING),
        "order-bool-ascending-variant" => Some(icons::ORDER_BOOL_ASCENDING_VARIANT),
        "order-bool-ascending" => Some(icons::ORDER_BOOL_ASCENDING),
        "order-bool-descending-variant" => Some(icons::ORDER_BOOL_DESCENDING_VARIANT),
        "order-bool-descending" => Some(icons::ORDER_BOOL_DESCENDING),
        "order-numeric-ascending" => Some(icons::ORDER_NUMERIC_ASCENDING),
        "order-numeric-descending" => Some(icons::ORDER_NUMERIC_DESCENDING),
        #[allow(deprecated)]
        "origin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'origin' is deprecated.").print(py);
            }
            Some(icons::ORIGIN)
        }
        "ornament-variant" => Some(icons::ORNAMENT_VARIANT),
        "ornament" => Some(icons::ORNAMENT),
        "outdoor-lamp" => Some(icons::OUTDOOR_LAMP),
        "overscan" => Some(icons::OVERSCAN),
        "owl" => Some(icons::OWL),
        "pac-man" => Some(icons::PAC_MAN),
        "package-check" => Some(icons::PACKAGE_CHECK),
        "package-down" => Some(icons::PACKAGE_DOWN),
        "package-up" => Some(icons::PACKAGE_UP),
        "package-variant-closed-check" => Some(icons::PACKAGE_VARIANT_CLOSED_CHECK),
        "package-variant-closed-minus" => Some(icons::PACKAGE_VARIANT_CLOSED_MINUS),
        "package-variant-closed-plus" => Some(icons::PACKAGE_VARIANT_CLOSED_PLUS),
        "package-variant-closed-remove" => Some(icons::PACKAGE_VARIANT_CLOSED_REMOVE),
        "package-variant-closed" => Some(icons::PACKAGE_VARIANT_CLOSED),
        "package-variant-minus" => Some(icons::PACKAGE_VARIANT_MINUS),
        "package-variant-plus" => Some(icons::PACKAGE_VARIANT_PLUS),
        "package-variant-remove" => Some(icons::PACKAGE_VARIANT_REMOVE),
        "package-variant" => Some(icons::PACKAGE_VARIANT),
        "package" => Some(icons::PACKAGE),
        "page-first" => Some(icons::PAGE_FIRST),
        "page-last" => Some(icons::PAGE_LAST),
        "page-layout-body" => Some(icons::PAGE_LAYOUT_BODY),
        "page-layout-footer" => Some(icons::PAGE_LAYOUT_FOOTER),
        "page-layout-header-footer" => Some(icons::PAGE_LAYOUT_HEADER_FOOTER),
        "page-layout-header" => Some(icons::PAGE_LAYOUT_HEADER),
        "page-layout-sidebar-left" => Some(icons::PAGE_LAYOUT_SIDEBAR_LEFT),
        "page-layout-sidebar-right" => Some(icons::PAGE_LAYOUT_SIDEBAR_RIGHT),
        "page-next-outline" => Some(icons::PAGE_NEXT_OUTLINE),
        "page-next" => Some(icons::PAGE_NEXT),
        "page-previous-outline" => Some(icons::PAGE_PREVIOUS_OUTLINE),
        "page-previous" => Some(icons::PAGE_PREVIOUS),
        "pail-minus-outline" => Some(icons::PAIL_MINUS_OUTLINE),
        "pail-minus" => Some(icons::PAIL_MINUS),
        "pail-off-outline" => Some(icons::PAIL_OFF_OUTLINE),
        "pail-off" => Some(icons::PAIL_OFF),
        "pail-outline" => Some(icons::PAIL_OUTLINE),
        "pail-plus-outline" => Some(icons::PAIL_PLUS_OUTLINE),
        "pail-plus" => Some(icons::PAIL_PLUS),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "pail-remove" => Some(icons::PAIL_REMOVE),
        "pail" => Some(icons::PAIL),
        "palette-advanced" => Some(icons::PALETTE_ADVANCED),
        "palette-outline" => Some(icons::PALETTE_OUTLINE),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "palette-swatch-variant" => Some(icons::PALETTE_SWATCH_VARIANT),
        "palette-swatch" => Some(icons::PALETTE_SWATCH),
        "palette" => Some(icons::PALETTE),
        "palm-tree" => Some(icons::PALM_TREE),
        "pan-bottom-left" => Some(icons::PAN_BOTTOM_LEFT),
        "pan-bottom-right" => Some(icons::PAN_BOTTOM_RIGHT),
        "pan-down" => Some(icons::PAN_DOWN),
        "pan-horizontal" => Some(icons::PAN_HORIZONTAL),
        "pan-left" => Some(icons::PAN_LEFT),
        "pan-right" => Some(icons::PAN_RIGHT),
        "pan-top-left" => Some(icons::PAN_TOP_LEFT),
        "pan-top-right" => Some(icons::PAN_TOP_RIGHT),
        "pan-up" => Some(icons::PAN_UP),
        "pan-vertical" => Some(icons::PAN_VERTICAL),
        "pan" => Some(icons::PAN),
        "panda" => Some(icons::PANDA),
        #[allow(deprecated)]
        "pandora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pandora' is deprecated.").print(py);
            }
            Some(icons::PANDORA)
        }
        "panorama-fisheye" => Some(icons::PANORAMA_FISHEYE),
        "panorama-horizontal-outline" => Some(icons::PANORAMA_HORIZONTAL_OUTLINE),
        "panorama-horizontal" => Some(icons::PANORAMA_HORIZONTAL),
        "panorama-outline" => Some(icons::PANORAMA_OUTLINE),
        "panorama-sphere-outline" => Some(icons::PANORAMA_SPHERE_OUTLINE),
        "panorama-sphere" => Some(icons::PANORAMA_SPHERE),
        "panorama-variant-outline" => Some(icons::PANORAMA_VARIANT_OUTLINE),
        "panorama-variant" => Some(icons::PANORAMA_VARIANT),
        "panorama-vertical-outline" => Some(icons::PANORAMA_VERTICAL_OUTLINE),
        "panorama-vertical" => Some(icons::PANORAMA_VERTICAL),
        "panorama-wide-angle-outline" => Some(icons::PANORAMA_WIDE_ANGLE_OUTLINE),
        "panorama-wide-angle" => Some(icons::PANORAMA_WIDE_ANGLE),
        "panorama" => Some(icons::PANORAMA),
        "paper-cut-vertical" => Some(icons::PAPER_CUT_VERTICAL),
        "paper-roll-outline" => Some(icons::PAPER_ROLL_OUTLINE),
        "paper-roll" => Some(icons::PAPER_ROLL),
        "paperclip-check" => Some(icons::PAPERCLIP_CHECK),
        "paperclip-lock" => Some(icons::PAPERCLIP_LOCK),
        "paperclip-minus" => Some(icons::PAPERCLIP_MINUS),
        "paperclip-off" => Some(icons::PAPERCLIP_OFF),
        "paperclip-plus" => Some(icons::PAPERCLIP_PLUS),
        "paperclip-remove" => Some(icons::PAPERCLIP_REMOVE),
        "paperclip" => Some(icons::PAPERCLIP),
        "parachute-outline" => Some(icons::PARACHUTE_OUTLINE),
        "parachute" => Some(icons::PARACHUTE),
        "paragliding" => Some(icons::PARAGLIDING),
        "parking" => Some(icons::PARKING),
        "party-popper" => Some(icons::PARTY_POPPER),
        "passport-alert" => Some(icons::PASSPORT_ALERT),
        "passport-biometric" => Some(icons::PASSPORT_BIOMETRIC),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "passport-check" => Some(icons::PASSPORT_CHECK),
        "passport-minus" => Some(icons::PASSPORT_MINUS),
        "passport-plus" => Some(icons::PASSPORT_PLUS),
        "passport-remove" => Some(icons::PASSPORT_REMOVE),
        "passport" => Some(icons::PASSPORT),
        "pasta" => Some(icons::PASTA),
        "patio-heater" => Some(icons::PATIO_HEATER),
        #[allow(deprecated)]
        "patreon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'patreon' is deprecated.").print(py);
            }
            Some(icons::PATREON)
        }
        "pause-box-outline" => Some(icons::PAUSE_BOX_OUTLINE),
        "pause-box" => Some(icons::PAUSE_BOX),
        "pause-circle-outline" => Some(icons::PAUSE_CIRCLE_OUTLINE),
        "pause-circle" => Some(icons::PAUSE_CIRCLE),
        "pause-octagon-outline" => Some(icons::PAUSE_OCTAGON_OUTLINE),
        "pause-octagon" => Some(icons::PAUSE_OCTAGON),
        "pause" => Some(icons::PAUSE),
        "paw-off-outline" => Some(icons::PAW_OFF_OUTLINE),
        "paw-off" => Some(icons::PAW_OFF),
        "paw-outline" => Some(icons::PAW_OUTLINE),
        "paw" => Some(icons::PAW),
        "peace" => Some(icons::PEACE),
        "peanut-off-outline" => Some(icons::PEANUT_OFF_OUTLINE),
        "peanut-off" => Some(icons::PEANUT_OFF),
        "peanut-outline" => Some(icons::PEANUT_OUTLINE),
        "peanut" => Some(icons::PEANUT),
        "pen-lock" => Some(icons::PEN_LOCK),
        "pen-minus" => Some(icons::PEN_MINUS),
        "pen-off" => Some(icons::PEN_OFF),
        "pen-plus" => Some(icons::PEN_PLUS),
        "pen-remove" => Some(icons::PEN_REMOVE),
        "pen" => Some(icons::PEN),
        "pencil-box-multiple-outline" => Some(icons::PENCIL_BOX_MULTIPLE_OUTLINE),
        "pencil-box-multiple" => Some(icons::PENCIL_BOX_MULTIPLE),
        "pencil-box-outline" => Some(icons::PENCIL_BOX_OUTLINE),
        "pencil-box" => Some(icons::PENCIL_BOX),
        "pencil-circle-outline" => Some(icons::PENCIL_CIRCLE_OUTLINE),
        "pencil-circle" => Some(icons::PENCIL_CIRCLE),
        "pencil-lock-outline" => Some(icons::PENCIL_LOCK_OUTLINE),
        "pencil-lock" => Some(icons::PENCIL_LOCK),
        "pencil-minus-outline" => Some(icons::PENCIL_MINUS_OUTLINE),
        "pencil-minus" => Some(icons::PENCIL_MINUS),
        "pencil-off-outline" => Some(icons::PENCIL_OFF_OUTLINE),
        "pencil-off" => Some(icons::PENCIL_OFF),
        "pencil-outline" => Some(icons::PENCIL_OUTLINE),
        "pencil-plus-outline" => Some(icons::PENCIL_PLUS_OUTLINE),
        "pencil-plus" => Some(icons::PENCIL_PLUS),
        "pencil-remove-outline" => Some(icons::PENCIL_REMOVE_OUTLINE),
        "pencil-remove" => Some(icons::PENCIL_REMOVE),
        "pencil-ruler-outline" => Some(icons::PENCIL_RULER_OUTLINE),
        "pencil-ruler" => Some(icons::PENCIL_RULER),
        "pencil" => Some(icons::PENCIL),
        "penguin" => Some(icons::PENGUIN),
        "pentagon-outline" => Some(icons::PENTAGON_OUTLINE),
        "pentagon" => Some(icons::PENTAGON),
        "pentagram" => Some(icons::PENTAGRAM),
        "percent-box-outline" => Some(icons::PERCENT_BOX_OUTLINE),
        "percent-box" => Some(icons::PERCENT_BOX),
        "percent-circle-outline" => Some(icons::PERCENT_CIRCLE_OUTLINE),
        "percent-circle" => Some(icons::PERCENT_CIRCLE),
        "percent-outline" => Some(icons::PERCENT_OUTLINE),
        "percent" => Some(icons::PERCENT),
        "periodic-table" => Some(icons::PERIODIC_TABLE),
        "perspective-less" => Some(icons::PERSPECTIVE_LESS),
        "perspective-more" => Some(icons::PERSPECTIVE_MORE),
        _ => None,
    }
}
