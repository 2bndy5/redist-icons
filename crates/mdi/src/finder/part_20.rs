// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_20(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "tire" => Some(icons::TIRE),
        "contain-start" => Some(icons::CONTAIN_START),
        "filter-menu-outline" => Some(icons::FILTER_MENU_OUTLINE),
        "lightbulb-on-80" => Some(icons::LIGHTBULB_ON_80),
        "snowflake-off" => Some(icons::SNOWFLAKE_OFF),
        "numeric-0-box-multiple-outline" => Some(icons::NUMERIC_0_BOX_MULTIPLE_OUTLINE),
        "account-arrow-up" => Some(icons::ACCOUNT_ARROW_UP),
        "wifi-lock" => Some(icons::WIFI_LOCK),
        "fruit-pineapple" => Some(icons::FRUIT_PINEAPPLE),
        "emoticon-lol-outline" => Some(icons::EMOTICON_LOL_OUTLINE),
        "alarm-plus" => Some(icons::ALARM_PLUS),
        "invert-colors" => Some(icons::INVERT_COLORS),
        "hand-front-right-outline" => Some(icons::HAND_FRONT_RIGHT_OUTLINE),
        "rewind-outline" => Some(icons::REWIND_OUTLINE),
        "invoice-text-remove-outline" => Some(icons::INVOICE_TEXT_REMOVE_OUTLINE),
        "rollerblade" => Some(icons::ROLLERBLADE),
        "flag-plus-outline" => Some(icons::FLAG_PLUS_OUTLINE),
        "kettle-steam" => Some(icons::KETTLE_STEAM),
        "account-settings" => Some(icons::ACCOUNT_SETTINGS),
        "checkbox-blank-off-outline" => Some(icons::CHECKBOX_BLANK_OFF_OUTLINE),
        "movie-open" => Some(icons::MOVIE_OPEN),
        "wall-sconce-flat" => Some(icons::WALL_SCONCE_FLAT),
        "email-lock-outline" => Some(icons::EMAIL_LOCK_OUTLINE),
        "arrow-top-right-thick" => Some(icons::ARROW_TOP_RIGHT_THICK),
        #[allow(deprecated)]
        "dropbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dropbox' is deprecated.").print(py);
            }
            Some(icons::DROPBOX)
        }
        "upload-network-outline" => Some(icons::UPLOAD_NETWORK_OUTLINE),
        "usb" => Some(icons::USB),
        "inbox-full-outline" => Some(icons::INBOX_FULL_OUTLINE),
        "alphabet-latin" => Some(icons::ALPHABET_LATIN),
        "clipboard-play-multiple-outline" => Some(icons::CLIPBOARD_PLAY_MULTIPLE_OUTLINE),
        "key" => Some(icons::KEY),
        "car-speed-limiter" => Some(icons::CAR_SPEED_LIMITER),
        "rabbit" => Some(icons::RABBIT),
        "table-minus" => Some(icons::TABLE_MINUS),
        "null" => Some(icons::NULL),
        "airbag" => Some(icons::AIRBAG),
        "format-list-bulleted-type" => Some(icons::FORMAT_LIST_BULLETED_TYPE),
        "robot-off-outline" => Some(icons::ROBOT_OFF_OUTLINE),
        "ph" => Some(icons::PH),
        "home-thermometer-outline" => Some(icons::HOME_THERMOMETER_OUTLINE),
        "high-definition" => Some(icons::HIGH_DEFINITION),
        "bug-pause-outline" => Some(icons::BUG_PAUSE_OUTLINE),
        "hail" => Some(icons::HAIL),
        "gender-non-binary" => Some(icons::GENDER_NON_BINARY),
        "book-off-outline" => Some(icons::BOOK_OFF_OUTLINE),
        "bunk-bed-outline" => Some(icons::BUNK_BED_OUTLINE),
        "metronome-tick" => Some(icons::METRONOME_TICK),
        "heart-box" => Some(icons::HEART_BOX),
        "chart-gantt" => Some(icons::CHART_GANTT),
        "folder-check" => Some(icons::FOLDER_CHECK),
        "phone-log" => Some(icons::PHONE_LOG),
        "snowman" => Some(icons::SNOWMAN),
        "file-swap-outline" => Some(icons::FILE_SWAP_OUTLINE),
        "chart-timeline" => Some(icons::CHART_TIMELINE),
        "view-dashboard-outline" => Some(icons::VIEW_DASHBOARD_OUTLINE),
        "laptop-account" => Some(icons::LAPTOP_ACCOUNT),
        "source-branch-minus" => Some(icons::SOURCE_BRANCH_MINUS),
        "phone-rotate-landscape" => Some(icons::PHONE_ROTATE_LANDSCAPE),
        "home-switch-outline" => Some(icons::HOME_SWITCH_OUTLINE),
        "page-previous-outline" => Some(icons::PAGE_PREVIOUS_OUTLINE),
        "arrow-right-bold-outline" => Some(icons::ARROW_RIGHT_BOLD_OUTLINE),
        "face-man-outline" => Some(icons::FACE_MAN_OUTLINE),
        "storefront" => Some(icons::STOREFRONT),
        "checkbox-blank-badge" => Some(icons::CHECKBOX_BLANK_BADGE),
        "chevron-double-up" => Some(icons::CHEVRON_DOUBLE_UP),
        "gondola" => Some(icons::GONDOLA),
        "sim-off" => Some(icons::SIM_OFF),
        "star-crescent" => Some(icons::STAR_CRESCENT),
        "database-eye-outline" => Some(icons::DATABASE_EYE_OUTLINE),
        "head-dots-horizontal-outline" => Some(icons::HEAD_DOTS_HORIZONTAL_OUTLINE),
        "archive-alert" => Some(icons::ARCHIVE_ALERT),
        "timer-refresh-outline" => Some(icons::TIMER_REFRESH_OUTLINE),
        "image-remove-outline" => Some(icons::IMAGE_REMOVE_OUTLINE),
        "cast" => Some(icons::CAST),
        "paw-off-outline" => Some(icons::PAW_OFF_OUTLINE),
        "water-pump" => Some(icons::WATER_PUMP),
        "home-percent-outline" => Some(icons::HOME_PERCENT_OUTLINE),
        "food-steak-off" => Some(icons::FOOD_STEAK_OFF),
        "alpha-t-box-outline" => Some(icons::ALPHA_T_BOX_OUTLINE),
        "candycane" => Some(icons::CANDYCANE),
        "wiper" => Some(icons::WIPER),
        "creation-outline" => Some(icons::CREATION_OUTLINE),
        "square-off-outline" => Some(icons::SQUARE_OFF_OUTLINE),
        "archive-settings-outline" => Some(icons::ARCHIVE_SETTINGS_OUTLINE),
        "storefront-edit" => Some(icons::STOREFRONT_EDIT),
        "basket-plus-outline" => Some(icons::BASKET_PLUS_OUTLINE),
        "priority-high" => Some(icons::PRIORITY_HIGH),
        "tooth" => Some(icons::TOOTH),
        "alpha-d-circle-outline" => Some(icons::ALPHA_D_CIRCLE_OUTLINE),
        "file-document-refresh-outline" => Some(icons::FILE_DOCUMENT_REFRESH_OUTLINE),
        "message-cog" => Some(icons::MESSAGE_COG),
        "replay" => Some(icons::REPLAY),
        "bus-stop-covered" => Some(icons::BUS_STOP_COVERED),
        "folder-arrow-left-outline" => Some(icons::FOLDER_ARROW_LEFT_OUTLINE),
        "ghost" => Some(icons::GHOST),
        "selection-marker" => Some(icons::SELECTION_MARKER),
        "receipt-text-minus-outline" => Some(icons::RECEIPT_TEXT_MINUS_OUTLINE),
        "weather-partly-snowy" => Some(icons::WEATHER_PARTLY_SNOWY),
        "briefcase-search-outline" => Some(icons::BRIEFCASE_SEARCH_OUTLINE),
        "train-car-passenger-variant" => Some(icons::TRAIN_CAR_PASSENGER_VARIANT),
        "signal-off" => Some(icons::SIGNAL_OFF),
        "gas-station-in-use" => Some(icons::GAS_STATION_IN_USE),
        "battery-minus-variant" => Some(icons::BATTERY_MINUS_VARIANT),
        "delete-empty-outline" => Some(icons::DELETE_EMPTY_OUTLINE),
        "rewind-10" => Some(icons::REWIND_10),
        "leak" => Some(icons::LEAK),
        "home-percent" => Some(icons::HOME_PERCENT),
        "pipe-disconnected" => Some(icons::PIPE_DISCONNECTED),
        "home-flood" => Some(icons::HOME_FLOOD),
        "arrow-top-right-bottom-left-bold" => Some(icons::ARROW_TOP_RIGHT_BOTTOM_LEFT_BOLD),
        "check-decagram" => Some(icons::CHECK_DECAGRAM),
        "induction" => Some(icons::INDUCTION),
        "send-circle" => Some(icons::SEND_CIRCLE),
        "file-undo-outline" => Some(icons::FILE_UNDO_OUTLINE),
        "power-plug-outline" => Some(icons::POWER_PLUG_OUTLINE),
        "opacity" => Some(icons::OPACITY),
        "clipboard-arrow-up-outline" => Some(icons::CLIPBOARD_ARROW_UP_OUTLINE),
        "movie-outline" => Some(icons::MOVIE_OUTLINE),
        "sort-calendar-descending" => Some(icons::SORT_CALENDAR_DESCENDING),
        "account-minus-outline" => Some(icons::ACCOUNT_MINUS_OUTLINE),
        "cloud-refresh" => Some(icons::CLOUD_REFRESH),
        "numeric-6-box-outline" => Some(icons::NUMERIC_6_BOX_OUTLINE),
        #[allow(deprecated)]
        "google-downasaur" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-downasaur' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_DOWNASAUR)
        }
        "store-minus-outline" => Some(icons::STORE_MINUS_OUTLINE),
        "diving" => Some(icons::DIVING),
        "car-lifted-pickup" => Some(icons::CAR_LIFTED_PICKUP),
        "timetable" => Some(icons::TIMETABLE),
        "movie-search" => Some(icons::MOVIE_SEARCH),
        "star-minus-outline" => Some(icons::STAR_MINUS_OUTLINE),
        "clock-time-two" => Some(icons::CLOCK_TIME_TWO),
        "headphones" => Some(icons::HEADPHONES),
        "database-refresh" => Some(icons::DATABASE_REFRESH),
        "vector-polyline-remove" => Some(icons::VECTOR_POLYLINE_REMOVE),
        "note-edit-outline" => Some(icons::NOTE_EDIT_OUTLINE),
        "mortar-pestle-plus" => Some(icons::MORTAR_PESTLE_PLUS),
        "relation-one-to-one-or-many" => Some(icons::RELATION_ONE_TO_ONE_OR_MANY),
        "notebook-plus" => Some(icons::NOTEBOOK_PLUS),
        "sticker-text" => Some(icons::STICKER_TEXT),
        "folder-image" => Some(icons::FOLDER_IMAGE),
        "archive-minus-outline" => Some(icons::ARCHIVE_MINUS_OUTLINE),
        "card-outline" => Some(icons::CARD_OUTLINE),
        "grid" => Some(icons::GRID),
        "dog" => Some(icons::DOG),
        "ray-end-arrow" => Some(icons::RAY_END_ARROW),
        "subway-variant" => Some(icons::SUBWAY_VARIANT),
        "library-outline" => Some(icons::LIBRARY_OUTLINE),
        "baseball-diamond-outline" => Some(icons::BASEBALL_DIAMOND_OUTLINE),
        "arrow-bottom-right" => Some(icons::ARROW_BOTTOM_RIGHT),
        "cog-box" => Some(icons::COG_BOX),
        #[allow(deprecated)]
        "font-awesome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'font-awesome' is deprecated.").print(py);
            }
            Some(icons::FONT_AWESOME)
        }
        "multicast" => Some(icons::MULTICAST),
        "clipboard-outline" => Some(icons::CLIPBOARD_OUTLINE),
        "motion-sensor" => Some(icons::MOTION_SENSOR),
        "invoice-arrow-right-outline" => Some(icons::INVOICE_ARROW_RIGHT_OUTLINE),
        "numeric-7-box" => Some(icons::NUMERIC_7_BOX),
        "numeric-0-box" => Some(icons::NUMERIC_0_BOX),
        "video-input-antenna" => Some(icons::VIDEO_INPUT_ANTENNA),
        "map-marker-path" => Some(icons::MAP_MARKER_PATH),
        "cards-playing-club" => Some(icons::CARDS_PLAYING_CLUB),
        #[allow(deprecated)]
        "microsoft-onedrive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onedrive' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONEDRIVE)
        }
        "all-inclusive-box-outline" => Some(icons::ALL_INCLUSIVE_BOX_OUTLINE),
        #[allow(deprecated)]
        "tailwind" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'tailwind' is deprecated.").print(py);
            }
            Some(icons::TAILWIND)
        }
        "table-heart" => Some(icons::TABLE_HEART),
        "check-underline-circle-outline" => Some(icons::CHECK_UNDERLINE_CIRCLE_OUTLINE),
        "pail-plus-outline" => Some(icons::PAIL_PLUS_OUTLINE),
        "timer-settings-outline" => Some(icons::TIMER_SETTINGS_OUTLINE),
        "relation-one-or-many-to-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONE),
        "thermometer-water" => Some(icons::THERMOMETER_WATER),
        "newspaper-remove" => Some(icons::NEWSPAPER_REMOVE),
        "gift-off" => Some(icons::GIFT_OFF),
        "playlist-star" => Some(icons::PLAYLIST_STAR),
        "file-document-multiple-outline" => Some(icons::FILE_DOCUMENT_MULTIPLE_OUTLINE),
        "tag-edit-outline" => Some(icons::TAG_EDIT_OUTLINE),
        "magnify-minus-outline" => Some(icons::MAGNIFY_MINUS_OUTLINE),
        "chevron-right-circle" => Some(icons::CHEVRON_RIGHT_CIRCLE),
        "archive-outline" => Some(icons::ARCHIVE_OUTLINE),
        "microphone-question" => Some(icons::MICROPHONE_QUESTION),
        "book-arrow-up-outline" => Some(icons::BOOK_ARROW_UP_OUTLINE),
        "archive-cancel" => Some(icons::ARCHIVE_CANCEL),
        "cookie-outline" => Some(icons::COOKIE_OUTLINE),
        "chart-sankey-variant" => Some(icons::CHART_SANKEY_VARIANT),
        "message-settings" => Some(icons::MESSAGE_SETTINGS),
        "toggle-switch-variant" => Some(icons::TOGGLE_SWITCH_VARIANT),
        "waves-arrow-right" => Some(icons::WAVES_ARROW_RIGHT),
        "gamepad-round" => Some(icons::GAMEPAD_ROUND),
        "file-image-minus" => Some(icons::FILE_IMAGE_MINUS),
        "hdr-off" => Some(icons::HDR_OFF),
        "card-account-mail" => Some(icons::CARD_ACCOUNT_MAIL),
        "home-sound-out-outline" => Some(icons::HOME_SOUND_OUT_OUTLINE),
        "table-lock" => Some(icons::TABLE_LOCK),
        "room-service" => Some(icons::ROOM_SERVICE),
        "battery-alert" => Some(icons::BATTERY_ALERT),
        "printer-pos-check-outline" => Some(icons::PRINTER_POS_CHECK_OUTLINE),
        "music-rest-sixteenth" => Some(icons::MUSIC_REST_SIXTEENTH),
        #[allow(deprecated)]
        "google-ads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-ads' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_ADS)
        }
        "arm-flex-outline" => Some(icons::ARM_FLEX_OUTLINE),
        "state-machine" => Some(icons::STATE_MACHINE),
        "dice-d4" => Some(icons::DICE_D4),
        "gradient-horizontal" => Some(icons::GRADIENT_HORIZONTAL),
        "post-outline" => Some(icons::POST_OUTLINE),
        _ => None,
    }
}
