// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_8(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "eyedropper-minus" => Some(icons::EYEDROPPER_MINUS),
        "arrow-left-top-bold" => Some(icons::ARROW_LEFT_TOP_BOLD),
        "comment-text-outline" => Some(icons::COMMENT_TEXT_OUTLINE),
        #[allow(deprecated)]
        "polymer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'polymer' is deprecated.").print(py);
            }
            Some(icons::POLYMER)
        }
        "align-vertical-distribute" => Some(icons::ALIGN_VERTICAL_DISTRIBUTE),
        "television-shimmer" => Some(icons::TELEVISION_SHIMMER),
        "numeric-2-box-multiple" => Some(icons::NUMERIC_2_BOX_MULTIPLE),
        "cookie-minus" => Some(icons::COOKIE_MINUS),
        "format-paint" => Some(icons::FORMAT_PAINT),
        #[allow(deprecated)]
        "graphql" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'graphql' is deprecated.").print(py);
            }
            Some(icons::GRAPHQL)
        }
        "elevator-passenger-off" => Some(icons::ELEVATOR_PASSENGER_OFF),
        "folder-clock-outline" => Some(icons::FOLDER_CLOCK_OUTLINE),
        "table-clock" => Some(icons::TABLE_CLOCK),
        "wall-sconce-round-variant" => Some(icons::WALL_SCONCE_ROUND_VARIANT),
        "air-humidifier" => Some(icons::AIR_HUMIDIFIER),
        "play-box-lock" => Some(icons::PLAY_BOX_LOCK),
        "timeline-question-outline" => Some(icons::TIMELINE_QUESTION_OUTLINE),
        "fingerprint" => Some(icons::FINGERPRINT),
        "phone-clock" => Some(icons::PHONE_CLOCK),
        "iron-board" => Some(icons::IRON_BOARD),
        "music-accidental-sharp" => Some(icons::MUSIC_ACCIDENTAL_SHARP),
        "pot-mix-outline" => Some(icons::POT_MIX_OUTLINE),
        "margin" => Some(icons::MARGIN),
        "arrow-up-thin" => Some(icons::ARROW_UP_THIN),
        "layers-remove" => Some(icons::LAYERS_REMOVE),
        "tray-remove" => Some(icons::TRAY_REMOVE),
        "clock-time-four" => Some(icons::CLOCK_TIME_FOUR),
        "file-send-outline" => Some(icons::FILE_SEND_OUTLINE),
        "wifi-strength-4" => Some(icons::WIFI_STRENGTH_4),
        #[allow(deprecated)]
        "codepen" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'codepen' is deprecated.").print(py);
            }
            Some(icons::CODEPEN)
        }
        "sine-wave" => Some(icons::SINE_WAVE),
        "calendar-multiselect" => Some(icons::CALENDAR_MULTISELECT),
        "soy-sauce-off" => Some(icons::SOY_SAUCE_OFF),
        "truck-minus" => Some(icons::TRUCK_MINUS),
        "record-circle-outline" => Some(icons::RECORD_CIRCLE_OUTLINE),
        "fish" => Some(icons::FISH),
        "view-comfy-outline" => Some(icons::VIEW_COMFY_OUTLINE),
        "download-multiple-outline" => Some(icons::DOWNLOAD_MULTIPLE_OUTLINE),
        "battery-positive" => Some(icons::BATTERY_POSITIVE),
        "alert-octagon" => Some(icons::ALERT_OCTAGON),
        "home-alert-outline" => Some(icons::HOME_ALERT_OUTLINE),
        "bell-off-outline" => Some(icons::BELL_OFF_OUTLINE),
        "format-font-size-decrease" => Some(icons::FORMAT_FONT_SIZE_DECREASE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-medium" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-medium' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_MEDIUM)
        }
        #[allow(deprecated)]
        "unicode" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unicode' is deprecated.").print(py);
            }
            Some(icons::UNICODE)
        }
        "paw-off-outline" => Some(icons::PAW_OFF_OUTLINE),
        "shield-lock-open-outline" => Some(icons::SHIELD_LOCK_OPEN_OUTLINE),
        "ultra-high-definition" => Some(icons::ULTRA_HIGH_DEFINITION),
        "arrow-u-up-right" => Some(icons::ARROW_U_UP_RIGHT),
        "spear" => Some(icons::SPEAR),
        "map-marker-check-outline" => Some(icons::MAP_MARKER_CHECK_OUTLINE),
        "cookie-off" => Some(icons::COOKIE_OFF),
        "currency-inr" => Some(icons::CURRENCY_INR),
        "movie-star-outline" => Some(icons::MOVIE_STAR_OUTLINE),
        "headphones-off" => Some(icons::HEADPHONES_OFF),
        "comment-account" => Some(icons::COMMENT_ACCOUNT),
        "paw" => Some(icons::PAW),
        "download" => Some(icons::DOWNLOAD),
        "book-open-page-variant" => Some(icons::BOOK_OPEN_PAGE_VARIANT),
        "book-plus-multiple" => Some(icons::BOOK_PLUS_MULTIPLE),
        "tooltip-remove" => Some(icons::TOOLTIP_REMOVE),
        "format-header-equal" => Some(icons::FORMAT_HEADER_EQUAL),
        "briefcase-check" => Some(icons::BRIEFCASE_CHECK),
        "axis-y-rotate-counterclockwise" => Some(icons::AXIS_Y_ROTATE_COUNTERCLOCKWISE),
        "printer-pos-star-outline" => Some(icons::PRINTER_POS_STAR_OUTLINE),
        #[allow(deprecated)]
        "jquery" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jquery' is deprecated.").print(py);
            }
            Some(icons::JQUERY)
        }
        "page-layout-sidebar-right" => Some(icons::PAGE_LAYOUT_SIDEBAR_RIGHT),
        "message-bookmark" => Some(icons::MESSAGE_BOOKMARK),
        "forest" => Some(icons::FOREST),
        "calendar-search-outline" => Some(icons::CALENDAR_SEARCH_OUTLINE),
        "scoreboard" => Some(icons::SCOREBOARD),
        "alpha-o-circle-outline" => Some(icons::ALPHA_O_CIRCLE_OUTLINE),
        "corn" => Some(icons::CORN),
        "head-lightbulb" => Some(icons::HEAD_LIGHTBULB),
        "note-off-outline" => Some(icons::NOTE_OFF_OUTLINE),
        "arrow-down-bold-hexagon-outline" => Some(icons::ARROW_DOWN_BOLD_HEXAGON_OUTLINE),
        "train-car-flatbed-car" => Some(icons::TRAIN_CAR_FLATBED_CAR),
        "alpha-a-circle-outline" => Some(icons::ALPHA_A_CIRCLE_OUTLINE),
        "ethernet-cable-off" => Some(icons::ETHERNET_CABLE_OFF),
        "home-floor-a" => Some(icons::HOME_FLOOR_A),
        "food-hot-dog" => Some(icons::FOOD_HOT_DOG),
        "eye-closed" => Some(icons::EYE_CLOSED),
        "cloud" => Some(icons::CLOUD),
        "bell-off" => Some(icons::BELL_OFF),
        "file-arrow-left-right-outline" => Some(icons::FILE_ARROW_LEFT_RIGHT_OUTLINE),
        "skateboard" => Some(icons::SKATEBOARD),
        "kettle-alert-outline" => Some(icons::KETTLE_ALERT_OUTLINE),
        "cash-refund" => Some(icons::CASH_REFUND),
        "storefront-minus" => Some(icons::STOREFRONT_MINUS),
        "heart-half" => Some(icons::HEART_HALF),
        "heart-half-full" => Some(icons::HEART_HALF_FULL),
        "toggle-switch" => Some(icons::TOGGLE_SWITCH),
        "signal-cellular-1" => Some(icons::SIGNAL_CELLULAR_1),
        "arrow-left-circle-outline" => Some(icons::ARROW_LEFT_CIRCLE_OUTLINE),
        "focus-field-vertical" => Some(icons::FOCUS_FIELD_VERTICAL),
        "movie-open-remove-outline" => Some(icons::MOVIE_OPEN_REMOVE_OUTLINE),
        "door-open" => Some(icons::DOOR_OPEN),
        #[allow(deprecated)]
        "npm" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'npm' is deprecated.").print(py);
            }
            Some(icons::NPM)
        }
        "book-cancel" => Some(icons::BOOK_CANCEL),
        "earth-remove" => Some(icons::EARTH_REMOVE),
        "chili-hot-outline" => Some(icons::CHILI_HOT_OUTLINE),
        "card-account-details-outline" => Some(icons::CARD_ACCOUNT_DETAILS_OUTLINE),
        "bicycle-basket" => Some(icons::BICYCLE_BASKET),
        "tie" => Some(icons::TIE),
        "battery-90" => Some(icons::BATTERY_90),
        "submarine" => Some(icons::SUBMARINE),
        "podium" => Some(icons::PODIUM),
        "cross-celtic" => Some(icons::CROSS_CELTIC),
        "relation-only-one-to-zero-or-one" => Some(icons::RELATION_ONLY_ONE_TO_ZERO_OR_ONE),
        "bowl" => Some(icons::BOWL),
        "text-box-remove" => Some(icons::TEXT_BOX_REMOVE),
        "vhs" => Some(icons::VHS),
        "video-plus" => Some(icons::VIDEO_PLUS),
        "image-broken-variant" => Some(icons::IMAGE_BROKEN_VARIANT),
        "hand-coin-outline" => Some(icons::HAND_COIN_OUTLINE),
        "asterisk-circle-outline" => Some(icons::ASTERISK_CIRCLE_OUTLINE),
        "slope-downhill" => Some(icons::SLOPE_DOWNHILL),
        "hub-outline" => Some(icons::HUB_OUTLINE),
        "link-plus" => Some(icons::LINK_PLUS),
        "phone-in-talk-outline" => Some(icons::PHONE_IN_TALK_OUTLINE),
        "qrcode-minus" => Some(icons::QRCODE_MINUS),
        "circle-half" => Some(icons::CIRCLE_HALF),
        "invoice-text-edit-outline" => Some(icons::INVOICE_TEXT_EDIT_OUTLINE),
        "mouse-variant" => Some(icons::MOUSE_VARIANT),
        "target-variant" => Some(icons::TARGET_VARIANT),
        "server-network" => Some(icons::SERVER_NETWORK),
        "ballot-outline" => Some(icons::BALLOT_OUTLINE),
        "clipboard-minus" => Some(icons::CLIPBOARD_MINUS),
        "smoke-detector-variant-off" => Some(icons::SMOKE_DETECTOR_VARIANT_OFF),
        "map-marker-multiple-outline" => Some(icons::MAP_MARKER_MULTIPLE_OUTLINE),
        "package-variant-plus" => Some(icons::PACKAGE_VARIANT_PLUS),
        "timer-10" => Some(icons::TIMER_10),
        "checkbox-multiple-marked-circle-outline" => {
            Some(icons::CHECKBOX_MULTIPLE_MARKED_CIRCLE_OUTLINE)
        }
        "invoice-text-plus-outline" => Some(icons::INVOICE_TEXT_PLUS_OUTLINE),
        "account-network-outline" => Some(icons::ACCOUNT_NETWORK_OUTLINE),
        "currency-rial" => Some(icons::CURRENCY_RIAL),
        "human-child" => Some(icons::HUMAN_CHILD),
        "snake" => Some(icons::SNAKE),
        "phone-missed-outline" => Some(icons::PHONE_MISSED_OUTLINE),
        "radiobox-indeterminate-variant" => Some(icons::RADIOBOX_INDETERMINATE_VARIANT),
        "keyboard-settings-outline" => Some(icons::KEYBOARD_SETTINGS_OUTLINE),
        "printer-pos-off" => Some(icons::PRINTER_POS_OFF),
        "eject" => Some(icons::EJECT),
        "sprinkler-variant" => Some(icons::SPRINKLER_VARIANT),
        "calendar-week-begin" => Some(icons::CALENDAR_WEEK_BEGIN),
        "toy-brick-minus" => Some(icons::TOY_BRICK_MINUS),
        "undo-variant" => Some(icons::UNDO_VARIANT),
        "alpha-h-circle" => Some(icons::ALPHA_H_CIRCLE),
        "bell-alert" => Some(icons::BELL_ALERT),
        "clipboard-pulse" => Some(icons::CLIPBOARD_PULSE),
        "chart-bar-stacked" => Some(icons::CHART_BAR_STACKED),
        "screw-lag" => Some(icons::SCREW_LAG),
        "nature-people" => Some(icons::NATURE_PEOPLE),
        "file-phone" => Some(icons::FILE_PHONE),
        "radioactive" => Some(icons::RADIOACTIVE),
        "ceiling-light-multiple" => Some(icons::CEILING_LIGHT_MULTIPLE),
        "cogs" => Some(icons::COGS),
        "office-building-minus" => Some(icons::OFFICE_BUILDING_MINUS),
        "battery-high" => Some(icons::BATTERY_HIGH),
        "city-variant" => Some(icons::CITY_VARIANT),
        "hazard-lights" => Some(icons::HAZARD_LIGHTS),
        "cash-minus" => Some(icons::CASH_MINUS),
        "gesture-spread" => Some(icons::GESTURE_SPREAD),
        "archive-plus" => Some(icons::ARCHIVE_PLUS),
        "progress-tag" => Some(icons::PROGRESS_TAG),
        "history" => Some(icons::HISTORY),
        "ev-station" => Some(icons::EV_STATION),
        "view-module" => Some(icons::VIEW_MODULE),
        "snowflake-alert" => Some(icons::SNOWFLAKE_ALERT),
        "numeric-3-circle-outline" => Some(icons::NUMERIC_3_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "firefox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firefox' is deprecated.").print(py);
            }
            Some(icons::FIREFOX)
        }
        "file-remove" => Some(icons::FILE_REMOVE),
        "arrow-right-thick" => Some(icons::ARROW_RIGHT_THICK),
        "message-reply-text" => Some(icons::MESSAGE_REPLY_TEXT),
        "lock-open-minus-outline" => Some(icons::LOCK_OPEN_MINUS_OUTLINE),
        "clock-minus-outline" => Some(icons::CLOCK_MINUS_OUTLINE),
        "label-multiple-outline" => Some(icons::LABEL_MULTIPLE_OUTLINE),
        "gesture-swipe-vertical" => Some(icons::GESTURE_SWIPE_VERTICAL),
        "database-alert-outline" => Some(icons::DATABASE_ALERT_OUTLINE),
        "circle" => Some(icons::CIRCLE),
        "hand-water" => Some(icons::HAND_WATER),
        "navigation-variant-outline" => Some(icons::NAVIGATION_VARIANT_OUTLINE),
        "bed-double-outline" => Some(icons::BED_DOUBLE_OUTLINE),
        "fullscreen-exit" => Some(icons::FULLSCREEN_EXIT),
        "chart-areaspline-variant" => Some(icons::CHART_AREASPLINE_VARIANT),
        "clipboard-arrow-left-outline" => Some(icons::CLIPBOARD_ARROW_LEFT_OUTLINE),
        "cards-playing" => Some(icons::CARDS_PLAYING),
        "boom-gate-up" => Some(icons::BOOM_GATE_UP),
        "car-esp" => Some(icons::CAR_ESP),
        "forum-minus" => Some(icons::FORUM_MINUS),
        "backspace-reverse" => Some(icons::BACKSPACE_REVERSE),
        "clock-star-four-points" => Some(icons::CLOCK_STAR_FOUR_POINTS),
        #[allow(deprecated)]
        "semantic-web" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'semantic-web' is deprecated.").print(py);
            }
            Some(icons::SEMANTIC_WEB)
        }
        "gas-cylinder" => Some(icons::GAS_CYLINDER),
        "fridge-bottom" => Some(icons::FRIDGE_BOTTOM),
        "webcam-off" => Some(icons::WEBCAM_OFF),
        "numeric-8-box" => Some(icons::NUMERIC_8_BOX),
        "book-sync" => Some(icons::BOOK_SYNC),
        "camera-lock-outline" => Some(icons::CAMERA_LOCK_OUTLINE),
        "pickaxe" => Some(icons::PICKAXE),
        _ => None,
    }
}
