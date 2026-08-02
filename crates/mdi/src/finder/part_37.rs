// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_37(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "keyboard-space" => Some(icons::KEYBOARD_SPACE),
        "alpha-o" => Some(icons::ALPHA_O),
        "web-remove" => Some(icons::WEB_REMOVE),
        "highway" => Some(icons::HIGHWAY),
        "flag-triangle" => Some(icons::FLAG_TRIANGLE),
        "diabetes" => Some(icons::DIABETES),
        "emoticon-plus-outline" => Some(icons::EMOTICON_PLUS_OUTLINE),
        "contain-end" => Some(icons::CONTAIN_END),
        #[allow(deprecated)]
        "apache-kafka" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apache-kafka' is deprecated.").print(py);
            }
            Some(icons::APACHE_KAFKA)
        }
        "bee" => Some(icons::BEE),
        "copyleft" => Some(icons::COPYLEFT),
        "account-child" => Some(icons::ACCOUNT_CHILD),
        "clock-remove-outline" => Some(icons::CLOCK_REMOVE_OUTLINE),
        "chart-box-outline" => Some(icons::CHART_BOX_OUTLINE),
        "message-text" => Some(icons::MESSAGE_TEXT),
        "ring" => Some(icons::RING),
        "alpha-e-circle" => Some(icons::ALPHA_E_CIRCLE),
        "arrow-up-box" => Some(icons::ARROW_UP_BOX),
        "grill" => Some(icons::GRILL),
        "lock-check-outline" => Some(icons::LOCK_CHECK_OUTLINE),
        "piano-off" => Some(icons::PIANO_OFF),
        "folder-upload" => Some(icons::FOLDER_UPLOAD),
        "racing-helmet" => Some(icons::RACING_HELMET),
        "map-check" => Some(icons::MAP_CHECK),
        "screwdriver" => Some(icons::SCREWDRIVER),
        "screw-machine-round-top" => Some(icons::SCREW_MACHINE_ROUND_TOP),
        "format-strikethrough" => Some(icons::FORMAT_STRIKETHROUGH),
        "cash-marker" => Some(icons::CASH_MARKER),
        "code-greater-than" => Some(icons::CODE_GREATER_THAN),
        "peanut-outline" => Some(icons::PEANUT_OUTLINE),
        "handball" => Some(icons::HANDBALL),
        "violin" => Some(icons::VIOLIN),
        "home-variant" => Some(icons::HOME_VARIANT),
        "food-turkey" => Some(icons::FOOD_TURKEY),
        "view-day" => Some(icons::VIEW_DAY),
        "cursor-move" => Some(icons::CURSOR_MOVE),
        "watch-export-variant" => Some(icons::WATCH_EXPORT_VARIANT),
        "panorama" => Some(icons::PANORAMA),
        "valve-closed" => Some(icons::VALVE_CLOSED),
        "border-top" => Some(icons::BORDER_TOP),
        "power-socket-eu" => Some(icons::POWER_SOCKET_EU),
        "ip" => Some(icons::IP),
        "paw-off-outline" => Some(icons::PAW_OFF_OUTLINE),
        "archive-arrow-up-outline" => Some(icons::ARCHIVE_ARROW_UP_OUTLINE),
        "delete-clock-outline" => Some(icons::DELETE_CLOCK_OUTLINE),
        "hand-extended-outline" => Some(icons::HAND_EXTENDED_OUTLINE),
        "cards-playing-spade-multiple" => Some(icons::CARDS_PLAYING_SPADE_MULTIPLE),
        _ => None,
    }
}
