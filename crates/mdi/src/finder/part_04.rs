// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_4(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "dice-d10" => Some(icons::DICE_D10),
        "printer-eye" => Some(icons::PRINTER_EYE),
        "form-textbox-password" => Some(icons::FORM_TEXTBOX_PASSWORD),
        "database-sync-outline" => Some(icons::DATABASE_SYNC_OUTLINE),
        "arrow-up-thin" => Some(icons::ARROW_UP_THIN),
        "sun-compass" => Some(icons::SUN_COMPASS),
        "keyboard-f5" => Some(icons::KEYBOARD_F5),
        "fit-to-screen-outline" => Some(icons::FIT_TO_SCREEN_OUTLINE),
        "fruit-grapes-outline" => Some(icons::FRUIT_GRAPES_OUTLINE),
        "weather-night-partly-cloudy" => Some(icons::WEATHER_NIGHT_PARTLY_CLOUDY),
        "signal-2g" => Some(icons::SIGNAL_2G),
        "dots-vertical-circle" => Some(icons::DOTS_VERTICAL_CIRCLE),
        "cash-fast" => Some(icons::CASH_FAST),
        "delete-clock-outline" => Some(icons::DELETE_CLOCK_OUTLINE),
        "calendar-badge-outline" => Some(icons::CALENDAR_BADGE_OUTLINE),
        "store-search-outline" => Some(icons::STORE_SEARCH_OUTLINE),
        "tooltip-text-outline" => Some(icons::TOOLTIP_TEXT_OUTLINE),
        "image-album" => Some(icons::IMAGE_ALBUM),
        "information-slab-circle-outline" => Some(icons::INFORMATION_SLAB_CIRCLE_OUTLINE),
        "movie-filter" => Some(icons::MOVIE_FILTER),
        "high-definition-box" => Some(icons::HIGH_DEFINITION_BOX),
        "numeric-negative-1" => Some(icons::NUMERIC_NEGATIVE_1),
        "weight-kilogram" => Some(icons::WEIGHT_KILOGRAM),
        "robot-angry" => Some(icons::ROBOT_ANGRY),
        "printer-pos-cog" => Some(icons::PRINTER_POS_COG),
        #[allow(deprecated)]
        "electron-framework" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'electron-framework' is deprecated.")
                    .print(py);
            }
            Some(icons::ELECTRON_FRAMEWORK)
        }
        #[allow(deprecated)]
        "kickstarter" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kickstarter' is deprecated.").print(py);
            }
            Some(icons::KICKSTARTER)
        }
        "wall-sconce-round-outline" => Some(icons::WALL_SCONCE_ROUND_OUTLINE),
        "violin" => Some(icons::VIOLIN),
        "cookie-edit-outline" => Some(icons::COOKIE_EDIT_OUTLINE),
        "pliers" => Some(icons::PLIERS),
        "water-opacity" => Some(icons::WATER_OPACITY),
        "door-sliding-lock" => Some(icons::DOOR_SLIDING_LOCK),
        "stadium-variant" => Some(icons::STADIUM_VARIANT),
        "gamepad-outline" => Some(icons::GAMEPAD_OUTLINE),
        "archive-cog" => Some(icons::ARCHIVE_COG),
        "label-off" => Some(icons::LABEL_OFF),
        "incognito-circle" => Some(icons::INCOGNITO_CIRCLE),
        "motion" => Some(icons::MOTION),
        "folder-image" => Some(icons::FOLDER_IMAGE),
        "window-shutter-auto" => Some(icons::WINDOW_SHUTTER_AUTO),
        "image-filter-center-focus-strong-outline" => {
            Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG_OUTLINE)
        }
        "sun-angle" => Some(icons::SUN_ANGLE),
        "note-off-outline" => Some(icons::NOTE_OFF_OUTLINE),
        "plus-lock-open" => Some(icons::PLUS_LOCK_OPEN),
        "account-wrench-outline" => Some(icons::ACCOUNT_WRENCH_OUTLINE),
        "fit-to-page-outline" => Some(icons::FIT_TO_PAGE_OUTLINE),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "octagram-outline" => Some(icons::OCTAGRAM_OUTLINE),
        "flower" => Some(icons::FLOWER),
        "head-question" => Some(icons::HEAD_QUESTION),
        "briefcase-remove-outline" => Some(icons::BRIEFCASE_REMOVE_OUTLINE),
        "vector-ellipse" => Some(icons::VECTOR_ELLIPSE),
        "watch-vibrate-off" => Some(icons::WATCH_VIBRATE_OFF),
        "credit-card-wireless-off-outline" => Some(icons::CREDIT_CARD_WIRELESS_OFF_OUTLINE),
        "blender" => Some(icons::BLENDER),
        "pin-outline" => Some(icons::PIN_OUTLINE),
        "store-marker" => Some(icons::STORE_MARKER),
        "file-alert" => Some(icons::FILE_ALERT),
        "pump-off" => Some(icons::PUMP_OFF),
        "cube" => Some(icons::CUBE),
        #[allow(deprecated)]
        "apple-ios" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-ios' is deprecated.").print(py);
            }
            Some(icons::APPLE_IOS)
        }
        "movie-open-edit" => Some(icons::MOVIE_OPEN_EDIT),
        "eject" => Some(icons::EJECT),
        "boom-gate-alert-outline" => Some(icons::BOOM_GATE_ALERT_OUTLINE),
        "scale-unbalanced" => Some(icons::SCALE_UNBALANCED),
        "account-tie-woman" => Some(icons::ACCOUNT_TIE_WOMAN),
        "arrow-right-bold-circle-outline" => Some(icons::ARROW_RIGHT_BOLD_CIRCLE_OUTLINE),
        "graph-outline" => Some(icons::GRAPH_OUTLINE),
        "filter-outline" => Some(icons::FILTER_OUTLINE),
        "alphabetical-variant" => Some(icons::ALPHABETICAL_VARIANT),
        "gauge-low" => Some(icons::GAUGE_LOW),
        "clock-time-one" => Some(icons::CLOCK_TIME_ONE),
        "chart-box-multiple-outline" => Some(icons::CHART_BOX_MULTIPLE_OUTLINE),
        "rotate-3d-variant" => Some(icons::ROTATE_3D_VARIANT),
        "flower-pollen-outline" => Some(icons::FLOWER_POLLEN_OUTLINE),
        "timer-refresh" => Some(icons::TIMER_REFRESH),
        "store-clock-outline" => Some(icons::STORE_CLOCK_OUTLINE),
        "puzzle" => Some(icons::PUZZLE),
        "radio-handheld" => Some(icons::RADIO_HANDHELD),
        "compass-off-outline" => Some(icons::COMPASS_OFF_OUTLINE),
        "phone-bluetooth-outline" => Some(icons::PHONE_BLUETOOTH_OUTLINE),
        "alpha-k-circle" => Some(icons::ALPHA_K_CIRCLE),
        "play-network" => Some(icons::PLAY_NETWORK),
        "text-box-plus" => Some(icons::TEXT_BOX_PLUS),
        "dump-truck" => Some(icons::DUMP_TRUCK),
        "store-minus-outline" => Some(icons::STORE_MINUS_OUTLINE),
        "dog-side-off" => Some(icons::DOG_SIDE_OFF),
        "menu-left-outline" => Some(icons::MENU_LEFT_OUTLINE),
        "invoice-text-clock" => Some(icons::INVOICE_TEXT_CLOCK),
        "cookie-clock-outline" => Some(icons::COOKIE_CLOCK_OUTLINE),
        "heart-cog" => Some(icons::HEART_COG),
        "podium-bronze" => Some(icons::PODIUM_BRONZE),
        #[allow(deprecated)]
        "steam" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'steam' is deprecated.").print(py);
            }
            Some(icons::STEAM)
        }
        "closed-caption-outline" => Some(icons::CLOSED_CAPTION_OUTLINE),
        #[allow(deprecated)]
        "digital-ocean" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'digital-ocean' is deprecated.").print(py);
            }
            Some(icons::DIGITAL_OCEAN)
        }
        "ip-outline" => Some(icons::IP_OUTLINE),
        #[allow(deprecated)]
        "sina-weibo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sina-weibo' is deprecated.").print(py);
            }
            Some(icons::SINA_WEIBO)
        }
        #[allow(deprecated)]
        "google-chrome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-chrome' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CHROME)
        }
        "magnify-close" => Some(icons::MAGNIFY_CLOSE),
        "vector-rectangle" => Some(icons::VECTOR_RECTANGLE),
        "shark-fin-outline" => Some(icons::SHARK_FIN_OUTLINE),
        "printer-pos-wrench" => Some(icons::PRINTER_POS_WRENCH),
        "application-braces" => Some(icons::APPLICATION_BRACES),
        "circle-slice-4" => Some(icons::CIRCLE_SLICE_4),
        "ice-cream" => Some(icons::ICE_CREAM),
        "pencil-plus" => Some(icons::PENCIL_PLUS),
        "water-sync" => Some(icons::WATER_SYNC),
        "sort-calendar-ascending" => Some(icons::SORT_CALENDAR_ASCENDING),
        "shield-account" => Some(icons::SHIELD_ACCOUNT),
        "head-lightbulb-outline" => Some(icons::HEAD_LIGHTBULB_OUTLINE),
        "note-alert-outline" => Some(icons::NOTE_ALERT_OUTLINE),
        "menu-right" => Some(icons::MENU_RIGHT),
        "sun-snowflake" => Some(icons::SUN_SNOWFLAKE),
        "notebook-outline" => Some(icons::NOTEBOOK_OUTLINE),
        "transmission-tower-off" => Some(icons::TRANSMISSION_TOWER_OFF),
        "email-mark-as-unread" => Some(icons::EMAIL_MARK_AS_UNREAD),
        "battery-medium" => Some(icons::BATTERY_MEDIUM),
        "tooth-outline" => Some(icons::TOOTH_OUTLINE),
        "chevron-double-up" => Some(icons::CHEVRON_DOUBLE_UP),
        "signal-cellular-2" => Some(icons::SIGNAL_CELLULAR_2),
        "fit-to-screen" => Some(icons::FIT_TO_SCREEN),
        "relation-zero-or-one-to-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE),
        "lightbulb-group" => Some(icons::LIGHTBULB_GROUP),
        "chevron-double-right" => Some(icons::CHEVRON_DOUBLE_RIGHT),
        "sleep-off" => Some(icons::SLEEP_OFF),
        "factory" => Some(icons::FACTORY),
        "sitemap-outline" => Some(icons::SITEMAP_OUTLINE),
        #[allow(deprecated)]
        "yahoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'yahoo' is deprecated.").print(py);
            }
            Some(icons::YAHOO)
        }
        "angle-acute" => Some(icons::ANGLE_ACUTE),
        "arrow-top-right-thin" => Some(icons::ARROW_TOP_RIGHT_THIN),
        "video-outline" => Some(icons::VIDEO_OUTLINE),
        "skip-next-circle" => Some(icons::SKIP_NEXT_CIRCLE),
        "microphone-variant" => Some(icons::MICROPHONE_VARIANT),
        "microscope" => Some(icons::MICROSCOPE),
        "seat-legroom-reduced" => Some(icons::SEAT_LEGROOM_REDUCED),
        #[allow(deprecated)]
        "google-analytics" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-analytics' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ANALYTICS)
        }
        "store-search" => Some(icons::STORE_SEARCH),
        "share" => Some(icons::SHARE),
        "cog-refresh-outline" => Some(icons::COG_REFRESH_OUTLINE),
        "sort-clock-descending-outline" => Some(icons::SORT_CLOCK_DESCENDING_OUTLINE),
        #[allow(deprecated)]
        "language-python" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-python' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_PYTHON)
        }
        "speaker-play" => Some(icons::SPEAKER_PLAY),
        "octagon" => Some(icons::OCTAGON),
        "power-socket-ch" => Some(icons::POWER_SOCKET_CH),
        "tea-outline" => Some(icons::TEA_OUTLINE),
        "bus" => Some(icons::BUS),
        "water" => Some(icons::WATER),
        "archive-sync-outline" => Some(icons::ARCHIVE_SYNC_OUTLINE),
        "arrow-down-bold-box-outline" => Some(icons::ARROW_DOWN_BOLD_BOX_OUTLINE),
        "gesture-double-tap" => Some(icons::GESTURE_DOUBLE_TAP),
        "email-open-multiple-outline" => Some(icons::EMAIL_OPEN_MULTIPLE_OUTLINE),
        "folder-question" => Some(icons::FOLDER_QUESTION),
        "party-popper" => Some(icons::PARTY_POPPER),
        "email-check" => Some(icons::EMAIL_CHECK),
        "handshake-outline" => Some(icons::HANDSHAKE_OUTLINE),
        "camera-wireless" => Some(icons::CAMERA_WIRELESS),
        "cellphone-wireless" => Some(icons::CELLPHONE_WIRELESS),
        "numeric-3-circle-outline" => Some(icons::NUMERIC_3_CIRCLE_OUTLINE),
        "air-purifier" => Some(icons::AIR_PURIFIER),
        "view-split-vertical" => Some(icons::VIEW_SPLIT_VERTICAL),
        "sticker-check" => Some(icons::STICKER_CHECK),
        #[allow(deprecated)]
        "teamviewer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'teamviewer' is deprecated.").print(py);
            }
            Some(icons::TEAMVIEWER)
        }
        "pound-box-outline" => Some(icons::POUND_BOX_OUTLINE),
        "eyedropper-variant" => Some(icons::EYEDROPPER_VARIANT),
        "store-settings-outline" => Some(icons::STORE_SETTINGS_OUTLINE),
        "equal" => Some(icons::EQUAL),
        "folder-edit-outline" => Some(icons::FOLDER_EDIT_OUTLINE),
        "bus-electric" => Some(icons::BUS_ELECTRIC),
        "toolbox" => Some(icons::TOOLBOX),
        "thermometer-minus" => Some(icons::THERMOMETER_MINUS),
        "speaker-pause" => Some(icons::SPEAKER_PAUSE),
        "printer-pos-cog-outline" => Some(icons::PRINTER_POS_COG_OUTLINE),
        "arrow-all" => Some(icons::ARROW_ALL),
        "account-cowboy-hat-outline" => Some(icons::ACCOUNT_COWBOY_HAT_OUTLINE),
        "meter-gas-outline" => Some(icons::METER_GAS_OUTLINE),
        "van-passenger" => Some(icons::VAN_PASSENGER),
        "select-off" => Some(icons::SELECT_OFF),
        "volcano-outline" => Some(icons::VOLCANO_OUTLINE),
        "camera-lock-open-outline" => Some(icons::CAMERA_LOCK_OPEN_OUTLINE),
        "arrange-send-backward" => Some(icons::ARRANGE_SEND_BACKWARD),
        "music-note-minus" => Some(icons::MUSIC_NOTE_MINUS),
        "credit-card-wireless-off" => Some(icons::CREDIT_CARD_WIRELESS_OFF),
        "numeric-8" => Some(icons::NUMERIC_8),
        "vector-arrange-above" => Some(icons::VECTOR_ARRANGE_ABOVE),
        "menorah" => Some(icons::MENORAH),
        "alarm-bell" => Some(icons::ALARM_BELL),
        "roman-numeral-6" => Some(icons::ROMAN_NUMERAL_6),
        "upload-multiple-outline" => Some(icons::UPLOAD_MULTIPLE_OUTLINE),
        "calendar-edit-outline" => Some(icons::CALENDAR_EDIT_OUTLINE),
        "shuriken" => Some(icons::SHURIKEN),
        "garage-lock" => Some(icons::GARAGE_LOCK),
        "baby-carriage" => Some(icons::BABY_CARRIAGE),
        "jump-rope" => Some(icons::JUMP_ROPE),
        "timer-stop" => Some(icons::TIMER_STOP),
        "arrow-u-right-bottom-bold" => Some(icons::ARROW_U_RIGHT_BOTTOM_BOLD),
        "minus-thick" => Some(icons::MINUS_THICK),
        "bathtub" => Some(icons::BATHTUB),
        "airplane-search" => Some(icons::AIRPLANE_SEARCH),
        "movie-cog-outline" => Some(icons::MOVIE_COG_OUTLINE),
        _ => None,
    }
}
