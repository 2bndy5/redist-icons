// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_22(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "mailbox-up-outline" => Some(icons::MAILBOX_UP_OUTLINE),
        "mailbox-up" => Some(icons::MAILBOX_UP),
        "mailbox" => Some(icons::MAILBOX),
        #[allow(deprecated)]
        "manjaro" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'manjaro' is deprecated.").print(py);
            }
            Some(icons::MANJARO)
        }
        "map-check-outline" => Some(icons::MAP_CHECK_OUTLINE),
        "map-check" => Some(icons::MAP_CHECK),
        "map-clock-outline" => Some(icons::MAP_CLOCK_OUTLINE),
        "map-clock" => Some(icons::MAP_CLOCK),
        "map-legend" => Some(icons::MAP_LEGEND),
        "map-marker-account-outline" => Some(icons::MAP_MARKER_ACCOUNT_OUTLINE),
        "map-marker-account" => Some(icons::MAP_MARKER_ACCOUNT),
        "map-marker-alert-outline" => Some(icons::MAP_MARKER_ALERT_OUTLINE),
        "map-marker-alert" => Some(icons::MAP_MARKER_ALERT),
        "map-marker-check-outline" => Some(icons::MAP_MARKER_CHECK_OUTLINE),
        "map-marker-check" => Some(icons::MAP_MARKER_CHECK),
        "map-marker-circle" => Some(icons::MAP_MARKER_CIRCLE),
        "map-marker-distance" => Some(icons::MAP_MARKER_DISTANCE),
        "map-marker-down" => Some(icons::MAP_MARKER_DOWN),
        "map-marker-left-outline" => Some(icons::MAP_MARKER_LEFT_OUTLINE),
        "map-marker-left" => Some(icons::MAP_MARKER_LEFT),
        "map-marker-minus-outline" => Some(icons::MAP_MARKER_MINUS_OUTLINE),
        "map-marker-minus" => Some(icons::MAP_MARKER_MINUS),
        "map-marker-multiple-outline" => Some(icons::MAP_MARKER_MULTIPLE_OUTLINE),
        "map-marker-multiple" => Some(icons::MAP_MARKER_MULTIPLE),
        "map-marker-off-outline" => Some(icons::MAP_MARKER_OFF_OUTLINE),
        "map-marker-off" => Some(icons::MAP_MARKER_OFF),
        "map-marker-outline" => Some(icons::MAP_MARKER_OUTLINE),
        "map-marker-path" => Some(icons::MAP_MARKER_PATH),
        "map-marker-plus-outline" => Some(icons::MAP_MARKER_PLUS_OUTLINE),
        "map-marker-plus" => Some(icons::MAP_MARKER_PLUS),
        "map-marker-question-outline" => Some(icons::MAP_MARKER_QUESTION_OUTLINE),
        "map-marker-question" => Some(icons::MAP_MARKER_QUESTION),
        "map-marker-radius-outline" => Some(icons::MAP_MARKER_RADIUS_OUTLINE),
        "map-marker-radius" => Some(icons::MAP_MARKER_RADIUS),
        "map-marker-remove-outline" => Some(icons::MAP_MARKER_REMOVE_OUTLINE),
        "map-marker-remove-variant" => Some(icons::MAP_MARKER_REMOVE_VARIANT),
        "map-marker-remove" => Some(icons::MAP_MARKER_REMOVE),
        "map-marker-right-outline" => Some(icons::MAP_MARKER_RIGHT_OUTLINE),
        "map-marker-right" => Some(icons::MAP_MARKER_RIGHT),
        "map-marker-star-outline" => Some(icons::MAP_MARKER_STAR_OUTLINE),
        "map-marker-star" => Some(icons::MAP_MARKER_STAR),
        "map-marker-up" => Some(icons::MAP_MARKER_UP),
        "map-marker" => Some(icons::MAP_MARKER),
        "map-minus" => Some(icons::MAP_MINUS),
        "map-outline" => Some(icons::MAP_OUTLINE),
        "map-plus" => Some(icons::MAP_PLUS),
        "map-search-outline" => Some(icons::MAP_SEARCH_OUTLINE),
        "map-search" => Some(icons::MAP_SEARCH),
        "map" => Some(icons::MAP),
        #[allow(deprecated)]
        "mapbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mapbox' is deprecated.").print(py);
            }
            Some(icons::MAPBOX)
        }
        "margin" => Some(icons::MARGIN),
        "marker-cancel" => Some(icons::MARKER_CANCEL),
        "marker-check" => Some(icons::MARKER_CHECK),
        "marker" => Some(icons::MARKER),
        #[allow(deprecated)]
        "mastodon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mastodon' is deprecated.").print(py);
            }
            Some(icons::MASTODON)
        }
        #[allow(deprecated)]
        "material-design" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-design' is deprecated.")
                    .print(py);
            }
            Some(icons::MATERIAL_DESIGN)
        }
        #[allow(deprecated)]
        "material-ui" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-ui' is deprecated.").print(py);
            }
            Some(icons::MATERIAL_UI)
        }
        "math-compass" => Some(icons::MATH_COMPASS),
        "math-cos" => Some(icons::MATH_COS),
        "math-integral-box" => Some(icons::MATH_INTEGRAL_BOX),
        "math-integral" => Some(icons::MATH_INTEGRAL),
        "math-log" => Some(icons::MATH_LOG),
        "math-norm-box" => Some(icons::MATH_NORM_BOX),
        "math-norm" => Some(icons::MATH_NORM),
        "math-sin" => Some(icons::MATH_SIN),
        "math-tan" => Some(icons::MATH_TAN),
        "matrix" => Some(icons::MATRIX),
        "medal-outline" => Some(icons::MEDAL_OUTLINE),
        "medal" => Some(icons::MEDAL),
        "medical-bag" => Some(icons::MEDICAL_BAG),
        "medical-cotton-swab" => Some(icons::MEDICAL_COTTON_SWAB),
        "medication-outline" => Some(icons::MEDICATION_OUTLINE),
        "medication" => Some(icons::MEDICATION),
        "meditation" => Some(icons::MEDITATION),
        "memory-arrow-down" => Some(icons::MEMORY_ARROW_DOWN),
        "memory" => Some(icons::MEMORY),
        "menorah-fire" => Some(icons::MENORAH_FIRE),
        "menorah" => Some(icons::MENORAH),
        "menu-close" => Some(icons::MENU_CLOSE),
        "menu-down-outline" => Some(icons::MENU_DOWN_OUTLINE),
        "menu-down" => Some(icons::MENU_DOWN),
        "menu-left-outline" => Some(icons::MENU_LEFT_OUTLINE),
        "menu-left" => Some(icons::MENU_LEFT),
        "menu-open" => Some(icons::MENU_OPEN),
        "menu-right-outline" => Some(icons::MENU_RIGHT_OUTLINE),
        "menu-right" => Some(icons::MENU_RIGHT),
        "menu-swap-outline" => Some(icons::MENU_SWAP_OUTLINE),
        "menu-swap" => Some(icons::MENU_SWAP),
        "menu-up-outline" => Some(icons::MENU_UP_OUTLINE),
        "menu-up" => Some(icons::MENU_UP),
        "menu" => Some(icons::MENU),
        "merge" => Some(icons::MERGE),
        "message-alert-outline" => Some(icons::MESSAGE_ALERT_OUTLINE),
        "message-alert" => Some(icons::MESSAGE_ALERT),
        "message-arrow-left-outline" => Some(icons::MESSAGE_ARROW_LEFT_OUTLINE),
        "message-arrow-left" => Some(icons::MESSAGE_ARROW_LEFT),
        "message-arrow-right-outline" => Some(icons::MESSAGE_ARROW_RIGHT_OUTLINE),
        "message-arrow-right" => Some(icons::MESSAGE_ARROW_RIGHT),
        "message-badge-outline" => Some(icons::MESSAGE_BADGE_OUTLINE),
        "message-badge" => Some(icons::MESSAGE_BADGE),
        "message-bookmark-outline" => Some(icons::MESSAGE_BOOKMARK_OUTLINE),
        "message-bookmark" => Some(icons::MESSAGE_BOOKMARK),
        "message-bulleted-off" => Some(icons::MESSAGE_BULLETED_OFF),
        "message-bulleted" => Some(icons::MESSAGE_BULLETED),
        "message-check-outline" => Some(icons::MESSAGE_CHECK_OUTLINE),
        "message-check" => Some(icons::MESSAGE_CHECK),
        "message-cog-outline" => Some(icons::MESSAGE_COG_OUTLINE),
        "message-cog" => Some(icons::MESSAGE_COG),
        "message-draw" => Some(icons::MESSAGE_DRAW),
        "message-fast-outline" => Some(icons::MESSAGE_FAST_OUTLINE),
        "message-fast" => Some(icons::MESSAGE_FAST),
        "message-flash-outline" => Some(icons::MESSAGE_FLASH_OUTLINE),
        "message-flash" => Some(icons::MESSAGE_FLASH),
        "message-image-outline" => Some(icons::MESSAGE_IMAGE_OUTLINE),
        "message-image" => Some(icons::MESSAGE_IMAGE),
        "message-lock-outline" => Some(icons::MESSAGE_LOCK_OUTLINE),
        "message-lock" => Some(icons::MESSAGE_LOCK),
        "message-minus-outline" => Some(icons::MESSAGE_MINUS_OUTLINE),
        "message-minus" => Some(icons::MESSAGE_MINUS),
        "message-off-outline" => Some(icons::MESSAGE_OFF_OUTLINE),
        "message-off" => Some(icons::MESSAGE_OFF),
        "message-outline" => Some(icons::MESSAGE_OUTLINE),
        "message-plus-outline" => Some(icons::MESSAGE_PLUS_OUTLINE),
        "message-plus" => Some(icons::MESSAGE_PLUS),
        "message-processing-outline" => Some(icons::MESSAGE_PROCESSING_OUTLINE),
        "message-processing" => Some(icons::MESSAGE_PROCESSING),
        "message-question-outline" => Some(icons::MESSAGE_QUESTION_OUTLINE),
        "message-question" => Some(icons::MESSAGE_QUESTION),
        "message-reply-outline" => Some(icons::MESSAGE_REPLY_OUTLINE),
        "message-reply-text-outline" => Some(icons::MESSAGE_REPLY_TEXT_OUTLINE),
        "message-reply-text" => Some(icons::MESSAGE_REPLY_TEXT),
        "message-reply" => Some(icons::MESSAGE_REPLY),
        "message-settings-outline" => Some(icons::MESSAGE_SETTINGS_OUTLINE),
        "message-settings" => Some(icons::MESSAGE_SETTINGS),
        "message-star-outline" => Some(icons::MESSAGE_STAR_OUTLINE),
        "message-star" => Some(icons::MESSAGE_STAR),
        "message-text-clock-outline" => Some(icons::MESSAGE_TEXT_CLOCK_OUTLINE),
        "message-text-clock" => Some(icons::MESSAGE_TEXT_CLOCK),
        "message-text-fast-outline" => Some(icons::MESSAGE_TEXT_FAST_OUTLINE),
        "message-text-fast" => Some(icons::MESSAGE_TEXT_FAST),
        "message-text-lock-outline" => Some(icons::MESSAGE_TEXT_LOCK_OUTLINE),
        "message-text-lock" => Some(icons::MESSAGE_TEXT_LOCK),
        "message-text-outline" => Some(icons::MESSAGE_TEXT_OUTLINE),
        "message-text" => Some(icons::MESSAGE_TEXT),
        "message-video" => Some(icons::MESSAGE_VIDEO),
        "message" => Some(icons::MESSAGE),
        #[allow(deprecated)]
        "meteor" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'meteor' is deprecated.").print(py);
            }
            Some(icons::METEOR)
        }
        "meter-electric-outline" => Some(icons::METER_ELECTRIC_OUTLINE),
        "meter-electric" => Some(icons::METER_ELECTRIC),
        "meter-gas-outline" => Some(icons::METER_GAS_OUTLINE),
        "meter-gas" => Some(icons::METER_GAS),
        "metronome-tick" => Some(icons::METRONOME_TICK),
        "metronome" => Some(icons::METRONOME),
        "micro-sd" => Some(icons::MICRO_SD),
        "microphone-message-off" => Some(icons::MICROPHONE_MESSAGE_OFF),
        "microphone-message" => Some(icons::MICROPHONE_MESSAGE),
        "microphone-minus" => Some(icons::MICROPHONE_MINUS),
        "microphone-off" => Some(icons::MICROPHONE_OFF),
        "microphone-outline" => Some(icons::MICROPHONE_OUTLINE),
        "microphone-plus" => Some(icons::MICROPHONE_PLUS),
        "microphone-question-outline" => Some(icons::MICROPHONE_QUESTION_OUTLINE),
        "microphone-question" => Some(icons::MICROPHONE_QUESTION),
        "microphone-settings" => Some(icons::MICROPHONE_SETTINGS),
        "microphone-variant-off" => Some(icons::MICROPHONE_VARIANT_OFF),
        "microphone-variant" => Some(icons::MICROPHONE_VARIANT),
        "microphone" => Some(icons::MICROPHONE),
        "microscope" => Some(icons::MICROSCOPE),
        #[allow(deprecated)]
        "microsoft-access" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-access' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ACCESS)
        }
        #[allow(deprecated)]
        "microsoft-azure-devops" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure-devops' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE_DEVOPS)
        }
        #[allow(deprecated)]
        "microsoft-azure" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE)
        }
        #[allow(deprecated)]
        "microsoft-bing" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-bing' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_BING)
        }
        #[allow(deprecated)]
        "microsoft-dynamics-365" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-dynamics-365' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_DYNAMICS_365)
        }
        #[allow(deprecated)]
        "microsoft-edge" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-edge' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_EDGE)
        }
        #[allow(deprecated)]
        "microsoft-excel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-excel' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_EXCEL)
        }
        #[allow(deprecated)]
        "microsoft-internet-explorer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-internet-explorer' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_INTERNET_EXPLORER)
        }
        #[allow(deprecated)]
        "microsoft-office" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-office' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OFFICE)
        }
        #[allow(deprecated)]
        "microsoft-onedrive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onedrive' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONEDRIVE)
        }
        #[allow(deprecated)]
        "microsoft-onenote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onenote' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONENOTE)
        }
        #[allow(deprecated)]
        "microsoft-outlook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-outlook' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OUTLOOK)
        }
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        #[allow(deprecated)]
        "microsoft-sharepoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-sharepoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_SHAREPOINT)
        }
        #[allow(deprecated)]
        "microsoft-teams" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-teams' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_TEAMS)
        }
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
        #[allow(deprecated)]
        "microsoft-visual-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-visual-studio' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_VISUAL_STUDIO)
        }
        #[allow(deprecated)]
        "microsoft-windows-classic" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-windows-classic' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS_CLASSIC)
        }
        #[allow(deprecated)]
        "microsoft-windows" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-windows' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS)
        }
        #[allow(deprecated)]
        "microsoft-word" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-word' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_WORD)
        }
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
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-charging" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-charging' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_CHARGING)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-empty" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-empty' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_EMPTY)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-full" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-full' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_FULL)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-low" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-low' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_LOW)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-medium" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-medium' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_MEDIUM)
        }
        "microsoft-xbox-controller-battery-unknown" => {
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_UNKNOWN)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller-menu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-menu' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_MENU)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller-off" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-off' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_OFF)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-view' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_VIEW)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER)
        }
        #[allow(deprecated)]
        "microsoft-xbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-xbox' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_XBOX)
        }
        #[allow(deprecated)]
        "microsoft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT)
        }
        _ => None,
    }
}
