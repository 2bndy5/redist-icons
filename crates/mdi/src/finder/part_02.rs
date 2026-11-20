// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_2(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "watch-import" => Some(icons::WATCH_IMPORT),
        "bag-personal-plus" => Some(icons::BAG_PERSONAL_PLUS),
        "cast-variant" => Some(icons::CAST_VARIANT),
        "motion-sensor-off" => Some(icons::MOTION_SENSOR_OFF),
        "table-multiple" => Some(icons::TABLE_MULTIPLE),
        "script-text-play-outline" => Some(icons::SCRIPT_TEXT_PLAY_OUTLINE),
        "gesture-tap" => Some(icons::GESTURE_TAP),
        "map-marker-minus" => Some(icons::MAP_MARKER_MINUS),
        "book-settings-outline" => Some(icons::BOOK_SETTINGS_OUTLINE),
        "timer-off-outline" => Some(icons::TIMER_OFF_OUTLINE),
        "car-door-lock" => Some(icons::CAR_DOOR_LOCK),
        "comment" => Some(icons::COMMENT),
        "presentation-play" => Some(icons::PRESENTATION_PLAY),
        "hand-heart-outline" => Some(icons::HAND_HEART_OUTLINE),
        "file-image-minus-outline" => Some(icons::FILE_IMAGE_MINUS_OUTLINE),
        "numeric-6-box-multiple" => Some(icons::NUMERIC_6_BOX_MULTIPLE),
        "blur-linear" => Some(icons::BLUR_LINEAR),
        "robot-excited" => Some(icons::ROBOT_EXCITED),
        "train-car-flatbed-tank" => Some(icons::TRAIN_CAR_FLATBED_TANK),
        "inbox-arrow-down" => Some(icons::INBOX_ARROW_DOWN),
        "arrow-u-up-right" => Some(icons::ARROW_U_UP_RIGHT),
        "eye-check" => Some(icons::EYE_CHECK),
        "gesture-spread" => Some(icons::GESTURE_SPREAD),
        "charity" => Some(icons::CHARITY),
        "vibrate" => Some(icons::VIBRATE),
        "glass-mug" => Some(icons::GLASS_MUG),
        "image-filter-center-focus-strong" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG),
        "shopping" => Some(icons::SHOPPING),
        "angle-acute" => Some(icons::ANGLE_ACUTE),
        "invoice-text-edit-outline" => Some(icons::INVOICE_TEXT_EDIT_OUTLINE),
        "axis-x-rotate-clockwise" => Some(icons::AXIS_X_ROTATE_CLOCKWISE),
        "folder-lock-outline" => Some(icons::FOLDER_LOCK_OUTLINE),
        "tab-unselected" => Some(icons::TAB_UNSELECTED),
        "tunnel-outline" => Some(icons::TUNNEL_OUTLINE),
        "calendar-collapse-horizontal" => Some(icons::CALENDAR_COLLAPSE_HORIZONTAL),
        "test-tube-off" => Some(icons::TEST_TUBE_OFF),
        "cellphone-sound" => Some(icons::CELLPHONE_SOUND),
        "calendar" => Some(icons::CALENDAR),
        "floor-lamp-dual" => Some(icons::FLOOR_LAMP_DUAL),
        "emoticon-plus" => Some(icons::EMOTICON_PLUS),
        "webhook" => Some(icons::WEBHOOK),
        "faucet" => Some(icons::FAUCET),
        "elevator" => Some(icons::ELEVATOR),
        "folder-remove" => Some(icons::FOLDER_REMOVE),
        "alpha-o-circle-outline" => Some(icons::ALPHA_O_CIRCLE_OUTLINE),
        "umbrella-beach" => Some(icons::UMBRELLA_BEACH),
        "sticker-minus-outline" => Some(icons::STICKER_MINUS_OUTLINE),
        "clipboard-off-outline" => Some(icons::CLIPBOARD_OFF_OUTLINE),
        "credit-card-scan" => Some(icons::CREDIT_CARD_SCAN),
        "archive-star-outline" => Some(icons::ARCHIVE_STAR_OUTLINE),
        "shaker" => Some(icons::SHAKER),
        "satellite-variant" => Some(icons::SATELLITE_VARIANT),
        "file-arrow-left-right" => Some(icons::FILE_ARROW_LEFT_RIGHT),
        "car-brake-temperature" => Some(icons::CAR_BRAKE_TEMPERATURE),
        "hook-off" => Some(icons::HOOK_OFF),
        "circle-double" => Some(icons::CIRCLE_DOUBLE),
        "battery-clock-outline" => Some(icons::BATTERY_CLOCK_OUTLINE),
        "file-chart-check" => Some(icons::FILE_CHART_CHECK),
        "shield-check" => Some(icons::SHIELD_CHECK),
        "truck-alert-outline" => Some(icons::TRUCK_ALERT_OUTLINE),
        "lock-percent-open" => Some(icons::LOCK_PERCENT_OPEN),
        "cellphone-nfc-off" => Some(icons::CELLPHONE_NFC_OFF),
        "magnify-minus" => Some(icons::MAGNIFY_MINUS),
        "shield-star" => Some(icons::SHIELD_STAR),
        "battery-charging-50" => Some(icons::BATTERY_CHARGING_50),
        "seat-outline" => Some(icons::SEAT_OUTLINE),
        "bell-off" => Some(icons::BELL_OFF),
        "checkbox-blank-circle-outline" => Some(icons::CHECKBOX_BLANK_CIRCLE_OUTLINE),
        "mixed-martial-arts" => Some(icons::MIXED_MARTIAL_ARTS),
        "roman-numeral-3" => Some(icons::ROMAN_NUMERAL_3),
        "loupe" => Some(icons::LOUPE),
        "cookie-alert" => Some(icons::COOKIE_ALERT),
        "home-floor-0" => Some(icons::HOME_FLOOR_0),
        "ballot-recount" => Some(icons::BALLOT_RECOUNT),
        "transit-connection-variant" => Some(icons::TRANSIT_CONNECTION_VARIANT),
        "file-hidden" => Some(icons::FILE_HIDDEN),
        "filter-minus" => Some(icons::FILTER_MINUS),
        "shore" => Some(icons::SHORE),
        "hexagon-outline" => Some(icons::HEXAGON_OUTLINE),
        "directions" => Some(icons::DIRECTIONS),
        "keyboard-f2" => Some(icons::KEYBOARD_F2),
        "alpha-m-circle" => Some(icons::ALPHA_M_CIRCLE),
        "movie-open-star" => Some(icons::MOVIE_OPEN_STAR),
        "home-alert" => Some(icons::HOME_ALERT),
        "database-outline" => Some(icons::DATABASE_OUTLINE),
        "screw-lag" => Some(icons::SCREW_LAG),
        "trash-can" => Some(icons::TRASH_CAN),
        "phone-forward-outline" => Some(icons::PHONE_FORWARD_OUTLINE),
        "elephant" => Some(icons::ELEPHANT),
        #[allow(deprecated)]
        "google-circles-extended" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles-extended' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_EXTENDED)
        }
        "cast-connected" => Some(icons::CAST_CONNECTED),
        "radar" => Some(icons::RADAR),
        "seal" => Some(icons::SEAL),
        "gesture-tap-button" => Some(icons::GESTURE_TAP_BUTTON),
        "video-plus" => Some(icons::VIDEO_PLUS),
        "hydraulic-oil-temperature" => Some(icons::HYDRAULIC_OIL_TEMPERATURE),
        "arrow-bottom-left-thick" => Some(icons::ARROW_BOTTOM_LEFT_THICK),
        "folder-zip" => Some(icons::FOLDER_ZIP),
        "weather-tornado" => Some(icons::WEATHER_TORNADO),
        "movie-open-check-outline" => Some(icons::MOVIE_OPEN_CHECK_OUTLINE),
        "cloud-arrow-down" => Some(icons::CLOUD_ARROW_DOWN),
        "map-search" => Some(icons::MAP_SEARCH),
        "clock-remove-outline" => Some(icons::CLOCK_REMOVE_OUTLINE),
        "passport-check" => Some(icons::PASSPORT_CHECK),
        "file-tree" => Some(icons::FILE_TREE),
        "train-car-gondola" => Some(icons::TRAIN_CAR_GONDOLA),
        "music-note-off-outline" => Some(icons::MUSIC_NOTE_OFF_OUTLINE),
        "chevron-down-box-outline" => Some(icons::CHEVRON_DOWN_BOX_OUTLINE),
        "star-box-outline" => Some(icons::STAR_BOX_OUTLINE),
        "keyboard-f10" => Some(icons::KEYBOARD_F10),
        "format-text-rotation-vertical" => Some(icons::FORMAT_TEXT_ROTATION_VERTICAL),
        "bank-circle-outline" => Some(icons::BANK_CIRCLE_OUTLINE),
        "ribbon" => Some(icons::RIBBON),
        "delete-alert-outline" => Some(icons::DELETE_ALERT_OUTLINE),
        "timer-lock-open-outline" => Some(icons::TIMER_LOCK_OPEN_OUTLINE),
        "diving-helmet" => Some(icons::DIVING_HELMET),
        #[allow(deprecated)]
        "microsoft-teams" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-teams' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_TEAMS)
        }
        "gas-station-outline" => Some(icons::GAS_STATION_OUTLINE),
        "dog-side" => Some(icons::DOG_SIDE),
        "share-off-outline" => Some(icons::SHARE_OFF_OUTLINE),
        "shield-home-outline" => Some(icons::SHIELD_HOME_OUTLINE),
        "briefcase-upload-outline" => Some(icons::BRIEFCASE_UPLOAD_OUTLINE),
        "database-export-outline" => Some(icons::DATABASE_EXPORT_OUTLINE),
        "fire-truck" => Some(icons::FIRE_TRUCK),
        "cached" => Some(icons::CACHED),
        #[allow(deprecated)]
        "quora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'quora' is deprecated.").print(py);
            }
            Some(icons::QUORA)
        }
        "kettle-off" => Some(icons::KETTLE_OFF),
        "flower-tulip" => Some(icons::FLOWER_TULIP),
        "waveform" => Some(icons::WAVEFORM),
        "account-child-circle" => Some(icons::ACCOUNT_CHILD_CIRCLE),
        "pot-mix" => Some(icons::POT_MIX),
        #[allow(deprecated)]
        "language-php" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-php' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_PHP)
        }
        "phone-forward" => Some(icons::PHONE_FORWARD),
        "video-switch" => Some(icons::VIDEO_SWITCH),
        "weather-moonset-down" => Some(icons::WEATHER_MOONSET_DOWN),
        "locker-multiple" => Some(icons::LOCKER_MULTIPLE),
        "clock-remove" => Some(icons::CLOCK_REMOVE),
        "chart-multiline" => Some(icons::CHART_MULTILINE),
        "book-heart" => Some(icons::BOOK_HEART),
        "calendar-sync-outline" => Some(icons::CALENDAR_SYNC_OUTLINE),
        "script-text-play" => Some(icons::SCRIPT_TEXT_PLAY),
        "flash" => Some(icons::FLASH),
        "car-light-fog" => Some(icons::CAR_LIGHT_FOG),
        #[allow(deprecated)]
        "language-xaml" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-xaml' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_XAML)
        }
        "shopping-search" => Some(icons::SHOPPING_SEARCH),
        "leaf" => Some(icons::LEAF),
        "elevator-passenger-off-outline" => Some(icons::ELEVATOR_PASSENGER_OFF_OUTLINE),
        "fast-forward-60" => Some(icons::FAST_FORWARD_60),
        "message-text-clock" => Some(icons::MESSAGE_TEXT_CLOCK),
        "tag-minus" => Some(icons::TAG_MINUS),
        "folder-star-multiple" => Some(icons::FOLDER_STAR_MULTIPLE),
        "arrow-down-bold-circle-outline" => Some(icons::ARROW_DOWN_BOLD_CIRCLE_OUTLINE),
        "bicycle-electric" => Some(icons::BICYCLE_ELECTRIC),
        "tooltip-image-outline" => Some(icons::TOOLTIP_IMAGE_OUTLINE),
        "format-align-right" => Some(icons::FORMAT_ALIGN_RIGHT),
        "square-outline" => Some(icons::SQUARE_OUTLINE),
        "video-check-outline" => Some(icons::VIDEO_CHECK_OUTLINE),
        "ab-testing" => Some(icons::AB_TESTING),
        "flag-variant-outline" => Some(icons::FLAG_VARIANT_OUTLINE),
        "shape-plus-outline" => Some(icons::SHAPE_PLUS_OUTLINE),
        "card-account-mail" => Some(icons::CARD_ACCOUNT_MAIL),
        "skip-next-circle-outline" => Some(icons::SKIP_NEXT_CIRCLE_OUTLINE),
        "battery-50-bluetooth" => Some(icons::BATTERY_50_BLUETOOTH),
        "lifebuoy" => Some(icons::LIFEBUOY),
        "cat" => Some(icons::CAT),
        "border-vertical" => Some(icons::BORDER_VERTICAL),
        "gamepad-round-down" => Some(icons::GAMEPAD_ROUND_DOWN),
        "file-table-box" => Some(icons::FILE_TABLE_BOX),
        "point-of-sale" => Some(icons::POINT_OF_SALE),
        "seat-flat-angled" => Some(icons::SEAT_FLAT_ANGLED),
        "arrow-right-thin-circle-outline" => Some(icons::ARROW_RIGHT_THIN_CIRCLE_OUTLINE),
        "tooltip-edit-outline" => Some(icons::TOOLTIP_EDIT_OUTLINE),
        "border-left-variant" => Some(icons::BORDER_LEFT_VARIANT),
        "battery-charging" => Some(icons::BATTERY_CHARGING),
        "spirit-level" => Some(icons::SPIRIT_LEVEL),
        "animation-play-outline" => Some(icons::ANIMATION_PLAY_OUTLINE),
        "briefcase-variant-off-outline" => Some(icons::BRIEFCASE_VARIANT_OFF_OUTLINE),
        "folder-file-outline" => Some(icons::FOLDER_FILE_OUTLINE),
        "chair-rolling" => Some(icons::CHAIR_ROLLING),
        "glass-cocktail" => Some(icons::GLASS_COCKTAIL),
        "microphone-variant-off" => Some(icons::MICROPHONE_VARIANT_OFF),
        "badge-account-horizontal" => Some(icons::BADGE_ACCOUNT_HORIZONTAL),
        "alert-octagram-outline" => Some(icons::ALERT_OCTAGRAM_OUTLINE),
        "lamp-outline" => Some(icons::LAMP_OUTLINE),
        "alert-rhombus" => Some(icons::ALERT_RHOMBUS),
        "trending-neutral" => Some(icons::TRENDING_NEUTRAL),
        "car-brake-low-pressure" => Some(icons::CAR_BRAKE_LOW_PRESSURE),
        "shark-fin-outline" => Some(icons::SHARK_FIN_OUTLINE),
        "alpha-m-box" => Some(icons::ALPHA_M_BOX),
        "file-eye-outline" => Some(icons::FILE_EYE_OUTLINE),
        "pentagon-outline" => Some(icons::PENTAGON_OUTLINE),
        "calendar-start-outline" => Some(icons::CALENDAR_START_OUTLINE),
        "relation-zero-or-one-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ZERO_OR_ONE),
        "gesture-swipe-right" => Some(icons::GESTURE_SWIPE_RIGHT),
        "ticket-account" => Some(icons::TICKET_ACCOUNT),
        "arrow-bottom-right-bold-outline" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_OUTLINE),
        "safe-square" => Some(icons::SAFE_SQUARE),
        "audio-input-xlr" => Some(icons::AUDIO_INPUT_XLR),
        "format-wrap-tight" => Some(icons::FORMAT_WRAP_TIGHT),
        "image-size-select-large" => Some(icons::IMAGE_SIZE_SELECT_LARGE),
        _ => None,
    }
}
