// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_25(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "star-half" => Some(icons::STAR_HALF),
        "lock-open-plus-outline" => Some(icons::LOCK_OPEN_PLUS_OUTLINE),
        "sticker-plus" => Some(icons::STICKER_PLUS),
        "alpha-p-box" => Some(icons::ALPHA_P_BOX),
        "battery-arrow-down-outline" => Some(icons::BATTERY_ARROW_DOWN_OUTLINE),
        "altimeter" => Some(icons::ALTIMETER),
        "sun-clock-outline" => Some(icons::SUN_CLOCK_OUTLINE),
        "induction" => Some(icons::INDUCTION),
        "lipstick" => Some(icons::LIPSTICK),
        "arrow-bottom-left-thick" => Some(icons::ARROW_BOTTOM_LEFT_THICK),
        "arrow-bottom-right-bold-box" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX),
        #[allow(deprecated)]
        "home-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'home-assistant' is deprecated.").print(py);
            }
            Some(icons::HOME_ASSISTANT)
        }
        "infinity" => Some(icons::INFINITY),
        "folder-arrow-left-right" => Some(icons::FOLDER_ARROW_LEFT_RIGHT),
        "book-education-outline" => Some(icons::BOOK_EDUCATION_OUTLINE),
        "access-point-check" => Some(icons::ACCESS_POINT_CHECK),
        "package-variant-closed-remove" => Some(icons::PACKAGE_VARIANT_CLOSED_REMOVE),
        "phone-hangup-outline" => Some(icons::PHONE_HANGUP_OUTLINE),
        "pail-outline" => Some(icons::PAIL_OUTLINE),
        "account-circle" => Some(icons::ACCOUNT_CIRCLE),
        "bed" => Some(icons::BED),
        "food-variant-off" => Some(icons::FOOD_VARIANT_OFF),
        "file-delimited-outline" => Some(icons::FILE_DELIMITED_OUTLINE),
        "folder-off" => Some(icons::FOLDER_OFF),
        "tune" => Some(icons::TUNE),
        "camera-wireless-outline" => Some(icons::CAMERA_WIRELESS_OUTLINE),
        #[allow(deprecated)]
        "mapbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mapbox' is deprecated.").print(py);
            }
            Some(icons::MAPBOX)
        }
        #[allow(deprecated)]
        "youtube-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-studio' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_STUDIO)
        }
        "music-box-multiple-outline" => Some(icons::MUSIC_BOX_MULTIPLE_OUTLINE),
        "lock-open-alert" => Some(icons::LOCK_OPEN_ALERT),
        "sofa-outline" => Some(icons::SOFA_OUTLINE),
        "overscan" => Some(icons::OVERSCAN),
        "map-legend" => Some(icons::MAP_LEGEND),
        "lock-pattern" => Some(icons::LOCK_PATTERN),
        "traffic-light-outline" => Some(icons::TRAFFIC_LIGHT_OUTLINE),
        "link-box-variant-outline" => Some(icons::LINK_BOX_VARIANT_OUTLINE),
        "parachute" => Some(icons::PARACHUTE),
        "timer-lock-open-outline" => Some(icons::TIMER_LOCK_OPEN_OUTLINE),
        "sun-compass" => Some(icons::SUN_COMPASS),
        "key-alert" => Some(icons::KEY_ALERT),
        "cards" => Some(icons::CARDS),
        "battery-off-outline" => Some(icons::BATTERY_OFF_OUTLINE),
        "bed-clock" => Some(icons::BED_CLOCK),
        "music-note-quarter-dotted" => Some(icons::MUSIC_NOTE_QUARTER_DOTTED),
        "key-alert-outline" => Some(icons::KEY_ALERT_OUTLINE),
        "message-image" => Some(icons::MESSAGE_IMAGE),
        "puzzle" => Some(icons::PUZZLE),
        "castle" => Some(icons::CASTLE),
        "database-export" => Some(icons::DATABASE_EXPORT),
        #[allow(deprecated)]
        "language-html5" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-html5' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_HTML5)
        }
        "format-list-checkbox" => Some(icons::FORMAT_LIST_CHECKBOX),
        "led-variant-off" => Some(icons::LED_VARIANT_OFF),
        #[allow(deprecated)]
        "gmail" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gmail' is deprecated.").print(py);
            }
            Some(icons::GMAIL)
        }
        "wave-undercurrent" => Some(icons::WAVE_UNDERCURRENT),
        "chart-pie" => Some(icons::CHART_PIE),
        #[allow(deprecated)]
        "language-markdown-outline" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'language-markdown-outline' is deprecated.",
                )
                .print(py);
            }
            Some(icons::LANGUAGE_MARKDOWN_OUTLINE)
        }
        "alpha-a" => Some(icons::ALPHA_A),
        "invoice-text-fast-outline" => Some(icons::INVOICE_TEXT_FAST_OUTLINE),
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "ray-end-arrow" => Some(icons::RAY_END_ARROW),
        "folder-arrow-up-down-outline" => Some(icons::FOLDER_ARROW_UP_DOWN_OUTLINE),
        "information-slab-circle" => Some(icons::INFORMATION_SLAB_CIRCLE),
        #[allow(deprecated)]
        "google-earth" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-earth' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_EARTH)
        }
        "download-box-outline" => Some(icons::DOWNLOAD_BOX_OUTLINE),
        "emoticon" => Some(icons::EMOTICON),
        "cpu-32-bit" => Some(icons::CPU_32_BIT),
        "remote-tv-off" => Some(icons::REMOTE_TV_OFF),
        "sort-bool-ascending" => Some(icons::SORT_BOOL_ASCENDING),
        "briefcase-account" => Some(icons::BRIEFCASE_ACCOUNT),
        #[allow(deprecated)]
        "language-fortran" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-fortran' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_FORTRAN)
        }
        "radiology-box-outline" => Some(icons::RADIOLOGY_BOX_OUTLINE),
        "alpha-a-box-outline" => Some(icons::ALPHA_A_BOX_OUTLINE),
        "pen-remove" => Some(icons::PEN_REMOVE),
        "nfc-variant-off" => Some(icons::NFC_VARIANT_OFF),
        "home-off" => Some(icons::HOME_OFF),
        "table-border" => Some(icons::TABLE_BORDER),
        "data-matrix-remove" => Some(icons::DATA_MATRIX_REMOVE),
        "vector-point-select" => Some(icons::VECTOR_POINT_SELECT),
        "office-building-plus-outline" => Some(icons::OFFICE_BUILDING_PLUS_OUTLINE),
        "face-man" => Some(icons::FACE_MAN),
        "relation-zero-or-one-to-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE),
        "phone-plus" => Some(icons::PHONE_PLUS),
        "power-sleep" => Some(icons::POWER_SLEEP),
        "credit-card-lock-outline" => Some(icons::CREDIT_CARD_LOCK_OUTLINE),
        "account-key-outline" => Some(icons::ACCOUNT_KEY_OUTLINE),
        "beaker-check" => Some(icons::BEAKER_CHECK),
        "land-rows-horizontal" => Some(icons::LAND_ROWS_HORIZONTAL),
        "calendar-filter-outline" => Some(icons::CALENDAR_FILTER_OUTLINE),
        "football-helmet" => Some(icons::FOOTBALL_HELMET),
        "battery-off" => Some(icons::BATTERY_OFF),
        "rename-box-outline" => Some(icons::RENAME_BOX_OUTLINE),
        "information-variant-box-outline" => Some(icons::INFORMATION_VARIANT_BOX_OUTLINE),
        "eye-plus-outline" => Some(icons::EYE_PLUS_OUTLINE),
        "clipboard-text-play-outline" => Some(icons::CLIPBOARD_TEXT_PLAY_OUTLINE),
        #[allow(deprecated)]
        "google-fit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-fit' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_FIT)
        }
        "hamburger-off" => Some(icons::HAMBURGER_OFF),
        "information-slab-box-outline" => Some(icons::INFORMATION_SLAB_BOX_OUTLINE),
        "battery-high" => Some(icons::BATTERY_HIGH),
        "headset-off" => Some(icons::HEADSET_OFF),
        "square-rounded-badge-outline" => Some(icons::SQUARE_ROUNDED_BADGE_OUTLINE),
        "car-brake-hold" => Some(icons::CAR_BRAKE_HOLD),
        "numeric-4-circle" => Some(icons::NUMERIC_4_CIRCLE),
        "receipt-text-clock-outline" => Some(icons::RECEIPT_TEXT_CLOCK_OUTLINE),
        #[allow(deprecated)]
        "webpack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webpack' is deprecated.").print(py);
            }
            Some(icons::WEBPACK)
        }
        "checkbox-blank-circle-outline" => Some(icons::CHECKBOX_BLANK_CIRCLE_OUTLINE),
        "cards-playing-heart" => Some(icons::CARDS_PLAYING_HEART),
        "face-agent" => Some(icons::FACE_AGENT),
        "axis-arrow-lock" => Some(icons::AXIS_ARROW_LOCK),
        "tab-unselected" => Some(icons::TAB_UNSELECTED),
        "texture-box" => Some(icons::TEXTURE_BOX),
        "bookmark-multiple" => Some(icons::BOOKMARK_MULTIPLE),
        "unfold-more-vertical" => Some(icons::UNFOLD_MORE_VERTICAL),
        "calendar-blank-outline" => Some(icons::CALENDAR_BLANK_OUTLINE),
        "ev-plug-chademo" => Some(icons::EV_PLUG_CHADEMO),
        #[allow(deprecated)]
        "vlc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vlc' is deprecated.").print(py);
            }
            Some(icons::VLC)
        }
        "tooltip-plus-outline" => Some(icons::TOOLTIP_PLUS_OUTLINE),
        "battery-charging-100" => Some(icons::BATTERY_CHARGING_100),
        "butterfly-outline" => Some(icons::BUTTERFLY_OUTLINE),
        "movie-open-play" => Some(icons::MOVIE_OPEN_PLAY),
        "calculator-variant" => Some(icons::CALCULATOR_VARIANT),
        "calendar-text-outline" => Some(icons::CALENDAR_TEXT_OUTLINE),
        "arrow-right-circle" => Some(icons::ARROW_RIGHT_CIRCLE),
        "clipboard-text-clock" => Some(icons::CLIPBOARD_TEXT_CLOCK),
        "head-heart" => Some(icons::HEAD_HEART),
        "heart-plus-outline" => Some(icons::HEART_PLUS_OUTLINE),
        "folder-remove" => Some(icons::FOLDER_REMOVE),
        "archive-off" => Some(icons::ARCHIVE_OFF),
        "account-supervisor-outline" => Some(icons::ACCOUNT_SUPERVISOR_OUTLINE),
        "table-edit" => Some(icons::TABLE_EDIT),
        #[allow(deprecated)]
        "waze" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'waze' is deprecated.").print(py);
            }
            Some(icons::WAZE)
        }
        "water-boiler-off" => Some(icons::WATER_BOILER_OFF),
        "wallet-plus" => Some(icons::WALLET_PLUS),
        "shield-remove-outline" => Some(icons::SHIELD_REMOVE_OUTLINE),
        "stamper" => Some(icons::STAMPER),
        "star-box-multiple-outline" => Some(icons::STAR_BOX_MULTIPLE_OUTLINE),
        "upload-network-outline" => Some(icons::UPLOAD_NETWORK_OUTLINE),
        "folder-pound" => Some(icons::FOLDER_POUND),
        "note-off" => Some(icons::NOTE_OFF),
        "bullet" => Some(icons::BULLET),
        "timer-check" => Some(icons::TIMER_CHECK),
        "gesture-swipe-vertical" => Some(icons::GESTURE_SWIPE_VERTICAL),
        "bottle-tonic-skull-outline" => Some(icons::BOTTLE_TONIC_SKULL_OUTLINE),
        "airplane-search" => Some(icons::AIRPLANE_SEARCH),
        "nature-people" => Some(icons::NATURE_PEOPLE),
        "cards-club" => Some(icons::CARDS_CLUB),
        "account-voice-off" => Some(icons::ACCOUNT_VOICE_OFF),
        "calendar-import" => Some(icons::CALENDAR_IMPORT),
        "battery-60" => Some(icons::BATTERY_60),
        "comment-arrow-right-outline" => Some(icons::COMMENT_ARROW_RIGHT_OUTLINE),
        "axis-x-rotate-clockwise" => Some(icons::AXIS_X_ROTATE_CLOCKWISE),
        "receipt-text-minus" => Some(icons::RECEIPT_TEXT_MINUS),
        "send-lock" => Some(icons::SEND_LOCK),
        "head-minus" => Some(icons::HEAD_MINUS),
        "wifi-lock-open" => Some(icons::WIFI_LOCK_OPEN),
        "cart-arrow-down" => Some(icons::CART_ARROW_DOWN),
        "shield-key" => Some(icons::SHIELD_KEY),
        "newspaper-variant-outline" => Some(icons::NEWSPAPER_VARIANT_OUTLINE),
        "drag-variant" => Some(icons::DRAG_VARIANT),
        "baby" => Some(icons::BABY),
        "flask-off-outline" => Some(icons::FLASK_OFF_OUTLINE),
        "alpha-e-circle-outline" => Some(icons::ALPHA_E_CIRCLE_OUTLINE),
        "segment" => Some(icons::SEGMENT),
        "message-arrow-right-outline" => Some(icons::MESSAGE_ARROW_RIGHT_OUTLINE),
        "calculator" => Some(icons::CALCULATOR),
        "clock-time-twelve" => Some(icons::CLOCK_TIME_TWELVE),
        "truck-fast" => Some(icons::TRUCK_FAST),
        "fruit-pineapple" => Some(icons::FRUIT_PINEAPPLE),
        "earbuds" => Some(icons::EARBUDS),
        "squeegee" => Some(icons::SQUEEGEE),
        "upload-circle-outline" => Some(icons::UPLOAD_CIRCLE_OUTLINE),
        "server-outline" => Some(icons::SERVER_OUTLINE),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "arrow-all" => Some(icons::ARROW_ALL),
        "book-plus-multiple" => Some(icons::BOOK_PLUS_MULTIPLE),
        "pencil-ruler" => Some(icons::PENCIL_RULER),
        "database-arrow-right" => Some(icons::DATABASE_ARROW_RIGHT),
        "book-play-outline" => Some(icons::BOOK_PLAY_OUTLINE),
        "eight-track" => Some(icons::EIGHT_TRACK),
        "battery-70-bluetooth" => Some(icons::BATTERY_70_BLUETOOTH),
        "format-annotation-minus" => Some(icons::FORMAT_ANNOTATION_MINUS),
        "led-on" => Some(icons::LED_ON),
        "yeast" => Some(icons::YEAST),
        "weather-moonset-up" => Some(icons::WEATHER_MOONSET_UP),
        "bookmark-remove-outline" => Some(icons::BOOKMARK_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "google-street-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-street-view' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_STREET_VIEW)
        }
        "sort-reverse-variant" => Some(icons::SORT_REVERSE_VARIANT),
        "radio-am" => Some(icons::RADIO_AM),
        "leaf-maple" => Some(icons::LEAF_MAPLE),
        "store-check-outline" => Some(icons::STORE_CHECK_OUTLINE),
        "window-shutter-cog" => Some(icons::WINDOW_SHUTTER_COG),
        "skate" => Some(icons::SKATE),
        "paperclip-check" => Some(icons::PAPERCLIP_CHECK),
        "content-save-edit-outline" => Some(icons::CONTENT_SAVE_EDIT_OUTLINE),
        "sort-clock-ascending-outline" => Some(icons::SORT_CLOCK_ASCENDING_OUTLINE),
        "alert-plus-outline" => Some(icons::ALERT_PLUS_OUTLINE),
        "chevron-triple-up" => Some(icons::CHEVRON_TRIPLE_UP),
        "arrow-left-bold-outline" => Some(icons::ARROW_LEFT_BOLD_OUTLINE),
        "content-save-all" => Some(icons::CONTENT_SAVE_ALL),
        "folder-plus-outline" => Some(icons::FOLDER_PLUS_OUTLINE),
        "cookie-cog" => Some(icons::COOKIE_COG),
        _ => None,
    }
}
