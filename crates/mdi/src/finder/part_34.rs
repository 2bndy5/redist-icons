// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_34(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "database-off-outline" => Some(icons::DATABASE_OFF_OUTLINE),
        "chart-arc" => Some(icons::CHART_ARC),
        "lightbulb-question" => Some(icons::LIGHTBULB_QUESTION),
        "car-defrost-front" => Some(icons::CAR_DEFROST_FRONT),
        "weather-fog" => Some(icons::WEATHER_FOG),
        "contrast" => Some(icons::CONTRAST),
        "book-arrow-left" => Some(icons::BOOK_ARROW_LEFT),
        "server-remove" => Some(icons::SERVER_REMOVE),
        "layers-off-outline" => Some(icons::LAYERS_OFF_OUTLINE),
        "lead-pencil" => Some(icons::LEAD_PENCIL),
        "coffee-off-outline" => Some(icons::COFFEE_OFF_OUTLINE),
        "monitor-dashboard" => Some(icons::MONITOR_DASHBOARD),
        #[allow(deprecated)]
        "plex" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'plex' is deprecated.").print(py);
            }
            Some(icons::PLEX)
        }
        "fish" => Some(icons::FISH),
        "clock-alert-outline" => Some(icons::CLOCK_ALERT_OUTLINE),
        "human-male-female-child" => Some(icons::HUMAN_MALE_FEMALE_CHILD),
        "radiator" => Some(icons::RADIATOR),
        "format-underline-wavy" => Some(icons::FORMAT_UNDERLINE_WAVY),
        "cash-register" => Some(icons::CASH_REGISTER),
        "dots-hexagon" => Some(icons::DOTS_HEXAGON),
        "numeric-positive-1" => Some(icons::NUMERIC_POSITIVE_1),
        #[allow(deprecated)]
        "blender-software" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'blender-software' is deprecated.")
                    .print(py);
            }
            Some(icons::BLENDER_SOFTWARE)
        }
        "protocol" => Some(icons::PROTOCOL),
        "cancel" => Some(icons::CANCEL),
        "filter-menu-outline" => Some(icons::FILTER_MENU_OUTLINE),
        "gas-station-off" => Some(icons::GAS_STATION_OFF),
        "robot-off" => Some(icons::ROBOT_OFF),
        "wifi-strength-1-lock" => Some(icons::WIFI_STRENGTH_1_LOCK),
        "ph" => Some(icons::PH),
        "fraction-one-half" => Some(icons::FRACTION_ONE_HALF),
        "table-cancel" => Some(icons::TABLE_CANCEL),
        "diversify" => Some(icons::DIVERSIFY),
        "lock-open-plus-outline" => Some(icons::LOCK_OPEN_PLUS_OUTLINE),
        "view-sequential" => Some(icons::VIEW_SEQUENTIAL),
        "currency-cny" => Some(icons::CURRENCY_CNY),
        "octagram-plus-outline" => Some(icons::OCTAGRAM_PLUS_OUTLINE),
        "vector-combine" => Some(icons::VECTOR_COMBINE),
        "phone-hangup-outline" => Some(icons::PHONE_HANGUP_OUTLINE),
        "message-video" => Some(icons::MESSAGE_VIDEO),
        "timer-cog-outline" => Some(icons::TIMER_COG_OUTLINE),
        "camera-front-variant" => Some(icons::CAMERA_FRONT_VARIANT),
        "motion-sensor-off" => Some(icons::MOTION_SENSOR_OFF),
        "format-vertical-align-bottom" => Some(icons::FORMAT_VERTICAL_ALIGN_BOTTOM),
        "mortar-pestle" => Some(icons::MORTAR_PESTLE),
        "source-pull" => Some(icons::SOURCE_PULL),
        "currency-php" => Some(icons::CURRENCY_PHP),
        "reply-outline" => Some(icons::REPLY_OUTLINE),
        "folder-settings" => Some(icons::FOLDER_SETTINGS),
        "select-multiple-marker" => Some(icons::SELECT_MULTIPLE_MARKER),
        "home-lock" => Some(icons::HOME_LOCK),
        "boom-gate" => Some(icons::BOOM_GATE),
        "lock-remove-outline" => Some(icons::LOCK_REMOVE_OUTLINE),
        "motorbike-off" => Some(icons::MOTORBIKE_OFF),
        "oil-level" => Some(icons::OIL_LEVEL),
        "truck-outline" => Some(icons::TRUCK_OUTLINE),
        "rolodex-outline" => Some(icons::ROLODEX_OUTLINE),
        "fan-remove" => Some(icons::FAN_REMOVE),
        "grid-large" => Some(icons::GRID_LARGE),
        "image-minus" => Some(icons::IMAGE_MINUS),
        "battery-charging-wireless-20" => Some(icons::BATTERY_CHARGING_WIRELESS_20),
        "account-multiple-outline" => Some(icons::ACCOUNT_MULTIPLE_OUTLINE),
        "monitor-arrow-down-variant" => Some(icons::MONITOR_ARROW_DOWN_VARIANT),
        "routes" => Some(icons::ROUTES),
        "redo" => Some(icons::REDO),
        "folder-account-outline" => Some(icons::FOLDER_ACCOUNT_OUTLINE),
        "account-hard-hat" => Some(icons::ACCOUNT_HARD_HAT),
        "file-marker" => Some(icons::FILE_MARKER),
        "book-alert" => Some(icons::BOOK_ALERT),
        "bookmark-minus" => Some(icons::BOOKMARK_MINUS),
        "church" => Some(icons::CHURCH),
        "pan-bottom-left" => Some(icons::PAN_BOTTOM_LEFT),
        "alpha-p" => Some(icons::ALPHA_P),
        "file-edit" => Some(icons::FILE_EDIT),
        "atm" => Some(icons::ATM),
        "folder-move-outline" => Some(icons::FOLDER_MOVE_OUTLINE),
        "relative-scale" => Some(icons::RELATIVE_SCALE),
        "numeric-1" => Some(icons::NUMERIC_1),
        "arrow-top-left-thin-circle-outline" => Some(icons::ARROW_TOP_LEFT_THIN_CIRCLE_OUTLINE),
        "airballoon" => Some(icons::AIRBALLOON),
        #[allow(deprecated)]
        "disqus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'disqus' is deprecated.").print(py);
            }
            Some(icons::DISQUS)
        }
        "projector-screen-off-outline" => Some(icons::PROJECTOR_SCREEN_OFF_OUTLINE),
        "wifi-star" => Some(icons::WIFI_STAR),
        "printer-pos-off" => Some(icons::PRINTER_POS_OFF),
        "battery-charging-wireless-alert" => Some(icons::BATTERY_CHARGING_WIRELESS_ALERT),
        "attachment-remove" => Some(icons::ATTACHMENT_REMOVE),
        "human-white-cane" => Some(icons::HUMAN_WHITE_CANE),
        "rewind-5" => Some(icons::REWIND_5),
        "vector-polyline-plus" => Some(icons::VECTOR_POLYLINE_PLUS),
        "toy-brick-remove-outline" => Some(icons::TOY_BRICK_REMOVE_OUTLINE),
        "truck-minus-outline" => Some(icons::TRUCK_MINUS_OUTLINE),
        "diving" => Some(icons::DIVING),
        "battery-charging-high" => Some(icons::BATTERY_CHARGING_HIGH),
        "lotion" => Some(icons::LOTION),
        "human-non-binary" => Some(icons::HUMAN_NON_BINARY),
        "qrcode-edit" => Some(icons::QRCODE_EDIT),
        "file-sync-outline" => Some(icons::FILE_SYNC_OUTLINE),
        "bike-pedal" => Some(icons::BIKE_PEDAL),
        "clock-time-five" => Some(icons::CLOCK_TIME_FIVE),
        "chevron-up-circle-outline" => Some(icons::CHEVRON_UP_CIRCLE_OUTLINE),
        "database-alert" => Some(icons::DATABASE_ALERT),
        "handshake" => Some(icons::HANDSHAKE),
        "gas-station-in-use" => Some(icons::GAS_STATION_IN_USE),
        "needle-off" => Some(icons::NEEDLE_OFF),
        "dresser-outline" => Some(icons::DRESSER_OUTLINE),
        "folder-lock" => Some(icons::FOLDER_LOCK),
        "relation-one-or-many-to-zero-or-many" => Some(icons::RELATION_ONE_OR_MANY_TO_ZERO_OR_MANY),
        "attachment" => Some(icons::ATTACHMENT),
        "spider-outline" => Some(icons::SPIDER_OUTLINE),
        "mouse-left-click" => Some(icons::MOUSE_LEFT_CLICK),
        "email-fast" => Some(icons::EMAIL_FAST),
        "skip-previous" => Some(icons::SKIP_PREVIOUS),
        "code-parentheses" => Some(icons::CODE_PARENTHESES),
        "cellphone-nfc" => Some(icons::CELLPHONE_NFC),
        "animation-play" => Some(icons::ANIMATION_PLAY),
        "file-table" => Some(icons::FILE_TABLE),
        "numeric-8-circle-outline" => Some(icons::NUMERIC_8_CIRCLE_OUTLINE),
        "gate-not" => Some(icons::GATE_NOT),
        "fast-forward-outline" => Some(icons::FAST_FORWARD_OUTLINE),
        "scale-balance" => Some(icons::SCALE_BALANCE),
        "cash-100" => Some(icons::CASH_100),
        "horse-human" => Some(icons::HORSE_HUMAN),
        "ampersand" => Some(icons::AMPERSAND),
        "variable-box" => Some(icons::VARIABLE_BOX),
        "cart-arrow-up" => Some(icons::CART_ARROW_UP),
        "translate-off" => Some(icons::TRANSLATE_OFF),
        "waterfall" => Some(icons::WATERFALL),
        "square-circle-outline" => Some(icons::SQUARE_CIRCLE_OUTLINE),
        "phone-return" => Some(icons::PHONE_RETURN),
        "folder-arrow-left-outline" => Some(icons::FOLDER_ARROW_LEFT_OUTLINE),
        "export" => Some(icons::EXPORT),
        "gamepad-square-outline" => Some(icons::GAMEPAD_SQUARE_OUTLINE),
        "cookie-minus" => Some(icons::COOKIE_MINUS),
        "land-plots" => Some(icons::LAND_PLOTS),
        "town-hall" => Some(icons::TOWN_HALL),
        "pencil-circle" => Some(icons::PENCIL_CIRCLE),
        "percent-box" => Some(icons::PERCENT_BOX),
        "basketball-hoop" => Some(icons::BASKETBALL_HOOP),
        #[allow(deprecated)]
        "microsoft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT)
        }
        "home-edit" => Some(icons::HOME_EDIT),
        "movie-open-play-outline" => Some(icons::MOVIE_OPEN_PLAY_OUTLINE),
        "archive-arrow-down-outline" => Some(icons::ARCHIVE_ARROW_DOWN_OUTLINE),
        "shield-bug-outline" => Some(icons::SHIELD_BUG_OUTLINE),
        "radioactive" => Some(icons::RADIOACTIVE),
        "rabbit-variant" => Some(icons::RABBIT_VARIANT),
        "cookie-minus-outline" => Some(icons::COOKIE_MINUS_OUTLINE),
        "numeric-10-circle-outline" => Some(icons::NUMERIC_10_CIRCLE_OUTLINE),
        "printer-pos-edit-outline" => Some(icons::PRINTER_POS_EDIT_OUTLINE),
        "cloud-question-outline" => Some(icons::CLOUD_QUESTION_OUTLINE),
        "dishwasher" => Some(icons::DISHWASHER),
        "shield-check-outline" => Some(icons::SHIELD_CHECK_OUTLINE),
        "cloud-refresh-variant-outline" => Some(icons::CLOUD_REFRESH_VARIANT_OUTLINE),
        "home-floor-3" => Some(icons::HOME_FLOOR_3),
        "tooltip" => Some(icons::TOOLTIP),
        "timer-10" => Some(icons::TIMER_10),
        "file-clock" => Some(icons::FILE_CLOCK),
        "cloud-download-outline" => Some(icons::CLOUD_DOWNLOAD_OUTLINE),
        "cog-sync-outline" => Some(icons::COG_SYNC_OUTLINE),
        "skip-previous-outline" => Some(icons::SKIP_PREVIOUS_OUTLINE),
        "format-header-3" => Some(icons::FORMAT_HEADER_3),
        "sort-ascending" => Some(icons::SORT_ASCENDING),
        "elevator-passenger-off" => Some(icons::ELEVATOR_PASSENGER_OFF),
        "chart-areaspline-variant" => Some(icons::CHART_AREASPLINE_VARIANT),
        "volleyball" => Some(icons::VOLLEYBALL),
        "backspace-outline" => Some(icons::BACKSPACE_OUTLINE),
        "arrow-u-right-top-bold" => Some(icons::ARROW_U_RIGHT_TOP_BOLD),
        "book-check" => Some(icons::BOOK_CHECK),
        "format-rotate-90" => Some(icons::FORMAT_ROTATE_90),
        "format-horizontal-align-right" => Some(icons::FORMAT_HORIZONTAL_ALIGN_RIGHT),
        "wind-power-outline" => Some(icons::WIND_POWER_OUTLINE),
        #[allow(deprecated)]
        "language-xaml" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-xaml' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_XAML)
        }
        "mouse-move-up" => Some(icons::MOUSE_MOVE_UP),
        "home-remove-outline" => Some(icons::HOME_REMOVE_OUTLINE),
        "archive-outline" => Some(icons::ARCHIVE_OUTLINE),
        "nutrition" => Some(icons::NUTRITION),
        "table-edit" => Some(icons::TABLE_EDIT),
        "dice-3-outline" => Some(icons::DICE_3_OUTLINE),
        "roman-numeral-8" => Some(icons::ROMAN_NUMERAL_8),
        "step-backward" => Some(icons::STEP_BACKWARD),
        "clipboard-alert" => Some(icons::CLIPBOARD_ALERT),
        "record-circle" => Some(icons::RECORD_CIRCLE),
        "select" => Some(icons::SELECT),
        "tooltip-remove-outline" => Some(icons::TOOLTIP_REMOVE_OUTLINE),
        "card-multiple-outline" => Some(icons::CARD_MULTIPLE_OUTLINE),
        "view-column-outline" => Some(icons::VIEW_COLUMN_OUTLINE),
        "roller-skate-off" => Some(icons::ROLLER_SKATE_OFF),
        "currency-ngn" => Some(icons::CURRENCY_NGN),
        "numeric-6-box-outline" => Some(icons::NUMERIC_6_BOX_OUTLINE),
        "dots-horizontal-circle-outline" => Some(icons::DOTS_HORIZONTAL_CIRCLE_OUTLINE),
        "sail-boat" => Some(icons::SAIL_BOAT),
        "music-rest-whole" => Some(icons::MUSIC_REST_WHOLE),
        "kettle-alert-outline" => Some(icons::KETTLE_ALERT_OUTLINE),
        "castle" => Some(icons::CASTLE),
        "help" => Some(icons::HELP),
        "pause-circle-outline" => Some(icons::PAUSE_CIRCLE_OUTLINE),
        "skip-forward-outline" => Some(icons::SKIP_FORWARD_OUTLINE),
        "sofa-single" => Some(icons::SOFA_SINGLE),
        #[allow(deprecated)]
        "microsoft-outlook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-outlook' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OUTLOOK)
        }
        "dice-5" => Some(icons::DICE_5),
        "database-clock" => Some(icons::DATABASE_CLOCK),
        "subtitles" => Some(icons::SUBTITLES),
        _ => None,
    }
}
