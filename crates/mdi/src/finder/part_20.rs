// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_20(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "alert-box" => Some(icons::ALERT_BOX),
        "format-list-bulleted-type" => Some(icons::FORMAT_LIST_BULLETED_TYPE),
        "image-size-select-actual" => Some(icons::IMAGE_SIZE_SELECT_ACTUAL),
        "food-steak" => Some(icons::FOOD_STEAK),
        "traffic-light" => Some(icons::TRAFFIC_LIGHT),
        "unicycle" => Some(icons::UNICYCLE),
        #[allow(deprecated)]
        "openid" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'openid' is deprecated.").print(py);
            }
            Some(icons::OPENID)
        }
        "format-text-rotation-angle-down" => Some(icons::FORMAT_TEXT_ROTATION_ANGLE_DOWN),
        "wall-sconce-flat" => Some(icons::WALL_SCONCE_FLAT),
        "thermometer-low" => Some(icons::THERMOMETER_LOW),
        "briefcase-variant" => Some(icons::BRIEFCASE_VARIANT),
        "image-filter-center-focus" => Some(icons::IMAGE_FILTER_CENTER_FOCUS),
        "baseball-diamond" => Some(icons::BASEBALL_DIAMOND),
        "video-image" => Some(icons::VIDEO_IMAGE),
        "dice-5" => Some(icons::DICE_5),
        "weather-moonset" => Some(icons::WEATHER_MOONSET),
        "math-integral-box" => Some(icons::MATH_INTEGRAL_BOX),
        "calendar-week" => Some(icons::CALENDAR_WEEK),
        "progress-alert" => Some(icons::PROGRESS_ALERT),
        "comment-text" => Some(icons::COMMENT_TEXT),
        "cart-remove" => Some(icons::CART_REMOVE),
        "file-marker" => Some(icons::FILE_MARKER),
        "waveform" => Some(icons::WAVEFORM),
        "lock-question" => Some(icons::LOCK_QUESTION),
        "crop-landscape" => Some(icons::CROP_LANDSCAPE),
        "shield-sync-outline" => Some(icons::SHIELD_SYNC_OUTLINE),
        "auto-download" => Some(icons::AUTO_DOWNLOAD),
        "database-arrow-down" => Some(icons::DATABASE_ARROW_DOWN),
        "violin" => Some(icons::VIOLIN),
        "alpha-a-box" => Some(icons::ALPHA_A_BOX),
        "database-arrow-up" => Some(icons::DATABASE_ARROW_UP),
        #[allow(deprecated)]
        "microsoft-outlook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-outlook' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OUTLOOK)
        }
        "format-color-marker-cancel" => Some(icons::FORMAT_COLOR_MARKER_CANCEL),
        "triangle-wave" => Some(icons::TRIANGLE_WAVE),
        "folder-arrow-right" => Some(icons::FOLDER_ARROW_RIGHT),
        "help-box-outline" => Some(icons::HELP_BOX_OUTLINE),
        "book-minus-outline" => Some(icons::BOOK_MINUS_OUTLINE),
        "notebook-check-outline" => Some(icons::NOTEBOOK_CHECK_OUTLINE),
        #[allow(deprecated)]
        "youtube" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE)
        }
        "calendar-export" => Some(icons::CALENDAR_EXPORT),
        "phone-missed" => Some(icons::PHONE_MISSED),
        "currency-eur" => Some(icons::CURRENCY_EUR),
        "power-socket-de" => Some(icons::POWER_SOCKET_DE),
        "video-box-off" => Some(icons::VIDEO_BOX_OFF),
        "book-remove-multiple-outline" => Some(icons::BOOK_REMOVE_MULTIPLE_OUTLINE),
        "file-image" => Some(icons::FILE_IMAGE),
        "folder-table-outline" => Some(icons::FOLDER_TABLE_OUTLINE),
        "division-box" => Some(icons::DIVISION_BOX),
        "marker-check" => Some(icons::MARKER_CHECK),
        "contrast-box" => Some(icons::CONTRAST_BOX),
        "movie-search" => Some(icons::MOVIE_SEARCH),
        "information-outline" => Some(icons::INFORMATION_OUTLINE),
        "folder-arrow-up-down" => Some(icons::FOLDER_ARROW_UP_DOWN),
        "tennis-ball" => Some(icons::TENNIS_BALL),
        "matrix" => Some(icons::MATRIX),
        "view-gallery" => Some(icons::VIEW_GALLERY),
        "checkbox-multiple-outline" => Some(icons::CHECKBOX_MULTIPLE_OUTLINE),
        "map-marker-left" => Some(icons::MAP_MARKER_LEFT),
        "source-commit" => Some(icons::SOURCE_COMMIT),
        "arrow-down-right-bold" => Some(icons::ARROW_DOWN_RIGHT_BOLD),
        "bank-plus" => Some(icons::BANK_PLUS),
        "robot-love" => Some(icons::ROBOT_LOVE),
        "head-dots-horizontal-outline" => Some(icons::HEAD_DOTS_HORIZONTAL_OUTLINE),
        "numeric-8-circle" => Some(icons::NUMERIC_8_CIRCLE),
        "account-lock-open-outline" => Some(icons::ACCOUNT_LOCK_OPEN_OUTLINE),
        "phone-message-outline" => Some(icons::PHONE_MESSAGE_OUTLINE),
        "satellite-variant" => Some(icons::SATELLITE_VARIANT),
        "credit-card-lock-outline" => Some(icons::CREDIT_CARD_LOCK_OUTLINE),
        "image-filter-center-focus-strong-outline" => {
            Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG_OUTLINE)
        }
        "battery-70-bluetooth" => Some(icons::BATTERY_70_BLUETOOTH),
        "bank-remove" => Some(icons::BANK_REMOVE),
        "apple-keyboard-shift" => Some(icons::APPLE_KEYBOARD_SHIFT),
        "thumb-up-outline" => Some(icons::THUMB_UP_OUTLINE),
        #[allow(deprecated)]
        "steam" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'steam' is deprecated.").print(py);
            }
            Some(icons::STEAM)
        }
        "signal-hspa-plus" => Some(icons::SIGNAL_HSPA_PLUS),
        "calendar-text" => Some(icons::CALENDAR_TEXT),
        "battery-clock-outline" => Some(icons::BATTERY_CLOCK_OUTLINE),
        "tag-text-outline" => Some(icons::TAG_TEXT_OUTLINE),
        "hospital-box-outline" => Some(icons::HOSPITAL_BOX_OUTLINE),
        #[allow(deprecated)]
        "google-analytics" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-analytics' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ANALYTICS)
        }
        #[allow(deprecated)]
        "wechat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wechat' is deprecated.").print(py);
            }
            Some(icons::WECHAT)
        }
        "forum-minus-outline" => Some(icons::FORUM_MINUS_OUTLINE),
        "credit-card-wireless-off-outline" => Some(icons::CREDIT_CARD_WIRELESS_OFF_OUTLINE),
        "heart-settings" => Some(icons::HEART_SETTINGS),
        "dice-d8-outline" => Some(icons::DICE_D8_OUTLINE),
        "boom-gate-arrow-down-outline" => Some(icons::BOOM_GATE_ARROW_DOWN_OUTLINE),
        "code-block-brackets" => Some(icons::CODE_BLOCK_BRACKETS),
        "read" => Some(icons::READ),
        "credit-card-scan" => Some(icons::CREDIT_CARD_SCAN),
        "sort-alphabetical-variant" => Some(icons::SORT_ALPHABETICAL_VARIANT),
        "keyboard-f10" => Some(icons::KEYBOARD_F10),
        "table-headers-eye" => Some(icons::TABLE_HEADERS_EYE),
        "playlist-edit" => Some(icons::PLAYLIST_EDIT),
        "beaker" => Some(icons::BEAKER),
        "chili-mild-outline" => Some(icons::CHILI_MILD_OUTLINE),
        "comment-arrow-right" => Some(icons::COMMENT_ARROW_RIGHT),
        "cloud-arrow-down" => Some(icons::CLOUD_ARROW_DOWN),
        "map-marker-path" => Some(icons::MAP_MARKER_PATH),
        "fridge-industrial-outline" => Some(icons::FRIDGE_INDUSTRIAL_OUTLINE),
        "cloud-search-outline" => Some(icons::CLOUD_SEARCH_OUTLINE),
        "briefcase-edit" => Some(icons::BRIEFCASE_EDIT),
        "sun-thermometer" => Some(icons::SUN_THERMOMETER),
        "coat-rack" => Some(icons::COAT_RACK),
        "tag-multiple" => Some(icons::TAG_MULTIPLE),
        "compass-off" => Some(icons::COMPASS_OFF),
        "gamepad-circle-right" => Some(icons::GAMEPAD_CIRCLE_RIGHT),
        "book-edit-outline" => Some(icons::BOOK_EDIT_OUTLINE),
        "gymnastics" => Some(icons::GYMNASTICS),
        "sphere" => Some(icons::SPHERE),
        "printer-pos-plus-outline" => Some(icons::PRINTER_POS_PLUS_OUTLINE),
        "food-steak-off" => Some(icons::FOOD_STEAK_OFF),
        "baseball-outline" => Some(icons::BASEBALL_OUTLINE),
        "tally-mark-3" => Some(icons::TALLY_MARK_3),
        "brush-variant" => Some(icons::BRUSH_VARIANT),
        "emoticon-happy" => Some(icons::EMOTICON_HAPPY),
        "calendar-minus-outline" => Some(icons::CALENDAR_MINUS_OUTLINE),
        "arrow-down-bold-circle" => Some(icons::ARROW_DOWN_BOLD_CIRCLE),
        "perspective-more" => Some(icons::PERSPECTIVE_MORE),
        "folder-multiple-image" => Some(icons::FOLDER_MULTIPLE_IMAGE),
        "select-color" => Some(icons::SELECT_COLOR),
        "message-text-outline" => Some(icons::MESSAGE_TEXT_OUTLINE),
        "numeric-2-circle-outline" => Some(icons::NUMERIC_2_CIRCLE_OUTLINE),
        "signal-distance-variant" => Some(icons::SIGNAL_DISTANCE_VARIANT),
        "upload-box" => Some(icons::UPLOAD_BOX),
        "arrow-down-thick" => Some(icons::ARROW_DOWN_THICK),
        #[allow(deprecated)]
        "apple-icloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-icloud' is deprecated.").print(py);
            }
            Some(icons::APPLE_ICLOUD)
        }
        "credit-card-chip-outline" => Some(icons::CREDIT_CARD_CHIP_OUTLINE),
        "desktop-classic" => Some(icons::DESKTOP_CLASSIC),
        "account-filter" => Some(icons::ACCOUNT_FILTER),
        "circle-opacity" => Some(icons::CIRCLE_OPACITY),
        "image-move" => Some(icons::IMAGE_MOVE),
        "pig-variant-outline" => Some(icons::PIG_VARIANT_OUTLINE),
        "spider-thread" => Some(icons::SPIDER_THREAD),
        "briefcase-variant-off" => Some(icons::BRIEFCASE_VARIANT_OFF),
        "medication-outline" => Some(icons::MEDICATION_OUTLINE),
        "bookmark-minus" => Some(icons::BOOKMARK_MINUS),
        "face-agent" => Some(icons::FACE_AGENT),
        "train-car-caboose" => Some(icons::TRAIN_CAR_CABOOSE),
        "movie-roll" => Some(icons::MOVIE_ROLL),
        "arrow-down-thin-circle-outline" => Some(icons::ARROW_DOWN_THIN_CIRCLE_OUTLINE),
        "circle-expand" => Some(icons::CIRCLE_EXPAND),
        "hand-back-right-off" => Some(icons::HAND_BACK_RIGHT_OFF),
        "launch" => Some(icons::LAUNCH),
        "store-outline" => Some(icons::STORE_OUTLINE),
        "bookmark-box-outline" => Some(icons::BOOKMARK_BOX_OUTLINE),
        "calendar-blank" => Some(icons::CALENDAR_BLANK),
        "battery-lock-open" => Some(icons::BATTERY_LOCK_OPEN),
        "battery-charging-10" => Some(icons::BATTERY_CHARGING_10),
        "waterfall" => Some(icons::WATERFALL),
        "coffee" => Some(icons::COFFEE),
        "star-check-outline" => Some(icons::STAR_CHECK_OUTLINE),
        "view-quilt-outline" => Some(icons::VIEW_QUILT_OUTLINE),
        "axis-y-arrow-lock" => Some(icons::AXIS_Y_ARROW_LOCK),
        "vector-point" => Some(icons::VECTOR_POINT),
        "network-strength-3-alert" => Some(icons::NETWORK_STRENGTH_3_ALERT),
        "publish-off" => Some(icons::PUBLISH_OFF),
        "white-balance-iridescent" => Some(icons::WHITE_BALANCE_IRIDESCENT),
        "basketball-hoop-outline" => Some(icons::BASKETBALL_HOOP_OUTLINE),
        "chat-minus-outline" => Some(icons::CHAT_MINUS_OUTLINE),
        "cloud-clock" => Some(icons::CLOUD_CLOCK),
        "bell-sleep" => Some(icons::BELL_SLEEP),
        "head-plus" => Some(icons::HEAD_PLUS),
        "texture" => Some(icons::TEXTURE),
        "sort-numeric-descending-variant" => Some(icons::SORT_NUMERIC_DESCENDING_VARIANT),
        "delete-forever-outline" => Some(icons::DELETE_FOREVER_OUTLINE),
        "credit-card-minus" => Some(icons::CREDIT_CARD_MINUS),
        "leak-off" => Some(icons::LEAK_OFF),
        "fire-extinguisher" => Some(icons::FIRE_EXTINGUISHER),
        "book" => Some(icons::BOOK),
        "warehouse" => Some(icons::WAREHOUSE),
        "receipt-text-arrow-left-outline" => Some(icons::RECEIPT_TEXT_ARROW_LEFT_OUTLINE),
        "archive-lock-open-outline" => Some(icons::ARCHIVE_LOCK_OPEN_OUTLINE),
        "tooltip-plus-outline" => Some(icons::TOOLTIP_PLUS_OUTLINE),
        "inbox-arrow-up" => Some(icons::INBOX_ARROW_UP),
        "close-box" => Some(icons::CLOSE_BOX),
        "math-sin" => Some(icons::MATH_SIN),
        "coach-lamp" => Some(icons::COACH_LAMP),
        "note-plus-outline" => Some(icons::NOTE_PLUS_OUTLINE),
        "chart-line" => Some(icons::CHART_LINE),
        "network-strength-3" => Some(icons::NETWORK_STRENGTH_3),
        "microphone-settings" => Some(icons::MICROPHONE_SETTINGS),
        "human-male-board-poll" => Some(icons::HUMAN_MALE_BOARD_POLL),
        "calendar-badge-outline" => Some(icons::CALENDAR_BADGE_OUTLINE),
        "database-arrow-left-outline" => Some(icons::DATABASE_ARROW_LEFT_OUTLINE),
        "tape-measure" => Some(icons::TAPE_MEASURE),
        "arrow-right-drop-circle" => Some(icons::ARROW_RIGHT_DROP_CIRCLE),
        "sign-direction-minus" => Some(icons::SIGN_DIRECTION_MINUS),
        "email-lock" => Some(icons::EMAIL_LOCK),
        "light-switch" => Some(icons::LIGHT_SWITCH),
        "seed-off" => Some(icons::SEED_OFF),
        "cloud-key" => Some(icons::CLOUD_KEY),
        "tag-arrow-left-outline" => Some(icons::TAG_ARROW_LEFT_OUTLINE),
        "folder-edit" => Some(icons::FOLDER_EDIT),
        "battery-20" => Some(icons::BATTERY_20),
        "archive-music-outline" => Some(icons::ARCHIVE_MUSIC_OUTLINE),
        "progress-check" => Some(icons::PROGRESS_CHECK),
        #[allow(deprecated)]
        "aws" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'aws' is deprecated.").print(py);
            }
            Some(icons::AWS)
        }
        #[allow(deprecated)]
        "firebase" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firebase' is deprecated.").print(py);
            }
            Some(icons::FIREBASE)
        }
        "card-plus" => Some(icons::CARD_PLUS),
        "thermometer-chevron-up" => Some(icons::THERMOMETER_CHEVRON_UP),
        _ => None,
    }
}
