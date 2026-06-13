// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_21(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "head-check" => Some(icons::HEAD_CHECK),
        "washing-machine-alert" => Some(icons::WASHING_MACHINE_ALERT),
        "alpha-c" => Some(icons::ALPHA_C),
        "flower-pollen" => Some(icons::FLOWER_POLLEN),
        "file-outline" => Some(icons::FILE_OUTLINE),
        "home-account" => Some(icons::HOME_ACCOUNT),
        "message-arrow-right" => Some(icons::MESSAGE_ARROW_RIGHT),
        "human-male-height-variant" => Some(icons::HUMAN_MALE_HEIGHT_VARIANT),
        "briefcase-arrow-left-right-outline" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT_OUTLINE),
        "keg" => Some(icons::KEG),
        "math-sin" => Some(icons::MATH_SIN),
        "water-minus" => Some(icons::WATER_MINUS),
        "arrow-oscillating-off" => Some(icons::ARROW_OSCILLATING_OFF),
        "tag-off-outline" => Some(icons::TAG_OFF_OUTLINE),
        #[allow(deprecated)]
        "language-r" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-r' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_R)
        }
        "hand-clap-off" => Some(icons::HAND_CLAP_OFF),
        "emoticon-sad" => Some(icons::EMOTICON_SAD),
        "folder-swap" => Some(icons::FOLDER_SWAP),
        "window-close" => Some(icons::WINDOW_CLOSE),
        "pail-remove" => Some(icons::PAIL_REMOVE),
        "atom-variant" => Some(icons::ATOM_VARIANT),
        "cart-outline" => Some(icons::CART_OUTLINE),
        "package-variant-closed-remove" => Some(icons::PACKAGE_VARIANT_CLOSED_REMOVE),
        "cards-diamond-outline" => Some(icons::CARDS_DIAMOND_OUTLINE),
        "open-in-app" => Some(icons::OPEN_IN_APP),
        "bench-back" => Some(icons::BENCH_BACK),
        "selection-multiple" => Some(icons::SELECTION_MULTIPLE),
        "hammer-screwdriver" => Some(icons::HAMMER_SCREWDRIVER),
        "decimal-comma-increase" => Some(icons::DECIMAL_COMMA_INCREASE),
        "alarm-multiple" => Some(icons::ALARM_MULTIPLE),
        "account-minus-outline" => Some(icons::ACCOUNT_MINUS_OUTLINE),
        "cart-check" => Some(icons::CART_CHECK),
        "emoticon-frown" => Some(icons::EMOTICON_FROWN),
        "content-save-minus" => Some(icons::CONTENT_SAVE_MINUS),
        "timer-minus-outline" => Some(icons::TIMER_MINUS_OUTLINE),
        "border-top-variant" => Some(icons::BORDER_TOP_VARIANT),
        "magnify-scan" => Some(icons::MAGNIFY_SCAN),
        "credit-card-fast-outline" => Some(icons::CREDIT_CARD_FAST_OUTLINE),
        "car-wash" => Some(icons::CAR_WASH),
        "multiplication-box" => Some(icons::MULTIPLICATION_BOX),
        "pencil-plus" => Some(icons::PENCIL_PLUS),
        "battery-bluetooth" => Some(icons::BATTERY_BLUETOOTH),
        "alpha-h-circle-outline" => Some(icons::ALPHA_H_CIRCLE_OUTLINE),
        "cupboard-outline" => Some(icons::CUPBOARD_OUTLINE),
        "alpha-w-circle" => Some(icons::ALPHA_W_CIRCLE),
        "database-cog" => Some(icons::DATABASE_COG),
        "folder-star" => Some(icons::FOLDER_STAR),
        "wave-arrow-up" => Some(icons::WAVE_ARROW_UP),
        "email-minus" => Some(icons::EMAIL_MINUS),
        "bus-sign" => Some(icons::BUS_SIGN),
        "moon-full" => Some(icons::MOON_FULL),
        "account-box-minus-outline" => Some(icons::ACCOUNT_BOX_MINUS_OUTLINE),
        "briefcase-minus-outline" => Some(icons::BRIEFCASE_MINUS_OUTLINE),
        "movie-open" => Some(icons::MOVIE_OPEN),
        "mower-bag-on" => Some(icons::MOWER_BAG_ON),
        "lan" => Some(icons::LAN),
        "truck-snowflake" => Some(icons::TRUCK_SNOWFLAKE),
        "television" => Some(icons::TELEVISION),
        "alert-plus" => Some(icons::ALERT_PLUS),
        "sync-alert" => Some(icons::SYNC_ALERT),
        "battery-sync-outline" => Some(icons::BATTERY_SYNC_OUTLINE),
        "table-merge-cells" => Some(icons::TABLE_MERGE_CELLS),
        "image-lock-outline" => Some(icons::IMAGE_LOCK_OUTLINE),
        "currency-kzt" => Some(icons::CURRENCY_KZT),
        #[allow(deprecated)]
        "humble-bundle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'humble-bundle' is deprecated.").print(py);
            }
            Some(icons::HUMBLE_BUNDLE)
        }
        "train-car-box-full" => Some(icons::TRAIN_CAR_BOX_FULL),
        "currency-rial" => Some(icons::CURRENCY_RIAL),
        "food-off-outline" => Some(icons::FOOD_OFF_OUTLINE),
        "fan-plus" => Some(icons::FAN_PLUS),
        "flash-triangle-outline" => Some(icons::FLASH_TRIANGLE_OUTLINE),
        "boom-gate-up" => Some(icons::BOOM_GATE_UP),
        "order-alphabetical-descending" => Some(icons::ORDER_ALPHABETICAL_DESCENDING),
        "circle-double" => Some(icons::CIRCLE_DOUBLE),
        "archive-arrow-up" => Some(icons::ARCHIVE_ARROW_UP),
        "fridge-industrial-alert-outline" => Some(icons::FRIDGE_INDUSTRIAL_ALERT_OUTLINE),
        "silverware-fork" => Some(icons::SILVERWARE_FORK),
        "ev-plug-type1" => Some(icons::EV_PLUG_TYPE1),
        "cast-connected" => Some(icons::CAST_CONNECTED),
        #[allow(deprecated)]
        "goodreads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'goodreads' is deprecated.").print(py);
            }
            Some(icons::GOODREADS)
        }
        "signature-freehand" => Some(icons::SIGNATURE_FREEHAND),
        "qrcode-edit" => Some(icons::QRCODE_EDIT),
        "motion-play-outline" => Some(icons::MOTION_PLAY_OUTLINE),
        "alpha-m" => Some(icons::ALPHA_M),
        "animation-outline" => Some(icons::ANIMATION_OUTLINE),
        "human-dolly" => Some(icons::HUMAN_DOLLY),
        "television-speaker-off" => Some(icons::TELEVISION_SPEAKER_OFF),
        "account-arrow-left-outline" => Some(icons::ACCOUNT_ARROW_LEFT_OUTLINE),
        "fruit-citrus" => Some(icons::FRUIT_CITRUS),
        "alpha-x" => Some(icons::ALPHA_X),
        "dice-6" => Some(icons::DICE_6),
        "format-italic" => Some(icons::FORMAT_ITALIC),
        "cards-spade-outline" => Some(icons::CARDS_SPADE_OUTLINE),
        "book-play-outline" => Some(icons::BOOK_PLAY_OUTLINE),
        #[allow(deprecated)]
        "microsoft-dynamics-365" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-dynamics-365' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_DYNAMICS_365)
        }
        "account-child" => Some(icons::ACCOUNT_CHILD),
        "rename-box" => Some(icons::RENAME_BOX),
        "eye-settings" => Some(icons::EYE_SETTINGS),
        "human-female" => Some(icons::HUMAN_FEMALE),
        "link-lock" => Some(icons::LINK_LOCK),
        "cradle" => Some(icons::CRADLE),
        "phone-classic-off" => Some(icons::PHONE_CLASSIC_OFF),
        "robot" => Some(icons::ROBOT),
        "octagram-plus" => Some(icons::OCTAGRAM_PLUS),
        "magnify-plus" => Some(icons::MAGNIFY_PLUS),
        "tooltip-plus" => Some(icons::TOOLTIP_PLUS),
        "shield-cross" => Some(icons::SHIELD_CROSS),
        "butterfly" => Some(icons::BUTTERFLY),
        "train" => Some(icons::TRAIN),
        "robot-confused-outline" => Some(icons::ROBOT_CONFUSED_OUTLINE),
        "cursor-pointer" => Some(icons::CURSOR_POINTER),
        "email-seal" => Some(icons::EMAIL_SEAL),
        "wall-sconce" => Some(icons::WALL_SCONCE),
        "vector-bezier" => Some(icons::VECTOR_BEZIER),
        "file-jpg-box" => Some(icons::FILE_JPG_BOX),
        "triforce" => Some(icons::TRIFORCE),
        "gesture-swipe-left" => Some(icons::GESTURE_SWIPE_LEFT),
        "bulldozer" => Some(icons::BULLDOZER),
        "numeric-6-circle" => Some(icons::NUMERIC_6_CIRCLE),
        "information-variant" => Some(icons::INFORMATION_VARIANT),
        "book-arrow-down" => Some(icons::BOOK_ARROW_DOWN),
        "application-variable-outline" => Some(icons::APPLICATION_VARIABLE_OUTLINE),
        "history" => Some(icons::HISTORY),
        "golf" => Some(icons::GOLF),
        "image-filter-hdr" => Some(icons::IMAGE_FILTER_HDR),
        "phone-forward-outline" => Some(icons::PHONE_FORWARD_OUTLINE),
        "view-dashboard-edit-outline" => Some(icons::VIEW_DASHBOARD_EDIT_OUTLINE),
        "format-superscript" => Some(icons::FORMAT_SUPERSCRIPT),
        "notebook-minus-outline" => Some(icons::NOTEBOOK_MINUS_OUTLINE),
        "format-font-size-decrease" => Some(icons::FORMAT_FONT_SIZE_DECREASE),
        "dice-3-outline" => Some(icons::DICE_3_OUTLINE),
        "delete-forever" => Some(icons::DELETE_FOREVER),
        "battery-60-bluetooth" => Some(icons::BATTERY_60_BLUETOOTH),
        "phone-plus-outline" => Some(icons::PHONE_PLUS_OUTLINE),
        "cake-variant" => Some(icons::CAKE_VARIANT),
        "segment" => Some(icons::SEGMENT),
        "plus-box-multiple" => Some(icons::PLUS_BOX_MULTIPLE),
        "projector-screen-variant-off-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF_OUTLINE),
        "note-alert-outline" => Some(icons::NOTE_ALERT_OUTLINE),
        "cloud-plus-outline" => Some(icons::CLOUD_PLUS_OUTLINE),
        "account-supervisor-outline" => Some(icons::ACCOUNT_SUPERVISOR_OUTLINE),
        "file-hidden" => Some(icons::FILE_HIDDEN),
        "wall-sconce-outline" => Some(icons::WALL_SCONCE_OUTLINE),
        "serial-port" => Some(icons::SERIAL_PORT),
        "domain-plus" => Some(icons::DOMAIN_PLUS),
        #[allow(deprecated)]
        "language-typescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-typescript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_TYPESCRIPT)
        }
        "alpha-d-box-outline" => Some(icons::ALPHA_D_BOX_OUTLINE),
        "send-outline" => Some(icons::SEND_OUTLINE),
        "tray-minus" => Some(icons::TRAY_MINUS),
        "alpha-v-box" => Some(icons::ALPHA_V_BOX),
        #[allow(deprecated)]
        "facebook-workplace" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-workplace' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_WORKPLACE)
        }
        "card-account-mail" => Some(icons::CARD_ACCOUNT_MAIL),
        "dice-d20" => Some(icons::DICE_D20),
        "plus-box-multiple-outline" => Some(icons::PLUS_BOX_MULTIPLE_OUTLINE),
        "bell-sleep-outline" => Some(icons::BELL_SLEEP_OUTLINE),
        "rice" => Some(icons::RICE),
        "speedometer" => Some(icons::SPEEDOMETER),
        "human-greeting" => Some(icons::HUMAN_GREETING),
        "timer-remove" => Some(icons::TIMER_REMOVE),
        "layers-search-outline" => Some(icons::LAYERS_SEARCH_OUTLINE),
        "square-root-box" => Some(icons::SQUARE_ROOT_BOX),
        "hand-wash" => Some(icons::HAND_WASH),
        "battery-charging-wireless-70" => Some(icons::BATTERY_CHARGING_WIRELESS_70),
        "eye-remove-outline" => Some(icons::EYE_REMOVE_OUTLINE),
        "lock-remove-outline" => Some(icons::LOCK_REMOVE_OUTLINE),
        "transit-detour" => Some(icons::TRANSIT_DETOUR),
        "circle-edit-outline" => Some(icons::CIRCLE_EDIT_OUTLINE),
        "check" => Some(icons::CHECK),
        "car-door" => Some(icons::CAR_DOOR),
        "format-text-rotation-angle-down" => Some(icons::FORMAT_TEXT_ROTATION_ANGLE_DOWN),
        "robot-industrial-outline" => Some(icons::ROBOT_INDUSTRIAL_OUTLINE),
        "relation-zero-or-one-to-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_MANY),
        "parachute-outline" => Some(icons::PARACHUTE_OUTLINE),
        "bus-wrench" => Some(icons::BUS_WRENCH),
        "calendar-end" => Some(icons::CALENDAR_END),
        "egg-outline" => Some(icons::EGG_OUTLINE),
        "bug-stop" => Some(icons::BUG_STOP),
        "safe-square-outline" => Some(icons::SAFE_SQUARE_OUTLINE),
        "file-image-plus-outline" => Some(icons::FILE_IMAGE_PLUS_OUTLINE),
        "fan-speed-2" => Some(icons::FAN_SPEED_2),
        "fan-speed-1" => Some(icons::FAN_SPEED_1),
        "chevron-left-box" => Some(icons::CHEVRON_LEFT_BOX),
        "temple-buddhist-outline" => Some(icons::TEMPLE_BUDDHIST_OUTLINE),
        "weather-windy" => Some(icons::WEATHER_WINDY),
        "monitor-account" => Some(icons::MONITOR_ACCOUNT),
        "boom-gate" => Some(icons::BOOM_GATE),
        "movie-star-outline" => Some(icons::MOVIE_STAR_OUTLINE),
        "paw" => Some(icons::PAW),
        "map-marker-left" => Some(icons::MAP_MARKER_LEFT),
        "waves-arrow-up" => Some(icons::WAVES_ARROW_UP),
        "satellite-uplink" => Some(icons::SATELLITE_UPLINK),
        "filter-off" => Some(icons::FILTER_OFF),
        "upload-circle-outline" => Some(icons::UPLOAD_CIRCLE_OUTLINE),
        "generator-stationary" => Some(icons::GENERATOR_STATIONARY),
        "phone-bluetooth-outline" => Some(icons::PHONE_BLUETOOTH_OUTLINE),
        "store-off-outline" => Some(icons::STORE_OFF_OUTLINE),
        "file" => Some(icons::FILE),
        "phone-log-outline" => Some(icons::PHONE_LOG_OUTLINE),
        #[allow(deprecated)]
        "dolby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dolby' is deprecated.").print(py);
            }
            Some(icons::DOLBY)
        }
        "printer-pos-refresh-outline" => Some(icons::PRINTER_POS_REFRESH_OUTLINE),
        "hydraulic-oil-temperature" => Some(icons::HYDRAULIC_OIL_TEMPERATURE),
        _ => None,
    }
}
