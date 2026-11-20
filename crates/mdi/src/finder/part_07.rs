// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_7(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "ray-start-end" => Some(icons::RAY_START_END),
        "arrow-up-bold-outline" => Some(icons::ARROW_UP_BOLD_OUTLINE),
        "dice-d20" => Some(icons::DICE_D20),
        "protocol" => Some(icons::PROTOCOL),
        "av-timer" => Some(icons::AV_TIMER),
        "rodent" => Some(icons::RODENT),
        "camera" => Some(icons::CAMERA),
        "cards-playing-club" => Some(icons::CARDS_PLAYING_CLUB),
        "message-reply-outline" => Some(icons::MESSAGE_REPLY_OUTLINE),
        "mouse-move-up" => Some(icons::MOUSE_MOVE_UP),
        "arrow-u-right-bottom" => Some(icons::ARROW_U_RIGHT_BOTTOM),
        "shield-check" => Some(icons::SHIELD_CHECK),
        "tennis-ball" => Some(icons::TENNIS_BALL),
        "cog-clockwise" => Some(icons::COG_CLOCKWISE),
        "file-clock" => Some(icons::FILE_CLOCK),
        "sim" => Some(icons::SIM),
        "filmstrip-box-multiple" => Some(icons::FILMSTRIP_BOX_MULTIPLE),
        "phone-refresh" => Some(icons::PHONE_REFRESH),
        #[allow(deprecated)]
        "material-ui" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-ui' is deprecated.").print(py);
            }
            Some(icons::MATERIAL_UI)
        }
        "filmstrip-off" => Some(icons::FILMSTRIP_OFF),
        "silverware-clean" => Some(icons::SILVERWARE_CLEAN),
        "rotate-left" => Some(icons::ROTATE_LEFT),
        "numeric-5-box-outline" => Some(icons::NUMERIC_5_BOX_OUTLINE),
        "account-supervisor" => Some(icons::ACCOUNT_SUPERVISOR),
        "alpha-o-box" => Some(icons::ALPHA_O_BOX),
        "tag-arrow-up" => Some(icons::TAG_ARROW_UP),
        "weight-gram" => Some(icons::WEIGHT_GRAM),
        "chart-ppf" => Some(icons::CHART_PPF),
        "table-sync" => Some(icons::TABLE_SYNC),
        "quadcopter" => Some(icons::QUADCOPTER),
        "exit-to-app" => Some(icons::EXIT_TO_APP),
        "crowd" => Some(icons::CROWD),
        #[allow(deprecated)]
        "google-classroom" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-classroom' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CLASSROOM)
        }
        "pail-minus-outline" => Some(icons::PAIL_MINUS_OUTLINE),
        "white-balance-iridescent" => Some(icons::WHITE_BALANCE_IRIDESCENT),
        "tools" => Some(icons::TOOLS),
        "lightbulb-auto-outline" => Some(icons::LIGHTBULB_AUTO_OUTLINE),
        "car-brake-retarder" => Some(icons::CAR_BRAKE_RETARDER),
        "train-bus" => Some(icons::TRAIN_BUS),
        "pause-octagon-outline" => Some(icons::PAUSE_OCTAGON_OUTLINE),
        "snowflake-alert" => Some(icons::SNOWFLAKE_ALERT),
        "video-marker-outline" => Some(icons::VIDEO_MARKER_OUTLINE),
        "face-woman" => Some(icons::FACE_WOMAN),
        "minus-circle" => Some(icons::MINUS_CIRCLE),
        "invoice-minus-outline" => Some(icons::INVOICE_MINUS_OUTLINE),
        "dice-d10" => Some(icons::DICE_D10),
        "palette-swatch-variant" => Some(icons::PALETTE_SWATCH_VARIANT),
        "weather-sunny" => Some(icons::WEATHER_SUNNY),
        "rugby" => Some(icons::RUGBY),
        "folder-image" => Some(icons::FOLDER_IMAGE),
        "pencil-minus-outline" => Some(icons::PENCIL_MINUS_OUTLINE),
        "snowshoeing" => Some(icons::SNOWSHOEING),
        "notebook-edit" => Some(icons::NOTEBOOK_EDIT),
        "television-pause" => Some(icons::TELEVISION_PAUSE),
        "keyboard-f8" => Some(icons::KEYBOARD_F8),
        "hammer-sickle" => Some(icons::HAMMER_SICKLE),
        "music-clef-bass" => Some(icons::MUSIC_CLEF_BASS),
        "folder-lock-open-outline" => Some(icons::FOLDER_LOCK_OPEN_OUTLINE),
        "solar-power" => Some(icons::SOLAR_POWER),
        "restore" => Some(icons::RESTORE),
        "locker" => Some(icons::LOCKER),
        "circle-slice-6" => Some(icons::CIRCLE_SLICE_6),
        "wifi-arrow-down" => Some(icons::WIFI_ARROW_DOWN),
        "speaker-pause" => Some(icons::SPEAKER_PAUSE),
        "television-box" => Some(icons::TELEVISION_BOX),
        "fast-forward-5" => Some(icons::FAST_FORWARD_5),
        "tray-arrow-up" => Some(icons::TRAY_ARROW_UP),
        "backburger" => Some(icons::BACKBURGER),
        "toothbrush-electric" => Some(icons::TOOTHBRUSH_ELECTRIC),
        "database-lock" => Some(icons::DATABASE_LOCK),
        "elevation-rise" => Some(icons::ELEVATION_RISE),
        "lightbulb-on-50" => Some(icons::LIGHTBULB_ON_50),
        "lightbulb-spot-off" => Some(icons::LIGHTBULB_SPOT_OFF),
        "car-info" => Some(icons::CAR_INFO),
        "inbox-arrow-down-outline" => Some(icons::INBOX_ARROW_DOWN_OUTLINE),
        "presentation-play" => Some(icons::PRESENTATION_PLAY),
        "octagram-edit" => Some(icons::OCTAGRAM_EDIT),
        "needle" => Some(icons::NEEDLE),
        "flask-round-bottom-empty" => Some(icons::FLASK_ROUND_BOTTOM_EMPTY),
        "amplifier" => Some(icons::AMPLIFIER),
        "arrow-bottom-right-bold-outline" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_OUTLINE),
        "comment-bookmark-outline" => Some(icons::COMMENT_BOOKMARK_OUTLINE),
        "keyboard-settings-outline" => Some(icons::KEYBOARD_SETTINGS_OUTLINE),
        "phone-cancel" => Some(icons::PHONE_CANCEL),
        "layers-remove" => Some(icons::LAYERS_REMOVE),
        "source-branch-check" => Some(icons::SOURCE_BRANCH_CHECK),
        "code-greater-than-or-equal" => Some(icons::CODE_GREATER_THAN_OR_EQUAL),
        "fit-to-page-outline" => Some(icons::FIT_TO_PAGE_OUTLINE),
        "orbit-variant" => Some(icons::ORBIT_VARIANT),
        "symbol" => Some(icons::SYMBOL),
        "relation-one-or-many-to-many" => Some(icons::RELATION_ONE_OR_MANY_TO_MANY),
        "arrow-top-left-bottom-right" => Some(icons::ARROW_TOP_LEFT_BOTTOM_RIGHT),
        "help-circle" => Some(icons::HELP_CIRCLE),
        "seesaw" => Some(icons::SEESAW),
        #[allow(deprecated)]
        "pokemon-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pokemon-go' is deprecated.").print(py);
            }
            Some(icons::POKEMON_GO)
        }
        "close-network-outline" => Some(icons::CLOSE_NETWORK_OUTLINE),
        "atm" => Some(icons::ATM),
        "call-made" => Some(icons::CALL_MADE),
        "alpha-z-circle" => Some(icons::ALPHA_Z_CIRCLE),
        "crown" => Some(icons::CROWN),
        "border-right-variant" => Some(icons::BORDER_RIGHT_VARIANT),
        "lock-percent-open" => Some(icons::LOCK_PERCENT_OPEN),
        "golf-cart" => Some(icons::GOLF_CART),
        "heart-circle" => Some(icons::HEART_CIRCLE),
        "access-point-network-off" => Some(icons::ACCESS_POINT_NETWORK_OFF),
        "receipt-clock-outline" => Some(icons::RECEIPT_CLOCK_OUTLINE),
        "email-sync-outline" => Some(icons::EMAIL_SYNC_OUTLINE),
        "umbrella-outline" => Some(icons::UMBRELLA_OUTLINE),
        "palette-outline" => Some(icons::PALETTE_OUTLINE),
        #[allow(deprecated)]
        "ubuntu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ubuntu' is deprecated.").print(py);
            }
            Some(icons::UBUNTU)
        }
        "skull-scan-outline" => Some(icons::SKULL_SCAN_OUTLINE),
        "layers-triple" => Some(icons::LAYERS_TRIPLE),
        "camera-timer" => Some(icons::CAMERA_TIMER),
        "contain" => Some(icons::CONTAIN),
        "hammer" => Some(icons::HAMMER),
        "tag-multiple-outline" => Some(icons::TAG_MULTIPLE_OUTLINE),
        "hard-hat" => Some(icons::HARD_HAT),
        "human-wheelchair" => Some(icons::HUMAN_WHEELCHAIR),
        "message-fast-outline" => Some(icons::MESSAGE_FAST_OUTLINE),
        "eye-lock" => Some(icons::EYE_LOCK),
        "toaster" => Some(icons::TOASTER),
        "cloud-arrow-down-outline" => Some(icons::CLOUD_ARROW_DOWN_OUTLINE),
        #[allow(deprecated)]
        "nintendo-wii" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wii' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WII)
        }
        "graph-outline" => Some(icons::GRAPH_OUTLINE),
        "roman-numeral-8" => Some(icons::ROMAN_NUMERAL_8),
        "account-lock-open" => Some(icons::ACCOUNT_LOCK_OPEN),
        #[allow(deprecated)]
        "angular" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'angular' is deprecated.").print(py);
            }
            Some(icons::ANGULAR)
        }
        "printer-wireless" => Some(icons::PRINTER_WIRELESS),
        "lightbulb-group-off" => Some(icons::LIGHTBULB_GROUP_OFF),
        "water-well-outline" => Some(icons::WATER_WELL_OUTLINE),
        "view-column-outline" => Some(icons::VIEW_COLUMN_OUTLINE),
        "printer-pos-plus" => Some(icons::PRINTER_POS_PLUS),
        "video" => Some(icons::VIDEO),
        "arrow-top-left-bottom-right-bold" => Some(icons::ARROW_TOP_LEFT_BOTTOM_RIGHT_BOLD),
        "filter-outline" => Some(icons::FILTER_OUTLINE),
        "vector-curve" => Some(icons::VECTOR_CURVE),
        "toothbrush-paste" => Some(icons::TOOTHBRUSH_PASTE),
        "selection-ellipse-arrow-inside" => Some(icons::SELECTION_ELLIPSE_ARROW_INSIDE),
        "bus-wrench" => Some(icons::BUS_WRENCH),
        "camera-flip-outline" => Some(icons::CAMERA_FLIP_OUTLINE),
        "eye-off-outline" => Some(icons::EYE_OFF_OUTLINE),
        "snowflake-off" => Some(icons::SNOWFLAKE_OFF),
        "calendar-export" => Some(icons::CALENDAR_EXPORT),
        "turkey" => Some(icons::TURKEY),
        "pyramid-off" => Some(icons::PYRAMID_OFF),
        "file-swap" => Some(icons::FILE_SWAP),
        "storefront-check" => Some(icons::STOREFRONT_CHECK),
        "backspace" => Some(icons::BACKSPACE),
        "fridge-top" => Some(icons::FRIDGE_TOP),
        "store-outline" => Some(icons::STORE_OUTLINE),
        "note-edit-outline" => Some(icons::NOTE_EDIT_OUTLINE),
        "state-machine" => Some(icons::STATE_MACHINE),
        "alphabet-aurebesh" => Some(icons::ALPHABET_AUREBESH),
        "compare" => Some(icons::COMPARE),
        "snowflake" => Some(icons::SNOWFLAKE),
        "elevator-passenger-outline" => Some(icons::ELEVATOR_PASSENGER_OUTLINE),
        "steering" => Some(icons::STEERING),
        "server-minus" => Some(icons::SERVER_MINUS),
        "shoe-print" => Some(icons::SHOE_PRINT),
        "star-remove-outline" => Some(icons::STAR_REMOVE_OUTLINE),
        "seat-legroom-normal" => Some(icons::SEAT_LEGROOM_NORMAL),
        "account-heart-outline" => Some(icons::ACCOUNT_HEART_OUTLINE),
        "bug-pause-outline" => Some(icons::BUG_PAUSE_OUTLINE),
        "star-four-points-small" => Some(icons::STAR_FOUR_POINTS_SMALL),
        "square-medium" => Some(icons::SQUARE_MEDIUM),
        "boom-gate-arrow-up-outline" => Some(icons::BOOM_GATE_ARROW_UP_OUTLINE),
        "message-check-outline" => Some(icons::MESSAGE_CHECK_OUTLINE),
        "account-multiple-check-outline" => Some(icons::ACCOUNT_MULTIPLE_CHECK_OUTLINE),
        "bullseye" => Some(icons::BULLSEYE),
        "deskphone" => Some(icons::DESKPHONE),
        "comment-bookmark" => Some(icons::COMMENT_BOOKMARK),
        "archive-check" => Some(icons::ARCHIVE_CHECK),
        "pan-top-right" => Some(icons::PAN_TOP_RIGHT),
        "check-decagram-outline" => Some(icons::CHECK_DECAGRAM_OUTLINE),
        "ceiling-light-multiple-outline" => Some(icons::CEILING_LIGHT_MULTIPLE_OUTLINE),
        "battery-charging-wireless-60" => Some(icons::BATTERY_CHARGING_WIRELESS_60),
        "camera-enhance" => Some(icons::CAMERA_ENHANCE),
        "bank-off" => Some(icons::BANK_OFF),
        "square-edit-outline" => Some(icons::SQUARE_EDIT_OUTLINE),
        "smoke-detector-alert" => Some(icons::SMOKE_DETECTOR_ALERT),
        "lightbulb-variant" => Some(icons::LIGHTBULB_VARIANT),
        "format-wrap-tight" => Some(icons::FORMAT_WRAP_TIGHT),
        "selection-multiple" => Some(icons::SELECTION_MULTIPLE),
        "grid-off" => Some(icons::GRID_OFF),
        "treasure-chest-outline" => Some(icons::TREASURE_CHEST_OUTLINE),
        "sofa" => Some(icons::SOFA),
        "one-up" => Some(icons::ONE_UP),
        "motorbike-off" => Some(icons::MOTORBIKE_OFF),
        "file-minus" => Some(icons::FILE_MINUS),
        "plus-network" => Some(icons::PLUS_NETWORK),
        "shield-off" => Some(icons::SHIELD_OFF),
        "alert-rhombus-outline" => Some(icons::ALERT_RHOMBUS_OUTLINE),
        "car-tire-alert" => Some(icons::CAR_TIRE_ALERT),
        "circle" => Some(icons::CIRCLE),
        "bus-articulated-front" => Some(icons::BUS_ARTICULATED_FRONT),
        "checkbox-multiple-marked-circle-outline" => {
            Some(icons::CHECKBOX_MULTIPLE_MARKED_CIRCLE_OUTLINE)
        }
        "alpha-q-box-outline" => Some(icons::ALPHA_Q_BOX_OUTLINE),
        "garage-open-variant" => Some(icons::GARAGE_OPEN_VARIANT),
        "comment-remove-outline" => Some(icons::COMMENT_REMOVE_OUTLINE),
        "server-plus" => Some(icons::SERVER_PLUS),
        _ => None,
    }
}
