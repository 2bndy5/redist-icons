// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_25(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "pier" => Some(icons::PIER),
        "timer-cog-outline" => Some(icons::TIMER_COG_OUTLINE),
        "network" => Some(icons::NETWORK),
        "information-off-outline" => Some(icons::INFORMATION_OFF_OUTLINE),
        "alpha-q" => Some(icons::ALPHA_Q),
        #[allow(deprecated)]
        "dolby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dolby' is deprecated.").print(py);
            }
            Some(icons::DOLBY)
        }
        "volume-low" => Some(icons::VOLUME_LOW),
        "play-box-lock" => Some(icons::PLAY_BOX_LOCK),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-full" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-full' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_FULL)
        }
        "file-excel-outline" => Some(icons::FILE_EXCEL_OUTLINE),
        "car-wireless" => Some(icons::CAR_WIRELESS),
        "liquor" => Some(icons::LIQUOR),
        "air-humidifier-off" => Some(icons::AIR_HUMIDIFIER_OFF),
        "battery-minus" => Some(icons::BATTERY_MINUS),
        "timer-sand-complete" => Some(icons::TIMER_SAND_COMPLETE),
        "lock-open-check" => Some(icons::LOCK_OPEN_CHECK),
        "camera-lock-open" => Some(icons::CAMERA_LOCK_OPEN),
        "passport-alert" => Some(icons::PASSPORT_ALERT),
        "file-edit-outline" => Some(icons::FILE_EDIT_OUTLINE),
        "notification-clear-all" => Some(icons::NOTIFICATION_CLEAR_ALL),
        "animation-play" => Some(icons::ANIMATION_PLAY),
        "table-headers-eye-off" => Some(icons::TABLE_HEADERS_EYE_OFF),
        "sprinkler" => Some(icons::SPRINKLER),
        "check-underline-circle-outline" => Some(icons::CHECK_UNDERLINE_CIRCLE_OUTLINE),
        "delete-circle-outline" => Some(icons::DELETE_CIRCLE_OUTLINE),
        "helicopter" => Some(icons::HELICOPTER),
        "calendar-range-outline" => Some(icons::CALENDAR_RANGE_OUTLINE),
        "fuel" => Some(icons::FUEL),
        "home-group-minus" => Some(icons::HOME_GROUP_MINUS),
        "badge-account-alert-outline" => Some(icons::BADGE_ACCOUNT_ALERT_OUTLINE),
        "relation-only-one-to-many" => Some(icons::RELATION_ONLY_ONE_TO_MANY),
        "shoe-formal" => Some(icons::SHOE_FORMAL),
        "vector-polyline-minus" => Some(icons::VECTOR_POLYLINE_MINUS),
        "star-check" => Some(icons::STAR_CHECK),
        "account-alert-outline" => Some(icons::ACCOUNT_ALERT_OUTLINE),
        "tray-arrow-up" => Some(icons::TRAY_ARROW_UP),
        "developer-board" => Some(icons::DEVELOPER_BOARD),
        "home-lock" => Some(icons::HOME_LOCK),
        "image-outline" => Some(icons::IMAGE_OUTLINE),
        "music-note-quarter-dotted" => Some(icons::MUSIC_NOTE_QUARTER_DOTTED),
        #[allow(deprecated)]
        "microsoft-office" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-office' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OFFICE)
        }
        "hub" => Some(icons::HUB),
        "flash-auto" => Some(icons::FLASH_AUTO),
        "nature" => Some(icons::NATURE),
        "wifi-strength-3" => Some(icons::WIFI_STRENGTH_3),
        "invoice-check" => Some(icons::INVOICE_CHECK),
        "gradient-horizontal" => Some(icons::GRADIENT_HORIZONTAL),
        "account-network-outline" => Some(icons::ACCOUNT_NETWORK_OUTLINE),
        "button-cursor" => Some(icons::BUTTON_CURSOR),
        "bag-suitcase-outline" => Some(icons::BAG_SUITCASE_OUTLINE),
        "message-reply-outline" => Some(icons::MESSAGE_REPLY_OUTLINE),
        "movie-star" => Some(icons::MOVIE_STAR),
        "filter-variant-plus" => Some(icons::FILTER_VARIANT_PLUS),
        "currency-sign" => Some(icons::CURRENCY_SIGN),
        "marker-cancel" => Some(icons::MARKER_CANCEL),
        "link-variant-minus" => Some(icons::LINK_VARIANT_MINUS),
        "compass-off-outline" => Some(icons::COMPASS_OFF_OUTLINE),
        "zip-box" => Some(icons::ZIP_BOX),
        "battery-minus-outline" => Some(icons::BATTERY_MINUS_OUTLINE),
        "nuke" => Some(icons::NUKE),
        "chess-pawn" => Some(icons::CHESS_PAWN),
        "arrow-down-bold-box-outline" => Some(icons::ARROW_DOWN_BOLD_BOX_OUTLINE),
        "cog-off" => Some(icons::COG_OFF),
        "pin-off" => Some(icons::PIN_OFF),
        "account-tag" => Some(icons::ACCOUNT_TAG),
        "account-convert-outline" => Some(icons::ACCOUNT_CONVERT_OUTLINE),
        "image-size-select-actual" => Some(icons::IMAGE_SIZE_SELECT_ACTUAL),
        #[allow(deprecated)]
        "language-ruby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUBY)
        }
        "archive-refresh-outline" => Some(icons::ARCHIVE_REFRESH_OUTLINE),
        "library-shelves" => Some(icons::LIBRARY_SHELVES),
        "location-enter" => Some(icons::LOCATION_ENTER),
        "home-lock-open" => Some(icons::HOME_LOCK_OPEN),
        "phone-log" => Some(icons::PHONE_LOG),
        "power-socket" => Some(icons::POWER_SOCKET),
        "cookie-check" => Some(icons::COOKIE_CHECK),
        "hammer" => Some(icons::HAMMER),
        "gauge-full" => Some(icons::GAUGE_FULL),
        "skip-backward-outline" => Some(icons::SKIP_BACKWARD_OUTLINE),
        "call-missed" => Some(icons::CALL_MISSED),
        "head-flash-outline" => Some(icons::HEAD_FLASH_OUTLINE),
        "wifi-strength-outline" => Some(icons::WIFI_STRENGTH_OUTLINE),
        "cookie-lock-outline" => Some(icons::COOKIE_LOCK_OUTLINE),
        "arrow-bottom-left-thin" => Some(icons::ARROW_BOTTOM_LEFT_THIN),
        "tray-minus" => Some(icons::TRAY_MINUS),
        "power-socket-eu" => Some(icons::POWER_SOCKET_EU),
        "seat-recline-extra" => Some(icons::SEAT_RECLINE_EXTRA),
        "baby" => Some(icons::BABY),
        "ph" => Some(icons::PH),
        "briefcase-clock-outline" => Some(icons::BRIEFCASE_CLOCK_OUTLINE),
        "flash" => Some(icons::FLASH),
        "table-split-cell" => Some(icons::TABLE_SPLIT_CELL),
        "numeric-10-box-multiple-outline" => Some(icons::NUMERIC_10_BOX_MULTIPLE_OUTLINE),
        "pail" => Some(icons::PAIL),
        "clipboard-text-search-outline" => Some(icons::CLIPBOARD_TEXT_SEARCH_OUTLINE),
        "calendar-week-begin" => Some(icons::CALENDAR_WEEK_BEGIN),
        "pipe-disconnected" => Some(icons::PIPE_DISCONNECTED),
        "vector-selection" => Some(icons::VECTOR_SELECTION),
        "smart-card-reader" => Some(icons::SMART_CARD_READER),
        "calendar-heart-outline" => Some(icons::CALENDAR_HEART_OUTLINE),
        "calendar-import-outline" => Some(icons::CALENDAR_IMPORT_OUTLINE),
        "approximately-equal" => Some(icons::APPROXIMATELY_EQUAL),
        "home-modern" => Some(icons::HOME_MODERN),
        "dharmachakra" => Some(icons::DHARMACHAKRA),
        "emoticon-plus" => Some(icons::EMOTICON_PLUS),
        "notebook-heart" => Some(icons::NOTEBOOK_HEART),
        "send-check-outline" => Some(icons::SEND_CHECK_OUTLINE),
        "hand-pointing-right" => Some(icons::HAND_POINTING_RIGHT),
        "checkbox-multiple-marked-outline" => Some(icons::CHECKBOX_MULTIPLE_MARKED_OUTLINE),
        "toy-brick-minus" => Some(icons::TOY_BRICK_MINUS),
        "undo-variant" => Some(icons::UNDO_VARIANT),
        "basket-minus" => Some(icons::BASKET_MINUS),
        "microphone-question" => Some(icons::MICROPHONE_QUESTION),
        "pause-box-outline" => Some(icons::PAUSE_BOX_OUTLINE),
        "camera-marker-outline" => Some(icons::CAMERA_MARKER_OUTLINE),
        "cached" => Some(icons::CACHED),
        "archive-marker" => Some(icons::ARCHIVE_MARKER),
        "bluetooth-off" => Some(icons::BLUETOOTH_OFF),
        "image-off" => Some(icons::IMAGE_OFF),
        "longitude" => Some(icons::LONGITUDE),
        "content-save-settings" => Some(icons::CONTENT_SAVE_SETTINGS),
        "format-italic" => Some(icons::FORMAT_ITALIC),
        "battery-sync-outline" => Some(icons::BATTERY_SYNC_OUTLINE),
        "sim" => Some(icons::SIM),
        "ticket-confirmation" => Some(icons::TICKET_CONFIRMATION),
        "ice-cream" => Some(icons::ICE_CREAM),
        "key-outline" => Some(icons::KEY_OUTLINE),
        "kettle-steam-outline" => Some(icons::KETTLE_STEAM_OUTLINE),
        "web-off" => Some(icons::WEB_OFF),
        "battery-arrow-up" => Some(icons::BATTERY_ARROW_UP),
        "arrow-left-right-bold-outline" => Some(icons::ARROW_LEFT_RIGHT_BOLD_OUTLINE),
        "archive-lock-open" => Some(icons::ARCHIVE_LOCK_OPEN),
        "bee" => Some(icons::BEE),
        "elevator-down" => Some(icons::ELEVATOR_DOWN),
        "eye-arrow-right-outline" => Some(icons::EYE_ARROW_RIGHT_OUTLINE),
        "briefcase-check-outline" => Some(icons::BRIEFCASE_CHECK_OUTLINE),
        "moon-first-quarter" => Some(icons::MOON_FIRST_QUARTER),
        "approximately-equal-box" => Some(icons::APPROXIMATELY_EQUAL_BOX),
        "numeric-9" => Some(icons::NUMERIC_9),
        "tag-arrow-right" => Some(icons::TAG_ARROW_RIGHT),
        "plus-circle-outline" => Some(icons::PLUS_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "humble-bundle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'humble-bundle' is deprecated.").print(py);
            }
            Some(icons::HUMBLE_BUNDLE)
        }
        "calendar-badge" => Some(icons::CALENDAR_BADGE),
        "alpha-x" => Some(icons::ALPHA_X),
        "package-variant" => Some(icons::PACKAGE_VARIANT),
        "account-clock" => Some(icons::ACCOUNT_CLOCK),
        "window-shutter-alert" => Some(icons::WINDOW_SHUTTER_ALERT),
        "minus-network-outline" => Some(icons::MINUS_NETWORK_OUTLINE),
        #[allow(deprecated)]
        "waze" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'waze' is deprecated.").print(py);
            }
            Some(icons::WAZE)
        }
        "motion-sensor-off" => Some(icons::MOTION_SENSOR_OFF),
        "camera-enhance-outline" => Some(icons::CAMERA_ENHANCE_OUTLINE),
        "raw-off" => Some(icons::RAW_OFF),
        "comment-flash-outline" => Some(icons::COMMENT_FLASH_OUTLINE),
        "numeric-5-box-outline" => Some(icons::NUMERIC_5_BOX_OUTLINE),
        "ipod" => Some(icons::IPOD),
        "arrow-top-left-bold-box" => Some(icons::ARROW_TOP_LEFT_BOLD_BOX),
        "bell-cog-outline" => Some(icons::BELL_COG_OUTLINE),
        "server-off" => Some(icons::SERVER_OFF),
        "crown" => Some(icons::CROWN),
        "chart-box" => Some(icons::CHART_BOX),
        "account-switch-outline" => Some(icons::ACCOUNT_SWITCH_OUTLINE),
        "fuel-cell" => Some(icons::FUEL_CELL),
        "link-circle" => Some(icons::LINK_CIRCLE),
        "pi" => Some(icons::PI),
        "ghost-outline" => Some(icons::GHOST_OUTLINE),
        "flower-pollen-outline" => Some(icons::FLOWER_POLLEN_OUTLINE),
        "close-box-multiple-outline" => Some(icons::CLOSE_BOX_MULTIPLE_OUTLINE),
        "car-light-fog" => Some(icons::CAR_LIGHT_FOG),
        "tablet-dashboard" => Some(icons::TABLET_DASHBOARD),
        "storefront-check" => Some(icons::STOREFRONT_CHECK),
        "wheelchair-accessibility" => Some(icons::WHEELCHAIR_ACCESSIBILITY),
        "camera-metering-spot" => Some(icons::CAMERA_METERING_SPOT),
        "distribute-horizontal-center" => Some(icons::DISTRIBUTE_HORIZONTAL_CENTER),
        "timer-sand-empty" => Some(icons::TIMER_SAND_EMPTY),
        "book" => Some(icons::BOOK),
        "clock-time-twelve" => Some(icons::CLOCK_TIME_TWELVE),
        "text-long" => Some(icons::TEXT_LONG),
        "air-conditioner" => Some(icons::AIR_CONDITIONER),
        "key-alert" => Some(icons::KEY_ALERT),
        "code-not-equal" => Some(icons::CODE_NOT_EQUAL),
        "temple-buddhist" => Some(icons::TEMPLE_BUDDHIST),
        "billiards" => Some(icons::BILLIARDS),
        "checkbox-multiple-blank" => Some(icons::CHECKBOX_MULTIPLE_BLANK),
        "emoticon-tongue-outline" => Some(icons::EMOTICON_TONGUE_OUTLINE),
        "cards-diamond-outline" => Some(icons::CARDS_DIAMOND_OUTLINE),
        "smoke-detector-alert" => Some(icons::SMOKE_DETECTOR_ALERT),
        "gauge" => Some(icons::GAUGE),
        "monitor-account" => Some(icons::MONITOR_ACCOUNT),
        "battery-charging-100" => Some(icons::BATTERY_CHARGING_100),
        "alert-remove" => Some(icons::ALERT_REMOVE),
        "text-box-search-outline" => Some(icons::TEXT_BOX_SEARCH_OUTLINE),
        "van-utility" => Some(icons::VAN_UTILITY),
        "keyboard-f5" => Some(icons::KEYBOARD_F5),
        "bus-side" => Some(icons::BUS_SIDE),
        "numeric-8" => Some(icons::NUMERIC_8),
        "calendar-end-outline" => Some(icons::CALENDAR_END_OUTLINE),
        "fish" => Some(icons::FISH),
        "gender-male-female-variant" => Some(icons::GENDER_MALE_FEMALE_VARIANT),
        "credit-card-wireless-outline" => Some(icons::CREDIT_CARD_WIRELESS_OUTLINE),
        "invoice-arrow-right-outline" => Some(icons::INVOICE_ARROW_RIGHT_OUTLINE),
        "sickle" => Some(icons::SICKLE),
        _ => None,
    }
}
