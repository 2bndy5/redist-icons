// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_24(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "ambulance" => Some(icons::AMBULANCE),
        "database-sync" => Some(icons::DATABASE_SYNC),
        "key-plus" => Some(icons::KEY_PLUS),
        "brightness-7" => Some(icons::BRIGHTNESS_7),
        "invoice-text-edit-outline" => Some(icons::INVOICE_TEXT_EDIT_OUTLINE),
        "bottle-tonic" => Some(icons::BOTTLE_TONIC),
        "distribute-vertical-top" => Some(icons::DISTRIBUTE_VERTICAL_TOP),
        "account-cancel-outline" => Some(icons::ACCOUNT_CANCEL_OUTLINE),
        "lightbulb-cfl-spiral-off" => Some(icons::LIGHTBULB_CFL_SPIRAL_OFF),
        #[allow(deprecated)]
        "symfony" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'symfony' is deprecated.").print(py);
            }
            Some(icons::SYMFONY)
        }
        "fast-forward" => Some(icons::FAST_FORWARD),
        "square-off" => Some(icons::SQUARE_OFF),
        "credit-card-refund-outline" => Some(icons::CREDIT_CARD_REFUND_OUTLINE),
        "vector-intersection" => Some(icons::VECTOR_INTERSECTION),
        "raw-off" => Some(icons::RAW_OFF),
        "email-remove" => Some(icons::EMAIL_REMOVE),
        "briefcase-minus" => Some(icons::BRIEFCASE_MINUS),
        "archive-check-outline" => Some(icons::ARCHIVE_CHECK_OUTLINE),
        "upload-multiple" => Some(icons::UPLOAD_MULTIPLE),
        "lightbulb-variant-outline" => Some(icons::LIGHTBULB_VARIANT_OUTLINE),
        "filter-settings" => Some(icons::FILTER_SETTINGS),
        #[allow(deprecated)]
        "webrtc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webrtc' is deprecated.").print(py);
            }
            Some(icons::WEBRTC)
        }
        "folder-lock-open" => Some(icons::FOLDER_LOCK_OPEN),
        "alpha-s" => Some(icons::ALPHA_S),
        "animation-outline" => Some(icons::ANIMATION_OUTLINE),
        "weather-fog" => Some(icons::WEATHER_FOG),
        "flower-pollen-outline" => Some(icons::FLOWER_POLLEN_OUTLINE),
        "book" => Some(icons::BOOK),
        "axe-battle" => Some(icons::AXE_BATTLE),
        "table-large" => Some(icons::TABLE_LARGE),
        "alpha-w-box-outline" => Some(icons::ALPHA_W_BOX_OUTLINE),
        "toolbox-outline" => Some(icons::TOOLBOX_OUTLINE),
        "hockey-sticks" => Some(icons::HOCKEY_STICKS),
        "new-box" => Some(icons::NEW_BOX),
        "folder-download" => Some(icons::FOLDER_DOWNLOAD),
        "arrow-u-up-left" => Some(icons::ARROW_U_UP_LEFT),
        "alpha-k-circle" => Some(icons::ALPHA_K_CIRCLE),
        #[allow(deprecated)]
        "onepassword" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'onepassword' is deprecated.").print(py);
            }
            Some(icons::ONEPASSWORD)
        }
        "football" => Some(icons::FOOTBALL),
        #[allow(deprecated)]
        "language-php" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-php' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_PHP)
        }
        "briefcase-arrow-left-right-outline" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT_OUTLINE),
        "format-list-numbered-rtl" => Some(icons::FORMAT_LIST_NUMBERED_RTL),
        "mailbox-outline" => Some(icons::MAILBOX_OUTLINE),
        "printer-search" => Some(icons::PRINTER_SEARCH),
        "headphones-off" => Some(icons::HEADPHONES_OFF),
        "code-block-brackets" => Some(icons::CODE_BLOCK_BRACKETS),
        "home-group-minus" => Some(icons::HOME_GROUP_MINUS),
        "nature" => Some(icons::NATURE),
        "pan-horizontal" => Some(icons::PAN_HORIZONTAL),
        "pail-minus" => Some(icons::PAIL_MINUS),
        "arrow-bottom-right-thick" => Some(icons::ARROW_BOTTOM_RIGHT_THICK),
        "car-outline" => Some(icons::CAR_OUTLINE),
        "receipt-text-minus-outline" => Some(icons::RECEIPT_TEXT_MINUS_OUTLINE),
        "power-socket" => Some(icons::POWER_SOCKET),
        "view-module" => Some(icons::VIEW_MODULE),
        "hexagon-multiple-outline" => Some(icons::HEXAGON_MULTIPLE_OUTLINE),
        "sign-language-outline" => Some(icons::SIGN_LANGUAGE_OUTLINE),
        "stool-outline" => Some(icons::STOOL_OUTLINE),
        "reply-all-outline" => Some(icons::REPLY_ALL_OUTLINE),
        "heart-plus" => Some(icons::HEART_PLUS),
        "export-variant" => Some(icons::EXPORT_VARIANT),
        "broadcast" => Some(icons::BROADCAST),
        "view-grid-compact" => Some(icons::VIEW_GRID_COMPACT),
        "account-box-minus-outline" => Some(icons::ACCOUNT_BOX_MINUS_OUTLINE),
        "web-off" => Some(icons::WEB_OFF),
        "file-lock-open" => Some(icons::FILE_LOCK_OPEN),
        "car-coolant-level" => Some(icons::CAR_COOLANT_LEVEL),
        "format-wrap-top-bottom" => Some(icons::FORMAT_WRAP_TOP_BOTTOM),
        "rocket-launch-outline" => Some(icons::ROCKET_LAUNCH_OUTLINE),
        "horizontal-rotate-counterclockwise" => Some(icons::HORIZONTAL_ROTATE_COUNTERCLOCKWISE),
        "dots-hexagon" => Some(icons::DOTS_HEXAGON),
        "calendar-clock-outline" => Some(icons::CALENDAR_CLOCK_OUTLINE),
        "alpha-u-circle" => Some(icons::ALPHA_U_CIRCLE),
        "flag-remove-outline" => Some(icons::FLAG_REMOVE_OUTLINE),
        "trophy-variant" => Some(icons::TROPHY_VARIANT),
        "file-send-outline" => Some(icons::FILE_SEND_OUTLINE),
        "flag-variant-outline" => Some(icons::FLAG_VARIANT_OUTLINE),
        "transcribe-close" => Some(icons::TRANSCRIBE_CLOSE),
        "image-plus" => Some(icons::IMAGE_PLUS),
        "message-processing-outline" => Some(icons::MESSAGE_PROCESSING_OUTLINE),
        "calendar-check" => Some(icons::CALENDAR_CHECK),
        "smoke-detector-outline" => Some(icons::SMOKE_DETECTOR_OUTLINE),
        "archive-clock-outline" => Some(icons::ARCHIVE_CLOCK_OUTLINE),
        "shuffle-disabled" => Some(icons::SHUFFLE_DISABLED),
        "zodiac-capricorn" => Some(icons::ZODIAC_CAPRICORN),
        #[allow(deprecated)]
        "language-javascript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-javascript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_JAVASCRIPT)
        }
        "shuffle" => Some(icons::SHUFFLE),
        "bookshelf" => Some(icons::BOOKSHELF),
        "filter-settings-outline" => Some(icons::FILTER_SETTINGS_OUTLINE),
        "eye-circle-outline" => Some(icons::EYE_CIRCLE_OUTLINE),
        "bacteria-outline" => Some(icons::BACTERIA_OUTLINE),
        "toggle-switch-outline" => Some(icons::TOGGLE_SWITCH_OUTLINE),
        "numeric-3-box-outline" => Some(icons::NUMERIC_3_BOX_OUTLINE),
        "arrow-down" => Some(icons::ARROW_DOWN),
        "death-star" => Some(icons::DEATH_STAR),
        "view-quilt" => Some(icons::VIEW_QUILT),
        "slope-downhill" => Some(icons::SLOPE_DOWNHILL),
        "power-socket-us" => Some(icons::POWER_SOCKET_US),
        "table-network" => Some(icons::TABLE_NETWORK),
        "coffee-maker" => Some(icons::COFFEE_MAKER),
        "timer-marker" => Some(icons::TIMER_MARKER),
        "alpha-a-box" => Some(icons::ALPHA_A_BOX),
        "dice-multiple-outline" => Some(icons::DICE_MULTIPLE_OUTLINE),
        "lock-open-variant-outline" => Some(icons::LOCK_OPEN_VARIANT_OUTLINE),
        "axis-x-arrow-lock" => Some(icons::AXIS_X_ARROW_LOCK),
        "format-header-5" => Some(icons::FORMAT_HEADER_5),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-empty" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-empty' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_EMPTY)
        }
        "alarm-check" => Some(icons::ALARM_CHECK),
        "phone-incoming-outgoing-outline" => Some(icons::PHONE_INCOMING_OUTGOING_OUTLINE),
        "soccer-field" => Some(icons::SOCCER_FIELD),
        "folder-question" => Some(icons::FOLDER_QUESTION),
        "egg-fried" => Some(icons::EGG_FRIED),
        "floor-lamp-outline" => Some(icons::FLOOR_LAMP_OUTLINE),
        "relation-zero-or-many-to-one-or-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE_OR_MANY),
        "folder-edit" => Some(icons::FOLDER_EDIT),
        "link" => Some(icons::LINK),
        "brightness-6" => Some(icons::BRIGHTNESS_6),
        "clipboard-file-outline" => Some(icons::CLIPBOARD_FILE_OUTLINE),
        "credit-card-chip" => Some(icons::CREDIT_CARD_CHIP),
        "smart-card-outline" => Some(icons::SMART_CARD_OUTLINE),
        "fan-alert" => Some(icons::FAN_ALERT),
        "image-filter-center-focus" => Some(icons::IMAGE_FILTER_CENTER_FOCUS),
        "format-text-rotation-vertical" => Some(icons::FORMAT_TEXT_ROTATION_VERTICAL),
        "credit-card-edit-outline" => Some(icons::CREDIT_CARD_EDIT_OUTLINE),
        "panorama-sphere" => Some(icons::PANORAMA_SPHERE),
        "delete-off-outline" => Some(icons::DELETE_OFF_OUTLINE),
        "lasso" => Some(icons::LASSO),
        "window-maximize" => Some(icons::WINDOW_MAXIMIZE),
        "car-3-plus" => Some(icons::CAR_3_PLUS),
        "calendar-multiple" => Some(icons::CALENDAR_MULTIPLE),
        "kayaking" => Some(icons::KAYAKING),
        "chart-tree" => Some(icons::CHART_TREE),
        "signal-5g" => Some(icons::SIGNAL_5G),
        "movie-minus-outline" => Some(icons::MOVIE_MINUS_OUTLINE),
        "unfold-less-vertical" => Some(icons::UNFOLD_LESS_VERTICAL),
        "food-drumstick-off-outline" => Some(icons::FOOD_DRUMSTICK_OFF_OUTLINE),
        "shaker-outline" => Some(icons::SHAKER_OUTLINE),
        "note-plus-outline" => Some(icons::NOTE_PLUS_OUTLINE),
        "dice-d6" => Some(icons::DICE_D6),
        "emoticon-kiss" => Some(icons::EMOTICON_KISS),
        "skip-next-outline" => Some(icons::SKIP_NEXT_OUTLINE),
        "numeric-9-plus-box-multiple-outline" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE_OUTLINE),
        "calendar-end-outline" => Some(icons::CALENDAR_END_OUTLINE),
        "ceiling-fan-light" => Some(icons::CEILING_FAN_LIGHT),
        "order-numeric-ascending" => Some(icons::ORDER_NUMERIC_ASCENDING),
        "timeline-check" => Some(icons::TIMELINE_CHECK),
        "dump-truck" => Some(icons::DUMP_TRUCK),
        "cradle" => Some(icons::CRADLE),
        "triangle-wave" => Some(icons::TRIANGLE_WAVE),
        "folder-information" => Some(icons::FOLDER_INFORMATION),
        #[allow(deprecated)]
        "yahoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'yahoo' is deprecated.").print(py);
            }
            Some(icons::YAHOO)
        }
        "file-star-four-points" => Some(icons::FILE_STAR_FOUR_POINTS),
        #[allow(deprecated)]
        "reddit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'reddit' is deprecated.").print(py);
            }
            Some(icons::REDDIT)
        }
        "phone-classic" => Some(icons::PHONE_CLASSIC),
        "gate-nor" => Some(icons::GATE_NOR),
        "content-cut" => Some(icons::CONTENT_CUT),
        "percent-circle" => Some(icons::PERCENT_CIRCLE),
        "cloud-alert-outline" => Some(icons::CLOUD_ALERT_OUTLINE),
        "sprout-outline" => Some(icons::SPROUT_OUTLINE),
        "wallet-plus-outline" => Some(icons::WALLET_PLUS_OUTLINE),
        "bag-carry-on-check" => Some(icons::BAG_CARRY_ON_CHECK),
        "shark-off" => Some(icons::SHARK_OFF),
        "bed-double" => Some(icons::BED_DOUBLE),
        "power-standby" => Some(icons::POWER_STANDBY),
        "copyleft" => Some(icons::COPYLEFT),
        "purse-outline" => Some(icons::PURSE_OUTLINE),
        "bluetooth-transfer" => Some(icons::BLUETOOTH_TRANSFER),
        "chart-histogram" => Some(icons::CHART_HISTOGRAM),
        "widgets-outline" => Some(icons::WIDGETS_OUTLINE),
        "gauge-full" => Some(icons::GAUGE_FULL),
        "file-table-box" => Some(icons::FILE_TABLE_BOX),
        "alpha-k-box-outline" => Some(icons::ALPHA_K_BOX_OUTLINE),
        "basket-unfill" => Some(icons::BASKET_UNFILL),
        "printer-alert" => Some(icons::PRINTER_ALERT),
        "pier" => Some(icons::PIER),
        "wifi-cancel" => Some(icons::WIFI_CANCEL),
        "puzzle-heart" => Some(icons::PUZZLE_HEART),
        "mailbox" => Some(icons::MAILBOX),
        "briefcase-variant-off-outline" => Some(icons::BRIEFCASE_VARIANT_OFF_OUTLINE),
        "numeric-4-box-multiple" => Some(icons::NUMERIC_4_BOX_MULTIPLE),
        "microwave-off" => Some(icons::MICROWAVE_OFF),
        "cellphone-information" => Some(icons::CELLPHONE_INFORMATION),
        #[allow(deprecated)]
        "google-circles-extended" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles-extended' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_EXTENDED)
        }
        "firework-off" => Some(icons::FIREWORK_OFF),
        "chat-outline" => Some(icons::CHAT_OUTLINE),
        "checkbook-arrow-left" => Some(icons::CHECKBOOK_ARROW_LEFT),
        "tree-outline" => Some(icons::TREE_OUTLINE),
        "domain-switch" => Some(icons::DOMAIN_SWITCH),
        "account-off" => Some(icons::ACCOUNT_OFF),
        "gender-female" => Some(icons::GENDER_FEMALE),
        "plus-box-multiple" => Some(icons::PLUS_BOX_MULTIPLE),
        "power-plug-off" => Some(icons::POWER_PLUG_OFF),
        "cards-playing-spade" => Some(icons::CARDS_PLAYING_SPADE),
        "weather-hurricane-outline" => Some(icons::WEATHER_HURRICANE_OUTLINE),
        "clock-time-twelve-outline" => Some(icons::CLOCK_TIME_TWELVE_OUTLINE),
        "instrument-triangle" => Some(icons::INSTRUMENT_TRIANGLE),
        "van-passenger" => Some(icons::VAN_PASSENGER),
        "filter" => Some(icons::FILTER),
        #[allow(deprecated)]
        "youtube-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-gaming' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_GAMING)
        }
        "alpha-y-circle" => Some(icons::ALPHA_Y_CIRCLE),
        _ => None,
    }
}
