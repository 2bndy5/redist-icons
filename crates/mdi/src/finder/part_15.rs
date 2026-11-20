// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_15(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "chart-bubble" => Some(icons::CHART_BUBBLE),
        #[allow(deprecated)]
        "npm" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'npm' is deprecated.").print(py);
            }
            Some(icons::NPM)
        }
        "lightbulb-off-outline" => Some(icons::LIGHTBULB_OFF_OUTLINE),
        "truck-check" => Some(icons::TRUCK_CHECK),
        "gauge-empty" => Some(icons::GAUGE_EMPTY),
        "format-horizontal-align-right" => Some(icons::FORMAT_HORIZONTAL_ALIGN_RIGHT),
        "stretch-to-page" => Some(icons::STRETCH_TO_PAGE),
        "wifi-strength-3-lock" => Some(icons::WIFI_STRENGTH_3_LOCK),
        "tooltip" => Some(icons::TOOLTIP),
        "swap-vertical" => Some(icons::SWAP_VERTICAL),
        "coffee-to-go-outline" => Some(icons::COFFEE_TO_GO_OUTLINE),
        "login-variant" => Some(icons::LOGIN_VARIANT),
        "ticket-confirmation" => Some(icons::TICKET_CONFIRMATION),
        "cellphone-dock" => Some(icons::CELLPHONE_DOCK),
        "shield-remove-outline" => Some(icons::SHIELD_REMOVE_OUTLINE),
        "seatbelt" => Some(icons::SEATBELT),
        "island-variant" => Some(icons::ISLAND_VARIANT),
        "pulse" => Some(icons::PULSE),
        "chef-hat" => Some(icons::CHEF_HAT),
        "city-variant" => Some(icons::CITY_VARIANT),
        "controller-classic" => Some(icons::CONTROLLER_CLASSIC),
        "credit-card-lock-outline" => Some(icons::CREDIT_CARD_LOCK_OUTLINE),
        "arrow-top-right" => Some(icons::ARROW_TOP_RIGHT),
        "folder-clock" => Some(icons::FOLDER_CLOCK),
        "file-lock-outline" => Some(icons::FILE_LOCK_OUTLINE),
        "weather-windy" => Some(icons::WEATHER_WINDY),
        "snowflake-variant" => Some(icons::SNOWFLAKE_VARIANT),
        "responsive" => Some(icons::RESPONSIVE),
        "doctor" => Some(icons::DOCTOR),
        "movie-open-cog" => Some(icons::MOVIE_OPEN_COG),
        "attachment-plus" => Some(icons::ATTACHMENT_PLUS),
        "bell-remove-outline" => Some(icons::BELL_REMOVE_OUTLINE),
        "lightbulb-variant-outline" => Some(icons::LIGHTBULB_VARIANT_OUTLINE),
        "movie-off-outline" => Some(icons::MOVIE_OFF_OUTLINE),
        "message-arrow-left" => Some(icons::MESSAGE_ARROW_LEFT),
        "city" => Some(icons::CITY),
        "text-recognition" => Some(icons::TEXT_RECOGNITION),
        "note-text" => Some(icons::NOTE_TEXT),
        "calendar-refresh-outline" => Some(icons::CALENDAR_REFRESH_OUTLINE),
        "roman-numeral-6" => Some(icons::ROMAN_NUMERAL_6),
        "robot-vacuum-variant-off" => Some(icons::ROBOT_VACUUM_VARIANT_OFF),
        "arrow-left-circle" => Some(icons::ARROW_LEFT_CIRCLE),
        "book-remove-multiple-outline" => Some(icons::BOOK_REMOVE_MULTIPLE_OUTLINE),
        "rectangle" => Some(icons::RECTANGLE),
        "calendar-badge-outline" => Some(icons::CALENDAR_BADGE_OUTLINE),
        "axis-arrow" => Some(icons::AXIS_ARROW),
        "gas-station-off" => Some(icons::GAS_STATION_OFF),
        "truck-delivery" => Some(icons::TRUCK_DELIVERY),
        "division-box" => Some(icons::DIVISION_BOX),
        "zodiac-virgo" => Some(icons::ZODIAC_VIRGO),
        "van-utility" => Some(icons::VAN_UTILITY),
        "beaker-check" => Some(icons::BEAKER_CHECK),
        "source-commit-local" => Some(icons::SOURCE_COMMIT_LOCAL),
        "pot-mix-outline" => Some(icons::POT_MIX_OUTLINE),
        "star-check-outline" => Some(icons::STAR_CHECK_OUTLINE),
        "email-off-outline" => Some(icons::EMAIL_OFF_OUTLINE),
        "printer-pos-cancel-outline" => Some(icons::PRINTER_POS_CANCEL_OUTLINE),
        "note-check" => Some(icons::NOTE_CHECK),
        "cards-playing-spade-multiple" => Some(icons::CARDS_PLAYING_SPADE_MULTIPLE),
        "credit-card-check-outline" => Some(icons::CREDIT_CARD_CHECK_OUTLINE),
        "movie-remove-outline" => Some(icons::MOVIE_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "docker" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'docker' is deprecated.").print(py);
            }
            Some(icons::DOCKER)
        }
        "axis-arrow-info" => Some(icons::AXIS_ARROW_INFO),
        "lightbulb-group-off" => Some(icons::LIGHTBULB_GROUP_OFF),
        "leak" => Some(icons::LEAK),
        #[allow(deprecated)]
        "youtube-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-studio' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_STUDIO)
        }
        "clock-outline" => Some(icons::CLOCK_OUTLINE),
        "paperclip-minus" => Some(icons::PAPERCLIP_MINUS),
        "timeline-plus-outline" => Some(icons::TIMELINE_PLUS_OUTLINE),
        "file" => Some(icons::FILE),
        "call-received" => Some(icons::CALL_RECEIVED),
        "car-battery" => Some(icons::CAR_BATTERY),
        "account-cash-outline" => Some(icons::ACCOUNT_CASH_OUTLINE),
        "window-restore" => Some(icons::WINDOW_RESTORE),
        "account-sync" => Some(icons::ACCOUNT_SYNC),
        "flash-off-outline" => Some(icons::FLASH_OFF_OUTLINE),
        "bottle-tonic" => Some(icons::BOTTLE_TONIC),
        "flip-horizontal" => Some(icons::FLIP_HORIZONTAL),
        "alpha-g" => Some(icons::ALPHA_G),
        "filter-settings" => Some(icons::FILTER_SETTINGS),
        "information-slab-circle" => Some(icons::INFORMATION_SLAB_CIRCLE),
        "pan-vertical" => Some(icons::PAN_VERTICAL),
        "rename" => Some(icons::RENAME),
        "clock-check" => Some(icons::CLOCK_CHECK),
        "power-plug-off" => Some(icons::POWER_PLUG_OFF),
        "shield-refresh-outline" => Some(icons::SHIELD_REFRESH_OUTLINE),
        "invoice-text-clock-outline" => Some(icons::INVOICE_TEXT_CLOCK_OUTLINE),
        "database-sync" => Some(icons::DATABASE_SYNC),
        "credit-card-edit-outline" => Some(icons::CREDIT_CARD_EDIT_OUTLINE),
        #[allow(deprecated)]
        "google-cardboard" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cardboard' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CARDBOARD)
        }
        "archive-check-outline" => Some(icons::ARCHIVE_CHECK_OUTLINE),
        "cookie-check-outline" => Some(icons::COOKIE_CHECK_OUTLINE),
        "order-bool-descending" => Some(icons::ORDER_BOOL_DESCENDING),
        "sticker-outline" => Some(icons::STICKER_OUTLINE),
        "seed-off-outline" => Some(icons::SEED_OFF_OUTLINE),
        "human-female-female" => Some(icons::HUMAN_FEMALE_FEMALE),
        "incognito-circle-off" => Some(icons::INCOGNITO_CIRCLE_OFF),
        "plus-lock" => Some(icons::PLUS_LOCK),
        "zip-box-outline" => Some(icons::ZIP_BOX_OUTLINE),
        "credit-card-minus-outline" => Some(icons::CREDIT_CARD_MINUS_OUTLINE),
        "diversify" => Some(icons::DIVERSIFY),
        "printer" => Some(icons::PRINTER),
        "flag-off-outline" => Some(icons::FLAG_OFF_OUTLINE),
        "store-minus-outline" => Some(icons::STORE_MINUS_OUTLINE),
        "headset" => Some(icons::HEADSET),
        #[allow(deprecated)]
        "google-my-business" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-my-business' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_MY_BUSINESS)
        }
        "hard-hat" => Some(icons::HARD_HAT),
        "card-off-outline" => Some(icons::CARD_OFF_OUTLINE),
        "corn" => Some(icons::CORN),
        "movie-search" => Some(icons::MOVIE_SEARCH),
        "weather-partly-cloudy" => Some(icons::WEATHER_PARTLY_CLOUDY),
        "square-medium" => Some(icons::SQUARE_MEDIUM),
        "phone-minus" => Some(icons::PHONE_MINUS),
        "alpha-o-circle" => Some(icons::ALPHA_O_CIRCLE),
        "forum-minus" => Some(icons::FORUM_MINUS),
        "cassette" => Some(icons::CASSETTE),
        "star-david" => Some(icons::STAR_DAVID),
        "tray-remove" => Some(icons::TRAY_REMOVE),
        "wifi-strength-1" => Some(icons::WIFI_STRENGTH_1),
        #[allow(deprecated)]
        "plex" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'plex' is deprecated.").print(py);
            }
            Some(icons::PLEX)
        }
        "car-windshield" => Some(icons::CAR_WINDSHIELD),
        "music-circle" => Some(icons::MUSIC_CIRCLE),
        "store-remove-outline" => Some(icons::STORE_REMOVE_OUTLINE),
        "file-image-remove-outline" => Some(icons::FILE_IMAGE_REMOVE_OUTLINE),
        "kite" => Some(icons::KITE),
        "format-line-style" => Some(icons::FORMAT_LINE_STYLE),
        "robot-mower" => Some(icons::ROBOT_MOWER),
        "boom-gate-arrow-down" => Some(icons::BOOM_GATE_ARROW_DOWN),
        "square-medium-outline" => Some(icons::SQUARE_MEDIUM_OUTLINE),
        "radiobox-blank" => Some(icons::RADIOBOX_BLANK),
        "chart-timeline-variant-shimmer" => Some(icons::CHART_TIMELINE_VARIANT_SHIMMER),
        "numeric-9-circle" => Some(icons::NUMERIC_9_CIRCLE),
        "text-box-search-outline" => Some(icons::TEXT_BOX_SEARCH_OUTLINE),
        "om" => Some(icons::OM),
        "marker-check" => Some(icons::MARKER_CHECK),
        "image-move" => Some(icons::IMAGE_MOVE),
        "owl" => Some(icons::OWL),
        "format-annotation-plus" => Some(icons::FORMAT_ANNOTATION_PLUS),
        "camera-control" => Some(icons::CAMERA_CONTROL),
        "battery-alert" => Some(icons::BATTERY_ALERT),
        "phone-cancel-outline" => Some(icons::PHONE_CANCEL_OUTLINE),
        "cheese-off" => Some(icons::CHEESE_OFF),
        "checkbox-blank-circle" => Some(icons::CHECKBOX_BLANK_CIRCLE),
        "laptop-account" => Some(icons::LAPTOP_ACCOUNT),
        "phone-ring-outline" => Some(icons::PHONE_RING_OUTLINE),
        "book-search-outline" => Some(icons::BOOK_SEARCH_OUTLINE),
        "invoice-text-minus" => Some(icons::INVOICE_TEXT_MINUS),
        "bowl" => Some(icons::BOWL),
        "currency-try" => Some(icons::CURRENCY_TRY),
        "lock-check-outline" => Some(icons::LOCK_CHECK_OUTLINE),
        "checkbox-multiple-blank" => Some(icons::CHECKBOX_MULTIPLE_BLANK),
        "cloud-download" => Some(icons::CLOUD_DOWNLOAD),
        "credit-card-wireless-outline" => Some(icons::CREDIT_CARD_WIRELESS_OUTLINE),
        "iv-bag" => Some(icons::IV_BAG),
        "chat-alert-outline" => Some(icons::CHAT_ALERT_OUTLINE),
        "beach" => Some(icons::BEACH),
        "alpha-c-circle" => Some(icons::ALPHA_C_CIRCLE),
        "lock-outline" => Some(icons::LOCK_OUTLINE),
        "table-picnic" => Some(icons::TABLE_PICNIC),
        "emoticon-poop" => Some(icons::EMOTICON_POOP),
        "menu-left" => Some(icons::MENU_LEFT),
        "comment-minus-outline" => Some(icons::COMMENT_MINUS_OUTLINE),
        "chevron-triple-left" => Some(icons::CHEVRON_TRIPLE_LEFT),
        "dolly" => Some(icons::DOLLY),
        "pier-crane" => Some(icons::PIER_CRANE),
        "file-arrow-up-down-outline" => Some(icons::FILE_ARROW_UP_DOWN_OUTLINE),
        "octagon-outline" => Some(icons::OCTAGON_OUTLINE),
        "network-off" => Some(icons::NETWORK_OFF),
        "signature-freehand" => Some(icons::SIGNATURE_FREEHAND),
        "exponent-box" => Some(icons::EXPONENT_BOX),
        "account-circle-outline" => Some(icons::ACCOUNT_CIRCLE_OUTLINE),
        "diameter-outline" => Some(icons::DIAMETER_OUTLINE),
        #[allow(deprecated)]
        "zigbee" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'zigbee' is deprecated.").print(py);
            }
            Some(icons::ZIGBEE)
        }
        "tray-alert" => Some(icons::TRAY_ALERT),
        "coffee-to-go" => Some(icons::COFFEE_TO_GO),
        "crystal-ball" => Some(icons::CRYSTAL_BALL),
        "thermometer-low" => Some(icons::THERMOMETER_LOW),
        "led-variant-outline" => Some(icons::LED_VARIANT_OUTLINE),
        "barn" => Some(icons::BARN),
        "box-shadow" => Some(icons::BOX_SHADOW),
        "consolidate" => Some(icons::CONSOLIDATE),
        "bank-off-outline" => Some(icons::BANK_OFF_OUTLINE),
        "tumble-dryer-off" => Some(icons::TUMBLE_DRYER_OFF),
        "run-fast" => Some(icons::RUN_FAST),
        "hexagram-outline" => Some(icons::HEXAGRAM_OUTLINE),
        "fruit-cherries" => Some(icons::FRUIT_CHERRIES),
        "polo" => Some(icons::POLO),
        "camera-metering-matrix" => Some(icons::CAMERA_METERING_MATRIX),
        "lava-lamp" => Some(icons::LAVA_LAMP),
        "music-note-plus" => Some(icons::MUSIC_NOTE_PLUS),
        "shark-off" => Some(icons::SHARK_OFF),
        "send-lock-outline" => Some(icons::SEND_LOCK_OUTLINE),
        "debug-step-out" => Some(icons::DEBUG_STEP_OUT),
        "circle" => Some(icons::CIRCLE),
        "gesture-tap-box" => Some(icons::GESTURE_TAP_BOX),
        "home-thermometer-outline" => Some(icons::HOME_THERMOMETER_OUTLINE),
        "battery-charging-wireless-20" => Some(icons::BATTERY_CHARGING_WIRELESS_20),
        "gate-not" => Some(icons::GATE_NOT),
        "tune-vertical-variant" => Some(icons::TUNE_VERTICAL_VARIANT),
        "waterfall" => Some(icons::WATERFALL),
        _ => None,
    }
}
