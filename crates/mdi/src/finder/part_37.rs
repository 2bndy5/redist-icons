// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_37(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "unfold-less-vertical" => Some(icons::UNFOLD_LESS_VERTICAL),
        "cpu-32-bit" => Some(icons::CPU_32_BIT),
        "smoke-detector-alert-outline" => Some(icons::SMOKE_DETECTOR_ALERT_OUTLINE),
        "square-medium-outline" => Some(icons::SQUARE_MEDIUM_OUTLINE),
        "battery-check-outline" => Some(icons::BATTERY_CHECK_OUTLINE),
        "alpha-j-box" => Some(icons::ALPHA_J_BOX),
        "chevron-down-circle" => Some(icons::CHEVRON_DOWN_CIRCLE),
        "printer-alert" => Some(icons::PRINTER_ALERT),
        "account-cancel" => Some(icons::ACCOUNT_CANCEL),
        "arrow-left-bottom-bold" => Some(icons::ARROW_LEFT_BOTTOM_BOLD),
        "newspaper-check" => Some(icons::NEWSPAPER_CHECK),
        "battery-charging-wireless-90" => Some(icons::BATTERY_CHARGING_WIRELESS_90),
        "chevron-double-left" => Some(icons::CHEVRON_DOUBLE_LEFT),
        "tag-outline" => Some(icons::TAG_OUTLINE),
        "format-page-break" => Some(icons::FORMAT_PAGE_BREAK),
        "shield-plus" => Some(icons::SHIELD_PLUS),
        "delete-empty" => Some(icons::DELETE_EMPTY),
        "currency-cny" => Some(icons::CURRENCY_CNY),
        "rewind-60" => Some(icons::REWIND_60),
        "water-off" => Some(icons::WATER_OFF),
        "emoticon-happy-outline" => Some(icons::EMOTICON_HAPPY_OUTLINE),
        "window-restore" => Some(icons::WINDOW_RESTORE),
        "message-lock-outline" => Some(icons::MESSAGE_LOCK_OUTLINE),
        "code-greater-than-or-equal" => Some(icons::CODE_GREATER_THAN_OR_EQUAL),
        "file-settings" => Some(icons::FILE_SETTINGS),
        "newspaper-variant-multiple" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE),
        "death-star" => Some(icons::DEATH_STAR),
        "dots-vertical-circle" => Some(icons::DOTS_VERTICAL_CIRCLE),
        #[allow(deprecated)]
        "qqchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qqchat' is deprecated.").print(py);
            }
            Some(icons::QQCHAT)
        }
        "television-shimmer" => Some(icons::TELEVISION_SHIMMER),
        "move-resize-variant" => Some(icons::MOVE_RESIZE_VARIANT),
        "vector-rectangle" => Some(icons::VECTOR_RECTANGLE),
        "lock-off" => Some(icons::LOCK_OFF),
        "checkbox-blank" => Some(icons::CHECKBOX_BLANK),
        "bike-fast" => Some(icons::BIKE_FAST),
        "message-draw" => Some(icons::MESSAGE_DRAW),
        "card-bulleted-settings-outline" => Some(icons::CARD_BULLETED_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-edge" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-edge' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_EDGE)
        }
        "gesture-swipe-up" => Some(icons::GESTURE_SWIPE_UP),
        "webcam" => Some(icons::WEBCAM),
        "volume-off" => Some(icons::VOLUME_OFF),
        "tag-faces" => Some(icons::TAG_FACES),
        #[allow(deprecated)]
        "google-lens" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-lens' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_LENS)
        }
        "water-boiler-auto" => Some(icons::WATER_BOILER_AUTO),
        "ip" => Some(icons::IP),
        "video-3d" => Some(icons::VIDEO_3D),
        "storefront-remove-outline" => Some(icons::STOREFRONT_REMOVE_OUTLINE),
        _ => None,
    }
}
