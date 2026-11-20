// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_10(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "glasses" => Some(icons::GLASSES),
        "brush-variant" => Some(icons::BRUSH_VARIANT),
        "chart-donut-variant" => Some(icons::CHART_DONUT_VARIANT),
        "arrow-left-bold-circle" => Some(icons::ARROW_LEFT_BOLD_CIRCLE),
        "thermometer-auto" => Some(icons::THERMOMETER_AUTO),
        "eye-outline" => Some(icons::EYE_OUTLINE),
        "source-fork" => Some(icons::SOURCE_FORK),
        "archive-music" => Some(icons::ARCHIVE_MUSIC),
        "panorama" => Some(icons::PANORAMA),
        "dome-light" => Some(icons::DOME_LIGHT),
        #[allow(deprecated)]
        "folder-google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'folder-google-drive' is deprecated.")
                    .print(py);
            }
            Some(icons::FOLDER_GOOGLE_DRIVE)
        }
        "weather-hazy" => Some(icons::WEATHER_HAZY),
        "dots-horizontal-circle-outline" => Some(icons::DOTS_HORIZONTAL_CIRCLE_OUTLINE),
        "image-auto-adjust" => Some(icons::IMAGE_AUTO_ADJUST),
        "reflect-vertical" => Some(icons::REFLECT_VERTICAL),
        "border-bottom-variant" => Some(icons::BORDER_BOTTOM_VARIANT),
        #[allow(deprecated)]
        "hulu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'hulu' is deprecated.").print(py);
            }
            Some(icons::HULU)
        }
        "credit-card" => Some(icons::CREDIT_CARD),
        #[allow(deprecated)]
        "google-ads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-ads' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_ADS)
        }
        "tag-hidden" => Some(icons::TAG_HIDDEN),
        "human-white-cane" => Some(icons::HUMAN_WHITE_CANE),
        "tag-edit" => Some(icons::TAG_EDIT),
        "book-lock-outline" => Some(icons::BOOK_LOCK_OUTLINE),
        "phone-bluetooth-outline" => Some(icons::PHONE_BLUETOOTH_OUTLINE),
        "credit-card-settings" => Some(icons::CREDIT_CARD_SETTINGS),
        "decimal-decrease" => Some(icons::DECIMAL_DECREASE),
        "relation-one-or-many-to-only-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONLY_ONE),
        "send-variant-clock" => Some(icons::SEND_VARIANT_CLOCK),
        "numeric-7-circle-outline" => Some(icons::NUMERIC_7_CIRCLE_OUTLINE),
        "cast" => Some(icons::CAST),
        "console-line" => Some(icons::CONSOLE_LINE),
        "unicycle" => Some(icons::UNICYCLE),
        "atom-variant" => Some(icons::ATOM_VARIANT),
        "database-plus" => Some(icons::DATABASE_PLUS),
        "flash-auto" => Some(icons::FLASH_AUTO),
        #[allow(deprecated)]
        "facebook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook' is deprecated.").print(py);
            }
            Some(icons::FACEBOOK)
        }
        "arrow-up-left-bold" => Some(icons::ARROW_UP_LEFT_BOLD),
        "credit-card-wireless" => Some(icons::CREDIT_CARD_WIRELESS),
        "file-check-outline" => Some(icons::FILE_CHECK_OUTLINE),
        "middleware-outline" => Some(icons::MIDDLEWARE_OUTLINE),
        "folder-arrow-left-right-outline" => Some(icons::FOLDER_ARROW_LEFT_RIGHT_OUTLINE),
        "invoice-multiple-outline" => Some(icons::INVOICE_MULTIPLE_OUTLINE),
        "basket-plus-outline" => Some(icons::BASKET_PLUS_OUTLINE),
        "book-minus-multiple" => Some(icons::BOOK_MINUS_MULTIPLE),
        "calendar-text" => Some(icons::CALENDAR_TEXT),
        "filter-plus-outline" => Some(icons::FILTER_PLUS_OUTLINE),
        "numeric-1-box" => Some(icons::NUMERIC_1_BOX),
        "robot-love" => Some(icons::ROBOT_LOVE),
        "table-large-plus" => Some(icons::TABLE_LARGE_PLUS),
        "propane-tank-outline" => Some(icons::PROPANE_TANK_OUTLINE),
        "flag-minus-outline" => Some(icons::FLAG_MINUS_OUTLINE),
        "motion-pause-outline" => Some(icons::MOTION_PAUSE_OUTLINE),
        "truck-minus" => Some(icons::TRUCK_MINUS),
        "crop-portrait" => Some(icons::CROP_PORTRAIT),
        "message-reply-text" => Some(icons::MESSAGE_REPLY_TEXT),
        "monitor" => Some(icons::MONITOR),
        "leaf-circle" => Some(icons::LEAF_CIRCLE),
        "network-strength-1" => Some(icons::NETWORK_STRENGTH_1),
        "camera-metering-partial" => Some(icons::CAMERA_METERING_PARTIAL),
        "layers-triple" => Some(icons::LAYERS_TRIPLE),
        "account-details-outline" => Some(icons::ACCOUNT_DETAILS_OUTLINE),
        "book-alphabet" => Some(icons::BOOK_ALPHABET),
        "magnify-close" => Some(icons::MAGNIFY_CLOSE),
        "book-remove-outline" => Some(icons::BOOK_REMOVE_OUTLINE),
        "book-education" => Some(icons::BOOK_EDUCATION),
        "rowing" => Some(icons::ROWING),
        "human-capacity-increase" => Some(icons::HUMAN_CAPACITY_INCREASE),
        "approximately-equal-box" => Some(icons::APPROXIMATELY_EQUAL_BOX),
        "numeric-0-box" => Some(icons::NUMERIC_0_BOX),
        "file-account-outline" => Some(icons::FILE_ACCOUNT_OUTLINE),
        "calendar-multiple" => Some(icons::CALENDAR_MULTIPLE),
        "bluetooth" => Some(icons::BLUETOOTH),
        "receipt-text-send" => Some(icons::RECEIPT_TEXT_SEND),
        "beaker-check-outline" => Some(icons::BEAKER_CHECK_OUTLINE),
        "blur-off" => Some(icons::BLUR_OFF),
        "message-check" => Some(icons::MESSAGE_CHECK),
        "video-image" => Some(icons::VIDEO_IMAGE),
        "emoticon-cool" => Some(icons::EMOTICON_COOL),
        "ticket-outline" => Some(icons::TICKET_OUTLINE),
        "compare-remove" => Some(icons::COMPARE_REMOVE),
        "cookie-refresh" => Some(icons::COOKIE_REFRESH),
        "camera-lock-open" => Some(icons::CAMERA_LOCK_OPEN),
        "account-reactivate-outline" => Some(icons::ACCOUNT_REACTIVATE_OUTLINE),
        "package-variant-remove" => Some(icons::PACKAGE_VARIANT_REMOVE),
        "hamburger-remove" => Some(icons::HAMBURGER_REMOVE),
        "elevator-up" => Some(icons::ELEVATOR_UP),
        "glass-mug-off" => Some(icons::GLASS_MUG_OFF),
        "brightness-2" => Some(icons::BRIGHTNESS_2),
        "controller-off" => Some(icons::CONTROLLER_OFF),
        "close-network" => Some(icons::CLOSE_NETWORK),
        "printer-pos-cog-outline" => Some(icons::PRINTER_POS_COG_OUTLINE),
        "folder-music" => Some(icons::FOLDER_MUSIC),
        "order-bool-ascending" => Some(icons::ORDER_BOOL_ASCENDING),
        "battery-off-outline" => Some(icons::BATTERY_OFF_OUTLINE),
        "office-building-plus" => Some(icons::OFFICE_BUILDING_PLUS),
        "dice-d12" => Some(icons::DICE_D12),
        "cloud-check-variant-outline" => Some(icons::CLOUD_CHECK_VARIANT_OUTLINE),
        "format-italic" => Some(icons::FORMAT_ITALIC),
        "view-grid-plus" => Some(icons::VIEW_GRID_PLUS),
        "shape-plus" => Some(icons::SHAPE_PLUS),
        "card-account-phone" => Some(icons::CARD_ACCOUNT_PHONE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-off" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-off' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_OFF)
        }
        "television-speaker" => Some(icons::TELEVISION_SPEAKER),
        "circle-box" => Some(icons::CIRCLE_BOX),
        "hand-okay" => Some(icons::HAND_OKAY),
        "timeline" => Some(icons::TIMELINE),
        "table-column" => Some(icons::TABLE_COLUMN),
        "arrow-right-bold-hexagon-outline" => Some(icons::ARROW_RIGHT_BOLD_HEXAGON_OUTLINE),
        "web-remove" => Some(icons::WEB_REMOVE),
        "dump-truck" => Some(icons::DUMP_TRUCK),
        "car-search" => Some(icons::CAR_SEARCH),
        "ampersand" => Some(icons::AMPERSAND),
        "lightbulb-auto" => Some(icons::LIGHTBULB_AUTO),
        "satellite" => Some(icons::SATELLITE),
        "cloud-alert" => Some(icons::CLOUD_ALERT),
        "hops" => Some(icons::HOPS),
        #[allow(deprecated)]
        "microsoft-azure" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE)
        }
        "comment-processing-outline" => Some(icons::COMMENT_PROCESSING_OUTLINE),
        "battery-positive" => Some(icons::BATTERY_POSITIVE),
        "bolt" => Some(icons::BOLT),
        "circle-outline" => Some(icons::CIRCLE_OUTLINE),
        "headphones-settings" => Some(icons::HEADPHONES_SETTINGS),
        "database-settings-outline" => Some(icons::DATABASE_SETTINGS_OUTLINE),
        "home" => Some(icons::HOME),
        "dock-bottom" => Some(icons::DOCK_BOTTOM),
        "table-cancel" => Some(icons::TABLE_CANCEL),
        "air-humidifier-off" => Some(icons::AIR_HUMIDIFIER_OFF),
        "briefcase-plus" => Some(icons::BRIEFCASE_PLUS),
        "link-off" => Some(icons::LINK_OFF),
        "file-remove-outline" => Some(icons::FILE_REMOVE_OUTLINE),
        "basket" => Some(icons::BASKET),
        "comment-flash-outline" => Some(icons::COMMENT_FLASH_OUTLINE),
        "alpha-y-box-outline" => Some(icons::ALPHA_Y_BOX_OUTLINE),
        "video-outline" => Some(icons::VIDEO_OUTLINE),
        "pail-off-outline" => Some(icons::PAIL_OFF_OUTLINE),
        "book-marker" => Some(icons::BOOK_MARKER),
        "atm" => Some(icons::ATM),
        "information-variant-box" => Some(icons::INFORMATION_VARIANT_BOX),
        "folder-refresh" => Some(icons::FOLDER_REFRESH),
        "weather-fog" => Some(icons::WEATHER_FOG),
        "format-text-wrapping-wrap" => Some(icons::FORMAT_TEXT_WRAPPING_WRAP),
        "format-align-center" => Some(icons::FORMAT_ALIGN_CENTER),
        "led-strip" => Some(icons::LED_STRIP),
        "cellphone-key" => Some(icons::CELLPHONE_KEY),
        "notebook-heart-outline" => Some(icons::NOTEBOOK_HEART_OUTLINE),
        "set-left" => Some(icons::SET_LEFT),
        "sticker-check-outline" => Some(icons::STICKER_CHECK_OUTLINE),
        "shield-lock-outline" => Some(icons::SHIELD_LOCK_OUTLINE),
        "shower" => Some(icons::SHOWER),
        "circle-half" => Some(icons::CIRCLE_HALF),
        "battery-charging-90" => Some(icons::BATTERY_CHARGING_90),
        "greenhouse" => Some(icons::GREENHOUSE),
        "molecule-co" => Some(icons::MOLECULE_CO),
        "tag-search" => Some(icons::TAG_SEARCH),
        "wrench-cog" => Some(icons::WRENCH_COG),
        "file-phone" => Some(icons::FILE_PHONE),
        "message-text-lock" => Some(icons::MESSAGE_TEXT_LOCK),
        "timer-star" => Some(icons::TIMER_STAR),
        "crown-circle-outline" => Some(icons::CROWN_CIRCLE_OUTLINE),
        "shape" => Some(icons::SHAPE),
        #[allow(deprecated)]
        "google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-drive' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_DRIVE)
        }
        "crop-square" => Some(icons::CROP_SQUARE),
        "numeric-2-circle" => Some(icons::NUMERIC_2_CIRCLE),
        "lightbulb-multiple-off" => Some(icons::LIGHTBULB_MULTIPLE_OFF),
        "city-switch" => Some(icons::CITY_SWITCH),
        "server-remove" => Some(icons::SERVER_REMOVE),
        "hand-clap" => Some(icons::HAND_CLAP),
        "hand-peace-variant" => Some(icons::HAND_PEACE_VARIANT),
        "diamond" => Some(icons::DIAMOND),
        "account-box-outline" => Some(icons::ACCOUNT_BOX_OUTLINE),
        "application-cog-outline" => Some(icons::APPLICATION_COG_OUTLINE),
        "printer-pos-plus" => Some(icons::PRINTER_POS_PLUS),
        "alphabet-aurebesh" => Some(icons::ALPHABET_AUREBESH),
        "fridge-variant-alert" => Some(icons::FRIDGE_VARIANT_ALERT),
        "fridge-top" => Some(icons::FRIDGE_TOP),
        "stop-circle-outline" => Some(icons::STOP_CIRCLE_OUTLINE),
        "medal" => Some(icons::MEDAL),
        "email-seal" => Some(icons::EMAIL_SEAL),
        "timer-play-outline" => Some(icons::TIMER_PLAY_OUTLINE),
        "toaster" => Some(icons::TOASTER),
        "clover" => Some(icons::CLOVER),
        "sim-alert-outline" => Some(icons::SIM_ALERT_OUTLINE),
        "printer-check" => Some(icons::PRINTER_CHECK),
        "roman-numeral-5" => Some(icons::ROMAN_NUMERAL_5),
        "dns" => Some(icons::DNS),
        "file-png-box" => Some(icons::FILE_PNG_BOX),
        "email-open" => Some(icons::EMAIL_OPEN),
        "pine-tree" => Some(icons::PINE_TREE),
        "clock-star-four-points-outline" => Some(icons::CLOCK_STAR_FOUR_POINTS_OUTLINE),
        "align-vertical-distribute" => Some(icons::ALIGN_VERTICAL_DISTRIBUTE),
        "flag" => Some(icons::FLAG),
        "account-filter" => Some(icons::ACCOUNT_FILTER),
        "quality-high" => Some(icons::QUALITY_HIGH),
        "floor-lamp-torchiere" => Some(icons::FLOOR_LAMP_TORCHIERE),
        "alpha-j-box" => Some(icons::ALPHA_J_BOX),
        "bulkhead-light" => Some(icons::BULKHEAD_LIGHT),
        "briefcase-account" => Some(icons::BRIEFCASE_ACCOUNT),
        "arrow-right-bold-outline" => Some(icons::ARROW_RIGHT_BOLD_OUTLINE),
        "gas-station-in-use" => Some(icons::GAS_STATION_IN_USE),
        "tshirt-crew" => Some(icons::TSHIRT_CREW),
        _ => None,
    }
}
