// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_11(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "mouse-right-click-outline" => Some(icons::MOUSE_RIGHT_CLICK_OUTLINE),
        "overscan" => Some(icons::OVERSCAN),
        "lightbulb-cfl" => Some(icons::LIGHTBULB_CFL),
        "database-cog" => Some(icons::DATABASE_COG),
        "hair-dryer" => Some(icons::HAIR_DRYER),
        "escalator-up" => Some(icons::ESCALATOR_UP),
        "note-plus-outline" => Some(icons::NOTE_PLUS_OUTLINE),
        "relation-zero-or-many-to-only-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONLY_ONE),
        "label-multiple-outline" => Some(icons::LABEL_MULTIPLE_OUTLINE),
        "bag-personal-plus" => Some(icons::BAG_PERSONAL_PLUS),
        "heart-plus" => Some(icons::HEART_PLUS),
        "motion-play-outline" => Some(icons::MOTION_PLAY_OUTLINE),
        "briefcase-variant-off-outline" => Some(icons::BRIEFCASE_VARIANT_OFF_OUTLINE),
        "comment-processing" => Some(icons::COMMENT_PROCESSING),
        "account-multiple-check-outline" => Some(icons::ACCOUNT_MULTIPLE_CHECK_OUTLINE),
        "soccer-field" => Some(icons::SOCCER_FIELD),
        "eye-remove" => Some(icons::EYE_REMOVE),
        "text-box-edit" => Some(icons::TEXT_BOX_EDIT),
        "file-table-box-multiple-outline" => Some(icons::FILE_TABLE_BOX_MULTIPLE_OUTLINE),
        "calendar-multiselect" => Some(icons::CALENDAR_MULTISELECT),
        "alpha-c-circle-outline" => Some(icons::ALPHA_C_CIRCLE_OUTLINE),
        "island" => Some(icons::ISLAND),
        "alpha-x" => Some(icons::ALPHA_X),
        "alpha-h" => Some(icons::ALPHA_H),
        "beaker-alert-outline" => Some(icons::BEAKER_ALERT_OUTLINE),
        "cone-off" => Some(icons::CONE_OFF),
        "image-area-close" => Some(icons::IMAGE_AREA_CLOSE),
        "skull-outline" => Some(icons::SKULL_OUTLINE),
        "panorama-sphere-outline" => Some(icons::PANORAMA_SPHERE_OUTLINE),
        "star-circle-outline" => Some(icons::STAR_CIRCLE_OUTLINE),
        "calendar-multiple" => Some(icons::CALENDAR_MULTIPLE),
        "calendar-edit" => Some(icons::CALENDAR_EDIT),
        "earth-off" => Some(icons::EARTH_OFF),
        "bunk-bed" => Some(icons::BUNK_BED),
        "cart-arrow-right" => Some(icons::CART_ARROW_RIGHT),
        "stove" => Some(icons::STOVE),
        "trophy-variant" => Some(icons::TROPHY_VARIANT),
        "sort-bool-ascending-variant" => Some(icons::SORT_BOOL_ASCENDING_VARIANT),
        "menu-close" => Some(icons::MENU_CLOSE),
        "car-clutch" => Some(icons::CAR_CLUTCH),
        "folder-alert-outline" => Some(icons::FOLDER_ALERT_OUTLINE),
        "pickaxe" => Some(icons::PICKAXE),
        "file-cabinet" => Some(icons::FILE_CABINET),
        "text-long" => Some(icons::TEXT_LONG),
        "comment-alert-outline" => Some(icons::COMMENT_ALERT_OUTLINE),
        "table-split-cell" => Some(icons::TABLE_SPLIT_CELL),
        "file-arrow-up-down-outline" => Some(icons::FILE_ARROW_UP_DOWN_OUTLINE),
        "floor-lamp-torchiere" => Some(icons::FLOOR_LAMP_TORCHIERE),
        "file-video-outline" => Some(icons::FILE_VIDEO_OUTLINE),
        "format-list-bulleted-triangle" => Some(icons::FORMAT_LIST_BULLETED_TRIANGLE),
        "ghost-off-outline" => Some(icons::GHOST_OFF_OUTLINE),
        "puzzle-outline" => Some(icons::PUZZLE_OUTLINE),
        "signal-cellular-outline" => Some(icons::SIGNAL_CELLULAR_OUTLINE),
        "image-filter-center-focus-weak" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_WEAK),
        "checkbox-multiple-marked-outline" => Some(icons::CHECKBOX_MULTIPLE_MARKED_OUTLINE),
        #[allow(deprecated)]
        "jira" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jira' is deprecated.").print(py);
            }
            Some(icons::JIRA)
        }
        "pencil-lock-outline" => Some(icons::PENCIL_LOCK_OUTLINE),
        "percent-box-outline" => Some(icons::PERCENT_BOX_OUTLINE),
        "format-float-right" => Some(icons::FORMAT_FLOAT_RIGHT),
        "soccer" => Some(icons::SOCCER),
        "gate-alert" => Some(icons::GATE_ALERT),
        "file-send" => Some(icons::FILE_SEND),
        "folder-pound-outline" => Some(icons::FOLDER_POUND_OUTLINE),
        "shield-alert-outline" => Some(icons::SHIELD_ALERT_OUTLINE),
        "delete-restore" => Some(icons::DELETE_RESTORE),
        "bullhorn-variant" => Some(icons::BULLHORN_VARIANT),
        "taco" => Some(icons::TACO),
        "cards-spade" => Some(icons::CARDS_SPADE),
        "butterfly-outline" => Some(icons::BUTTERFLY_OUTLINE),
        "baseball-outline" => Some(icons::BASEBALL_OUTLINE),
        "shield-star-outline" => Some(icons::SHIELD_STAR_OUTLINE),
        "string-lights-off" => Some(icons::STRING_LIGHTS_OFF),
        "panorama-outline" => Some(icons::PANORAMA_OUTLINE),
        "credit-card-multiple" => Some(icons::CREDIT_CARD_MULTIPLE),
        "mouse-left-click-outline" => Some(icons::MOUSE_LEFT_CLICK_OUTLINE),
        "cards-diamond-outline" => Some(icons::CARDS_DIAMOND_OUTLINE),
        "power-socket" => Some(icons::POWER_SOCKET),
        "gate-arrow-right" => Some(icons::GATE_ARROW_RIGHT),
        "racquetball" => Some(icons::RACQUETBALL),
        "printer-3d-nozzle-off-outline" => Some(icons::PRINTER_3D_NOZZLE_OFF_OUTLINE),
        "exponent-box" => Some(icons::EXPONENT_BOX),
        "coach-lamp-variant" => Some(icons::COACH_LAMP_VARIANT),
        "link-plus" => Some(icons::LINK_PLUS),
        "card-off-outline" => Some(icons::CARD_OFF_OUTLINE),
        "music-note-whole-dotted" => Some(icons::MUSIC_NOTE_WHOLE_DOTTED),
        "video-3d-variant" => Some(icons::VIDEO_3D_VARIANT),
        "signal" => Some(icons::SIGNAL),
        "credit-card-check-outline" => Some(icons::CREDIT_CARD_CHECK_OUTLINE),
        "human-capacity-increase" => Some(icons::HUMAN_CAPACITY_INCREASE),
        "arrow-collapse-horizontal" => Some(icons::ARROW_COLLAPSE_HORIZONTAL),
        "thermometer-auto" => Some(icons::THERMOMETER_AUTO),
        #[allow(deprecated)]
        "nativescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nativescript' is deprecated.").print(py);
            }
            Some(icons::NATIVESCRIPT)
        }
        "account-school" => Some(icons::ACCOUNT_SCHOOL),
        "select-drag" => Some(icons::SELECT_DRAG),
        "book-open" => Some(icons::BOOK_OPEN),
        "gamepad-round" => Some(icons::GAMEPAD_ROUND),
        "alarm-snooze" => Some(icons::ALARM_SNOOZE),
        "podium-gold" => Some(icons::PODIUM_GOLD),
        "passport-biometric" => Some(icons::PASSPORT_BIOMETRIC),
        "touch-text-outline" => Some(icons::TOUCH_TEXT_OUTLINE),
        "gate-open" => Some(icons::GATE_OPEN),
        "text-box-edit-outline" => Some(icons::TEXT_BOX_EDIT_OUTLINE),
        "hand-pointing-down" => Some(icons::HAND_POINTING_DOWN),
        "stairs" => Some(icons::STAIRS),
        "format-list-checkbox" => Some(icons::FORMAT_LIST_CHECKBOX),
        "camera-rear-variant" => Some(icons::CAMERA_REAR_VARIANT),
        "seed-plus" => Some(icons::SEED_PLUS),
        "fan-off" => Some(icons::FAN_OFF),
        "file-percent-outline" => Some(icons::FILE_PERCENT_OUTLINE),
        "clipboard-text-multiple" => Some(icons::CLIPBOARD_TEXT_MULTIPLE),
        "arrow-up-thick" => Some(icons::ARROW_UP_THICK),
        "traffic-light-outline" => Some(icons::TRAFFIC_LIGHT_OUTLINE),
        "steering" => Some(icons::STEERING),
        "numeric-7-box-multiple-outline" => Some(icons::NUMERIC_7_BOX_MULTIPLE_OUTLINE),
        "truck-fast-outline" => Some(icons::TRUCK_FAST_OUTLINE),
        "artboard" => Some(icons::ARTBOARD),
        "lock-percent-open-variant" => Some(icons::LOCK_PERCENT_OPEN_VARIANT),
        "kite" => Some(icons::KITE),
        "light-switch" => Some(icons::LIGHT_SWITCH),
        "phone-log-outline" => Some(icons::PHONE_LOG_OUTLINE),
        "briefcase-account" => Some(icons::BRIEFCASE_ACCOUNT),
        "checkbox-blank-off" => Some(icons::CHECKBOX_BLANK_OFF),
        "swap-horizontal-bold" => Some(icons::SWAP_HORIZONTAL_BOLD),
        "bug-stop-outline" => Some(icons::BUG_STOP_OUTLINE),
        "animation-outline" => Some(icons::ANIMATION_OUTLINE),
        "pig-variant-outline" => Some(icons::PIG_VARIANT_OUTLINE),
        "vector-point" => Some(icons::VECTOR_POINT),
        "movie-filter-outline" => Some(icons::MOVIE_FILTER_OUTLINE),
        "camera-document-off" => Some(icons::CAMERA_DOCUMENT_OFF),
        "peanut-outline" => Some(icons::PEANUT_OUTLINE),
        "piggy-bank" => Some(icons::PIGGY_BANK),
        "shield-account-variant" => Some(icons::SHIELD_ACCOUNT_VARIANT),
        "jeepney" => Some(icons::JEEPNEY),
        "car-outline" => Some(icons::CAR_OUTLINE),
        #[allow(deprecated)]
        "language-haskell" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-haskell' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_HASKELL)
        }
        "numeric-8-box-multiple" => Some(icons::NUMERIC_8_BOX_MULTIPLE),
        "cookie-outline" => Some(icons::COOKIE_OUTLINE),
        #[allow(deprecated)]
        "patreon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'patreon' is deprecated.").print(py);
            }
            Some(icons::PATREON)
        }
        "tow-truck" => Some(icons::TOW_TRUCK),
        "network-outline" => Some(icons::NETWORK_OUTLINE),
        "hockey-puck" => Some(icons::HOCKEY_PUCK),
        "chat-minus" => Some(icons::CHAT_MINUS),
        "desk-lamp-on" => Some(icons::DESK_LAMP_ON),
        "food-steak-off" => Some(icons::FOOD_STEAK_OFF),
        "arrow-down-bold-box" => Some(icons::ARROW_DOWN_BOLD_BOX),
        "file-word-box-outline" => Some(icons::FILE_WORD_BOX_OUTLINE),
        "weather-pouring" => Some(icons::WEATHER_POURING),
        "cloud-cog" => Some(icons::CLOUD_COG),
        "truck-remove" => Some(icons::TRUCK_REMOVE),
        "fire-extinguisher" => Some(icons::FIRE_EXTINGUISHER),
        "monitor-multiple" => Some(icons::MONITOR_MULTIPLE),
        "rhombus-outline" => Some(icons::RHOMBUS_OUTLINE),
        #[allow(deprecated)]
        "meteor" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'meteor' is deprecated.").print(py);
            }
            Some(icons::METEOR)
        }
        "invoice-check" => Some(icons::INVOICE_CHECK),
        "car-pickup" => Some(icons::CAR_PICKUP),
        "kettle-steam-outline" => Some(icons::KETTLE_STEAM_OUTLINE),
        "aspect-ratio" => Some(icons::ASPECT_RATIO),
        "loupe" => Some(icons::LOUPE),
        "shower-head" => Some(icons::SHOWER_HEAD),
        "format-list-text" => Some(icons::FORMAT_LIST_TEXT),
        "filter-variant-remove" => Some(icons::FILTER_VARIANT_REMOVE),
        "shield-airplane" => Some(icons::SHIELD_AIRPLANE),
        "gesture-pinch" => Some(icons::GESTURE_PINCH),
        "view-compact" => Some(icons::VIEW_COMPACT),
        "alert-octagon-outline" => Some(icons::ALERT_OCTAGON_OUTLINE),
        "dance-ballroom" => Some(icons::DANCE_BALLROOM),
        "filter-minus-outline" => Some(icons::FILTER_MINUS_OUTLINE),
        #[allow(deprecated)]
        "language-r" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-r' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_R)
        }
        "shield-moon-outline" => Some(icons::SHIELD_MOON_OUTLINE),
        "fruit-grapes" => Some(icons::FRUIT_GRAPES),
        "water-percent" => Some(icons::WATER_PERCENT),
        "cloud-plus" => Some(icons::CLOUD_PLUS),
        "truck-alert-outline" => Some(icons::TRUCK_ALERT_OUTLINE),
        "play-box-lock-open-outline" => Some(icons::PLAY_BOX_LOCK_OPEN_OUTLINE),
        "alert" => Some(icons::ALERT),
        "decagram-outline" => Some(icons::DECAGRAM_OUTLINE),
        #[allow(deprecated)]
        "iobroker" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'iobroker' is deprecated.").print(py);
            }
            Some(icons::IOBROKER)
        }
        "rickshaw-electric" => Some(icons::RICKSHAW_ELECTRIC),
        "cloud-search-outline" => Some(icons::CLOUD_SEARCH_OUTLINE),
        "format-wrap-top-bottom" => Some(icons::FORMAT_WRAP_TOP_BOTTOM),
        "cards-playing-diamond" => Some(icons::CARDS_PLAYING_DIAMOND),
        "football" => Some(icons::FOOTBALL),
        "mower-on" => Some(icons::MOWER_ON),
        "arrow-bottom-left" => Some(icons::ARROW_BOTTOM_LEFT),
        "folder-download" => Some(icons::FOLDER_DOWNLOAD),
        "sticker-check-outline" => Some(icons::STICKER_CHECK_OUTLINE),
        "developer-board" => Some(icons::DEVELOPER_BOARD),
        "filter-plus" => Some(icons::FILTER_PLUS),
        "car-windshield" => Some(icons::CAR_WINDSHIELD),
        "snowflake-thermometer" => Some(icons::SNOWFLAKE_THERMOMETER),
        "square-edit-outline" => Some(icons::SQUARE_EDIT_OUTLINE),
        "apple-keyboard-caps" => Some(icons::APPLE_KEYBOARD_CAPS),
        "script-outline" => Some(icons::SCRIPT_OUTLINE),
        "account-file-outline" => Some(icons::ACCOUNT_FILE_OUTLINE),
        "sort-bool-descending-variant" => Some(icons::SORT_BOOL_DESCENDING_VARIANT),
        "movie-remove-outline" => Some(icons::MOVIE_REMOVE_OUTLINE),
        "invoice-outline" => Some(icons::INVOICE_OUTLINE),
        "scissors-cutting" => Some(icons::SCISSORS_CUTTING),
        "account-supervisor-circle-outline" => Some(icons::ACCOUNT_SUPERVISOR_CIRCLE_OUTLINE),
        "window-open" => Some(icons::WINDOW_OPEN),
        _ => None,
    }
}
