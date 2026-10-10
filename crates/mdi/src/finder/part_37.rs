// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_37(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "source-commit-start" => Some(icons::SOURCE_COMMIT_START),
        "book-remove" => Some(icons::BOOK_REMOVE),
        "timer-check-outline" => Some(icons::TIMER_CHECK_OUTLINE),
        "database-arrow-up" => Some(icons::DATABASE_ARROW_UP),
        "arrow-top-left-bottom-right-bold" => Some(icons::ARROW_TOP_LEFT_BOTTOM_RIGHT_BOLD),
        "movie-remove-outline" => Some(icons::MOVIE_REMOVE_OUTLINE),
        "rv-truck" => Some(icons::RV_TRUCK),
        "om" => Some(icons::OM),
        "directions-fork" => Some(icons::DIRECTIONS_FORK),
        "store-check" => Some(icons::STORE_CHECK),
        "bicycle-penny-farthing" => Some(icons::BICYCLE_PENNY_FARTHING),
        "car-shift-pattern" => Some(icons::CAR_SHIFT_PATTERN),
        "delete-forever-outline" => Some(icons::DELETE_FOREVER_OUTLINE),
        "shield-off" => Some(icons::SHIELD_OFF),
        "tanker-truck" => Some(icons::TANKER_TRUCK),
        "cloud-arrow-left-outline" => Some(icons::CLOUD_ARROW_LEFT_OUTLINE),
        "close-box" => Some(icons::CLOSE_BOX),
        "cookie-off" => Some(icons::COOKIE_OFF),
        "paperclip" => Some(icons::PAPERCLIP),
        "car-coolant-level" => Some(icons::CAR_COOLANT_LEVEL),
        "numeric-3-box-multiple-outline" => Some(icons::NUMERIC_3_BOX_MULTIPLE_OUTLINE),
        "star-david" => Some(icons::STAR_DAVID),
        "human-greeting-proximity" => Some(icons::HUMAN_GREETING_PROXIMITY),
        "book-marker" => Some(icons::BOOK_MARKER),
        "tab-unselected" => Some(icons::TAB_UNSELECTED),
        "camera-front" => Some(icons::CAMERA_FRONT),
        "mail" => Some(icons::MAIL),
        "phone-sync" => Some(icons::PHONE_SYNC),
        #[allow(deprecated)]
        "vlc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vlc' is deprecated.").print(py);
            }
            Some(icons::VLC)
        }
        "dump-truck" => Some(icons::DUMP_TRUCK),
        "equalizer-outline" => Some(icons::EQUALIZER_OUTLINE),
        "volume-mute" => Some(icons::VOLUME_MUTE),
        "numeric-7-circle-outline" => Some(icons::NUMERIC_7_CIRCLE_OUTLINE),
        "boom-gate-arrow-down" => Some(icons::BOOM_GATE_ARROW_DOWN),
        "dice-d12" => Some(icons::DICE_D12),
        "bunk-bed" => Some(icons::BUNK_BED),
        "cloud-off" => Some(icons::CLOUD_OFF),
        "owl" => Some(icons::OWL),
        "sticker-plus" => Some(icons::STICKER_PLUS),
        "alarm-panel-outline" => Some(icons::ALARM_PANEL_OUTLINE),
        "bus-side" => Some(icons::BUS_SIDE),
        "hanger" => Some(icons::HANGER),
        "moped" => Some(icons::MOPED),
        "currency-eur" => Some(icons::CURRENCY_EUR),
        "note-multiple" => Some(icons::NOTE_MULTIPLE),
        "format-paragraph-spacing" => Some(icons::FORMAT_PARAGRAPH_SPACING),
        "network-strength-4" => Some(icons::NETWORK_STRENGTH_4),
        _ => None,
    }
}
