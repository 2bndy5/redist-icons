// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_13(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "menu-down" => Some(icons::MENU_DOWN),
        "heart-minus-outline" => Some(icons::HEART_MINUS_OUTLINE),
        "triangle-down-outline" => Some(icons::TRIANGLE_DOWN_OUTLINE),
        "cone" => Some(icons::CONE),
        "alert-plus-outline" => Some(icons::ALERT_PLUS_OUTLINE),
        "account-arrow-up-outline" => Some(icons::ACCOUNT_ARROW_UP_OUTLINE),
        "radiobox-indeterminate-variant" => Some(icons::RADIOBOX_INDETERMINATE_VARIANT),
        "alpha-f-box-outline" => Some(icons::ALPHA_F_BOX_OUTLINE),
        "arrow-expand" => Some(icons::ARROW_EXPAND),
        "alpha-z-box-outline" => Some(icons::ALPHA_Z_BOX_OUTLINE),
        "rss-off" => Some(icons::RSS_OFF),
        "mine" => Some(icons::MINE),
        "smart-card-off-outline" => Some(icons::SMART_CARD_OFF_OUTLINE),
        "clipboard-edit-outline" => Some(icons::CLIPBOARD_EDIT_OUTLINE),
        "share-variant-outline" => Some(icons::SHARE_VARIANT_OUTLINE),
        "relation-many-to-zero-or-many" => Some(icons::RELATION_MANY_TO_ZERO_OR_MANY),
        #[allow(deprecated)]
        "netflix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'netflix' is deprecated.").print(py);
            }
            Some(icons::NETFLIX)
        }
        "open-in-new" => Some(icons::OPEN_IN_NEW),
        "checkbox-blank-badge" => Some(icons::CHECKBOX_BLANK_BADGE),
        "city-switch" => Some(icons::CITY_SWITCH),
        "shield-key-outline" => Some(icons::SHIELD_KEY_OUTLINE),
        "view-dashboard-variant-outline" => Some(icons::VIEW_DASHBOARD_VARIANT_OUTLINE),
        "archive-eye" => Some(icons::ARCHIVE_EYE),
        "printer-pos-wrench-outline" => Some(icons::PRINTER_POS_WRENCH_OUTLINE),
        #[allow(deprecated)]
        "language-php" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-php' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_PHP)
        }
        "credit-card-chip-outline" => Some(icons::CREDIT_CARD_CHIP_OUTLINE),
        "details" => Some(icons::DETAILS),
        "emoticon-angry" => Some(icons::EMOTICON_ANGRY),
        #[allow(deprecated)]
        "fedora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'fedora' is deprecated.").print(py);
            }
            Some(icons::FEDORA)
        }
        "card-remove-outline" => Some(icons::CARD_REMOVE_OUTLINE),
        "book-settings" => Some(icons::BOOK_SETTINGS),
        "consolidate" => Some(icons::CONSOLIDATE),
        "store-marker-outline" => Some(icons::STORE_MARKER_OUTLINE),
        #[allow(deprecated)]
        "terraform" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'terraform' is deprecated.").print(py);
            }
            Some(icons::TERRAFORM)
        }
        "auto-download" => Some(icons::AUTO_DOWNLOAD),
        "tray" => Some(icons::TRAY),
        "smart-card" => Some(icons::SMART_CARD),
        "remote-tv" => Some(icons::REMOTE_TV),
        "numeric-6-circle" => Some(icons::NUMERIC_6_CIRCLE),
        "cctv-off" => Some(icons::CCTV_OFF),
        "printer-3d-nozzle-alert" => Some(icons::PRINTER_3D_NOZZLE_ALERT),
        "border-none" => Some(icons::BORDER_NONE),
        "file-restore-outline" => Some(icons::FILE_RESTORE_OUTLINE),
        "wrench-clock" => Some(icons::WRENCH_CLOCK),
        "music-box-multiple" => Some(icons::MUSIC_BOX_MULTIPLE),
        "flask-empty-remove" => Some(icons::FLASK_EMPTY_REMOVE),
        "candy" => Some(icons::CANDY),
        "tune-vertical" => Some(icons::TUNE_VERTICAL),
        "movie" => Some(icons::MOVIE),
        "wifi-arrow-left-right" => Some(icons::WIFI_ARROW_LEFT_RIGHT),
        "file-image-minus" => Some(icons::FILE_IMAGE_MINUS),
        "camera-account" => Some(icons::CAMERA_ACCOUNT),
        "plus-network-outline" => Some(icons::PLUS_NETWORK_OUTLINE),
        "tumble-dryer-off" => Some(icons::TUMBLE_DRYER_OFF),
        "upload-network-outline" => Some(icons::UPLOAD_NETWORK_OUTLINE),
        #[allow(deprecated)]
        "facebook-messenger" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-messenger' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_MESSENGER)
        }
        "gamepad-circle-right" => Some(icons::GAMEPAD_CIRCLE_RIGHT),
        "weather-partly-snowy-rainy" => Some(icons::WEATHER_PARTLY_SNOWY_RAINY),
        "car-cruise-control" => Some(icons::CAR_CRUISE_CONTROL),
        "file-pdf-box" => Some(icons::FILE_PDF_BOX),
        "switch" => Some(icons::SWITCH),
        "koala" => Some(icons::KOALA),
        "account-credit-card" => Some(icons::ACCOUNT_CREDIT_CARD),
        "badge-account-alert-outline" => Some(icons::BADGE_ACCOUNT_ALERT_OUTLINE),
        "tag-text" => Some(icons::TAG_TEXT),
        "contain-end" => Some(icons::CONTAIN_END),
        "alpha-q-circle-outline" => Some(icons::ALPHA_Q_CIRCLE_OUTLINE),
        "book-cross" => Some(icons::BOOK_CROSS),
        "sync-alert" => Some(icons::SYNC_ALERT),
        "office-building-marker" => Some(icons::OFFICE_BUILDING_MARKER),
        "jellyfish" => Some(icons::JELLYFISH),
        "book-cog-outline" => Some(icons::BOOK_COG_OUTLINE),
        "arm-flex" => Some(icons::ARM_FLEX),
        "folder-multiple-outline" => Some(icons::FOLDER_MULTIPLE_OUTLINE),
        "lingerie" => Some(icons::LINGERIE),
        "thermometer-chevron-down" => Some(icons::THERMOMETER_CHEVRON_DOWN),
        "fridge-off-outline" => Some(icons::FRIDGE_OFF_OUTLINE),
        "cog-clockwise" => Some(icons::COG_CLOCKWISE),
        "cloud-arrow-down-outline" => Some(icons::CLOUD_ARROW_DOWN_OUTLINE),
        "folder-upload-outline" => Some(icons::FOLDER_UPLOAD_OUTLINE),
        "wifi-strength-lock-open-outline" => Some(icons::WIFI_STRENGTH_LOCK_OPEN_OUTLINE),
        "folder-edit" => Some(icons::FOLDER_EDIT),
        "biohazard" => Some(icons::BIOHAZARD),
        "lightning-bolt" => Some(icons::LIGHTNING_BOLT),
        "lightbulb-multiple-off-outline" => Some(icons::LIGHTBULB_MULTIPLE_OFF_OUTLINE),
        #[allow(deprecated)]
        "unreal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unreal' is deprecated.").print(py);
            }
            Some(icons::UNREAL)
        }
        "pail-off" => Some(icons::PAIL_OFF),
        "newspaper-check" => Some(icons::NEWSPAPER_CHECK),
        "clipboard-arrow-right-outline" => Some(icons::CLIPBOARD_ARROW_RIGHT_OUTLINE),
        "square-off" => Some(icons::SQUARE_OFF),
        "bus-stop-covered" => Some(icons::BUS_STOP_COVERED),
        "passport-alert" => Some(icons::PASSPORT_ALERT),
        "link-edit" => Some(icons::LINK_EDIT),
        "wifi-strength-lock-outline" => Some(icons::WIFI_STRENGTH_LOCK_OUTLINE),
        "arrow-top-right" => Some(icons::ARROW_TOP_RIGHT),
        "movie-open-cog-outline" => Some(icons::MOVIE_OPEN_COG_OUTLINE),
        "format-header-6" => Some(icons::FORMAT_HEADER_6),
        "skull-crossbones-outline" => Some(icons::SKULL_CROSSBONES_OUTLINE),
        "arrow-left-thin-circle-outline" => Some(icons::ARROW_LEFT_THIN_CIRCLE_OUTLINE),
        "ipod" => Some(icons::IPOD),
        "car-child-seat" => Some(icons::CAR_CHILD_SEAT),
        "dumbbell" => Some(icons::DUMBBELL),
        "hexagram" => Some(icons::HEXAGRAM),
        "washing-machine-alert" => Some(icons::WASHING_MACHINE_ALERT),
        "page-layout-sidebar-left" => Some(icons::PAGE_LAYOUT_SIDEBAR_LEFT),
        "garage-variant" => Some(icons::GARAGE_VARIANT),
        "shoe-ballet" => Some(icons::SHOE_BALLET),
        "dots-vertical-circle-outline" => Some(icons::DOTS_VERTICAL_CIRCLE_OUTLINE),
        "timer-stop-outline" => Some(icons::TIMER_STOP_OUTLINE),
        "account-multiple-plus" => Some(icons::ACCOUNT_MULTIPLE_PLUS),
        "bank-minus" => Some(icons::BANK_MINUS),
        "bicycle-penny-farthing" => Some(icons::BICYCLE_PENNY_FARTHING),
        "bag-suitcase-off-outline" => Some(icons::BAG_SUITCASE_OFF_OUTLINE),
        "clipboard-remove-outline" => Some(icons::CLIPBOARD_REMOVE_OUTLINE),
        "arrow-left-bold-outline" => Some(icons::ARROW_LEFT_BOLD_OUTLINE),
        "printer-outline" => Some(icons::PRINTER_OUTLINE),
        "image-search-outline" => Some(icons::IMAGE_SEARCH_OUTLINE),
        "account-network-off" => Some(icons::ACCOUNT_NETWORK_OFF),
        "car-search-outline" => Some(icons::CAR_SEARCH_OUTLINE),
        "home-sound-in-outline" => Some(icons::HOME_SOUND_IN_OUTLINE),
        "clock-check-outline" => Some(icons::CLOCK_CHECK_OUTLINE),
        "floor-lamp-dual" => Some(icons::FLOOR_LAMP_DUAL),
        "copyleft" => Some(icons::COPYLEFT),
        "standard-definition" => Some(icons::STANDARD_DEFINITION),
        "repeat-once" => Some(icons::REPEAT_ONCE),
        "swap-horizontal-hidden" => Some(icons::SWAP_HORIZONTAL_HIDDEN),
        "clippy" => Some(icons::CLIPPY),
        "checkbox-intermediate" => Some(icons::CHECKBOX_INTERMEDIATE),
        "shield-outline" => Some(icons::SHIELD_OUTLINE),
        "home-floor-negative-1" => Some(icons::HOME_FLOOR_NEGATIVE_1),
        "face-mask" => Some(icons::FACE_MASK),
        "apple-keyboard-control" => Some(icons::APPLE_KEYBOARD_CONTROL),
        "table-column-width" => Some(icons::TABLE_COLUMN_WIDTH),
        "relation-zero-or-many-to-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE),
        "invoice-import" => Some(icons::INVOICE_IMPORT),
        "comment-arrow-left" => Some(icons::COMMENT_ARROW_LEFT),
        "train-car-passenger-variant" => Some(icons::TRAIN_CAR_PASSENGER_VARIANT),
        "file-document-edit-outline" => Some(icons::FILE_DOCUMENT_EDIT_OUTLINE),
        "shape-plus" => Some(icons::SHAPE_PLUS),
        "numeric-6" => Some(icons::NUMERIC_6),
        "reminder" => Some(icons::REMINDER),
        "chili-mild" => Some(icons::CHILI_MILD),
        "alien" => Some(icons::ALIEN),
        "bookmark-plus" => Some(icons::BOOKMARK_PLUS),
        "dice-2" => Some(icons::DICE_2),
        "submarine" => Some(icons::SUBMARINE),
        "lock-open-remove" => Some(icons::LOCK_OPEN_REMOVE),
        "home-variant-outline" => Some(icons::HOME_VARIANT_OUTLINE),
        "bank-plus" => Some(icons::BANK_PLUS),
        "folder-refresh-outline" => Some(icons::FOLDER_REFRESH_OUTLINE),
        "rocket-launch" => Some(icons::ROCKET_LAUNCH),
        "play" => Some(icons::PLAY),
        "cookie-refresh" => Some(icons::COOKIE_REFRESH),
        "monitor-account" => Some(icons::MONITOR_ACCOUNT),
        "relation-zero-or-one-to-one-or-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE_OR_MANY),
        "battery-charging-wireless-50" => Some(icons::BATTERY_CHARGING_WIRELESS_50),
        "antenna" => Some(icons::ANTENNA),
        "send-variant" => Some(icons::SEND_VARIANT),
        "table-eye-off" => Some(icons::TABLE_EYE_OFF),
        "new-box" => Some(icons::NEW_BOX),
        "alpha-g-box-outline" => Some(icons::ALPHA_G_BOX_OUTLINE),
        "content-paste" => Some(icons::CONTENT_PASTE),
        #[allow(deprecated)]
        "language-css3" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-css3' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CSS3)
        }
        "table-arrow-down" => Some(icons::TABLE_ARROW_DOWN),
        "bone-off" => Some(icons::BONE_OFF),
        "led-variant-on" => Some(icons::LED_VARIANT_ON),
        "clock-time-three-outline" => Some(icons::CLOCK_TIME_THREE_OUTLINE),
        "arrow-left-right-bold" => Some(icons::ARROW_LEFT_RIGHT_BOLD),
        "movie-search" => Some(icons::MOVIE_SEARCH),
        "file-powerpoint-outline" => Some(icons::FILE_POWERPOINT_OUTLINE),
        "calendar-lock" => Some(icons::CALENDAR_LOCK),
        "cookie-clock" => Some(icons::COOKIE_CLOCK),
        "format-align-justify" => Some(icons::FORMAT_ALIGN_JUSTIFY),
        "alpha-d-box-outline" => Some(icons::ALPHA_D_BOX_OUTLINE),
        "package-variant-plus" => Some(icons::PACKAGE_VARIANT_PLUS),
        "puzzle-heart-outline" => Some(icons::PUZZLE_HEART_OUTLINE),
        "phone-bluetooth" => Some(icons::PHONE_BLUETOOTH),
        "hail" => Some(icons::HAIL),
        "flag-remove" => Some(icons::FLAG_REMOVE),
        "file-question" => Some(icons::FILE_QUESTION),
        "garage-alert" => Some(icons::GARAGE_ALERT),
        "poker-chip" => Some(icons::POKER_CHIP),
        "beaker-outline" => Some(icons::BEAKER_OUTLINE),
        "printer-pos-refresh-outline" => Some(icons::PRINTER_POS_REFRESH_OUTLINE),
        "car-brake-alert" => Some(icons::CAR_BRAKE_ALERT),
        "image-filter-frames" => Some(icons::IMAGE_FILTER_FRAMES),
        "alpha-i" => Some(icons::ALPHA_I),
        "comment-remove" => Some(icons::COMMENT_REMOVE),
        "tea" => Some(icons::TEA),
        "border-bottom" => Some(icons::BORDER_BOTTOM),
        "sail-boat-sink" => Some(icons::SAIL_BOAT_SINK),
        "volume-low" => Some(icons::VOLUME_LOW),
        "cable-data" => Some(icons::CABLE_DATA),
        "locker-multiple" => Some(icons::LOCKER_MULTIPLE),
        "account-lock-outline" => Some(icons::ACCOUNT_LOCK_OUTLINE),
        "forward" => Some(icons::FORWARD),
        "account-off" => Some(icons::ACCOUNT_OFF),
        "moped-electric-outline" => Some(icons::MOPED_ELECTRIC_OUTLINE),
        "garage-variant-lock" => Some(icons::GARAGE_VARIANT_LOCK),
        "image-edit-outline" => Some(icons::IMAGE_EDIT_OUTLINE),
        _ => None,
    }
}
