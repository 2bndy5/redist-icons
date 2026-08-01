// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_2(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "thumb-up" => Some(icons::THUMB_UP),
        "api-off" => Some(icons::API_OFF),
        "magnify-minus-cursor" => Some(icons::MAGNIFY_MINUS_CURSOR),
        "credit-card" => Some(icons::CREDIT_CARD),
        "relation-only-one-to-only-one" => Some(icons::RELATION_ONLY_ONE_TO_ONLY_ONE),
        "airplane-takeoff" => Some(icons::AIRPLANE_TAKEOFF),
        "text-search" => Some(icons::TEXT_SEARCH),
        "folder-clock-outline" => Some(icons::FOLDER_CLOCK_OUTLINE),
        "move-resize" => Some(icons::MOVE_RESIZE),
        #[allow(deprecated)]
        "unity" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unity' is deprecated.").print(py);
            }
            Some(icons::UNITY)
        }
        "table-merge-cells" => Some(icons::TABLE_MERGE_CELLS),
        #[allow(deprecated)]
        "android-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android-studio' is deprecated.").print(py);
            }
            Some(icons::ANDROID_STUDIO)
        }
        "source-branch-refresh" => Some(icons::SOURCE_BRANCH_REFRESH),
        "map-plus" => Some(icons::MAP_PLUS),
        "dots-hexagon" => Some(icons::DOTS_HEXAGON),
        "card-off-outline" => Some(icons::CARD_OFF_OUTLINE),
        "math-tan" => Some(icons::MATH_TAN),
        "cursor-default-gesture-outline" => Some(icons::CURSOR_DEFAULT_GESTURE_OUTLINE),
        "table-picnic" => Some(icons::TABLE_PICNIC),
        "compare-remove" => Some(icons::COMPARE_REMOVE),
        "timer-check-outline" => Some(icons::TIMER_CHECK_OUTLINE),
        "mouse-left-click" => Some(icons::MOUSE_LEFT_CLICK),
        "fit-to-screen-outline" => Some(icons::FIT_TO_SCREEN_OUTLINE),
        "alpha-w" => Some(icons::ALPHA_W),
        "stop-circle" => Some(icons::STOP_CIRCLE),
        "microphone" => Some(icons::MICROPHONE),
        "alphabetical" => Some(icons::ALPHABETICAL),
        "flask-empty-remove-outline" => Some(icons::FLASK_EMPTY_REMOVE_OUTLINE),
        "server-security" => Some(icons::SERVER_SECURITY),
        "comment-quote" => Some(icons::COMMENT_QUOTE),
        "data-matrix-plus" => Some(icons::DATA_MATRIX_PLUS),
        "arrow-left-thin-circle-outline" => Some(icons::ARROW_LEFT_THIN_CIRCLE_OUTLINE),
        "timeline-remove-outline" => Some(icons::TIMELINE_REMOVE_OUTLINE),
        "hexagon" => Some(icons::HEXAGON),
        "relation-one-to-many" => Some(icons::RELATION_ONE_TO_MANY),
        "van-passenger" => Some(icons::VAN_PASSENGER),
        "smart-card" => Some(icons::SMART_CARD),
        #[allow(deprecated)]
        "google-play" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-play' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLAY)
        }
        "tune-variant" => Some(icons::TUNE_VARIANT),
        "boom-gate-arrow-down" => Some(icons::BOOM_GATE_ARROW_DOWN),
        "food-steak" => Some(icons::FOOD_STEAK),
        "tractor-variant" => Some(icons::TRACTOR_VARIANT),
        "filter-variant-minus" => Some(icons::FILTER_VARIANT_MINUS),
        "format-annotation-plus" => Some(icons::FORMAT_ANNOTATION_PLUS),
        "emoticon-excited" => Some(icons::EMOTICON_EXCITED),
        #[allow(deprecated)]
        "apple" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple' is deprecated.").print(py);
            }
            Some(icons::APPLE)
        }
        "calendar-week-outline" => Some(icons::CALENDAR_WEEK_OUTLINE),
        "skip-forward-outline" => Some(icons::SKIP_FORWARD_OUTLINE),
        "archive-star" => Some(icons::ARCHIVE_STAR),
        "folder-off-outline" => Some(icons::FOLDER_OFF_OUTLINE),
        "arrow-right-thin-circle-outline" => Some(icons::ARROW_RIGHT_THIN_CIRCLE_OUTLINE),
        "arrow-up-down-bold-outline" => Some(icons::ARROW_UP_DOWN_BOLD_OUTLINE),
        "page-layout-sidebar-left" => Some(icons::PAGE_LAYOUT_SIDEBAR_LEFT),
        "book-arrow-left-outline" => Some(icons::BOOK_ARROW_LEFT_OUTLINE),
        "nature-people" => Some(icons::NATURE_PEOPLE),
        "elephant" => Some(icons::ELEPHANT),
        "certificate-outline" => Some(icons::CERTIFICATE_OUTLINE),
        "download-multiple" => Some(icons::DOWNLOAD_MULTIPLE),
        "numeric-9-circle" => Some(icons::NUMERIC_9_CIRCLE),
        "keyboard-close" => Some(icons::KEYBOARD_CLOSE),
        "file-image-minus-outline" => Some(icons::FILE_IMAGE_MINUS_OUTLINE),
        "stool" => Some(icons::STOOL),
        "weather-cloudy" => Some(icons::WEATHER_CLOUDY),
        "store-off-outline" => Some(icons::STORE_OFF_OUTLINE),
        "pasta" => Some(icons::PASTA),
        "hospital-box" => Some(icons::HOSPITAL_BOX),
        "flag-variant-minus-outline" => Some(icons::FLAG_VARIANT_MINUS_OUTLINE),
        "medical-bag" => Some(icons::MEDICAL_BAG),
        "alpha-k-circle-outline" => Some(icons::ALPHA_K_CIRCLE_OUTLINE),
        "invoice-text-minus-outline" => Some(icons::INVOICE_TEXT_MINUS_OUTLINE),
        "invoice-text-multiple" => Some(icons::INVOICE_TEXT_MULTIPLE),
        "cctv-off" => Some(icons::CCTV_OFF),
        "moped-outline" => Some(icons::MOPED_OUTLINE),
        "multiplication-box" => Some(icons::MULTIPLICATION_BOX),
        "battery-alert" => Some(icons::BATTERY_ALERT),
        "purse" => Some(icons::PURSE),
        "file-star-outline" => Some(icons::FILE_STAR_OUTLINE),
        "window-closed-variant" => Some(icons::WINDOW_CLOSED_VARIANT),
        "printer-pos-sync" => Some(icons::PRINTER_POS_SYNC),
        "folder-arrow-up-outline" => Some(icons::FOLDER_ARROW_UP_OUTLINE),
        "phone-bluetooth" => Some(icons::PHONE_BLUETOOTH),
        "console-network-outline" => Some(icons::CONSOLE_NETWORK_OUTLINE),
        "screen-rotation" => Some(icons::SCREEN_ROTATION),
        "dolly" => Some(icons::DOLLY),
        "film" => Some(icons::FILM),
        "propane-tank-outline" => Some(icons::PROPANE_TANK_OUTLINE),
        "wiper" => Some(icons::WIPER),
        "fruit-watermelon" => Some(icons::FRUIT_WATERMELON),
        "video-box-off" => Some(icons::VIDEO_BOX_OFF),
        "passport-biometric" => Some(icons::PASSPORT_BIOMETRIC),
        "printer-pos-minus" => Some(icons::PRINTER_POS_MINUS),
        "camera-control" => Some(icons::CAMERA_CONTROL),
        "calendar-cursor-outline" => Some(icons::CALENDAR_CURSOR_OUTLINE),
        "solar-power-variant-outline" => Some(icons::SOLAR_POWER_VARIANT_OUTLINE),
        "account" => Some(icons::ACCOUNT),
        "bank-minus" => Some(icons::BANK_MINUS),
        "transfer-left" => Some(icons::TRANSFER_LEFT),
        "tea-outline" => Some(icons::TEA_OUTLINE),
        "account-off-outline" => Some(icons::ACCOUNT_OFF_OUTLINE),
        "train-car-centerbeam" => Some(icons::TRAIN_CAR_CENTERBEAM),
        "projector-screen-off" => Some(icons::PROJECTOR_SCREEN_OFF),
        "circle-edit-outline" => Some(icons::CIRCLE_EDIT_OUTLINE),
        "numeric-5-box-multiple-outline" => Some(icons::NUMERIC_5_BOX_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "vuetify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuetify' is deprecated.").print(py);
            }
            Some(icons::VUETIFY)
        }
        "controller-classic" => Some(icons::CONTROLLER_CLASSIC),
        "monitor-lock" => Some(icons::MONITOR_LOCK),
        "arrow-left-bold-box" => Some(icons::ARROW_LEFT_BOLD_BOX),
        "registered-trademark" => Some(icons::REGISTERED_TRADEMARK),
        "syllabary-katakana-halfwidth" => Some(icons::SYLLABARY_KATAKANA_HALFWIDTH),
        "office-building-minus-outline" => Some(icons::OFFICE_BUILDING_MINUS_OUTLINE),
        "briefcase-arrow-up-down" => Some(icons::BRIEFCASE_ARROW_UP_DOWN),
        "battery-charging-50" => Some(icons::BATTERY_CHARGING_50),
        "ray-end" => Some(icons::RAY_END),
        "layers-off-outline" => Some(icons::LAYERS_OFF_OUTLINE),
        "database-eye-off-outline" => Some(icons::DATABASE_EYE_OFF_OUTLINE),
        "account-off" => Some(icons::ACCOUNT_OFF),
        "account-sync-outline" => Some(icons::ACCOUNT_SYNC_OUTLINE),
        "format-list-bulleted-triangle" => Some(icons::FORMAT_LIST_BULLETED_TRIANGLE),
        #[allow(deprecated)]
        "openid" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'openid' is deprecated.").print(py);
            }
            Some(icons::OPENID)
        }
        "account-badge-outline" => Some(icons::ACCOUNT_BADGE_OUTLINE),
        "text-box-check-outline" => Some(icons::TEXT_BOX_CHECK_OUTLINE),
        "alphabet-latin" => Some(icons::ALPHABET_LATIN),
        "zodiac-pisces" => Some(icons::ZODIAC_PISCES),
        "timer-lock-open" => Some(icons::TIMER_LOCK_OPEN),
        "format-header-5" => Some(icons::FORMAT_HEADER_5),
        "phone-message" => Some(icons::PHONE_MESSAGE),
        "sticker-alert" => Some(icons::STICKER_ALERT),
        "message-reply-text-outline" => Some(icons::MESSAGE_REPLY_TEXT_OUTLINE),
        #[allow(deprecated)]
        "bulma" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bulma' is deprecated.").print(py);
            }
            Some(icons::BULMA)
        }
        "file-chart-check-outline" => Some(icons::FILE_CHART_CHECK_OUTLINE),
        "glass-pint-outline" => Some(icons::GLASS_PINT_OUTLINE),
        "vector-circle-variant" => Some(icons::VECTOR_CIRCLE_VARIANT),
        "shield-alert-outline" => Some(icons::SHIELD_ALERT_OUTLINE),
        "umbrella-closed-outline" => Some(icons::UMBRELLA_CLOSED_OUTLINE),
        "account-tag" => Some(icons::ACCOUNT_TAG),
        "timeline-plus-outline" => Some(icons::TIMELINE_PLUS_OUTLINE),
        "playlist-remove" => Some(icons::PLAYLIST_REMOVE),
        "human-male-female" => Some(icons::HUMAN_MALE_FEMALE),
        "nutrition" => Some(icons::NUTRITION),
        "hand-wash-outline" => Some(icons::HAND_WASH_OUTLINE),
        #[allow(deprecated)]
        "google-keep" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-keep' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_KEEP)
        }
        "gamepad-down" => Some(icons::GAMEPAD_DOWN),
        "book-arrow-up-outline" => Some(icons::BOOK_ARROW_UP_OUTLINE),
        "food-apple-outline" => Some(icons::FOOD_APPLE_OUTLINE),
        "newspaper-plus" => Some(icons::NEWSPAPER_PLUS),
        "numeric-4-box-multiple-outline" => Some(icons::NUMERIC_4_BOX_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "gentoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gentoo' is deprecated.").print(py);
            }
            Some(icons::GENTOO)
        }
        "email-multiple" => Some(icons::EMAIL_MULTIPLE),
        "calendar-alert" => Some(icons::CALENDAR_ALERT),
        "microscope" => Some(icons::MICROSCOPE),
        "lock-minus" => Some(icons::LOCK_MINUS),
        "drawing" => Some(icons::DRAWING),
        "thermometer-high" => Some(icons::THERMOMETER_HIGH),
        "lightbulb-off-outline" => Some(icons::LIGHTBULB_OFF_OUTLINE),
        "bag-suitcase" => Some(icons::BAG_SUITCASE),
        "led-strip-variant-off" => Some(icons::LED_STRIP_VARIANT_OFF),
        "briefcase-variant-outline" => Some(icons::BRIEFCASE_VARIANT_OUTLINE),
        "gate-nand" => Some(icons::GATE_NAND),
        "tooltip-edit-outline" => Some(icons::TOOLTIP_EDIT_OUTLINE),
        #[allow(deprecated)]
        "google-classroom" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-classroom' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CLASSROOM)
        }
        "message-bookmark-outline" => Some(icons::MESSAGE_BOOKMARK_OUTLINE),
        "eye-plus-outline" => Some(icons::EYE_PLUS_OUTLINE),
        "dog-side-off" => Some(icons::DOG_SIDE_OFF),
        "transmission-tower-export" => Some(icons::TRANSMISSION_TOWER_EXPORT),
        "clipboard-alert" => Some(icons::CLIPBOARD_ALERT),
        "cloud-off-outline" => Some(icons::CLOUD_OFF_OUTLINE),
        "movie-open-star" => Some(icons::MOVIE_OPEN_STAR),
        "information-variant-circle-outline" => Some(icons::INFORMATION_VARIANT_CIRCLE_OUTLINE),
        "truck-delivery-outline" => Some(icons::TRUCK_DELIVERY_OUTLINE),
        "border-left-variant" => Some(icons::BORDER_LEFT_VARIANT),
        "basket" => Some(icons::BASKET),
        "printer-off" => Some(icons::PRINTER_OFF),
        "land-plots-circle" => Some(icons::LAND_PLOTS_CIRCLE),
        "excavator" => Some(icons::EXCAVATOR),
        "receipt-text-arrow-left" => Some(icons::RECEIPT_TEXT_ARROW_LEFT),
        "solar-panel-large" => Some(icons::SOLAR_PANEL_LARGE),
        "receipt-text-send" => Some(icons::RECEIPT_TEXT_SEND),
        "fuse" => Some(icons::FUSE),
        "beer" => Some(icons::BEER),
        "home-off" => Some(icons::HOME_OFF),
        "flag-outline" => Some(icons::FLAG_OUTLINE),
        #[allow(deprecated)]
        "nintendo-wiiu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wiiu' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WIIU)
        }
        "invoice-text-arrow-right-outline" => Some(icons::INVOICE_TEXT_ARROW_RIGHT_OUTLINE),
        "incognito-circle" => Some(icons::INCOGNITO_CIRCLE),
        "alpha-c-circle" => Some(icons::ALPHA_C_CIRCLE),
        "palm-tree" => Some(icons::PALM_TREE),
        "pail-remove" => Some(icons::PAIL_REMOVE),
        "creation-outline" => Some(icons::CREATION_OUTLINE),
        "hand-water" => Some(icons::HAND_WATER),
        "underwear-outline" => Some(icons::UNDERWEAR_OUTLINE),
        "gavel" => Some(icons::GAVEL),
        "lightbulb-on" => Some(icons::LIGHTBULB_ON),
        #[allow(deprecated)]
        "aws" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'aws' is deprecated.").print(py);
            }
            Some(icons::AWS)
        }
        "glass-tulip" => Some(icons::GLASS_TULIP),
        "clock-time-five-outline" => Some(icons::CLOCK_TIME_FIVE_OUTLINE),
        "dice-6-outline" => Some(icons::DICE_6_OUTLINE),
        "video-minus" => Some(icons::VIDEO_MINUS),
        "fan-alert" => Some(icons::FAN_ALERT),
        "robot-vacuum-alert" => Some(icons::ROBOT_VACUUM_ALERT),
        "ballot-recount" => Some(icons::BALLOT_RECOUNT),
        _ => None,
    }
}
