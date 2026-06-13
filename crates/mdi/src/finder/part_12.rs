// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_12(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "battery-clock-outline" => Some(icons::BATTERY_CLOCK_OUTLINE),
        "clipboard-check-multiple-outline" => Some(icons::CLIPBOARD_CHECK_MULTIPLE_OUTLINE),
        "video-image" => Some(icons::VIDEO_IMAGE),
        "account-question" => Some(icons::ACCOUNT_QUESTION),
        "car-2-plus" => Some(icons::CAR_2_PLUS),
        "link-variant" => Some(icons::LINK_VARIANT),
        "hexagram-outline" => Some(icons::HEXAGRAM_OUTLINE),
        "fridge-off" => Some(icons::FRIDGE_OFF),
        "console-network" => Some(icons::CONSOLE_NETWORK),
        #[allow(deprecated)]
        "microsoft-onedrive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onedrive' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONEDRIVE)
        }
        "human-walker" => Some(icons::HUMAN_WALKER),
        "skateboard" => Some(icons::SKATEBOARD),
        "shield-remove" => Some(icons::SHIELD_REMOVE),
        "water-plus" => Some(icons::WATER_PLUS),
        "thermostat-cog" => Some(icons::THERMOSTAT_COG),
        "account-alert" => Some(icons::ACCOUNT_ALERT),
        "ribbon" => Some(icons::RIBBON),
        "gradient-horizontal" => Some(icons::GRADIENT_HORIZONTAL),
        "file-word-box-outline" => Some(icons::FILE_WORD_BOX_OUTLINE),
        "car-estate" => Some(icons::CAR_ESTATE),
        "movie-remove-outline" => Some(icons::MOVIE_REMOVE_OUTLINE),
        "hand-back-right" => Some(icons::HAND_BACK_RIGHT),
        "signal-2g" => Some(icons::SIGNAL_2G),
        "alarm-light" => Some(icons::ALARM_LIGHT),
        "party-popper" => Some(icons::PARTY_POPPER),
        "filter-remove" => Some(icons::FILTER_REMOVE),
        "car-traction-control" => Some(icons::CAR_TRACTION_CONTROL),
        #[allow(deprecated)]
        "electron-framework" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'electron-framework' is deprecated.")
                    .print(py);
            }
            Some(icons::ELECTRON_FRAMEWORK)
        }
        "music-note-off" => Some(icons::MUSIC_NOTE_OFF),
        "cloud-download" => Some(icons::CLOUD_DOWNLOAD),
        "format-horizontal-align-center" => Some(icons::FORMAT_HORIZONTAL_ALIGN_CENTER),
        "music-box-multiple-outline" => Some(icons::MUSIC_BOX_MULTIPLE_OUTLINE),
        "air-purifier" => Some(icons::AIR_PURIFIER),
        "wheel-barrow" => Some(icons::WHEEL_BARROW),
        "notification-clear-all" => Some(icons::NOTIFICATION_CLEAR_ALL),
        "battery-remove" => Some(icons::BATTERY_REMOVE),
        #[allow(deprecated)]
        "netflix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'netflix' is deprecated.").print(py);
            }
            Some(icons::NETFLIX)
        }
        "hard-hat" => Some(icons::HARD_HAT),
        "calculator-variant" => Some(icons::CALCULATOR_VARIANT),
        "smoking" => Some(icons::SMOKING),
        "format-text-rotation-none" => Some(icons::FORMAT_TEXT_ROTATION_NONE),
        "bicycle-penny-farthing" => Some(icons::BICYCLE_PENNY_FARTHING),
        "archive-alert" => Some(icons::ARCHIVE_ALERT),
        "tooltip-plus-outline" => Some(icons::TOOLTIP_PLUS_OUTLINE),
        "car" => Some(icons::CAR),
        "home-battery" => Some(icons::HOME_BATTERY),
        "blinds-horizontal-closed" => Some(icons::BLINDS_HORIZONTAL_CLOSED),
        "waves-arrow-left" => Some(icons::WAVES_ARROW_LEFT),
        "dome-light" => Some(icons::DOME_LIGHT),
        "sim-off" => Some(icons::SIM_OFF),
        "weight-pound" => Some(icons::WEIGHT_POUND),
        "expansion-card-variant" => Some(icons::EXPANSION_CARD_VARIANT),
        "undo-variant" => Some(icons::UNDO_VARIANT),
        "cloud-print-outline" => Some(icons::CLOUD_PRINT_OUTLINE),
        "alpha-h" => Some(icons::ALPHA_H),
        "at" => Some(icons::AT),
        "ghost-off" => Some(icons::GHOST_OFF),
        #[allow(deprecated)]
        "litecoin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'litecoin' is deprecated.").print(py);
            }
            Some(icons::LITECOIN)
        }
        "message-reply-text" => Some(icons::MESSAGE_REPLY_TEXT),
        "nut" => Some(icons::NUT),
        "resize" => Some(icons::RESIZE),
        "account-file-text-outline" => Some(icons::ACCOUNT_FILE_TEXT_OUTLINE),
        "account-tie" => Some(icons::ACCOUNT_TIE),
        "battery-40-bluetooth" => Some(icons::BATTERY_40_BLUETOOTH),
        "cart-remove" => Some(icons::CART_REMOVE),
        "floor-lamp-dual-outline" => Some(icons::FLOOR_LAMP_DUAL_OUTLINE),
        "cookie-refresh" => Some(icons::COOKIE_REFRESH),
        "alpha-g-circle" => Some(icons::ALPHA_G_CIRCLE),
        "contacts" => Some(icons::CONTACTS),
        "pine-tree-variant" => Some(icons::PINE_TREE_VARIANT),
        "temple-hindu-outline" => Some(icons::TEMPLE_HINDU_OUTLINE),
        "note-search" => Some(icons::NOTE_SEARCH),
        "pen" => Some(icons::PEN),
        "glass-wine" => Some(icons::GLASS_WINE),
        "page-layout-sidebar-left" => Some(icons::PAGE_LAYOUT_SIDEBAR_LEFT),
        "ghost-outline" => Some(icons::GHOST_OUTLINE),
        "abugida-thai" => Some(icons::ABUGIDA_THAI),
        "alarm-light-off-outline" => Some(icons::ALARM_LIGHT_OFF_OUTLINE),
        "format-paragraph" => Some(icons::FORMAT_PARAGRAPH),
        "weight-kilogram" => Some(icons::WEIGHT_KILOGRAM),
        "arrow-u-up-right-bold" => Some(icons::ARROW_U_UP_RIGHT_BOLD),
        "relation-many-to-one" => Some(icons::RELATION_MANY_TO_ONE),
        "comment-text-outline" => Some(icons::COMMENT_TEXT_OUTLINE),
        "text" => Some(icons::TEXT),
        "flower" => Some(icons::FLOWER),
        "navigation" => Some(icons::NAVIGATION),
        "content-save-edit" => Some(icons::CONTENT_SAVE_EDIT),
        "wallet-bifold-outline" => Some(icons::WALLET_BIFOLD_OUTLINE),
        "phone-hangup-outline" => Some(icons::PHONE_HANGUP_OUTLINE),
        #[allow(deprecated)]
        "drupal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'drupal' is deprecated.").print(py);
            }
            Some(icons::DRUPAL)
        }
        "power-cycle" => Some(icons::POWER_CYCLE),
        "road-variant" => Some(icons::ROAD_VARIANT),
        "home-heart" => Some(icons::HOME_HEART),
        "currency-thb" => Some(icons::CURRENCY_THB),
        "water-remove" => Some(icons::WATER_REMOVE),
        "remote" => Some(icons::REMOTE),
        "star-settings-outline" => Some(icons::STAR_SETTINGS_OUTLINE),
        "minus" => Some(icons::MINUS),
        "form-textbox-lock" => Some(icons::FORM_TEXTBOX_LOCK),
        "pipe-disconnected" => Some(icons::PIPE_DISCONNECTED),
        "invoice-fast-outline" => Some(icons::INVOICE_FAST_OUTLINE),
        "heart-half-outline" => Some(icons::HEART_HALF_OUTLINE),
        "note-remove-outline" => Some(icons::NOTE_REMOVE_OUTLINE),
        "paperclip-lock" => Some(icons::PAPERCLIP_LOCK),
        "pump-off" => Some(icons::PUMP_OFF),
        "wall-fire" => Some(icons::WALL_FIRE),
        "hexagon-slice-6" => Some(icons::HEXAGON_SLICE_6),
        "page-previous-outline" => Some(icons::PAGE_PREVIOUS_OUTLINE),
        "fit-to-screen" => Some(icons::FIT_TO_SCREEN),
        "message-badge" => Some(icons::MESSAGE_BADGE),
        "order-bool-descending-variant" => Some(icons::ORDER_BOOL_DESCENDING_VARIANT),
        "crystal-ball" => Some(icons::CRYSTAL_BALL),
        "cctv-off" => Some(icons::CCTV_OFF),
        "airplane-search" => Some(icons::AIRPLANE_SEARCH),
        "hexagon-outline" => Some(icons::HEXAGON_OUTLINE),
        "bus-school" => Some(icons::BUS_SCHOOL),
        "fast-forward-10" => Some(icons::FAST_FORWARD_10),
        "home-minus-outline" => Some(icons::HOME_MINUS_OUTLINE),
        "desktop-tower-monitor" => Some(icons::DESKTOP_TOWER_MONITOR),
        "cookie-minus" => Some(icons::COOKIE_MINUS),
        "fruit-grapes" => Some(icons::FRUIT_GRAPES),
        "lock-smart" => Some(icons::LOCK_SMART),
        "server-outline" => Some(icons::SERVER_OUTLINE),
        "server-minus" => Some(icons::SERVER_MINUS),
        "human-female-female-child" => Some(icons::HUMAN_FEMALE_FEMALE_CHILD),
        "folder-lock-open-outline" => Some(icons::FOLDER_LOCK_OPEN_OUTLINE),
        "lambda" => Some(icons::LAMBDA),
        "music-note-minus" => Some(icons::MUSIC_NOTE_MINUS),
        "server-off" => Some(icons::SERVER_OFF),
        "swap-horizontal-hidden" => Some(icons::SWAP_HORIZONTAL_HIDDEN),
        "alpha-i" => Some(icons::ALPHA_I),
        "longitude" => Some(icons::LONGITUDE),
        "calendar-sync" => Some(icons::CALENDAR_SYNC),
        "human-capacity-increase" => Some(icons::HUMAN_CAPACITY_INCREASE),
        "database-cog-outline" => Some(icons::DATABASE_COG_OUTLINE),
        "audio-input-xlr" => Some(icons::AUDIO_INPUT_XLR),
        "database-import-outline" => Some(icons::DATABASE_IMPORT_OUTLINE),
        "hospital" => Some(icons::HOSPITAL),
        "select-remove" => Some(icons::SELECT_REMOVE),
        "badge-account-horizontal-outline" => Some(icons::BADGE_ACCOUNT_HORIZONTAL_OUTLINE),
        "checkbook-arrow-right" => Some(icons::CHECKBOOK_ARROW_RIGHT),
        "account-voice" => Some(icons::ACCOUNT_VOICE),
        "memory-arrow-down" => Some(icons::MEMORY_ARROW_DOWN),
        "data-matrix-plus" => Some(icons::DATA_MATRIX_PLUS),
        "alpha-k-circle-outline" => Some(icons::ALPHA_K_CIRCLE_OUTLINE),
        "phone-minus" => Some(icons::PHONE_MINUS),
        "alpha-x-box" => Some(icons::ALPHA_X_BOX),
        "swap-horizontal-circle-outline" => Some(icons::SWAP_HORIZONTAL_CIRCLE_OUTLINE),
        "account-heart-outline" => Some(icons::ACCOUNT_HEART_OUTLINE),
        "text-long" => Some(icons::TEXT_LONG),
        "puzzle-edit" => Some(icons::PUZZLE_EDIT),
        "food-takeout-box" => Some(icons::FOOD_TAKEOUT_BOX),
        "music-accidental-sharp" => Some(icons::MUSIC_ACCIDENTAL_SHARP),
        "minus-circle-off" => Some(icons::MINUS_CIRCLE_OFF),
        "calendar-badge" => Some(icons::CALENDAR_BADGE),
        "vector-radius" => Some(icons::VECTOR_RADIUS),
        "content-save-plus" => Some(icons::CONTENT_SAVE_PLUS),
        "home-sound-out" => Some(icons::HOME_SOUND_OUT),
        "wall-sconce-flat-outline" => Some(icons::WALL_SCONCE_FLAT_OUTLINE),
        "numeric-9-plus-box" => Some(icons::NUMERIC_9_PLUS_BOX),
        "music-clef-alto" => Some(icons::MUSIC_CLEF_ALTO),
        "folder-edit" => Some(icons::FOLDER_EDIT),
        "puzzle" => Some(icons::PUZZLE),
        "gauge-full" => Some(icons::GAUGE_FULL),
        "camera-gopro" => Some(icons::CAMERA_GOPRO),
        "swap-vertical-bold" => Some(icons::SWAP_VERTICAL_BOLD),
        "gauge" => Some(icons::GAUGE),
        "hair-dryer" => Some(icons::HAIR_DRYER),
        "hoop-house" => Some(icons::HOOP_HOUSE),
        "alarm-note" => Some(icons::ALARM_NOTE),
        "robot-off" => Some(icons::ROBOT_OFF),
        "hexagon-slice-4" => Some(icons::HEXAGON_SLICE_4),
        "robber" => Some(icons::ROBBER),
        "cloud-upload-outline" => Some(icons::CLOUD_UPLOAD_OUTLINE),
        "align-vertical-bottom" => Some(icons::ALIGN_VERTICAL_BOTTOM),
        "paperclip-off" => Some(icons::PAPERCLIP_OFF),
        "numeric-4-box-multiple" => Some(icons::NUMERIC_4_BOX_MULTIPLE),
        "crop-free" => Some(icons::CROP_FREE),
        "turbine" => Some(icons::TURBINE),
        "picture-in-picture-top-right-outline" => Some(icons::PICTURE_IN_PICTURE_TOP_RIGHT_OUTLINE),
        "football" => Some(icons::FOOTBALL),
        "star-box-outline" => Some(icons::STAR_BOX_OUTLINE),
        "text-short" => Some(icons::TEXT_SHORT),
        "note" => Some(icons::NOTE),
        "micro-sd" => Some(icons::MICRO_SD),
        "garage-lock" => Some(icons::GARAGE_LOCK),
        "camera-rear" => Some(icons::CAMERA_REAR),
        "signature-text" => Some(icons::SIGNATURE_TEXT),
        "gate-and" => Some(icons::GATE_AND),
        "format-pilcrow" => Some(icons::FORMAT_PILCROW),
        "gesture-tap-box" => Some(icons::GESTURE_TAP_BOX),
        "robot-love-outline" => Some(icons::ROBOT_LOVE_OUTLINE),
        "thermometer-high" => Some(icons::THERMOMETER_HIGH),
        "file-question" => Some(icons::FILE_QUESTION),
        "human-wheelchair" => Some(icons::HUMAN_WHEELCHAIR),
        "code-not-equal" => Some(icons::CODE_NOT_EQUAL),
        "speaker" => Some(icons::SPEAKER),
        "snowboard" => Some(icons::SNOWBOARD),
        "wallet-travel" => Some(icons::WALLET_TRAVEL),
        "format-overline" => Some(icons::FORMAT_OVERLINE),
        _ => None,
    }
}
