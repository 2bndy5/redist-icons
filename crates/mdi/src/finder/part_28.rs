// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_28(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "video-check" => Some(icons::VIDEO_CHECK),
        "filter-menu" => Some(icons::FILTER_MENU),
        "power-settings" => Some(icons::POWER_SETTINGS),
        "phone-refresh" => Some(icons::PHONE_REFRESH),
        "tilde-off" => Some(icons::TILDE_OFF),
        "chevron-left-box-outline" => Some(icons::CHEVRON_LEFT_BOX_OUTLINE),
        "screwdriver" => Some(icons::SCREWDRIVER),
        "cookie-lock" => Some(icons::COOKIE_LOCK),
        "help-network" => Some(icons::HELP_NETWORK),
        "table-row-plus-after" => Some(icons::TABLE_ROW_PLUS_AFTER),
        "smoke-detector-alert" => Some(icons::SMOKE_DETECTOR_ALERT),
        "fire-hydrant-alert" => Some(icons::FIRE_HYDRANT_ALERT),
        "battery-charging-wireless-80" => Some(icons::BATTERY_CHARGING_WIRELESS_80),
        "transmission-tower" => Some(icons::TRANSMISSION_TOWER),
        "carabiner" => Some(icons::CARABINER),
        "dice-d6" => Some(icons::DICE_D6),
        "human-edit" => Some(icons::HUMAN_EDIT),
        "nut" => Some(icons::NUT),
        "arrow-bottom-right-bold-box-outline" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX_OUTLINE),
        "grid" => Some(icons::GRID),
        "numeric-4-box-multiple-outline" => Some(icons::NUMERIC_4_BOX_MULTIPLE_OUTLINE),
        "numeric-2-box-outline" => Some(icons::NUMERIC_2_BOX_OUTLINE),
        "storefront-outline" => Some(icons::STOREFRONT_OUTLINE),
        "gate-xnor" => Some(icons::GATE_XNOR),
        "weather-cloudy-arrow-right" => Some(icons::WEATHER_CLOUDY_ARROW_RIGHT),
        "tally-mark-5" => Some(icons::TALLY_MARK_5),
        "soy-sauce" => Some(icons::SOY_SAUCE),
        "car-seat-cooler" => Some(icons::CAR_SEAT_COOLER),
        "rotate-right" => Some(icons::ROTATE_RIGHT),
        #[allow(deprecated)]
        "microsoft-xbox-controller" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER)
        }
        "sun-snowflake-variant" => Some(icons::SUN_SNOWFLAKE_VARIANT),
        "folder-open-outline" => Some(icons::FOLDER_OPEN_OUTLINE),
        "toggle-switch" => Some(icons::TOGGLE_SWITCH),
        "glass-mug-variant-off" => Some(icons::GLASS_MUG_VARIANT_OFF),
        "archive-star-outline" => Some(icons::ARCHIVE_STAR_OUTLINE),
        "alarm-off" => Some(icons::ALARM_OFF),
        "wifi-strength-2-lock" => Some(icons::WIFI_STRENGTH_2_LOCK),
        "relation-zero-or-many-to-one-or-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE_OR_MANY),
        "lightbulb-variant" => Some(icons::LIGHTBULB_VARIANT),
        "face-woman-profile" => Some(icons::FACE_WOMAN_PROFILE),
        "mosque" => Some(icons::MOSQUE),
        "ruler" => Some(icons::RULER),
        #[allow(deprecated)]
        "youtube-subscription" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-subscription' is deprecated.")
                    .print(py);
            }
            Some(icons::YOUTUBE_SUBSCRIPTION)
        }
        "folder-heart-outline" => Some(icons::FOLDER_HEART_OUTLINE),
        "weather-sunny-off" => Some(icons::WEATHER_SUNNY_OFF),
        "flash-off" => Some(icons::FLASH_OFF),
        "account-tie-voice-off-outline" => Some(icons::ACCOUNT_TIE_VOICE_OFF_OUTLINE),
        "amplifier" => Some(icons::AMPLIFIER),
        "comment-search-outline" => Some(icons::COMMENT_SEARCH_OUTLINE),
        "fence-electric" => Some(icons::FENCE_ELECTRIC),
        "monitor-off" => Some(icons::MONITOR_OFF),
        "bomb-off" => Some(icons::BOMB_OFF),
        "power-socket-it" => Some(icons::POWER_SOCKET_IT),
        "chevron-down-box-outline" => Some(icons::CHEVRON_DOWN_BOX_OUTLINE),
        "message-outline" => Some(icons::MESSAGE_OUTLINE),
        #[allow(deprecated)]
        "microsoft-excel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-excel' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_EXCEL)
        }
        "lifebuoy" => Some(icons::LIFEBUOY),
        "image-filter-center-focus-strong" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG),
        "bunk-bed-outline" => Some(icons::BUNK_BED_OUTLINE),
        "relation-many-to-one-or-many" => Some(icons::RELATION_MANY_TO_ONE_OR_MANY),
        "tag" => Some(icons::TAG),
        "router-network" => Some(icons::ROUTER_NETWORK),
        "music-accidental-sharp" => Some(icons::MUSIC_ACCIDENTAL_SHARP),
        "phone-alert-outline" => Some(icons::PHONE_ALERT_OUTLINE),
        #[allow(deprecated)]
        "opera" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'opera' is deprecated.").print(py);
            }
            Some(icons::OPERA)
        }
        "format-letter-case-lower" => Some(icons::FORMAT_LETTER_CASE_LOWER),
        "eiffel-tower" => Some(icons::EIFFEL_TOWER),
        "tag-search-outline" => Some(icons::TAG_SEARCH_OUTLINE),
        "rewind" => Some(icons::REWIND),
        "water-pump" => Some(icons::WATER_PUMP),
        "help-rhombus" => Some(icons::HELP_RHOMBUS),
        "elevation-rise" => Some(icons::ELEVATION_RISE),
        "guitar-electric" => Some(icons::GUITAR_ELECTRIC),
        "text-box-search-outline" => Some(icons::TEXT_BOX_SEARCH_OUTLINE),
        "robot-vacuum-alert" => Some(icons::ROBOT_VACUUM_ALERT),
        "tennis" => Some(icons::TENNIS),
        "water-plus" => Some(icons::WATER_PLUS),
        "account-switch-outline" => Some(icons::ACCOUNT_SWITCH_OUTLINE),
        #[allow(deprecated)]
        "microsoft-windows" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-windows' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS)
        }
        "file-cog" => Some(icons::FILE_COG),
        "printer-pos-sync" => Some(icons::PRINTER_POS_SYNC),
        "office-building-remove" => Some(icons::OFFICE_BUILDING_REMOVE),
        "account-tag" => Some(icons::ACCOUNT_TAG),
        "lightbulb-off" => Some(icons::LIGHTBULB_OFF),
        "account-filter" => Some(icons::ACCOUNT_FILTER),
        "home-sound-out" => Some(icons::HOME_SOUND_OUT),
        "battery-heart-outline" => Some(icons::BATTERY_HEART_OUTLINE),
        "contactless-payment" => Some(icons::CONTACTLESS_PAYMENT),
        "battery-minus-outline" => Some(icons::BATTERY_MINUS_OUTLINE),
        "arrow-left-bold-hexagon-outline" => Some(icons::ARROW_LEFT_BOLD_HEXAGON_OUTLINE),
        "triangle-outline" => Some(icons::TRIANGLE_OUTLINE),
        "arrow-left-bold" => Some(icons::ARROW_LEFT_BOLD),
        "autorenew" => Some(icons::AUTORENEW),
        "folder-arrow-up-outline" => Some(icons::FOLDER_ARROW_UP_OUTLINE),
        "alert-circle-check-outline" => Some(icons::ALERT_CIRCLE_CHECK_OUTLINE),
        "shopping" => Some(icons::SHOPPING),
        "battery-lock-open" => Some(icons::BATTERY_LOCK_OPEN),
        "file-certificate" => Some(icons::FILE_CERTIFICATE),
        "hand-heart-outline" => Some(icons::HAND_HEART_OUTLINE),
        "hook-off" => Some(icons::HOOK_OFF),
        "truck-trailer" => Some(icons::TRUCK_TRAILER),
        "network-strength-2-alert" => Some(icons::NETWORK_STRENGTH_2_ALERT),
        "playlist-plus" => Some(icons::PLAYLIST_PLUS),
        "view-agenda-outline" => Some(icons::VIEW_AGENDA_OUTLINE),
        "layers" => Some(icons::LAYERS),
        "scale" => Some(icons::SCALE),
        "account-sync-outline" => Some(icons::ACCOUNT_SYNC_OUTLINE),
        "temple-hindu" => Some(icons::TEMPLE_HINDU),
        "hockey-sticks" => Some(icons::HOCKEY_STICKS),
        "emoticon-happy" => Some(icons::EMOTICON_HAPPY),
        "message-text-clock" => Some(icons::MESSAGE_TEXT_CLOCK),
        "cellphone-settings" => Some(icons::CELLPHONE_SETTINGS),
        "account-alert" => Some(icons::ACCOUNT_ALERT),
        "file-restore" => Some(icons::FILE_RESTORE),
        "fish-off" => Some(icons::FISH_OFF),
        "rewind-outline" => Some(icons::REWIND_OUTLINE),
        "treasure-chest-outline" => Some(icons::TREASURE_CHEST_OUTLINE),
        "surround-sound-5-1-2" => Some(icons::SURROUND_SOUND_5_1_2),
        "relation-only-one-to-zero-or-one" => Some(icons::RELATION_ONLY_ONE_TO_ZERO_OR_ONE),
        "arrow-u-up-right-bold" => Some(icons::ARROW_U_UP_RIGHT_BOLD),
        "comment-remove-outline" => Some(icons::COMMENT_REMOVE_OUTLINE),
        "timer-edit-outline" => Some(icons::TIMER_EDIT_OUTLINE),
        "invoice-text-plus-outline" => Some(icons::INVOICE_TEXT_PLUS_OUTLINE),
        "storefront-check" => Some(icons::STOREFRONT_CHECK),
        "alpha-b-box" => Some(icons::ALPHA_B_BOX),
        "lock-off" => Some(icons::LOCK_OFF),
        "gesture-two-double-tap" => Some(icons::GESTURE_TWO_DOUBLE_TAP),
        "file-music-outline" => Some(icons::FILE_MUSIC_OUTLINE),
        "battery-minus-variant" => Some(icons::BATTERY_MINUS_VARIANT),
        "sort-variant-remove" => Some(icons::SORT_VARIANT_REMOVE),
        "select-search" => Some(icons::SELECT_SEARCH),
        "mailbox" => Some(icons::MAILBOX),
        "file-edit-outline" => Some(icons::FILE_EDIT_OUTLINE),
        "crown-circle-outline" => Some(icons::CROWN_CIRCLE_OUTLINE),
        "subdirectory-arrow-right" => Some(icons::SUBDIRECTORY_ARROW_RIGHT),
        "television-shimmer" => Some(icons::TELEVISION_SHIMMER),
        "alpha-c-box-outline" => Some(icons::ALPHA_C_BOX_OUTLINE),
        "emoticon-kiss-outline" => Some(icons::EMOTICON_KISS_OUTLINE),
        "lightbulb-auto-outline" => Some(icons::LIGHTBULB_AUTO_OUTLINE),
        "cellphone-screenshot" => Some(icons::CELLPHONE_SCREENSHOT),
        "folder-information" => Some(icons::FOLDER_INFORMATION),
        "propane-tank-outline" => Some(icons::PROPANE_TANK_OUTLINE),
        "bookmark-remove" => Some(icons::BOOKMARK_REMOVE),
        "stool" => Some(icons::STOOL),
        #[allow(deprecated)]
        "google-street-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-street-view' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_STREET_VIEW)
        }
        "rewind-15" => Some(icons::REWIND_15),
        "account-question-outline" => Some(icons::ACCOUNT_QUESTION_OUTLINE),
        "battery-charging-wireless-outline" => Some(icons::BATTERY_CHARGING_WIRELESS_OUTLINE),
        "star-remove-outline" => Some(icons::STAR_REMOVE_OUTLINE),
        "file-link" => Some(icons::FILE_LINK),
        "test-tube-empty" => Some(icons::TEST_TUBE_EMPTY),
        "map-marker-down" => Some(icons::MAP_MARKER_DOWN),
        "account-supervisor" => Some(icons::ACCOUNT_SUPERVISOR),
        "bluetooth-transfer" => Some(icons::BLUETOOTH_TRANSFER),
        "arrow-right-bold" => Some(icons::ARROW_RIGHT_BOLD),
        "map-marker-account-outline" => Some(icons::MAP_MARKER_ACCOUNT_OUTLINE),
        "page-next-outline" => Some(icons::PAGE_NEXT_OUTLINE),
        "music-note-eighth" => Some(icons::MUSIC_NOTE_EIGHTH),
        "car-back" => Some(icons::CAR_BACK),
        "ray-start" => Some(icons::RAY_START),
        "file-arrow-up-down" => Some(icons::FILE_ARROW_UP_DOWN),
        "home-modern" => Some(icons::HOME_MODERN),
        "pin" => Some(icons::PIN),
        "format-wrap-tight" => Some(icons::FORMAT_WRAP_TIGHT),
        #[allow(deprecated)]
        "origin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'origin' is deprecated.").print(py);
            }
            Some(icons::ORIGIN)
        }
        "focus-field" => Some(icons::FOCUS_FIELD),
        "align-horizontal-left" => Some(icons::ALIGN_HORIZONTAL_LEFT),
        "power-plug-battery" => Some(icons::POWER_PLUG_BATTERY),
        "chevron-triple-right" => Some(icons::CHEVRON_TRIPLE_RIGHT),
        "calendar-plus" => Some(icons::CALENDAR_PLUS),
        "clock-time-eleven-outline" => Some(icons::CLOCK_TIME_ELEVEN_OUTLINE),
        "note-alert" => Some(icons::NOTE_ALERT),
        "regex" => Some(icons::REGEX),
        "alpha-c" => Some(icons::ALPHA_C),
        "briefcase-arrow-left-right" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT),
        "package-down" => Some(icons::PACKAGE_DOWN),
        "ocr" => Some(icons::OCR),
        "rabbit" => Some(icons::RABBIT),
        "knob" => Some(icons::KNOB),
        "clipboard-text-outline" => Some(icons::CLIPBOARD_TEXT_OUTLINE),
        "account-multiple-plus-outline" => Some(icons::ACCOUNT_MULTIPLE_PLUS_OUTLINE),
        "quality-low" => Some(icons::QUALITY_LOW),
        "music-note" => Some(icons::MUSIC_NOTE),
        "umbrella-beach" => Some(icons::UMBRELLA_BEACH),
        "layers-edit" => Some(icons::LAYERS_EDIT),
        "calendar-range" => Some(icons::CALENDAR_RANGE),
        "web-refresh" => Some(icons::WEB_REFRESH),
        "shield-refresh" => Some(icons::SHIELD_REFRESH),
        "weather-sunset-up" => Some(icons::WEATHER_SUNSET_UP),
        "cup" => Some(icons::CUP),
        "incognito-off" => Some(icons::INCOGNITO_OFF),
        "invoice-text-minus-outline" => Some(icons::INVOICE_TEXT_MINUS_OUTLINE),
        "weather-cloudy-alert" => Some(icons::WEATHER_CLOUDY_ALERT),
        "arrow-down-thick" => Some(icons::ARROW_DOWN_THICK),
        "share-outline" => Some(icons::SHARE_OUTLINE),
        "battery-heart-variant" => Some(icons::BATTERY_HEART_VARIANT),
        "tent" => Some(icons::TENT),
        "zodiac-virgo" => Some(icons::ZODIAC_VIRGO),
        "briefcase-clock" => Some(icons::BRIEFCASE_CLOCK),
        "shield-remove-outline" => Some(icons::SHIELD_REMOVE_OUTLINE),
        _ => None,
    }
}
