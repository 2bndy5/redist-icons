// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_37(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "hand-water" => Some(icons::HAND_WATER),
        "alpha-u" => Some(icons::ALPHA_U),
        "wall-sconce-flat-outline" => Some(icons::WALL_SCONCE_FLAT_OUTLINE),
        "hexagon-slice-3" => Some(icons::HEXAGON_SLICE_3),
        "flash-alert-outline" => Some(icons::FLASH_ALERT_OUTLINE),
        "view-week" => Some(icons::VIEW_WEEK),
        "hand-saw" => Some(icons::HAND_SAW),
        "alert-remove-outline" => Some(icons::ALERT_REMOVE_OUTLINE),
        "mower-bag" => Some(icons::MOWER_BAG),
        "table-refresh" => Some(icons::TABLE_REFRESH),
        "emoticon-happy-outline" => Some(icons::EMOTICON_HAPPY_OUTLINE),
        "leaf-circle-outline" => Some(icons::LEAF_CIRCLE_OUTLINE),
        "multiplication-box" => Some(icons::MULTIPLICATION_BOX),
        #[allow(deprecated)]
        "mastodon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mastodon' is deprecated.").print(py);
            }
            Some(icons::MASTODON)
        }
        "image" => Some(icons::IMAGE),
        "projector-screen" => Some(icons::PROJECTOR_SCREEN),
        "book-music-outline" => Some(icons::BOOK_MUSIC_OUTLINE),
        "target" => Some(icons::TARGET),
        "power-cycle" => Some(icons::POWER_CYCLE),
        #[allow(deprecated)]
        "google-nearby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-nearby' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_NEARBY)
        }
        "menu-up-outline" => Some(icons::MENU_UP_OUTLINE),
        "image-refresh-outline" => Some(icons::IMAGE_REFRESH_OUTLINE),
        "format-letter-case" => Some(icons::FORMAT_LETTER_CASE),
        "flask-off" => Some(icons::FLASK_OFF),
        "flag-remove-outline" => Some(icons::FLAG_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "language-markdown-outline" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'language-markdown-outline' is deprecated.",
                )
                .print(py);
            }
            Some(icons::LANGUAGE_MARKDOWN_OUTLINE)
        }
        #[allow(deprecated)]
        "apache-kafka" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apache-kafka' is deprecated.").print(py);
            }
            Some(icons::APACHE_KAFKA)
        }
        "barn" => Some(icons::BARN),
        "asterisk-circle-outline" => Some(icons::ASTERISK_CIRCLE_OUTLINE),
        "hat-fedora" => Some(icons::HAT_FEDORA),
        "peace" => Some(icons::PEACE),
        "bank-circle" => Some(icons::BANK_CIRCLE),
        "spray" => Some(icons::SPRAY),
        "database-minus" => Some(icons::DATABASE_MINUS),
        "brightness-percent" => Some(icons::BRIGHTNESS_PERCENT),
        "lotion-plus-outline" => Some(icons::LOTION_PLUS_OUTLINE),
        "brightness-3" => Some(icons::BRIGHTNESS_3),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-alert" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-alert' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_ALERT)
        }
        "beach" => Some(icons::BEACH),
        "shield-lock" => Some(icons::SHIELD_LOCK),
        "bus-school" => Some(icons::BUS_SCHOOL),
        "book-remove-outline" => Some(icons::BOOK_REMOVE_OUTLINE),
        "calendar-text-outline" => Some(icons::CALENDAR_TEXT_OUTLINE),
        "microphone-question" => Some(icons::MICROPHONE_QUESTION),
        #[allow(deprecated)]
        "salesforce" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'salesforce' is deprecated.").print(py);
            }
            Some(icons::SALESFORCE)
        }
        "weather-partly-lightning" => Some(icons::WEATHER_PARTLY_LIGHTNING),
        "flash-auto" => Some(icons::FLASH_AUTO),
        _ => None,
    }
}
