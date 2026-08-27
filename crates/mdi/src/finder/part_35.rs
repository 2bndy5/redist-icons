// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_35(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "data-matrix" => Some(icons::DATA_MATRIX),
        "upload-outline" => Some(icons::UPLOAD_OUTLINE),
        "battery-30-bluetooth" => Some(icons::BATTERY_30_BLUETOOTH),
        "crop-free" => Some(icons::CROP_FREE),
        "invoice-text-arrow-right" => Some(icons::INVOICE_TEXT_ARROW_RIGHT),
        "numeric-9-plus-box-multiple-outline" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE_OUTLINE),
        "eye-settings-outline" => Some(icons::EYE_SETTINGS_OUTLINE),
        "alpha-h-circle-outline" => Some(icons::ALPHA_H_CIRCLE_OUTLINE),
        "headphones" => Some(icons::HEADPHONES),
        "printer-pos-remove-outline" => Some(icons::PRINTER_POS_REMOVE_OUTLINE),
        "gymnastics" => Some(icons::GYMNASTICS),
        "message-alert" => Some(icons::MESSAGE_ALERT),
        "arrow-up-down-bold" => Some(icons::ARROW_UP_DOWN_BOLD),
        "shield-sun-outline" => Some(icons::SHIELD_SUN_OUTLINE),
        "wheel-barrow" => Some(icons::WHEEL_BARROW),
        "waves-arrow-left" => Some(icons::WAVES_ARROW_LEFT),
        "skull-scan" => Some(icons::SKULL_SCAN),
        "plus-circle-multiple" => Some(icons::PLUS_CIRCLE_MULTIPLE),
        "pool" => Some(icons::POOL),
        "cog-pause" => Some(icons::COG_PAUSE),
        "invoice-send" => Some(icons::INVOICE_SEND),
        "spray-bottle" => Some(icons::SPRAY_BOTTLE),
        "palette-outline" => Some(icons::PALETTE_OUTLINE),
        "gamepad-circle-down" => Some(icons::GAMEPAD_CIRCLE_DOWN),
        "file-chart-check-outline" => Some(icons::FILE_CHART_CHECK_OUTLINE),
        "soldering-iron" => Some(icons::SOLDERING_IRON),
        "database-arrow-up-outline" => Some(icons::DATABASE_ARROW_UP_OUTLINE),
        "alpha-p-circle-outline" => Some(icons::ALPHA_P_CIRCLE_OUTLINE),
        "leek" => Some(icons::LEEK),
        "alpha-k" => Some(icons::ALPHA_K),
        "chat-minus-outline" => Some(icons::CHAT_MINUS_OUTLINE),
        #[allow(deprecated)]
        "reddit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'reddit' is deprecated.").print(py);
            }
            Some(icons::REDDIT)
        }
        "code-greater-than-or-equal" => Some(icons::CODE_GREATER_THAN_OR_EQUAL),
        "numeric-9-plus-circle" => Some(icons::NUMERIC_9_PLUS_CIRCLE),
        "arrow-up-bold-box" => Some(icons::ARROW_UP_BOLD_BOX),
        "sim-outline" => Some(icons::SIM_OUTLINE),
        "nfc-variant-off" => Some(icons::NFC_VARIANT_OFF),
        "palette-advanced" => Some(icons::PALETTE_ADVANCED),
        "compass-rose" => Some(icons::COMPASS_ROSE),
        "bed-queen-outline" => Some(icons::BED_QUEEN_OUTLINE),
        "text-shadow" => Some(icons::TEXT_SHADOW),
        "math-integral" => Some(icons::MATH_INTEGRAL),
        "image-filter-vintage" => Some(icons::IMAGE_FILTER_VINTAGE),
        "menu" => Some(icons::MENU),
        "currency-usd-off" => Some(icons::CURRENCY_USD_OFF),
        "lamp-outline" => Some(icons::LAMP_OUTLINE),
        "priority-high" => Some(icons::PRIORITY_HIGH),
        "ticket-account" => Some(icons::TICKET_ACCOUNT),
        "square-root" => Some(icons::SQUARE_ROOT),
        "toy-brick-search" => Some(icons::TOY_BRICK_SEARCH),
        "star-three-points-outline" => Some(icons::STAR_THREE_POINTS_OUTLINE),
        "coffee-maker-check-outline" => Some(icons::COFFEE_MAKER_CHECK_OUTLINE),
        "bell-ring-outline" => Some(icons::BELL_RING_OUTLINE),
        "invoice-arrow-right" => Some(icons::INVOICE_ARROW_RIGHT),
        "view-carousel" => Some(icons::VIEW_CAROUSEL),
        #[allow(deprecated)]
        "ember" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ember' is deprecated.").print(py);
            }
            Some(icons::EMBER)
        }
        "database-sync" => Some(icons::DATABASE_SYNC),
        "lock-open-remove-outline" => Some(icons::LOCK_OPEN_REMOVE_OUTLINE),
        "microphone-minus" => Some(icons::MICROPHONE_MINUS),
        "content-save-alert-outline" => Some(icons::CONTENT_SAVE_ALERT_OUTLINE),
        "snake" => Some(icons::SNAKE),
        "alien-outline" => Some(icons::ALIEN_OUTLINE),
        "power-socket-eu" => Some(icons::POWER_SOCKET_EU),
        "food-outline" => Some(icons::FOOD_OUTLINE),
        "arrow-top-right-bottom-left" => Some(icons::ARROW_TOP_RIGHT_BOTTOM_LEFT),
        "store-cog-outline" => Some(icons::STORE_COG_OUTLINE),
        "head-cog-outline" => Some(icons::HEAD_COG_OUTLINE),
        "safe-square" => Some(icons::SAFE_SQUARE),
        "bag-carry-on-off" => Some(icons::BAG_CARRY_ON_OFF),
        "calendar-question-outline" => Some(icons::CALENDAR_QUESTION_OUTLINE),
        "flash-off-outline" => Some(icons::FLASH_OFF_OUTLINE),
        "ev-plug-ccs2" => Some(icons::EV_PLUG_CCS2),
        "crosshairs" => Some(icons::CROSSHAIRS),
        "pocket" => Some(icons::POCKET),
        "format-pilcrow-arrow-left" => Some(icons::FORMAT_PILCROW_ARROW_LEFT),
        "water-check-outline" => Some(icons::WATER_CHECK_OUTLINE),
        "battery-charging-60" => Some(icons::BATTERY_CHARGING_60),
        "order-bool-descending" => Some(icons::ORDER_BOOL_DESCENDING),
        "toy-brick" => Some(icons::TOY_BRICK),
        "map-marker-left-outline" => Some(icons::MAP_MARKER_LEFT_OUTLINE),
        "circle-multiple" => Some(icons::CIRCLE_MULTIPLE),
        "numeric-0-box-outline" => Some(icons::NUMERIC_0_BOX_OUTLINE),
        "file-image-remove-outline" => Some(icons::FILE_IMAGE_REMOVE_OUTLINE),
        "arrow-down-right" => Some(icons::ARROW_DOWN_RIGHT),
        "volume-mute" => Some(icons::VOLUME_MUTE),
        "badminton" => Some(icons::BADMINTON),
        "lightbulb-multiple-outline" => Some(icons::LIGHTBULB_MULTIPLE_OUTLINE),
        "music-note-off" => Some(icons::MUSIC_NOTE_OFF),
        "clipboard-check-multiple" => Some(icons::CLIPBOARD_CHECK_MULTIPLE),
        "weather-sunset" => Some(icons::WEATHER_SUNSET),
        "credit-card-fast" => Some(icons::CREDIT_CARD_FAST),
        "navigation-variant-outline" => Some(icons::NAVIGATION_VARIANT_OUTLINE),
        "camera-timer" => Some(icons::CAMERA_TIMER),
        "data-matrix-edit" => Some(icons::DATA_MATRIX_EDIT),
        "album" => Some(icons::ALBUM),
        "train-car" => Some(icons::TRAIN_CAR),
        "database-off" => Some(icons::DATABASE_OFF),
        "beaker-remove" => Some(icons::BEAKER_REMOVE),
        "calendar-filter" => Some(icons::CALENDAR_FILTER),
        "projector-screen-variant-off" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF),
        "octagram-minus" => Some(icons::OCTAGRAM_MINUS),
        "view-comfy-outline" => Some(icons::VIEW_COMFY_OUTLINE),
        "battery-heart" => Some(icons::BATTERY_HEART),
        "controller-classic-outline" => Some(icons::CONTROLLER_CLASSIC_OUTLINE),
        "rhombus-split-outline" => Some(icons::RHOMBUS_SPLIT_OUTLINE),
        "layers-triple" => Some(icons::LAYERS_TRIPLE),
        "file-undo" => Some(icons::FILE_UNDO),
        "wrap-disabled" => Some(icons::WRAP_DISABLED),
        "library" => Some(icons::LIBRARY),
        "deskphone" => Some(icons::DESKPHONE),
        "chart-bubble" => Some(icons::CHART_BUBBLE),
        "head-question-outline" => Some(icons::HEAD_QUESTION_OUTLINE),
        "minus-circle" => Some(icons::MINUS_CIRCLE),
        "sprinkler-fire" => Some(icons::SPRINKLER_FIRE),
        "heart-off" => Some(icons::HEART_OFF),
        "bed-single" => Some(icons::BED_SINGLE),
        "emoticon-cry" => Some(icons::EMOTICON_CRY),
        "bag-suitcase-outline" => Some(icons::BAG_SUITCASE_OUTLINE),
        "content-save-minus-outline" => Some(icons::CONTENT_SAVE_MINUS_OUTLINE),
        #[allow(deprecated)]
        "google-spreadsheet" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-spreadsheet' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_SPREADSHEET)
        }
        "clock-start" => Some(icons::CLOCK_START),
        "folder-star-multiple" => Some(icons::FOLDER_STAR_MULTIPLE),
        "glass-fragile" => Some(icons::GLASS_FRAGILE),
        #[allow(deprecated)]
        "google-plus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-plus' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLUS)
        }
        "pine-tree-variant-outline" => Some(icons::PINE_TREE_VARIANT_OUTLINE),
        "arrow-collapse-down" => Some(icons::ARROW_COLLAPSE_DOWN),
        "clipboard-off" => Some(icons::CLIPBOARD_OFF),
        "piano-off" => Some(icons::PIANO_OFF),
        "email-lock" => Some(icons::EMAIL_LOCK),
        "image-check" => Some(icons::IMAGE_CHECK),
        "clipboard-text-search" => Some(icons::CLIPBOARD_TEXT_SEARCH),
        "lightning-bolt-outline" => Some(icons::LIGHTNING_BOLT_OUTLINE),
        "pier" => Some(icons::PIER),
        "folder-plus" => Some(icons::FOLDER_PLUS),
        "hdr" => Some(icons::HDR),
        "rewind-45" => Some(icons::REWIND_45),
        "bluetooth" => Some(icons::BLUETOOTH),
        "grave-stone" => Some(icons::GRAVE_STONE),
        "zodiac-libra" => Some(icons::ZODIAC_LIBRA),
        "package" => Some(icons::PACKAGE),
        "lightbulb-on-80" => Some(icons::LIGHTBULB_ON_80),
        "weather-rainy" => Some(icons::WEATHER_RAINY),
        "cow-off" => Some(icons::COW_OFF),
        "calendar-week-begin" => Some(icons::CALENDAR_WEEK_BEGIN),
        "image-broken" => Some(icons::IMAGE_BROKEN),
        "voicemail" => Some(icons::VOICEMAIL),
        "battery-90" => Some(icons::BATTERY_90),
        "invoice-text-multiple" => Some(icons::INVOICE_TEXT_MULTIPLE),
        "led-strip" => Some(icons::LED_STRIP),
        "ethernet-cable" => Some(icons::ETHERNET_CABLE),
        "bullhorn-outline" => Some(icons::BULLHORN_OUTLINE),
        "chevron-right-box-outline" => Some(icons::CHEVRON_RIGHT_BOX_OUTLINE),
        "octagram-edit" => Some(icons::OCTAGRAM_EDIT),
        "diamond-stone" => Some(icons::DIAMOND_STONE),
        "tumble-dryer" => Some(icons::TUMBLE_DRYER),
        "book-account-outline" => Some(icons::BOOK_ACCOUNT_OUTLINE),
        "timer-edit" => Some(icons::TIMER_EDIT),
        "home-clock" => Some(icons::HOME_CLOCK),
        "chevron-double-left" => Some(icons::CHEVRON_DOUBLE_LEFT),
        "account-settings-outline" => Some(icons::ACCOUNT_SETTINGS_OUTLINE),
        "cloud-remove-outline" => Some(icons::CLOUD_REMOVE_OUTLINE),
        "archive-minus-outline" => Some(icons::ARCHIVE_MINUS_OUTLINE),
        "timer-star-outline" => Some(icons::TIMER_STAR_OUTLINE),
        "temperature-fahrenheit" => Some(icons::TEMPERATURE_FAHRENHEIT),
        "pail-remove" => Some(icons::PAIL_REMOVE),
        "comment-off-outline" => Some(icons::COMMENT_OFF_OUTLINE),
        "format-font-size-increase" => Some(icons::FORMAT_FONT_SIZE_INCREASE),
        "comment-text-outline" => Some(icons::COMMENT_TEXT_OUTLINE),
        "key-chain" => Some(icons::KEY_CHAIN),
        "post" => Some(icons::POST),
        "decimal-comma-increase" => Some(icons::DECIMAL_COMMA_INCREASE),
        "home-floor-g" => Some(icons::HOME_FLOOR_G),
        "printer-pos-stop-outline" => Some(icons::PRINTER_POS_STOP_OUTLINE),
        "crop-rotate" => Some(icons::CROP_ROTATE),
        "spider-thread" => Some(icons::SPIDER_THREAD),
        "door-closed-cancel" => Some(icons::DOOR_CLOSED_CANCEL),
        "delete-off" => Some(icons::DELETE_OFF),
        "receipt-text-arrow-left" => Some(icons::RECEIPT_TEXT_ARROW_LEFT),
        "keyboard-f2" => Some(icons::KEYBOARD_F2),
        "square" => Some(icons::SQUARE),
        "account-network-off-outline" => Some(icons::ACCOUNT_NETWORK_OFF_OUTLINE),
        "coffee-off" => Some(icons::COFFEE_OFF),
        "home-city" => Some(icons::HOME_CITY),
        "delete" => Some(icons::DELETE),
        "filter-cog-outline" => Some(icons::FILTER_COG_OUTLINE),
        #[allow(deprecated)]
        "microsoft-edge" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-edge' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_EDGE)
        }
        "paper-roll-outline" => Some(icons::PAPER_ROLL_OUTLINE),
        "shovel-off" => Some(icons::SHOVEL_OFF),
        "inbox-arrow-up-outline" => Some(icons::INBOX_ARROW_UP_OUTLINE),
        "domain-plus" => Some(icons::DOMAIN_PLUS),
        "bus-side" => Some(icons::BUS_SIDE),
        "camera-retake" => Some(icons::CAMERA_RETAKE),
        "thermometer-low" => Some(icons::THERMOMETER_LOW),
        "application-import" => Some(icons::APPLICATION_IMPORT),
        "web-box" => Some(icons::WEB_BOX),
        "sack-percent" => Some(icons::SACK_PERCENT),
        "near-me" => Some(icons::NEAR_ME),
        "content-save-edit" => Some(icons::CONTENT_SAVE_EDIT),
        "cloud-refresh" => Some(icons::CLOUD_REFRESH),
        #[allow(deprecated)]
        "nfc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nfc' is deprecated.").print(py);
            }
            Some(icons::NFC)
        }
        _ => None,
    }
}
