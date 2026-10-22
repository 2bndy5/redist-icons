// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_6(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "notebook-remove-outline" => Some(icons::NOTEBOOK_REMOVE_OUTLINE),
        "tooth-outline" => Some(icons::TOOTH_OUTLINE),
        "chevron-double-left" => Some(icons::CHEVRON_DOUBLE_LEFT),
        "file-excel-box-outline" => Some(icons::FILE_EXCEL_BOX_OUTLINE),
        "content-save-cog-outline" => Some(icons::CONTENT_SAVE_COG_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-off" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-off' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_OFF)
        }
        "decimal-decrease" => Some(icons::DECIMAL_DECREASE),
        "timer-sand" => Some(icons::TIMER_SAND),
        "arrow-u-left-bottom-bold" => Some(icons::ARROW_U_LEFT_BOTTOM_BOLD),
        "format-align-top" => Some(icons::FORMAT_ALIGN_TOP),
        "snowflake-alert" => Some(icons::SNOWFLAKE_ALERT),
        "bank-transfer-in" => Some(icons::BANK_TRANSFER_IN),
        "phone-remove" => Some(icons::PHONE_REMOVE),
        "bottle-tonic-skull" => Some(icons::BOTTLE_TONIC_SKULL),
        "treasure-chest-outline" => Some(icons::TREASURE_CHEST_OUTLINE),
        "alpha-r-box" => Some(icons::ALPHA_R_BOX),
        "database-plus-outline" => Some(icons::DATABASE_PLUS_OUTLINE),
        "printer-pos-remove" => Some(icons::PRINTER_POS_REMOVE),
        "table-sync" => Some(icons::TABLE_SYNC),
        "trophy-variant-outline" => Some(icons::TROPHY_VARIANT_OUTLINE),
        "weather-snowy-rainy" => Some(icons::WEATHER_SNOWY_RAINY),
        "contrast-circle" => Some(icons::CONTRAST_CIRCLE),
        "server-plus-outline" => Some(icons::SERVER_PLUS_OUTLINE),
        "keyboard-esc" => Some(icons::KEYBOARD_ESC),
        "tray-arrow-up" => Some(icons::TRAY_ARROW_UP),
        "clock-time-one-outline" => Some(icons::CLOCK_TIME_ONE_OUTLINE),
        "arrow-down-right" => Some(icons::ARROW_DOWN_RIGHT),
        "cash-lock" => Some(icons::CASH_LOCK),
        "rhombus-split" => Some(icons::RHOMBUS_SPLIT),
        "baby-face-outline" => Some(icons::BABY_FACE_OUTLINE),
        "calendar-alert-outline" => Some(icons::CALENDAR_ALERT_OUTLINE),
        "chevron-double-down" => Some(icons::CHEVRON_DOUBLE_DOWN),
        "credit-card-edit-outline" => Some(icons::CREDIT_CARD_EDIT_OUTLINE),
        "shield-link-variant" => Some(icons::SHIELD_LINK_VARIANT),
        #[allow(deprecated)]
        "google-hangouts" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-hangouts' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_HANGOUTS)
        }
        "timelapse" => Some(icons::TIMELAPSE),
        "folder-text-outline" => Some(icons::FOLDER_TEXT_OUTLINE),
        "battery-charging-100" => Some(icons::BATTERY_CHARGING_100),
        "chess-queen" => Some(icons::CHESS_QUEEN),
        "toy-brick" => Some(icons::TOY_BRICK),
        "transfer-up" => Some(icons::TRANSFER_UP),
        "horse-variant" => Some(icons::HORSE_VARIANT),
        "fan-speed-1" => Some(icons::FAN_SPEED_1),
        "sail-boat" => Some(icons::SAIL_BOAT),
        "signature-image" => Some(icons::SIGNATURE_IMAGE),
        "printer-check" => Some(icons::PRINTER_CHECK),
        "gas-station-in-use-outline" => Some(icons::GAS_STATION_IN_USE_OUTLINE),
        "crown" => Some(icons::CROWN),
        "controller-classic-outline" => Some(icons::CONTROLLER_CLASSIC_OUTLINE),
        "clipboard-check-outline" => Some(icons::CLIPBOARD_CHECK_OUTLINE),
        #[allow(deprecated)]
        "language-csharp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-csharp' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_CSHARP)
        }
        "alert-plus" => Some(icons::ALERT_PLUS),
        "cloud-remove-outline" => Some(icons::CLOUD_REMOVE_OUTLINE),
        "newspaper-plus" => Some(icons::NEWSPAPER_PLUS),
        "tooltip-outline" => Some(icons::TOOLTIP_OUTLINE),
        "hvac-off" => Some(icons::HVAC_OFF),
        "car-defrost-front" => Some(icons::CAR_DEFROST_FRONT),
        "cast-education" => Some(icons::CAST_EDUCATION),
        "shape-polygon-plus" => Some(icons::SHAPE_POLYGON_PLUS),
        "wall-sconce-round" => Some(icons::WALL_SCONCE_ROUND),
        "angle-obtuse" => Some(icons::ANGLE_OBTUSE),
        "message-reply" => Some(icons::MESSAGE_REPLY),
        "clock-alert-outline" => Some(icons::CLOCK_ALERT_OUTLINE),
        "shield-moon-outline" => Some(icons::SHIELD_MOON_OUTLINE),
        "book-multiple" => Some(icons::BOOK_MULTIPLE),
        "palette-advanced" => Some(icons::PALETTE_ADVANCED),
        "tooltip-question-outline" => Some(icons::TOOLTIP_QUESTION_OUTLINE),
        "cloud-minus" => Some(icons::CLOUD_MINUS),
        "layers-remove" => Some(icons::LAYERS_REMOVE),
        "numeric-3-box" => Some(icons::NUMERIC_3_BOX),
        "timeline-remove-outline" => Some(icons::TIMELINE_REMOVE_OUTLINE),
        "message-fast-outline" => Some(icons::MESSAGE_FAST_OUTLINE),
        "skip-previous-circle" => Some(icons::SKIP_PREVIOUS_CIRCLE),
        "projector-screen-variant" => Some(icons::PROJECTOR_SCREEN_VARIANT),
        "file-lock-outline" => Some(icons::FILE_LOCK_OUTLINE),
        "weather-hurricane-outline" => Some(icons::WEATHER_HURRICANE_OUTLINE),
        "volleyball" => Some(icons::VOLLEYBALL),
        "chevron-left-circle-outline" => Some(icons::CHEVRON_LEFT_CIRCLE_OUTLINE),
        "hammer-sickle" => Some(icons::HAMMER_SICKLE),
        "chili-off-outline" => Some(icons::CHILI_OFF_OUTLINE),
        "alert-box" => Some(icons::ALERT_BOX),
        "horseshoe" => Some(icons::HORSESHOE),
        "fridge-off" => Some(icons::FRIDGE_OFF),
        "thermostat-box-auto" => Some(icons::THERMOSTAT_BOX_AUTO),
        "eye-plus-outline" => Some(icons::EYE_PLUS_OUTLINE),
        "code-brackets" => Some(icons::CODE_BRACKETS),
        "heart-plus-outline" => Some(icons::HEART_PLUS_OUTLINE),
        "water-percent" => Some(icons::WATER_PERCENT),
        "dice-d8" => Some(icons::DICE_D8),
        "microsoft-xbox-controller-battery-unknown" => {
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_UNKNOWN)
        }
        #[allow(deprecated)]
        "quora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'quora' is deprecated.").print(py);
            }
            Some(icons::QUORA)
        }
        "printer-3d-nozzle-off-outline" => Some(icons::PRINTER_3D_NOZZLE_OFF_OUTLINE),
        "file-document-plus" => Some(icons::FILE_DOCUMENT_PLUS),
        "candy-off-outline" => Some(icons::CANDY_OFF_OUTLINE),
        "baguette" => Some(icons::BAGUETTE),
        "compare-remove" => Some(icons::COMPARE_REMOVE),
        "circle-double" => Some(icons::CIRCLE_DOUBLE),
        "waves" => Some(icons::WAVES),
        "storefront-minus" => Some(icons::STOREFRONT_MINUS),
        "alarm-bell" => Some(icons::ALARM_BELL),
        "hospital-box" => Some(icons::HOSPITAL_BOX),
        "information-slab-circle-outline" => Some(icons::INFORMATION_SLAB_CIRCLE_OUTLINE),
        "toggle-switch-variant-off" => Some(icons::TOGGLE_SWITCH_VARIANT_OFF),
        "roman-numeral-10" => Some(icons::ROMAN_NUMERAL_10),
        "view-gallery" => Some(icons::VIEW_GALLERY),
        "chart-box-multiple-outline" => Some(icons::CHART_BOX_MULTIPLE_OUTLINE),
        "alpha-s-box" => Some(icons::ALPHA_S_BOX),
        "keyboard-f11" => Some(icons::KEYBOARD_F11),
        "window-restore" => Some(icons::WINDOW_RESTORE),
        "chat-outline" => Some(icons::CHAT_OUTLINE),
        "beach" => Some(icons::BEACH),
        "script-text-play" => Some(icons::SCRIPT_TEXT_PLAY),
        "pause-box" => Some(icons::PAUSE_BOX),
        "alert-circle-check" => Some(icons::ALERT_CIRCLE_CHECK),
        "chevron-left-box" => Some(icons::CHEVRON_LEFT_BOX),
        "truck-plus" => Some(icons::TRUCK_PLUS),
        "star-off" => Some(icons::STAR_OFF),
        "briefcase-clock" => Some(icons::BRIEFCASE_CLOCK),
        "cookie-remove" => Some(icons::COOKIE_REMOVE),
        "battery-70-bluetooth" => Some(icons::BATTERY_70_BLUETOOTH),
        "subtitles-outline" => Some(icons::SUBTITLES_OUTLINE),
        "cloud-check-outline" => Some(icons::CLOUD_CHECK_OUTLINE),
        "battery-plus-variant" => Some(icons::BATTERY_PLUS_VARIANT),
        "cactus" => Some(icons::CACTUS),
        "battery-80-bluetooth" => Some(icons::BATTERY_80_BLUETOOTH),
        "truck-off-road-off" => Some(icons::TRUCK_OFF_ROAD_OFF),
        "toy-brick-marker-outline" => Some(icons::TOY_BRICK_MARKER_OUTLINE),
        "update" => Some(icons::UPDATE),
        "email-plus-outline" => Some(icons::EMAIL_PLUS_OUTLINE),
        "trending-down" => Some(icons::TRENDING_DOWN),
        "account-plus" => Some(icons::ACCOUNT_PLUS),
        "code-braces-box" => Some(icons::CODE_BRACES_BOX),
        "format-align-center" => Some(icons::FORMAT_ALIGN_CENTER),
        "file-link-outline" => Some(icons::FILE_LINK_OUTLINE),
        "message-check-outline" => Some(icons::MESSAGE_CHECK_OUTLINE),
        "pin-outline" => Some(icons::PIN_OUTLINE),
        "robot-vacuum-variant-alert" => Some(icons::ROBOT_VACUUM_VARIANT_ALERT),
        "application-brackets-outline" => Some(icons::APPLICATION_BRACKETS_OUTLINE),
        "key-chain" => Some(icons::KEY_CHAIN),
        "hand-back-left-off" => Some(icons::HAND_BACK_LEFT_OFF),
        #[allow(deprecated)]
        "bulma" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bulma' is deprecated.").print(py);
            }
            Some(icons::BULMA)
        }
        "account-arrow-down-outline" => Some(icons::ACCOUNT_ARROW_DOWN_OUTLINE),
        "movie-open-star-outline" => Some(icons::MOVIE_OPEN_STAR_OUTLINE),
        "cellphone-screenshot" => Some(icons::CELLPHONE_SCREENSHOT),
        "stairs-box" => Some(icons::STAIRS_BOX),
        "email-mark-as-unread" => Some(icons::EMAIL_MARK_AS_UNREAD),
        "ferry" => Some(icons::FERRY),
        #[allow(deprecated)]
        "evernote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'evernote' is deprecated.").print(py);
            }
            Some(icons::EVERNOTE)
        }
        "movie-open-edit-outline" => Some(icons::MOVIE_OPEN_EDIT_OUTLINE),
        "mine" => Some(icons::MINE),
        "butterfly" => Some(icons::BUTTERFLY),
        "database-alert" => Some(icons::DATABASE_ALERT),
        "scale-unbalanced" => Some(icons::SCALE_UNBALANCED),
        "web-cancel" => Some(icons::WEB_CANCEL),
        "gender-male-female-variant" => Some(icons::GENDER_MALE_FEMALE_VARIANT),
        "gate-nor" => Some(icons::GATE_NOR),
        "eye-arrow-right-outline" => Some(icons::EYE_ARROW_RIGHT_OUTLINE),
        "map-marker-left" => Some(icons::MAP_MARKER_LEFT),
        "message-minus" => Some(icons::MESSAGE_MINUS),
        "kayaking" => Some(icons::KAYAKING),
        "saxophone" => Some(icons::SAXOPHONE),
        "fridge-variant-off" => Some(icons::FRIDGE_VARIANT_OFF),
        "alphabet-tengwar" => Some(icons::ALPHABET_TENGWAR),
        "arrow-left-top-bold" => Some(icons::ARROW_LEFT_TOP_BOLD),
        "fire-extinguisher" => Some(icons::FIRE_EXTINGUISHER),
        "synagogue" => Some(icons::SYNAGOGUE),
        "battery-sync" => Some(icons::BATTERY_SYNC),
        "alien-outline" => Some(icons::ALIEN_OUTLINE),
        "wifi-strength-1-lock-open" => Some(icons::WIFI_STRENGTH_1_LOCK_OPEN),
        "alpha-s-circle" => Some(icons::ALPHA_S_CIRCLE),
        "play-box-multiple-outline" => Some(icons::PLAY_BOX_MULTIPLE_OUTLINE),
        "cellphone-arrow-down-variant" => Some(icons::CELLPHONE_ARROW_DOWN_VARIANT),
        "lock-open-minus" => Some(icons::LOCK_OPEN_MINUS),
        "roman-numeral-4" => Some(icons::ROMAN_NUMERAL_4),
        "television-classic-off" => Some(icons::TELEVISION_CLASSIC_OFF),
        "check" => Some(icons::CHECK),
        "form-dropdown" => Some(icons::FORM_DROPDOWN),
        "battery-arrow-up-outline" => Some(icons::BATTERY_ARROW_UP_OUTLINE),
        "microwave-off" => Some(icons::MICROWAVE_OFF),
        "alpha-s-box-outline" => Some(icons::ALPHA_S_BOX_OUTLINE),
        "cellphone-wireless" => Some(icons::CELLPHONE_WIRELESS),
        "account-star-outline" => Some(icons::ACCOUNT_STAR_OUTLINE),
        "waves-arrow-up" => Some(icons::WAVES_ARROW_UP),
        "speaker-message" => Some(icons::SPEAKER_MESSAGE),
        "lotion-plus-outline" => Some(icons::LOTION_PLUS_OUTLINE),
        "home-battery" => Some(icons::HOME_BATTERY),
        "comment-alert" => Some(icons::COMMENT_ALERT),
        "bench" => Some(icons::BENCH),
        "alpha-w-circle" => Some(icons::ALPHA_W_CIRCLE),
        "home-city-outline" => Some(icons::HOME_CITY_OUTLINE),
        "menorah" => Some(icons::MENORAH),
        "file-key-outline" => Some(icons::FILE_KEY_OUTLINE),
        "car-wrench" => Some(icons::CAR_WRENCH),
        "credit-card" => Some(icons::CREDIT_CARD),
        "contacts" => Some(icons::CONTACTS),
        "piano" => Some(icons::PIANO),
        "account-child-circle" => Some(icons::ACCOUNT_CHILD_CIRCLE),
        #[allow(deprecated)]
        "microsoft-bing" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-bing' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_BING)
        }
        "map-marker-off-outline" => Some(icons::MAP_MARKER_OFF_OUTLINE),
        "file-presentation-box" => Some(icons::FILE_PRESENTATION_BOX),
        _ => None,
    }
}
