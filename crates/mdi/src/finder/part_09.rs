// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_9(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "sort-alphabetical-descending-variant" => Some(icons::SORT_ALPHABETICAL_DESCENDING_VARIANT),
        "folder-remove" => Some(icons::FOLDER_REMOVE),
        "keyboard-close" => Some(icons::KEYBOARD_CLOSE),
        "paper-roll" => Some(icons::PAPER_ROLL),
        "alpha-v-circle-outline" => Some(icons::ALPHA_V_CIRCLE_OUTLINE),
        "video-minus" => Some(icons::VIDEO_MINUS),
        "safe" => Some(icons::SAFE),
        "reorder-horizontal" => Some(icons::REORDER_HORIZONTAL),
        "bell-remove-outline" => Some(icons::BELL_REMOVE_OUTLINE),
        "chemical-weapon" => Some(icons::CHEMICAL_WEAPON),
        "calendar-export" => Some(icons::CALENDAR_EXPORT),
        "mouse-variant" => Some(icons::MOUSE_VARIANT),
        "file-rotate-right" => Some(icons::FILE_ROTATE_RIGHT),
        "code-greater-than" => Some(icons::CODE_GREATER_THAN),
        "arrow-left-circle" => Some(icons::ARROW_LEFT_CIRCLE),
        "numeric-4-box-multiple" => Some(icons::NUMERIC_4_BOX_MULTIPLE),
        "sticker-remove" => Some(icons::STICKER_REMOVE),
        "alpha-r-box" => Some(icons::ALPHA_R_BOX),
        "movie-remove" => Some(icons::MOVIE_REMOVE),
        "text-box-multiple" => Some(icons::TEXT_BOX_MULTIPLE),
        "diving-scuba-tank" => Some(icons::DIVING_SCUBA_TANK),
        "hand-peace-variant" => Some(icons::HAND_PEACE_VARIANT),
        "fuse-alert" => Some(icons::FUSE_ALERT),
        "calendar-star-outline" => Some(icons::CALENDAR_STAR_OUTLINE),
        "diving-helmet" => Some(icons::DIVING_HELMET),
        "database-edit" => Some(icons::DATABASE_EDIT),
        "signal-cellular-3" => Some(icons::SIGNAL_CELLULAR_3),
        "filter-plus-outline" => Some(icons::FILTER_PLUS_OUTLINE),
        "spider-web" => Some(icons::SPIDER_WEB),
        "arrow-right" => Some(icons::ARROW_RIGHT),
        "flask-empty-outline" => Some(icons::FLASK_EMPTY_OUTLINE),
        "message-cog-outline" => Some(icons::MESSAGE_COG_OUTLINE),
        "script-text-outline" => Some(icons::SCRIPT_TEXT_OUTLINE),
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
        "aurora" => Some(icons::AURORA),
        "account-card-outline" => Some(icons::ACCOUNT_CARD_OUTLINE),
        "baby-carriage-off" => Some(icons::BABY_CARRIAGE_OFF),
        "ballot-outline" => Some(icons::BALLOT_OUTLINE),
        #[allow(deprecated)]
        "ethereum" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ethereum' is deprecated.").print(py);
            }
            Some(icons::ETHEREUM)
        }
        "certificate" => Some(icons::CERTIFICATE),
        "playlist-star" => Some(icons::PLAYLIST_STAR),
        "klingon" => Some(icons::KLINGON),
        "cloud-lock-outline" => Some(icons::CLOUD_LOCK_OUTLINE),
        "play-network-outline" => Some(icons::PLAY_NETWORK_OUTLINE),
        "cookie-refresh-outline" => Some(icons::COOKIE_REFRESH_OUTLINE),
        "box-cutter-off" => Some(icons::BOX_CUTTER_OFF),
        "golf-cart" => Some(icons::GOLF_CART),
        "timeline-remove-outline" => Some(icons::TIMELINE_REMOVE_OUTLINE),
        "book-arrow-up-outline" => Some(icons::BOOK_ARROW_UP_OUTLINE),
        #[allow(deprecated)]
        "linkedin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linkedin' is deprecated.").print(py);
            }
            Some(icons::LINKEDIN)
        }
        "table-large-remove" => Some(icons::TABLE_LARGE_REMOVE),
        #[allow(deprecated)]
        "language-javascript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-javascript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_JAVASCRIPT)
        }
        "glass-mug-off" => Some(icons::GLASS_MUG_OFF),
        "paw" => Some(icons::PAW),
        "receipt-text-arrow-right-outline" => Some(icons::RECEIPT_TEXT_ARROW_RIGHT_OUTLINE),
        "vector-square-close" => Some(icons::VECTOR_SQUARE_CLOSE),
        #[allow(deprecated)]
        "sass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sass' is deprecated.").print(py);
            }
            Some(icons::SASS)
        }
        "office-building-marker-outline" => Some(icons::OFFICE_BUILDING_MARKER_OUTLINE),
        "cloud-check-variant" => Some(icons::CLOUD_CHECK_VARIANT),
        "clipboard-text-play" => Some(icons::CLIPBOARD_TEXT_PLAY),
        "chart-box-plus-outline" => Some(icons::CHART_BOX_PLUS_OUTLINE),
        "car-brake-abs" => Some(icons::CAR_BRAKE_ABS),
        "source-commit-next-local" => Some(icons::SOURCE_COMMIT_NEXT_LOCAL),
        "numeric-8-box-outline" => Some(icons::NUMERIC_8_BOX_OUTLINE),
        "flask-remove-outline" => Some(icons::FLASK_REMOVE_OUTLINE),
        "sort-alphabetical-ascending-variant" => Some(icons::SORT_ALPHABETICAL_ASCENDING_VARIANT),
        "broadcast" => Some(icons::BROADCAST),
        "axis-y-rotate-counterclockwise" => Some(icons::AXIS_Y_ROTATE_COUNTERCLOCKWISE),
        "view-list" => Some(icons::VIEW_LIST),
        "music-rest-eighth" => Some(icons::MUSIC_REST_EIGHTH),
        "desk" => Some(icons::DESK),
        "file-compare" => Some(icons::FILE_COMPARE),
        "monitor-edit" => Some(icons::MONITOR_EDIT),
        "script-text-key" => Some(icons::SCRIPT_TEXT_KEY),
        "floor-lamp-dual-outline" => Some(icons::FLOOR_LAMP_DUAL_OUTLINE),
        "database-lock" => Some(icons::DATABASE_LOCK),
        "arrow-top-left-bottom-right" => Some(icons::ARROW_TOP_LEFT_BOTTOM_RIGHT),
        "human-greeting" => Some(icons::HUMAN_GREETING),
        "anchor" => Some(icons::ANCHOR),
        "head-sync-outline" => Some(icons::HEAD_SYNC_OUTLINE),
        "periodic-table" => Some(icons::PERIODIC_TABLE),
        "baguette" => Some(icons::BAGUETTE),
        "face-mask-outline" => Some(icons::FACE_MASK_OUTLINE),
        "fruit-pear" => Some(icons::FRUIT_PEAR),
        "tooltip-account" => Some(icons::TOOLTIP_ACCOUNT),
        "server-network" => Some(icons::SERVER_NETWORK),
        "sort-numeric-ascending-variant" => Some(icons::SORT_NUMERIC_ASCENDING_VARIANT),
        "account-group" => Some(icons::ACCOUNT_GROUP),
        "format-color-fill" => Some(icons::FORMAT_COLOR_FILL),
        "book-remove-multiple" => Some(icons::BOOK_REMOVE_MULTIPLE),
        "clipboard-multiple-outline" => Some(icons::CLIPBOARD_MULTIPLE_OUTLINE),
        "file-question-outline" => Some(icons::FILE_QUESTION_OUTLINE),
        "arrow-bottom-left-thick" => Some(icons::ARROW_BOTTOM_LEFT_THICK),
        "comment-question" => Some(icons::COMMENT_QUESTION),
        "run-fast" => Some(icons::RUN_FAST),
        "power-socket-au" => Some(icons::POWER_SOCKET_AU),
        #[allow(deprecated)]
        "vimeo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vimeo' is deprecated.").print(py);
            }
            Some(icons::VIMEO)
        }
        "camera-metering-partial" => Some(icons::CAMERA_METERING_PARTIAL),
        "spear" => Some(icons::SPEAR),
        "arrow-expand-down" => Some(icons::ARROW_EXPAND_DOWN),
        "alphabet-piqad" => Some(icons::ALPHABET_PIQAD),
        "receipt-text-plus" => Some(icons::RECEIPT_TEXT_PLUS),
        "sunglasses" => Some(icons::SUNGLASSES),
        "phone-dial" => Some(icons::PHONE_DIAL),
        "controller" => Some(icons::CONTROLLER),
        "clock-time-one-outline" => Some(icons::CLOCK_TIME_ONE_OUTLINE),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "puzzle-remove" => Some(icons::PUZZLE_REMOVE),
        "train-car-gondola-full" => Some(icons::TRAIN_CAR_GONDOLA_FULL),
        "flag-checkered" => Some(icons::FLAG_CHECKERED),
        "file-powerpoint-box" => Some(icons::FILE_POWERPOINT_BOX),
        "alpha-x-box-outline" => Some(icons::ALPHA_X_BOX_OUTLINE),
        #[allow(deprecated)]
        "pandora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pandora' is deprecated.").print(py);
            }
            Some(icons::PANDORA)
        }
        "bell-alert-outline" => Some(icons::BELL_ALERT_OUTLINE),
        "format-text-wrapping-wrap" => Some(icons::FORMAT_TEXT_WRAPPING_WRAP),
        "alpha-a-box-outline" => Some(icons::ALPHA_A_BOX_OUTLINE),
        "weather-cloudy-clock" => Some(icons::WEATHER_CLOUDY_CLOCK),
        "briefcase-plus" => Some(icons::BRIEFCASE_PLUS),
        "movie-open-remove-outline" => Some(icons::MOVIE_OPEN_REMOVE_OUTLINE),
        "timer-cancel" => Some(icons::TIMER_CANCEL),
        "alpha-d-circle-outline" => Some(icons::ALPHA_D_CIRCLE_OUTLINE),
        "help-circle" => Some(icons::HELP_CIRCLE),
        "headphones-settings" => Some(icons::HEADPHONES_SETTINGS),
        "virus-outline" => Some(icons::VIRUS_OUTLINE),
        "glass-wine" => Some(icons::GLASS_WINE),
        "video-account" => Some(icons::VIDEO_ACCOUNT),
        "alpha-l-box" => Some(icons::ALPHA_L_BOX),
        "human-pregnant" => Some(icons::HUMAN_PREGNANT),
        "account-settings" => Some(icons::ACCOUNT_SETTINGS),
        #[allow(deprecated)]
        "openid" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'openid' is deprecated.").print(py);
            }
            Some(icons::OPENID)
        }
        "sigma" => Some(icons::SIGMA),
        "fridge-industrial-alert-outline" => Some(icons::FRIDGE_INDUSTRIAL_ALERT_OUTLINE),
        "home-search-outline" => Some(icons::HOME_SEARCH_OUTLINE),
        "key-chain-variant" => Some(icons::KEY_CHAIN_VARIANT),
        "backspace" => Some(icons::BACKSPACE),
        "bus-double-decker" => Some(icons::BUS_DOUBLE_DECKER),
        "email-sync-outline" => Some(icons::EMAIL_SYNC_OUTLINE),
        "find-replace" => Some(icons::FIND_REPLACE),
        "star-off" => Some(icons::STAR_OFF),
        "star-three-points" => Some(icons::STAR_THREE_POINTS),
        "alpha-f" => Some(icons::ALPHA_F),
        "picture-in-picture-bottom-right-outline" => {
            Some(icons::PICTURE_IN_PICTURE_BOTTOM_RIGHT_OUTLINE)
        }
        "wall" => Some(icons::WALL),
        "sticker-remove-outline" => Some(icons::STICKER_REMOVE_OUTLINE),
        "book-open-variant-outline" => Some(icons::BOOK_OPEN_VARIANT_OUTLINE),
        "clock-time-twelve-outline" => Some(icons::CLOCK_TIME_TWELVE_OUTLINE),
        "chevron-up-box" => Some(icons::CHEVRON_UP_BOX),
        "coffee-to-go-outline" => Some(icons::COFFEE_TO_GO_OUTLINE),
        "gesture-tap-box" => Some(icons::GESTURE_TAP_BOX),
        "moon-waxing-crescent" => Some(icons::MOON_WAXING_CRESCENT),
        "leaf-circle" => Some(icons::LEAF_CIRCLE),
        "alpha-t-box" => Some(icons::ALPHA_T_BOX),
        "alpha-x-box" => Some(icons::ALPHA_X_BOX),
        "invoice-text-check" => Some(icons::INVOICE_TEXT_CHECK),
        "format-list-checks" => Some(icons::FORMAT_LIST_CHECKS),
        "car-clock" => Some(icons::CAR_CLOCK),
        "relation-only-one-to-many" => Some(icons::RELATION_ONLY_ONE_TO_MANY),
        "moon-last-quarter" => Some(icons::MOON_LAST_QUARTER),
        "weather-moonset-down" => Some(icons::WEATHER_MOONSET_DOWN),
        "briefcase-outline" => Some(icons::BRIEFCASE_OUTLINE),
        "arrow-down-left" => Some(icons::ARROW_DOWN_LEFT),
        "message-text" => Some(icons::MESSAGE_TEXT),
        "cog-off" => Some(icons::COG_OFF),
        "calendar-week-begin-outline" => Some(icons::CALENDAR_WEEK_BEGIN_OUTLINE),
        "file-gif-box" => Some(icons::FILE_GIF_BOX),
        "sun-clock-outline" => Some(icons::SUN_CLOCK_OUTLINE),
        "send-circle" => Some(icons::SEND_CIRCLE),
        "folder-arrow-left" => Some(icons::FOLDER_ARROW_LEFT),
        "chili-off-outline" => Some(icons::CHILI_OFF_OUTLINE),
        "checkerboard" => Some(icons::CHECKERBOARD),
        "contain-start" => Some(icons::CONTAIN_START),
        "bowl-mix-outline" => Some(icons::BOWL_MIX_OUTLINE),
        "code-tags-check" => Some(icons::CODE_TAGS_CHECK),
        "clock-remove" => Some(icons::CLOCK_REMOVE),
        "plus-circle-outline" => Some(icons::PLUS_CIRCLE_OUTLINE),
        "bluetooth-audio" => Some(icons::BLUETOOTH_AUDIO),
        "arrow-right-bold-box" => Some(icons::ARROW_RIGHT_BOLD_BOX),
        "caps-lock" => Some(icons::CAPS_LOCK),
        "lasso" => Some(icons::LASSO),
        "cloud-alert" => Some(icons::CLOUD_ALERT),
        "view-agenda" => Some(icons::VIEW_AGENDA),
        "silo" => Some(icons::SILO),
        "gas-station-in-use-outline" => Some(icons::GAS_STATION_IN_USE_OUTLINE),
        "reflect-horizontal" => Some(icons::REFLECT_HORIZONTAL),
        "bucket-outline" => Some(icons::BUCKET_OUTLINE),
        "contrast-box" => Some(icons::CONTRAST_BOX),
        "bottle-tonic-plus-outline" => Some(icons::BOTTLE_TONIC_PLUS_OUTLINE),
        "tune-vertical-variant" => Some(icons::TUNE_VERTICAL_VARIANT),
        "forum-remove" => Some(icons::FORUM_REMOVE),
        "arrow-right-top-bold" => Some(icons::ARROW_RIGHT_TOP_BOLD),
        "eye-refresh-outline" => Some(icons::EYE_REFRESH_OUTLINE),
        "alpha-q-circle" => Some(icons::ALPHA_Q_CIRCLE),
        "panorama-wide-angle" => Some(icons::PANORAMA_WIDE_ANGLE),
        "phone-log" => Some(icons::PHONE_LOG),
        "chevron-left-circle" => Some(icons::CHEVRON_LEFT_CIRCLE),
        "cloud-key" => Some(icons::CLOUD_KEY),
        "numeric-5-box-multiple-outline" => Some(icons::NUMERIC_5_BOX_MULTIPLE_OUTLINE),
        "blinds-vertical-closed" => Some(icons::BLINDS_VERTICAL_CLOSED),
        "clock-outline" => Some(icons::CLOCK_OUTLINE),
        "archive-edit-outline" => Some(icons::ARCHIVE_EDIT_OUTLINE),
        _ => None,
    }
}
