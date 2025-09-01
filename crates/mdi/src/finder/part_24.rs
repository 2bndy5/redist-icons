// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_24(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "nail" => Some(icons::NAIL),
        "nas" => Some(icons::NAS),
        #[allow(deprecated)]
        "nativescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nativescript' is deprecated.").print(py);
            }
            Some(icons::NATIVESCRIPT)
        }
        "nature-outline" => Some(icons::NATURE_OUTLINE),
        "nature-people-outline" => Some(icons::NATURE_PEOPLE_OUTLINE),
        "nature-people" => Some(icons::NATURE_PEOPLE),
        "nature" => Some(icons::NATURE),
        "navigation-outline" => Some(icons::NAVIGATION_OUTLINE),
        "navigation-variant-outline" => Some(icons::NAVIGATION_VARIANT_OUTLINE),
        "navigation-variant" => Some(icons::NAVIGATION_VARIANT),
        "navigation" => Some(icons::NAVIGATION),
        "near-me" => Some(icons::NEAR_ME),
        "necklace" => Some(icons::NECKLACE),
        "needle-off" => Some(icons::NEEDLE_OFF),
        "needle" => Some(icons::NEEDLE),
        #[allow(deprecated)]
        "netflix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'netflix' is deprecated.").print(py);
            }
            Some(icons::NETFLIX)
        }
        "network-off-outline" => Some(icons::NETWORK_OFF_OUTLINE),
        "network-off" => Some(icons::NETWORK_OFF),
        "network-outline" => Some(icons::NETWORK_OUTLINE),
        "network-pos" => Some(icons::NETWORK_POS),
        "network-strength-1-alert" => Some(icons::NETWORK_STRENGTH_1_ALERT),
        "network-strength-1" => Some(icons::NETWORK_STRENGTH_1),
        "network-strength-2-alert" => Some(icons::NETWORK_STRENGTH_2_ALERT),
        "network-strength-2" => Some(icons::NETWORK_STRENGTH_2),
        "network-strength-3-alert" => Some(icons::NETWORK_STRENGTH_3_ALERT),
        "network-strength-3" => Some(icons::NETWORK_STRENGTH_3),
        "network-strength-4-alert" => Some(icons::NETWORK_STRENGTH_4_ALERT),
        "network-strength-4-cog" => Some(icons::NETWORK_STRENGTH_4_COG),
        "network-strength-4" => Some(icons::NETWORK_STRENGTH_4),
        "network-strength-off-outline" => Some(icons::NETWORK_STRENGTH_OFF_OUTLINE),
        "network-strength-off" => Some(icons::NETWORK_STRENGTH_OFF),
        "network-strength-outline" => Some(icons::NETWORK_STRENGTH_OUTLINE),
        "network" => Some(icons::NETWORK),
        "new-box" => Some(icons::NEW_BOX),
        "newspaper-check" => Some(icons::NEWSPAPER_CHECK),
        "newspaper-minus" => Some(icons::NEWSPAPER_MINUS),
        "newspaper-plus" => Some(icons::NEWSPAPER_PLUS),
        "newspaper-remove" => Some(icons::NEWSPAPER_REMOVE),
        "newspaper-variant-multiple-outline" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE_OUTLINE),
        "newspaper-variant-multiple" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE),
        "newspaper-variant-outline" => Some(icons::NEWSPAPER_VARIANT_OUTLINE),
        "newspaper-variant" => Some(icons::NEWSPAPER_VARIANT),
        "newspaper" => Some(icons::NEWSPAPER),
        "nfc-search-variant" => Some(icons::NFC_SEARCH_VARIANT),
        "nfc-tap" => Some(icons::NFC_TAP),
        "nfc-variant-off" => Some(icons::NFC_VARIANT_OFF),
        "nfc-variant" => Some(icons::NFC_VARIANT),
        #[allow(deprecated)]
        "nfc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nfc' is deprecated.").print(py);
            }
            Some(icons::NFC)
        }
        "ninja" => Some(icons::NINJA),
        "nintendo-game-boy" => Some(icons::NINTENDO_GAME_BOY),
        #[allow(deprecated)]
        "nintendo-switch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-switch' is deprecated.")
                    .print(py);
            }
            Some(icons::NINTENDO_SWITCH)
        }
        #[allow(deprecated)]
        "nintendo-wii" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wii' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WII)
        }
        #[allow(deprecated)]
        "nintendo-wiiu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wiiu' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WIIU)
        }
        #[allow(deprecated)]
        "nix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nix' is deprecated.").print(py);
            }
            Some(icons::NIX)
        }
        #[allow(deprecated)]
        "nodejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nodejs' is deprecated.").print(py);
            }
            Some(icons::NODEJS)
        }
        "noodles" => Some(icons::NOODLES),
        "not-equal-variant" => Some(icons::NOT_EQUAL_VARIANT),
        "not-equal" => Some(icons::NOT_EQUAL),
        "note-alert-outline" => Some(icons::NOTE_ALERT_OUTLINE),
        "note-alert" => Some(icons::NOTE_ALERT),
        "note-check-outline" => Some(icons::NOTE_CHECK_OUTLINE),
        "note-check" => Some(icons::NOTE_CHECK),
        "note-edit-outline" => Some(icons::NOTE_EDIT_OUTLINE),
        "note-edit" => Some(icons::NOTE_EDIT),
        "note-minus-outline" => Some(icons::NOTE_MINUS_OUTLINE),
        "note-minus" => Some(icons::NOTE_MINUS),
        "note-multiple-outline" => Some(icons::NOTE_MULTIPLE_OUTLINE),
        "note-multiple" => Some(icons::NOTE_MULTIPLE),
        "note-off-outline" => Some(icons::NOTE_OFF_OUTLINE),
        "note-off" => Some(icons::NOTE_OFF),
        "note-outline" => Some(icons::NOTE_OUTLINE),
        "note-plus-outline" => Some(icons::NOTE_PLUS_OUTLINE),
        "note-plus" => Some(icons::NOTE_PLUS),
        "note-remove-outline" => Some(icons::NOTE_REMOVE_OUTLINE),
        "note-remove" => Some(icons::NOTE_REMOVE),
        "note-search-outline" => Some(icons::NOTE_SEARCH_OUTLINE),
        "note-search" => Some(icons::NOTE_SEARCH),
        "note-text-outline" => Some(icons::NOTE_TEXT_OUTLINE),
        "note-text" => Some(icons::NOTE_TEXT),
        "note" => Some(icons::NOTE),
        "notebook-check-outline" => Some(icons::NOTEBOOK_CHECK_OUTLINE),
        "notebook-check" => Some(icons::NOTEBOOK_CHECK),
        "notebook-edit-outline" => Some(icons::NOTEBOOK_EDIT_OUTLINE),
        "notebook-edit" => Some(icons::NOTEBOOK_EDIT),
        "notebook-heart-outline" => Some(icons::NOTEBOOK_HEART_OUTLINE),
        "notebook-heart" => Some(icons::NOTEBOOK_HEART),
        "notebook-minus-outline" => Some(icons::NOTEBOOK_MINUS_OUTLINE),
        "notebook-minus" => Some(icons::NOTEBOOK_MINUS),
        "notebook-multiple" => Some(icons::NOTEBOOK_MULTIPLE),
        "notebook-outline" => Some(icons::NOTEBOOK_OUTLINE),
        "notebook-plus-outline" => Some(icons::NOTEBOOK_PLUS_OUTLINE),
        "notebook-plus" => Some(icons::NOTEBOOK_PLUS),
        "notebook-remove-outline" => Some(icons::NOTEBOOK_REMOVE_OUTLINE),
        "notebook-remove" => Some(icons::NOTEBOOK_REMOVE),
        "notebook" => Some(icons::NOTEBOOK),
        "notification-clear-all" => Some(icons::NOTIFICATION_CLEAR_ALL),
        #[allow(deprecated)]
        "npm" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'npm' is deprecated.").print(py);
            }
            Some(icons::NPM)
        }
        "nuke" => Some(icons::NUKE),
        "null" => Some(icons::NULL),
        "numeric-0-box-multiple-outline" => Some(icons::NUMERIC_0_BOX_MULTIPLE_OUTLINE),
        "numeric-0-box-multiple" => Some(icons::NUMERIC_0_BOX_MULTIPLE),
        "numeric-0-box-outline" => Some(icons::NUMERIC_0_BOX_OUTLINE),
        "numeric-0-box" => Some(icons::NUMERIC_0_BOX),
        "numeric-0-circle-outline" => Some(icons::NUMERIC_0_CIRCLE_OUTLINE),
        "numeric-0-circle" => Some(icons::NUMERIC_0_CIRCLE),
        "numeric-0" => Some(icons::NUMERIC_0),
        "numeric-1-box-multiple-outline" => Some(icons::NUMERIC_1_BOX_MULTIPLE_OUTLINE),
        "numeric-1-box-multiple" => Some(icons::NUMERIC_1_BOX_MULTIPLE),
        "numeric-1-box-outline" => Some(icons::NUMERIC_1_BOX_OUTLINE),
        "numeric-1-box" => Some(icons::NUMERIC_1_BOX),
        "numeric-1-circle-outline" => Some(icons::NUMERIC_1_CIRCLE_OUTLINE),
        "numeric-1-circle" => Some(icons::NUMERIC_1_CIRCLE),
        "numeric-1" => Some(icons::NUMERIC_1),
        "numeric-10-box-multiple-outline" => Some(icons::NUMERIC_10_BOX_MULTIPLE_OUTLINE),
        "numeric-10-box-multiple" => Some(icons::NUMERIC_10_BOX_MULTIPLE),
        "numeric-10-box-outline" => Some(icons::NUMERIC_10_BOX_OUTLINE),
        "numeric-10-box" => Some(icons::NUMERIC_10_BOX),
        "numeric-10-circle-outline" => Some(icons::NUMERIC_10_CIRCLE_OUTLINE),
        "numeric-10-circle" => Some(icons::NUMERIC_10_CIRCLE),
        "numeric-10" => Some(icons::NUMERIC_10),
        "numeric-2-box-multiple-outline" => Some(icons::NUMERIC_2_BOX_MULTIPLE_OUTLINE),
        "numeric-2-box-multiple" => Some(icons::NUMERIC_2_BOX_MULTIPLE),
        "numeric-2-box-outline" => Some(icons::NUMERIC_2_BOX_OUTLINE),
        "numeric-2-box" => Some(icons::NUMERIC_2_BOX),
        "numeric-2-circle-outline" => Some(icons::NUMERIC_2_CIRCLE_OUTLINE),
        "numeric-2-circle" => Some(icons::NUMERIC_2_CIRCLE),
        "numeric-2" => Some(icons::NUMERIC_2),
        "numeric-3-box-multiple-outline" => Some(icons::NUMERIC_3_BOX_MULTIPLE_OUTLINE),
        "numeric-3-box-multiple" => Some(icons::NUMERIC_3_BOX_MULTIPLE),
        "numeric-3-box-outline" => Some(icons::NUMERIC_3_BOX_OUTLINE),
        "numeric-3-box" => Some(icons::NUMERIC_3_BOX),
        "numeric-3-circle-outline" => Some(icons::NUMERIC_3_CIRCLE_OUTLINE),
        "numeric-3-circle" => Some(icons::NUMERIC_3_CIRCLE),
        "numeric-3" => Some(icons::NUMERIC_3),
        "numeric-4-box-multiple-outline" => Some(icons::NUMERIC_4_BOX_MULTIPLE_OUTLINE),
        "numeric-4-box-multiple" => Some(icons::NUMERIC_4_BOX_MULTIPLE),
        "numeric-4-box-outline" => Some(icons::NUMERIC_4_BOX_OUTLINE),
        "numeric-4-box" => Some(icons::NUMERIC_4_BOX),
        "numeric-4-circle-outline" => Some(icons::NUMERIC_4_CIRCLE_OUTLINE),
        "numeric-4-circle" => Some(icons::NUMERIC_4_CIRCLE),
        "numeric-4" => Some(icons::NUMERIC_4),
        "numeric-5-box-multiple-outline" => Some(icons::NUMERIC_5_BOX_MULTIPLE_OUTLINE),
        "numeric-5-box-multiple" => Some(icons::NUMERIC_5_BOX_MULTIPLE),
        "numeric-5-box-outline" => Some(icons::NUMERIC_5_BOX_OUTLINE),
        "numeric-5-box" => Some(icons::NUMERIC_5_BOX),
        "numeric-5-circle-outline" => Some(icons::NUMERIC_5_CIRCLE_OUTLINE),
        "numeric-5-circle" => Some(icons::NUMERIC_5_CIRCLE),
        "numeric-5" => Some(icons::NUMERIC_5),
        "numeric-6-box-multiple-outline" => Some(icons::NUMERIC_6_BOX_MULTIPLE_OUTLINE),
        "numeric-6-box-multiple" => Some(icons::NUMERIC_6_BOX_MULTIPLE),
        "numeric-6-box-outline" => Some(icons::NUMERIC_6_BOX_OUTLINE),
        "numeric-6-box" => Some(icons::NUMERIC_6_BOX),
        "numeric-6-circle-outline" => Some(icons::NUMERIC_6_CIRCLE_OUTLINE),
        "numeric-6-circle" => Some(icons::NUMERIC_6_CIRCLE),
        "numeric-6" => Some(icons::NUMERIC_6),
        "numeric-7-box-multiple-outline" => Some(icons::NUMERIC_7_BOX_MULTIPLE_OUTLINE),
        "numeric-7-box-multiple" => Some(icons::NUMERIC_7_BOX_MULTIPLE),
        "numeric-7-box-outline" => Some(icons::NUMERIC_7_BOX_OUTLINE),
        "numeric-7-box" => Some(icons::NUMERIC_7_BOX),
        "numeric-7-circle-outline" => Some(icons::NUMERIC_7_CIRCLE_OUTLINE),
        "numeric-7-circle" => Some(icons::NUMERIC_7_CIRCLE),
        "numeric-7" => Some(icons::NUMERIC_7),
        "numeric-8-box-multiple-outline" => Some(icons::NUMERIC_8_BOX_MULTIPLE_OUTLINE),
        "numeric-8-box-multiple" => Some(icons::NUMERIC_8_BOX_MULTIPLE),
        "numeric-8-box-outline" => Some(icons::NUMERIC_8_BOX_OUTLINE),
        "numeric-8-box" => Some(icons::NUMERIC_8_BOX),
        "numeric-8-circle-outline" => Some(icons::NUMERIC_8_CIRCLE_OUTLINE),
        "numeric-8-circle" => Some(icons::NUMERIC_8_CIRCLE),
        "numeric-8" => Some(icons::NUMERIC_8),
        "numeric-9-box-multiple-outline" => Some(icons::NUMERIC_9_BOX_MULTIPLE_OUTLINE),
        "numeric-9-box-multiple" => Some(icons::NUMERIC_9_BOX_MULTIPLE),
        "numeric-9-box-outline" => Some(icons::NUMERIC_9_BOX_OUTLINE),
        "numeric-9-box" => Some(icons::NUMERIC_9_BOX),
        "numeric-9-circle-outline" => Some(icons::NUMERIC_9_CIRCLE_OUTLINE),
        "numeric-9-circle" => Some(icons::NUMERIC_9_CIRCLE),
        "numeric-9-plus-box-multiple-outline" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE_OUTLINE),
        "numeric-9-plus-box-multiple" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE),
        "numeric-9-plus-box-outline" => Some(icons::NUMERIC_9_PLUS_BOX_OUTLINE),
        "numeric-9-plus-box" => Some(icons::NUMERIC_9_PLUS_BOX),
        "numeric-9-plus-circle-outline" => Some(icons::NUMERIC_9_PLUS_CIRCLE_OUTLINE),
        "numeric-9-plus-circle" => Some(icons::NUMERIC_9_PLUS_CIRCLE),
        "numeric-9-plus" => Some(icons::NUMERIC_9_PLUS),
        "numeric-9" => Some(icons::NUMERIC_9),
        "numeric-negative-1" => Some(icons::NUMERIC_NEGATIVE_1),
        "numeric-off" => Some(icons::NUMERIC_OFF),
        "numeric-positive-1" => Some(icons::NUMERIC_POSITIVE_1),
        "numeric" => Some(icons::NUMERIC),
        "nut" => Some(icons::NUT),
        "nutrition" => Some(icons::NUTRITION),
        #[allow(deprecated)]
        "nuxt" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nuxt' is deprecated.").print(py);
            }
            Some(icons::NUXT)
        }
        "oar" => Some(icons::OAR),
        "ocarina" => Some(icons::OCARINA),
        #[allow(deprecated)]
        "oci" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'oci' is deprecated.").print(py);
            }
            Some(icons::OCI)
        }
        "ocr" => Some(icons::OCR),
        "octagon-outline" => Some(icons::OCTAGON_OUTLINE),
        "octagon" => Some(icons::OCTAGON),
        "octagram-edit-outline" => Some(icons::OCTAGRAM_EDIT_OUTLINE),
        "octagram-edit" => Some(icons::OCTAGRAM_EDIT),
        "octagram-minus-outline" => Some(icons::OCTAGRAM_MINUS_OUTLINE),
        "octagram-minus" => Some(icons::OCTAGRAM_MINUS),
        _ => None,
    }
}
