// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_2(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "alert-octagram-outline" => Some(icons::ALERT_OCTAGRAM_OUTLINE),
        "wiper-wash-alert" => Some(icons::WIPER_WASH_ALERT),
        "phone-alert-outline" => Some(icons::PHONE_ALERT_OUTLINE),
        "download-multiple-outline" => Some(icons::DOWNLOAD_MULTIPLE_OUTLINE),
        "account-wrench" => Some(icons::ACCOUNT_WRENCH),
        "train-car-box-full" => Some(icons::TRAIN_CAR_BOX_FULL),
        "air-purifier" => Some(icons::AIR_PURIFIER),
        "sign-direction-remove" => Some(icons::SIGN_DIRECTION_REMOVE),
        "paw-outline" => Some(icons::PAW_OUTLINE),
        "alert" => Some(icons::ALERT),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "bank-transfer-in" => Some(icons::BANK_TRANSFER_IN),
        #[allow(deprecated)]
        "language-java" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-java' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_JAVA)
        }
        "timer-edit" => Some(icons::TIMER_EDIT),
        "flash-triangle-outline" => Some(icons::FLASH_TRIANGLE_OUTLINE),
        "car-esp" => Some(icons::CAR_ESP),
        "coffee-maker" => Some(icons::COFFEE_MAKER),
        "file-lock-open" => Some(icons::FILE_LOCK_OPEN),
        "food-hot-dog" => Some(icons::FOOD_HOT_DOG),
        "format-text-rotation-down" => Some(icons::FORMAT_TEXT_ROTATION_DOWN),
        "cellphone-nfc-off" => Some(icons::CELLPHONE_NFC_OFF),
        "search-web" => Some(icons::SEARCH_WEB),
        #[allow(deprecated)]
        "language-swift" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-swift' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_SWIFT)
        }
        "star-four-points-circle-outline" => Some(icons::STAR_FOUR_POINTS_CIRCLE_OUTLINE),
        "alpha-d-box" => Some(icons::ALPHA_D_BOX),
        "tooltip-edit-outline" => Some(icons::TOOLTIP_EDIT_OUTLINE),
        "bank-off" => Some(icons::BANK_OFF),
        "violin" => Some(icons::VIOLIN),
        "chevron-double-down" => Some(icons::CHEVRON_DOUBLE_DOWN),
        "refresh" => Some(icons::REFRESH),
        "headphones" => Some(icons::HEADPHONES),
        "bell-badge-outline" => Some(icons::BELL_BADGE_OUTLINE),
        "relation-zero-or-one-to-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_MANY),
        "audio-input-stereo-minijack" => Some(icons::AUDIO_INPUT_STEREO_MINIJACK),
        "format-list-numbered-rtl" => Some(icons::FORMAT_LIST_NUMBERED_RTL),
        "email-remove-outline" => Some(icons::EMAIL_REMOVE_OUTLINE),
        "puzzle-check-outline" => Some(icons::PUZZLE_CHECK_OUTLINE),
        "application-edit-outline" => Some(icons::APPLICATION_EDIT_OUTLINE),
        "timeline-clock" => Some(icons::TIMELINE_CLOCK),
        "alpha-j-box-outline" => Some(icons::ALPHA_J_BOX_OUTLINE),
        "reply-all" => Some(icons::REPLY_ALL),
        "alert-plus-outline" => Some(icons::ALERT_PLUS_OUTLINE),
        "gamepad-circle-up" => Some(icons::GAMEPAD_CIRCLE_UP),
        "function-variant" => Some(icons::FUNCTION_VARIANT),
        "page-next" => Some(icons::PAGE_NEXT),
        "whistle-outline" => Some(icons::WHISTLE_OUTLINE),
        "badge-account" => Some(icons::BADGE_ACCOUNT),
        "beach" => Some(icons::BEACH),
        "projector-screen-off" => Some(icons::PROJECTOR_SCREEN_OFF),
        "tune" => Some(icons::TUNE),
        "train-car-gondola-full" => Some(icons::TRAIN_CAR_GONDOLA_FULL),
        "chart-sankey" => Some(icons::CHART_SANKEY),
        "relation-zero-or-many-to-zero-or-many" => {
            Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_MANY)
        }
        "store-alert-outline" => Some(icons::STORE_ALERT_OUTLINE),
        "timeline-question-outline" => Some(icons::TIMELINE_QUESTION_OUTLINE),
        "glass-stange" => Some(icons::GLASS_STANGE),
        "printer-pos-remove-outline" => Some(icons::PRINTER_POS_REMOVE_OUTLINE),
        "image-broken-variant" => Some(icons::IMAGE_BROKEN_VARIANT),
        "shuffle-disabled" => Some(icons::SHUFFLE_DISABLED),
        "alarm-panel" => Some(icons::ALARM_PANEL),
        "calendar-question" => Some(icons::CALENDAR_QUESTION),
        "chevron-double-up" => Some(icons::CHEVRON_DOUBLE_UP),
        "drawing-box" => Some(icons::DRAWING_BOX),
        "source-pull" => Some(icons::SOURCE_PULL),
        "book-arrow-left-outline" => Some(icons::BOOK_ARROW_LEFT_OUTLINE),
        #[allow(deprecated)]
        "nativescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nativescript' is deprecated.").print(py);
            }
            Some(icons::NATIVESCRIPT)
        }
        "dock-window" => Some(icons::DOCK_WINDOW),
        "home-search" => Some(icons::HOME_SEARCH),
        "eight-track" => Some(icons::EIGHT_TRACK),
        "code-less-than-or-equal" => Some(icons::CODE_LESS_THAN_OR_EQUAL),
        "roman-numeral-4" => Some(icons::ROMAN_NUMERAL_4),
        "clipboard-off" => Some(icons::CLIPBOARD_OFF),
        "seed-outline" => Some(icons::SEED_OUTLINE),
        "shield-refresh-outline" => Some(icons::SHIELD_REFRESH_OUTLINE),
        "format-color-marker-cancel" => Some(icons::FORMAT_COLOR_MARKER_CANCEL),
        "cookie-off-outline" => Some(icons::COOKIE_OFF_OUTLINE),
        "account-off" => Some(icons::ACCOUNT_OFF),
        "checkbox-marked-circle-minus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_MINUS_OUTLINE),
        "video-input-antenna" => Some(icons::VIDEO_INPUT_ANTENNA),
        "numeric-9-plus-box-outline" => Some(icons::NUMERIC_9_PLUS_BOX_OUTLINE),
        "archive-clock-outline" => Some(icons::ARCHIVE_CLOCK_OUTLINE),
        "camera-lock-open-outline" => Some(icons::CAMERA_LOCK_OPEN_OUTLINE),
        "shield-edit" => Some(icons::SHIELD_EDIT),
        "cup-outline" => Some(icons::CUP_OUTLINE),
        "arrow-u-right-bottom-bold" => Some(icons::ARROW_U_RIGHT_BOTTOM_BOLD),
        "puzzle-remove-outline" => Some(icons::PUZZLE_REMOVE_OUTLINE),
        "fridge-variant-outline" => Some(icons::FRIDGE_VARIANT_OUTLINE),
        "store-clock" => Some(icons::STORE_CLOCK),
        "archive-lock-outline" => Some(icons::ARCHIVE_LOCK_OUTLINE),
        "dice-4-outline" => Some(icons::DICE_4_OUTLINE),
        "text-box-edit" => Some(icons::TEXT_BOX_EDIT),
        "cellphone-dock" => Some(icons::CELLPHONE_DOCK),
        "chess-knight" => Some(icons::CHESS_KNIGHT),
        "transit-connection-horizontal" => Some(icons::TRANSIT_CONNECTION_HORIZONTAL),
        "shield-account-outline" => Some(icons::SHIELD_ACCOUNT_OUTLINE),
        "triangle-wave" => Some(icons::TRIANGLE_WAVE),
        "chat-processing-outline" => Some(icons::CHAT_PROCESSING_OUTLINE),
        "bottle-soda-classic-outline" => Some(icons::BOTTLE_SODA_CLASSIC_OUTLINE),
        "arrow-left-right-bold" => Some(icons::ARROW_LEFT_RIGHT_BOLD),
        "email-minus-outline" => Some(icons::EMAIL_MINUS_OUTLINE),
        "folder-eye-outline" => Some(icons::FOLDER_EYE_OUTLINE),
        "bell-check-outline" => Some(icons::BELL_CHECK_OUTLINE),
        "chili-medium-outline" => Some(icons::CHILI_MEDIUM_OUTLINE),
        "file-document-refresh" => Some(icons::FILE_DOCUMENT_REFRESH),
        "home-search-outline" => Some(icons::HOME_SEARCH_OUTLINE),
        "mushroom" => Some(icons::MUSHROOM),
        "label-off-outline" => Some(icons::LABEL_OFF_OUTLINE),
        "tooltip-question" => Some(icons::TOOLTIP_QUESTION),
        "feather" => Some(icons::FEATHER),
        "jeepney" => Some(icons::JEEPNEY),
        "pill-off" => Some(icons::PILL_OFF),
        "calendar-cursor" => Some(icons::CALENDAR_CURSOR),
        "bug-pause" => Some(icons::BUG_PAUSE),
        "controller" => Some(icons::CONTROLLER),
        "comment-bookmark-outline" => Some(icons::COMMENT_BOOKMARK_OUTLINE),
        "eye-lock-open" => Some(icons::EYE_LOCK_OPEN),
        "home-percent-outline" => Some(icons::HOME_PERCENT_OUTLINE),
        "file-cad" => Some(icons::FILE_CAD),
        "tune-vertical-variant" => Some(icons::TUNE_VERTICAL_VARIANT),
        "lock-minus" => Some(icons::LOCK_MINUS),
        #[allow(deprecated)]
        "nodejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nodejs' is deprecated.").print(py);
            }
            Some(icons::NODEJS)
        }
        "robot-confused" => Some(icons::ROBOT_CONFUSED),
        "comment-arrow-left" => Some(icons::COMMENT_ARROW_LEFT),
        "toy-brick-remove-outline" => Some(icons::TOY_BRICK_REMOVE_OUTLINE),
        "monitor-vertical" => Some(icons::MONITOR_VERTICAL),
        "lightning-bolt-circle" => Some(icons::LIGHTNING_BOLT_CIRCLE),
        "invoice-text-remove" => Some(icons::INVOICE_TEXT_REMOVE),
        "eyedropper-off" => Some(icons::EYEDROPPER_OFF),
        "numeric-3-box-multiple" => Some(icons::NUMERIC_3_BOX_MULTIPLE),
        "adjust" => Some(icons::ADJUST),
        "earth-box-plus" => Some(icons::EARTH_BOX_PLUS),
        "select-group" => Some(icons::SELECT_GROUP),
        "paper-cut-vertical" => Some(icons::PAPER_CUT_VERTICAL),
        "calendar-month-outline" => Some(icons::CALENDAR_MONTH_OUTLINE),
        "jellyfish" => Some(icons::JELLYFISH),
        "skip-previous" => Some(icons::SKIP_PREVIOUS),
        "home-clock" => Some(icons::HOME_CLOCK),
        "pier-crane" => Some(icons::PIER_CRANE),
        "upload-network-outline" => Some(icons::UPLOAD_NETWORK_OUTLINE),
        "train-car-box" => Some(icons::TRAIN_CAR_BOX),
        "chart-areaspline-variant" => Some(icons::CHART_AREASPLINE_VARIANT),
        "format-text-wrapping-clip" => Some(icons::FORMAT_TEXT_WRAPPING_CLIP),
        "format-text-rotation-angle-up" => Some(icons::FORMAT_TEXT_ROTATION_ANGLE_UP),
        #[allow(deprecated)]
        "microsoft-onedrive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onedrive' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONEDRIVE)
        }
        "file-settings-outline" => Some(icons::FILE_SETTINGS_OUTLINE),
        "invoice-arrow-left" => Some(icons::INVOICE_ARROW_LEFT),
        "vector-square-plus" => Some(icons::VECTOR_SQUARE_PLUS),
        "flag-off-outline" => Some(icons::FLAG_OFF_OUTLINE),
        "hand-pointing-left" => Some(icons::HAND_POINTING_LEFT),
        "weather-sunny-off" => Some(icons::WEATHER_SUNNY_OFF),
        "relation-zero-or-many-to-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_MANY),
        "network-strength-3" => Some(icons::NETWORK_STRENGTH_3),
        "cookie-off" => Some(icons::COOKIE_OFF),
        "plus-minus" => Some(icons::PLUS_MINUS),
        "solar-power-variant-outline" => Some(icons::SOLAR_POWER_VARIANT_OUTLINE),
        "source-merge" => Some(icons::SOURCE_MERGE),
        "wind-power-outline" => Some(icons::WIND_POWER_OUTLINE),
        "file-word-outline" => Some(icons::FILE_WORD_OUTLINE),
        "audio-video-off" => Some(icons::AUDIO_VIDEO_OFF),
        "format-indent-decrease" => Some(icons::FORMAT_INDENT_DECREASE),
        "tag-search" => Some(icons::TAG_SEARCH),
        "seed-plus-outline" => Some(icons::SEED_PLUS_OUTLINE),
        "file-image-plus-outline" => Some(icons::FILE_IMAGE_PLUS_OUTLINE),
        "alpha-k-box-outline" => Some(icons::ALPHA_K_BOX_OUTLINE),
        "cog-pause-outline" => Some(icons::COG_PAUSE_OUTLINE),
        "message-check-outline" => Some(icons::MESSAGE_CHECK_OUTLINE),
        "bottle-tonic-outline" => Some(icons::BOTTLE_TONIC_OUTLINE),
        "trackpad" => Some(icons::TRACKPAD),
        "database-sync" => Some(icons::DATABASE_SYNC),
        "piggy-bank-outline" => Some(icons::PIGGY_BANK_OUTLINE),
        #[allow(deprecated)]
        "microsoft-sharepoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-sharepoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_SHAREPOINT)
        }
        "keyboard" => Some(icons::KEYBOARD),
        "null" => Some(icons::NULL),
        "trending-up" => Some(icons::TRENDING_UP),
        "information-box" => Some(icons::INFORMATION_BOX),
        "flash-alert-outline" => Some(icons::FLASH_ALERT_OUTLINE),
        "close-circle-multiple-outline" => Some(icons::CLOSE_CIRCLE_MULTIPLE_OUTLINE),
        "at" => Some(icons::AT),
        "symbol" => Some(icons::SYMBOL),
        "shredder" => Some(icons::SHREDDER),
        "alpha-z-box-outline" => Some(icons::ALPHA_Z_BOX_OUTLINE),
        "fireplace-off" => Some(icons::FIREPLACE_OFF),
        "cloud-arrow-down" => Some(icons::CLOUD_ARROW_DOWN),
        "office-building-outline" => Some(icons::OFFICE_BUILDING_OUTLINE),
        "compass-outline" => Some(icons::COMPASS_OUTLINE),
        "emoticon" => Some(icons::EMOTICON),
        "human-male-girl" => Some(icons::HUMAN_MALE_GIRL),
        "lightbulb-on-60" => Some(icons::LIGHTBULB_ON_60),
        "car-light-high" => Some(icons::CAR_LIGHT_HIGH),
        "hand-wave-outline" => Some(icons::HAND_WAVE_OUTLINE),
        "bicycle-electric" => Some(icons::BICYCLE_ELECTRIC),
        "turbine" => Some(icons::TURBINE),
        "lightbulb-auto" => Some(icons::LIGHTBULB_AUTO),
        "database-eye" => Some(icons::DATABASE_EYE),
        "script-text-key-outline" => Some(icons::SCRIPT_TEXT_KEY_OUTLINE),
        "label-outline" => Some(icons::LABEL_OUTLINE),
        "earbuds-off" => Some(icons::EARBUDS_OFF),
        "printer-pos-network-outline" => Some(icons::PRINTER_POS_NETWORK_OUTLINE),
        "car-lifted-pickup" => Some(icons::CAR_LIFTED_PICKUP),
        "printer-wireless" => Some(icons::PRINTER_WIRELESS),
        _ => None,
    }
}
