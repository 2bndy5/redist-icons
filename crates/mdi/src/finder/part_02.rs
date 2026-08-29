// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_2(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "bookshelf" => Some(icons::BOOKSHELF),
        "play-box-lock" => Some(icons::PLAY_BOX_LOCK),
        "archive-clock-outline" => Some(icons::ARCHIVE_CLOCK_OUTLINE),
        "medical-bag" => Some(icons::MEDICAL_BAG),
        "relation-only-one-to-only-one" => Some(icons::RELATION_ONLY_ONE_TO_ONLY_ONE),
        "note-multiple" => Some(icons::NOTE_MULTIPLE),
        "attachment-plus" => Some(icons::ATTACHMENT_PLUS),
        "vector-curve" => Some(icons::VECTOR_CURVE),
        "step-forward" => Some(icons::STEP_FORWARD),
        "vector-square-open" => Some(icons::VECTOR_SQUARE_OPEN),
        "card-plus" => Some(icons::CARD_PLUS),
        "calendar-arrow-left" => Some(icons::CALENDAR_ARROW_LEFT),
        "land-rows-vertical" => Some(icons::LAND_ROWS_VERTICAL),
        "bell-minus" => Some(icons::BELL_MINUS),
        "file-video" => Some(icons::FILE_VIDEO),
        "reorder-vertical" => Some(icons::REORDER_VERTICAL),
        "message-lock-outline" => Some(icons::MESSAGE_LOCK_OUTLINE),
        "firework" => Some(icons::FIREWORK),
        "file-document-plus" => Some(icons::FILE_DOCUMENT_PLUS),
        "weight" => Some(icons::WEIGHT),
        "wifi-settings" => Some(icons::WIFI_SETTINGS),
        "cloud-check-variant-outline" => Some(icons::CLOUD_CHECK_VARIANT_OUTLINE),
        "music-accidental-double-sharp" => Some(icons::MUSIC_ACCIDENTAL_DOUBLE_SHARP),
        "countertop-outline" => Some(icons::COUNTERTOP_OUTLINE),
        "menu-open" => Some(icons::MENU_OPEN),
        "badge-account-alert" => Some(icons::BADGE_ACCOUNT_ALERT),
        "shield-crown" => Some(icons::SHIELD_CROWN),
        "trending-down" => Some(icons::TRENDING_DOWN),
        "cupboard" => Some(icons::CUPBOARD),
        "flag" => Some(icons::FLAG),
        "office-building" => Some(icons::OFFICE_BUILDING),
        "currency-btc" => Some(icons::CURRENCY_BTC),
        "palm-tree" => Some(icons::PALM_TREE),
        "bag-checked" => Some(icons::BAG_CHECKED),
        "debug-step-into" => Some(icons::DEBUG_STEP_INTO),
        "ethernet-off" => Some(icons::ETHERNET_OFF),
        "hamburger-check" => Some(icons::HAMBURGER_CHECK),
        "squeegee" => Some(icons::SQUEEGEE),
        "curling" => Some(icons::CURLING),
        "ellipse-outline" => Some(icons::ELLIPSE_OUTLINE),
        "weather-sunny" => Some(icons::WEATHER_SUNNY),
        "comment-processing-outline" => Some(icons::COMMENT_PROCESSING_OUTLINE),
        "message-check" => Some(icons::MESSAGE_CHECK),
        "file-cancel" => Some(icons::FILE_CANCEL),
        "map-marker-circle" => Some(icons::MAP_MARKER_CIRCLE),
        "archive-music-outline" => Some(icons::ARCHIVE_MUSIC_OUTLINE),
        "trophy" => Some(icons::TROPHY),
        "gate-and" => Some(icons::GATE_AND),
        "lightning-bolt-circle" => Some(icons::LIGHTNING_BOLT_CIRCLE),
        "home-switch-outline" => Some(icons::HOME_SWITCH_OUTLINE),
        "pokeball" => Some(icons::POKEBALL),
        "compass-outline" => Some(icons::COMPASS_OUTLINE),
        "water-plus-outline" => Some(icons::WATER_PLUS_OUTLINE),
        "map-check-outline" => Some(icons::MAP_CHECK_OUTLINE),
        "vector-difference-ba" => Some(icons::VECTOR_DIFFERENCE_BA),
        "ammunition" => Some(icons::AMMUNITION),
        "ceiling-light-multiple-outline" => Some(icons::CEILING_LIGHT_MULTIPLE_OUTLINE),
        "expand-all" => Some(icons::EXPAND_ALL),
        "leaf-off" => Some(icons::LEAF_OFF),
        "waves-arrow-right" => Some(icons::WAVES_ARROW_RIGHT),
        "microphone-variant-off" => Some(icons::MICROPHONE_VARIANT_OFF),
        "inbox-outline" => Some(icons::INBOX_OUTLINE),
        "video-stabilization" => Some(icons::VIDEO_STABILIZATION),
        "alarm-light-off-outline" => Some(icons::ALARM_LIGHT_OFF_OUTLINE),
        "toggle-switch-variant" => Some(icons::TOGGLE_SWITCH_VARIANT),
        "torch" => Some(icons::TORCH),
        "invoice-import-outline" => Some(icons::INVOICE_IMPORT_OUTLINE),
        "rotate-right-variant" => Some(icons::ROTATE_RIGHT_VARIANT),
        "card-off" => Some(icons::CARD_OFF),
        "spade" => Some(icons::SPADE),
        "cookie-check-outline" => Some(icons::COOKIE_CHECK_OUTLINE),
        "thermometer-bluetooth" => Some(icons::THERMOMETER_BLUETOOTH),
        "book-remove" => Some(icons::BOOK_REMOVE),
        "railroad-light" => Some(icons::RAILROAD_LIGHT),
        "battery-70-bluetooth" => Some(icons::BATTERY_70_BLUETOOTH),
        "drag-horizontal-variant" => Some(icons::DRAG_HORIZONTAL_VARIANT),
        "cookie-plus-outline" => Some(icons::COOKIE_PLUS_OUTLINE),
        "hexagon-outline" => Some(icons::HEXAGON_OUTLINE),
        "abugida-devanagari" => Some(icons::ABUGIDA_DEVANAGARI),
        "trending-up" => Some(icons::TRENDING_UP),
        "playlist-remove" => Some(icons::PLAYLIST_REMOVE),
        "archive-sync" => Some(icons::ARCHIVE_SYNC),
        "page-layout-footer" => Some(icons::PAGE_LAYOUT_FOOTER),
        #[allow(deprecated)]
        "pokemon-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pokemon-go' is deprecated.").print(py);
            }
            Some(icons::POKEMON_GO)
        }
        "chair-rolling" => Some(icons::CHAIR_ROLLING),
        "archive-cog-outline" => Some(icons::ARCHIVE_COG_OUTLINE),
        "license" => Some(icons::LICENSE),
        "shield-cross" => Some(icons::SHIELD_CROSS),
        "wifi-arrow-up-down" => Some(icons::WIFI_ARROW_UP_DOWN),
        "phone-alert" => Some(icons::PHONE_ALERT),
        #[allow(deprecated)]
        "quora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'quora' is deprecated.").print(py);
            }
            Some(icons::QUORA)
        }
        "ufo-outline" => Some(icons::UFO_OUTLINE),
        "alpha-k-circle-outline" => Some(icons::ALPHA_K_CIRCLE_OUTLINE),
        "home-plus-outline" => Some(icons::HOME_PLUS_OUTLINE),
        "pipe" => Some(icons::PIPE),
        "weather-snowy" => Some(icons::WEATHER_SNOWY),
        "paperclip-lock" => Some(icons::PAPERCLIP_LOCK),
        "axis-x-rotate-clockwise" => Some(icons::AXIS_X_ROTATE_CLOCKWISE),
        "pyramid-off" => Some(icons::PYRAMID_OFF),
        "sign-language" => Some(icons::SIGN_LANGUAGE),
        "movie-edit-outline" => Some(icons::MOVIE_EDIT_OUTLINE),
        "head-alert-outline" => Some(icons::HEAD_ALERT_OUTLINE),
        #[allow(deprecated)]
        "nintendo-wii" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wii' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WII)
        }
        "square-medium-outline" => Some(icons::SQUARE_MEDIUM_OUTLINE),
        "alert-rhombus-outline" => Some(icons::ALERT_RHOMBUS_OUTLINE),
        "motion-play" => Some(icons::MOTION_PLAY),
        "mixed-reality" => Some(icons::MIXED_REALITY),
        "receipt-text-send" => Some(icons::RECEIPT_TEXT_SEND),
        "debug-step-out" => Some(icons::DEBUG_STEP_OUT),
        "account-details" => Some(icons::ACCOUNT_DETAILS),
        "calendar-cursor" => Some(icons::CALENDAR_CURSOR),
        "battery-off" => Some(icons::BATTERY_OFF),
        "pin-off" => Some(icons::PIN_OFF),
        "alpha-z-box" => Some(icons::ALPHA_Z_BOX),
        "map-clock" => Some(icons::MAP_CLOCK),
        "arrow-bottom-right-bold-box" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX),
        "cellphone-check" => Some(icons::CELLPHONE_CHECK),
        "file-cloud" => Some(icons::FILE_CLOUD),
        "longitude" => Some(icons::LONGITUDE),
        "face-woman" => Some(icons::FACE_WOMAN),
        "weather-cloudy" => Some(icons::WEATHER_CLOUDY),
        "arrow-u-right-top" => Some(icons::ARROW_U_RIGHT_TOP),
        "video-marker" => Some(icons::VIDEO_MARKER),
        "bulkhead-light" => Some(icons::BULKHEAD_LIGHT),
        "scoreboard-outline" => Some(icons::SCOREBOARD_OUTLINE),
        "cellphone-charging" => Some(icons::CELLPHONE_CHARGING),
        "train-car-flatbed" => Some(icons::TRAIN_CAR_FLATBED),
        "bench-back" => Some(icons::BENCH_BACK),
        "trumpet" => Some(icons::TRUMPET),
        "package-variant-closed-minus" => Some(icons::PACKAGE_VARIANT_CLOSED_MINUS),
        "stairs-up" => Some(icons::STAIRS_UP),
        "pill-multiple" => Some(icons::PILL_MULTIPLE),
        "medical-cotton-swab" => Some(icons::MEDICAL_COTTON_SWAB),
        "border-outside" => Some(icons::BORDER_OUTSIDE),
        "phone-outgoing" => Some(icons::PHONE_OUTGOING),
        "check-circle-outline" => Some(icons::CHECK_CIRCLE_OUTLINE),
        "check-network-outline" => Some(icons::CHECK_NETWORK_OUTLINE),
        "map-marker-multiple-outline" => Some(icons::MAP_MARKER_MULTIPLE_OUTLINE),
        "train-car-box-open" => Some(icons::TRAIN_CAR_BOX_OPEN),
        "car-search" => Some(icons::CAR_SEARCH),
        "view-grid-outline" => Some(icons::VIEW_GRID_OUTLINE),
        "timeline-clock-outline" => Some(icons::TIMELINE_CLOCK_OUTLINE),
        "wifi-strength-4-lock" => Some(icons::WIFI_STRENGTH_4_LOCK),
        "monitor-lock" => Some(icons::MONITOR_LOCK),
        "registered-trademark" => Some(icons::REGISTERED_TRADEMARK),
        "chat-processing" => Some(icons::CHAT_PROCESSING),
        "chart-donut-variant" => Some(icons::CHART_DONUT_VARIANT),
        "camera-marker-outline" => Some(icons::CAMERA_MARKER_OUTLINE),
        "rabbit-variant-outline" => Some(icons::RABBIT_VARIANT_OUTLINE),
        "ab-testing" => Some(icons::AB_TESTING),
        "alpha-e" => Some(icons::ALPHA_E),
        "data-matrix-plus" => Some(icons::DATA_MATRIX_PLUS),
        "domain-switch" => Some(icons::DOMAIN_SWITCH),
        "file-cancel-outline" => Some(icons::FILE_CANCEL_OUTLINE),
        "passport-plus" => Some(icons::PASSPORT_PLUS),
        "account-multiple-remove-outline" => Some(icons::ACCOUNT_MULTIPLE_REMOVE_OUTLINE),
        "hand-peace" => Some(icons::HAND_PEACE),
        "oil-lamp" => Some(icons::OIL_LAMP),
        "sticker-text" => Some(icons::STICKER_TEXT),
        "family-tree" => Some(icons::FAMILY_TREE),
        "gesture-tap-hold" => Some(icons::GESTURE_TAP_HOLD),
        "contrast-circle" => Some(icons::CONTRAST_CIRCLE),
        "invoice-edit-outline" => Some(icons::INVOICE_EDIT_OUTLINE),
        "set-none" => Some(icons::SET_NONE),
        "inbox-full" => Some(icons::INBOX_FULL),
        "account-reactivate-outline" => Some(icons::ACCOUNT_REACTIVATE_OUTLINE),
        "calendar-weekend" => Some(icons::CALENDAR_WEEKEND),
        "step-backward-2" => Some(icons::STEP_BACKWARD_2),
        "alert-octagram-outline" => Some(icons::ALERT_OCTAGRAM_OUTLINE),
        "file-link-outline" => Some(icons::FILE_LINK_OUTLINE),
        "zodiac-taurus" => Some(icons::ZODIAC_TAURUS),
        "relation-zero-or-one-to-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_MANY),
        "human-female" => Some(icons::HUMAN_FEMALE),
        "keg" => Some(icons::KEG),
        "battery-bluetooth" => Some(icons::BATTERY_BLUETOOTH),
        "star-plus-outline" => Some(icons::STAR_PLUS_OUTLINE),
        "eyedropper" => Some(icons::EYEDROPPER),
        "pan" => Some(icons::PAN),
        "chess-knight" => Some(icons::CHESS_KNIGHT),
        "arrow-right-bold-hexagon-outline" => Some(icons::ARROW_RIGHT_BOLD_HEXAGON_OUTLINE),
        "alpha-t-box-outline" => Some(icons::ALPHA_T_BOX_OUTLINE),
        "file-arrow-left-right-outline" => Some(icons::FILE_ARROW_LEFT_RIGHT_OUTLINE),
        "star-settings-outline" => Some(icons::STAR_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "simple-icons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'simple-icons' is deprecated.").print(py);
            }
            Some(icons::SIMPLE_ICONS)
        }
        "robot-mower" => Some(icons::ROBOT_MOWER),
        "speedometer-medium" => Some(icons::SPEEDOMETER_MEDIUM),
        "bolt" => Some(icons::BOLT),
        "bug-stop" => Some(icons::BUG_STOP),
        "timer-plus-outline" => Some(icons::TIMER_PLUS_OUTLINE),
        "target-variant" => Some(icons::TARGET_VARIANT),
        #[allow(deprecated)]
        "google-circles-extended" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles-extended' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_EXTENDED)
        }
        "robot-excited" => Some(icons::ROBOT_EXCITED),
        "email-variant" => Some(icons::EMAIL_VARIANT),
        "mirror-rectangle" => Some(icons::MIRROR_RECTANGLE),
        "numeric-8-box-multiple-outline" => Some(icons::NUMERIC_8_BOX_MULTIPLE_OUTLINE),
        "boom-gate-arrow-down-outline" => Some(icons::BOOM_GATE_ARROW_DOWN_OUTLINE),
        "microsoft-xbox-controller-battery-unknown" => {
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_UNKNOWN)
        }
        #[allow(deprecated)]
        "language-cpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-cpp' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CPP)
        }
        "comment-minus-outline" => Some(icons::COMMENT_MINUS_OUTLINE),
        "more" => Some(icons::MORE),
        _ => None,
    }
}
