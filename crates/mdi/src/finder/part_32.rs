// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_32(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "folder" => Some(icons::FOLDER),
        "account-settings-outline" => Some(icons::ACCOUNT_SETTINGS_OUTLINE),
        "engine-outline" => Some(icons::ENGINE_OUTLINE),
        "rabbit-variant-outline" => Some(icons::RABBIT_VARIANT_OUTLINE),
        "strategy" => Some(icons::STRATEGY),
        "water-minus" => Some(icons::WATER_MINUS),
        "vector-difference" => Some(icons::VECTOR_DIFFERENCE),
        "cannabis-off" => Some(icons::CANNABIS_OFF),
        "chili-hot-outline" => Some(icons::CHILI_HOT_OUTLINE),
        "angle-obtuse" => Some(icons::ANGLE_OBTUSE),
        "map-marker-radius" => Some(icons::MAP_MARKER_RADIUS),
        "water-alert-outline" => Some(icons::WATER_ALERT_OUTLINE),
        "shield-half-full" => Some(icons::SHIELD_HALF_FULL),
        "lightbulb-multiple-outline" => Some(icons::LIGHTBULB_MULTIPLE_OUTLINE),
        "car-emergency" => Some(icons::CAR_EMERGENCY),
        "sort-variant-lock" => Some(icons::SORT_VARIANT_LOCK),
        "truck-delivery-outline" => Some(icons::TRUCK_DELIVERY_OUTLINE),
        "fan-off" => Some(icons::FAN_OFF),
        "cloud-refresh" => Some(icons::CLOUD_REFRESH),
        "surround-sound-2-1" => Some(icons::SURROUND_SOUND_2_1),
        "shopping-outline" => Some(icons::SHOPPING_OUTLINE),
        "view-grid" => Some(icons::VIEW_GRID),
        "tray-remove" => Some(icons::TRAY_REMOVE),
        "chat-sleep" => Some(icons::CHAT_SLEEP),
        "hexagon-slice-1" => Some(icons::HEXAGON_SLICE_1),
        "weather-night" => Some(icons::WEATHER_NIGHT),
        "coffee-to-go-outline" => Some(icons::COFFEE_TO_GO_OUTLINE),
        "check-network" => Some(icons::CHECK_NETWORK),
        "cctv" => Some(icons::CCTV),
        "alpha-n-box-outline" => Some(icons::ALPHA_N_BOX_OUTLINE),
        "shield-crown-outline" => Some(icons::SHIELD_CROWN_OUTLINE),
        "map-marker-star-outline" => Some(icons::MAP_MARKER_STAR_OUTLINE),
        "blender" => Some(icons::BLENDER),
        "folder-search" => Some(icons::FOLDER_SEARCH),
        "television-guide" => Some(icons::TELEVISION_GUIDE),
        "cloud-check-variant" => Some(icons::CLOUD_CHECK_VARIANT),
        "clippy" => Some(icons::CLIPPY),
        "archive-arrow-down" => Some(icons::ARCHIVE_ARROW_DOWN),
        "folder-key-network-outline" => Some(icons::FOLDER_KEY_NETWORK_OUTLINE),
        "compass" => Some(icons::COMPASS),
        "folder-eye" => Some(icons::FOLDER_EYE),
        "video-3d-variant" => Some(icons::VIDEO_3D_VARIANT),
        "glass-mug" => Some(icons::GLASS_MUG),
        "calendar-cursor-outline" => Some(icons::CALENDAR_CURSOR_OUTLINE),
        "calendar-expand-horizontal-outline" => Some(icons::CALENDAR_EXPAND_HORIZONTAL_OUTLINE),
        "phone-bluetooth" => Some(icons::PHONE_BLUETOOTH),
        "calendar-cursor" => Some(icons::CALENDAR_CURSOR),
        "content-save" => Some(icons::CONTENT_SAVE),
        "flask-empty" => Some(icons::FLASK_EMPTY),
        "invoice-import" => Some(icons::INVOICE_IMPORT),
        "plane-train" => Some(icons::PLANE_TRAIN),
        "arrow-split-vertical" => Some(icons::ARROW_SPLIT_VERTICAL),
        "key-change" => Some(icons::KEY_CHANGE),
        "slot-machine-outline" => Some(icons::SLOT_MACHINE_OUTLINE),
        "bullhorn-variant" => Some(icons::BULLHORN_VARIANT),
        "keyboard-backspace" => Some(icons::KEYBOARD_BACKSPACE),
        "tow-truck" => Some(icons::TOW_TRUCK),
        "newspaper" => Some(icons::NEWSPAPER),
        "file-search" => Some(icons::FILE_SEARCH),
        "bell-circle-outline" => Some(icons::BELL_CIRCLE_OUTLINE),
        "cloud-check-variant-outline" => Some(icons::CLOUD_CHECK_VARIANT_OUTLINE),
        "reminder" => Some(icons::REMINDER),
        "filter-variant-remove" => Some(icons::FILTER_VARIANT_REMOVE),
        "message-cog-outline" => Some(icons::MESSAGE_COG_OUTLINE),
        #[allow(deprecated)]
        "rollupjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'rollupjs' is deprecated.").print(py);
            }
            Some(icons::ROLLUPJS)
        }
        "clock-time-three" => Some(icons::CLOCK_TIME_THREE),
        "earth-off" => Some(icons::EARTH_OFF),
        "human-greeting" => Some(icons::HUMAN_GREETING),
        "cash-lock-open" => Some(icons::CASH_LOCK_OPEN),
        "oil-temperature" => Some(icons::OIL_TEMPERATURE),
        "human-female-female-child" => Some(icons::HUMAN_FEMALE_FEMALE_CHILD),
        "printer-pos-minus" => Some(icons::PRINTER_POS_MINUS),
        "diamond-outline" => Some(icons::DIAMOND_OUTLINE),
        "format-text-wrapping-overflow" => Some(icons::FORMAT_TEXT_WRAPPING_OVERFLOW),
        "database-cog-outline" => Some(icons::DATABASE_COG_OUTLINE),
        "barcode" => Some(icons::BARCODE),
        "pencil-minus" => Some(icons::PENCIL_MINUS),
        "qrcode" => Some(icons::QRCODE),
        "airplane-settings" => Some(icons::AIRPLANE_SETTINGS),
        "home-roof" => Some(icons::HOME_ROOF),
        "text-box-plus" => Some(icons::TEXT_BOX_PLUS),
        "cash-refund" => Some(icons::CASH_REFUND),
        "exponent-box" => Some(icons::EXPONENT_BOX),
        "shield-crown" => Some(icons::SHIELD_CROWN),
        "calendar-multiselect" => Some(icons::CALENDAR_MULTISELECT),
        "pot-mix-outline" => Some(icons::POT_MIX_OUTLINE),
        "comment-text-multiple-outline" => Some(icons::COMMENT_TEXT_MULTIPLE_OUTLINE),
        "file-arrow-left-right-outline" => Some(icons::FILE_ARROW_LEFT_RIGHT_OUTLINE),
        "star-david" => Some(icons::STAR_DAVID),
        "alpha-s-box" => Some(icons::ALPHA_S_BOX),
        "calendar-sync-outline" => Some(icons::CALENDAR_SYNC_OUTLINE),
        "battery-charging-wireless-20" => Some(icons::BATTERY_CHARGING_WIRELESS_20),
        "shield-moon-outline" => Some(icons::SHIELD_MOON_OUTLINE),
        "arrow-expand-horizontal" => Some(icons::ARROW_EXPAND_HORIZONTAL),
        "sort-alphabetical-descending-variant" => Some(icons::SORT_ALPHABETICAL_DESCENDING_VARIANT),
        "plus-circle-multiple-outline" => Some(icons::PLUS_CIRCLE_MULTIPLE_OUTLINE),
        "tooltip-question-outline" => Some(icons::TOOLTIP_QUESTION_OUTLINE),
        "foot-print" => Some(icons::FOOT_PRINT),
        "format-horizontal-align-right" => Some(icons::FORMAT_HORIZONTAL_ALIGN_RIGHT),
        "music-note-bluetooth" => Some(icons::MUSIC_NOTE_BLUETOOTH),
        "invoice-remove-outline" => Some(icons::INVOICE_REMOVE_OUTLINE),
        "stool" => Some(icons::STOOL),
        "currency-gbp" => Some(icons::CURRENCY_GBP),
        "pinwheel-outline" => Some(icons::PINWHEEL_OUTLINE),
        "car-light-dimmed" => Some(icons::CAR_LIGHT_DIMMED),
        "file-document" => Some(icons::FILE_DOCUMENT),
        "hamburger-plus" => Some(icons::HAMBURGER_PLUS),
        "table-headers-eye" => Some(icons::TABLE_HEADERS_EYE),
        "cloud-arrow-right" => Some(icons::CLOUD_ARROW_RIGHT),
        "alpha-g-circle-outline" => Some(icons::ALPHA_G_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "microsoft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT)
        }
        "file-link" => Some(icons::FILE_LINK),
        "calendar-refresh" => Some(icons::CALENDAR_REFRESH),
        "dumbbell" => Some(icons::DUMBBELL),
        "message-off-outline" => Some(icons::MESSAGE_OFF_OUTLINE),
        "heart-minus" => Some(icons::HEART_MINUS),
        "truck-alert" => Some(icons::TRUCK_ALERT),
        "dots-square" => Some(icons::DOTS_SQUARE),
        "crane" => Some(icons::CRANE),
        "car-limousine" => Some(icons::CAR_LIMOUSINE),
        "movie-edit-outline" => Some(icons::MOVIE_EDIT_OUTLINE),
        "sim-alert" => Some(icons::SIM_ALERT),
        "alpha-d" => Some(icons::ALPHA_D),
        "reorder-vertical" => Some(icons::REORDER_VERTICAL),
        "file-phone-outline" => Some(icons::FILE_PHONE_OUTLINE),
        "office-building-remove" => Some(icons::OFFICE_BUILDING_REMOVE),
        "volume-variant-off" => Some(icons::VOLUME_VARIANT_OFF),
        "fire-hydrant" => Some(icons::FIRE_HYDRANT),
        "source-commit-start-next-local" => Some(icons::SOURCE_COMMIT_START_NEXT_LOCAL),
        "emoticon-dead" => Some(icons::EMOTICON_DEAD),
        "printer-pos-off" => Some(icons::PRINTER_POS_OFF),
        "cellphone-message-off" => Some(icons::CELLPHONE_MESSAGE_OFF),
        "numeric-2-box" => Some(icons::NUMERIC_2_BOX),
        "format-list-bulleted" => Some(icons::FORMAT_LIST_BULLETED),
        "zodiac-libra" => Some(icons::ZODIAC_LIBRA),
        "gamepad-right" => Some(icons::GAMEPAD_RIGHT),
        "arrow-projectile-multiple" => Some(icons::ARROW_PROJECTILE_MULTIPLE),
        "window-close" => Some(icons::WINDOW_CLOSE),
        "equalizer-outline" => Some(icons::EQUALIZER_OUTLINE),
        "briefcase-off" => Some(icons::BRIEFCASE_OFF),
        #[allow(deprecated)]
        "twitch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitch' is deprecated.").print(py);
            }
            Some(icons::TWITCH)
        }
        "numeric-8-box-outline" => Some(icons::NUMERIC_8_BOX_OUTLINE),
        "integrated-circuit-chip" => Some(icons::INTEGRATED_CIRCUIT_CHIP),
        "human-baby-changing-table" => Some(icons::HUMAN_BABY_CHANGING_TABLE),
        "network-strength-4" => Some(icons::NETWORK_STRENGTH_4),
        "propane-tank-outline" => Some(icons::PROPANE_TANK_OUTLINE),
        "archive-marker" => Some(icons::ARCHIVE_MARKER),
        "brush-outline" => Some(icons::BRUSH_OUTLINE),
        "archive-arrow-up-outline" => Some(icons::ARCHIVE_ARROW_UP_OUTLINE),
        "flash-off-outline" => Some(icons::FLASH_OFF_OUTLINE),
        "home-outline" => Some(icons::HOME_OUTLINE),
        #[allow(deprecated)]
        "evernote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'evernote' is deprecated.").print(py);
            }
            Some(icons::EVERNOTE)
        }
        "basket-off-outline" => Some(icons::BASKET_OFF_OUTLINE),
        "temperature-fahrenheit" => Some(icons::TEMPERATURE_FAHRENHEIT),
        "radius-outline" => Some(icons::RADIUS_OUTLINE),
        "border-horizontal" => Some(icons::BORDER_HORIZONTAL),
        "message-alert" => Some(icons::MESSAGE_ALERT),
        "wind-turbine-check" => Some(icons::WIND_TURBINE_CHECK),
        #[allow(deprecated)]
        "soundcloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'soundcloud' is deprecated.").print(py);
            }
            Some(icons::SOUNDCLOUD)
        }
        "delete-forever-outline" => Some(icons::DELETE_FOREVER_OUTLINE),
        "currency-jpy" => Some(icons::CURRENCY_JPY),
        "audio-video" => Some(icons::AUDIO_VIDEO),
        "timer-lock" => Some(icons::TIMER_LOCK),
        #[allow(deprecated)]
        "fedora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'fedora' is deprecated.").print(py);
            }
            Some(icons::FEDORA)
        }
        "update" => Some(icons::UPDATE),
        "label-variant" => Some(icons::LABEL_VARIANT),
        "car-brake-alert" => Some(icons::CAR_BRAKE_ALERT),
        "shoe-cleat" => Some(icons::SHOE_CLEAT),
        "calendar-import-outline" => Some(icons::CALENDAR_IMPORT_OUTLINE),
        "menu-down" => Some(icons::MENU_DOWN),
        "amplifier-off" => Some(icons::AMPLIFIER_OFF),
        "emoticon-devil-outline" => Some(icons::EMOTICON_DEVIL_OUTLINE),
        "message-reply-text-outline" => Some(icons::MESSAGE_REPLY_TEXT_OUTLINE),
        "reload-alert" => Some(icons::RELOAD_ALERT),
        "target-variant" => Some(icons::TARGET_VARIANT),
        "egg-outline" => Some(icons::EGG_OUTLINE),
        "watermark" => Some(icons::WATERMARK),
        "store-clock" => Some(icons::STORE_CLOCK),
        "expand-all-outline" => Some(icons::EXPAND_ALL_OUTLINE),
        "cube-off" => Some(icons::CUBE_OFF),
        "palette-swatch" => Some(icons::PALETTE_SWATCH),
        "truck-cargo-container" => Some(icons::TRUCK_CARGO_CONTAINER),
        "train-car-centerbeam-full" => Some(icons::TRAIN_CAR_CENTERBEAM_FULL),
        "shield-sun-outline" => Some(icons::SHIELD_SUN_OUTLINE),
        "cloud-question-outline" => Some(icons::CLOUD_QUESTION_OUTLINE),
        "pencil-box-outline" => Some(icons::PENCIL_BOX_OUTLINE),
        "table-column-plus-after" => Some(icons::TABLE_COLUMN_PLUS_AFTER),
        "test-tube-off" => Some(icons::TEST_TUBE_OFF),
        "cart-arrow-right" => Some(icons::CART_ARROW_RIGHT),
        "console-network-outline" => Some(icons::CONSOLE_NETWORK_OUTLINE),
        #[allow(deprecated)]
        "artstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'artstation' is deprecated.").print(py);
            }
            Some(icons::ARTSTATION)
        }
        "relation-one-or-many-to-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONE),
        "bank-off-outline" => Some(icons::BANK_OFF_OUTLINE),
        "battery-lock" => Some(icons::BATTERY_LOCK),
        "archive-remove-outline" => Some(icons::ARCHIVE_REMOVE_OUTLINE),
        "folder-refresh-outline" => Some(icons::FOLDER_REFRESH_OUTLINE),
        "monitor-share" => Some(icons::MONITOR_SHARE),
        "file-download" => Some(icons::FILE_DOWNLOAD),
        "credit-card-minus-outline" => Some(icons::CREDIT_CARD_MINUS_OUTLINE),
        "autorenew-off" => Some(icons::AUTORENEW_OFF),
        _ => None,
    }
}
