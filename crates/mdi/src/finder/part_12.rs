// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_12(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "emoticon-excited" => Some(icons::EMOTICON_EXCITED),
        "moon-full" => Some(icons::MOON_FULL),
        "draw" => Some(icons::DRAW),
        "home-group" => Some(icons::HOME_GROUP),
        "newspaper-variant" => Some(icons::NEWSPAPER_VARIANT),
        "owl" => Some(icons::OWL),
        "format-header-increase" => Some(icons::FORMAT_HEADER_INCREASE),
        "tray" => Some(icons::TRAY),
        "bed-queen-outline" => Some(icons::BED_QUEEN_OUTLINE),
        "content-save-settings-outline" => Some(icons::CONTENT_SAVE_SETTINGS_OUTLINE),
        "note-text" => Some(icons::NOTE_TEXT),
        "printer-pos-wrench" => Some(icons::PRINTER_POS_WRENCH),
        "checkerboard-minus" => Some(icons::CHECKERBOARD_MINUS),
        "math-tan" => Some(icons::MATH_TAN),
        "tag-hidden" => Some(icons::TAG_HIDDEN),
        "crosshairs-off" => Some(icons::CROSSHAIRS_OFF),
        "printer-pos-pause" => Some(icons::PRINTER_POS_PAUSE),
        "wifi-sync" => Some(icons::WIFI_SYNC),
        "shield-alert-outline" => Some(icons::SHIELD_ALERT_OUTLINE),
        "select-multiple-marker" => Some(icons::SELECT_MULTIPLE_MARKER),
        #[allow(deprecated)]
        "stack-overflow" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-overflow' is deprecated.").print(py);
            }
            Some(icons::STACK_OVERFLOW)
        }
        "dharmachakra" => Some(icons::DHARMACHAKRA),
        "arrow-left-box" => Some(icons::ARROW_LEFT_BOX),
        "file-png-box" => Some(icons::FILE_PNG_BOX),
        "refresh" => Some(icons::REFRESH),
        "map-marker-question" => Some(icons::MAP_MARKER_QUESTION),
        #[allow(deprecated)]
        "graphql" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'graphql' is deprecated.").print(py);
            }
            Some(icons::GRAPHQL)
        }
        "leaf-circle" => Some(icons::LEAF_CIRCLE),
        "identifier" => Some(icons::IDENTIFIER),
        "school-outline" => Some(icons::SCHOOL_OUTLINE),
        "account-reactivate-outline" => Some(icons::ACCOUNT_REACTIVATE_OUTLINE),
        "thermostat-cog" => Some(icons::THERMOSTAT_COG),
        "calendar-star-outline" => Some(icons::CALENDAR_STAR_OUTLINE),
        "shield-bug" => Some(icons::SHIELD_BUG),
        "source-branch-refresh" => Some(icons::SOURCE_BRANCH_REFRESH),
        "drag-horizontal-variant" => Some(icons::DRAG_HORIZONTAL_VARIANT),
        "store-remove-outline" => Some(icons::STORE_REMOVE_OUTLINE),
        "chat-processing-outline" => Some(icons::CHAT_PROCESSING_OUTLINE),
        "comma" => Some(icons::COMMA),
        "roller-shade-closed" => Some(icons::ROLLER_SHADE_CLOSED),
        "water-remove-outline" => Some(icons::WATER_REMOVE_OUTLINE),
        "ipod" => Some(icons::IPOD),
        "arrow-top-left-thin" => Some(icons::ARROW_TOP_LEFT_THIN),
        #[allow(deprecated)]
        "google-plus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-plus' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLUS)
        }
        "parking" => Some(icons::PARKING),
        "credit-card-settings" => Some(icons::CREDIT_CARD_SETTINGS),
        "clover" => Some(icons::CLOVER),
        "power-plug-off-outline" => Some(icons::POWER_PLUG_OFF_OUTLINE),
        "border-top-variant" => Some(icons::BORDER_TOP_VARIANT),
        "comment-text-outline" => Some(icons::COMMENT_TEXT_OUTLINE),
        "numeric-6-circle" => Some(icons::NUMERIC_6_CIRCLE),
        "briefcase-eye" => Some(icons::BRIEFCASE_EYE),
        "format-list-bulleted-triangle" => Some(icons::FORMAT_LIST_BULLETED_TRIANGLE),
        "store-clock-outline" => Some(icons::STORE_CLOCK_OUTLINE),
        #[allow(deprecated)]
        "google-podcast" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-podcast' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PODCAST)
        }
        "truck-remove-outline" => Some(icons::TRUCK_REMOVE_OUTLINE),
        "qrcode-scan" => Some(icons::QRCODE_SCAN),
        #[allow(deprecated)]
        "pandora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pandora' is deprecated.").print(py);
            }
            Some(icons::PANDORA)
        }
        #[allow(deprecated)]
        "dolby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dolby' is deprecated.").print(py);
            }
            Some(icons::DOLBY)
        }
        "layers-plus" => Some(icons::LAYERS_PLUS),
        "cards-playing-spade-multiple-outline" => Some(icons::CARDS_PLAYING_SPADE_MULTIPLE_OUTLINE),
        "account-multiple-plus" => Some(icons::ACCOUNT_MULTIPLE_PLUS),
        "receipt-text-plus" => Some(icons::RECEIPT_TEXT_PLUS),
        "walk" => Some(icons::WALK),
        "thermometer-alert" => Some(icons::THERMOMETER_ALERT),
        "alpha-c-box" => Some(icons::ALPHA_C_BOX),
        "thermometer-chevron-down" => Some(icons::THERMOMETER_CHEVRON_DOWN),
        "tag-heart-outline" => Some(icons::TAG_HEART_OUTLINE),
        "backspace-reverse-outline" => Some(icons::BACKSPACE_REVERSE_OUTLINE),
        "bell-circle" => Some(icons::BELL_CIRCLE),
        "cake" => Some(icons::CAKE),
        "square-circle" => Some(icons::SQUARE_CIRCLE),
        "train-car-hopper" => Some(icons::TRAIN_CAR_HOPPER),
        #[allow(deprecated)]
        "black-mesa" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'black-mesa' is deprecated.").print(py);
            }
            Some(icons::BLACK_MESA)
        }
        "car-traction-control" => Some(icons::CAR_TRACTION_CONTROL),
        "battery-unknown-bluetooth" => Some(icons::BATTERY_UNKNOWN_BLUETOOTH),
        "bell-outline" => Some(icons::BELL_OUTLINE),
        "mushroom-off-outline" => Some(icons::MUSHROOM_OFF_OUTLINE),
        "cabin-a-frame" => Some(icons::CABIN_A_FRAME),
        "phone-settings" => Some(icons::PHONE_SETTINGS),
        "align-horizontal-distribute" => Some(icons::ALIGN_HORIZONTAL_DISTRIBUTE),
        "video-input-svideo" => Some(icons::VIDEO_INPUT_SVIDEO),
        "shoe-formal" => Some(icons::SHOE_FORMAL),
        "robot-vacuum-variant" => Some(icons::ROBOT_VACUUM_VARIANT),
        "head-snowflake" => Some(icons::HEAD_SNOWFLAKE),
        "format-clear" => Some(icons::FORMAT_CLEAR),
        "alpha-x" => Some(icons::ALPHA_X),
        "web-sync" => Some(icons::WEB_SYNC),
        "table-split-cell" => Some(icons::TABLE_SPLIT_CELL),
        "music-note-whole-dotted" => Some(icons::MUSIC_NOTE_WHOLE_DOTTED),
        "sign-pole" => Some(icons::SIGN_POLE),
        "account-lock-open-outline" => Some(icons::ACCOUNT_LOCK_OPEN_OUTLINE),
        "debug-step-into" => Some(icons::DEBUG_STEP_INTO),
        "invoice-text-arrow-right-outline" => Some(icons::INVOICE_TEXT_ARROW_RIGHT_OUTLINE),
        "folder-move-outline" => Some(icons::FOLDER_MOVE_OUTLINE),
        "table-check" => Some(icons::TABLE_CHECK),
        "grid-large" => Some(icons::GRID_LARGE),
        #[allow(deprecated)]
        "vuetify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuetify' is deprecated.").print(py);
            }
            Some(icons::VUETIFY)
        }
        "door-open" => Some(icons::DOOR_OPEN),
        "dns" => Some(icons::DNS),
        "code-block-braces" => Some(icons::CODE_BLOCK_BRACES),
        "tune-vertical" => Some(icons::TUNE_VERTICAL),
        "book-plus-outline" => Some(icons::BOOK_PLUS_OUTLINE),
        "swap-horizontal-circle" => Some(icons::SWAP_HORIZONTAL_CIRCLE),
        "hand-back-left" => Some(icons::HAND_BACK_LEFT),
        "music-clef-treble" => Some(icons::MUSIC_CLEF_TREBLE),
        "folder-arrow-up-outline" => Some(icons::FOLDER_ARROW_UP_OUTLINE),
        "elevator-passenger-off" => Some(icons::ELEVATOR_PASSENGER_OFF),
        #[allow(deprecated)]
        "kubernetes" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kubernetes' is deprecated.").print(py);
            }
            Some(icons::KUBERNETES)
        }
        "diameter-outline" => Some(icons::DIAMETER_OUTLINE),
        "wifi-strength-2-alert" => Some(icons::WIFI_STRENGTH_2_ALERT),
        "led-variant-on" => Some(icons::LED_VARIANT_ON),
        "hand-pointing-up" => Some(icons::HAND_POINTING_UP),
        "arrow-horizontal-lock" => Some(icons::ARROW_HORIZONTAL_LOCK),
        "signal-cellular-2" => Some(icons::SIGNAL_CELLULAR_2),
        "face-man-shimmer" => Some(icons::FACE_MAN_SHIMMER),
        "toggle-switch-off" => Some(icons::TOGGLE_SWITCH_OFF),
        "arrow-left-thin-circle-outline" => Some(icons::ARROW_LEFT_THIN_CIRCLE_OUTLINE),
        "flashlight" => Some(icons::FLASHLIGHT),
        "sail-boat" => Some(icons::SAIL_BOAT),
        "account-tie-voice" => Some(icons::ACCOUNT_TIE_VOICE),
        "circle-slice-8" => Some(icons::CIRCLE_SLICE_8),
        "cloud-download" => Some(icons::CLOUD_DOWNLOAD),
        "expansion-card-variant" => Some(icons::EXPANSION_CARD_VARIANT),
        "source-commit-end-local" => Some(icons::SOURCE_COMMIT_END_LOCAL),
        "emoticon-plus-outline" => Some(icons::EMOTICON_PLUS_OUTLINE),
        "sofa-single" => Some(icons::SOFA_SINGLE),
        "broom" => Some(icons::BROOM),
        "land-plots-marker" => Some(icons::LAND_PLOTS_MARKER),
        "label-percent" => Some(icons::LABEL_PERCENT),
        #[allow(deprecated)]
        "language-csharp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-csharp' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_CSHARP)
        }
        "car-clock" => Some(icons::CAR_CLOCK),
        "comment-question-outline" => Some(icons::COMMENT_QUESTION_OUTLINE),
        "close-network" => Some(icons::CLOSE_NETWORK),
        "weather-sunset" => Some(icons::WEATHER_SUNSET),
        #[allow(deprecated)]
        "cordova" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cordova' is deprecated.").print(py);
            }
            Some(icons::CORDOVA)
        }
        "pulse" => Some(icons::PULSE),
        "focus-field" => Some(icons::FOCUS_FIELD),
        "folder-arrow-left" => Some(icons::FOLDER_ARROW_LEFT),
        "movie-off-outline" => Some(icons::MOVIE_OFF_OUTLINE),
        "puzzle-heart-outline" => Some(icons::PUZZLE_HEART_OUTLINE),
        "circular-saw" => Some(icons::CIRCULAR_SAW),
        "ray-start-vertex-end" => Some(icons::RAY_START_VERTEX_END),
        "content-save-outline" => Some(icons::CONTENT_SAVE_OUTLINE),
        #[allow(deprecated)]
        "spotify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'spotify' is deprecated.").print(py);
            }
            Some(icons::SPOTIFY)
        }
        "alert-circle" => Some(icons::ALERT_CIRCLE),
        "rotate-left-variant" => Some(icons::ROTATE_LEFT_VARIANT),
        "om" => Some(icons::OM),
        "pot-steam-outline" => Some(icons::POT_STEAM_OUTLINE),
        "robot-vacuum" => Some(icons::ROBOT_VACUUM),
        "alpha-s-circle-outline" => Some(icons::ALPHA_S_CIRCLE_OUTLINE),
        "gesture-two-double-tap" => Some(icons::GESTURE_TWO_DOUBLE_TAP),
        "email-alert" => Some(icons::EMAIL_ALERT),
        "checkbox-blank" => Some(icons::CHECKBOX_BLANK),
        "camera-control" => Some(icons::CAMERA_CONTROL),
        "map-marker-right" => Some(icons::MAP_MARKER_RIGHT),
        "cast-education" => Some(icons::CAST_EDUCATION),
        "message-text-lock-outline" => Some(icons::MESSAGE_TEXT_LOCK_OUTLINE),
        "label" => Some(icons::LABEL),
        "gender-transgender" => Some(icons::GENDER_TRANSGENDER),
        "account-multiple-minus-outline" => Some(icons::ACCOUNT_MULTIPLE_MINUS_OUTLINE),
        "texture" => Some(icons::TEXTURE),
        "checkbox-blank-badge-outline" => Some(icons::CHECKBOX_BLANK_BADGE_OUTLINE),
        "location-exit" => Some(icons::LOCATION_EXIT),
        "garage-open" => Some(icons::GARAGE_OPEN),
        "bike-pedal-mountain" => Some(icons::BIKE_PEDAL_MOUNTAIN),
        "printer-pos-refresh" => Some(icons::PRINTER_POS_REFRESH),
        "battery-charging-outline" => Some(icons::BATTERY_CHARGING_OUTLINE),
        "rewind-outline" => Some(icons::REWIND_OUTLINE),
        "playlist-remove" => Some(icons::PLAYLIST_REMOVE),
        "code-less-than-or-equal" => Some(icons::CODE_LESS_THAN_OR_EQUAL),
        "view-day" => Some(icons::VIEW_DAY),
        "cards-playing-diamond-multiple" => Some(icons::CARDS_PLAYING_DIAMOND_MULTIPLE),
        "blinds" => Some(icons::BLINDS),
        "bone" => Some(icons::BONE),
        "tag-check" => Some(icons::TAG_CHECK),
        "database-search-outline" => Some(icons::DATABASE_SEARCH_OUTLINE),
        "circle-box-outline" => Some(icons::CIRCLE_BOX_OUTLINE),
        "relation-many-to-only-one" => Some(icons::RELATION_MANY_TO_ONLY_ONE),
        "alpha-a-circle-outline" => Some(icons::ALPHA_A_CIRCLE_OUTLINE),
        "sun-angle-outline" => Some(icons::SUN_ANGLE_OUTLINE),
        "reiterate" => Some(icons::REITERATE),
        "link-plus" => Some(icons::LINK_PLUS),
        "credit-card-scan" => Some(icons::CREDIT_CARD_SCAN),
        "share-all-outline" => Some(icons::SHARE_ALL_OUTLINE),
        "alien-outline" => Some(icons::ALIEN_OUTLINE),
        "warehouse" => Some(icons::WAREHOUSE),
        "battery-charging-70" => Some(icons::BATTERY_CHARGING_70),
        "database-clock" => Some(icons::DATABASE_CLOCK),
        "map-marker-alert-outline" => Some(icons::MAP_MARKER_ALERT_OUTLINE),
        "clock-alert-outline" => Some(icons::CLOCK_ALERT_OUTLINE),
        "read" => Some(icons::READ),
        "alpha-n-circle-outline" => Some(icons::ALPHA_N_CIRCLE_OUTLINE),
        "face-mask-outline" => Some(icons::FACE_MASK_OUTLINE),
        "monitor-arrow-down" => Some(icons::MONITOR_ARROW_DOWN),
        "shield-home-outline" => Some(icons::SHIELD_HOME_OUTLINE),
        "human-male-female" => Some(icons::HUMAN_MALE_FEMALE),
        "fence-electric" => Some(icons::FENCE_ELECTRIC),
        "shield-key-outline" => Some(icons::SHIELD_KEY_OUTLINE),
        "progress-wrench" => Some(icons::PROGRESS_WRENCH),
        _ => None,
    }
}
