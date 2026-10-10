// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_7(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "airplane-check" => Some(icons::AIRPLANE_CHECK),
        "format-list-bulleted-square" => Some(icons::FORMAT_LIST_BULLETED_SQUARE),
        "podium-gold" => Some(icons::PODIUM_GOLD),
        "clock-time-twelve" => Some(icons::CLOCK_TIME_TWELVE),
        "source-branch-plus" => Some(icons::SOURCE_BRANCH_PLUS),
        "cards-heart" => Some(icons::CARDS_HEART),
        #[allow(deprecated)]
        "home-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'home-assistant' is deprecated.").print(py);
            }
            Some(icons::HOME_ASSISTANT)
        }
        "spear" => Some(icons::SPEAR),
        "phone" => Some(icons::PHONE),
        "wrap-disabled" => Some(icons::WRAP_DISABLED),
        "newspaper-check" => Some(icons::NEWSPAPER_CHECK),
        "monitor-account" => Some(icons::MONITOR_ACCOUNT),
        "wrench-check-outline" => Some(icons::WRENCH_CHECK_OUTLINE),
        "lasso" => Some(icons::LASSO),
        "light-flood-down" => Some(icons::LIGHT_FLOOD_DOWN),
        "comment-check" => Some(icons::COMMENT_CHECK),
        "briefcase-clock-outline" => Some(icons::BRIEFCASE_CLOCK_OUTLINE),
        "text-box-minus" => Some(icons::TEXT_BOX_MINUS),
        "numeric-9-plus-box" => Some(icons::NUMERIC_9_PLUS_BOX),
        "network-strength-outline" => Some(icons::NETWORK_STRENGTH_OUTLINE),
        "folder-arrow-up-down-outline" => Some(icons::FOLDER_ARROW_UP_DOWN_OUTLINE),
        "gauge" => Some(icons::GAUGE),
        "tooltip-check-outline" => Some(icons::TOOLTIP_CHECK_OUTLINE),
        "size-l" => Some(icons::SIZE_L),
        "food-drumstick-off" => Some(icons::FOOD_DRUMSTICK_OFF),
        "math-compass" => Some(icons::MATH_COMPASS),
        "axis-z-arrow" => Some(icons::AXIS_Z_ARROW),
        "wifi-minus" => Some(icons::WIFI_MINUS),
        "arrow-u-up-right" => Some(icons::ARROW_U_UP_RIGHT),
        "keyboard-f2" => Some(icons::KEYBOARD_F2),
        "voicemail" => Some(icons::VOICEMAIL),
        "star-settings" => Some(icons::STAR_SETTINGS),
        "food" => Some(icons::FOOD),
        "folder-lock" => Some(icons::FOLDER_LOCK),
        "robot-happy" => Some(icons::ROBOT_HAPPY),
        "clock-start" => Some(icons::CLOCK_START),
        "bookshelf" => Some(icons::BOOKSHELF),
        "standard-definition" => Some(icons::STANDARD_DEFINITION),
        "head-cog" => Some(icons::HEAD_COG),
        "vpn" => Some(icons::VPN),
        "amplifier" => Some(icons::AMPLIFIER),
        "spoon-sugar" => Some(icons::SPOON_SUGAR),
        "package-variant-closed-check" => Some(icons::PACKAGE_VARIANT_CLOSED_CHECK),
        "inbox-remove-outline" => Some(icons::INBOX_REMOVE_OUTLINE),
        "inbox-arrow-down-outline" => Some(icons::INBOX_ARROW_DOWN_OUTLINE),
        "magnify-minus-cursor" => Some(icons::MAGNIFY_MINUS_CURSOR),
        "battery-charging-wireless" => Some(icons::BATTERY_CHARGING_WIRELESS),
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "face-mask" => Some(icons::FACE_MASK),
        "account-injury" => Some(icons::ACCOUNT_INJURY),
        "view-list" => Some(icons::VIEW_LIST),
        "keg" => Some(icons::KEG),
        "book-minus" => Some(icons::BOOK_MINUS),
        "lock-check-outline" => Some(icons::LOCK_CHECK_OUTLINE),
        "account-search-outline" => Some(icons::ACCOUNT_SEARCH_OUTLINE),
        "train-car-gondola" => Some(icons::TRAIN_CAR_GONDOLA),
        "timer-lock" => Some(icons::TIMER_LOCK),
        "account-settings-outline" => Some(icons::ACCOUNT_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-charging" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-charging' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_CHARGING)
        }
        "chat-minus" => Some(icons::CHAT_MINUS),
        "application-outline" => Some(icons::APPLICATION_OUTLINE),
        "camcorder-off" => Some(icons::CAMCORDER_OFF),
        "file-chart-check" => Some(icons::FILE_CHART_CHECK),
        "wizard-hat" => Some(icons::WIZARD_HAT),
        "chevron-left-box-outline" => Some(icons::CHEVRON_LEFT_BOX_OUTLINE),
        "airplane-edit" => Some(icons::AIRPLANE_EDIT),
        "graph" => Some(icons::GRAPH),
        "home-plus-outline" => Some(icons::HOME_PLUS_OUTLINE),
        "home-alert" => Some(icons::HOME_ALERT),
        "tooltip-plus-outline" => Some(icons::TOOLTIP_PLUS_OUTLINE),
        "seal-variant" => Some(icons::SEAL_VARIANT),
        "bag-personal-off" => Some(icons::BAG_PERSONAL_OFF),
        "television-off" => Some(icons::TELEVISION_OFF),
        #[allow(deprecated)]
        "nintendo-wiiu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wiiu' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WIIU)
        }
        "function" => Some(icons::FUNCTION),
        "note-plus" => Some(icons::NOTE_PLUS),
        "alpha-z-box-outline" => Some(icons::ALPHA_Z_BOX_OUTLINE),
        "television-guide" => Some(icons::TELEVISION_GUIDE),
        "file-excel-outline" => Some(icons::FILE_EXCEL_OUTLINE),
        "flash-triangle" => Some(icons::FLASH_TRIANGLE),
        "truck-minus-outline" => Some(icons::TRUCK_MINUS_OUTLINE),
        "chat-processing" => Some(icons::CHAT_PROCESSING),
        "cards-playing-diamond-multiple-outline" => {
            Some(icons::CARDS_PLAYING_DIAMOND_MULTIPLE_OUTLINE)
        }
        "home-remove" => Some(icons::HOME_REMOVE),
        "toolbox-outline" => Some(icons::TOOLBOX_OUTLINE),
        "timer-music-outline" => Some(icons::TIMER_MUSIC_OUTLINE),
        "beta" => Some(icons::BETA),
        "table-split-cell" => Some(icons::TABLE_SPLIT_CELL),
        "file-move" => Some(icons::FILE_MOVE),
        "wind-power-outline" => Some(icons::WIND_POWER_OUTLINE),
        "sort-clock-descending" => Some(icons::SORT_CLOCK_DESCENDING),
        "swap-horizontal-circle" => Some(icons::SWAP_HORIZONTAL_CIRCLE),
        "car-clutch" => Some(icons::CAR_CLUTCH),
        "globe-light-outline" => Some(icons::GLOBE_LIGHT_OUTLINE),
        "pencil-ruler-outline" => Some(icons::PENCIL_RULER_OUTLINE),
        "invoice-minus-outline" => Some(icons::INVOICE_MINUS_OUTLINE),
        "zodiac-taurus" => Some(icons::ZODIAC_TAURUS),
        "boxing-glove" => Some(icons::BOXING_GLOVE),
        "location-enter" => Some(icons::LOCATION_ENTER),
        "invoice-text-fast-outline" => Some(icons::INVOICE_TEXT_FAST_OUTLINE),
        "atm" => Some(icons::ATM),
        "solar-power-variant" => Some(icons::SOLAR_POWER_VARIANT),
        "account-filter" => Some(icons::ACCOUNT_FILTER),
        "vector-square-close" => Some(icons::VECTOR_SQUARE_CLOSE),
        "emoticon-angry" => Some(icons::EMOTICON_ANGRY),
        "floor-lamp-torchiere-variant-outline" => Some(icons::FLOOR_LAMP_TORCHIERE_VARIANT_OUTLINE),
        "clipboard-check" => Some(icons::CLIPBOARD_CHECK),
        #[allow(deprecated)]
        "onepassword" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'onepassword' is deprecated.").print(py);
            }
            Some(icons::ONEPASSWORD)
        }
        "alpha-d" => Some(icons::ALPHA_D),
        "printer-pos-star" => Some(icons::PRINTER_POS_STAR),
        "car-brake-temperature" => Some(icons::CAR_BRAKE_TEMPERATURE),
        "ethernet" => Some(icons::ETHERNET),
        "water-off" => Some(icons::WATER_OFF),
        "bluetooth-settings" => Some(icons::BLUETOOTH_SETTINGS),
        "eye-check-outline" => Some(icons::EYE_CHECK_OUTLINE),
        "weather-lightning" => Some(icons::WEATHER_LIGHTNING),
        "power-standby" => Some(icons::POWER_STANDBY),
        "folder-star" => Some(icons::FOLDER_STAR),
        "checkbox-multiple-outline" => Some(icons::CHECKBOX_MULTIPLE_OUTLINE),
        "shield-cross" => Some(icons::SHIELD_CROSS),
        "fuse" => Some(icons::FUSE),
        "drag-horizontal-variant" => Some(icons::DRAG_HORIZONTAL_VARIANT),
        "numeric-3" => Some(icons::NUMERIC_3),
        "clock-digital" => Some(icons::CLOCK_DIGITAL),
        "beaker-outline" => Some(icons::BEAKER_OUTLINE),
        "folder-alert-outline" => Some(icons::FOLDER_ALERT_OUTLINE),
        "cast-connected" => Some(icons::CAST_CONNECTED),
        "power-socket-jp" => Some(icons::POWER_SOCKET_JP),
        "file-find-outline" => Some(icons::FILE_FIND_OUTLINE),
        "calendar-week" => Some(icons::CALENDAR_WEEK),
        "upload" => Some(icons::UPLOAD),
        "arrow-right-bold" => Some(icons::ARROW_RIGHT_BOLD),
        "arrow-bottom-right-bold-box-outline" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX_OUTLINE),
        "water-boiler-auto" => Some(icons::WATER_BOILER_AUTO),
        "file-account-outline" => Some(icons::FILE_ACCOUNT_OUTLINE),
        "bank-minus" => Some(icons::BANK_MINUS),
        "recycle-variant" => Some(icons::RECYCLE_VARIANT),
        #[allow(deprecated)]
        "microsoft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT)
        }
        "tally-mark-1" => Some(icons::TALLY_MARK_1),
        "triangle-small-down" => Some(icons::TRIANGLE_SMALL_DOWN),
        "screen-rotation-lock" => Some(icons::SCREEN_ROTATION_LOCK),
        "comment-check-outline" => Some(icons::COMMENT_CHECK_OUTLINE),
        "comma-circle-outline" => Some(icons::COMMA_CIRCLE_OUTLINE),
        "order-bool-descending" => Some(icons::ORDER_BOOL_DESCENDING),
        "swap-horizontal-bold" => Some(icons::SWAP_HORIZONTAL_BOLD),
        "bell-off-outline" => Some(icons::BELL_OFF_OUTLINE),
        "hand-front-left-outline" => Some(icons::HAND_FRONT_LEFT_OUTLINE),
        "roller-shade-closed" => Some(icons::ROLLER_SHADE_CLOSED),
        #[allow(deprecated)]
        "apple-safari" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-safari' is deprecated.").print(py);
            }
            Some(icons::APPLE_SAFARI)
        }
        #[allow(deprecated)]
        "laravel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'laravel' is deprecated.").print(py);
            }
            Some(icons::LARAVEL)
        }
        "light-switch" => Some(icons::LIGHT_SWITCH),
        "lock-alert-outline" => Some(icons::LOCK_ALERT_OUTLINE),
        "chart-box-multiple" => Some(icons::CHART_BOX_MULTIPLE),
        "view-dashboard-variant" => Some(icons::VIEW_DASHBOARD_VARIANT),
        "robot-vacuum-alert" => Some(icons::ROBOT_VACUUM_ALERT),
        "eye-lock-outline" => Some(icons::EYE_LOCK_OUTLINE),
        "tag-arrow-down-outline" => Some(icons::TAG_ARROW_DOWN_OUTLINE),
        "peanut-off" => Some(icons::PEANUT_OFF),
        "arrow-u-right-top-bold" => Some(icons::ARROW_U_RIGHT_TOP_BOLD),
        "heart-multiple" => Some(icons::HEART_MULTIPLE),
        "tag-off" => Some(icons::TAG_OFF),
        "human-male-boy" => Some(icons::HUMAN_MALE_BOY),
        "account-music-outline" => Some(icons::ACCOUNT_MUSIC_OUTLINE),
        "coffee-maker-check-outline" => Some(icons::COFFEE_MAKER_CHECK_OUTLINE),
        "cradle-outline" => Some(icons::CRADLE_OUTLINE),
        "dice-5" => Some(icons::DICE_5),
        "folder-file-outline" => Some(icons::FOLDER_FILE_OUTLINE),
        "alarm-note" => Some(icons::ALARM_NOTE),
        "fencing" => Some(icons::FENCING),
        "folder-network" => Some(icons::FOLDER_NETWORK),
        "arrow-top-right-thin-circle-outline" => Some(icons::ARROW_TOP_RIGHT_THIN_CIRCLE_OUTLINE),
        "folder-pound" => Some(icons::FOLDER_POUND),
        "sort-clock-ascending-outline" => Some(icons::SORT_CLOCK_ASCENDING_OUTLINE),
        "account-arrow-left" => Some(icons::ACCOUNT_ARROW_LEFT),
        "shield-car" => Some(icons::SHIELD_CAR),
        "hexagon-multiple-outline" => Some(icons::HEXAGON_MULTIPLE_OUTLINE),
        "clock-out" => Some(icons::CLOCK_OUT),
        "train-car-container" => Some(icons::TRAIN_CAR_CONTAINER),
        "view-grid-outline" => Some(icons::VIEW_GRID_OUTLINE),
        "square-edit-outline" => Some(icons::SQUARE_EDIT_OUTLINE),
        "cctv" => Some(icons::CCTV),
        "arrow-down-bold-circle" => Some(icons::ARROW_DOWN_BOLD_CIRCLE),
        "bed-clock" => Some(icons::BED_CLOCK),
        "space-invaders" => Some(icons::SPACE_INVADERS),
        "set-center" => Some(icons::SET_CENTER),
        "timer-cancel-outline" => Some(icons::TIMER_CANCEL_OUTLINE),
        "briefcase-account" => Some(icons::BRIEFCASE_ACCOUNT),
        "arrow-top-left-thick" => Some(icons::ARROW_TOP_LEFT_THICK),
        "orbit" => Some(icons::ORBIT),
        "wifi-strength-alert-outline" => Some(icons::WIFI_STRENGTH_ALERT_OUTLINE),
        "border-inside" => Some(icons::BORDER_INSIDE),
        "ticket" => Some(icons::TICKET),
        "robot-dead-outline" => Some(icons::ROBOT_DEAD_OUTLINE),
        #[allow(deprecated)]
        "terraform" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'terraform' is deprecated.").print(py);
            }
            Some(icons::TERRAFORM)
        }
        "transmission-tower-import" => Some(icons::TRANSMISSION_TOWER_IMPORT),
        "clock-time-nine" => Some(icons::CLOCK_TIME_NINE),
        "tooltip-minus" => Some(icons::TOOLTIP_MINUS),
        "image-move" => Some(icons::IMAGE_MOVE),
        "presentation-play" => Some(icons::PRESENTATION_PLAY),
        "swap-horizontal" => Some(icons::SWAP_HORIZONTAL),
        _ => None,
    }
}
