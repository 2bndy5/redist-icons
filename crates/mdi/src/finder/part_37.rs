// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_37(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        #[allow(deprecated)]
        "unreal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unreal' is deprecated.").print(py);
            }
            Some(icons::UNREAL)
        }
        "format-align-center" => Some(icons::FORMAT_ALIGN_CENTER),
        "transit-skip" => Some(icons::TRANSIT_SKIP),
        "water-off-outline" => Some(icons::WATER_OFF_OUTLINE),
        "traffic-cone" => Some(icons::TRAFFIC_CONE),
        "multiplication-box" => Some(icons::MULTIPLICATION_BOX),
        "guitar-pick" => Some(icons::GUITAR_PICK),
        "receipt-send" => Some(icons::RECEIPT_SEND),
        "numeric-8-box-outline" => Some(icons::NUMERIC_8_BOX_OUTLINE),
        "safe-square" => Some(icons::SAFE_SQUARE),
        "alpha-b" => Some(icons::ALPHA_B),
        "image-size-select-large" => Some(icons::IMAGE_SIZE_SELECT_LARGE),
        "archive-music" => Some(icons::ARCHIVE_MUSIC),
        "heat-wave" => Some(icons::HEAT_WAVE),
        "ray-vertex" => Some(icons::RAY_VERTEX),
        "ocr" => Some(icons::OCR),
        "send" => Some(icons::SEND),
        "barcode-scan" => Some(icons::BARCODE_SCAN),
        "microphone-message" => Some(icons::MICROPHONE_MESSAGE),
        "bike-fast" => Some(icons::BIKE_FAST),
        "clipboard-check-multiple-outline" => Some(icons::CLIPBOARD_CHECK_MULTIPLE_OUTLINE),
        "stove" => Some(icons::STOVE),
        "bike-pedal" => Some(icons::BIKE_PEDAL),
        "heart-search" => Some(icons::HEART_SEARCH),
        "store-edit" => Some(icons::STORE_EDIT),
        "phone-message" => Some(icons::PHONE_MESSAGE),
        "movie-edit" => Some(icons::MOVIE_EDIT),
        "credit-card-wireless-off" => Some(icons::CREDIT_CARD_WIRELESS_OFF),
        "storefront-check-outline" => Some(icons::STOREFRONT_CHECK_OUTLINE),
        "av-timer" => Some(icons::AV_TIMER),
        "set-split" => Some(icons::SET_SPLIT),
        "cash-register" => Some(icons::CASH_REGISTER),
        "audio-video" => Some(icons::AUDIO_VIDEO),
        "headphones-off" => Some(icons::HEADPHONES_OFF),
        "arrow-bottom-left-thick" => Some(icons::ARROW_BOTTOM_LEFT_THICK),
        "fruit-pear" => Some(icons::FRUIT_PEAR),
        "download-box-outline" => Some(icons::DOWNLOAD_BOX_OUTLINE),
        "alpha-a-circle" => Some(icons::ALPHA_A_CIRCLE),
        "gate-and" => Some(icons::GATE_AND),
        "fridge-variant-alert" => Some(icons::FRIDGE_VARIANT_ALERT),
        "wifi-strength-2-lock-open" => Some(icons::WIFI_STRENGTH_2_LOCK_OPEN),
        "chart-arc" => Some(icons::CHART_ARC),
        "file-rotate-right" => Some(icons::FILE_ROTATE_RIGHT),
        "key-chain" => Some(icons::KEY_CHAIN),
        "hiking" => Some(icons::HIKING),
        "fruit-pineapple" => Some(icons::FRUIT_PINEAPPLE),
        "cog-clockwise" => Some(icons::COG_CLOCKWISE),
        _ => None,
    }
}
