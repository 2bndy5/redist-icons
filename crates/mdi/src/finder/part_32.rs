// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_32(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "briefcase-arrow-left-right-outline" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT_OUTLINE),
        #[allow(deprecated)]
        "sony-playstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sony-playstation' is deprecated.")
                    .print(py);
            }
            Some(icons::SONY_PLAYSTATION)
        }
        "file-link-outline" => Some(icons::FILE_LINK_OUTLINE),
        "wifi-lock-open" => Some(icons::WIFI_LOCK_OPEN),
        "skip-next" => Some(icons::SKIP_NEXT),
        "beaker-question-outline" => Some(icons::BEAKER_QUESTION_OUTLINE),
        "lotion" => Some(icons::LOTION),
        "star-face" => Some(icons::STAR_FACE),
        "shield-lock" => Some(icons::SHIELD_LOCK),
        "alpha-z" => Some(icons::ALPHA_Z),
        "exit-run" => Some(icons::EXIT_RUN),
        "numeric-0-box-outline" => Some(icons::NUMERIC_0_BOX_OUTLINE),
        "liquor" => Some(icons::LIQUOR),
        "audio-video-off" => Some(icons::AUDIO_VIDEO_OFF),
        "record-player" => Some(icons::RECORD_PLAYER),
        #[allow(deprecated)]
        "google-circles-extended" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles-extended' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_EXTENDED)
        }
        "monitor" => Some(icons::MONITOR),
        "cards-heart" => Some(icons::CARDS_HEART),
        "alpha-i-circle-outline" => Some(icons::ALPHA_I_CIRCLE_OUTLINE),
        "moon-first-quarter" => Some(icons::MOON_FIRST_QUARTER),
        #[allow(deprecated)]
        "nuxt" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nuxt' is deprecated.").print(py);
            }
            Some(icons::NUXT)
        }
        "battery-charging-50" => Some(icons::BATTERY_CHARGING_50),
        "cash-fast" => Some(icons::CASH_FAST),
        "ladder" => Some(icons::LADDER),
        "gamepad-down" => Some(icons::GAMEPAD_DOWN),
        "lan-check" => Some(icons::LAN_CHECK),
        "garage-lock" => Some(icons::GARAGE_LOCK),
        "gamepad-round" => Some(icons::GAMEPAD_ROUND),
        "greenhouse" => Some(icons::GREENHOUSE),
        "link-circle" => Some(icons::LINK_CIRCLE),
        "view-split-horizontal" => Some(icons::VIEW_SPLIT_HORIZONTAL),
        "folder-sync" => Some(icons::FOLDER_SYNC),
        "chevron-triple-right" => Some(icons::CHEVRON_TRIPLE_RIGHT),
        "numeric-5-circle-outline" => Some(icons::NUMERIC_5_CIRCLE_OUTLINE),
        "guitar-pick-outline" => Some(icons::GUITAR_PICK_OUTLINE),
        "movie-open-edit-outline" => Some(icons::MOVIE_OPEN_EDIT_OUTLINE),
        "file-link" => Some(icons::FILE_LINK),
        "arrange-bring-to-front" => Some(icons::ARRANGE_BRING_TO_FRONT),
        "campfire" => Some(icons::CAMPFIRE),
        "text-box-check-outline" => Some(icons::TEXT_BOX_CHECK_OUTLINE),
        "human-male-boy" => Some(icons::HUMAN_MALE_BOY),
        "speedometer" => Some(icons::SPEEDOMETER),
        "account-switch" => Some(icons::ACCOUNT_SWITCH),
        "flashlight-off" => Some(icons::FLASHLIGHT_OFF),
        "briefcase-arrow-up-down" => Some(icons::BRIEFCASE_ARROW_UP_DOWN),
        "roller-skate" => Some(icons::ROLLER_SKATE),
        "cylinder" => Some(icons::CYLINDER),
        "switch" => Some(icons::SWITCH),
        "arrow-decision-auto-outline" => Some(icons::ARROW_DECISION_AUTO_OUTLINE),
        "music-rest-sixteenth" => Some(icons::MUSIC_REST_SIXTEENTH),
        "store-check" => Some(icons::STORE_CHECK),
        "wallet-plus" => Some(icons::WALLET_PLUS),
        "car-shift-pattern" => Some(icons::CAR_SHIFT_PATTERN),
        "folder-remove-outline" => Some(icons::FOLDER_REMOVE_OUTLINE),
        "egg-outline" => Some(icons::EGG_OUTLINE),
        "robot-vacuum-off" => Some(icons::ROBOT_VACUUM_OFF),
        "weather-cloudy-clock" => Some(icons::WEATHER_CLOUDY_CLOCK),
        "timeline-outline" => Some(icons::TIMELINE_OUTLINE),
        "music-accidental-double-sharp" => Some(icons::MUSIC_ACCIDENTAL_DOUBLE_SHARP),
        "transit-skip" => Some(icons::TRANSIT_SKIP),
        "badminton" => Some(icons::BADMINTON),
        "sort-numeric-ascending-variant" => Some(icons::SORT_NUMERIC_ASCENDING_VARIANT),
        "printer-3d" => Some(icons::PRINTER_3D),
        #[allow(deprecated)]
        "language-typescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-typescript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_TYPESCRIPT)
        }
        "not-equal-variant" => Some(icons::NOT_EQUAL_VARIANT),
        "cloud-arrow-right" => Some(icons::CLOUD_ARROW_RIGHT),
        "door-sliding" => Some(icons::DOOR_SLIDING),
        "bell-plus" => Some(icons::BELL_PLUS),
        "office-building" => Some(icons::OFFICE_BUILDING),
        "download-box" => Some(icons::DOWNLOAD_BOX),
        "temple-buddhist-outline" => Some(icons::TEMPLE_BUDDHIST_OUTLINE),
        "source-branch-check" => Some(icons::SOURCE_BRANCH_CHECK),
        "notification-clear-all" => Some(icons::NOTIFICATION_CLEAR_ALL),
        "mushroom-outline" => Some(icons::MUSHROOM_OUTLINE),
        "download-box-outline" => Some(icons::DOWNLOAD_BOX_OUTLINE),
        "home-floor-b" => Some(icons::HOME_FLOOR_B),
        "tag-arrow-down" => Some(icons::TAG_ARROW_DOWN),
        "battery-charging-wireless-outline" => Some(icons::BATTERY_CHARGING_WIRELESS_OUTLINE),
        "cloud-off" => Some(icons::CLOUD_OFF),
        "monitor-lock" => Some(icons::MONITOR_LOCK),
        "home-plus-outline" => Some(icons::HOME_PLUS_OUTLINE),
        "file-minus-outline" => Some(icons::FILE_MINUS_OUTLINE),
        "store-remove" => Some(icons::STORE_REMOVE),
        "hydro-power" => Some(icons::HYDRO_POWER),
        "reply-all" => Some(icons::REPLY_ALL),
        "earth-box-remove" => Some(icons::EARTH_BOX_REMOVE),
        "image-refresh" => Some(icons::IMAGE_REFRESH),
        "emoticon-sad-outline" => Some(icons::EMOTICON_SAD_OUTLINE),
        "cash-100" => Some(icons::CASH_100),
        "file-replace" => Some(icons::FILE_REPLACE),
        #[allow(deprecated)]
        "stack-overflow" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-overflow' is deprecated.").print(py);
            }
            Some(icons::STACK_OVERFLOW)
        }
        "moon-full" => Some(icons::MOON_FULL),
        "movie-off-outline" => Some(icons::MOVIE_OFF_OUTLINE),
        "bag-personal-off" => Some(icons::BAG_PERSONAL_OFF),
        "delete-off-outline" => Some(icons::DELETE_OFF_OUTLINE),
        "network-strength-1-alert" => Some(icons::NETWORK_STRENGTH_1_ALERT),
        "compare-horizontal" => Some(icons::COMPARE_HORIZONTAL),
        "checkbox-marked-circle" => Some(icons::CHECKBOX_MARKED_CIRCLE),
        "calendar-range-outline" => Some(icons::CALENDAR_RANGE_OUTLINE),
        "numeric-2-box-outline" => Some(icons::NUMERIC_2_BOX_OUTLINE),
        "forum-plus" => Some(icons::FORUM_PLUS),
        "shield-cross" => Some(icons::SHIELD_CROSS),
        "roller-shade" => Some(icons::ROLLER_SHADE),
        "tag-heart-outline" => Some(icons::TAG_HEART_OUTLINE),
        "hand-extended" => Some(icons::HAND_EXTENDED),
        "alert-remove" => Some(icons::ALERT_REMOVE),
        "cellphone-check" => Some(icons::CELLPHONE_CHECK),
        "ellipse-outline" => Some(icons::ELLIPSE_OUTLINE),
        "thermometer-minus" => Some(icons::THERMOMETER_MINUS),
        "dog-service" => Some(icons::DOG_SERVICE),
        "lotion-plus" => Some(icons::LOTION_PLUS),
        "arrow-decision-auto" => Some(icons::ARROW_DECISION_AUTO),
        "food-drumstick-outline" => Some(icons::FOOD_DRUMSTICK_OUTLINE),
        "arrow-down" => Some(icons::ARROW_DOWN),
        "train-variant" => Some(icons::TRAIN_VARIANT),
        "car-brake-fluid-level" => Some(icons::CAR_BRAKE_FLUID_LEVEL),
        "arrow-split-vertical" => Some(icons::ARROW_SPLIT_VERTICAL),
        "map-marker-multiple" => Some(icons::MAP_MARKER_MULTIPLE),
        "invoice" => Some(icons::INVOICE),
        "food-takeout-box" => Some(icons::FOOD_TAKEOUT_BOX),
        "numeric-6-box" => Some(icons::NUMERIC_6_BOX),
        "flip-to-front" => Some(icons::FLIP_TO_FRONT),
        "account-box-plus-outline" => Some(icons::ACCOUNT_BOX_PLUS_OUTLINE),
        "server-network-off" => Some(icons::SERVER_NETWORK_OFF),
        "all-inclusive-box" => Some(icons::ALL_INCLUSIVE_BOX),
        "sheep" => Some(icons::SHEEP),
        "battery-charging-40" => Some(icons::BATTERY_CHARGING_40),
        "generator-stationary" => Some(icons::GENERATOR_STATIONARY),
        "pencil-minus" => Some(icons::PENCIL_MINUS),
        "lock-percent-open-outline" => Some(icons::LOCK_PERCENT_OPEN_OUTLINE),
        "karate" => Some(icons::KARATE),
        "toggle-switch-variant-off" => Some(icons::TOGGLE_SWITCH_VARIANT_OFF),
        "data-matrix-remove" => Some(icons::DATA_MATRIX_REMOVE),
        "music-box-outline" => Some(icons::MUSIC_BOX_OUTLINE),
        "calendar-weekend" => Some(icons::CALENDAR_WEEKEND),
        "alpha-i" => Some(icons::ALPHA_I),
        "weather-fog" => Some(icons::WEATHER_FOG),
        "lambda" => Some(icons::LAMBDA),
        "bullseye-arrow" => Some(icons::BULLSEYE_ARROW),
        "arrow-u-up-right-bold" => Some(icons::ARROW_U_UP_RIGHT_BOLD),
        "sticker-check-outline" => Some(icons::STICKER_CHECK_OUTLINE),
        "blinds-horizontal-closed" => Some(icons::BLINDS_HORIZONTAL_CLOSED),
        #[allow(deprecated)]
        "google-downasaur" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-downasaur' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_DOWNASAUR)
        }
        "account-file" => Some(icons::ACCOUNT_FILE),
        "notebook-heart-outline" => Some(icons::NOTEBOOK_HEART_OUTLINE),
        "application-brackets" => Some(icons::APPLICATION_BRACKETS),
        "fruit-grapes" => Some(icons::FRUIT_GRAPES),
        "ski-water" => Some(icons::SKI_WATER),
        #[allow(deprecated)]
        "electron-framework" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'electron-framework' is deprecated.")
                    .print(py);
            }
            Some(icons::ELECTRON_FRAMEWORK)
        }
        "arrow-left-bold-outline" => Some(icons::ARROW_LEFT_BOLD_OUTLINE),
        "steering" => Some(icons::STEERING),
        "fullscreen" => Some(icons::FULLSCREEN),
        "map-check-outline" => Some(icons::MAP_CHECK_OUTLINE),
        "scanner-off" => Some(icons::SCANNER_OFF),
        "briefcase-remove-outline" => Some(icons::BRIEFCASE_REMOVE_OUTLINE),
        "button-pointer" => Some(icons::BUTTON_POINTER),
        "microphone-off" => Some(icons::MICROPHONE_OFF),
        "folder-eye" => Some(icons::FOLDER_EYE),
        "face-recognition" => Some(icons::FACE_RECOGNITION),
        "toothbrush" => Some(icons::TOOTHBRUSH),
        "raw-off" => Some(icons::RAW_OFF),
        "video-check" => Some(icons::VIDEO_CHECK),
        "cookie-settings-outline" => Some(icons::COOKIE_SETTINGS_OUTLINE),
        "home-remove-outline" => Some(icons::HOME_REMOVE_OUTLINE),
        "led-variant-on" => Some(icons::LED_VARIANT_ON),
        "vacuum-outline" => Some(icons::VACUUM_OUTLINE),
        "tooltip-check" => Some(icons::TOOLTIP_CHECK),
        "passport-biometric" => Some(icons::PASSPORT_BIOMETRIC),
        "axe" => Some(icons::AXE),
        "thermometer-high" => Some(icons::THERMOMETER_HIGH),
        "rolodex" => Some(icons::ROLODEX),
        "bowling" => Some(icons::BOWLING),
        "fountain-pen-tip" => Some(icons::FOUNTAIN_PEN_TIP),
        "arrow-up-bold" => Some(icons::ARROW_UP_BOLD),
        "thermometer-probe-off" => Some(icons::THERMOMETER_PROBE_OFF),
        "timer-sand" => Some(icons::TIMER_SAND),
        "tooltip-check-outline" => Some(icons::TOOLTIP_CHECK_OUTLINE),
        "airplane-search" => Some(icons::AIRPLANE_SEARCH),
        "tag-check" => Some(icons::TAG_CHECK),
        "align-horizontal-left" => Some(icons::ALIGN_HORIZONTAL_LEFT),
        "checkbox-blank-off-outline" => Some(icons::CHECKBOX_BLANK_OFF_OUTLINE),
        "format-header-6" => Some(icons::FORMAT_HEADER_6),
        "cloud-alert" => Some(icons::CLOUD_ALERT),
        "arrow-top-left-thin-circle-outline" => Some(icons::ARROW_TOP_LEFT_THIN_CIRCLE_OUTLINE),
        "cards-playing-club-outline" => Some(icons::CARDS_PLAYING_CLUB_OUTLINE),
        "comment-alert" => Some(icons::COMMENT_ALERT),
        "arrow-top-right-bold-box" => Some(icons::ARROW_TOP_RIGHT_BOLD_BOX),
        "chevron-up-box-outline" => Some(icons::CHEVRON_UP_BOX_OUTLINE),
        "taxi" => Some(icons::TAXI),
        "cow" => Some(icons::COW),
        "format-list-checkbox" => Some(icons::FORMAT_LIST_CHECKBOX),
        "hands-pray" => Some(icons::HANDS_PRAY),
        "numeric-5-circle" => Some(icons::NUMERIC_5_CIRCLE),
        "battery-minus-variant" => Some(icons::BATTERY_MINUS_VARIANT),
        "bacteria-outline" => Some(icons::BACTERIA_OUTLINE),
        "minus-box-multiple-outline" => Some(icons::MINUS_BOX_MULTIPLE_OUTLINE),
        "battery-charging-70" => Some(icons::BATTERY_CHARGING_70),
        "pot" => Some(icons::POT),
        "french-fries" => Some(icons::FRENCH_FRIES),
        "server-outline" => Some(icons::SERVER_OUTLINE),
        _ => None,
    }
}
