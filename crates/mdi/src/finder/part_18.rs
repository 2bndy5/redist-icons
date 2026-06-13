// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_18(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "tools" => Some(icons::TOOLS),
        "account-switch-outline" => Some(icons::ACCOUNT_SWITCH_OUTLINE),
        "fishbowl" => Some(icons::FISHBOWL),
        "page-first" => Some(icons::PAGE_FIRST),
        "lotion-plus-outline" => Some(icons::LOTION_PLUS_OUTLINE),
        "subway-variant" => Some(icons::SUBWAY_VARIANT),
        "lock-off-outline" => Some(icons::LOCK_OFF_OUTLINE),
        "star-david" => Some(icons::STAR_DAVID),
        "set-all" => Some(icons::SET_ALL),
        "molecule-co" => Some(icons::MOLECULE_CO),
        "attachment-plus" => Some(icons::ATTACHMENT_PLUS),
        "arrow-left-thin-circle-outline" => Some(icons::ARROW_LEFT_THIN_CIRCLE_OUTLINE),
        "clipboard-multiple" => Some(icons::CLIPBOARD_MULTIPLE),
        "diving" => Some(icons::DIVING),
        "fan-chevron-up" => Some(icons::FAN_CHEVRON_UP),
        "map-marker-alert" => Some(icons::MAP_MARKER_ALERT),
        "numeric-10-box-outline" => Some(icons::NUMERIC_10_BOX_OUTLINE),
        "circle-slice-4" => Some(icons::CIRCLE_SLICE_4),
        "printer-pos-refresh" => Some(icons::PRINTER_POS_REFRESH),
        "nutrition" => Some(icons::NUTRITION),
        "timer-minus" => Some(icons::TIMER_MINUS),
        "truck-alert-outline" => Some(icons::TRUCK_ALERT_OUTLINE),
        "gender-female" => Some(icons::GENDER_FEMALE),
        "gesture-two-double-tap" => Some(icons::GESTURE_TWO_DOUBLE_TAP),
        "clock-alert" => Some(icons::CLOCK_ALERT),
        "powershell" => Some(icons::POWERSHELL),
        "archive-outline" => Some(icons::ARCHIVE_OUTLINE),
        "clipboard-arrow-up" => Some(icons::CLIPBOARD_ARROW_UP),
        "weather-sunny-off" => Some(icons::WEATHER_SUNNY_OFF),
        "credit-card-off-outline" => Some(icons::CREDIT_CARD_OFF_OUTLINE),
        "arrow-up-bold-box-outline" => Some(icons::ARROW_UP_BOLD_BOX_OUTLINE),
        "comment-check" => Some(icons::COMMENT_CHECK),
        "near-me" => Some(icons::NEAR_ME),
        "adjust" => Some(icons::ADJUST),
        "sprinkler-variant" => Some(icons::SPRINKLER_VARIANT),
        "folder-cog" => Some(icons::FOLDER_COG),
        "dots-circle" => Some(icons::DOTS_CIRCLE),
        "book-account-outline" => Some(icons::BOOK_ACCOUNT_OUTLINE),
        "engine-outline" => Some(icons::ENGINE_OUTLINE),
        "folder-lock-open" => Some(icons::FOLDER_LOCK_OPEN),
        "apps" => Some(icons::APPS),
        "robot-vacuum-alert" => Some(icons::ROBOT_VACUUM_ALERT),
        "swap-horizontal-variant" => Some(icons::SWAP_HORIZONTAL_VARIANT),
        "map-marker-radius-outline" => Some(icons::MAP_MARKER_RADIUS_OUTLINE),
        "menu-swap" => Some(icons::MENU_SWAP),
        "database-off" => Some(icons::DATABASE_OFF),
        #[allow(deprecated)]
        "wechat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wechat' is deprecated.").print(py);
            }
            Some(icons::WECHAT)
        }
        "numeric-10-box-multiple" => Some(icons::NUMERIC_10_BOX_MULTIPLE),
        "chat" => Some(icons::CHAT),
        "parachute" => Some(icons::PARACHUTE),
        "arrow-collapse-left" => Some(icons::ARROW_COLLAPSE_LEFT),
        "play-box-lock-open" => Some(icons::PLAY_BOX_LOCK_OPEN),
        "note-multiple" => Some(icons::NOTE_MULTIPLE),
        "shield-star-outline" => Some(icons::SHIELD_STAR_OUTLINE),
        "calendar-week-outline" => Some(icons::CALENDAR_WEEK_OUTLINE),
        "lightbulb-group-off-outline" => Some(icons::LIGHTBULB_GROUP_OFF_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-charging" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-charging' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_CHARGING)
        }
        "message-off-outline" => Some(icons::MESSAGE_OFF_OUTLINE),
        "decimal-increase" => Some(icons::DECIMAL_INCREASE),
        "calendar-plus-outline" => Some(icons::CALENDAR_PLUS_OUTLINE),
        #[allow(deprecated)]
        "evernote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'evernote' is deprecated.").print(py);
            }
            Some(icons::EVERNOTE)
        }
        "skip-forward-outline" => Some(icons::SKIP_FORWARD_OUTLINE),
        "music-note-half" => Some(icons::MUSIC_NOTE_HALF),
        "paperclip-check" => Some(icons::PAPERCLIP_CHECK),
        "weight-gram" => Some(icons::WEIGHT_GRAM),
        "fruit-cherries-off" => Some(icons::FRUIT_CHERRIES_OFF),
        "cast" => Some(icons::CAST),
        "temple-buddhist" => Some(icons::TEMPLE_BUDDHIST),
        "lock" => Some(icons::LOCK),
        "server-plus" => Some(icons::SERVER_PLUS),
        "map-marker-question-outline" => Some(icons::MAP_MARKER_QUESTION_OUTLINE),
        #[allow(deprecated)]
        "microsoft-onenote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onenote' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONENOTE)
        }
        "alpha-a-box" => Some(icons::ALPHA_A_BOX),
        #[allow(deprecated)]
        "dev-to" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dev-to' is deprecated.").print(py);
            }
            Some(icons::DEV_TO)
        }
        "circle-box-outline" => Some(icons::CIRCLE_BOX_OUTLINE),
        "file-video-outline" => Some(icons::FILE_VIDEO_OUTLINE),
        "weather-lightning-rainy" => Some(icons::WEATHER_LIGHTNING_RAINY),
        "battery-90-bluetooth" => Some(icons::BATTERY_90_BLUETOOTH),
        "toaster-off" => Some(icons::TOASTER_OFF),
        "list-box" => Some(icons::LIST_BOX),
        "timer" => Some(icons::TIMER),
        "leaf" => Some(icons::LEAF),
        "message-flash-outline" => Some(icons::MESSAGE_FLASH_OUTLINE),
        "leak-off" => Some(icons::LEAK_OFF),
        "alpha-b-circle-outline" => Some(icons::ALPHA_B_CIRCLE_OUTLINE),
        "flashlight" => Some(icons::FLASHLIGHT),
        "message-outline" => Some(icons::MESSAGE_OUTLINE),
        "account-box" => Some(icons::ACCOUNT_BOX),
        "tag-text" => Some(icons::TAG_TEXT),
        "rss-box" => Some(icons::RSS_BOX),
        "flask-minus" => Some(icons::FLASK_MINUS),
        "axis-z-arrow-lock" => Some(icons::AXIS_Z_ARROW_LOCK),
        "gate-alert" => Some(icons::GATE_ALERT),
        "clock-time-twelve-outline" => Some(icons::CLOCK_TIME_TWELVE_OUTLINE),
        "map-marker-remove-variant" => Some(icons::MAP_MARKER_REMOVE_VARIANT),
        "access-point-off" => Some(icons::ACCESS_POINT_OFF),
        "camera-flip" => Some(icons::CAMERA_FLIP),
        "crop" => Some(icons::CROP),
        "redo-variant" => Some(icons::REDO_VARIANT),
        "lightbulb-cfl-spiral" => Some(icons::LIGHTBULB_CFL_SPIRAL),
        "dharmachakra" => Some(icons::DHARMACHAKRA),
        "border-vertical" => Some(icons::BORDER_VERTICAL),
        "note-minus-outline" => Some(icons::NOTE_MINUS_OUTLINE),
        "swap-horizontal" => Some(icons::SWAP_HORIZONTAL),
        "dumbbell" => Some(icons::DUMBBELL),
        "timer-lock-open" => Some(icons::TIMER_LOCK_OPEN),
        "blender" => Some(icons::BLENDER),
        "seed-off-outline" => Some(icons::SEED_OFF_OUTLINE),
        "awning" => Some(icons::AWNING),
        #[allow(deprecated)]
        "terraform" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'terraform' is deprecated.").print(py);
            }
            Some(icons::TERRAFORM)
        }
        "numeric-4" => Some(icons::NUMERIC_4),
        "percent-circle" => Some(icons::PERCENT_CIRCLE),
        "floor-lamp-dual" => Some(icons::FLOOR_LAMP_DUAL),
        "card-account-details-star-outline" => Some(icons::CARD_ACCOUNT_DETAILS_STAR_OUTLINE),
        "clipboard-text-search" => Some(icons::CLIPBOARD_TEXT_SEARCH),
        "panorama-variant-outline" => Some(icons::PANORAMA_VARIANT_OUTLINE),
        "database-refresh-outline" => Some(icons::DATABASE_REFRESH_OUTLINE),
        "head-snowflake-outline" => Some(icons::HEAD_SNOWFLAKE_OUTLINE),
        "book-open-page-variant-outline" => Some(icons::BOOK_OPEN_PAGE_VARIANT_OUTLINE),
        "swap-vertical" => Some(icons::SWAP_VERTICAL),
        "network-off" => Some(icons::NETWORK_OFF),
        "lock-check-outline" => Some(icons::LOCK_CHECK_OUTLINE),
        "music-box-multiple" => Some(icons::MUSIC_BOX_MULTIPLE),
        "message-settings" => Some(icons::MESSAGE_SETTINGS),
        "egg-easter" => Some(icons::EGG_EASTER),
        "cat" => Some(icons::CAT),
        "camera-account" => Some(icons::CAMERA_ACCOUNT),
        "comment-plus-outline" => Some(icons::COMMENT_PLUS_OUTLINE),
        "arm-flex-outline" => Some(icons::ARM_FLEX_OUTLINE),
        "leaf-circle" => Some(icons::LEAF_CIRCLE),
        "bottle-soda-classic-outline" => Some(icons::BOTTLE_SODA_CLASSIC_OUTLINE),
        "mower" => Some(icons::MOWER),
        "clipboard-pulse-outline" => Some(icons::CLIPBOARD_PULSE_OUTLINE),
        "strategy" => Some(icons::STRATEGY),
        "backspace-reverse-outline" => Some(icons::BACKSPACE_REVERSE_OUTLINE),
        "share-variant" => Some(icons::SHARE_VARIANT),
        "movie-open-plus" => Some(icons::MOVIE_OPEN_PLUS),
        "file-code-outline" => Some(icons::FILE_CODE_OUTLINE),
        "elevation-rise" => Some(icons::ELEVATION_RISE),
        "close-box-multiple-outline" => Some(icons::CLOSE_BOX_MULTIPLE_OUTLINE),
        "menu-down" => Some(icons::MENU_DOWN),
        "calendar-month-outline" => Some(icons::CALENDAR_MONTH_OUTLINE),
        "invoice-arrow-left-outline" => Some(icons::INVOICE_ARROW_LEFT_OUTLINE),
        "lock-percent-open-variant-outline" => Some(icons::LOCK_PERCENT_OPEN_VARIANT_OUTLINE),
        "soldering-iron" => Some(icons::SOLDERING_IRON),
        "radiator-disabled" => Some(icons::RADIATOR_DISABLED),
        "volume-high" => Some(icons::VOLUME_HIGH),
        "sitemap-outline" => Some(icons::SITEMAP_OUTLINE),
        "plus-network" => Some(icons::PLUS_NETWORK),
        "hospital-building" => Some(icons::HOSPITAL_BUILDING),
        "thermostat-auto" => Some(icons::THERMOSTAT_AUTO),
        "power-socket" => Some(icons::POWER_SOCKET),
        "button-pointer" => Some(icons::BUTTON_POINTER),
        "shoe-print" => Some(icons::SHOE_PRINT),
        "message-flash" => Some(icons::MESSAGE_FLASH),
        "home-off-outline" => Some(icons::HOME_OFF_OUTLINE),
        "truck-delivery-outline" => Some(icons::TRUCK_DELIVERY_OUTLINE),
        "power-plug-battery" => Some(icons::POWER_PLUG_BATTERY),
        "flag-variant-minus-outline" => Some(icons::FLAG_VARIANT_MINUS_OUTLINE),
        "vector-point-edit" => Some(icons::VECTOR_POINT_EDIT),
        "star-outline" => Some(icons::STAR_OUTLINE),
        "calendar-cursor" => Some(icons::CALENDAR_CURSOR),
        "code-block-tags" => Some(icons::CODE_BLOCK_TAGS),
        "wifi-strength-3-lock" => Some(icons::WIFI_STRENGTH_3_LOCK),
        "thermometer-off" => Some(icons::THERMOMETER_OFF),
        "phone-dial" => Some(icons::PHONE_DIAL),
        "car-outline" => Some(icons::CAR_OUTLINE),
        #[allow(deprecated)]
        "google-cloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cloud' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CLOUD)
        }
        "texture" => Some(icons::TEXTURE),
        #[allow(deprecated)]
        "qi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qi' is deprecated.").print(py);
            }
            Some(icons::QI)
        }
        "content-save-all-outline" => Some(icons::CONTENT_SAVE_ALL_OUTLINE),
        "source-branch-check" => Some(icons::SOURCE_BRANCH_CHECK),
        "gesture-tap" => Some(icons::GESTURE_TAP),
        "swap-vertical-variant" => Some(icons::SWAP_VERTICAL_VARIANT),
        "shield-half-full" => Some(icons::SHIELD_HALF_FULL),
        "cursor-default-gesture" => Some(icons::CURSOR_DEFAULT_GESTURE),
        "card-multiple" => Some(icons::CARD_MULTIPLE),
        "head-check-outline" => Some(icons::HEAD_CHECK_OUTLINE),
        "battery-charging-wireless-alert" => Some(icons::BATTERY_CHARGING_WIRELESS_ALERT),
        "calendar-blank-multiple" => Some(icons::CALENDAR_BLANK_MULTIPLE),
        "clock-check" => Some(icons::CLOCK_CHECK),
        "calendar-month" => Some(icons::CALENDAR_MONTH),
        "backspace" => Some(icons::BACKSPACE),
        "pentagram" => Some(icons::PENTAGRAM),
        "content-save-cog-outline" => Some(icons::CONTENT_SAVE_COG_OUTLINE),
        "pencil-circle-outline" => Some(icons::PENCIL_CIRCLE_OUTLINE),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "card-bulleted" => Some(icons::CARD_BULLETED),
        "source-commit-next-local" => Some(icons::SOURCE_COMMIT_NEXT_LOCAL),
        "size-l" => Some(icons::SIZE_L),
        "store-24-hour" => Some(icons::STORE_24_HOUR),
        "tooltip-question" => Some(icons::TOOLTIP_QUESTION),
        "slope-uphill" => Some(icons::SLOPE_UPHILL),
        "store-remove-outline" => Some(icons::STORE_REMOVE_OUTLINE),
        "wifi-cancel" => Some(icons::WIFI_CANCEL),
        "emoticon-excited" => Some(icons::EMOTICON_EXCITED),
        "upload-outline" => Some(icons::UPLOAD_OUTLINE),
        "briefcase-minus" => Some(icons::BRIEFCASE_MINUS),
        #[allow(deprecated)]
        "youtube-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-gaming' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_GAMING)
        }
        "security-network" => Some(icons::SECURITY_NETWORK),
        _ => None,
    }
}
