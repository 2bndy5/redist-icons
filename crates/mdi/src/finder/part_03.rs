// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_3(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "lock-open-alert" => Some(icons::LOCK_OPEN_ALERT),
        "book-alphabet" => Some(icons::BOOK_ALPHABET),
        #[allow(deprecated)]
        "material-ui" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-ui' is deprecated.").print(py);
            }
            Some(icons::MATERIAL_UI)
        }
        "home-roof" => Some(icons::HOME_ROOF),
        "package-variant-minus" => Some(icons::PACKAGE_VARIANT_MINUS),
        "phone-in-talk-outline" => Some(icons::PHONE_IN_TALK_OUTLINE),
        "scoreboard" => Some(icons::SCOREBOARD),
        "calendar-week-begin" => Some(icons::CALENDAR_WEEK_BEGIN),
        "keyboard-off-outline" => Some(icons::KEYBOARD_OFF_OUTLINE),
        #[allow(deprecated)]
        "jquery" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jquery' is deprecated.").print(py);
            }
            Some(icons::JQUERY)
        }
        "line-scan" => Some(icons::LINE_SCAN),
        "shape-outline" => Some(icons::SHAPE_OUTLINE),
        "clipboard-alert" => Some(icons::CLIPBOARD_ALERT),
        "razor-single-edge" => Some(icons::RAZOR_SINGLE_EDGE),
        #[allow(deprecated)]
        "artstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'artstation' is deprecated.").print(py);
            }
            Some(icons::ARTSTATION)
        }
        "battery-charging-30" => Some(icons::BATTERY_CHARGING_30),
        "syllabary-katakana" => Some(icons::SYLLABARY_KATAKANA),
        "timer-music" => Some(icons::TIMER_MUSIC),
        "chevron-down-box-outline" => Some(icons::CHEVRON_DOWN_BOX_OUTLINE),
        "signature-text" => Some(icons::SIGNATURE_TEXT),
        "bike-pedal-mountain" => Some(icons::BIKE_PEDAL_MOUNTAIN),
        "timer-plus" => Some(icons::TIMER_PLUS),
        "table-picnic" => Some(icons::TABLE_PICNIC),
        "cheese" => Some(icons::CHEESE),
        "elevator-down" => Some(icons::ELEVATOR_DOWN),
        "phone-incoming-outline" => Some(icons::PHONE_INCOMING_OUTLINE),
        "comment-alert-outline" => Some(icons::COMMENT_ALERT_OUTLINE),
        "arrow-u-down-left-bold" => Some(icons::ARROW_U_DOWN_LEFT_BOLD),
        "square-circle" => Some(icons::SQUARE_CIRCLE),
        "open-in-app" => Some(icons::OPEN_IN_APP),
        "candy-outline" => Some(icons::CANDY_OUTLINE),
        "database-eye-off" => Some(icons::DATABASE_EYE_OFF),
        "account-alert-outline" => Some(icons::ACCOUNT_ALERT_OUTLINE),
        "arrow-left-bottom-bold" => Some(icons::ARROW_LEFT_BOTTOM_BOLD),
        #[allow(deprecated)]
        "microsoft-excel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-excel' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_EXCEL)
        }
        "billboard" => Some(icons::BILLBOARD),
        "calendar-remove-outline" => Some(icons::CALENDAR_REMOVE_OUTLINE),
        "car-limousine" => Some(icons::CAR_LIMOUSINE),
        "alpha-t" => Some(icons::ALPHA_T),
        "calendar-account" => Some(icons::CALENDAR_ACCOUNT),
        "close-box-outline" => Some(icons::CLOSE_BOX_OUTLINE),
        "table-cancel" => Some(icons::TABLE_CANCEL),
        "usb-c-port" => Some(icons::USB_C_PORT),
        "wall-fire" => Some(icons::WALL_FIRE),
        "emoticon-wink-outline" => Some(icons::EMOTICON_WINK_OUTLINE),
        "blinds-horizontal-closed" => Some(icons::BLINDS_HORIZONTAL_CLOSED),
        "prescription" => Some(icons::PRESCRIPTION),
        "texture" => Some(icons::TEXTURE),
        "table-row-plus-before" => Some(icons::TABLE_ROW_PLUS_BEFORE),
        "selection-search" => Some(icons::SELECTION_SEARCH),
        "hand-front-left" => Some(icons::HAND_FRONT_LEFT),
        "podcast" => Some(icons::PODCAST),
        "timer-refresh" => Some(icons::TIMER_REFRESH),
        "chart-timeline-variant-shimmer" => Some(icons::CHART_TIMELINE_VARIANT_SHIMMER),
        "torch" => Some(icons::TORCH),
        "dance-pole" => Some(icons::DANCE_POLE),
        "table-refresh" => Some(icons::TABLE_REFRESH),
        "battery-high" => Some(icons::BATTERY_HIGH),
        "chart-histogram" => Some(icons::CHART_HISTOGRAM),
        "folder-check-outline" => Some(icons::FOLDER_CHECK_OUTLINE),
        "disc-player" => Some(icons::DISC_PLAYER),
        "export" => Some(icons::EXPORT),
        "land-plots-circle-variant" => Some(icons::LAND_PLOTS_CIRCLE_VARIANT),
        "format-quote-open" => Some(icons::FORMAT_QUOTE_OPEN),
        "video-check-outline" => Some(icons::VIDEO_CHECK_OUTLINE),
        "code-greater-than" => Some(icons::CODE_GREATER_THAN),
        "text-box-remove-outline" => Some(icons::TEXT_BOX_REMOVE_OUTLINE),
        "currency-rub" => Some(icons::CURRENCY_RUB),
        "blinds-vertical-closed" => Some(icons::BLINDS_VERTICAL_CLOSED),
        "alpha-o-box-outline" => Some(icons::ALPHA_O_BOX_OUTLINE),
        #[allow(deprecated)]
        "minecraft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'minecraft' is deprecated.").print(py);
            }
            Some(icons::MINECRAFT)
        }
        "string-lights" => Some(icons::STRING_LIGHTS),
        "parachute" => Some(icons::PARACHUTE),
        #[allow(deprecated)]
        "youtube-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-gaming' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_GAMING)
        }
        "temple-buddhist-outline" => Some(icons::TEMPLE_BUDDHIST_OUTLINE),
        "arrow-decision" => Some(icons::ARROW_DECISION),
        "escalator-up" => Some(icons::ESCALATOR_UP),
        "tower-fire" => Some(icons::TOWER_FIRE),
        "death-star-variant" => Some(icons::DEATH_STAR_VARIANT),
        "podium" => Some(icons::PODIUM),
        "progress-star" => Some(icons::PROGRESS_STAR),
        "car-defrost-rear" => Some(icons::CAR_DEFROST_REAR),
        "wall-sconce" => Some(icons::WALL_SCONCE),
        "key-link" => Some(icons::KEY_LINK),
        "selection-drag" => Some(icons::SELECTION_DRAG),
        "numeric-10-box-multiple" => Some(icons::NUMERIC_10_BOX_MULTIPLE),
        "screw-flat-top" => Some(icons::SCREW_FLAT_TOP),
        "coat-rack" => Some(icons::COAT_RACK),
        "synagogue-outline" => Some(icons::SYNAGOGUE_OUTLINE),
        #[allow(deprecated)]
        "google-circles-extended" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles-extended' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_EXTENDED)
        }
        "all-inclusive" => Some(icons::ALL_INCLUSIVE),
        "folder-key" => Some(icons::FOLDER_KEY),
        "format-line-spacing" => Some(icons::FORMAT_LINE_SPACING),
        "clock-fast" => Some(icons::CLOCK_FAST),
        "vector-selection" => Some(icons::VECTOR_SELECTION),
        "ev-plug-type2" => Some(icons::EV_PLUG_TYPE2),
        "note-off" => Some(icons::NOTE_OFF),
        "backup-restore" => Some(icons::BACKUP_RESTORE),
        "patio-heater" => Some(icons::PATIO_HEATER),
        "arrow-up-bold-box" => Some(icons::ARROW_UP_BOLD_BOX),
        "car-wireless" => Some(icons::CAR_WIRELESS),
        "thermometer-low" => Some(icons::THERMOMETER_LOW),
        "api-off" => Some(icons::API_OFF),
        "battery-charging-20" => Some(icons::BATTERY_CHARGING_20),
        "play-circle" => Some(icons::PLAY_CIRCLE),
        "attachment-check" => Some(icons::ATTACHMENT_CHECK),
        "help-box-outline" => Some(icons::HELP_BOX_OUTLINE),
        #[allow(deprecated)]
        "linux-mint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux-mint' is deprecated.").print(py);
            }
            Some(icons::LINUX_MINT)
        }
        "whistle-outline" => Some(icons::WHISTLE_OUTLINE),
        "hammer" => Some(icons::HAMMER),
        "kettle-alert" => Some(icons::KETTLE_ALERT),
        "cloud-sync" => Some(icons::CLOUD_SYNC),
        "power-sleep" => Some(icons::POWER_SLEEP),
        "seat-legroom-normal" => Some(icons::SEAT_LEGROOM_NORMAL),
        "alpha-h-box" => Some(icons::ALPHA_H_BOX),
        "file-star-outline" => Some(icons::FILE_STAR_OUTLINE),
        "puzzle-plus" => Some(icons::PUZZLE_PLUS),
        "invoice-plus" => Some(icons::INVOICE_PLUS),
        "form-textbox" => Some(icons::FORM_TEXTBOX),
        "check-outline" => Some(icons::CHECK_OUTLINE),
        "printer-pos-cancel" => Some(icons::PRINTER_POS_CANCEL),
        "star-three-points" => Some(icons::STAR_THREE_POINTS),
        "file-cad" => Some(icons::FILE_CAD),
        "cup-off" => Some(icons::CUP_OFF),
        "lock-percent" => Some(icons::LOCK_PERCENT),
        "lightbulb-on-70" => Some(icons::LIGHTBULB_ON_70),
        "compass-outline" => Some(icons::COMPASS_OUTLINE),
        "account-network-outline" => Some(icons::ACCOUNT_NETWORK_OUTLINE),
        "television-stop" => Some(icons::TELEVISION_STOP),
        "dock-top" => Some(icons::DOCK_TOP),
        "skip-backward" => Some(icons::SKIP_BACKWARD),
        "blur-radial" => Some(icons::BLUR_RADIAL),
        "calendar-remove" => Some(icons::CALENDAR_REMOVE),
        "bell-minus-outline" => Some(icons::BELL_MINUS_OUTLINE),
        "tournament" => Some(icons::TOURNAMENT),
        "clock-edit" => Some(icons::CLOCK_EDIT),
        "bat" => Some(icons::BAT),
        "silverware-fork-knife" => Some(icons::SILVERWARE_FORK_KNIFE),
        "database-remove" => Some(icons::DATABASE_REMOVE),
        "clippy" => Some(icons::CLIPPY),
        "folder-arrow-up-outline" => Some(icons::FOLDER_ARROW_UP_OUTLINE),
        "pencil-lock-outline" => Some(icons::PENCIL_LOCK_OUTLINE),
        "arrow-top-right-bold-box" => Some(icons::ARROW_TOP_RIGHT_BOLD_BOX),
        "printer-pos-refresh" => Some(icons::PRINTER_POS_REFRESH),
        "label-multiple" => Some(icons::LABEL_MULTIPLE),
        "text-box-plus" => Some(icons::TEXT_BOX_PLUS),
        "wallet" => Some(icons::WALLET),
        "tumble-dryer-off" => Some(icons::TUMBLE_DRYER_OFF),
        "table-arrow-right" => Some(icons::TABLE_ARROW_RIGHT),
        "folder-key-outline" => Some(icons::FOLDER_KEY_OUTLINE),
        "bus-stop-uncovered" => Some(icons::BUS_STOP_UNCOVERED),
        "help-circle" => Some(icons::HELP_CIRCLE),
        "hydrogen-station" => Some(icons::HYDROGEN_STATION),
        "currency-kzt" => Some(icons::CURRENCY_KZT),
        "music-accidental-double-sharp" => Some(icons::MUSIC_ACCIDENTAL_DOUBLE_SHARP),
        "weather-fog" => Some(icons::WEATHER_FOG),
        "draw-pen" => Some(icons::DRAW_PEN),
        "wall-sconce-round-variant" => Some(icons::WALL_SCONCE_ROUND_VARIANT),
        "book-open-variant-outline" => Some(icons::BOOK_OPEN_VARIANT_OUTLINE),
        "gesture-swipe-down" => Some(icons::GESTURE_SWIPE_DOWN),
        "knife-military" => Some(icons::KNIFE_MILITARY),
        "eye-remove" => Some(icons::EYE_REMOVE),
        "axis-arrow" => Some(icons::AXIS_ARROW),
        "car-esp" => Some(icons::CAR_ESP),
        "record-rec" => Some(icons::RECORD_REC),
        "wifi-strength-off" => Some(icons::WIFI_STRENGTH_OFF),
        "filter-multiple-outline" => Some(icons::FILTER_MULTIPLE_OUTLINE),
        "car-connected" => Some(icons::CAR_CONNECTED),
        "bird" => Some(icons::BIRD),
        "format-bold" => Some(icons::FORMAT_BOLD),
        "glass-cocktail" => Some(icons::GLASS_COCKTAIL),
        "content-save-minus" => Some(icons::CONTENT_SAVE_MINUS),
        "server-minus" => Some(icons::SERVER_MINUS),
        "cog-pause-outline" => Some(icons::COG_PAUSE_OUTLINE),
        "escalator-down" => Some(icons::ESCALATOR_DOWN),
        "file-document-alert-outline" => Some(icons::FILE_DOCUMENT_ALERT_OUTLINE),
        "snowflake" => Some(icons::SNOWFLAKE),
        "calendar-lock" => Some(icons::CALENDAR_LOCK),
        "alpha-h-circle" => Some(icons::ALPHA_H_CIRCLE),
        "phone-settings-outline" => Some(icons::PHONE_SETTINGS_OUTLINE),
        "vector-arrange-below" => Some(icons::VECTOR_ARRANGE_BELOW),
        "motion-play-outline" => Some(icons::MOTION_PLAY_OUTLINE),
        "rollerblade-off" => Some(icons::ROLLERBLADE_OFF),
        "family-tree" => Some(icons::FAMILY_TREE),
        "rhombus-split-outline" => Some(icons::RHOMBUS_SPLIT_OUTLINE),
        "battery-plus" => Some(icons::BATTERY_PLUS),
        "eye-settings" => Some(icons::EYE_SETTINGS),
        "pulse" => Some(icons::PULSE),
        "format-font" => Some(icons::FORMAT_FONT),
        #[allow(deprecated)]
        "oci" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'oci' is deprecated.").print(py);
            }
            Some(icons::OCI)
        }
        "home-import-outline" => Some(icons::HOME_IMPORT_OUTLINE),
        "spray" => Some(icons::SPRAY),
        "basket" => Some(icons::BASKET),
        "water-boiler" => Some(icons::WATER_BOILER),
        "cloud-remove" => Some(icons::CLOUD_REMOVE),
        "book-arrow-right-outline" => Some(icons::BOOK_ARROW_RIGHT_OUTLINE),
        "battery-heart-outline" => Some(icons::BATTERY_HEART_OUTLINE),
        "human-handsup" => Some(icons::HUMAN_HANDSUP),
        "message-text-outline" => Some(icons::MESSAGE_TEXT_OUTLINE),
        "dice-d20-outline" => Some(icons::DICE_D20_OUTLINE),
        _ => None,
    }
}
