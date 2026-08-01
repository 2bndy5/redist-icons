// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_36(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        #[allow(deprecated)]
        "google-cardboard" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cardboard' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CARDBOARD)
        }
        "table-tennis" => Some(icons::TABLE_TENNIS),
        "image-size-select-actual" => Some(icons::IMAGE_SIZE_SELECT_ACTUAL),
        "head-heart" => Some(icons::HEAD_HEART),
        "button-pointer" => Some(icons::BUTTON_POINTER),
        "home-battery" => Some(icons::HOME_BATTERY),
        "clipboard-arrow-left-outline" => Some(icons::CLIPBOARD_ARROW_LEFT_OUTLINE),
        "battery-charging-90" => Some(icons::BATTERY_CHARGING_90),
        "diving-scuba-mask" => Some(icons::DIVING_SCUBA_MASK),
        "information-box-outline" => Some(icons::INFORMATION_BOX_OUTLINE),
        "lock-open-check" => Some(icons::LOCK_OPEN_CHECK),
        "format-overline" => Some(icons::FORMAT_OVERLINE),
        "align-vertical-top" => Some(icons::ALIGN_VERTICAL_TOP),
        "comment-arrow-right" => Some(icons::COMMENT_ARROW_RIGHT),
        "scent" => Some(icons::SCENT),
        "beehive-outline" => Some(icons::BEEHIVE_OUTLINE),
        "briefcase-edit-outline" => Some(icons::BRIEFCASE_EDIT_OUTLINE),
        "clipboard-text-outline" => Some(icons::CLIPBOARD_TEXT_OUTLINE),
        "printer-3d-nozzle-heat" => Some(icons::PRINTER_3D_NOZZLE_HEAT),
        "calendar" => Some(icons::CALENDAR),
        "collapse-all" => Some(icons::COLLAPSE_ALL),
        #[allow(deprecated)]
        "google-earth" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-earth' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_EARTH)
        }
        "table-pivot" => Some(icons::TABLE_PIVOT),
        "diving-helmet" => Some(icons::DIVING_HELMET),
        "shield-link-variant-outline" => Some(icons::SHIELD_LINK_VARIANT_OUTLINE),
        "garage-alert" => Some(icons::GARAGE_ALERT),
        "football-helmet" => Some(icons::FOOTBALL_HELMET),
        "numeric-positive-1" => Some(icons::NUMERIC_POSITIVE_1),
        "note" => Some(icons::NOTE),
        "hand-peace-variant" => Some(icons::HAND_PEACE_VARIANT),
        "set-center-right" => Some(icons::SET_CENTER_RIGHT),
        "monitor-star" => Some(icons::MONITOR_STAR),
        "shield-check-outline" => Some(icons::SHIELD_CHECK_OUTLINE),
        "view-array" => Some(icons::VIEW_ARRAY),
        "folder-arrow-right-outline" => Some(icons::FOLDER_ARROW_RIGHT_OUTLINE),
        "turkey" => Some(icons::TURKEY),
        "video-check" => Some(icons::VIDEO_CHECK),
        "account-question-outline" => Some(icons::ACCOUNT_QUESTION_OUTLINE),
        "water-opacity" => Some(icons::WATER_OPACITY),
        "radiobox-indeterminate-variant" => Some(icons::RADIOBOX_INDETERMINATE_VARIANT),
        "filter-menu-outline" => Some(icons::FILTER_MENU_OUTLINE),
        "basketball" => Some(icons::BASKETBALL),
        "leaf" => Some(icons::LEAF),
        "nfc-variant" => Some(icons::NFC_VARIANT),
        "router-wireless-off" => Some(icons::ROUTER_WIRELESS_OFF),
        "city-variant" => Some(icons::CITY_VARIANT),
        "floor-plan" => Some(icons::FLOOR_PLAN),
        "access-point-off" => Some(icons::ACCESS_POINT_OFF),
        "alpha-y" => Some(icons::ALPHA_Y),
        "shape-square-plus" => Some(icons::SHAPE_SQUARE_PLUS),
        "drama-masks" => Some(icons::DRAMA_MASKS),
        "wifi-star" => Some(icons::WIFI_STAR),
        "folder-home-outline" => Some(icons::FOLDER_HOME_OUTLINE),
        "human-white-cane" => Some(icons::HUMAN_WHITE_CANE),
        "bicycle" => Some(icons::BICYCLE),
        "trophy" => Some(icons::TROPHY),
        "music-note-minus" => Some(icons::MUSIC_NOTE_MINUS),
        "battery-charging-outline" => Some(icons::BATTERY_CHARGING_OUTLINE),
        "wifi-strength-4-alert" => Some(icons::WIFI_STRENGTH_4_ALERT),
        "keyboard-f9" => Some(icons::KEYBOARD_F9),
        "asterisk" => Some(icons::ASTERISK),
        "account-box-outline" => Some(icons::ACCOUNT_BOX_OUTLINE),
        "pencil-ruler-outline" => Some(icons::PENCIL_RULER_OUTLINE),
        "awning" => Some(icons::AWNING),
        "cube-send" => Some(icons::CUBE_SEND),
        "gamepad-circle-outline" => Some(icons::GAMEPAD_CIRCLE_OUTLINE),
        "credit-card-minus" => Some(icons::CREDIT_CARD_MINUS),
        "receipt-text-edit-outline" => Some(icons::RECEIPT_TEXT_EDIT_OUTLINE),
        "screw-lag" => Some(icons::SCREW_LAG),
        "camera-metering-center" => Some(icons::CAMERA_METERING_CENTER),
        "car-door" => Some(icons::CAR_DOOR),
        "blinds-vertical" => Some(icons::BLINDS_VERTICAL),
        "account-multiple-check" => Some(icons::ACCOUNT_MULTIPLE_CHECK),
        "file-document-outline" => Some(icons::FILE_DOCUMENT_OUTLINE),
        "video-image" => Some(icons::VIDEO_IMAGE),
        "timer-stop" => Some(icons::TIMER_STOP),
        "usb-flash-drive" => Some(icons::USB_FLASH_DRIVE),
        "consolidate" => Some(icons::CONSOLIDATE),
        "folder-minus" => Some(icons::FOLDER_MINUS),
        "view-compact-outline" => Some(icons::VIEW_COMPACT_OUTLINE),
        "eye-check-outline" => Some(icons::EYE_CHECK_OUTLINE),
        "storefront-check-outline" => Some(icons::STOREFRONT_CHECK_OUTLINE),
        "cached" => Some(icons::CACHED),
        "home-map-marker" => Some(icons::HOME_MAP_MARKER),
        "relation-one-or-many-to-one-or-many" => Some(icons::RELATION_ONE_OR_MANY_TO_ONE_OR_MANY),
        "file-clock-outline" => Some(icons::FILE_CLOCK_OUTLINE),
        "relation-zero-or-one-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ZERO_OR_ONE),
        "menu-right" => Some(icons::MENU_RIGHT),
        #[allow(deprecated)]
        "svg" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'svg' is deprecated.").print(py);
            }
            Some(icons::SVG)
        }
        #[allow(deprecated)]
        "gnome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gnome' is deprecated.").print(py);
            }
            Some(icons::GNOME)
        }
        "message-check" => Some(icons::MESSAGE_CHECK),
        "backspace-reverse-outline" => Some(icons::BACKSPACE_REVERSE_OUTLINE),
        "tshirt-v-outline" => Some(icons::TSHIRT_V_OUTLINE),
        "orbit-variant" => Some(icons::ORBIT_VARIANT),
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
        "cog-sync-outline" => Some(icons::COG_SYNC_OUTLINE),
        "timer-sync-outline" => Some(icons::TIMER_SYNC_OUTLINE),
        "crown-circle" => Some(icons::CROWN_CIRCLE),
        "home-city-outline" => Some(icons::HOME_CITY_OUTLINE),
        "sort-variant-remove" => Some(icons::SORT_VARIANT_REMOVE),
        "tag-arrow-down-outline" => Some(icons::TAG_ARROW_DOWN_OUTLINE),
        "arrow-split-vertical" => Some(icons::ARROW_SPLIT_VERTICAL),
        "shape-circle-plus" => Some(icons::SHAPE_CIRCLE_PLUS),
        "home-percent-outline" => Some(icons::HOME_PERCENT_OUTLINE),
        "cards-playing-heart-outline" => Some(icons::CARDS_PLAYING_HEART_OUTLINE),
        "format-pilcrow" => Some(icons::FORMAT_PILCROW),
        #[allow(deprecated)]
        "language-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-go' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_GO)
        }
        "bicycle-cargo" => Some(icons::BICYCLE_CARGO),
        "account-box-edit-outline" => Some(icons::ACCOUNT_BOX_EDIT_OUTLINE),
        "fridge-variant-outline" => Some(icons::FRIDGE_VARIANT_OUTLINE),
        "alpha-z-box" => Some(icons::ALPHA_Z_BOX),
        "monitor-speaker" => Some(icons::MONITOR_SPEAKER),
        "clock-time-three-outline" => Some(icons::CLOCK_TIME_THREE_OUTLINE),
        "door-closed" => Some(icons::DOOR_CLOSED),
        "filter-settings-outline" => Some(icons::FILTER_SETTINGS_OUTLINE),
        "checkbox-marked" => Some(icons::CHECKBOX_MARKED),
        "bookmark-box" => Some(icons::BOOKMARK_BOX),
        "tag-outline" => Some(icons::TAG_OUTLINE),
        "timer-remove-outline" => Some(icons::TIMER_REMOVE_OUTLINE),
        "movie-open-play-outline" => Some(icons::MOVIE_OPEN_PLAY_OUTLINE),
        "home-percent" => Some(icons::HOME_PERCENT),
        "chat-minus-outline" => Some(icons::CHAT_MINUS_OUTLINE),
        "eye-settings-outline" => Some(icons::EYE_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "google-translate" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-translate' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_TRANSLATE)
        }
        "roman-numeral-10" => Some(icons::ROMAN_NUMERAL_10),
        "calendar-month" => Some(icons::CALENDAR_MONTH),
        "alpha-m-circle-outline" => Some(icons::ALPHA_M_CIRCLE_OUTLINE),
        "square-medium-outline" => Some(icons::SQUARE_MEDIUM_OUTLINE),
        "button-cursor" => Some(icons::BUTTON_CURSOR),
        "pen-off" => Some(icons::PEN_OFF),
        "chart-arc" => Some(icons::CHART_ARC),
        "relation-only-one-to-one-or-many" => Some(icons::RELATION_ONLY_ONE_TO_ONE_OR_MANY),
        "moon-waxing-gibbous" => Some(icons::MOON_WAXING_GIBBOUS),
        "clock-alert-outline" => Some(icons::CLOCK_ALERT_OUTLINE),
        "router-wireless" => Some(icons::ROUTER_WIRELESS),
        "math-compass" => Some(icons::MATH_COMPASS),
        "truck-delivery" => Some(icons::TRUCK_DELIVERY),
        "view-comfy" => Some(icons::VIEW_COMFY),
        "glass-mug-variant" => Some(icons::GLASS_MUG_VARIANT),
        "hexadecimal" => Some(icons::HEXADECIMAL),
        "bus-double-decker" => Some(icons::BUS_DOUBLE_DECKER),
        "wifi-strength-lock-outline" => Some(icons::WIFI_STRENGTH_LOCK_OUTLINE),
        "clipboard-file" => Some(icons::CLIPBOARD_FILE),
        "folder-download-outline" => Some(icons::FOLDER_DOWNLOAD_OUTLINE),
        "form-textbox-password" => Some(icons::FORM_TEXTBOX_PASSWORD),
        "arrow-up-bold-circle-outline" => Some(icons::ARROW_UP_BOLD_CIRCLE_OUTLINE),
        "cookie-refresh-outline" => Some(icons::COOKIE_REFRESH_OUTLINE),
        "square-circle" => Some(icons::SQUARE_CIRCLE),
        "arrow-u-down-left" => Some(icons::ARROW_U_DOWN_LEFT),
        "alert-circle-outline" => Some(icons::ALERT_CIRCLE_OUTLINE),
        "lock-alert-outline" => Some(icons::LOCK_ALERT_OUTLINE),
        "shark" => Some(icons::SHARK),
        #[allow(deprecated)]
        "language-php" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-php' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_PHP)
        }
        "gas-burner" => Some(icons::GAS_BURNER),
        "snowflake-variant" => Some(icons::SNOWFLAKE_VARIANT),
        "align-horizontal-right" => Some(icons::ALIGN_HORIZONTAL_RIGHT),
        "gamepad" => Some(icons::GAMEPAD),
        "walk" => Some(icons::WALK),
        "code-not-equal-variant" => Some(icons::CODE_NOT_EQUAL_VARIANT),
        "desktop-tower" => Some(icons::DESKTOP_TOWER),
        "bench" => Some(icons::BENCH),
        "table-column-width" => Some(icons::TABLE_COLUMN_WIDTH),
        "arrow-u-left-bottom" => Some(icons::ARROW_U_LEFT_BOTTOM),
        "vector-rectangle" => Some(icons::VECTOR_RECTANGLE),
        "eraser" => Some(icons::ERASER),
        "boxing-glove" => Some(icons::BOXING_GLOVE),
        "alpha-b-circle" => Some(icons::ALPHA_B_CIRCLE),
        "briefcase-clock-outline" => Some(icons::BRIEFCASE_CLOCK_OUTLINE),
        "sofa-outline" => Some(icons::SOFA_OUTLINE),
        "elevator-passenger-off-outline" => Some(icons::ELEVATOR_PASSENGER_OFF_OUTLINE),
        "comment-text-multiple-outline" => Some(icons::COMMENT_TEXT_MULTIPLE_OUTLINE),
        "car-brake-alert" => Some(icons::CAR_BRAKE_ALERT),
        "nut" => Some(icons::NUT),
        "timeline-check-outline" => Some(icons::TIMELINE_CHECK_OUTLINE),
        "store-plus-outline" => Some(icons::STORE_PLUS_OUTLINE),
        "ev-plug-type1" => Some(icons::EV_PLUG_TYPE1),
        "format-quote-close" => Some(icons::FORMAT_QUOTE_CLOSE),
        "stove" => Some(icons::STOVE),
        "box-cutter" => Some(icons::BOX_CUTTER),
        "timer-sand-complete" => Some(icons::TIMER_SAND_COMPLETE),
        "relation-zero-or-many-to-one-or-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE_OR_MANY),
        "play-circle-outline" => Some(icons::PLAY_CIRCLE_OUTLINE),
        "network-strength-4-cog" => Some(icons::NETWORK_STRENGTH_4_COG),
        "arrow-up-drop-circle" => Some(icons::ARROW_UP_DROP_CIRCLE),
        "download-network-outline" => Some(icons::DOWNLOAD_NETWORK_OUTLINE),
        "file-undo-outline" => Some(icons::FILE_UNDO_OUTLINE),
        "reorder-vertical" => Some(icons::REORDER_VERTICAL),
        "table-arrow-down" => Some(icons::TABLE_ARROW_DOWN),
        "dice-multiple" => Some(icons::DICE_MULTIPLE),
        "relation-only-one-to-one" => Some(icons::RELATION_ONLY_ONE_TO_ONE),
        "car-estate" => Some(icons::CAR_ESTATE),
        "database-cog-outline" => Some(icons::DATABASE_COG_OUTLINE),
        "weather-night-partly-cloudy" => Some(icons::WEATHER_NIGHT_PARTLY_CLOUDY),
        "countertop" => Some(icons::COUNTERTOP),
        "crosshairs-off" => Some(icons::CROSSHAIRS_OFF),
        "bowl" => Some(icons::BOWL),
        "cog-stop-outline" => Some(icons::COG_STOP_OUTLINE),
        "account-sync" => Some(icons::ACCOUNT_SYNC),
        "numeric-6-box-multiple-outline" => Some(icons::NUMERIC_6_BOX_MULTIPLE_OUTLINE),
        "train-car-autorack" => Some(icons::TRAIN_CAR_AUTORACK),
        _ => None,
    }
}
