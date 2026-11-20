// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_23(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "spear" => Some(icons::SPEAR),
        "football" => Some(icons::FOOTBALL),
        "folder-information-outline" => Some(icons::FOLDER_INFORMATION_OUTLINE),
        #[allow(deprecated)]
        "language-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-go' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_GO)
        }
        "lock-percent-outline" => Some(icons::LOCK_PERCENT_OUTLINE),
        "cookie-refresh-outline" => Some(icons::COOKIE_REFRESH_OUTLINE),
        "moon-full" => Some(icons::MOON_FULL),
        "pan-up" => Some(icons::PAN_UP),
        #[allow(deprecated)]
        "microsoft-visual-studio-code" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-visual-studio-code' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_VISUAL_STUDIO_CODE)
        }
        "briefcase-arrow-up-down" => Some(icons::BRIEFCASE_ARROW_UP_DOWN),
        "alpha-r-box" => Some(icons::ALPHA_R_BOX),
        "link-variant-plus" => Some(icons::LINK_VARIANT_PLUS),
        "relation-only-one-to-many" => Some(icons::RELATION_ONLY_ONE_TO_MANY),
        "chat-alert" => Some(icons::CHAT_ALERT),
        "attachment-off" => Some(icons::ATTACHMENT_OFF),
        "store-off" => Some(icons::STORE_OFF),
        "waves-arrow-left" => Some(icons::WAVES_ARROW_LEFT),
        "ruler-square-compass" => Some(icons::RULER_SQUARE_COMPASS),
        "diving" => Some(icons::DIVING),
        "calendar-plus" => Some(icons::CALENDAR_PLUS),
        "home-group-minus" => Some(icons::HOME_GROUP_MINUS),
        "ghost-outline" => Some(icons::GHOST_OUTLINE),
        "access-point-network-off" => Some(icons::ACCESS_POINT_NETWORK_OFF),
        "diameter" => Some(icons::DIAMETER),
        "diving-scuba-flag" => Some(icons::DIVING_SCUBA_FLAG),
        "window-shutter" => Some(icons::WINDOW_SHUTTER),
        "arrow-top-right-thin-circle-outline" => Some(icons::ARROW_TOP_RIGHT_THIN_CIRCLE_OUTLINE),
        "credit-card-marker" => Some(icons::CREDIT_CARD_MARKER),
        "magazine-rifle" => Some(icons::MAGAZINE_RIFLE),
        "newspaper-variant-multiple" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE),
        "weather-sunset-down" => Some(icons::WEATHER_SUNSET_DOWN),
        "credit-card-search" => Some(icons::CREDIT_CARD_SEARCH),
        #[allow(deprecated)]
        "vlc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vlc' is deprecated.").print(py);
            }
            Some(icons::VLC)
        }
        "cellphone-link" => Some(icons::CELLPHONE_LINK),
        "magnet" => Some(icons::MAGNET),
        "pan" => Some(icons::PAN),
        "power-socket-jp" => Some(icons::POWER_SOCKET_JP),
        "receipt-text-plus-outline" => Some(icons::RECEIPT_TEXT_PLUS_OUTLINE),
        "arm-flex" => Some(icons::ARM_FLEX),
        "snowboard" => Some(icons::SNOWBOARD),
        "music-rest-eighth" => Some(icons::MUSIC_REST_EIGHTH),
        "overscan" => Some(icons::OVERSCAN),
        "bowl-mix-outline" => Some(icons::BOWL_MIX_OUTLINE),
        "sticker-remove" => Some(icons::STICKER_REMOVE),
        "arrow-collapse-left" => Some(icons::ARROW_COLLAPSE_LEFT),
        "lan-pending" => Some(icons::LAN_PENDING),
        "filmstrip-box-multiple" => Some(icons::FILMSTRIP_BOX_MULTIPLE),
        "alpha-d-box" => Some(icons::ALPHA_D_BOX),
        "calendar-arrow-left" => Some(icons::CALENDAR_ARROW_LEFT),
        "safe-square-outline" => Some(icons::SAFE_SQUARE_OUTLINE),
        "flask-remove" => Some(icons::FLASK_REMOVE),
        "arrow-bottom-right-thick" => Some(icons::ARROW_BOTTOM_RIGHT_THICK),
        "numeric-positive-1" => Some(icons::NUMERIC_POSITIVE_1),
        "folder-arrow-right-outline" => Some(icons::FOLDER_ARROW_RIGHT_OUTLINE),
        "cloud-check-outline" => Some(icons::CLOUD_CHECK_OUTLINE),
        "battery-20" => Some(icons::BATTERY_20),
        "bag-suitcase-outline" => Some(icons::BAG_SUITCASE_OUTLINE),
        "snowflake-check" => Some(icons::SNOWFLAKE_CHECK),
        "calendar-search" => Some(icons::CALENDAR_SEARCH),
        "alpha" => Some(icons::ALPHA),
        "square-small" => Some(icons::SQUARE_SMALL),
        "button-cursor" => Some(icons::BUTTON_CURSOR),
        "skull-outline" => Some(icons::SKULL_OUTLINE),
        "office-building" => Some(icons::OFFICE_BUILDING),
        "food-hot-dog" => Some(icons::FOOD_HOT_DOG),
        "fire" => Some(icons::FIRE),
        "trophy-variant" => Some(icons::TROPHY_VARIANT),
        "timeline-text" => Some(icons::TIMELINE_TEXT),
        "cash-clock" => Some(icons::CASH_CLOCK),
        "text-account" => Some(icons::TEXT_ACCOUNT),
        "folder-question" => Some(icons::FOLDER_QUESTION),
        "folder-table" => Some(icons::FOLDER_TABLE),
        "book-open-page-variant" => Some(icons::BOOK_OPEN_PAGE_VARIANT),
        "current-dc" => Some(icons::CURRENT_DC),
        "music-box" => Some(icons::MUSIC_BOX),
        "seed-outline" => Some(icons::SEED_OUTLINE),
        "home-clock-outline" => Some(icons::HOME_CLOCK_OUTLINE),
        "arrow-up-right" => Some(icons::ARROW_UP_RIGHT),
        "flag-plus" => Some(icons::FLAG_PLUS),
        "chevron-left-circle-outline" => Some(icons::CHEVRON_LEFT_CIRCLE_OUTLINE),
        "toy-brick-marker" => Some(icons::TOY_BRICK_MARKER),
        "store-settings" => Some(icons::STORE_SETTINGS),
        "hamburger-plus" => Some(icons::HAMBURGER_PLUS),
        "scatter-plot-outline" => Some(icons::SCATTER_PLOT_OUTLINE),
        "content-duplicate" => Some(icons::CONTENT_DUPLICATE),
        "backspace-outline" => Some(icons::BACKSPACE_OUTLINE),
        "chevron-right-circle-outline" => Some(icons::CHEVRON_RIGHT_CIRCLE_OUTLINE),
        "city-variant-outline" => Some(icons::CITY_VARIANT_OUTLINE),
        #[allow(deprecated)]
        "google-street-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-street-view' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_STREET_VIEW)
        }
        "map-marker-minus-outline" => Some(icons::MAP_MARKER_MINUS_OUTLINE),
        "timer-marker-outline" => Some(icons::TIMER_MARKER_OUTLINE),
        "cloud-arrow-left-outline" => Some(icons::CLOUD_ARROW_LEFT_OUTLINE),
        "escalator" => Some(icons::ESCALATOR),
        "folder-lock-open-outline" => Some(icons::FOLDER_LOCK_OPEN_OUTLINE),
        "boom-gate-arrow-down-outline" => Some(icons::BOOM_GATE_ARROW_DOWN_OUTLINE),
        "copyleft" => Some(icons::COPYLEFT),
        #[allow(deprecated)]
        "language-kotlin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-kotlin' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_KOTLIN)
        }
        "camera-gopro" => Some(icons::CAMERA_GOPRO),
        "emoticon-dead" => Some(icons::EMOTICON_DEAD),
        "key-outline" => Some(icons::KEY_OUTLINE),
        "source-branch-plus" => Some(icons::SOURCE_BRANCH_PLUS),
        "numeric-8-box-outline" => Some(icons::NUMERIC_8_BOX_OUTLINE),
        "file-table-box-multiple" => Some(icons::FILE_TABLE_BOX_MULTIPLE),
        "music-note-sixteenth" => Some(icons::MUSIC_NOTE_SIXTEENTH),
        "dog-side-off" => Some(icons::DOG_SIDE_OFF),
        "swap-horizontal-circle-outline" => Some(icons::SWAP_HORIZONTAL_CIRCLE_OUTLINE),
        "basket-minus" => Some(icons::BASKET_MINUS),
        "filter-remove" => Some(icons::FILTER_REMOVE),
        "golf" => Some(icons::GOLF),
        "silverware-variant" => Some(icons::SILVERWARE_VARIANT),
        "axis-x-y-arrow-lock" => Some(icons::AXIS_X_Y_ARROW_LOCK),
        "spellcheck" => Some(icons::SPELLCHECK),
        "floor-lamp-dual-outline" => Some(icons::FLOOR_LAMP_DUAL_OUTLINE),
        "sort-bool-descending-variant" => Some(icons::SORT_BOOL_DESCENDING_VARIANT),
        "mace" => Some(icons::MACE),
        "roman-numeral-7" => Some(icons::ROMAN_NUMERAL_7),
        "book-arrow-left-outline" => Some(icons::BOOK_ARROW_LEFT_OUTLINE),
        "axis-z-rotate-counterclockwise" => Some(icons::AXIS_Z_ROTATE_COUNTERCLOCKWISE),
        "book-cross" => Some(icons::BOOK_CROSS),
        "ballot-outline" => Some(icons::BALLOT_OUTLINE),
        "coach-lamp-variant" => Some(icons::COACH_LAMP_VARIANT),
        "pipe" => Some(icons::PIPE),
        "seesaw" => Some(icons::SEESAW),
        "network-strength-2" => Some(icons::NETWORK_STRENGTH_2),
        #[allow(deprecated)]
        "language-cpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-cpp' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CPP)
        }
        "rotate-left-variant" => Some(icons::ROTATE_LEFT_VARIANT),
        "radioactive-off" => Some(icons::RADIOACTIVE_OFF),
        "clock-time-three-outline" => Some(icons::CLOCK_TIME_THREE_OUTLINE),
        "calendar-multiple-check" => Some(icons::CALENDAR_MULTIPLE_CHECK),
        "format-header-increase" => Some(icons::FORMAT_HEADER_INCREASE),
        "head-cog" => Some(icons::HEAD_COG),
        "email-lock" => Some(icons::EMAIL_LOCK),
        "cloud-sync-outline" => Some(icons::CLOUD_SYNC_OUTLINE),
        "router-network-wireless" => Some(icons::ROUTER_NETWORK_WIRELESS),
        "credit-card-marker-outline" => Some(icons::CREDIT_CARD_MARKER_OUTLINE),
        "battery-10-bluetooth" => Some(icons::BATTERY_10_BLUETOOTH),
        "subway-alert-variant" => Some(icons::SUBWAY_ALERT_VARIANT),
        "file-key" => Some(icons::FILE_KEY),
        "numeric-5" => Some(icons::NUMERIC_5),
        "cigar" => Some(icons::CIGAR),
        "repeat-off" => Some(icons::REPEAT_OFF),
        "alpha-w-box" => Some(icons::ALPHA_W_BOX),
        "snowflake-off" => Some(icons::SNOWFLAKE_OFF),
        "wrap" => Some(icons::WRAP),
        "printer-pos-stop" => Some(icons::PRINTER_POS_STOP),
        "cards-playing-spade" => Some(icons::CARDS_PLAYING_SPADE),
        "clipboard-text-search" => Some(icons::CLIPBOARD_TEXT_SEARCH),
        "cloud-percent-outline" => Some(icons::CLOUD_PERCENT_OUTLINE),
        "numeric-5-circle" => Some(icons::NUMERIC_5_CIRCLE),
        "alphabetical-variant" => Some(icons::ALPHABETICAL_VARIANT),
        "sort-alphabetical-ascending" => Some(icons::SORT_ALPHABETICAL_ASCENDING),
        "brightness-1" => Some(icons::BRIGHTNESS_1),
        "shield-airplane" => Some(icons::SHIELD_AIRPLANE),
        "baseball-bat" => Some(icons::BASEBALL_BAT),
        "kayaking" => Some(icons::KAYAKING),
        "alpha-p-circle" => Some(icons::ALPHA_P_CIRCLE),
        "plus-network-outline" => Some(icons::PLUS_NETWORK_OUTLINE),
        "alphabetical-variant-off" => Some(icons::ALPHABETICAL_VARIANT_OFF),
        "numeric-9-plus-box-outline" => Some(icons::NUMERIC_9_PLUS_BOX_OUTLINE),
        "circle-medium" => Some(icons::CIRCLE_MEDIUM),
        #[allow(deprecated)]
        "google-classroom" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-classroom' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CLASSROOM)
        }
        "umbrella-closed-outline" => Some(icons::UMBRELLA_CLOSED_OUTLINE),
        "hospital" => Some(icons::HOSPITAL),
        #[allow(deprecated)]
        "microsoft-excel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-excel' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_EXCEL)
        }
        "message-star" => Some(icons::MESSAGE_STAR),
        "email-alert" => Some(icons::EMAIL_ALERT),
        "notebook-heart" => Some(icons::NOTEBOOK_HEART),
        "network-strength-3" => Some(icons::NETWORK_STRENGTH_3),
        "format-overline" => Some(icons::FORMAT_OVERLINE),
        "bike-pedal" => Some(icons::BIKE_PEDAL),
        "folder-upload-outline" => Some(icons::FOLDER_UPLOAD_OUTLINE),
        "wifi-arrow-left-right" => Some(icons::WIFI_ARROW_LEFT_RIGHT),
        "sun-wireless" => Some(icons::SUN_WIRELESS),
        "layers-outline" => Some(icons::LAYERS_OUTLINE),
        "arrow-u-right-bottom" => Some(icons::ARROW_U_RIGHT_BOTTOM),
        "link-box-variant-outline" => Some(icons::LINK_BOX_VARIANT_OUTLINE),
        "alpha-e-circle-outline" => Some(icons::ALPHA_E_CIRCLE_OUTLINE),
        "home-plus-outline" => Some(icons::HOME_PLUS_OUTLINE),
        "underwear-outline" => Some(icons::UNDERWEAR_OUTLINE),
        "washing-machine-off" => Some(icons::WASHING_MACHINE_OFF),
        "garage-alert" => Some(icons::GARAGE_ALERT),
        "currency-mnt" => Some(icons::CURRENCY_MNT),
        "dishwasher-off" => Some(icons::DISHWASHER_OFF),
        "braille" => Some(icons::BRAILLE),
        "file-percent" => Some(icons::FILE_PERCENT),
        "chevron-up-box-outline" => Some(icons::CHEVRON_UP_BOX_OUTLINE),
        "camera-metering-center" => Some(icons::CAMERA_METERING_CENTER),
        "phone" => Some(icons::PHONE),
        "border-horizontal" => Some(icons::BORDER_HORIZONTAL),
        "human-male-height" => Some(icons::HUMAN_MALE_HEIGHT),
        "comma-box-outline" => Some(icons::COMMA_BOX_OUTLINE),
        "lightbulb-cfl-spiral-off" => Some(icons::LIGHTBULB_CFL_SPIRAL_OFF),
        "heart-circle" => Some(icons::HEART_CIRCLE),
        "wifi-off" => Some(icons::WIFI_OFF),
        "battery-charging-wireless" => Some(icons::BATTERY_CHARGING_WIRELESS),
        "grid-large" => Some(icons::GRID_LARGE),
        "lock-off-outline" => Some(icons::LOCK_OFF_OUTLINE),
        "table-search" => Some(icons::TABLE_SEARCH),
        "apple-keyboard-shift" => Some(icons::APPLE_KEYBOARD_SHIFT),
        "emoticon-confused" => Some(icons::EMOTICON_CONFUSED),
        _ => None,
    }
}
