// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_8(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "badge-account" => Some(icons::BADGE_ACCOUNT),
        "folder-settings-outline" => Some(icons::FOLDER_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "git" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'git' is deprecated.").print(py);
            }
            Some(icons::GIT)
        }
        "toy-brick-outline" => Some(icons::TOY_BRICK_OUTLINE),
        "penguin" => Some(icons::PENGUIN),
        "flag-outline" => Some(icons::FLAG_OUTLINE),
        "shield-sun" => Some(icons::SHIELD_SUN),
        #[allow(deprecated)]
        "youtube-subscription" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-subscription' is deprecated.")
                    .print(py);
            }
            Some(icons::YOUTUBE_SUBSCRIPTION)
        }
        "folder-arrow-right-outline" => Some(icons::FOLDER_ARROW_RIGHT_OUTLINE),
        "file-document-refresh" => Some(icons::FILE_DOCUMENT_REFRESH),
        "cursor-default-click-outline" => Some(icons::CURSOR_DEFAULT_CLICK_OUTLINE),
        "arrow-down-bold-box-outline" => Some(icons::ARROW_DOWN_BOLD_BOX_OUTLINE),
        "zip-disk" => Some(icons::ZIP_DISK),
        #[allow(deprecated)]
        "sony-playstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sony-playstation' is deprecated.")
                    .print(py);
            }
            Some(icons::SONY_PLAYSTATION)
        }
        "elevator" => Some(icons::ELEVATOR),
        "plus-thick" => Some(icons::PLUS_THICK),
        "lan-check" => Some(icons::LAN_CHECK),
        "tag-edit-outline" => Some(icons::TAG_EDIT_OUTLINE),
        "guitar-pick-outline" => Some(icons::GUITAR_PICK_OUTLINE),
        "grass" => Some(icons::GRASS),
        "music-note-outline" => Some(icons::MUSIC_NOTE_OUTLINE),
        "vacuum" => Some(icons::VACUUM),
        "bread-slice-outline" => Some(icons::BREAD_SLICE_OUTLINE),
        "ab-testing" => Some(icons::AB_TESTING),
        "tooltip-edit-outline" => Some(icons::TOOLTIP_EDIT_OUTLINE),
        "set-none" => Some(icons::SET_NONE),
        "tray-alert" => Some(icons::TRAY_ALERT),
        "folder-account-outline" => Some(icons::FOLDER_ACCOUNT_OUTLINE),
        #[allow(deprecated)]
        "nfc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nfc' is deprecated.").print(py);
            }
            Some(icons::NFC)
        }
        "blinds-vertical-closed" => Some(icons::BLINDS_VERTICAL_CLOSED),
        "lock-off" => Some(icons::LOCK_OFF),
        "chevron-left-circle" => Some(icons::CHEVRON_LEFT_CIRCLE),
        "numeric-3-box-multiple" => Some(icons::NUMERIC_3_BOX_MULTIPLE),
        "clipboard-check" => Some(icons::CLIPBOARD_CHECK),
        "printer-pos-network" => Some(icons::PRINTER_POS_NETWORK),
        "home-battery" => Some(icons::HOME_BATTERY),
        "land-plots-circle" => Some(icons::LAND_PLOTS_CIRCLE),
        "clock-time-ten-outline" => Some(icons::CLOCK_TIME_TEN_OUTLINE),
        "alpha-t-box" => Some(icons::ALPHA_T_BOX),
        "scanner" => Some(icons::SCANNER),
        "spider" => Some(icons::SPIDER),
        #[allow(deprecated)]
        "nativescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nativescript' is deprecated.").print(py);
            }
            Some(icons::NATIVESCRIPT)
        }
        "multimedia" => Some(icons::MULTIMEDIA),
        "ear-hearing-loop" => Some(icons::EAR_HEARING_LOOP),
        "weather-partly-snowy" => Some(icons::WEATHER_PARTLY_SNOWY),
        "lock-open-outline" => Some(icons::LOCK_OPEN_OUTLINE),
        "controller-classic" => Some(icons::CONTROLLER_CLASSIC),
        "play-box-multiple" => Some(icons::PLAY_BOX_MULTIPLE),
        "cards-playing-outline" => Some(icons::CARDS_PLAYING_OUTLINE),
        "meter-electric-outline" => Some(icons::METER_ELECTRIC_OUTLINE),
        "alert-decagram" => Some(icons::ALERT_DECAGRAM),
        "chili-mild-outline" => Some(icons::CHILI_MILD_OUTLINE),
        "circle-box" => Some(icons::CIRCLE_BOX),
        "close-box" => Some(icons::CLOSE_BOX),
        "seal" => Some(icons::SEAL),
        "battery-low" => Some(icons::BATTERY_LOW),
        "torch" => Some(icons::TORCH),
        "invoice-fast-outline" => Some(icons::INVOICE_FAST_OUTLINE),
        "toy-brick" => Some(icons::TOY_BRICK),
        "star-minus-outline" => Some(icons::STAR_MINUS_OUTLINE),
        "server-network-off" => Some(icons::SERVER_NETWORK_OFF),
        "heart-broken" => Some(icons::HEART_BROKEN),
        "timeline-plus" => Some(icons::TIMELINE_PLUS),
        "swap-vertical-bold" => Some(icons::SWAP_VERTICAL_BOLD),
        "land-fields" => Some(icons::LAND_FIELDS),
        "eyedropper-minus" => Some(icons::EYEDROPPER_MINUS),
        "eye-arrow-left-outline" => Some(icons::EYE_ARROW_LEFT_OUTLINE),
        "surround-sound" => Some(icons::SURROUND_SOUND),
        "drama-masks" => Some(icons::DRAMA_MASKS),
        "image-outline" => Some(icons::IMAGE_OUTLINE),
        "lock-plus-outline" => Some(icons::LOCK_PLUS_OUTLINE),
        "passport-biometric" => Some(icons::PASSPORT_BIOMETRIC),
        "currency-brl" => Some(icons::CURRENCY_BRL),
        "food-apple-outline" => Some(icons::FOOD_APPLE_OUTLINE),
        "speaker-wireless" => Some(icons::SPEAKER_WIRELESS),
        "store-remove" => Some(icons::STORE_REMOVE),
        "fire-truck" => Some(icons::FIRE_TRUCK),
        "countertop-outline" => Some(icons::COUNTERTOP_OUTLINE),
        "camera-document" => Some(icons::CAMERA_DOCUMENT),
        "cursor-default-outline" => Some(icons::CURSOR_DEFAULT_OUTLINE),
        "alpha-b-circle" => Some(icons::ALPHA_B_CIRCLE),
        "circle-half" => Some(icons::CIRCLE_HALF),
        "grid" => Some(icons::GRID),
        "paper-cut-vertical" => Some(icons::PAPER_CUT_VERTICAL),
        "invoice-plus-outline" => Some(icons::INVOICE_PLUS_OUTLINE),
        "table-row-height" => Some(icons::TABLE_ROW_HEIGHT),
        "moon-waning-crescent" => Some(icons::MOON_WANING_CRESCENT),
        "cloud-percent-outline" => Some(icons::CLOUD_PERCENT_OUTLINE),
        "creation" => Some(icons::CREATION),
        "history" => Some(icons::HISTORY),
        #[allow(deprecated)]
        "microsoft-bing" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-bing' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_BING)
        }
        "message-cog" => Some(icons::MESSAGE_COG),
        "emoticon-cool-outline" => Some(icons::EMOTICON_COOL_OUTLINE),
        "fan-auto" => Some(icons::FAN_AUTO),
        "bag-personal-tag-outline" => Some(icons::BAG_PERSONAL_TAG_OUTLINE),
        "fan-minus" => Some(icons::FAN_MINUS),
        "record-circle-outline" => Some(icons::RECORD_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "midi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'midi' is deprecated.").print(py);
            }
            Some(icons::MIDI)
        }
        "bag-carry-on" => Some(icons::BAG_CARRY_ON),
        "relation-only-one-to-one-or-many" => Some(icons::RELATION_ONLY_ONE_TO_ONE_OR_MANY),
        "calendar-lock-outline" => Some(icons::CALENDAR_LOCK_OUTLINE),
        "folder-move" => Some(icons::FOLDER_MOVE),
        "content-save-off-outline" => Some(icons::CONTENT_SAVE_OFF_OUTLINE),
        "printer-3d" => Some(icons::PRINTER_3D),
        "radiobox-blank" => Some(icons::RADIOBOX_BLANK),
        "clipboard-clock" => Some(icons::CLIPBOARD_CLOCK),
        "signal-distance-variant" => Some(icons::SIGNAL_DISTANCE_VARIANT),
        "file-music" => Some(icons::FILE_MUSIC),
        "shield-lock-open" => Some(icons::SHIELD_LOCK_OPEN),
        "file-star-outline" => Some(icons::FILE_STAR_OUTLINE),
        "triangle-down-outline" => Some(icons::TRIANGLE_DOWN_OUTLINE),
        "relation-zero-or-many-to-only-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONLY_ONE),
        "credit-card-refresh" => Some(icons::CREDIT_CARD_REFRESH),
        "scale-bathroom" => Some(icons::SCALE_BATHROOM),
        "human-male-board" => Some(icons::HUMAN_MALE_BOARD),
        #[allow(deprecated)]
        "google-glass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-glass' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_GLASS)
        }
        "close-octagon" => Some(icons::CLOSE_OCTAGON),
        "comment-processing-outline" => Some(icons::COMMENT_PROCESSING_OUTLINE),
        "truck-minus" => Some(icons::TRUCK_MINUS),
        "gas-station-in-use" => Some(icons::GAS_STATION_IN_USE),
        "folder-marker" => Some(icons::FOLDER_MARKER),
        "comment-check" => Some(icons::COMMENT_CHECK),
        "water-off-outline" => Some(icons::WATER_OFF_OUTLINE),
        "storefront-minus-outline" => Some(icons::STOREFRONT_MINUS_OUTLINE),
        "police-badge-outline" => Some(icons::POLICE_BADGE_OUTLINE),
        "printer-3d-nozzle-alert" => Some(icons::PRINTER_3D_NOZZLE_ALERT),
        "flag-remove" => Some(icons::FLAG_REMOVE),
        #[allow(deprecated)]
        "debian" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'debian' is deprecated.").print(py);
            }
            Some(icons::DEBIAN)
        }
        "download-off" => Some(icons::DOWNLOAD_OFF),
        "heart-minus-outline" => Some(icons::HEART_MINUS_OUTLINE),
        "database-eye-outline" => Some(icons::DATABASE_EYE_OUTLINE),
        "alpha-l-circle" => Some(icons::ALPHA_L_CIRCLE),
        "wifi-strength-2" => Some(icons::WIFI_STRENGTH_2),
        "train-car-intermodal" => Some(icons::TRAIN_CAR_INTERMODAL),
        "timer-remove-outline" => Some(icons::TIMER_REMOVE_OUTLINE),
        "eye-minus" => Some(icons::EYE_MINUS),
        "file-document-minus-outline" => Some(icons::FILE_DOCUMENT_MINUS_OUTLINE),
        "pot-mix" => Some(icons::POT_MIX),
        "palette-advanced" => Some(icons::PALETTE_ADVANCED),
        "image-marker-outline" => Some(icons::IMAGE_MARKER_OUTLINE),
        "relation-many-to-one" => Some(icons::RELATION_MANY_TO_ONE),
        "bookmark-box-multiple-outline" => Some(icons::BOOKMARK_BOX_MULTIPLE_OUTLINE),
        "format-line-height" => Some(icons::FORMAT_LINE_HEIGHT),
        "printer-pos-minus-outline" => Some(icons::PRINTER_POS_MINUS_OUTLINE),
        "cupboard" => Some(icons::CUPBOARD),
        "thermometer-lines" => Some(icons::THERMOMETER_LINES),
        "timer-alert" => Some(icons::TIMER_ALERT),
        "hand-clap" => Some(icons::HAND_CLAP),
        "logout-variant" => Some(icons::LOGOUT_VARIANT),
        "fridge-off" => Some(icons::FRIDGE_OFF),
        "longitude" => Some(icons::LONGITUDE),
        "chart-scatter-plot-hexbin" => Some(icons::CHART_SCATTER_PLOT_HEXBIN),
        "brush" => Some(icons::BRUSH),
        "video-4k-box" => Some(icons::VIDEO_4K_BOX),
        "menu-swap-outline" => Some(icons::MENU_SWAP_OUTLINE),
        "map-marker-check-outline" => Some(icons::MAP_MARKER_CHECK_OUTLINE),
        "account-group-outline" => Some(icons::ACCOUNT_GROUP_OUTLINE),
        "account-child" => Some(icons::ACCOUNT_CHILD),
        "network-strength-1-alert" => Some(icons::NETWORK_STRENGTH_1_ALERT),
        "emoticon-wink-outline" => Some(icons::EMOTICON_WINK_OUTLINE),
        "hexagon-slice-6" => Some(icons::HEXAGON_SLICE_6),
        "cloud" => Some(icons::CLOUD),
        "book-refresh-outline" => Some(icons::BOOK_REFRESH_OUTLINE),
        "message-bulleted-off" => Some(icons::MESSAGE_BULLETED_OFF),
        "thermometer-water" => Some(icons::THERMOMETER_WATER),
        "earth-box-off" => Some(icons::EARTH_BOX_OFF),
        "file-chart" => Some(icons::FILE_CHART),
        "wind-power-outline" => Some(icons::WIND_POWER_OUTLINE),
        "comment-outline" => Some(icons::COMMENT_OUTLINE),
        "settings-helper" => Some(icons::SETTINGS_HELPER),
        "map-check" => Some(icons::MAP_CHECK),
        "puzzle-plus" => Some(icons::PUZZLE_PLUS),
        #[allow(deprecated)]
        "jsfiddle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jsfiddle' is deprecated.").print(py);
            }
            Some(icons::JSFIDDLE)
        }
        "numeric-9-box" => Some(icons::NUMERIC_9_BOX),
        #[allow(deprecated)]
        "language-c" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-c' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_C)
        }
        "badge-account-alert-outline" => Some(icons::BADGE_ACCOUNT_ALERT_OUTLINE),
        "brush-variant" => Some(icons::BRUSH_VARIANT),
        "gate-buffer" => Some(icons::GATE_BUFFER),
        "fraction-one-half" => Some(icons::FRACTION_ONE_HALF),
        "robber" => Some(icons::ROBBER),
        "shield-account-outline" => Some(icons::SHIELD_ACCOUNT_OUTLINE),
        "chandelier" => Some(icons::CHANDELIER),
        "table-arrow-down" => Some(icons::TABLE_ARROW_DOWN),
        "feather" => Some(icons::FEATHER),
        "yoga" => Some(icons::YOGA),
        "flask" => Some(icons::FLASK),
        "duck" => Some(icons::DUCK),
        "bug-pause" => Some(icons::BUG_PAUSE),
        "minidisc" => Some(icons::MINIDISC),
        "eye-outline" => Some(icons::EYE_OUTLINE),
        "battery-50-bluetooth" => Some(icons::BATTERY_50_BLUETOOTH),
        "movie-cog-outline" => Some(icons::MOVIE_COG_OUTLINE),
        "console-line" => Some(icons::CONSOLE_LINE),
        "lock-alert" => Some(icons::LOCK_ALERT),
        "gas-station-off" => Some(icons::GAS_STATION_OFF),
        "sticker-minus" => Some(icons::STICKER_MINUS),
        "database-arrow-right-outline" => Some(icons::DATABASE_ARROW_RIGHT_OUTLINE),
        "vector-polygon-variant" => Some(icons::VECTOR_POLYGON_VARIANT),
        "view-split-vertical" => Some(icons::VIEW_SPLIT_VERTICAL),
        "receipt-text-plus-outline" => Some(icons::RECEIPT_TEXT_PLUS_OUTLINE),
        _ => None,
    }
}
