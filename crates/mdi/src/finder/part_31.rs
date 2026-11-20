// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_31(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "bowling" => Some(icons::BOWLING),
        #[allow(deprecated)]
        "microsoft-dynamics-365" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-dynamics-365' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_DYNAMICS_365)
        }
        "airplane-edit" => Some(icons::AIRPLANE_EDIT),
        "eiffel-tower" => Some(icons::EIFFEL_TOWER),
        "sphere" => Some(icons::SPHERE),
        "movie-minus" => Some(icons::MOVIE_MINUS),
        "timer-sand-full" => Some(icons::TIMER_SAND_FULL),
        "ip" => Some(icons::IP),
        "movie-open-star-outline" => Some(icons::MOVIE_OPEN_STAR_OUTLINE),
        "alpha-s-box" => Some(icons::ALPHA_S_BOX),
        "weather-windy-variant" => Some(icons::WEATHER_WINDY_VARIANT),
        #[allow(deprecated)]
        "mastodon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mastodon' is deprecated.").print(py);
            }
            Some(icons::MASTODON)
        }
        "view-parallel-outline" => Some(icons::VIEW_PARALLEL_OUTLINE),
        "toggle-switch" => Some(icons::TOGGLE_SWITCH),
        "message-bulleted" => Some(icons::MESSAGE_BULLETED),
        "seat-flat" => Some(icons::SEAT_FLAT),
        "truck-delivery-outline" => Some(icons::TRUCK_DELIVERY_OUTLINE),
        #[allow(deprecated)]
        "language-lua" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-lua' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_LUA)
        }
        "format-align-middle" => Some(icons::FORMAT_ALIGN_MIDDLE),
        "road" => Some(icons::ROAD),
        "flask-empty-off" => Some(icons::FLASK_EMPTY_OFF),
        "server-plus" => Some(icons::SERVER_PLUS),
        "eye-lock-outline" => Some(icons::EYE_LOCK_OUTLINE),
        "atv" => Some(icons::ATV),
        "toggle-switch-off-outline" => Some(icons::TOGGLE_SWITCH_OFF_OUTLINE),
        "source-branch" => Some(icons::SOURCE_BRANCH),
        "paperclip" => Some(icons::PAPERCLIP),
        "alpha-x-box" => Some(icons::ALPHA_X_BOX),
        "clock-time-four" => Some(icons::CLOCK_TIME_FOUR),
        "map-marker-remove-variant" => Some(icons::MAP_MARKER_REMOVE_VARIANT),
        "robber" => Some(icons::ROBBER),
        "currency-twd" => Some(icons::CURRENCY_TWD),
        "account-multiple-check" => Some(icons::ACCOUNT_MULTIPLE_CHECK),
        "database-plus-outline" => Some(icons::DATABASE_PLUS_OUTLINE),
        "window-closed" => Some(icons::WINDOW_CLOSED),
        "arrow-expand-horizontal" => Some(icons::ARROW_EXPAND_HORIZONTAL),
        "folder-key-network-outline" => Some(icons::FOLDER_KEY_NETWORK_OUTLINE),
        "inbox-full" => Some(icons::INBOX_FULL),
        "comma-circle" => Some(icons::COMMA_CIRCLE),
        "robot-vacuum" => Some(icons::ROBOT_VACUUM),
        "alpha-l" => Some(icons::ALPHA_L),
        "map-clock-outline" => Some(icons::MAP_CLOCK_OUTLINE),
        "garage-alert-variant" => Some(icons::GARAGE_ALERT_VARIANT),
        "microphone-plus" => Some(icons::MICROPHONE_PLUS),
        "receipt-clock" => Some(icons::RECEIPT_CLOCK),
        "window-shutter-settings" => Some(icons::WINDOW_SHUTTER_SETTINGS),
        "moon-waning-crescent" => Some(icons::MOON_WANING_CRESCENT),
        #[allow(deprecated)]
        "bitbucket" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bitbucket' is deprecated.").print(py);
            }
            Some(icons::BITBUCKET)
        }
        "file-code-outline" => Some(icons::FILE_CODE_OUTLINE),
        #[allow(deprecated)]
        "unicode" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unicode' is deprecated.").print(py);
            }
            Some(icons::UNICODE)
        }
        "close-box" => Some(icons::CLOSE_BOX),
        "film" => Some(icons::FILM),
        "thought-bubble-outline" => Some(icons::THOUGHT_BUBBLE_OUTLINE),
        "coffee" => Some(icons::COFFEE),
        "rug" => Some(icons::RUG),
        "signal" => Some(icons::SIGNAL),
        "invoice-text-edit" => Some(icons::INVOICE_TEXT_EDIT),
        "key-arrow-right" => Some(icons::KEY_ARROW_RIGHT),
        "message-arrow-right" => Some(icons::MESSAGE_ARROW_RIGHT),
        "dog" => Some(icons::DOG),
        "phone-dial" => Some(icons::PHONE_DIAL),
        "certificate-outline" => Some(icons::CERTIFICATE_OUTLINE),
        "clipboard-text-search-outline" => Some(icons::CLIPBOARD_TEXT_SEARCH_OUTLINE),
        "chart-sankey-variant" => Some(icons::CHART_SANKEY_VARIANT),
        "robot-outline" => Some(icons::ROBOT_OUTLINE),
        #[allow(deprecated)]
        "google-nearby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-nearby' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_NEARBY)
        }
        "yin-yang" => Some(icons::YIN_YANG),
        "archive" => Some(icons::ARCHIVE),
        "record-circle" => Some(icons::RECORD_CIRCLE),
        "bullseye" => Some(icons::BULLSEYE),
        "gamepad-variant-outline" => Some(icons::GAMEPAD_VARIANT_OUTLINE),
        "beer" => Some(icons::BEER),
        "phone-return-outline" => Some(icons::PHONE_RETURN_OUTLINE),
        "send-lock" => Some(icons::SEND_LOCK),
        "chevron-up-circle-outline" => Some(icons::CHEVRON_UP_CIRCLE_OUTLINE),
        "battery-charging-30" => Some(icons::BATTERY_CHARGING_30),
        "text-box-multiple" => Some(icons::TEXT_BOX_MULTIPLE),
        "target" => Some(icons::TARGET),
        "content-save-edit-outline" => Some(icons::CONTENT_SAVE_EDIT_OUTLINE),
        "blinds-vertical-closed" => Some(icons::BLINDS_VERTICAL_CLOSED),
        "generator-stationary" => Some(icons::GENERATOR_STATIONARY),
        "human-wheelchair" => Some(icons::HUMAN_WHEELCHAIR),
        "phone-off-outline" => Some(icons::PHONE_OFF_OUTLINE),
        "bug" => Some(icons::BUG),
        "battery-minus" => Some(icons::BATTERY_MINUS),
        "perspective-more" => Some(icons::PERSPECTIVE_MORE),
        "call-split" => Some(icons::CALL_SPLIT),
        "heart-search" => Some(icons::HEART_SEARCH),
        "account-wrench-outline" => Some(icons::ACCOUNT_WRENCH_OUTLINE),
        "timeline-question" => Some(icons::TIMELINE_QUESTION),
        "heart-box-outline" => Some(icons::HEART_BOX_OUTLINE),
        "upload-box-outline" => Some(icons::UPLOAD_BOX_OUTLINE),
        "movie-settings-outline" => Some(icons::MOVIE_SETTINGS_OUTLINE),
        "headphones-bluetooth" => Some(icons::HEADPHONES_BLUETOOTH),
        "shield-bug" => Some(icons::SHIELD_BUG),
        "arrow-bottom-right-thin" => Some(icons::ARROW_BOTTOM_RIGHT_THIN),
        "account-switch" => Some(icons::ACCOUNT_SWITCH),
        "pirate" => Some(icons::PIRATE),
        "filter-menu-outline" => Some(icons::FILTER_MENU_OUTLINE),
        "text-search" => Some(icons::TEXT_SEARCH),
        "folder-arrow-up" => Some(icons::FOLDER_ARROW_UP),
        #[allow(deprecated)]
        "language-c" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-c' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_C)
        }
        "database-check-outline" => Some(icons::DATABASE_CHECK_OUTLINE),
        "head-dots-horizontal-outline" => Some(icons::HEAD_DOTS_HORIZONTAL_OUTLINE),
        "cog-stop" => Some(icons::COG_STOP),
        "notebook-edit" => Some(icons::NOTEBOOK_EDIT),
        #[allow(deprecated)]
        "sony-playstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sony-playstation' is deprecated.")
                    .print(py);
            }
            Some(icons::SONY_PLAYSTATION)
        }
        "shield-edit-outline" => Some(icons::SHIELD_EDIT_OUTLINE),
        "more" => Some(icons::MORE),
        "network-strength-3-alert" => Some(icons::NETWORK_STRENGTH_3_ALERT),
        "check-bold" => Some(icons::CHECK_BOLD),
        "alpha-z-box" => Some(icons::ALPHA_Z_BOX),
        "shuffle-disabled" => Some(icons::SHUFFLE_DISABLED),
        "message-outline" => Some(icons::MESSAGE_OUTLINE),
        "file-document-check-outline" => Some(icons::FILE_DOCUMENT_CHECK_OUTLINE),
        "format-text-rotation-angle-down" => Some(icons::FORMAT_TEXT_ROTATION_ANGLE_DOWN),
        "calendar-star" => Some(icons::CALENDAR_STAR),
        "download-network" => Some(icons::DOWNLOAD_NETWORK),
        "blur" => Some(icons::BLUR),
        "movie-cog-outline" => Some(icons::MOVIE_COG_OUTLINE),
        "invoice-text-send-outline" => Some(icons::INVOICE_TEXT_SEND_OUTLINE),
        "arrow-up-drop-circle" => Some(icons::ARROW_UP_DROP_CIRCLE),
        "minus" => Some(icons::MINUS),
        "sort-variant-lock" => Some(icons::SORT_VARIANT_LOCK),
        "delete-restore" => Some(icons::DELETE_RESTORE),
        "bus-side" => Some(icons::BUS_SIDE),
        "human-male-height-variant" => Some(icons::HUMAN_MALE_HEIGHT_VARIANT),
        "flip-vertical" => Some(icons::FLIP_VERTICAL),
        "cloud-search-outline" => Some(icons::CLOUD_SEARCH_OUTLINE),
        "palette-advanced" => Some(icons::PALETTE_ADVANCED),
        "gesture-swipe-horizontal" => Some(icons::GESTURE_SWIPE_HORIZONTAL),
        "source-branch-refresh" => Some(icons::SOURCE_BRANCH_REFRESH),
        "phone-outgoing-outline" => Some(icons::PHONE_OUTGOING_OUTLINE),
        "wrap-disabled" => Some(icons::WRAP_DISABLED),
        "map-marker-multiple-outline" => Some(icons::MAP_MARKER_MULTIPLE_OUTLINE),
        "light-switch-off" => Some(icons::LIGHT_SWITCH_OFF),
        "fountain-pen-tip" => Some(icons::FOUNTAIN_PEN_TIP),
        "table-alert" => Some(icons::TABLE_ALERT),
        "data-matrix-minus" => Some(icons::DATA_MATRIX_MINUS),
        "ev-plug-type1" => Some(icons::EV_PLUG_TYPE1),
        "border-right" => Some(icons::BORDER_RIGHT),
        "klingon" => Some(icons::KLINGON),
        "receipt-text" => Some(icons::RECEIPT_TEXT),
        "paperclip-check" => Some(icons::PAPERCLIP_CHECK),
        "garage-variant" => Some(icons::GARAGE_VARIANT),
        "file-edit-outline" => Some(icons::FILE_EDIT_OUTLINE),
        "view-grid-compact" => Some(icons::VIEW_GRID_COMPACT),
        "file-send-outline" => Some(icons::FILE_SEND_OUTLINE),
        "selection-multiple" => Some(icons::SELECTION_MULTIPLE),
        "credit-card-minus" => Some(icons::CREDIT_CARD_MINUS),
        "taco" => Some(icons::TACO),
        "rocket-launch" => Some(icons::ROCKET_LAUNCH),
        "arrow-right-drop-circle-outline" => Some(icons::ARROW_RIGHT_DROP_CIRCLE_OUTLINE),
        "delete-off-outline" => Some(icons::DELETE_OFF_OUTLINE),
        "clipboard-play-multiple" => Some(icons::CLIPBOARD_PLAY_MULTIPLE),
        "robot-angry" => Some(icons::ROBOT_ANGRY),
        "file-document-edit-outline" => Some(icons::FILE_DOCUMENT_EDIT_OUTLINE),
        "car-traction-control" => Some(icons::CAR_TRACTION_CONTROL),
        "format-text-variant-outline" => Some(icons::FORMAT_TEXT_VARIANT_OUTLINE),
        "liquor" => Some(icons::LIQUOR),
        "record-rec" => Some(icons::RECORD_REC),
        "database-refresh" => Some(icons::DATABASE_REFRESH),
        "lightbulb-on-20" => Some(icons::LIGHTBULB_ON_20),
        "pen" => Some(icons::PEN),
        "content-save-all" => Some(icons::CONTENT_SAVE_ALL),
        "clock-time-three" => Some(icons::CLOCK_TIME_THREE),
        "near-me" => Some(icons::NEAR_ME),
        #[allow(deprecated)]
        "trello" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'trello' is deprecated.").print(py);
            }
            Some(icons::TRELLO)
        }
        "phone-check" => Some(icons::PHONE_CHECK),
        "star-check" => Some(icons::STAR_CHECK),
        #[allow(deprecated)]
        "language-r" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-r' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_R)
        }
        "server" => Some(icons::SERVER),
        "hand-coin" => Some(icons::HAND_COIN),
        "upload-outline" => Some(icons::UPLOAD_OUTLINE),
        "phone-in-talk" => Some(icons::PHONE_IN_TALK),
        "numeric-4-box-multiple-outline" => Some(icons::NUMERIC_4_BOX_MULTIPLE_OUTLINE),
        "timer-cog-outline" => Some(icons::TIMER_COG_OUTLINE),
        #[allow(deprecated)]
        "apple-finder" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-finder' is deprecated.").print(py);
            }
            Some(icons::APPLE_FINDER)
        }
        "printer-pos-play-outline" => Some(icons::PRINTER_POS_PLAY_OUTLINE),
        "home-automation" => Some(icons::HOME_AUTOMATION),
        "roman-numeral-4" => Some(icons::ROMAN_NUMERAL_4),
        "amplifier" => Some(icons::AMPLIFIER),
        "clipboard-text-play" => Some(icons::CLIPBOARD_TEXT_PLAY),
        "treasure-chest" => Some(icons::TREASURE_CHEST),
        "synagogue-outline" => Some(icons::SYNAGOGUE_OUTLINE),
        "numeric-8-box-multiple-outline" => Some(icons::NUMERIC_8_BOX_MULTIPLE_OUTLINE),
        "water-plus" => Some(icons::WATER_PLUS),
        "comment-arrow-left-outline" => Some(icons::COMMENT_ARROW_LEFT_OUTLINE),
        "syllabary-katakana-halfwidth" => Some(icons::SYLLABARY_KATAKANA_HALFWIDTH),
        "contain" => Some(icons::CONTAIN),
        "target-account" => Some(icons::TARGET_ACCOUNT),
        "hours-12" => Some(icons::HOURS_12),
        "code-greater-than-or-equal" => Some(icons::CODE_GREATER_THAN_OR_EQUAL),
        "vector-square" => Some(icons::VECTOR_SQUARE),
        "heart-cog-outline" => Some(icons::HEART_COG_OUTLINE),
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
        "arrow-top-right-thick" => Some(icons::ARROW_TOP_RIGHT_THICK),
        "sun-compass" => Some(icons::SUN_COMPASS),
        "timer-cancel" => Some(icons::TIMER_CANCEL),
        "form-select" => Some(icons::FORM_SELECT),
        _ => None,
    }
}
