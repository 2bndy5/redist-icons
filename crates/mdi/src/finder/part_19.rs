// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_19(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "integrated-circuit-chip" => Some(icons::INTEGRATED_CIRCUIT_CHIP),
        "tow-truck" => Some(icons::TOW_TRUCK),
        "quality-medium" => Some(icons::QUALITY_MEDIUM),
        "redo" => Some(icons::REDO),
        "account-file-outline" => Some(icons::ACCOUNT_FILE_OUTLINE),
        "directions" => Some(icons::DIRECTIONS),
        "camera-retake-outline" => Some(icons::CAMERA_RETAKE_OUTLINE),
        "headphones-box" => Some(icons::HEADPHONES_BOX),
        "bookmark-off-outline" => Some(icons::BOOKMARK_OFF_OUTLINE),
        "arrow-right-thin-circle-outline" => Some(icons::ARROW_RIGHT_THIN_CIRCLE_OUTLINE),
        "car-cog" => Some(icons::CAR_COG),
        "weather-snowy" => Some(icons::WEATHER_SNOWY),
        "car-wrench" => Some(icons::CAR_WRENCH),
        "briefcase-edit" => Some(icons::BRIEFCASE_EDIT),
        "file-find-outline" => Some(icons::FILE_FIND_OUTLINE),
        "shape-square-plus" => Some(icons::SHAPE_SQUARE_PLUS),
        "minus-thick" => Some(icons::MINUS_THICK),
        "moon-full" => Some(icons::MOON_FULL),
        "account-search" => Some(icons::ACCOUNT_SEARCH),
        "checkbox-marked" => Some(icons::CHECKBOX_MARKED),
        "arrow-collapse-up" => Some(icons::ARROW_COLLAPSE_UP),
        #[allow(deprecated)]
        "microsoft-excel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-excel' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_EXCEL)
        }
        "relation-zero-or-one-to-one-or-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE_OR_MANY),
        "water-minus-outline" => Some(icons::WATER_MINUS_OUTLINE),
        "code-string" => Some(icons::CODE_STRING),
        "message-reply-text-outline" => Some(icons::MESSAGE_REPLY_TEXT_OUTLINE),
        "pig-variant-outline" => Some(icons::PIG_VARIANT_OUTLINE),
        "image-remove-outline" => Some(icons::IMAGE_REMOVE_OUTLINE),
        "form-textbox-password" => Some(icons::FORM_TEXTBOX_PASSWORD),
        "calendar-blank-multiple" => Some(icons::CALENDAR_BLANK_MULTIPLE),
        "emoticon-devil-outline" => Some(icons::EMOTICON_DEVIL_OUTLINE),
        "chart-donut-variant" => Some(icons::CHART_DONUT_VARIANT),
        "led-on" => Some(icons::LED_ON),
        "lock-check" => Some(icons::LOCK_CHECK),
        "exclamation" => Some(icons::EXCLAMATION),
        "card-bulleted-settings" => Some(icons::CARD_BULLETED_SETTINGS),
        "all-inclusive" => Some(icons::ALL_INCLUSIVE),
        "backburger" => Some(icons::BACKBURGER),
        "earth-box-off" => Some(icons::EARTH_BOX_OFF),
        "message-draw" => Some(icons::MESSAGE_DRAW),
        #[allow(deprecated)]
        "opera" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'opera' is deprecated.").print(py);
            }
            Some(icons::OPERA)
        }
        "timer-sync-outline" => Some(icons::TIMER_SYNC_OUTLINE),
        "basket-remove-outline" => Some(icons::BASKET_REMOVE_OUTLINE),
        "store-plus" => Some(icons::STORE_PLUS),
        "lamps" => Some(icons::LAMPS),
        "book-music" => Some(icons::BOOK_MUSIC),
        "download-off" => Some(icons::DOWNLOAD_OFF),
        "tooltip-check-outline" => Some(icons::TOOLTIP_CHECK_OUTLINE),
        "printer-3d-nozzle-heat" => Some(icons::PRINTER_3D_NOZZLE_HEAT),
        "bag-suitcase-off-outline" => Some(icons::BAG_SUITCASE_OFF_OUTLINE),
        "file-swap-outline" => Some(icons::FILE_SWAP_OUTLINE),
        "data-matrix-plus" => Some(icons::DATA_MATRIX_PLUS),
        "alpha-f-box" => Some(icons::ALPHA_F_BOX),
        "peanut-off-outline" => Some(icons::PEANUT_OFF_OUTLINE),
        "bench" => Some(icons::BENCH),
        "radiator" => Some(icons::RADIATOR),
        "message-processing" => Some(icons::MESSAGE_PROCESSING),
        "cards-spade-outline" => Some(icons::CARDS_SPADE_OUTLINE),
        "gesture-double-tap" => Some(icons::GESTURE_DOUBLE_TAP),
        "cast-off" => Some(icons::CAST_OFF),
        "phone-cancel-outline" => Some(icons::PHONE_CANCEL_OUTLINE),
        "spotlight-beam" => Some(icons::SPOTLIGHT_BEAM),
        "kettle" => Some(icons::KETTLE),
        "email-arrow-right" => Some(icons::EMAIL_ARROW_RIGHT),
        "state-machine" => Some(icons::STATE_MACHINE),
        "file-image-outline" => Some(icons::FILE_IMAGE_OUTLINE),
        "car-brake-fluid-level" => Some(icons::CAR_BRAKE_FLUID_LEVEL),
        "airplane-cog" => Some(icons::AIRPLANE_COG),
        "puzzle-heart" => Some(icons::PUZZLE_HEART),
        "folder-refresh-outline" => Some(icons::FOLDER_REFRESH_OUTLINE),
        "comma-box" => Some(icons::COMMA_BOX),
        "lightbulb-on-70" => Some(icons::LIGHTBULB_ON_70),
        "amplifier" => Some(icons::AMPLIFIER),
        "backspace-outline" => Some(icons::BACKSPACE_OUTLINE),
        "key-arrow-right" => Some(icons::KEY_ARROW_RIGHT),
        "pail-remove" => Some(icons::PAIL_REMOVE),
        "credit-card-clock-outline" => Some(icons::CREDIT_CARD_CLOCK_OUTLINE),
        "pipe" => Some(icons::PIPE),
        "selection-search" => Some(icons::SELECTION_SEARCH),
        "equal-box" => Some(icons::EQUAL_BOX),
        "content-save-settings-outline" => Some(icons::CONTENT_SAVE_SETTINGS_OUTLINE),
        "vector-difference-ba" => Some(icons::VECTOR_DIFFERENCE_BA),
        "sword-cross" => Some(icons::SWORD_CROSS),
        "plus-network" => Some(icons::PLUS_NETWORK),
        "heart-plus" => Some(icons::HEART_PLUS),
        "cloud-percent-outline" => Some(icons::CLOUD_PERCENT_OUTLINE),
        "help-network" => Some(icons::HELP_NETWORK),
        "clock-plus" => Some(icons::CLOCK_PLUS),
        "table-chair" => Some(icons::TABLE_CHAIR),
        "message-arrow-left-outline" => Some(icons::MESSAGE_ARROW_LEFT_OUTLINE),
        "head-question-outline" => Some(icons::HEAD_QUESTION_OUTLINE),
        "phone-hangup-outline" => Some(icons::PHONE_HANGUP_OUTLINE),
        "signal-hspa-plus" => Some(icons::SIGNAL_HSPA_PLUS),
        "pencil-minus" => Some(icons::PENCIL_MINUS),
        "wheel-barrow" => Some(icons::WHEEL_BARROW),
        "tag-check-outline" => Some(icons::TAG_CHECK_OUTLINE),
        "calendar-today-outline" => Some(icons::CALENDAR_TODAY_OUTLINE),
        "artboard" => Some(icons::ARTBOARD),
        "orbit-variant" => Some(icons::ORBIT_VARIANT),
        "sign-direction" => Some(icons::SIGN_DIRECTION),
        #[allow(deprecated)]
        "microsoft-visual-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-visual-studio' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_VISUAL_STUDIO)
        }
        "database-arrow-down-outline" => Some(icons::DATABASE_ARROW_DOWN_OUTLINE),
        "floor-lamp" => Some(icons::FLOOR_LAMP),
        "briefcase-plus-outline" => Some(icons::BRIEFCASE_PLUS_OUTLINE),
        "tooltip-edit" => Some(icons::TOOLTIP_EDIT),
        "ceiling-fan" => Some(icons::CEILING_FAN),
        "circle-small" => Some(icons::CIRCLE_SMALL),
        "phone-lock-outline" => Some(icons::PHONE_LOCK_OUTLINE),
        "map-outline" => Some(icons::MAP_OUTLINE),
        "bank-outline" => Some(icons::BANK_OUTLINE),
        "gas-station-off" => Some(icons::GAS_STATION_OFF),
        "surround-sound" => Some(icons::SURROUND_SOUND),
        "rice" => Some(icons::RICE),
        "lock-open-outline" => Some(icons::LOCK_OPEN_OUTLINE),
        "fishbowl-outline" => Some(icons::FISHBOWL_OUTLINE),
        "monitor-eye" => Some(icons::MONITOR_EYE),
        "file-code" => Some(icons::FILE_CODE),
        "archive-check" => Some(icons::ARCHIVE_CHECK),
        #[allow(deprecated)]
        "gnome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gnome' is deprecated.").print(py);
            }
            Some(icons::GNOME)
        }
        "color-helper" => Some(icons::COLOR_HELPER),
        "exit-to-app" => Some(icons::EXIT_TO_APP),
        "stadium" => Some(icons::STADIUM),
        "head-dots-horizontal" => Some(icons::HEAD_DOTS_HORIZONTAL),
        "shield-home" => Some(icons::SHIELD_HOME),
        "unfold-more-horizontal" => Some(icons::UNFOLD_MORE_HORIZONTAL),
        #[allow(deprecated)]
        "language-markdown" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-markdown' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_MARKDOWN)
        }
        "map-legend" => Some(icons::MAP_LEGEND),
        "invoice" => Some(icons::INVOICE),
        "arrow-decision-auto-outline" => Some(icons::ARROW_DECISION_AUTO_OUTLINE),
        "lotion-outline" => Some(icons::LOTION_OUTLINE),
        "battery-check" => Some(icons::BATTERY_CHECK),
        "peace" => Some(icons::PEACE),
        "thermometer-probe-off" => Some(icons::THERMOMETER_PROBE_OFF),
        "transfer" => Some(icons::TRANSFER),
        "ruler-square-compass" => Some(icons::RULER_SQUARE_COMPASS),
        "store-cog" => Some(icons::STORE_COG),
        "swap-horizontal" => Some(icons::SWAP_HORIZONTAL),
        "book-heart-outline" => Some(icons::BOOK_HEART_OUTLINE),
        "alpha-g-circle-outline" => Some(icons::ALPHA_G_CIRCLE_OUTLINE),
        "phone-missed-outline" => Some(icons::PHONE_MISSED_OUTLINE),
        "cookie-outline" => Some(icons::COOKIE_OUTLINE),
        "relation-one-to-many" => Some(icons::RELATION_ONE_TO_MANY),
        "cast-audio-variant" => Some(icons::CAST_AUDIO_VARIANT),
        "scanner" => Some(icons::SCANNER),
        "loading" => Some(icons::LOADING),
        "book-check" => Some(icons::BOOK_CHECK),
        "code-not-equal-variant" => Some(icons::CODE_NOT_EQUAL_VARIANT),
        "flask-outline" => Some(icons::FLASK_OUTLINE),
        "power-standby" => Some(icons::POWER_STANDBY),
        "volume-plus" => Some(icons::VOLUME_PLUS),
        "network-strength-4" => Some(icons::NETWORK_STRENGTH_4),
        "checkbox-marked-circle-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_OUTLINE),
        "cube-off" => Some(icons::CUBE_OFF),
        "play-circle" => Some(icons::PLAY_CIRCLE),
        "dots-vertical" => Some(icons::DOTS_VERTICAL),
        "doorbell-video" => Some(icons::DOORBELL_VIDEO),
        "watch" => Some(icons::WATCH),
        "gamepad-outline" => Some(icons::GAMEPAD_OUTLINE),
        "lightbulb-group-outline" => Some(icons::LIGHTBULB_GROUP_OUTLINE),
        "application-parentheses-outline" => Some(icons::APPLICATION_PARENTHESES_OUTLINE),
        "balloon" => Some(icons::BALLOON),
        "message-arrow-right-outline" => Some(icons::MESSAGE_ARROW_RIGHT_OUTLINE),
        "pause-octagon-outline" => Some(icons::PAUSE_OCTAGON_OUTLINE),
        "barley" => Some(icons::BARLEY),
        "navigation-variant-outline" => Some(icons::NAVIGATION_VARIANT_OUTLINE),
        "battery-unknown-bluetooth" => Some(icons::BATTERY_UNKNOWN_BLUETOOTH),
        "timeline-clock-outline" => Some(icons::TIMELINE_CLOCK_OUTLINE),
        "message-processing-outline" => Some(icons::MESSAGE_PROCESSING_OUTLINE),
        "police-badge-outline" => Some(icons::POLICE_BADGE_OUTLINE),
        #[allow(deprecated)]
        "quora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'quora' is deprecated.").print(py);
            }
            Some(icons::QUORA)
        }
        #[allow(deprecated)]
        "centos" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'centos' is deprecated.").print(py);
            }
            Some(icons::CENTOS)
        }
        "timer-star" => Some(icons::TIMER_STAR),
        "toslink" => Some(icons::TOSLINK),
        "gate" => Some(icons::GATE),
        "image-refresh" => Some(icons::IMAGE_REFRESH),
        "sausage" => Some(icons::SAUSAGE),
        "redo-variant" => Some(icons::REDO_VARIANT),
        "file-undo" => Some(icons::FILE_UNDO),
        "clipboard-check-multiple" => Some(icons::CLIPBOARD_CHECK_MULTIPLE),
        "window-closed" => Some(icons::WINDOW_CLOSED),
        "server-network-off" => Some(icons::SERVER_NETWORK_OFF),
        "calendar-range" => Some(icons::CALENDAR_RANGE),
        "pencil-off-outline" => Some(icons::PENCIL_OFF_OUTLINE),
        "magnify-plus-cursor" => Some(icons::MAGNIFY_PLUS_CURSOR),
        "repeat-once" => Some(icons::REPEAT_ONCE),
        "graph" => Some(icons::GRAPH),
        "pan" => Some(icons::PAN),
        "circle-slice-6" => Some(icons::CIRCLE_SLICE_6),
        "hexagon-slice-5" => Some(icons::HEXAGON_SLICE_5),
        "chili-off-outline" => Some(icons::CHILI_OFF_OUTLINE),
        "inbox-remove-outline" => Some(icons::INBOX_REMOVE_OUTLINE),
        "snowflake-melt" => Some(icons::SNOWFLAKE_MELT),
        "mirror-variant" => Some(icons::MIRROR_VARIANT),
        "home-switch-outline" => Some(icons::HOME_SWITCH_OUTLINE),
        "sort-clock-ascending-outline" => Some(icons::SORT_CLOCK_ASCENDING_OUTLINE),
        #[allow(deprecated)]
        "vimeo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vimeo' is deprecated.").print(py);
            }
            Some(icons::VIMEO)
        }
        "home-city" => Some(icons::HOME_CITY),
        "sign-language" => Some(icons::SIGN_LANGUAGE),
        "reply-all-outline" => Some(icons::REPLY_ALL_OUTLINE),
        "alpha-c-box" => Some(icons::ALPHA_C_BOX),
        _ => None,
    }
}
