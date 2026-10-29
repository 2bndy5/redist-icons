// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_7(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "file-chart-check-outline" => Some(icons::FILE_CHART_CHECK_OUTLINE),
        "kettle-steam" => Some(icons::KETTLE_STEAM),
        "segment" => Some(icons::SEGMENT),
        "phone-in-talk" => Some(icons::PHONE_IN_TALK),
        "shield-airplane" => Some(icons::SHIELD_AIRPLANE),
        "plus-lock" => Some(icons::PLUS_LOCK),
        "message-lock-outline" => Some(icons::MESSAGE_LOCK_OUTLINE),
        "trending-neutral" => Some(icons::TRENDING_NEUTRAL),
        "arrow-top-left-thin" => Some(icons::ARROW_TOP_LEFT_THIN),
        "note-remove-outline" => Some(icons::NOTE_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "manjaro" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'manjaro' is deprecated.").print(py);
            }
            Some(icons::MANJARO)
        }
        "delete" => Some(icons::DELETE),
        "comment-arrow-left-outline" => Some(icons::COMMENT_ARROW_LEFT_OUTLINE),
        "arrow-bottom-right" => Some(icons::ARROW_BOTTOM_RIGHT),
        #[allow(deprecated)]
        "angular" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'angular' is deprecated.").print(py);
            }
            Some(icons::ANGULAR)
        }
        "skip-next-circle-outline" => Some(icons::SKIP_NEXT_CIRCLE_OUTLINE),
        "order-bool-descending" => Some(icons::ORDER_BOOL_DESCENDING),
        #[allow(deprecated)]
        "dev-to" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dev-to' is deprecated.").print(py);
            }
            Some(icons::DEV_TO)
        }
        "clipboard-search" => Some(icons::CLIPBOARD_SEARCH),
        "pail-minus" => Some(icons::PAIL_MINUS),
        "pine-tree" => Some(icons::PINE_TREE),
        "account-file-text" => Some(icons::ACCOUNT_FILE_TEXT),
        "wifi-arrow-up" => Some(icons::WIFI_ARROW_UP),
        "window-shutter" => Some(icons::WINDOW_SHUTTER),
        "arrow-top-left-bold-box-outline" => Some(icons::ARROW_TOP_LEFT_BOLD_BOX_OUTLINE),
        "diameter" => Some(icons::DIAMETER),
        "information-box" => Some(icons::INFORMATION_BOX),
        "rhombus" => Some(icons::RHOMBUS),
        "alpha-l-circle" => Some(icons::ALPHA_L_CIRCLE),
        "alert-plus-outline" => Some(icons::ALERT_PLUS_OUTLINE),
        "reorder-horizontal" => Some(icons::REORDER_HORIZONTAL),
        "alpha-b-box-outline" => Some(icons::ALPHA_B_BOX_OUTLINE),
        "sync-alert" => Some(icons::SYNC_ALERT),
        "flag-off-outline" => Some(icons::FLAG_OFF_OUTLINE),
        "grill" => Some(icons::GRILL),
        "file-table-box-multiple-outline" => Some(icons::FILE_TABLE_BOX_MULTIPLE_OUTLINE),
        "phone-message" => Some(icons::PHONE_MESSAGE),
        "note-outline" => Some(icons::NOTE_OUTLINE),
        "movie-plus-outline" => Some(icons::MOVIE_PLUS_OUTLINE),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "account-voice-off" => Some(icons::ACCOUNT_VOICE_OFF),
        "ship-wheel" => Some(icons::SHIP_WHEEL),
        "bowl-mix-outline" => Some(icons::BOWL_MIX_OUTLINE),
        "music-note-half" => Some(icons::MUSIC_NOTE_HALF),
        "table-border" => Some(icons::TABLE_BORDER),
        "rocket-launch" => Some(icons::ROCKET_LAUNCH),
        "butterfly-outline" => Some(icons::BUTTERFLY_OUTLINE),
        "truck-outline" => Some(icons::TRUCK_OUTLINE),
        "format-align-right" => Some(icons::FORMAT_ALIGN_RIGHT),
        #[allow(deprecated)]
        "opera" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'opera' is deprecated.").print(py);
            }
            Some(icons::OPERA)
        }
        "octagram-plus-outline" => Some(icons::OCTAGRAM_PLUS_OUTLINE),
        "plus-box-multiple" => Some(icons::PLUS_BOX_MULTIPLE),
        "gate-xor" => Some(icons::GATE_XOR),
        "graph-outline" => Some(icons::GRAPH_OUTLINE),
        "arrow-u-down-left-bold" => Some(icons::ARROW_U_DOWN_LEFT_BOLD),
        "account-multiple-minus-outline" => Some(icons::ACCOUNT_MULTIPLE_MINUS_OUTLINE),
        "scan-helper" => Some(icons::SCAN_HELPER),
        "relation-zero-or-many-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_ONE),
        "clipboard-search-outline" => Some(icons::CLIPBOARD_SEARCH_OUTLINE),
        "database-outline" => Some(icons::DATABASE_OUTLINE),
        "shape-polygon-plus" => Some(icons::SHAPE_POLYGON_PLUS),
        "crown-circle-outline" => Some(icons::CROWN_CIRCLE_OUTLINE),
        "camera-rear" => Some(icons::CAMERA_REAR),
        "bed-king" => Some(icons::BED_KING),
        "kettle-pour-over" => Some(icons::KETTLE_POUR_OVER),
        "skate" => Some(icons::SKATE),
        "clock-fast" => Some(icons::CLOCK_FAST),
        "calendar-end-outline" => Some(icons::CALENDAR_END_OUTLINE),
        "flash-alert" => Some(icons::FLASH_ALERT),
        "seesaw" => Some(icons::SEESAW),
        "cloud-circle-outline" => Some(icons::CLOUD_CIRCLE_OUTLINE),
        "alpha-r-circle" => Some(icons::ALPHA_R_CIRCLE),
        "gauge-low" => Some(icons::GAUGE_LOW),
        "archive-marker" => Some(icons::ARCHIVE_MARKER),
        "signal-4g" => Some(icons::SIGNAL_4G),
        "tshirt-crew" => Some(icons::TSHIRT_CREW),
        "emoticon-angry" => Some(icons::EMOTICON_ANGRY),
        "library" => Some(icons::LIBRARY),
        "arrow-projectile-multiple" => Some(icons::ARROW_PROJECTILE_MULTIPLE),
        "printer-off-outline" => Some(icons::PRINTER_OFF_OUTLINE),
        "checkbox-multiple-blank-outline" => Some(icons::CHECKBOX_MULTIPLE_BLANK_OUTLINE),
        "account-sync" => Some(icons::ACCOUNT_SYNC),
        "beaker-check-outline" => Some(icons::BEAKER_CHECK_OUTLINE),
        "shield-sun-outline" => Some(icons::SHIELD_SUN_OUTLINE),
        "cheese" => Some(icons::CHEESE),
        "email-open" => Some(icons::EMAIL_OPEN),
        "alpha-z-circle" => Some(icons::ALPHA_Z_CIRCLE),
        "robot-vacuum-variant-alert" => Some(icons::ROBOT_VACUUM_VARIANT_ALERT),
        "rug" => Some(icons::RUG),
        "turnstile-outline" => Some(icons::TURNSTILE_OUTLINE),
        "nfc-variant-off" => Some(icons::NFC_VARIANT_OFF),
        "alpha-w-box-outline" => Some(icons::ALPHA_W_BOX_OUTLINE),
        "vector-triangle" => Some(icons::VECTOR_TRIANGLE),
        "archive-arrow-down" => Some(icons::ARCHIVE_ARROW_DOWN),
        "eject-circle-outline" => Some(icons::EJECT_CIRCLE_OUTLINE),
        "clipboard-check-outline" => Some(icons::CLIPBOARD_CHECK_OUTLINE),
        "arrow-expand-all" => Some(icons::ARROW_EXPAND_ALL),
        "ear-hearing-off" => Some(icons::EAR_HEARING_OFF),
        "tag-text" => Some(icons::TAG_TEXT),
        "strategy" => Some(icons::STRATEGY),
        #[allow(deprecated)]
        "language-fortran" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-fortran' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_FORTRAN)
        }
        "weather-moonset-down" => Some(icons::WEATHER_MOONSET_DOWN),
        "train-car-intermodal" => Some(icons::TRAIN_CAR_INTERMODAL),
        "cursor-pointer" => Some(icons::CURSOR_POINTER),
        "credit-card-clock" => Some(icons::CREDIT_CARD_CLOCK),
        "invert-colors" => Some(icons::INVERT_COLORS),
        #[allow(deprecated)]
        "language-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-go' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_GO)
        }
        "wall" => Some(icons::WALL),
        "emoticon-tongue" => Some(icons::EMOTICON_TONGUE),
        "file-delimited-outline" => Some(icons::FILE_DELIMITED_OUTLINE),
        "folder-key-outline" => Some(icons::FOLDER_KEY_OUTLINE),
        "server-network-outline" => Some(icons::SERVER_NETWORK_OUTLINE),
        "human-male-female" => Some(icons::HUMAN_MALE_FEMALE),
        "account-edit" => Some(icons::ACCOUNT_EDIT),
        "cards-heart-outline" => Some(icons::CARDS_HEART_OUTLINE),
        "account-tie-outline" => Some(icons::ACCOUNT_TIE_OUTLINE),
        "folder-settings" => Some(icons::FOLDER_SETTINGS),
        "layers-off-outline" => Some(icons::LAYERS_OFF_OUTLINE),
        "volcano" => Some(icons::VOLCANO),
        "eyedropper-variant" => Some(icons::EYEDROPPER_VARIANT),
        "email-arrow-left" => Some(icons::EMAIL_ARROW_LEFT),
        "folder-star-outline" => Some(icons::FOLDER_STAR_OUTLINE),
        "plus-box" => Some(icons::PLUS_BOX),
        "book-minus-multiple-outline" => Some(icons::BOOK_MINUS_MULTIPLE_OUTLINE),
        "arrow-bottom-left-thin" => Some(icons::ARROW_BOTTOM_LEFT_THIN),
        "chili-alert-outline" => Some(icons::CHILI_ALERT_OUTLINE),
        "file-sync-outline" => Some(icons::FILE_SYNC_OUTLINE),
        "border-none" => Some(icons::BORDER_NONE),
        "folder" => Some(icons::FOLDER),
        "chip" => Some(icons::CHIP),
        "blinds-horizontal" => Some(icons::BLINDS_HORIZONTAL),
        "numeric-7-box" => Some(icons::NUMERIC_7_BOX),
        "translate" => Some(icons::TRANSLATE),
        "water-off" => Some(icons::WATER_OFF),
        "gift-off" => Some(icons::GIFT_OFF),
        "hand-front-right" => Some(icons::HAND_FRONT_RIGHT),
        "music-clef-treble" => Some(icons::MUSIC_CLEF_TREBLE),
        "axis-z-rotate-clockwise" => Some(icons::AXIS_Z_ROTATE_CLOCKWISE),
        "coolant-temperature" => Some(icons::COOLANT_TEMPERATURE),
        "city-switch" => Some(icons::CITY_SWITCH),
        "file-import-outline" => Some(icons::FILE_IMPORT_OUTLINE),
        "sort-variant-remove" => Some(icons::SORT_VARIANT_REMOVE),
        "lock-percent-open" => Some(icons::LOCK_PERCENT_OPEN),
        #[allow(deprecated)]
        "folder-google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'folder-google-drive' is deprecated.")
                    .print(py);
            }
            Some(icons::FOLDER_GOOGLE_DRIVE)
        }
        "rewind-10" => Some(icons::REWIND_10),
        "credit-card-off-outline" => Some(icons::CREDIT_CARD_OFF_OUTLINE),
        "file-rotate-left-outline" => Some(icons::FILE_ROTATE_LEFT_OUTLINE),
        "car-off" => Some(icons::CAR_OFF),
        "battery-negative" => Some(icons::BATTERY_NEGATIVE),
        "battery-charging-low" => Some(icons::BATTERY_CHARGING_LOW),
        "shaker-outline" => Some(icons::SHAKER_OUTLINE),
        "train-car-centerbeam-full" => Some(icons::TRAIN_CAR_CENTERBEAM_FULL),
        "credit-card-fast" => Some(icons::CREDIT_CARD_FAST),
        "emoticon-minus-outline" => Some(icons::EMOTICON_MINUS_OUTLINE),
        "serial-port" => Some(icons::SERIAL_PORT),
        "message" => Some(icons::MESSAGE),
        "car-emergency" => Some(icons::CAR_EMERGENCY),
        "flask-empty-outline" => Some(icons::FLASK_EMPTY_OUTLINE),
        "lock-open-outline" => Some(icons::LOCK_OPEN_OUTLINE),
        "zodiac-pisces" => Some(icons::ZODIAC_PISCES),
        "glass-wine" => Some(icons::GLASS_WINE),
        "beta" => Some(icons::BETA),
        "pac-man" => Some(icons::PAC_MAN),
        "gender-male" => Some(icons::GENDER_MALE),
        "sign-direction" => Some(icons::SIGN_DIRECTION),
        "wind-turbine-alert" => Some(icons::WIND_TURBINE_ALERT),
        "relation-zero-or-many-to-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE),
        "hand-pointing-right" => Some(icons::HAND_POINTING_RIGHT),
        "alpha-f" => Some(icons::ALPHA_F),
        "carrot" => Some(icons::CARROT),
        "file-star-four-points" => Some(icons::FILE_STAR_FOUR_POINTS),
        #[allow(deprecated)]
        "facebook-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-gaming' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_GAMING)
        }
        "movie-open-play" => Some(icons::MOVIE_OPEN_PLAY),
        "apple-keyboard-caps" => Some(icons::APPLE_KEYBOARD_CAPS),
        "camera-metering-center" => Some(icons::CAMERA_METERING_CENTER),
        "credit-card-scan-outline" => Some(icons::CREDIT_CARD_SCAN_OUTLINE),
        "data-matrix-plus" => Some(icons::DATA_MATRIX_PLUS),
        "kettle-off" => Some(icons::KETTLE_OFF),
        "timeline-clock-outline" => Some(icons::TIMELINE_CLOCK_OUTLINE),
        "folder-plus-outline" => Some(icons::FOLDER_PLUS_OUTLINE),
        "lock-open-check" => Some(icons::LOCK_OPEN_CHECK),
        "basket" => Some(icons::BASKET),
        "piano" => Some(icons::PIANO),
        "file-question" => Some(icons::FILE_QUESTION),
        "share-all-outline" => Some(icons::SHARE_ALL_OUTLINE),
        "database-import" => Some(icons::DATABASE_IMPORT),
        "vector-polygon" => Some(icons::VECTOR_POLYGON),
        "account-convert" => Some(icons::ACCOUNT_CONVERT),
        "land-plots" => Some(icons::LAND_PLOTS),
        "cloud-percent" => Some(icons::CLOUD_PERCENT),
        "timer-plus" => Some(icons::TIMER_PLUS),
        "shield-crown" => Some(icons::SHIELD_CROWN),
        "folder-lock-outline" => Some(icons::FOLDER_LOCK_OUTLINE),
        "comment-plus" => Some(icons::COMMENT_PLUS),
        "battery-check-outline" => Some(icons::BATTERY_CHECK_OUTLINE),
        "cellphone-cog" => Some(icons::CELLPHONE_COG),
        "calendar-month-outline" => Some(icons::CALENDAR_MONTH_OUTLINE),
        "cone-off" => Some(icons::CONE_OFF),
        "wrench-clock-outline" => Some(icons::WRENCH_CLOCK_OUTLINE),
        "account-multiple-remove-outline" => Some(icons::ACCOUNT_MULTIPLE_REMOVE_OUTLINE),
        _ => None,
    }
}
