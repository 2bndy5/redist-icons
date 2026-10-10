// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_21(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "crown-circle" => Some(icons::CROWN_CIRCLE),
        "format-list-checkbox" => Some(icons::FORMAT_LIST_CHECKBOX),
        "inbox-multiple-outline" => Some(icons::INBOX_MULTIPLE_OUTLINE),
        "fountain-pen-tip" => Some(icons::FOUNTAIN_PEN_TIP),
        "arrow-bottom-right-bold-outline" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_OUTLINE),
        "briefcase-download" => Some(icons::BRIEFCASE_DOWNLOAD),
        "snowflake-melt" => Some(icons::SNOWFLAKE_MELT),
        "calendar-question" => Some(icons::CALENDAR_QUESTION),
        "coffee-to-go-outline" => Some(icons::COFFEE_TO_GO_OUTLINE),
        "set-right" => Some(icons::SET_RIGHT),
        "battery-lock-open" => Some(icons::BATTERY_LOCK_OPEN),
        "book-cog" => Some(icons::BOOK_COG),
        "file-star" => Some(icons::FILE_STAR),
        "expansion-card-variant" => Some(icons::EXPANSION_CARD_VARIANT),
        "map-marker-question" => Some(icons::MAP_MARKER_QUESTION),
        "microphone-message" => Some(icons::MICROPHONE_MESSAGE),
        "book" => Some(icons::BOOK),
        #[allow(deprecated)]
        "salesforce" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'salesforce' is deprecated.").print(py);
            }
            Some(icons::SALESFORCE)
        }
        "file-marker-outline" => Some(icons::FILE_MARKER_OUTLINE),
        "table-clock" => Some(icons::TABLE_CLOCK),
        "format-wrap-tight" => Some(icons::FORMAT_WRAP_TIGHT),
        "fan" => Some(icons::FAN),
        "map-marker-off" => Some(icons::MAP_MARKER_OFF),
        "truck-remove-outline" => Some(icons::TRUCK_REMOVE_OUTLINE),
        "view-parallel" => Some(icons::VIEW_PARALLEL),
        "roman-numeral-8" => Some(icons::ROMAN_NUMERAL_8),
        "floor-lamp-torchiere-variant" => Some(icons::FLOOR_LAMP_TORCHIERE_VARIANT),
        "format-letter-spacing-variant" => Some(icons::FORMAT_LETTER_SPACING_VARIANT),
        "filter-check" => Some(icons::FILTER_CHECK),
        "camera-metering-matrix" => Some(icons::CAMERA_METERING_MATRIX),
        "timer-sand-full" => Some(icons::TIMER_SAND_FULL),
        "signal-3g" => Some(icons::SIGNAL_3G),
        "database-eye" => Some(icons::DATABASE_EYE),
        "book-off" => Some(icons::BOOK_OFF),
        "fuse-blade" => Some(icons::FUSE_BLADE),
        "tennis" => Some(icons::TENNIS),
        "storage-tank" => Some(icons::STORAGE_TANK),
        #[allow(deprecated)]
        "jsfiddle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jsfiddle' is deprecated.").print(py);
            }
            Some(icons::JSFIDDLE)
        }
        "file-table-outline" => Some(icons::FILE_TABLE_OUTLINE),
        "numeric-2-circle" => Some(icons::NUMERIC_2_CIRCLE),
        "chart-bubble" => Some(icons::CHART_BUBBLE),
        "help-circle-outline" => Some(icons::HELP_CIRCLE_OUTLINE),
        "cards-playing-heart" => Some(icons::CARDS_PLAYING_HEART),
        "wifi-strength-2-alert" => Some(icons::WIFI_STRENGTH_2_ALERT),
        "close-network-outline" => Some(icons::CLOSE_NETWORK_OUTLINE),
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
        "email-outline" => Some(icons::EMAIL_OUTLINE),
        "label-off-outline" => Some(icons::LABEL_OFF_OUTLINE),
        "volcano-outline" => Some(icons::VOLCANO_OUTLINE),
        "bell-check-outline" => Some(icons::BELL_CHECK_OUTLINE),
        "tshirt-crew" => Some(icons::TSHIRT_CREW),
        "cached" => Some(icons::CACHED),
        "fridge-variant-alert-outline" => Some(icons::FRIDGE_VARIANT_ALERT_OUTLINE),
        "trademark" => Some(icons::TRADEMARK),
        "eye-refresh" => Some(icons::EYE_REFRESH),
        #[allow(deprecated)]
        "drupal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'drupal' is deprecated.").print(py);
            }
            Some(icons::DRUPAL)
        }
        "credit-card-fast" => Some(icons::CREDIT_CARD_FAST),
        "sim-alert" => Some(icons::SIM_ALERT),
        "home-circle-outline" => Some(icons::HOME_CIRCLE_OUTLINE),
        "format-letter-case" => Some(icons::FORMAT_LETTER_CASE),
        "cellphone-charging" => Some(icons::CELLPHONE_CHARGING),
        #[allow(deprecated)]
        "youtube-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-studio' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_STUDIO)
        }
        "store-plus-outline" => Some(icons::STORE_PLUS_OUTLINE),
        "coffin" => Some(icons::COFFIN),
        "numeric-2-box-multiple" => Some(icons::NUMERIC_2_BOX_MULTIPLE),
        "alpha-x-circle" => Some(icons::ALPHA_X_CIRCLE),
        "message-processing" => Some(icons::MESSAGE_PROCESSING),
        "archive-eye" => Some(icons::ARCHIVE_EYE),
        "lightbulb-on-90" => Some(icons::LIGHTBULB_ON_90),
        "calendar-clock-outline" => Some(icons::CALENDAR_CLOCK_OUTLINE),
        "border-radius" => Some(icons::BORDER_RADIUS),
        "filter-off-outline" => Some(icons::FILTER_OFF_OUTLINE),
        "home-sound-in" => Some(icons::HOME_SOUND_IN),
        "bomb-off" => Some(icons::BOMB_OFF),
        "palette-outline" => Some(icons::PALETTE_OUTLINE),
        "account-lock-outline" => Some(icons::ACCOUNT_LOCK_OUTLINE),
        "message-star" => Some(icons::MESSAGE_STAR),
        "comment-plus" => Some(icons::COMMENT_PLUS),
        "water-remove-outline" => Some(icons::WATER_REMOVE_OUTLINE),
        "cloud-arrow-up" => Some(icons::CLOUD_ARROW_UP),
        "package-check" => Some(icons::PACKAGE_CHECK),
        "hamburger-off" => Some(icons::HAMBURGER_OFF),
        "dice-d8-outline" => Some(icons::DICE_D8_OUTLINE),
        "paper-cut-vertical" => Some(icons::PAPER_CUT_VERTICAL),
        "select-drag" => Some(icons::SELECT_DRAG),
        "timer-minus" => Some(icons::TIMER_MINUS),
        "mother-nurse" => Some(icons::MOTHER_NURSE),
        "briefcase-arrow-left-right-outline" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT_OUTLINE),
        "battery-charging-wireless-60" => Some(icons::BATTERY_CHARGING_WIRELESS_60),
        "inbox-arrow-up-outline" => Some(icons::INBOX_ARROW_UP_OUTLINE),
        "bug-play-outline" => Some(icons::BUG_PLAY_OUTLINE),
        "table-border" => Some(icons::TABLE_BORDER),
        "multimedia" => Some(icons::MULTIMEDIA),
        "power-socket" => Some(icons::POWER_SOCKET),
        "reiterate" => Some(icons::REITERATE),
        "fit-to-screen" => Some(icons::FIT_TO_SCREEN),
        "badge-account" => Some(icons::BADGE_ACCOUNT),
        "bluetooth-audio" => Some(icons::BLUETOOTH_AUDIO),
        "information-box" => Some(icons::INFORMATION_BOX),
        "chat-alert" => Some(icons::CHAT_ALERT),
        "human-greeting-variant" => Some(icons::HUMAN_GREETING_VARIANT),
        "key-plus" => Some(icons::KEY_PLUS),
        "folder-refresh" => Some(icons::FOLDER_REFRESH),
        "content-paste" => Some(icons::CONTENT_PASTE),
        "account-cog" => Some(icons::ACCOUNT_COG),
        "calendar-multiple-check" => Some(icons::CALENDAR_MULTIPLE_CHECK),
        "account-child" => Some(icons::ACCOUNT_CHILD),
        "set-split" => Some(icons::SET_SPLIT),
        "power" => Some(icons::POWER),
        "alpha-u" => Some(icons::ALPHA_U),
        "car-3-plus" => Some(icons::CAR_3_PLUS),
        "gamepad-variant-outline" => Some(icons::GAMEPAD_VARIANT_OUTLINE),
        "vector-polygon" => Some(icons::VECTOR_POLYGON),
        "cup" => Some(icons::CUP),
        "arrow-right-drop-circle-outline" => Some(icons::ARROW_RIGHT_DROP_CIRCLE_OUTLINE),
        "download-lock" => Some(icons::DOWNLOAD_LOCK),
        "message-bookmark" => Some(icons::MESSAGE_BOOKMARK),
        "boom-gate-up-outline" => Some(icons::BOOM_GATE_UP_OUTLINE),
        "printer-pos-stop" => Some(icons::PRINTER_POS_STOP),
        "cash-plus" => Some(icons::CASH_PLUS),
        "toggle-switch-off-outline" => Some(icons::TOGGLE_SWITCH_OFF_OUTLINE),
        "help-box-multiple-outline" => Some(icons::HELP_BOX_MULTIPLE_OUTLINE),
        "axis-z-arrow-lock" => Some(icons::AXIS_Z_ARROW_LOCK),
        "volume-high" => Some(icons::VOLUME_HIGH),
        "cylinder-off" => Some(icons::CYLINDER_OFF),
        #[allow(deprecated)]
        "google-analytics" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-analytics' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ANALYTICS)
        }
        "account-wrench-outline" => Some(icons::ACCOUNT_WRENCH_OUTLINE),
        "comment-eye" => Some(icons::COMMENT_EYE),
        "circle-expand" => Some(icons::CIRCLE_EXPAND),
        #[allow(deprecated)]
        "kubernetes" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kubernetes' is deprecated.").print(py);
            }
            Some(icons::KUBERNETES)
        }
        "map-marker-star-outline" => Some(icons::MAP_MARKER_STAR_OUTLINE),
        "hand-back-right" => Some(icons::HAND_BACK_RIGHT),
        "alpha-d-circle" => Some(icons::ALPHA_D_CIRCLE),
        "solar-power" => Some(icons::SOLAR_POWER),
        #[allow(deprecated)]
        "cordova" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cordova' is deprecated.").print(py);
            }
            Some(icons::CORDOVA)
        }
        "archive-alert-outline" => Some(icons::ARCHIVE_ALERT_OUTLINE),
        "box-cutter" => Some(icons::BOX_CUTTER),
        "call-merge" => Some(icons::CALL_MERGE),
        "tea-outline" => Some(icons::TEA_OUTLINE),
        "message-image-outline" => Some(icons::MESSAGE_IMAGE_OUTLINE),
        "drawing" => Some(icons::DRAWING),
        "square-root" => Some(icons::SQUARE_ROOT),
        "store-alert" => Some(icons::STORE_ALERT),
        "pause-circle-outline" => Some(icons::PAUSE_CIRCLE_OUTLINE),
        "receipt-text-minus" => Some(icons::RECEIPT_TEXT_MINUS),
        "filter-minus-outline" => Some(icons::FILTER_MINUS_OUTLINE),
        "brush-variant" => Some(icons::BRUSH_VARIANT),
        "vector-circle-variant" => Some(icons::VECTOR_CIRCLE_VARIANT),
        "image-size-select-actual" => Some(icons::IMAGE_SIZE_SELECT_ACTUAL),
        "pail-minus" => Some(icons::PAIL_MINUS),
        "augmented-reality" => Some(icons::AUGMENTED_REALITY),
        "menorah-fire" => Some(icons::MENORAH_FIRE),
        "shield-airplane" => Some(icons::SHIELD_AIRPLANE),
        "transfer" => Some(icons::TRANSFER),
        "ethernet-cable-off" => Some(icons::ETHERNET_CABLE_OFF),
        "hand-back-left" => Some(icons::HAND_BACK_LEFT),
        "rewind-15" => Some(icons::REWIND_15),
        "timeline-clock-outline" => Some(icons::TIMELINE_CLOCK_OUTLINE),
        "pencil-ruler" => Some(icons::PENCIL_RULER),
        "information-box-outline" => Some(icons::INFORMATION_BOX_OUTLINE),
        "star-plus-outline" => Some(icons::STAR_PLUS_OUTLINE),
        "pine-tree-variant-outline" => Some(icons::PINE_TREE_VARIANT_OUTLINE),
        "alpha-y" => Some(icons::ALPHA_Y),
        "cookie-off-outline" => Some(icons::COOKIE_OFF_OUTLINE),
        "fast-forward-5" => Some(icons::FAST_FORWARD_5),
        "monitor-shimmer" => Some(icons::MONITOR_SHIMMER),
        "human-female-dance" => Some(icons::HUMAN_FEMALE_DANCE),
        "battery-80" => Some(icons::BATTERY_80),
        "flask" => Some(icons::FLASK),
        "egg-outline" => Some(icons::EGG_OUTLINE),
        "trash-can-outline" => Some(icons::TRASH_CAN_OUTLINE),
        "ray-start-arrow" => Some(icons::RAY_START_ARROW),
        "calendar-edit" => Some(icons::CALENDAR_EDIT),
        "file-rotate-left" => Some(icons::FILE_ROTATE_LEFT),
        "clipboard-multiple-outline" => Some(icons::CLIPBOARD_MULTIPLE_OUTLINE),
        "arrow-top-right-bottom-left" => Some(icons::ARROW_TOP_RIGHT_BOTTOM_LEFT),
        "logout-variant" => Some(icons::LOGOUT_VARIANT),
        "account-box-edit-outline" => Some(icons::ACCOUNT_BOX_EDIT_OUTLINE),
        "car-info" => Some(icons::CAR_INFO),
        "shield-alert" => Some(icons::SHIELD_ALERT),
        "human-scooter" => Some(icons::HUMAN_SCOOTER),
        "signal-cellular-2" => Some(icons::SIGNAL_CELLULAR_2),
        "signal-distance-variant" => Some(icons::SIGNAL_DISTANCE_VARIANT),
        "quality-low" => Some(icons::QUALITY_LOW),
        "skip-next-circle-outline" => Some(icons::SKIP_NEXT_CIRCLE_OUTLINE),
        "battery-outline" => Some(icons::BATTERY_OUTLINE),
        "trophy" => Some(icons::TROPHY),
        "theme-light-dark" => Some(icons::THEME_LIGHT_DARK),
        "router-wireless" => Some(icons::ROUTER_WIRELESS),
        "motorbike" => Some(icons::MOTORBIKE),
        "mixed-reality" => Some(icons::MIXED_REALITY),
        "cash" => Some(icons::CASH),
        "keyboard-f10" => Some(icons::KEYBOARD_F10),
        "bell-cancel-outline" => Some(icons::BELL_CANCEL_OUTLINE),
        "alert-plus-outline" => Some(icons::ALERT_PLUS_OUTLINE),
        "tshirt-v-outline" => Some(icons::TSHIRT_V_OUTLINE),
        "arrow-u-down-right-bold" => Some(icons::ARROW_U_DOWN_RIGHT_BOLD),
        "tray-plus" => Some(icons::TRAY_PLUS),
        "sign-caution" => Some(icons::SIGN_CAUTION),
        "watermark" => Some(icons::WATERMARK),
        _ => None,
    }
}
