// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_10(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "numeric-7-box" => Some(icons::NUMERIC_7_BOX),
        "alarm-panel-outline" => Some(icons::ALARM_PANEL_OUTLINE),
        "shield" => Some(icons::SHIELD),
        "ticket-confirmation" => Some(icons::TICKET_CONFIRMATION),
        "power-socket-us" => Some(icons::POWER_SOCKET_US),
        "hydraulic-oil-temperature" => Some(icons::HYDRAULIC_OIL_TEMPERATURE),
        "unicorn" => Some(icons::UNICORN),
        #[allow(deprecated)]
        "google-street-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-street-view' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_STREET_VIEW)
        }
        "camera-document" => Some(icons::CAMERA_DOCUMENT),
        "table-network" => Some(icons::TABLE_NETWORK),
        "contain" => Some(icons::CONTAIN),
        "record-circle-outline" => Some(icons::RECORD_CIRCLE_OUTLINE),
        "receipt-text-plus" => Some(icons::RECEIPT_TEXT_PLUS),
        "hand-coin-outline" => Some(icons::HAND_COIN_OUTLINE),
        "nfc-search-variant" => Some(icons::NFC_SEARCH_VARIANT),
        "purse-outline" => Some(icons::PURSE_OUTLINE),
        "format-float-center" => Some(icons::FORMAT_FLOAT_CENTER),
        "arrow-down-drop-circle-outline" => Some(icons::ARROW_DOWN_DROP_CIRCLE_OUTLINE),
        "table-star" => Some(icons::TABLE_STAR),
        "credit-card-refresh-outline" => Some(icons::CREDIT_CARD_REFRESH_OUTLINE),
        "white-balance-incandescent" => Some(icons::WHITE_BALANCE_INCANDESCENT),
        "axis-lock" => Some(icons::AXIS_LOCK),
        "leek" => Some(icons::LEEK),
        "archive-clock-outline" => Some(icons::ARCHIVE_CLOCK_OUTLINE),
        "numeric-8" => Some(icons::NUMERIC_8),
        "credit-card-outline" => Some(icons::CREDIT_CARD_OUTLINE),
        "arrow-collapse-all" => Some(icons::ARROW_COLLAPSE_ALL),
        "format-list-group" => Some(icons::FORMAT_LIST_GROUP),
        "elevation-rise" => Some(icons::ELEVATION_RISE),
        "download-box-outline" => Some(icons::DOWNLOAD_BOX_OUTLINE),
        "head-plus-outline" => Some(icons::HEAD_PLUS_OUTLINE),
        "web-cancel" => Some(icons::WEB_CANCEL),
        "glass-cocktail-off" => Some(icons::GLASS_COCKTAIL_OFF),
        "chess-king" => Some(icons::CHESS_KING),
        "fencing" => Some(icons::FENCING),
        "content-save-all-outline" => Some(icons::CONTENT_SAVE_ALL_OUTLINE),
        "account-group" => Some(icons::ACCOUNT_GROUP),
        "wardrobe" => Some(icons::WARDROBE),
        #[allow(deprecated)]
        "hulu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'hulu' is deprecated.").print(py);
            }
            Some(icons::HULU)
        }
        "email-off" => Some(icons::EMAIL_OFF),
        "clipboard-pulse-outline" => Some(icons::CLIPBOARD_PULSE_OUTLINE),
        "pound-box-outline" => Some(icons::POUND_BOX_OUTLINE),
        "crop" => Some(icons::CROP),
        "pot-steam" => Some(icons::POT_STEAM),
        "watermark" => Some(icons::WATERMARK),
        "file-move" => Some(icons::FILE_MOVE),
        "sync" => Some(icons::SYNC),
        "gesture-spread" => Some(icons::GESTURE_SPREAD),
        #[allow(deprecated)]
        "simple-icons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'simple-icons' is deprecated.").print(py);
            }
            Some(icons::SIMPLE_ICONS)
        }
        "circle-box" => Some(icons::CIRCLE_BOX),
        "thermometer-check" => Some(icons::THERMOMETER_CHECK),
        "car-wrench" => Some(icons::CAR_WRENCH),
        "account-multiple" => Some(icons::ACCOUNT_MULTIPLE),
        "email-alert" => Some(icons::EMAIL_ALERT),
        "arrange-send-backward" => Some(icons::ARRANGE_SEND_BACKWARD),
        "window-open" => Some(icons::WINDOW_OPEN),
        "boom-gate-arrow-down-outline" => Some(icons::BOOM_GATE_ARROW_DOWN_OUTLINE),
        "bug-outline" => Some(icons::BUG_OUTLINE),
        "clock-star-four-points-outline" => Some(icons::CLOCK_STAR_FOUR_POINTS_OUTLINE),
        "table-multiple" => Some(icons::TABLE_MULTIPLE),
        "cog-sync" => Some(icons::COG_SYNC),
        "oil-temperature" => Some(icons::OIL_TEMPERATURE),
        "cash-lock" => Some(icons::CASH_LOCK),
        "phone-bluetooth-outline" => Some(icons::PHONE_BLUETOOTH_OUTLINE),
        "database-alert" => Some(icons::DATABASE_ALERT),
        "relation-only-one-to-zero-or-one" => Some(icons::RELATION_ONLY_ONE_TO_ZERO_OR_ONE),
        "layers-edit" => Some(icons::LAYERS_EDIT),
        "human-non-binary" => Some(icons::HUMAN_NON_BINARY),
        "umbrella-closed" => Some(icons::UMBRELLA_CLOSED),
        "battery-arrow-down" => Some(icons::BATTERY_ARROW_DOWN),
        "pine-tree-box" => Some(icons::PINE_TREE_BOX),
        "menu-up-outline" => Some(icons::MENU_UP_OUTLINE),
        "umbrella-beach-outline" => Some(icons::UMBRELLA_BEACH_OUTLINE),
        "map-marker-right" => Some(icons::MAP_MARKER_RIGHT),
        "view-list" => Some(icons::VIEW_LIST),
        "book-remove" => Some(icons::BOOK_REMOVE),
        "hand-heart" => Some(icons::HAND_HEART),
        "numeric-negative-1" => Some(icons::NUMERIC_NEGATIVE_1),
        "pencil-circle-outline" => Some(icons::PENCIL_CIRCLE_OUTLINE),
        "toothbrush" => Some(icons::TOOTHBRUSH),
        "window-open-variant" => Some(icons::WINDOW_OPEN_VARIANT),
        "phone-check-outline" => Some(icons::PHONE_CHECK_OUTLINE),
        "keyboard-esc" => Some(icons::KEYBOARD_ESC),
        "cloud-check" => Some(icons::CLOUD_CHECK),
        "cookie-remove-outline" => Some(icons::COOKIE_REMOVE_OUTLINE),
        "glass-fragile" => Some(icons::GLASS_FRAGILE),
        "sim-alert" => Some(icons::SIM_ALERT),
        "phone-return-outline" => Some(icons::PHONE_RETURN_OUTLINE),
        "clipboard-play-multiple-outline" => Some(icons::CLIPBOARD_PLAY_MULTIPLE_OUTLINE),
        "flash-outline" => Some(icons::FLASH_OUTLINE),
        "camera-burst" => Some(icons::CAMERA_BURST),
        "calendar-clock-outline" => Some(icons::CALENDAR_CLOCK_OUTLINE),
        "chevron-right-circle-outline" => Some(icons::CHEVRON_RIGHT_CIRCLE_OUTLINE),
        "cloud-sync-outline" => Some(icons::CLOUD_SYNC_OUTLINE),
        "play-speed" => Some(icons::PLAY_SPEED),
        "pencil-ruler" => Some(icons::PENCIL_RULER),
        "text-box-plus-outline" => Some(icons::TEXT_BOX_PLUS_OUTLINE),
        "clover" => Some(icons::CLOVER),
        "currency-ngn" => Some(icons::CURRENCY_NGN),
        "playlist-music" => Some(icons::PLAYLIST_MUSIC),
        "video-input-hdmi" => Some(icons::VIDEO_INPUT_HDMI),
        "view-dashboard-edit" => Some(icons::VIEW_DASHBOARD_EDIT),
        "angle-right" => Some(icons::ANGLE_RIGHT),
        "human-female-girl" => Some(icons::HUMAN_FEMALE_GIRL),
        "wall-sconce-flat-outline" => Some(icons::WALL_SCONCE_FLAT_OUTLINE),
        "map-check-outline" => Some(icons::MAP_CHECK_OUTLINE),
        "filter-remove" => Some(icons::FILTER_REMOVE),
        "cash-minus" => Some(icons::CASH_MINUS),
        "printer-3d" => Some(icons::PRINTER_3D),
        "phone-cancel" => Some(icons::PHONE_CANCEL),
        "phone-remove" => Some(icons::PHONE_REMOVE),
        "bank-off" => Some(icons::BANK_OFF),
        "temple-buddhist" => Some(icons::TEMPLE_BUDDHIST),
        #[allow(deprecated)]
        "ethereum" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ethereum' is deprecated.").print(py);
            }
            Some(icons::ETHEREUM)
        }
        "cog-refresh-outline" => Some(icons::COG_REFRESH_OUTLINE),
        "email-open" => Some(icons::EMAIL_OPEN),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "safe-square" => Some(icons::SAFE_SQUARE),
        "sun-thermometer-outline" => Some(icons::SUN_THERMOMETER_OUTLINE),
        "text-shadow" => Some(icons::TEXT_SHADOW),
        "clipboard-remove" => Some(icons::CLIPBOARD_REMOVE),
        "koala" => Some(icons::KOALA),
        "hdr-off" => Some(icons::HDR_OFF),
        "vector-point-select" => Some(icons::VECTOR_POINT_SELECT),
        "relation-zero-or-many-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_ONE),
        "file-table-box" => Some(icons::FILE_TABLE_BOX),
        "exit-run" => Some(icons::EXIT_RUN),
        "wind-turbine-alert" => Some(icons::WIND_TURBINE_ALERT),
        "landslide-outline" => Some(icons::LANDSLIDE_OUTLINE),
        "alpha-g-box-outline" => Some(icons::ALPHA_G_BOX_OUTLINE),
        "close-outline" => Some(icons::CLOSE_OUTLINE),
        "image-lock" => Some(icons::IMAGE_LOCK),
        "chili-alert" => Some(icons::CHILI_ALERT),
        "chevron-left-circle-outline" => Some(icons::CHEVRON_LEFT_CIRCLE_OUTLINE),
        "bed-double-outline" => Some(icons::BED_DOUBLE_OUTLINE),
        "roman-numeral-9" => Some(icons::ROMAN_NUMERAL_9),
        "camera-wireless-outline" => Some(icons::CAMERA_WIRELESS_OUTLINE),
        "translate-variant" => Some(icons::TRANSLATE_VARIANT),
        #[allow(deprecated)]
        "dlna" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dlna' is deprecated.").print(py);
            }
            Some(icons::DLNA)
        }
        "cellphone-cog" => Some(icons::CELLPHONE_COG),
        "timer-star-outline" => Some(icons::TIMER_STAR_OUTLINE),
        "vector-circle" => Some(icons::VECTOR_CIRCLE),
        #[allow(deprecated)]
        "google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-drive' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_DRIVE)
        }
        "credit-card-remove" => Some(icons::CREDIT_CARD_REMOVE),
        "chili-off-outline" => Some(icons::CHILI_OFF_OUTLINE),
        "file-replace-outline" => Some(icons::FILE_REPLACE_OUTLINE),
        "currency-brl" => Some(icons::CURRENCY_BRL),
        "plus-minus-variant" => Some(icons::PLUS_MINUS_VARIANT),
        "format-textbox" => Some(icons::FORMAT_TEXTBOX),
        "dice-2-outline" => Some(icons::DICE_2_OUTLINE),
        "hand-clap" => Some(icons::HAND_CLAP),
        "lamps-outline" => Some(icons::LAMPS_OUTLINE),
        "swap-vertical-bold" => Some(icons::SWAP_VERTICAL_BOLD),
        "pencil-lock-outline" => Some(icons::PENCIL_LOCK_OUTLINE),
        "card-minus-outline" => Some(icons::CARD_MINUS_OUTLINE),
        "arrow-all" => Some(icons::ARROW_ALL),
        "shape-outline" => Some(icons::SHAPE_OUTLINE),
        "flag-remove-outline" => Some(icons::FLAG_REMOVE_OUTLINE),
        "data-matrix" => Some(icons::DATA_MATRIX),
        "lightbulb-night-outline" => Some(icons::LIGHTBULB_NIGHT_OUTLINE),
        "folder-key-outline" => Some(icons::FOLDER_KEY_OUTLINE),
        "archive-check" => Some(icons::ARCHIVE_CHECK),
        "map-marker-distance" => Some(icons::MAP_MARKER_DISTANCE),
        "star-four-points-circle-outline" => Some(icons::STAR_FOUR_POINTS_CIRCLE_OUTLINE),
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
        "halloween" => Some(icons::HALLOWEEN),
        "bow-arrow" => Some(icons::BOW_ARROW),
        "notebook" => Some(icons::NOTEBOOK),
        "lock-open-plus" => Some(icons::LOCK_OPEN_PLUS),
        "bell-circle" => Some(icons::BELL_CIRCLE),
        "shark-fin-outline" => Some(icons::SHARK_FIN_OUTLINE),
        "car-search" => Some(icons::CAR_SEARCH),
        "image-album" => Some(icons::IMAGE_ALBUM),
        "map-marker-check" => Some(icons::MAP_MARKER_CHECK),
        "axis-x-rotate-clockwise" => Some(icons::AXIS_X_ROTATE_CLOCKWISE),
        "sphere" => Some(icons::SPHERE),
        "camera-flip-outline" => Some(icons::CAMERA_FLIP_OUTLINE),
        "train-variant" => Some(icons::TRAIN_VARIANT),
        "assistant" => Some(icons::ASSISTANT),
        "brain" => Some(icons::BRAIN),
        "floppy-variant" => Some(icons::FLOPPY_VARIANT),
        "roman-numeral-7" => Some(icons::ROMAN_NUMERAL_7),
        #[allow(deprecated)]
        "language-markdown" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-markdown' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_MARKDOWN)
        }
        "cup-outline" => Some(icons::CUP_OUTLINE),
        "account-injury-outline" => Some(icons::ACCOUNT_INJURY_OUTLINE),
        "pause-octagon-outline" => Some(icons::PAUSE_OCTAGON_OUTLINE),
        "car-coolant-level" => Some(icons::CAR_COOLANT_LEVEL),
        "weather-tornado" => Some(icons::WEATHER_TORNADO),
        "ssh" => Some(icons::SSH),
        "tablet" => Some(icons::TABLET),
        "message-plus-outline" => Some(icons::MESSAGE_PLUS_OUTLINE),
        "alpha-a" => Some(icons::ALPHA_A),
        "lock-pattern" => Some(icons::LOCK_PATTERN),
        "file-delimited-outline" => Some(icons::FILE_DELIMITED_OUTLINE),
        "earth-remove" => Some(icons::EARTH_REMOVE),
        "numeric-4" => Some(icons::NUMERIC_4),
        "email-sync-outline" => Some(icons::EMAIL_SYNC_OUTLINE),
        "swap-vertical-variant" => Some(icons::SWAP_VERTICAL_VARIANT),
        "lightbulb-multiple-off-outline" => Some(icons::LIGHTBULB_MULTIPLE_OFF_OUTLINE),
        "sun-snowflake" => Some(icons::SUN_SNOWFLAKE),
        _ => None,
    }
}
