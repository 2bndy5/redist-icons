// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_0(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "sticker-text-outline" => Some(icons::STICKER_TEXT_OUTLINE),
        "timer-stop-outline" => Some(icons::TIMER_STOP_OUTLINE),
        "battery-charging-wireless-50" => Some(icons::BATTERY_CHARGING_WIRELESS_50),
        "movie-open-settings" => Some(icons::MOVIE_OPEN_SETTINGS),
        "mortar-pestle" => Some(icons::MORTAR_PESTLE),
        "speedometer-slow" => Some(icons::SPEEDOMETER_SLOW),
        #[allow(deprecated)]
        "language-kotlin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-kotlin' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_KOTLIN)
        }
        "timer-check" => Some(icons::TIMER_CHECK),
        "minidisc" => Some(icons::MINIDISC),
        "calendar-multiselect" => Some(icons::CALENDAR_MULTISELECT),
        "message-cog-outline" => Some(icons::MESSAGE_COG_OUTLINE),
        "storefront-plus-outline" => Some(icons::STOREFRONT_PLUS_OUTLINE),
        "tooltip-account" => Some(icons::TOOLTIP_ACCOUNT),
        "email-remove-outline" => Some(icons::EMAIL_REMOVE_OUTLINE),
        "source-repository" => Some(icons::SOURCE_REPOSITORY),
        "storefront-edit" => Some(icons::STOREFRONT_EDIT),
        "telescope" => Some(icons::TELESCOPE),
        "hand-coin-outline" => Some(icons::HAND_COIN_OUTLINE),
        "account-reactivate" => Some(icons::ACCOUNT_REACTIVATE),
        "folder-text" => Some(icons::FOLDER_TEXT),
        "file-lock-outline" => Some(icons::FILE_LOCK_OUTLINE),
        "card-bulleted-outline" => Some(icons::CARD_BULLETED_OUTLINE),
        "invert-colors-off" => Some(icons::INVERT_COLORS_OFF),
        "math-norm" => Some(icons::MATH_NORM),
        "check-network" => Some(icons::CHECK_NETWORK),
        "anchor" => Some(icons::ANCHOR),
        "alarm-bell" => Some(icons::ALARM_BELL),
        "bed-outline" => Some(icons::BED_OUTLINE),
        "quality-medium" => Some(icons::QUALITY_MEDIUM),
        "timeline-plus-outline" => Some(icons::TIMELINE_PLUS_OUTLINE),
        "format-line-style" => Some(icons::FORMAT_LINE_STYLE),
        "calendar-outline" => Some(icons::CALENDAR_OUTLINE),
        "lan-disconnect" => Some(icons::LAN_DISCONNECT),
        "clipboard-text-off" => Some(icons::CLIPBOARD_TEXT_OFF),
        "bed-clock" => Some(icons::BED_CLOCK),
        "usb-flash-drive" => Some(icons::USB_FLASH_DRIVE),
        "network-strength-1" => Some(icons::NETWORK_STRENGTH_1),
        "api-off" => Some(icons::API_OFF),
        "rabbit" => Some(icons::RABBIT),
        "briefcase-remove" => Some(icons::BRIEFCASE_REMOVE),
        "bed-single" => Some(icons::BED_SINGLE),
        "cash-minus" => Some(icons::CASH_MINUS),
        "alpha-x-box-outline" => Some(icons::ALPHA_X_BOX_OUTLINE),
        "stadium-variant" => Some(icons::STADIUM_VARIANT),
        "comment-search-outline" => Some(icons::COMMENT_SEARCH_OUTLINE),
        "information-off-outline" => Some(icons::INFORMATION_OFF_OUTLINE),
        "headphones-box" => Some(icons::HEADPHONES_BOX),
        "clipboard-play-outline" => Some(icons::CLIPBOARD_PLAY_OUTLINE),
        "razor-single-edge" => Some(icons::RAZOR_SINGLE_EDGE),
        "zodiac-sagittarius" => Some(icons::ZODIAC_SAGITTARIUS),
        "magnify-minus-cursor" => Some(icons::MAGNIFY_MINUS_CURSOR),
        "align-horizontal-center" => Some(icons::ALIGN_HORIZONTAL_CENTER),
        "file-excel-outline" => Some(icons::FILE_EXCEL_OUTLINE),
        #[allow(deprecated)]
        "gatsby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gatsby' is deprecated.").print(py);
            }
            Some(icons::GATSBY)
        }
        "receipt-text-check" => Some(icons::RECEIPT_TEXT_CHECK),
        "laser-pointer" => Some(icons::LASER_POINTER),
        "keyboard-close-outline" => Some(icons::KEYBOARD_CLOSE_OUTLINE),
        "cake" => Some(icons::CAKE),
        "widgets-outline" => Some(icons::WIDGETS_OUTLINE),
        "coffin" => Some(icons::COFFIN),
        "doorbell-video" => Some(icons::DOORBELL_VIDEO),
        "sd" => Some(icons::SD),
        "playlist-play" => Some(icons::PLAYLIST_PLAY),
        "checkbox-marked-outline" => Some(icons::CHECKBOX_MARKED_OUTLINE),
        "arrow-up" => Some(icons::ARROW_UP),
        "leaf-maple" => Some(icons::LEAF_MAPLE),
        "progress-wrench" => Some(icons::PROGRESS_WRENCH),
        "snowflake-off" => Some(icons::SNOWFLAKE_OFF),
        "play-speed" => Some(icons::PLAY_SPEED),
        "sim-alert-outline" => Some(icons::SIM_ALERT_OUTLINE),
        "account-wrench-outline" => Some(icons::ACCOUNT_WRENCH_OUTLINE),
        "sail-boat" => Some(icons::SAIL_BOAT),
        "folder-minus-outline" => Some(icons::FOLDER_MINUS_OUTLINE),
        "snake" => Some(icons::SNAKE),
        "spa" => Some(icons::SPA),
        "clipboard-list-outline" => Some(icons::CLIPBOARD_LIST_OUTLINE),
        "archive-arrow-down" => Some(icons::ARCHIVE_ARROW_DOWN),
        "arrow-down-bold-circle" => Some(icons::ARROW_DOWN_BOLD_CIRCLE),
        "map-marker-minus" => Some(icons::MAP_MARKER_MINUS),
        "network-strength-3-alert" => Some(icons::NETWORK_STRENGTH_3_ALERT),
        "menu-left" => Some(icons::MENU_LEFT),
        "fencing" => Some(icons::FENCING),
        "keyboard-f4" => Some(icons::KEYBOARD_F4),
        #[allow(deprecated)]
        "creative-commons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'creative-commons' is deprecated.")
                    .print(py);
            }
            Some(icons::CREATIVE_COMMONS)
        }
        "bike-pedal" => Some(icons::BIKE_PEDAL),
        "car-seat-heater" => Some(icons::CAR_SEAT_HEATER),
        "content-save-alert" => Some(icons::CONTENT_SAVE_ALERT),
        #[allow(deprecated)]
        "google-glass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-glass' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_GLASS)
        }
        #[allow(deprecated)]
        "hulu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'hulu' is deprecated.").print(py);
            }
            Some(icons::HULU)
        }
        "dice-d4" => Some(icons::DICE_D4),
        "blender-outline" => Some(icons::BLENDER_OUTLINE),
        "magnet-on" => Some(icons::MAGNET_ON),
        "folder-file-outline" => Some(icons::FOLDER_FILE_OUTLINE),
        "cards" => Some(icons::CARDS),
        "jump-rope" => Some(icons::JUMP_ROPE),
        "file-image-remove-outline" => Some(icons::FILE_IMAGE_REMOVE_OUTLINE),
        "movie-minus" => Some(icons::MOVIE_MINUS),
        "diamond-stone" => Some(icons::DIAMOND_STONE),
        "file-document-refresh" => Some(icons::FILE_DOCUMENT_REFRESH),
        "select-group" => Some(icons::SELECT_GROUP),
        #[allow(deprecated)]
        "nix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nix' is deprecated.").print(py);
            }
            Some(icons::NIX)
        }
        "teddy-bear" => Some(icons::TEDDY_BEAR),
        "view-list-outline" => Some(icons::VIEW_LIST_OUTLINE),
        "send-variant-clock" => Some(icons::SEND_VARIANT_CLOCK),
        "kayaking" => Some(icons::KAYAKING),
        "tooltip-image" => Some(icons::TOOLTIP_IMAGE),
        "bottle-soda-outline" => Some(icons::BOTTLE_SODA_OUTLINE),
        "alpha-r-box" => Some(icons::ALPHA_R_BOX),
        "relation-only-one-to-many" => Some(icons::RELATION_ONLY_ONE_TO_MANY),
        "arrow-up-thick" => Some(icons::ARROW_UP_THICK),
        "lightbulb-on-30" => Some(icons::LIGHTBULB_ON_30),
        "clock-minus-outline" => Some(icons::CLOCK_MINUS_OUTLINE),
        "plus-box-outline" => Some(icons::PLUS_BOX_OUTLINE),
        "battery" => Some(icons::BATTERY),
        "octagram-plus-outline" => Some(icons::OCTAGRAM_PLUS_OUTLINE),
        "football-australian" => Some(icons::FOOTBALL_AUSTRALIAN),
        "book-alphabet" => Some(icons::BOOK_ALPHABET),
        "numeric-2-box-outline" => Some(icons::NUMERIC_2_BOX_OUTLINE),
        "faucet-variant" => Some(icons::FAUCET_VARIANT),
        "bookmark-box-multiple-outline" => Some(icons::BOOKMARK_BOX_MULTIPLE_OUTLINE),
        "cloud-circle" => Some(icons::CLOUD_CIRCLE),
        "cellphone-nfc-off" => Some(icons::CELLPHONE_NFC_OFF),
        "set-left-right" => Some(icons::SET_LEFT_RIGHT),
        "bow-arrow" => Some(icons::BOW_ARROW),
        #[allow(deprecated)]
        "reddit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'reddit' is deprecated.").print(py);
            }
            Some(icons::REDDIT)
        }
        "tag-remove" => Some(icons::TAG_REMOVE),
        "clipboard-text-outline" => Some(icons::CLIPBOARD_TEXT_OUTLINE),
        "table-cancel" => Some(icons::TABLE_CANCEL),
        "lightbulb-on-70" => Some(icons::LIGHTBULB_ON_70),
        "emoticon-minus-outline" => Some(icons::EMOTICON_MINUS_OUTLINE),
        "bulkhead-light" => Some(icons::BULKHEAD_LIGHT),
        "peanut-off" => Some(icons::PEANUT_OFF),
        "timeline-alert-outline" => Some(icons::TIMELINE_ALERT_OUTLINE),
        "approximately-equal-box" => Some(icons::APPROXIMATELY_EQUAL_BOX),
        "television-stop" => Some(icons::TELEVISION_STOP),
        "decimal" => Some(icons::DECIMAL),
        "credit-card-minus" => Some(icons::CREDIT_CARD_MINUS),
        "phone" => Some(icons::PHONE),
        "tortoise" => Some(icons::TORTOISE),
        "shape-rectangle-plus" => Some(icons::SHAPE_RECTANGLE_PLUS),
        "arrow-down-bold-outline" => Some(icons::ARROW_DOWN_BOLD_OUTLINE),
        "checkbox-marked-circle-minus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_MINUS_OUTLINE),
        "access-point" => Some(icons::ACCESS_POINT),
        "eye-arrow-left-outline" => Some(icons::EYE_ARROW_LEFT_OUTLINE),
        "trophy-award" => Some(icons::TROPHY_AWARD),
        "tooltip-outline" => Some(icons::TOOLTIP_OUTLINE),
        "printer-pos-sync-outline" => Some(icons::PRINTER_POS_SYNC_OUTLINE),
        "vector-square-open" => Some(icons::VECTOR_SQUARE_OPEN),
        "tune" => Some(icons::TUNE),
        "pan" => Some(icons::PAN),
        #[allow(deprecated)]
        "centos" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'centos' is deprecated.").print(py);
            }
            Some(icons::CENTOS)
        }
        "account-key" => Some(icons::ACCOUNT_KEY),
        "account-lock-open-outline" => Some(icons::ACCOUNT_LOCK_OPEN_OUTLINE),
        "variable-box" => Some(icons::VARIABLE_BOX),
        "wifi-strength-4-lock" => Some(icons::WIFI_STRENGTH_4_LOCK),
        "bus-stop-covered" => Some(icons::BUS_STOP_COVERED),
        "timer-edit-outline" => Some(icons::TIMER_EDIT_OUTLINE),
        "package-down" => Some(icons::PACKAGE_DOWN),
        "puzzle-check-outline" => Some(icons::PUZZLE_CHECK_OUTLINE),
        "home-edit" => Some(icons::HOME_EDIT),
        "book-lock" => Some(icons::BOOK_LOCK),
        "hand-peace" => Some(icons::HAND_PEACE),
        "arrow-up-down-bold" => Some(icons::ARROW_UP_DOWN_BOLD),
        "alpha-p-circle-outline" => Some(icons::ALPHA_P_CIRCLE_OUTLINE),
        "torch" => Some(icons::TORCH),
        "minus-box-multiple" => Some(icons::MINUS_BOX_MULTIPLE),
        "escalator-box" => Some(icons::ESCALATOR_BOX),
        "car-search-outline" => Some(icons::CAR_SEARCH_OUTLINE),
        "head-minus-outline" => Some(icons::HEAD_MINUS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-excel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-excel' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_EXCEL)
        }
        "magnify-expand" => Some(icons::MAGNIFY_EXPAND),
        "weather-lightning" => Some(icons::WEATHER_LIGHTNING),
        "tally-mark-2" => Some(icons::TALLY_MARK_2),
        "sausage-off" => Some(icons::SAUSAGE_OFF),
        "clipboard-arrow-left-outline" => Some(icons::CLIPBOARD_ARROW_LEFT_OUTLINE),
        "paperclip" => Some(icons::PAPERCLIP),
        "truck-off-road-off" => Some(icons::TRUCK_OFF_ROAD_OFF),
        "microwave" => Some(icons::MICROWAVE),
        #[allow(deprecated)]
        "bulma" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bulma' is deprecated.").print(py);
            }
            Some(icons::BULMA)
        }
        "file-remove" => Some(icons::FILE_REMOVE),
        "bell-outline" => Some(icons::BELL_OUTLINE),
        "television-speaker" => Some(icons::TELEVISION_SPEAKER),
        "folder-arrow-left" => Some(icons::FOLDER_ARROW_LEFT),
        "chili-hot-outline" => Some(icons::CHILI_HOT_OUTLINE),
        "dice-d20-outline" => Some(icons::DICE_D20_OUTLINE),
        "shark" => Some(icons::SHARK),
        "koala" => Some(icons::KOALA),
        "battery-arrow-down" => Some(icons::BATTERY_ARROW_DOWN),
        "arrow-collapse-up" => Some(icons::ARROW_COLLAPSE_UP),
        "compass-off-outline" => Some(icons::COMPASS_OFF_OUTLINE),
        "landslide-outline" => Some(icons::LANDSLIDE_OUTLINE),
        "cart-percent" => Some(icons::CART_PERCENT),
        "tooltip-question-outline" => Some(icons::TOOLTIP_QUESTION_OUTLINE),
        "format-text-variant-outline" => Some(icons::FORMAT_TEXT_VARIANT_OUTLINE),
        "comment-eye-outline" => Some(icons::COMMENT_EYE_OUTLINE),
        "credit-card-wireless-outline" => Some(icons::CREDIT_CARD_WIRELESS_OUTLINE),
        "table-remove" => Some(icons::TABLE_REMOVE),
        "format-paint" => Some(icons::FORMAT_PAINT),
        "shield-sword-outline" => Some(icons::SHIELD_SWORD_OUTLINE),
        "progress-upload" => Some(icons::PROGRESS_UPLOAD),
        _ => None,
    }
}
