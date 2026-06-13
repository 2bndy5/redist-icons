// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_26(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
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
        "message-image" => Some(icons::MESSAGE_IMAGE),
        "server-network" => Some(icons::SERVER_NETWORK),
        "selection-off" => Some(icons::SELECTION_OFF),
        "axis-x-arrow-lock" => Some(icons::AXIS_X_ARROW_LOCK),
        "home-modern" => Some(icons::HOME_MODERN),
        "timeline-text-outline" => Some(icons::TIMELINE_TEXT_OUTLINE),
        "arrow-u-right-top" => Some(icons::ARROW_U_RIGHT_TOP),
        "exclamation-thick" => Some(icons::EXCLAMATION_THICK),
        "home-circle" => Some(icons::HOME_CIRCLE),
        "pound-box" => Some(icons::POUND_BOX),
        "engine" => Some(icons::ENGINE),
        "food-halal" => Some(icons::FOOD_HALAL),
        "format-list-bulleted-type" => Some(icons::FORMAT_LIST_BULLETED_TYPE),
        "monitor-cellphone-star" => Some(icons::MONITOR_CELLPHONE_STAR),
        "list-status" => Some(icons::LIST_STATUS),
        "invoice-edit" => Some(icons::INVOICE_EDIT),
        "web" => Some(icons::WEB),
        "invoice-text-edit-outline" => Some(icons::INVOICE_TEXT_EDIT_OUTLINE),
        "printer-pos-off" => Some(icons::PRINTER_POS_OFF),
        "close-circle" => Some(icons::CLOSE_CIRCLE),
        "screw-machine-flat-top" => Some(icons::SCREW_MACHINE_FLAT_TOP),
        "wifi-strength-alert-outline" => Some(icons::WIFI_STRENGTH_ALERT_OUTLINE),
        "content-save-plus-outline" => Some(icons::CONTENT_SAVE_PLUS_OUTLINE),
        "pen-plus" => Some(icons::PEN_PLUS),
        "keyboard-f12" => Some(icons::KEYBOARD_F12),
        "printer-off" => Some(icons::PRINTER_OFF),
        "flask-empty-minus" => Some(icons::FLASK_EMPTY_MINUS),
        "beaker" => Some(icons::BEAKER),
        "file-chart-outline" => Some(icons::FILE_CHART_OUTLINE),
        "wifi-strength-4" => Some(icons::WIFI_STRENGTH_4),
        "train-variant" => Some(icons::TRAIN_VARIANT),
        "qrcode-scan" => Some(icons::QRCODE_SCAN),
        "key" => Some(icons::KEY),
        "weather-windy-variant" => Some(icons::WEATHER_WINDY_VARIANT),
        "transit-connection" => Some(icons::TRANSIT_CONNECTION),
        "arrow-right-bold-box-outline" => Some(icons::ARROW_RIGHT_BOLD_BOX_OUTLINE),
        "calendar-multiple" => Some(icons::CALENDAR_MULTIPLE),
        "home-lightbulb-outline" => Some(icons::HOME_LIGHTBULB_OUTLINE),
        "star-box" => Some(icons::STAR_BOX),
        "cloud-remove" => Some(icons::CLOUD_REMOVE),
        "arrow-down-circle" => Some(icons::ARROW_DOWN_CIRCLE),
        "truck-minus" => Some(icons::TRUCK_MINUS),
        "alpha-s-circle-outline" => Some(icons::ALPHA_S_CIRCLE_OUTLINE),
        "peanut-off-outline" => Some(icons::PEANUT_OFF_OUTLINE),
        "phone-sync" => Some(icons::PHONE_SYNC),
        "attachment" => Some(icons::ATTACHMENT),
        "pillar" => Some(icons::PILLAR),
        "book-edit-outline" => Some(icons::BOOK_EDIT_OUTLINE),
        "pipe-valve" => Some(icons::PIPE_VALVE),
        "subdirectory-arrow-left" => Some(icons::SUBDIRECTORY_ARROW_LEFT),
        "test-tube-off" => Some(icons::TEST_TUBE_OFF),
        "printer-pos-wrench-outline" => Some(icons::PRINTER_POS_WRENCH_OUTLINE),
        #[allow(deprecated)]
        "google-nearby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-nearby' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_NEARBY)
        }
        "mustache" => Some(icons::MUSTACHE),
        "shape-square-plus" => Some(icons::SHAPE_SQUARE_PLUS),
        "microphone-variant-off" => Some(icons::MICROPHONE_VARIANT_OFF),
        "hand-heart" => Some(icons::HAND_HEART),
        "chess-king" => Some(icons::CHESS_KING),
        "mower-on" => Some(icons::MOWER_ON),
        #[allow(deprecated)]
        "microsoft-xbox-controller-menu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-menu' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_MENU)
        }
        "bag-personal-plus-outline" => Some(icons::BAG_PERSONAL_PLUS_OUTLINE),
        "account-box-multiple-outline" => Some(icons::ACCOUNT_BOX_MULTIPLE_OUTLINE),
        "alert-box" => Some(icons::ALERT_BOX),
        "car-windshield" => Some(icons::CAR_WINDSHIELD),
        "printer-pos-play-outline" => Some(icons::PRINTER_POS_PLAY_OUTLINE),
        "battery-alert" => Some(icons::BATTERY_ALERT),
        "cog-play" => Some(icons::COG_PLAY),
        "clock-time-four-outline" => Some(icons::CLOCK_TIME_FOUR_OUTLINE),
        "keyboard-f7" => Some(icons::KEYBOARD_F7),
        "map-check-outline" => Some(icons::MAP_CHECK_OUTLINE),
        "caps-lock" => Some(icons::CAPS_LOCK),
        "scoreboard-outline" => Some(icons::SCOREBOARD_OUTLINE),
        "skull-scan" => Some(icons::SKULL_SCAN),
        "earth-remove" => Some(icons::EARTH_REMOVE),
        "file-alert" => Some(icons::FILE_ALERT),
        "fountain-pen-tip" => Some(icons::FOUNTAIN_PEN_TIP),
        "alpha-l-box" => Some(icons::ALPHA_L_BOX),
        #[allow(deprecated)]
        "youtube-subscription" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-subscription' is deprecated.")
                    .print(py);
            }
            Some(icons::YOUTUBE_SUBSCRIPTION)
        }
        "buffet" => Some(icons::BUFFET),
        "calendar-alert-outline" => Some(icons::CALENDAR_ALERT_OUTLINE),
        "home-minus" => Some(icons::HOME_MINUS),
        "power" => Some(icons::POWER),
        "drawing-box" => Some(icons::DRAWING_BOX),
        "network-strength-2" => Some(icons::NETWORK_STRENGTH_2),
        #[allow(deprecated)]
        "instagram" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'instagram' is deprecated.").print(py);
            }
            Some(icons::INSTAGRAM)
        }
        "office-building-cog" => Some(icons::OFFICE_BUILDING_COG),
        "cloud" => Some(icons::CLOUD),
        "lock-open-outline" => Some(icons::LOCK_OPEN_OUTLINE),
        "car-off" => Some(icons::CAR_OFF),
        "heart-pulse" => Some(icons::HEART_PULSE),
        "folder-multiple-outline" => Some(icons::FOLDER_MULTIPLE_OUTLINE),
        "battery-charging-medium" => Some(icons::BATTERY_CHARGING_MEDIUM),
        "baby-bottle" => Some(icons::BABY_BOTTLE),
        "space-station" => Some(icons::SPACE_STATION),
        "translate" => Some(icons::TRANSLATE),
        "invoice-text-remove-outline" => Some(icons::INVOICE_TEXT_REMOVE_OUTLINE),
        "keyboard-f9" => Some(icons::KEYBOARD_F9),
        "code-less-than-or-equal" => Some(icons::CODE_LESS_THAN_OR_EQUAL),
        "umbrella-beach-outline" => Some(icons::UMBRELLA_BEACH_OUTLINE),
        "network-strength-4" => Some(icons::NETWORK_STRENGTH_4),
        "bus-alert" => Some(icons::BUS_ALERT),
        "call-missed" => Some(icons::CALL_MISSED),
        "ornament-variant" => Some(icons::ORNAMENT_VARIANT),
        "home-silo" => Some(icons::HOME_SILO),
        "skip-backward" => Some(icons::SKIP_BACKWARD),
        "zodiac-scorpio" => Some(icons::ZODIAC_SCORPIO),
        "pool-thermometer" => Some(icons::POOL_THERMOMETER),
        "alpha-h-box-outline" => Some(icons::ALPHA_H_BOX_OUTLINE),
        "curling" => Some(icons::CURLING),
        "phone-forward" => Some(icons::PHONE_FORWARD),
        "eraser-variant" => Some(icons::ERASER_VARIANT),
        "wallpaper" => Some(icons::WALLPAPER),
        "palette-swatch-variant" => Some(icons::PALETTE_SWATCH_VARIANT),
        #[allow(deprecated)]
        "jira" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jira' is deprecated.").print(py);
            }
            Some(icons::JIRA)
        }
        "map-marker-right-outline" => Some(icons::MAP_MARKER_RIGHT_OUTLINE),
        "propane-tank-outline" => Some(icons::PROPANE_TANK_OUTLINE),
        #[allow(deprecated)]
        "microsoft-office" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-office' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OFFICE)
        }
        "pause-octagon-outline" => Some(icons::PAUSE_OCTAGON_OUTLINE),
        "arrow-up-bold-hexagon-outline" => Some(icons::ARROW_UP_BOLD_HEXAGON_OUTLINE),
        "battery-charging-wireless-60" => Some(icons::BATTERY_CHARGING_WIRELESS_60),
        "fridge-variant-alert-outline" => Some(icons::FRIDGE_VARIANT_ALERT_OUTLINE),
        "plus-lock" => Some(icons::PLUS_LOCK),
        "car-search" => Some(icons::CAR_SEARCH),
        "delete-off-outline" => Some(icons::DELETE_OFF_OUTLINE),
        "emoticon-frown-outline" => Some(icons::EMOTICON_FROWN_OUTLINE),
        "alphabet-tengwar" => Some(icons::ALPHABET_TENGWAR),
        #[allow(deprecated)]
        "origin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'origin' is deprecated.").print(py);
            }
            Some(icons::ORIGIN)
        }
        "cookie-cog" => Some(icons::COOKIE_COG),
        "book-refresh-outline" => Some(icons::BOOK_REFRESH_OUTLINE),
        "television-pause" => Some(icons::TELEVISION_PAUSE),
        "export" => Some(icons::EXPORT),
        "thermostat" => Some(icons::THERMOSTAT),
        "basket-plus-outline" => Some(icons::BASKET_PLUS_OUTLINE),
        "share" => Some(icons::SHARE),
        "calendar-arrow-right" => Some(icons::CALENDAR_ARROW_RIGHT),
        #[allow(deprecated)]
        "ethereum" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ethereum' is deprecated.").print(py);
            }
            Some(icons::ETHEREUM)
        }
        #[allow(deprecated)]
        "wikipedia" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wikipedia' is deprecated.").print(py);
            }
            Some(icons::WIKIPEDIA)
        }
        "airplane-remove" => Some(icons::AIRPLANE_REMOVE),
        "relation-one-to-one-or-many" => Some(icons::RELATION_ONE_TO_ONE_OR_MANY),
        "shaker-outline" => Some(icons::SHAKER_OUTLINE),
        "invoice-text-arrow-right" => Some(icons::INVOICE_TEXT_ARROW_RIGHT),
        "zodiac-virgo" => Some(icons::ZODIAC_VIRGO),
        "human-male-female-child" => Some(icons::HUMAN_MALE_FEMALE_CHILD),
        "keyboard-f5" => Some(icons::KEYBOARD_F5),
        "briefcase-plus" => Some(icons::BRIEFCASE_PLUS),
        "city-switch" => Some(icons::CITY_SWITCH),
        "console-line" => Some(icons::CONSOLE_LINE),
        "elevator-down" => Some(icons::ELEVATOR_DOWN),
        "account-convert" => Some(icons::ACCOUNT_CONVERT),
        "sprout-outline" => Some(icons::SPROUT_OUTLINE),
        "brightness-2" => Some(icons::BRIGHTNESS_2),
        "account-lock" => Some(icons::ACCOUNT_LOCK),
        "chart-line-stacked" => Some(icons::CHART_LINE_STACKED),
        "content-save-off" => Some(icons::CONTENT_SAVE_OFF),
        "alpha-i-box-outline" => Some(icons::ALPHA_I_BOX_OUTLINE),
        "account-box-plus-outline" => Some(icons::ACCOUNT_BOX_PLUS_OUTLINE),
        "briefcase-variant" => Some(icons::BRIEFCASE_VARIANT),
        "bookmark-off-outline" => Some(icons::BOOKMARK_OFF_OUTLINE),
        "tram" => Some(icons::TRAM),
        "call-received" => Some(icons::CALL_RECEIVED),
        "palette-advanced" => Some(icons::PALETTE_ADVANCED),
        "shield-sun-outline" => Some(icons::SHIELD_SUN_OUTLINE),
        "robot-love" => Some(icons::ROBOT_LOVE),
        "information-off" => Some(icons::INFORMATION_OFF),
        "air-horn" => Some(icons::AIR_HORN),
        "folder-arrow-up-outline" => Some(icons::FOLDER_ARROW_UP_OUTLINE),
        "note-plus" => Some(icons::NOTE_PLUS),
        "decimal-comma-decrease" => Some(icons::DECIMAL_COMMA_DECREASE),
        "relation-zero-or-one-to-only-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONLY_ONE),
        "sort-variant-lock-open" => Some(icons::SORT_VARIANT_LOCK_OPEN),
        "play-circle" => Some(icons::PLAY_CIRCLE),
        "timeline-clock-outline" => Some(icons::TIMELINE_CLOCK_OUTLINE),
        "tag-multiple-outline" => Some(icons::TAG_MULTIPLE_OUTLINE),
        "airplane-minus" => Some(icons::AIRPLANE_MINUS),
        "seat-individual-suite" => Some(icons::SEAT_INDIVIDUAL_SUITE),
        "file-gif-box" => Some(icons::FILE_GIF_BOX),
        "chart-box" => Some(icons::CHART_BOX),
        "earth-minus" => Some(icons::EARTH_MINUS),
        "peanut-outline" => Some(icons::PEANUT_OUTLINE),
        "sim" => Some(icons::SIM),
        "code-tags" => Some(icons::CODE_TAGS),
        "blur-linear" => Some(icons::BLUR_LINEAR),
        "relation-zero-or-one-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ZERO_OR_ONE),
        "debug-step-out" => Some(icons::DEBUG_STEP_OUT),
        "router-wireless-settings" => Some(icons::ROUTER_WIRELESS_SETTINGS),
        "speaker-stop" => Some(icons::SPEAKER_STOP),
        "fullscreen-exit" => Some(icons::FULLSCREEN_EXIT),
        "database-settings-outline" => Some(icons::DATABASE_SETTINGS_OUTLINE),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "clover" => Some(icons::CLOVER),
        "division" => Some(icons::DIVISION),
        "weather-pouring" => Some(icons::WEATHER_POURING),
        #[allow(deprecated)]
        "graphql" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'graphql' is deprecated.").print(py);
            }
            Some(icons::GRAPHQL)
        }
        "train-car-hopper-covered" => Some(icons::TRAIN_CAR_HOPPER_COVERED),
        "folder-table-outline" => Some(icons::FOLDER_TABLE_OUTLINE),
        "car-arrow-right" => Some(icons::CAR_ARROW_RIGHT),
        "fruit-cherries" => Some(icons::FRUIT_CHERRIES),
        "yoga" => Some(icons::YOGA),
        "globe-model" => Some(icons::GLOBE_MODEL),
        _ => None,
    }
}
