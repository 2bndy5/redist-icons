// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_12(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "rake" => Some(icons::RAKE),
        "arrow-left-drop-circle-outline" => Some(icons::ARROW_LEFT_DROP_CIRCLE_OUTLINE),
        "store-clock" => Some(icons::STORE_CLOCK),
        "hook" => Some(icons::HOOK),
        "diameter" => Some(icons::DIAMETER),
        "message-alert-outline" => Some(icons::MESSAGE_ALERT_OUTLINE),
        "seat-recline-extra" => Some(icons::SEAT_RECLINE_EXTRA),
        "lock-percent" => Some(icons::LOCK_PERCENT),
        "hexagram-outline" => Some(icons::HEXAGRAM_OUTLINE),
        "eject-outline" => Some(icons::EJECT_OUTLINE),
        "format-text-rotation-down-vertical" => Some(icons::FORMAT_TEXT_ROTATION_DOWN_VERTICAL),
        #[allow(deprecated)]
        "rollupjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'rollupjs' is deprecated.").print(py);
            }
            Some(icons::ROLLUPJS)
        }
        "eight-track" => Some(icons::EIGHT_TRACK),
        "cookie-cog-outline" => Some(icons::COOKIE_COG_OUTLINE),
        "keyboard-backspace" => Some(icons::KEYBOARD_BACKSPACE),
        "archive-remove-outline" => Some(icons::ARCHIVE_REMOVE_OUTLINE),
        "information-variant-box" => Some(icons::INFORMATION_VARIANT_BOX),
        "train-car-centerbeam" => Some(icons::TRAIN_CAR_CENTERBEAM),
        "car-brake-hold" => Some(icons::CAR_BRAKE_HOLD),
        "monitor-screenshot" => Some(icons::MONITOR_SCREENSHOT),
        "bottle-tonic-plus" => Some(icons::BOTTLE_TONIC_PLUS),
        "keyboard-caps" => Some(icons::KEYBOARD_CAPS),
        "disc-alert" => Some(icons::DISC_ALERT),
        "car-speed-limiter" => Some(icons::CAR_SPEED_LIMITER),
        "air-conditioner" => Some(icons::AIR_CONDITIONER),
        "flag-triangle" => Some(icons::FLAG_TRIANGLE),
        "information-variant-box-outline" => Some(icons::INFORMATION_VARIANT_BOX_OUTLINE),
        "map-marker-alert" => Some(icons::MAP_MARKER_ALERT),
        "clock-time-ten" => Some(icons::CLOCK_TIME_TEN),
        "clipboard-arrow-up-outline" => Some(icons::CLIPBOARD_ARROW_UP_OUTLINE),
        "code-block-tags" => Some(icons::CODE_BLOCK_TAGS),
        "electric-switch" => Some(icons::ELECTRIC_SWITCH),
        "battery-bluetooth-variant" => Some(icons::BATTERY_BLUETOOTH_VARIANT),
        "sign-direction" => Some(icons::SIGN_DIRECTION),
        "send-variant-outline" => Some(icons::SEND_VARIANT_OUTLINE),
        "shredder" => Some(icons::SHREDDER),
        "thermometer-chevron-up" => Some(icons::THERMOMETER_CHEVRON_UP),
        "resistor" => Some(icons::RESISTOR),
        "ceiling-light-outline" => Some(icons::CEILING_LIGHT_OUTLINE),
        "monitor-star" => Some(icons::MONITOR_STAR),
        "hospital" => Some(icons::HOSPITAL),
        "cup-off" => Some(icons::CUP_OFF),
        "shield-home-outline" => Some(icons::SHIELD_HOME_OUTLINE),
        "cards-playing-spade-outline" => Some(icons::CARDS_PLAYING_SPADE_OUTLINE),
        "format-indent-increase" => Some(icons::FORMAT_INDENT_INCREASE),
        "form-textbox" => Some(icons::FORM_TEXTBOX),
        "file-image-minus-outline" => Some(icons::FILE_IMAGE_MINUS_OUTLINE),
        "music-note-off-outline" => Some(icons::MUSIC_NOTE_OFF_OUTLINE),
        "send" => Some(icons::SEND),
        #[allow(deprecated)]
        "language-c" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-c' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_C)
        }
        "receipt-text-remove-outline" => Some(icons::RECEIPT_TEXT_REMOVE_OUTLINE),
        "percent" => Some(icons::PERCENT),
        "application-settings" => Some(icons::APPLICATION_SETTINGS),
        "volume-vibrate" => Some(icons::VOLUME_VIBRATE),
        "shape-rectangle-plus" => Some(icons::SHAPE_RECTANGLE_PLUS),
        "gradient-horizontal" => Some(icons::GRADIENT_HORIZONTAL),
        "shield-refresh-outline" => Some(icons::SHIELD_REFRESH_OUTLINE),
        "lamps" => Some(icons::LAMPS),
        "numeric-3-box-multiple" => Some(icons::NUMERIC_3_BOX_MULTIPLE),
        "receipt-text-minus-outline" => Some(icons::RECEIPT_TEXT_MINUS_OUTLINE),
        "cursor-default-gesture-outline" => Some(icons::CURSOR_DEFAULT_GESTURE_OUTLINE),
        "movie-off-outline" => Some(icons::MOVIE_OFF_OUTLINE),
        "umbrella-closed-variant" => Some(icons::UMBRELLA_CLOSED_VARIANT),
        "tag-multiple" => Some(icons::TAG_MULTIPLE),
        "source-commit-start" => Some(icons::SOURCE_COMMIT_START),
        "tag-arrow-left-outline" => Some(icons::TAG_ARROW_LEFT_OUTLINE),
        "bathtub-outline" => Some(icons::BATHTUB_OUTLINE),
        "cloud-outline" => Some(icons::CLOUD_OUTLINE),
        "gamepad-round-up" => Some(icons::GAMEPAD_ROUND_UP),
        "format-line-weight" => Some(icons::FORMAT_LINE_WEIGHT),
        "food-drumstick-outline" => Some(icons::FOOD_DRUMSTICK_OUTLINE),
        #[allow(deprecated)]
        "google-keep" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-keep' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_KEEP)
        }
        "image-move" => Some(icons::IMAGE_MOVE),
        "wallet-membership" => Some(icons::WALLET_MEMBERSHIP),
        "motorbike" => Some(icons::MOTORBIKE),
        "application-outline" => Some(icons::APPLICATION_OUTLINE),
        "cup-outline" => Some(icons::CUP_OUTLINE),
        "egg" => Some(icons::EGG),
        "format-wrap-square" => Some(icons::FORMAT_WRAP_SQUARE),
        "midi-port" => Some(icons::MIDI_PORT),
        "reload-alert" => Some(icons::RELOAD_ALERT),
        "size-xxs" => Some(icons::SIZE_XXS),
        "backup-restore" => Some(icons::BACKUP_RESTORE),
        "bread-slice" => Some(icons::BREAD_SLICE),
        "selection-multiple-marker" => Some(icons::SELECTION_MULTIPLE_MARKER),
        "synagogue" => Some(icons::SYNAGOGUE),
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
        "exclamation-thick" => Some(icons::EXCLAMATION_THICK),
        "arrow-u-left-bottom" => Some(icons::ARROW_U_LEFT_BOTTOM),
        "pulse" => Some(icons::PULSE),
        "gift-open" => Some(icons::GIFT_OPEN),
        "set-left-right" => Some(icons::SET_LEFT_RIGHT),
        "led-strip-variant-off" => Some(icons::LED_STRIP_VARIANT_OFF),
        "car-emergency" => Some(icons::CAR_EMERGENCY),
        "store-plus-outline" => Some(icons::STORE_PLUS_OUTLINE),
        "blur-linear" => Some(icons::BLUR_LINEAR),
        "alpha-g" => Some(icons::ALPHA_G),
        "train" => Some(icons::TRAIN),
        "alpha-y" => Some(icons::ALPHA_Y),
        "briefcase-minus-outline" => Some(icons::BRIEFCASE_MINUS_OUTLINE),
        "cog-play-outline" => Some(icons::COG_PLAY_OUTLINE),
        "electric-switch-closed" => Some(icons::ELECTRIC_SWITCH_CLOSED),
        "zodiac-aries" => Some(icons::ZODIAC_ARIES),
        "set-left" => Some(icons::SET_LEFT),
        "moon-waning-crescent" => Some(icons::MOON_WANING_CRESCENT),
        "alpha-w-circle-outline" => Some(icons::ALPHA_W_CIRCLE_OUTLINE),
        "vector-square" => Some(icons::VECTOR_SQUARE),
        "car-electric" => Some(icons::CAR_ELECTRIC),
        "tooltip-edit" => Some(icons::TOOLTIP_EDIT),
        "dots-vertical" => Some(icons::DOTS_VERTICAL),
        "backspace-reverse" => Some(icons::BACKSPACE_REVERSE),
        "lightbulb-on-60" => Some(icons::LIGHTBULB_ON_60),
        "arrow-left-right-bold-outline" => Some(icons::ARROW_LEFT_RIGHT_BOLD_OUTLINE),
        "download-multiple-outline" => Some(icons::DOWNLOAD_MULTIPLE_OUTLINE),
        "checkbook" => Some(icons::CHECKBOOK),
        "identifier" => Some(icons::IDENTIFIER),
        "eyedropper-off" => Some(icons::EYEDROPPER_OFF),
        "basket-plus" => Some(icons::BASKET_PLUS),
        #[allow(deprecated)]
        "github" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'github' is deprecated.").print(py);
            }
            Some(icons::GITHUB)
        }
        "tooltip-image" => Some(icons::TOOLTIP_IMAGE),
        "printer-pos-minus-outline" => Some(icons::PRINTER_POS_MINUS_OUTLINE),
        "table" => Some(icons::TABLE),
        "memory" => Some(icons::MEMORY),
        "checkbox-multiple-blank-outline" => Some(icons::CHECKBOX_MULTIPLE_BLANK_OUTLINE),
        "television-play" => Some(icons::TELEVISION_PLAY),
        "thumb-down-outline" => Some(icons::THUMB_DOWN_OUTLINE),
        "briefcase-variant" => Some(icons::BRIEFCASE_VARIANT),
        "open-in-app" => Some(icons::OPEN_IN_APP),
        "pinwheel" => Some(icons::PINWHEEL),
        "store-alert-outline" => Some(icons::STORE_ALERT_OUTLINE),
        "alert-circle-check" => Some(icons::ALERT_CIRCLE_CHECK),
        "car-seat-heater" => Some(icons::CAR_SEAT_HEATER),
        "receipt-text-check-outline" => Some(icons::RECEIPT_TEXT_CHECK_OUTLINE),
        "set-center" => Some(icons::SET_CENTER),
        #[allow(deprecated)]
        "android" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android' is deprecated.").print(py);
            }
            Some(icons::ANDROID)
        }
        "bed-queen" => Some(icons::BED_QUEEN),
        "hand-front-right-outline" => Some(icons::HAND_FRONT_RIGHT_OUTLINE),
        "augmented-reality" => Some(icons::AUGMENTED_REALITY),
        "flash-outline" => Some(icons::FLASH_OUTLINE),
        "pencil-off" => Some(icons::PENCIL_OFF),
        "arrow-up" => Some(icons::ARROW_UP),
        "alpha-p-box-outline" => Some(icons::ALPHA_P_BOX_OUTLINE),
        "restart-off" => Some(icons::RESTART_OFF),
        "car-brake-temperature" => Some(icons::CAR_BRAKE_TEMPERATURE),
        "backspace-reverse-outline" => Some(icons::BACKSPACE_REVERSE_OUTLINE),
        "ellipse" => Some(icons::ELLIPSE),
        "clock-time-eight" => Some(icons::CLOCK_TIME_EIGHT),
        "lock-open-alert" => Some(icons::LOCK_OPEN_ALERT),
        "comment-search" => Some(icons::COMMENT_SEARCH),
        "alpha-x-circle" => Some(icons::ALPHA_X_CIRCLE),
        "printer-3d-nozzle-alert-outline" => Some(icons::PRINTER_3D_NOZZLE_ALERT_OUTLINE),
        "wave-arrow-up" => Some(icons::WAVE_ARROW_UP),
        "cash-minus" => Some(icons::CASH_MINUS),
        "snowflake-melt" => Some(icons::SNOWFLAKE_MELT),
        "book-open-outline" => Some(icons::BOOK_OPEN_OUTLINE),
        "toothbrush-electric" => Some(icons::TOOTHBRUSH_ELECTRIC),
        "chart-bar" => Some(icons::CHART_BAR),
        "image-filter-none" => Some(icons::IMAGE_FILTER_NONE),
        "selection-search" => Some(icons::SELECTION_SEARCH),
        "chart-ppf" => Some(icons::CHART_PPF),
        "battery-plus-variant" => Some(icons::BATTERY_PLUS_VARIANT),
        "chart-sankey" => Some(icons::CHART_SANKEY),
        "table-arrow-left" => Some(icons::TABLE_ARROW_LEFT),
        "key-alert" => Some(icons::KEY_ALERT),
        "web-cancel" => Some(icons::WEB_CANCEL),
        "battery-10-bluetooth" => Some(icons::BATTERY_10_BLUETOOTH),
        "cloud-sync-outline" => Some(icons::CLOUD_SYNC_OUTLINE),
        "seed" => Some(icons::SEED),
        "alpha-b-box-outline" => Some(icons::ALPHA_B_BOX_OUTLINE),
        "repeat" => Some(icons::REPEAT),
        "scan-helper" => Some(icons::SCAN_HELPER),
        "magazine-pistol" => Some(icons::MAGAZINE_PISTOL),
        "tshirt-v-outline" => Some(icons::TSHIRT_V_OUTLINE),
        "restore" => Some(icons::RESTORE),
        "flask-plus" => Some(icons::FLASK_PLUS),
        "spotlight" => Some(icons::SPOTLIGHT),
        "arrow-oscillating-off" => Some(icons::ARROW_OSCILLATING_OFF),
        "play-circle-outline" => Some(icons::PLAY_CIRCLE_OUTLINE),
        "battery-charging" => Some(icons::BATTERY_CHARGING),
        "progress-alert" => Some(icons::PROGRESS_ALERT),
        "message-draw" => Some(icons::MESSAGE_DRAW),
        "hand-back-left" => Some(icons::HAND_BACK_LEFT),
        "cctv" => Some(icons::CCTV),
        "wrench-cog" => Some(icons::WRENCH_COG),
        "pool-thermometer" => Some(icons::POOL_THERMOMETER),
        "sun-wireless" => Some(icons::SUN_WIRELESS),
        "arrow-top-left-thin" => Some(icons::ARROW_TOP_LEFT_THIN),
        "room-service" => Some(icons::ROOM_SERVICE),
        "cloud-percent" => Some(icons::CLOUD_PERCENT),
        "pinwheel-outline" => Some(icons::PINWHEEL_OUTLINE),
        "distribute-vertical-center" => Some(icons::DISTRIBUTE_VERTICAL_CENTER),
        "hydrogen-station" => Some(icons::HYDROGEN_STATION),
        "cookie-lock-outline" => Some(icons::COOKIE_LOCK_OUTLINE),
        "dice-1" => Some(icons::DICE_1),
        "radius" => Some(icons::RADIUS),
        "mower-bag-on" => Some(icons::MOWER_BAG_ON),
        "cpu-32-bit" => Some(icons::CPU_32_BIT),
        "arrow-collapse-vertical" => Some(icons::ARROW_COLLAPSE_VERTICAL),
        "screen-rotation-lock" => Some(icons::SCREEN_ROTATION_LOCK),
        "billiards-rack" => Some(icons::BILLIARDS_RACK),
        _ => None,
    }
}
