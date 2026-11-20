// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_19(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "music-circle" => Some(icons::MUSIC_CIRCLE),
        "battery-charging-20" => Some(icons::BATTERY_CHARGING_20),
        "timer-play" => Some(icons::TIMER_PLAY),
        "camera-outline" => Some(icons::CAMERA_OUTLINE),
        "scatter-plot-outline" => Some(icons::SCATTER_PLOT_OUTLINE),
        "certificate" => Some(icons::CERTIFICATE),
        "zodiac-aries" => Some(icons::ZODIAC_ARIES),
        "map-outline" => Some(icons::MAP_OUTLINE),
        "timer-play-outline" => Some(icons::TIMER_PLAY_OUTLINE),
        "home-floor-b" => Some(icons::HOME_FLOOR_B),
        "page-next-outline" => Some(icons::PAGE_NEXT_OUTLINE),
        "book-alert" => Some(icons::BOOK_ALERT),
        "application-settings" => Some(icons::APPLICATION_SETTINGS),
        #[allow(deprecated)]
        "microsoft-excel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-excel' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_EXCEL)
        }
        "lock-check-outline" => Some(icons::LOCK_CHECK_OUTLINE),
        "bookmark-remove" => Some(icons::BOOKMARK_REMOVE),
        "sine-wave" => Some(icons::SINE_WAVE),
        "pine-tree-variant-outline" => Some(icons::PINE_TREE_VARIANT_OUTLINE),
        "horizontal-rotate-clockwise" => Some(icons::HORIZONTAL_ROTATE_CLOCKWISE),
        "human-capacity-decrease" => Some(icons::HUMAN_CAPACITY_DECREASE),
        "snowman" => Some(icons::SNOWMAN),
        "alpha-m-box-outline" => Some(icons::ALPHA_M_BOX_OUTLINE),
        "calendar-start-outline" => Some(icons::CALENDAR_START_OUTLINE),
        "lan-pending" => Some(icons::LAN_PENDING),
        "cloud-outline" => Some(icons::CLOUD_OUTLINE),
        "home-lightbulb-outline" => Some(icons::HOME_LIGHTBULB_OUTLINE),
        "format-quote-close" => Some(icons::FORMAT_QUOTE_CLOSE),
        "trash-can-outline" => Some(icons::TRASH_CAN_OUTLINE),
        "battery-charging-high" => Some(icons::BATTERY_CHARGING_HIGH),
        "cellphone-play" => Some(icons::CELLPHONE_PLAY),
        "cellphone-charging" => Some(icons::CELLPHONE_CHARGING),
        "coach-lamp-variant" => Some(icons::COACH_LAMP_VARIANT),
        "wall-sconce-round" => Some(icons::WALL_SCONCE_ROUND),
        "folder-clock" => Some(icons::FOLDER_CLOCK),
        "betamax" => Some(icons::BETAMAX),
        "tooltip-remove" => Some(icons::TOOLTIP_REMOVE),
        "file-account-outline" => Some(icons::FILE_ACCOUNT_OUTLINE),
        "card-bulleted-off-outline" => Some(icons::CARD_BULLETED_OFF_OUTLINE),
        "bed-king-outline" => Some(icons::BED_KING_OUTLINE),
        "application-export" => Some(icons::APPLICATION_EXPORT),
        "message-text-lock" => Some(icons::MESSAGE_TEXT_LOCK),
        "apple-keyboard-command" => Some(icons::APPLE_KEYBOARD_COMMAND),
        "hamburger" => Some(icons::HAMBURGER),
        "chevron-left-box" => Some(icons::CHEVRON_LEFT_BOX),
        "timeline-text" => Some(icons::TIMELINE_TEXT),
        "alpha-w" => Some(icons::ALPHA_W),
        "camera-marker-outline" => Some(icons::CAMERA_MARKER_OUTLINE),
        "safe-square" => Some(icons::SAFE_SQUARE),
        "note-check" => Some(icons::NOTE_CHECK),
        "baby-bottle" => Some(icons::BABY_BOTTLE),
        "sort-variant" => Some(icons::SORT_VARIANT),
        "folder-star-outline" => Some(icons::FOLDER_STAR_OUTLINE),
        "select-search" => Some(icons::SELECT_SEARCH),
        "kettle-off-outline" => Some(icons::KETTLE_OFF_OUTLINE),
        "information-variant" => Some(icons::INFORMATION_VARIANT),
        "border-radius" => Some(icons::BORDER_RADIUS),
        "inbox-remove-outline" => Some(icons::INBOX_REMOVE_OUTLINE),
        "stretch-to-page-outline" => Some(icons::STRETCH_TO_PAGE_OUTLINE),
        "pan-bottom-right" => Some(icons::PAN_BOTTOM_RIGHT),
        "patio-heater" => Some(icons::PATIO_HEATER),
        "send-circle-outline" => Some(icons::SEND_CIRCLE_OUTLINE),
        "clock-time-two-outline" => Some(icons::CLOCK_TIME_TWO_OUTLINE),
        "rabbit-variant" => Some(icons::RABBIT_VARIANT),
        "image-filter-hdr" => Some(icons::IMAGE_FILTER_HDR),
        "signal-cellular-3" => Some(icons::SIGNAL_CELLULAR_3),
        "human-walker" => Some(icons::HUMAN_WALKER),
        "email-fast-outline" => Some(icons::EMAIL_FAST_OUTLINE),
        #[allow(deprecated)]
        "google" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google' is deprecated.").print(py);
            }
            Some(icons::GOOGLE)
        }
        "message-bookmark-outline" => Some(icons::MESSAGE_BOOKMARK_OUTLINE),
        "home-floor-0" => Some(icons::HOME_FLOOR_0),
        #[allow(deprecated)]
        "quora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'quora' is deprecated.").print(py);
            }
            Some(icons::QUORA)
        }
        "desktop-classic" => Some(icons::DESKTOP_CLASSIC),
        "image-minus-outline" => Some(icons::IMAGE_MINUS_OUTLINE),
        "sitemap-outline" => Some(icons::SITEMAP_OUTLINE),
        "audio-video-off" => Some(icons::AUDIO_VIDEO_OFF),
        "remote-off" => Some(icons::REMOTE_OFF),
        "transit-detour" => Some(icons::TRANSIT_DETOUR),
        "chart-bar-stacked" => Some(icons::CHART_BAR_STACKED),
        "menu-right" => Some(icons::MENU_RIGHT),
        "alpha-g" => Some(icons::ALPHA_G),
        "chat-question-outline" => Some(icons::CHAT_QUESTION_OUTLINE),
        "archive-eye" => Some(icons::ARCHIVE_EYE),
        "phone-off-outline" => Some(icons::PHONE_OFF_OUTLINE),
        "gamepad-circle" => Some(icons::GAMEPAD_CIRCLE),
        "account-wrench-outline" => Some(icons::ACCOUNT_WRENCH_OUTLINE),
        "map-minus" => Some(icons::MAP_MINUS),
        "eject-circle" => Some(icons::EJECT_CIRCLE),
        "map-marker-up" => Some(icons::MAP_MARKER_UP),
        "shape-polygon-plus" => Some(icons::SHAPE_POLYGON_PLUS),
        "pail-plus" => Some(icons::PAIL_PLUS),
        #[allow(deprecated)]
        "xmpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xmpp' is deprecated.").print(py);
            }
            Some(icons::XMPP)
        }
        "beaker" => Some(icons::BEAKER),
        "wrench-check" => Some(icons::WRENCH_CHECK),
        "alpha-c-box-outline" => Some(icons::ALPHA_C_BOX_OUTLINE),
        "calendar-month-outline" => Some(icons::CALENDAR_MONTH_OUTLINE),
        "table-merge-cells" => Some(icons::TABLE_MERGE_CELLS),
        "exclamation-thick" => Some(icons::EXCLAMATION_THICK),
        "arrow-top-left-thick" => Some(icons::ARROW_TOP_LEFT_THICK),
        "email" => Some(icons::EMAIL),
        "movie-star-outline" => Some(icons::MOVIE_STAR_OUTLINE),
        "alpha-d-circle" => Some(icons::ALPHA_D_CIRCLE),
        "trending-neutral" => Some(icons::TRENDING_NEUTRAL),
        "whistle-outline" => Some(icons::WHISTLE_OUTLINE),
        "launch" => Some(icons::LAUNCH),
        "car-speed-limiter" => Some(icons::CAR_SPEED_LIMITER),
        "plus-minus-variant" => Some(icons::PLUS_MINUS_VARIANT),
        "file-powerpoint" => Some(icons::FILE_POWERPOINT),
        "alpha-j-box" => Some(icons::ALPHA_J_BOX),
        "format-wrap-inline" => Some(icons::FORMAT_WRAP_INLINE),
        "wifi-minus" => Some(icons::WIFI_MINUS),
        "bed-outline" => Some(icons::BED_OUTLINE),
        "timeline-outline" => Some(icons::TIMELINE_OUTLINE),
        "cellphone-link-off" => Some(icons::CELLPHONE_LINK_OFF),
        "close-circle-multiple-outline" => Some(icons::CLOSE_CIRCLE_MULTIPLE_OUTLINE),
        "summit" => Some(icons::SUMMIT),
        "roman-numeral-6" => Some(icons::ROMAN_NUMERAL_6),
        "head" => Some(icons::HEAD),
        "headphones-bluetooth" => Some(icons::HEADPHONES_BLUETOOTH),
        #[allow(deprecated)]
        "salesforce" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'salesforce' is deprecated.").print(py);
            }
            Some(icons::SALESFORCE)
        }
        "arrow-up-drop-circle" => Some(icons::ARROW_UP_DROP_CIRCLE),
        "food-drumstick-off" => Some(icons::FOOD_DRUMSTICK_OFF),
        "order-bool-descending-variant" => Some(icons::ORDER_BOOL_DESCENDING_VARIANT),
        #[allow(deprecated)]
        "apple-safari" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-safari' is deprecated.").print(py);
            }
            Some(icons::APPLE_SAFARI)
        }
        "water-pump-off" => Some(icons::WATER_PUMP_OFF),
        "battery-charging-medium" => Some(icons::BATTERY_CHARGING_MEDIUM),
        "function-variant" => Some(icons::FUNCTION_VARIANT),
        "fruit-cherries-off" => Some(icons::FRUIT_CHERRIES_OFF),
        "share-off-outline" => Some(icons::SHARE_OFF_OUTLINE),
        "resistor-nodes" => Some(icons::RESISTOR_NODES),
        "current-ac" => Some(icons::CURRENT_AC),
        "checkbox-multiple-marked-outline" => Some(icons::CHECKBOX_MULTIPLE_MARKED_OUTLINE),
        "network-strength-2-alert" => Some(icons::NETWORK_STRENGTH_2_ALERT),
        "alpha-d-box" => Some(icons::ALPHA_D_BOX),
        "message-badge-outline" => Some(icons::MESSAGE_BADGE_OUTLINE),
        "bell-badge" => Some(icons::BELL_BADGE),
        "fridge-industrial-off" => Some(icons::FRIDGE_INDUSTRIAL_OFF),
        "code-not-equal-variant" => Some(icons::CODE_NOT_EQUAL_VARIANT),
        "cached" => Some(icons::CACHED),
        "source-branch-plus" => Some(icons::SOURCE_BRANCH_PLUS),
        "rivet" => Some(icons::RIVET),
        "filter-remove-outline" => Some(icons::FILTER_REMOVE_OUTLINE),
        "all-inclusive-box-outline" => Some(icons::ALL_INCLUSIVE_BOX_OUTLINE),
        "home-plus-outline" => Some(icons::HOME_PLUS_OUTLINE),
        "light-flood-down" => Some(icons::LIGHT_FLOOD_DOWN),
        "calendar-badge-outline" => Some(icons::CALENDAR_BADGE_OUTLINE),
        "receipt-clock" => Some(icons::RECEIPT_CLOCK),
        "card-multiple-outline" => Some(icons::CARD_MULTIPLE_OUTLINE),
        "bolt" => Some(icons::BOLT),
        "dresser-outline" => Some(icons::DRESSER_OUTLINE),
        "ph" => Some(icons::PH),
        "water-thermometer" => Some(icons::WATER_THERMOMETER),
        "flag-variant-off-outline" => Some(icons::FLAG_VARIANT_OFF_OUTLINE),
        "train-car-container" => Some(icons::TRAIN_CAR_CONTAINER),
        "numeric-9-plus-circle-outline" => Some(icons::NUMERIC_9_PLUS_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "lastpass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'lastpass' is deprecated.").print(py);
            }
            Some(icons::LASTPASS)
        }
        "head-alert" => Some(icons::HEAD_ALERT),
        "scanner-off" => Some(icons::SCANNER_OFF),
        "fireplace" => Some(icons::FIREPLACE),
        "cupboard-outline" => Some(icons::CUPBOARD_OUTLINE),
        "radiator-disabled" => Some(icons::RADIATOR_DISABLED),
        "account-box-edit-outline" => Some(icons::ACCOUNT_BOX_EDIT_OUTLINE),
        "card-account-details-star-outline" => Some(icons::CARD_ACCOUNT_DETAILS_STAR_OUTLINE),
        "phone-outline" => Some(icons::PHONE_OUTLINE),
        "tag-off-outline" => Some(icons::TAG_OFF_OUTLINE),
        "account-network-off" => Some(icons::ACCOUNT_NETWORK_OFF),
        "router-network-wireless" => Some(icons::ROUTER_NETWORK_WIRELESS),
        "drag-vertical-variant" => Some(icons::DRAG_VERTICAL_VARIANT),
        "crop-rotate" => Some(icons::CROP_ROTATE),
        "tram-side" => Some(icons::TRAM_SIDE),
        "home-switch" => Some(icons::HOME_SWITCH),
        "information-variant-circle-outline" => Some(icons::INFORMATION_VARIANT_CIRCLE_OUTLINE),
        "panorama-vertical-outline" => Some(icons::PANORAMA_VERTICAL_OUTLINE),
        "alarm-light" => Some(icons::ALARM_LIGHT),
        "balcony" => Some(icons::BALCONY),
        "jellyfish-outline" => Some(icons::JELLYFISH_OUTLINE),
        "diving-snorkel" => Some(icons::DIVING_SNORKEL),
        "clipboard-account" => Some(icons::CLIPBOARD_ACCOUNT),
        "clock-time-four-outline" => Some(icons::CLOCK_TIME_FOUR_OUTLINE),
        "decimal-increase" => Some(icons::DECIMAL_INCREASE),
        "ultra-high-definition" => Some(icons::ULTRA_HIGH_DEFINITION),
        "clipboard-alert" => Some(icons::CLIPBOARD_ALERT),
        "shield-account" => Some(icons::SHIELD_ACCOUNT),
        "arrow-u-left-bottom-bold" => Some(icons::ARROW_U_LEFT_BOTTOM_BOLD),
        "tune-vertical-variant" => Some(icons::TUNE_VERTICAL_VARIANT),
        "printer-3d-nozzle" => Some(icons::PRINTER_3D_NOZZLE),
        "bathtub-outline" => Some(icons::BATHTUB_OUTLINE),
        "lamp" => Some(icons::LAMP),
        "drag-vertical" => Some(icons::DRAG_VERTICAL),
        "cloud-upload" => Some(icons::CLOUD_UPLOAD),
        "information-box-outline" => Some(icons::INFORMATION_BOX_OUTLINE),
        "scent" => Some(icons::SCENT),
        "network-strength-1" => Some(icons::NETWORK_STRENGTH_1),
        "ghost" => Some(icons::GHOST),
        "string-lights" => Some(icons::STRING_LIGHTS),
        "sofa-single-outline" => Some(icons::SOFA_SINGLE_OUTLINE),
        "lightbulb-alert" => Some(icons::LIGHTBULB_ALERT),
        "label-off" => Some(icons::LABEL_OFF),
        "button-cursor" => Some(icons::BUTTON_CURSOR),
        "rice" => Some(icons::RICE),
        "phone-lock" => Some(icons::PHONE_LOCK),
        _ => None,
    }
}
