// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_14(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "shimmer" => Some(icons::SHIMMER),
        "seed" => Some(icons::SEED),
        "numeric-8-box" => Some(icons::NUMERIC_8_BOX),
        "email-lock" => Some(icons::EMAIL_LOCK),
        "fishbowl-outline" => Some(icons::FISHBOWL_OUTLINE),
        "chevron-left-circle-outline" => Some(icons::CHEVRON_LEFT_CIRCLE_OUTLINE),
        "account-alert-outline" => Some(icons::ACCOUNT_ALERT_OUTLINE),
        "keyboard-settings-outline" => Some(icons::KEYBOARD_SETTINGS_OUTLINE),
        "alarm-check" => Some(icons::ALARM_CHECK),
        "flask-remove" => Some(icons::FLASK_REMOVE),
        "wifi-strength-outline" => Some(icons::WIFI_STRENGTH_OUTLINE),
        "folder-remove-outline" => Some(icons::FOLDER_REMOVE_OUTLINE),
        "forest" => Some(icons::FOREST),
        #[allow(deprecated)]
        "webpack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webpack' is deprecated.").print(py);
            }
            Some(icons::WEBPACK)
        }
        "floor-lamp" => Some(icons::FLOOR_LAMP),
        "source-merge" => Some(icons::SOURCE_MERGE),
        "file-lock-open" => Some(icons::FILE_LOCK_OPEN),
        "baby-face" => Some(icons::BABY_FACE),
        "gate" => Some(icons::GATE),
        "attachment-remove" => Some(icons::ATTACHMENT_REMOVE),
        "greater-than-or-equal" => Some(icons::GREATER_THAN_OR_EQUAL),
        "timer-lock" => Some(icons::TIMER_LOCK),
        "bowl-mix-outline" => Some(icons::BOWL_MIX_OUTLINE),
        "sync-circle" => Some(icons::SYNC_CIRCLE),
        "tooltip-minus" => Some(icons::TOOLTIP_MINUS),
        "movie-open-edit-outline" => Some(icons::MOVIE_OPEN_EDIT_OUTLINE),
        "arrow-decision-outline" => Some(icons::ARROW_DECISION_OUTLINE),
        "relation-many-to-many" => Some(icons::RELATION_MANY_TO_MANY),
        "refresh-circle" => Some(icons::REFRESH_CIRCLE),
        "thermometer-low" => Some(icons::THERMOMETER_LOW),
        "pail-outline" => Some(icons::PAIL_OUTLINE),
        "credit-card-outline" => Some(icons::CREDIT_CARD_OUTLINE),
        "passport-biometric" => Some(icons::PASSPORT_BIOMETRIC),
        "link-edit" => Some(icons::LINK_EDIT),
        "lightbulb-alert-outline" => Some(icons::LIGHTBULB_ALERT_OUTLINE),
        "chat-outline" => Some(icons::CHAT_OUTLINE),
        "package-variant" => Some(icons::PACKAGE_VARIANT),
        "send-variant-clock-outline" => Some(icons::SEND_VARIANT_CLOCK_OUTLINE),
        "bus-stop" => Some(icons::BUS_STOP),
        "map-marker-star-outline" => Some(icons::MAP_MARKER_STAR_OUTLINE),
        "undo" => Some(icons::UNDO),
        "play-network" => Some(icons::PLAY_NETWORK),
        "comment-quote-outline" => Some(icons::COMMENT_QUOTE_OUTLINE),
        "folder-download-outline" => Some(icons::FOLDER_DOWNLOAD_OUTLINE),
        "battery-charging-wireless-30" => Some(icons::BATTERY_CHARGING_WIRELESS_30),
        "gavel" => Some(icons::GAVEL),
        "sim-alert" => Some(icons::SIM_ALERT),
        "receipt-text-remove" => Some(icons::RECEIPT_TEXT_REMOVE),
        "account-multiple-remove" => Some(icons::ACCOUNT_MULTIPLE_REMOVE),
        "plane-train" => Some(icons::PLANE_TRAIN),
        "controller-classic" => Some(icons::CONTROLLER_CLASSIC),
        "wifi-strength-3-alert" => Some(icons::WIFI_STRENGTH_3_ALERT),
        "database-arrow-down-outline" => Some(icons::DATABASE_ARROW_DOWN_OUTLINE),
        "router-network" => Some(icons::ROUTER_NETWORK),
        #[allow(deprecated)]
        "ansible" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ansible' is deprecated.").print(py);
            }
            Some(icons::ANSIBLE)
        }
        "progress-pencil" => Some(icons::PROGRESS_PENCIL),
        "lightbulb-off-outline" => Some(icons::LIGHTBULB_OFF_OUTLINE),
        "relation-one-or-many-to-many" => Some(icons::RELATION_ONE_OR_MANY_TO_MANY),
        "alpha-d" => Some(icons::ALPHA_D),
        "database-search" => Some(icons::DATABASE_SEARCH),
        "view-module" => Some(icons::VIEW_MODULE),
        "bottle-soda" => Some(icons::BOTTLE_SODA),
        "upload-off-outline" => Some(icons::UPLOAD_OFF_OUTLINE),
        "archive-settings" => Some(icons::ARCHIVE_SETTINGS),
        "printer-3d-nozzle-off-outline" => Some(icons::PRINTER_3D_NOZZLE_OFF_OUTLINE),
        "coffee-maker" => Some(icons::COFFEE_MAKER),
        "headset-dock" => Some(icons::HEADSET_DOCK),
        "format-quote-close" => Some(icons::FORMAT_QUOTE_CLOSE),
        "sigma-lower" => Some(icons::SIGMA_LOWER),
        "lightbulb-auto-outline" => Some(icons::LIGHTBULB_AUTO_OUTLINE),
        "sort-clock-ascending" => Some(icons::SORT_CLOCK_ASCENDING),
        "star-box-multiple" => Some(icons::STAR_BOX_MULTIPLE),
        "size-m" => Some(icons::SIZE_M),
        #[allow(deprecated)]
        "language-css3" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-css3' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CSS3)
        }
        "folder-key-outline" => Some(icons::FOLDER_KEY_OUTLINE),
        "pine-tree-box" => Some(icons::PINE_TREE_BOX),
        #[allow(deprecated)]
        "gmail" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gmail' is deprecated.").print(py);
            }
            Some(icons::GMAIL)
        }
        "directions" => Some(icons::DIRECTIONS),
        "gate-arrow-right" => Some(icons::GATE_ARROW_RIGHT),
        "remote-off" => Some(icons::REMOTE_OFF),
        "bench" => Some(icons::BENCH),
        "zodiac-taurus" => Some(icons::ZODIAC_TAURUS),
        "script-text" => Some(icons::SCRIPT_TEXT),
        "message-image-outline" => Some(icons::MESSAGE_IMAGE_OUTLINE),
        "weather-partly-lightning" => Some(icons::WEATHER_PARTLY_LIGHTNING),
        "sprinkler-fire" => Some(icons::SPRINKLER_FIRE),
        "leak" => Some(icons::LEAK),
        "campfire" => Some(icons::CAMPFIRE),
        "scale-unbalanced" => Some(icons::SCALE_UNBALANCED),
        "receipt-send-outline" => Some(icons::RECEIPT_SEND_OUTLINE),
        "folder-off" => Some(icons::FOLDER_OFF),
        "file-eye" => Some(icons::FILE_EYE),
        "tooltip-text" => Some(icons::TOOLTIP_TEXT),
        "desktop-classic" => Some(icons::DESKTOP_CLASSIC),
        "bell-ring" => Some(icons::BELL_RING),
        "lightning-bolt-outline" => Some(icons::LIGHTNING_BOLT_OUTLINE),
        "message-bulleted" => Some(icons::MESSAGE_BULLETED),
        "watch-vibrate" => Some(icons::WATCH_VIBRATE),
        "movie-open-check" => Some(icons::MOVIE_OPEN_CHECK),
        "sun-clock" => Some(icons::SUN_CLOCK),
        "view-carousel" => Some(icons::VIEW_CAROUSEL),
        "circle-expand" => Some(icons::CIRCLE_EXPAND),
        "newspaper" => Some(icons::NEWSPAPER),
        "locker-multiple" => Some(icons::LOCKER_MULTIPLE),
        "file-sign" => Some(icons::FILE_SIGN),
        "barn" => Some(icons::BARN),
        "door" => Some(icons::DOOR),
        "cloud-lock-open-outline" => Some(icons::CLOUD_LOCK_OPEN_OUTLINE),
        "alpha-e-circle-outline" => Some(icons::ALPHA_E_CIRCLE_OUTLINE),
        "flag-plus-outline" => Some(icons::FLAG_PLUS_OUTLINE),
        "hand-cycle" => Some(icons::HAND_CYCLE),
        "image-filter-center-focus-strong-outline" => {
            Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG_OUTLINE)
        }
        "numeric-8-box-outline" => Some(icons::NUMERIC_8_BOX_OUTLINE),
        "smog" => Some(icons::SMOG),
        "folder-multiple-plus-outline" => Some(icons::FOLDER_MULTIPLE_PLUS_OUTLINE),
        "controller-off" => Some(icons::CONTROLLER_OFF),
        "table-picnic" => Some(icons::TABLE_PICNIC),
        "volcano-outline" => Some(icons::VOLCANO_OUTLINE),
        "excavator" => Some(icons::EXCAVATOR),
        "arrow-up-left" => Some(icons::ARROW_UP_LEFT),
        "eye-lock-open-outline" => Some(icons::EYE_LOCK_OPEN_OUTLINE),
        "room-service" => Some(icons::ROOM_SERVICE),
        "brightness-percent" => Some(icons::BRIGHTNESS_PERCENT),
        "clipboard-alert-outline" => Some(icons::CLIPBOARD_ALERT_OUTLINE),
        "calendar-filter-outline" => Some(icons::CALENDAR_FILTER_OUTLINE),
        "bus-clock" => Some(icons::BUS_CLOCK),
        "phone-off-outline" => Some(icons::PHONE_OFF_OUTLINE),
        "camera-metering-center" => Some(icons::CAMERA_METERING_CENTER),
        "filter-off-outline" => Some(icons::FILTER_OFF_OUTLINE),
        "fridge-top" => Some(icons::FRIDGE_TOP),
        "cloud-print" => Some(icons::CLOUD_PRINT),
        "directions-fork" => Some(icons::DIRECTIONS_FORK),
        "camera-lock-open-outline" => Some(icons::CAMERA_LOCK_OPEN_OUTLINE),
        "store" => Some(icons::STORE),
        "account-supervisor-circle-outline" => Some(icons::ACCOUNT_SUPERVISOR_CIRCLE_OUTLINE),
        "key-wireless" => Some(icons::KEY_WIRELESS),
        "cog-transfer" => Some(icons::COG_TRANSFER),
        "home-remove" => Some(icons::HOME_REMOVE),
        "code-braces-box" => Some(icons::CODE_BRACES_BOX),
        "view-dashboard-variant" => Some(icons::VIEW_DASHBOARD_VARIANT),
        "invoice-text-arrow-right-outline" => Some(icons::INVOICE_TEXT_ARROW_RIGHT_OUTLINE),
        "emoticon-dead-outline" => Some(icons::EMOTICON_DEAD_OUTLINE),
        "bullhorn" => Some(icons::BULLHORN),
        "heart-remove" => Some(icons::HEART_REMOVE),
        "layers" => Some(icons::LAYERS),
        "fan-clock" => Some(icons::FAN_CLOCK),
        "alpha-j-circle" => Some(icons::ALPHA_J_CIRCLE),
        "alpha-g" => Some(icons::ALPHA_G),
        "flask-minus-outline" => Some(icons::FLASK_MINUS_OUTLINE),
        "numeric-9-plus-box-multiple-outline" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE_OUTLINE),
        "account-cash-outline" => Some(icons::ACCOUNT_CASH_OUTLINE),
        "postage-stamp" => Some(icons::POSTAGE_STAMP),
        "printer-pos-cog" => Some(icons::PRINTER_POS_COG),
        "cloud-alert-outline" => Some(icons::CLOUD_ALERT_OUTLINE),
        "tape-measure" => Some(icons::TAPE_MEASURE),
        "alpha-n-circle-outline" => Some(icons::ALPHA_N_CIRCLE_OUTLINE),
        "candycane" => Some(icons::CANDYCANE),
        "hand-front-right" => Some(icons::HAND_FRONT_RIGHT),
        "ship-wheel" => Some(icons::SHIP_WHEEL),
        "leek" => Some(icons::LEEK),
        "smoke-detector-outline" => Some(icons::SMOKE_DETECTOR_OUTLINE),
        "arrow-left-right" => Some(icons::ARROW_LEFT_RIGHT),
        "office-building-marker" => Some(icons::OFFICE_BUILDING_MARKER),
        "cosine-wave" => Some(icons::COSINE_WAVE),
        "solar-power-variant" => Some(icons::SOLAR_POWER_VARIANT),
        "email-alert-outline" => Some(icons::EMAIL_ALERT_OUTLINE),
        "server-minus-outline" => Some(icons::SERVER_MINUS_OUTLINE),
        "car-cruise-control" => Some(icons::CAR_CRUISE_CONTROL),
        "compost" => Some(icons::COMPOST),
        "camera-marker-outline" => Some(icons::CAMERA_MARKER_OUTLINE),
        "silverware-clean" => Some(icons::SILVERWARE_CLEAN),
        "cricket" => Some(icons::CRICKET),
        #[allow(deprecated)]
        "microsoft-xbox-controller" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER)
        }
        "gauge-low" => Some(icons::GAUGE_LOW),
        "navigation-variant-outline" => Some(icons::NAVIGATION_VARIANT_OUTLINE),
        "file-eye-outline" => Some(icons::FILE_EYE_OUTLINE),
        "face-man-shimmer-outline" => Some(icons::FACE_MAN_SHIMMER_OUTLINE),
        "router-network-wireless" => Some(icons::ROUTER_NETWORK_WIRELESS),
        "pin-off" => Some(icons::PIN_OFF),
        "bus" => Some(icons::BUS),
        "pan-top-right" => Some(icons::PAN_TOP_RIGHT),
        "crown-circle" => Some(icons::CROWN_CIRCLE),
        "plus-box" => Some(icons::PLUS_BOX),
        "alpha-w" => Some(icons::ALPHA_W),
        "chat-minus-outline" => Some(icons::CHAT_MINUS_OUTLINE),
        "valve" => Some(icons::VALVE),
        "fridge-alert-outline" => Some(icons::FRIDGE_ALERT_OUTLINE),
        "fridge-off-outline" => Some(icons::FRIDGE_OFF_OUTLINE),
        "receipt-text-minus-outline" => Some(icons::RECEIPT_TEXT_MINUS_OUTLINE),
        "cog-off" => Some(icons::COG_OFF),
        "phone-settings" => Some(icons::PHONE_SETTINGS),
        "fish" => Some(icons::FISH),
        "select-color" => Some(icons::SELECT_COLOR),
        "relation-one-to-many" => Some(icons::RELATION_ONE_TO_MANY),
        "calendar-weekend-outline" => Some(icons::CALENDAR_WEEKEND_OUTLINE),
        "human-white-cane" => Some(icons::HUMAN_WHITE_CANE),
        "mosque-outline" => Some(icons::MOSQUE_OUTLINE),
        "battery-unknown-bluetooth" => Some(icons::BATTERY_UNKNOWN_BLUETOOTH),
        #[allow(deprecated)]
        "angularjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'angularjs' is deprecated.").print(py);
            }
            Some(icons::ANGULARJS)
        }
        "peanut" => Some(icons::PEANUT),
        _ => None,
    }
}
