// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_37(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "roman-numeral-7" => Some(icons::ROMAN_NUMERAL_7),
        #[allow(deprecated)]
        "webpack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webpack' is deprecated.").print(py);
            }
            Some(icons::WEBPACK)
        }
        "battery-charging-80" => Some(icons::BATTERY_CHARGING_80),
        "pencil-lock" => Some(icons::PENCIL_LOCK),
        "file-swap-outline" => Some(icons::FILE_SWAP_OUTLINE),
        "note-multiple" => Some(icons::NOTE_MULTIPLE),
        "arrow-right-circle-outline" => Some(icons::ARROW_RIGHT_CIRCLE_OUTLINE),
        "alpha-d-circle" => Some(icons::ALPHA_D_CIRCLE),
        "contrast" => Some(icons::CONTRAST),
        "plus-minus-variant" => Some(icons::PLUS_MINUS_VARIANT),
        "cake-variant-outline" => Some(icons::CAKE_VARIANT_OUTLINE),
        "faucet" => Some(icons::FAUCET),
        #[allow(deprecated)]
        "microsoft-visual-studio-code" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-visual-studio-code' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_VISUAL_STUDIO_CODE)
        }
        "message-fast-outline" => Some(icons::MESSAGE_FAST_OUTLINE),
        "calendar-cursor-outline" => Some(icons::CALENDAR_CURSOR_OUTLINE),
        "clock-remove" => Some(icons::CLOCK_REMOVE),
        "message-image" => Some(icons::MESSAGE_IMAGE),
        "video-input-scart" => Some(icons::VIDEO_INPUT_SCART),
        "truck-delivery" => Some(icons::TRUCK_DELIVERY),
        "view-day-outline" => Some(icons::VIEW_DAY_OUTLINE),
        "radio-off" => Some(icons::RADIO_OFF),
        "hand-okay" => Some(icons::HAND_OKAY),
        "barrel" => Some(icons::BARREL),
        "window-open-variant" => Some(icons::WINDOW_OPEN_VARIANT),
        "video-3d-off" => Some(icons::VIDEO_3D_OFF),
        "image-filter-vintage" => Some(icons::IMAGE_FILTER_VINTAGE),
        "hoop-house" => Some(icons::HOOP_HOUSE),
        "chart-waterfall" => Some(icons::CHART_WATERFALL),
        "numeric-0" => Some(icons::NUMERIC_0),
        "video-4k-box" => Some(icons::VIDEO_4K_BOX),
        "weather-night-partly-cloudy" => Some(icons::WEATHER_NIGHT_PARTLY_CLOUDY),
        "book-multiple-outline" => Some(icons::BOOK_MULTIPLE_OUTLINE),
        "engine" => Some(icons::ENGINE),
        "account-group" => Some(icons::ACCOUNT_GROUP),
        "spider" => Some(icons::SPIDER),
        "electric-switch-closed" => Some(icons::ELECTRIC_SWITCH_CLOSED),
        "av-timer" => Some(icons::AV_TIMER),
        "rickshaw-electric" => Some(icons::RICKSHAW_ELECTRIC),
        #[allow(deprecated)]
        "jsfiddle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jsfiddle' is deprecated.").print(py);
            }
            Some(icons::JSFIDDLE)
        }
        "note-minus-outline" => Some(icons::NOTE_MINUS_OUTLINE),
        "pencil-minus-outline" => Some(icons::PENCIL_MINUS_OUTLINE),
        "battery-charging" => Some(icons::BATTERY_CHARGING),
        "chat-question" => Some(icons::CHAT_QUESTION),
        "comment-edit-outline" => Some(icons::COMMENT_EDIT_OUTLINE),
        "link-variant-remove" => Some(icons::LINK_VARIANT_REMOVE),
        "note-check-outline" => Some(icons::NOTE_CHECK_OUTLINE),
        "pin-off-outline" => Some(icons::PIN_OFF_OUTLINE),
        _ => None,
    }
}
