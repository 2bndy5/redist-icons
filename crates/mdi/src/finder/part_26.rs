// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_26(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        #[allow(deprecated)]
        "debian" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'debian' is deprecated.").print(py);
            }
            Some(icons::DEBIAN)
        }
        "palette-swatch" => Some(icons::PALETTE_SWATCH),
        "mortar-pestle-plus" => Some(icons::MORTAR_PESTLE_PLUS),
        "disc-player" => Some(icons::DISC_PLAYER),
        "clock-alert" => Some(icons::CLOCK_ALERT),
        "monitor" => Some(icons::MONITOR),
        "chevron-right-circle" => Some(icons::CHEVRON_RIGHT_CIRCLE),
        "dialpad" => Some(icons::DIALPAD),
        "smoke-detector-off-outline" => Some(icons::SMOKE_DETECTOR_OFF_OUTLINE),
        "kettle-off-outline" => Some(icons::KETTLE_OFF_OUTLINE),
        "format-pilcrow-arrow-left" => Some(icons::FORMAT_PILCROW_ARROW_LEFT),
        "gesture-swipe-vertical" => Some(icons::GESTURE_SWIPE_VERTICAL),
        "allergy" => Some(icons::ALLERGY),
        "arrow-bottom-left-bold-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_OUTLINE),
        "flash-off" => Some(icons::FLASH_OFF),
        "beaker-minus" => Some(icons::BEAKER_MINUS),
        "nas" => Some(icons::NAS),
        "test-tube-off" => Some(icons::TEST_TUBE_OFF),
        "music-circle-outline" => Some(icons::MUSIC_CIRCLE_OUTLINE),
        "earth-remove" => Some(icons::EARTH_REMOVE),
        "help-box-multiple" => Some(icons::HELP_BOX_MULTIPLE),
        "music-rest-sixteenth" => Some(icons::MUSIC_REST_SIXTEENTH),
        "account-minus-outline" => Some(icons::ACCOUNT_MINUS_OUTLINE),
        "view-quilt-outline" => Some(icons::VIEW_QUILT_OUTLINE),
        "polaroid" => Some(icons::POLAROID),
        "ev-plug-tesla" => Some(icons::EV_PLUG_TESLA),
        "diving-flippers" => Some(icons::DIVING_FLIPPERS),
        "firework-off" => Some(icons::FIREWORK_OFF),
        "shield-home-outline" => Some(icons::SHIELD_HOME_OUTLINE),
        "numeric-10-box-outline" => Some(icons::NUMERIC_10_BOX_OUTLINE),
        "numeric-10-circle" => Some(icons::NUMERIC_10_CIRCLE),
        "camera-front-variant" => Some(icons::CAMERA_FRONT_VARIANT),
        "forwardburger" => Some(icons::FORWARDBURGER),
        "tortoise" => Some(icons::TORTOISE),
        "soundbar" => Some(icons::SOUNDBAR),
        "hair-dryer-outline" => Some(icons::HAIR_DRYER_OUTLINE),
        "temperature-celsius" => Some(icons::TEMPERATURE_CELSIUS),
        "telescope" => Some(icons::TELESCOPE),
        "music-clef-bass" => Some(icons::MUSIC_CLEF_BASS),
        "set-all" => Some(icons::SET_ALL),
        "book-refresh" => Some(icons::BOOK_REFRESH),
        "shield-alert-outline" => Some(icons::SHIELD_ALERT_OUTLINE),
        "cookie-clock-outline" => Some(icons::COOKIE_CLOCK_OUTLINE),
        "menu-left-outline" => Some(icons::MENU_LEFT_OUTLINE),
        "fridge-industrial-off-outline" => Some(icons::FRIDGE_INDUSTRIAL_OFF_OUTLINE),
        "comment-quote" => Some(icons::COMMENT_QUOTE),
        "one-up" => Some(icons::ONE_UP),
        "treasure-chest-outline" => Some(icons::TREASURE_CHEST_OUTLINE),
        "car-emergency" => Some(icons::CAR_EMERGENCY),
        "wifi-arrow-down" => Some(icons::WIFI_ARROW_DOWN),
        "cloud-download" => Some(icons::CLOUD_DOWNLOAD),
        "playlist-edit" => Some(icons::PLAYLIST_EDIT),
        "card-plus-outline" => Some(icons::CARD_PLUS_OUTLINE),
        "hammer-sickle" => Some(icons::HAMMER_SICKLE),
        "electric-switch-closed" => Some(icons::ELECTRIC_SWITCH_CLOSED),
        "account-eye-outline" => Some(icons::ACCOUNT_EYE_OUTLINE),
        "wrench-cog-outline" => Some(icons::WRENCH_COG_OUTLINE),
        "library" => Some(icons::LIBRARY),
        "application-settings-outline" => Some(icons::APPLICATION_SETTINGS_OUTLINE),
        "cellphone-message-off" => Some(icons::CELLPHONE_MESSAGE_OFF),
        "ferry" => Some(icons::FERRY),
        "wallet-travel" => Some(icons::WALLET_TRAVEL),
        "chair-rolling" => Some(icons::CHAIR_ROLLING),
        "sprinkler-fire" => Some(icons::SPRINKLER_FIRE),
        "halloween" => Some(icons::HALLOWEEN),
        "arrow-up-down" => Some(icons::ARROW_UP_DOWN),
        "alpha-q-box" => Some(icons::ALPHA_Q_BOX),
        "format-header-4" => Some(icons::FORMAT_HEADER_4),
        "size-xl" => Some(icons::SIZE_XL),
        "pencil-remove-outline" => Some(icons::PENCIL_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "ubuntu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ubuntu' is deprecated.").print(py);
            }
            Some(icons::UBUNTU)
        }
        "numeric-3-circle" => Some(icons::NUMERIC_3_CIRCLE),
        "scale-balance" => Some(icons::SCALE_BALANCE),
        "arrow-left-top-bold" => Some(icons::ARROW_LEFT_TOP_BOLD),
        "database-plus-outline" => Some(icons::DATABASE_PLUS_OUTLINE),
        "sail-boat-sink" => Some(icons::SAIL_BOAT_SINK),
        "food-drumstick-off" => Some(icons::FOOD_DRUMSTICK_OFF),
        "pencil-box-outline" => Some(icons::PENCIL_BOX_OUTLINE),
        "sync" => Some(icons::SYNC),
        "arrow-projectile" => Some(icons::ARROW_PROJECTILE),
        "car-settings" => Some(icons::CAR_SETTINGS),
        "robot-love-outline" => Some(icons::ROBOT_LOVE_OUTLINE),
        "spray-bottle" => Some(icons::SPRAY_BOTTLE),
        "arrow-top-right-thick" => Some(icons::ARROW_TOP_RIGHT_THICK),
        "screw-machine-flat-top" => Some(icons::SCREW_MACHINE_FLAT_TOP),
        "play-outline" => Some(icons::PLAY_OUTLINE),
        "battery-80-bluetooth" => Some(icons::BATTERY_80_BLUETOOTH),
        "chevron-double-left" => Some(icons::CHEVRON_DOUBLE_LEFT),
        "printer-eye" => Some(icons::PRINTER_EYE),
        "cellphone-wireless" => Some(icons::CELLPHONE_WIRELESS),
        "lambda" => Some(icons::LAMBDA),
        "fruit-citrus" => Some(icons::FRUIT_CITRUS),
        "folder-swap" => Some(icons::FOLDER_SWAP),
        "bookmark-music" => Some(icons::BOOKMARK_MUSIC),
        "format-section" => Some(icons::FORMAT_SECTION),
        "access-point-network-off" => Some(icons::ACCESS_POINT_NETWORK_OFF),
        #[allow(deprecated)]
        "android-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android-studio' is deprecated.").print(py);
            }
            Some(icons::ANDROID_STUDIO)
        }
        "bicycle-penny-farthing" => Some(icons::BICYCLE_PENNY_FARTHING),
        "camera-rear-variant" => Some(icons::CAMERA_REAR_VARIANT),
        "train-car-passenger-variant" => Some(icons::TRAIN_CAR_PASSENGER_VARIANT),
        "tire" => Some(icons::TIRE),
        "format-paragraph-spacing" => Some(icons::FORMAT_PARAGRAPH_SPACING),
        "clipboard-arrow-down-outline" => Some(icons::CLIPBOARD_ARROW_DOWN_OUTLINE),
        "card-minus-outline" => Some(icons::CARD_MINUS_OUTLINE),
        "alpha-a-box" => Some(icons::ALPHA_A_BOX),
        #[allow(deprecated)]
        "google-analytics" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-analytics' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ANALYTICS)
        }
        "email-outline" => Some(icons::EMAIL_OUTLINE),
        "select-off" => Some(icons::SELECT_OFF),
        "code-array" => Some(icons::CODE_ARRAY),
        "earth-box-remove" => Some(icons::EARTH_BOX_REMOVE),
        "gesture-spread" => Some(icons::GESTURE_SPREAD),
        "pause" => Some(icons::PAUSE),
        "cast" => Some(icons::CAST),
        "square-off" => Some(icons::SQUARE_OFF),
        "source-commit-next-local" => Some(icons::SOURCE_COMMIT_NEXT_LOCAL),
        "vacuum" => Some(icons::VACUUM),
        "alpha-u-circle" => Some(icons::ALPHA_U_CIRCLE),
        "post" => Some(icons::POST),
        "robot-mower-outline" => Some(icons::ROBOT_MOWER_OUTLINE),
        "invoice-check-outline" => Some(icons::INVOICE_CHECK_OUTLINE),
        "source-branch-remove" => Some(icons::SOURCE_BRANCH_REMOVE),
        "format-columns" => Some(icons::FORMAT_COLUMNS),
        "phone-rotate-portrait" => Some(icons::PHONE_ROTATE_PORTRAIT),
        "numeric-0-circle-outline" => Some(icons::NUMERIC_0_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "language-csharp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-csharp' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_CSHARP)
        }
        "bench-back" => Some(icons::BENCH_BACK),
        "truck-minus-outline" => Some(icons::TRUCK_MINUS_OUTLINE),
        "arrow-u-up-left" => Some(icons::ARROW_U_UP_LEFT),
        "cog-transfer-outline" => Some(icons::COG_TRANSFER_OUTLINE),
        "mouse-outline" => Some(icons::MOUSE_OUTLINE),
        "phone-missed" => Some(icons::PHONE_MISSED),
        "book-search-outline" => Some(icons::BOOK_SEARCH_OUTLINE),
        "flower-poppy" => Some(icons::FLOWER_POPPY),
        "bucket-outline" => Some(icons::BUCKET_OUTLINE),
        "star-plus-outline" => Some(icons::STAR_PLUS_OUTLINE),
        "arrow-decision" => Some(icons::ARROW_DECISION),
        "watering-can" => Some(icons::WATERING_CAN),
        "door-sliding-open" => Some(icons::DOOR_SLIDING_OPEN),
        "cookie-alert-outline" => Some(icons::COOKIE_ALERT_OUTLINE),
        "chili-medium" => Some(icons::CHILI_MEDIUM),
        "bottle-tonic-plus-outline" => Some(icons::BOTTLE_TONIC_PLUS_OUTLINE),
        #[allow(deprecated)]
        "linux" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux' is deprecated.").print(py);
            }
            Some(icons::LINUX)
        }
        "filmstrip-box" => Some(icons::FILMSTRIP_BOX),
        "briefcase-upload" => Some(icons::BRIEFCASE_UPLOAD),
        "file-cancel" => Some(icons::FILE_CANCEL),
        "invoice-text-plus" => Some(icons::INVOICE_TEXT_PLUS),
        "bowl-mix" => Some(icons::BOWL_MIX),
        "gamepad-round-right" => Some(icons::GAMEPAD_ROUND_RIGHT),
        "network-outline" => Some(icons::NETWORK_OUTLINE),
        "account-tie-voice" => Some(icons::ACCOUNT_TIE_VOICE),
        "comment-account-outline" => Some(icons::COMMENT_ACCOUNT_OUTLINE),
        "bluetooth-transfer" => Some(icons::BLUETOOTH_TRANSFER),
        "pen" => Some(icons::PEN),
        #[allow(deprecated)]
        "youtube" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE)
        }
        "snail" => Some(icons::SNAIL),
        "format-line-height" => Some(icons::FORMAT_LINE_HEIGHT),
        "radiology-box-outline" => Some(icons::RADIOLOGY_BOX_OUTLINE),
        "bookmark-remove" => Some(icons::BOOKMARK_REMOVE),
        "view-grid-plus" => Some(icons::VIEW_GRID_PLUS),
        "biathlon" => Some(icons::BIATHLON),
        "human-scooter" => Some(icons::HUMAN_SCOOTER),
        "car-3-plus" => Some(icons::CAR_3_PLUS),
        "electric-switch" => Some(icons::ELECTRIC_SWITCH),
        "eye-arrow-left" => Some(icons::EYE_ARROW_LEFT),
        "script" => Some(icons::SCRIPT),
        "dots-grid" => Some(icons::DOTS_GRID),
        "format-quote-close" => Some(icons::FORMAT_QUOTE_CLOSE),
        "car-seat-cooler" => Some(icons::CAR_SEAT_COOLER),
        "wall-sconce-round-variant-outline" => Some(icons::WALL_SCONCE_ROUND_VARIANT_OUTLINE),
        "sticker-outline" => Some(icons::STICKER_OUTLINE),
        "cellphone-lock" => Some(icons::CELLPHONE_LOCK),
        "paperclip" => Some(icons::PAPERCLIP),
        "chevron-down" => Some(icons::CHEVRON_DOWN),
        "forward" => Some(icons::FORWARD),
        "label-multiple-outline" => Some(icons::LABEL_MULTIPLE_OUTLINE),
        "exclamation-thick" => Some(icons::EXCLAMATION_THICK),
        "sticker" => Some(icons::STICKER),
        "alpha-q-box-outline" => Some(icons::ALPHA_Q_BOX_OUTLINE),
        "boom-gate" => Some(icons::BOOM_GATE),
        "flag-off" => Some(icons::FLAG_OFF),
        "clipboard-search" => Some(icons::CLIPBOARD_SEARCH),
        "micro-sd" => Some(icons::MICRO_SD),
        "wall-sconce-flat-outline" => Some(icons::WALL_SCONCE_FLAT_OUTLINE),
        "share-all" => Some(icons::SHARE_ALL),
        "invoice-arrow-left-outline" => Some(icons::INVOICE_ARROW_LEFT_OUTLINE),
        #[allow(deprecated)]
        "microsoft-windows-classic" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-windows-classic' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS_CLASSIC)
        }
        "account-multiple-minus-outline" => Some(icons::ACCOUNT_MULTIPLE_MINUS_OUTLINE),
        "shopping" => Some(icons::SHOPPING),
        "border-right-variant" => Some(icons::BORDER_RIGHT_VARIANT),
        #[allow(deprecated)]
        "android" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android' is deprecated.").print(py);
            }
            Some(icons::ANDROID)
        }
        "bridge" => Some(icons::BRIDGE),
        "octagon" => Some(icons::OCTAGON),
        "robot-industrial-outline" => Some(icons::ROBOT_INDUSTRIAL_OUTLINE),
        "airplane-off" => Some(icons::AIRPLANE_OFF),
        "head-snowflake-outline" => Some(icons::HEAD_SNOWFLAKE_OUTLINE),
        "numeric-5-circle" => Some(icons::NUMERIC_5_CIRCLE),
        "application-settings" => Some(icons::APPLICATION_SETTINGS),
        "cellphone-play" => Some(icons::CELLPHONE_PLAY),
        "heart-circle" => Some(icons::HEART_CIRCLE),
        "link-lock" => Some(icons::LINK_LOCK),
        _ => None,
    }
}
