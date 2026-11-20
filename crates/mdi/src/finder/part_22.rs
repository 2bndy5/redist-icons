// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_22(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "account-details" => Some(icons::ACCOUNT_DETAILS),
        "seat-individual-suite" => Some(icons::SEAT_INDIVIDUAL_SUITE),
        "book-cog" => Some(icons::BOOK_COG),
        "book-sync" => Some(icons::BOOK_SYNC),
        "note-multiple" => Some(icons::NOTE_MULTIPLE),
        "application-parentheses-outline" => Some(icons::APPLICATION_PARENTHESES_OUTLINE),
        "elevator-passenger-off-outline" => Some(icons::ELEVATOR_PASSENGER_OFF_OUTLINE),
        "lamps-outline" => Some(icons::LAMPS_OUTLINE),
        "nfc-search-variant" => Some(icons::NFC_SEARCH_VARIANT),
        "railroad-light" => Some(icons::RAILROAD_LIGHT),
        "arrow-u-right-top-bold" => Some(icons::ARROW_U_RIGHT_TOP_BOLD),
        "database-minus-outline" => Some(icons::DATABASE_MINUS_OUTLINE),
        "ip" => Some(icons::IP),
        "clipboard-list" => Some(icons::CLIPBOARD_LIST),
        "eye-settings-outline" => Some(icons::EYE_SETTINGS_OUTLINE),
        "pan-right" => Some(icons::PAN_RIGHT),
        "music-rest-quarter" => Some(icons::MUSIC_REST_QUARTER),
        "robot-off-outline" => Some(icons::ROBOT_OFF_OUTLINE),
        "gas-burner" => Some(icons::GAS_BURNER),
        "tag-check-outline" => Some(icons::TAG_CHECK_OUTLINE),
        "delete-off" => Some(icons::DELETE_OFF),
        "ship-wheel" => Some(icons::SHIP_WHEEL),
        "account-arrow-up-outline" => Some(icons::ACCOUNT_ARROW_UP_OUTLINE),
        "credit-card-refund" => Some(icons::CREDIT_CARD_REFUND),
        "cube-unfolded" => Some(icons::CUBE_UNFOLDED),
        "database-off-outline" => Some(icons::DATABASE_OFF_OUTLINE),
        "publish-off" => Some(icons::PUBLISH_OFF),
        "format-align-middle" => Some(icons::FORMAT_ALIGN_MIDDLE),
        "check-network-outline" => Some(icons::CHECK_NETWORK_OUTLINE),
        "head-heart-outline" => Some(icons::HEAD_HEART_OUTLINE),
        "battery-charging-wireless-alert" => Some(icons::BATTERY_CHARGING_WIRELESS_ALERT),
        "emoticon-excited-outline" => Some(icons::EMOTICON_EXCITED_OUTLINE),
        "case-sensitive-alt" => Some(icons::CASE_SENSITIVE_ALT),
        "lock-minus" => Some(icons::LOCK_MINUS),
        "rickshaw-electric" => Some(icons::RICKSHAW_ELECTRIC),
        "land-rows-vertical" => Some(icons::LAND_ROWS_VERTICAL),
        "floor-plan" => Some(icons::FLOOR_PLAN),
        "view-carousel-outline" => Some(icons::VIEW_CAROUSEL_OUTLINE),
        "axis-z-arrow-lock" => Some(icons::AXIS_Z_ARROW_LOCK),
        "numeric-6-box-multiple-outline" => Some(icons::NUMERIC_6_BOX_MULTIPLE_OUTLINE),
        "format-text-rotation-down" => Some(icons::FORMAT_TEXT_ROTATION_DOWN),
        "account-box-multiple-outline" => Some(icons::ACCOUNT_BOX_MULTIPLE_OUTLINE),
        "post-outline" => Some(icons::POST_OUTLINE),
        "train-car-flatbed-car" => Some(icons::TRAIN_CAR_FLATBED_CAR),
        "note-remove" => Some(icons::NOTE_REMOVE),
        "arrow-right-bottom-bold" => Some(icons::ARROW_RIGHT_BOTTOM_BOLD),
        "book-arrow-down" => Some(icons::BOOK_ARROW_DOWN),
        "umbrella-closed-variant" => Some(icons::UMBRELLA_CLOSED_VARIANT),
        "handball" => Some(icons::HANDBALL),
        "vector-square" => Some(icons::VECTOR_SQUARE),
        "calendar-star-four-points" => Some(icons::CALENDAR_STAR_FOUR_POINTS),
        "chart-timeline-variant" => Some(icons::CHART_TIMELINE_VARIANT),
        "content-paste" => Some(icons::CONTENT_PASTE),
        "kite-outline" => Some(icons::KITE_OUTLINE),
        "play-box" => Some(icons::PLAY_BOX),
        "vector-circle" => Some(icons::VECTOR_CIRCLE),
        "mortar-pestle-plus" => Some(icons::MORTAR_PESTLE_PLUS),
        "select-off" => Some(icons::SELECT_OFF),
        "chevron-down-box" => Some(icons::CHEVRON_DOWN_BOX),
        "vector-radius" => Some(icons::VECTOR_RADIUS),
        "clock-time-eleven-outline" => Some(icons::CLOCK_TIME_ELEVEN_OUTLINE),
        "pencil-box-multiple" => Some(icons::PENCIL_BOX_MULTIPLE),
        "calendar-multiple-check" => Some(icons::CALENDAR_MULTIPLE_CHECK),
        "battery-alert" => Some(icons::BATTERY_ALERT),
        "cash-fast" => Some(icons::CASH_FAST),
        "archive-outline" => Some(icons::ARCHIVE_OUTLINE),
        "hamburger-check" => Some(icons::HAMBURGER_CHECK),
        "bell-plus-outline" => Some(icons::BELL_PLUS_OUTLINE),
        "circle-medium" => Some(icons::CIRCLE_MEDIUM),
        "progress-alert" => Some(icons::PROGRESS_ALERT),
        "barrel-outline" => Some(icons::BARREL_OUTLINE),
        "weather-tornado" => Some(icons::WEATHER_TORNADO),
        "folder-minus-outline" => Some(icons::FOLDER_MINUS_OUTLINE),
        "alpha-m" => Some(icons::ALPHA_M),
        "laser-pointer" => Some(icons::LASER_POINTER),
        "piano" => Some(icons::PIANO),
        "subway-variant" => Some(icons::SUBWAY_VARIANT),
        "lotion-plus" => Some(icons::LOTION_PLUS),
        "numeric-5" => Some(icons::NUMERIC_5),
        "cube-off-outline" => Some(icons::CUBE_OFF_OUTLINE),
        "folder-home" => Some(icons::FOLDER_HOME),
        #[allow(deprecated)]
        "google-hangouts" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-hangouts' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_HANGOUTS)
        }
        "abjad-arabic" => Some(icons::ABJAD_ARABIC),
        "home-sound-in-outline" => Some(icons::HOME_SOUND_IN_OUTLINE),
        "flag-variant-minus" => Some(icons::FLAG_VARIANT_MINUS),
        "book-arrow-up-outline" => Some(icons::BOOK_ARROW_UP_OUTLINE),
        "dice-d12-outline" => Some(icons::DICE_D12_OUTLINE),
        "view-parallel" => Some(icons::VIEW_PARALLEL),
        "smoke-detector-off" => Some(icons::SMOKE_DETECTOR_OFF),
        "cone-off" => Some(icons::CONE_OFF),
        "truck-minus-outline" => Some(icons::TRUCK_MINUS_OUTLINE),
        "roller-skate" => Some(icons::ROLLER_SKATE),
        "rewind-5" => Some(icons::REWIND_5),
        "microphone-message" => Some(icons::MICROPHONE_MESSAGE),
        "cellphone-off" => Some(icons::CELLPHONE_OFF),
        "gas-station-in-use-outline" => Some(icons::GAS_STATION_IN_USE_OUTLINE),
        "rename-outline" => Some(icons::RENAME_OUTLINE),
        "file-minus-outline" => Some(icons::FILE_MINUS_OUTLINE),
        "message-off" => Some(icons::MESSAGE_OFF),
        "alpha-l" => Some(icons::ALPHA_L),
        "notebook-multiple" => Some(icons::NOTEBOOK_MULTIPLE),
        "record" => Some(icons::RECORD),
        "coffee-maker-outline" => Some(icons::COFFEE_MAKER_OUTLINE),
        "keyboard-tab-reverse" => Some(icons::KEYBOARD_TAB_REVERSE),
        "arrow-collapse-vertical" => Some(icons::ARROW_COLLAPSE_VERTICAL),
        "cloud-off-outline" => Some(icons::CLOUD_OFF_OUTLINE),
        "selection" => Some(icons::SELECTION),
        "bag-suitcase-outline" => Some(icons::BAG_SUITCASE_OUTLINE),
        "cup-off" => Some(icons::CUP_OFF),
        "video-off" => Some(icons::VIDEO_OFF),
        "calendar-blank" => Some(icons::CALENDAR_BLANK),
        "printer-pos-star" => Some(icons::PRINTER_POS_STAR),
        "motorbike" => Some(icons::MOTORBIKE),
        "gate" => Some(icons::GATE),
        "alpha-i-circle-outline" => Some(icons::ALPHA_I_CIRCLE_OUTLINE),
        "motorbike-electric" => Some(icons::MOTORBIKE_ELECTRIC),
        "archive-plus-outline" => Some(icons::ARCHIVE_PLUS_OUTLINE),
        "view-dashboard-outline" => Some(icons::VIEW_DASHBOARD_OUTLINE),
        "tram" => Some(icons::TRAM),
        "table-row-plus-after" => Some(icons::TABLE_ROW_PLUS_AFTER),
        "grave-stone" => Some(icons::GRAVE_STONE),
        #[allow(deprecated)]
        "language-python" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-python' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_PYTHON)
        }
        "contactless-payment" => Some(icons::CONTACTLESS_PAYMENT),
        "network-strength-4-cog" => Some(icons::NETWORK_STRENGTH_4_COG),
        "drone" => Some(icons::DRONE),
        "file-document-arrow-right" => Some(icons::FILE_DOCUMENT_ARROW_RIGHT),
        "television-speaker-off" => Some(icons::TELEVISION_SPEAKER_OFF),
        "battery-charging-80" => Some(icons::BATTERY_CHARGING_80),
        "book-lock-open-outline" => Some(icons::BOOK_LOCK_OPEN_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-alert" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-alert' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_ALERT)
        }
        "aurora" => Some(icons::AURORA),
        "film" => Some(icons::FILM),
        "bicycle" => Some(icons::BICYCLE),
        "palm-tree" => Some(icons::PALM_TREE),
        "wheelchair" => Some(icons::WHEELCHAIR),
        "printer-pos-check-outline" => Some(icons::PRINTER_POS_CHECK_OUTLINE),
        "movie-open-minus" => Some(icons::MOVIE_OPEN_MINUS),
        "clipboard-text-multiple" => Some(icons::CLIPBOARD_TEXT_MULTIPLE),
        "timer-stop" => Some(icons::TIMER_STOP),
        #[allow(deprecated)]
        "whatsapp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'whatsapp' is deprecated.").print(py);
            }
            Some(icons::WHATSAPP)
        }
        "printer-pos-edit-outline" => Some(icons::PRINTER_POS_EDIT_OUTLINE),
        "pickaxe" => Some(icons::PICKAXE),
        "notebook" => Some(icons::NOTEBOOK),
        "monitor-small" => Some(icons::MONITOR_SMALL),
        "floor-lamp-torchiere" => Some(icons::FLOOR_LAMP_TORCHIERE),
        "battery-negative" => Some(icons::BATTERY_NEGATIVE),
        "harddisk" => Some(icons::HARDDISK),
        "source-fork" => Some(icons::SOURCE_FORK),
        "car-door-lock" => Some(icons::CAR_DOOR_LOCK),
        "cookie-settings" => Some(icons::COOKIE_SETTINGS),
        "printer-pos-edit" => Some(icons::PRINTER_POS_EDIT),
        "information" => Some(icons::INFORMATION),
        "tournament" => Some(icons::TOURNAMENT),
        "numeric-1-box" => Some(icons::NUMERIC_1_BOX),
        "flask-round-bottom-empty-outline" => Some(icons::FLASK_ROUND_BOTTOM_EMPTY_OUTLINE),
        "vector-line" => Some(icons::VECTOR_LINE),
        "white-balance-auto" => Some(icons::WHITE_BALANCE_AUTO),
        "hydraulic-oil-temperature" => Some(icons::HYDRAULIC_OIL_TEMPERATURE),
        "water-circle" => Some(icons::WATER_CIRCLE),
        "battery-bluetooth" => Some(icons::BATTERY_BLUETOOTH),
        "bottle-wine-outline" => Some(icons::BOTTLE_WINE_OUTLINE),
        "alpha-i" => Some(icons::ALPHA_I),
        "archive-cancel" => Some(icons::ARCHIVE_CANCEL),
        "database-search" => Some(icons::DATABASE_SEARCH),
        "comment-question" => Some(icons::COMMENT_QUESTION),
        "arrow-left-top-bold" => Some(icons::ARROW_LEFT_TOP_BOLD),
        "hdr-off" => Some(icons::HDR_OFF),
        "cart-minus" => Some(icons::CART_MINUS),
        "plus-network-outline" => Some(icons::PLUS_NETWORK_OUTLINE),
        "chevron-down-circle-outline" => Some(icons::CHEVRON_DOWN_CIRCLE_OUTLINE),
        "gamepad-circle-up" => Some(icons::GAMEPAD_CIRCLE_UP),
        "sale" => Some(icons::SALE),
        "file-document-multiple" => Some(icons::FILE_DOCUMENT_MULTIPLE),
        "ticket" => Some(icons::TICKET),
        "wall-sconce-flat" => Some(icons::WALL_SCONCE_FLAT),
        "fan-chevron-down" => Some(icons::FAN_CHEVRON_DOWN),
        "web-remove" => Some(icons::WEB_REMOVE),
        "lock-open-alert-outline" => Some(icons::LOCK_OPEN_ALERT_OUTLINE),
        "align-vertical-center" => Some(icons::ALIGN_VERTICAL_CENTER),
        "file-edit-outline" => Some(icons::FILE_EDIT_OUTLINE),
        "file-video-outline" => Some(icons::FILE_VIDEO_OUTLINE),
        "check-circle-outline" => Some(icons::CHECK_CIRCLE_OUTLINE),
        "power-socket-jp" => Some(icons::POWER_SOCKET_JP),
        "set-left-center" => Some(icons::SET_LEFT_CENTER),
        "table-column-remove" => Some(icons::TABLE_COLUMN_REMOVE),
        "volleyball" => Some(icons::VOLLEYBALL),
        "information-off" => Some(icons::INFORMATION_OFF),
        "pencil-box-multiple-outline" => Some(icons::PENCIL_BOX_MULTIPLE_OUTLINE),
        "dock-bottom" => Some(icons::DOCK_BOTTOM),
        "jump-rope" => Some(icons::JUMP_ROPE),
        "bag-personal-tag" => Some(icons::BAG_PERSONAL_TAG),
        "scent-off" => Some(icons::SCENT_OFF),
        "label-variant-outline" => Some(icons::LABEL_VARIANT_OUTLINE),
        "video-check-outline" => Some(icons::VIDEO_CHECK_OUTLINE),
        "panda" => Some(icons::PANDA),
        "roller-shade" => Some(icons::ROLLER_SHADE),
        "shark-fin" => Some(icons::SHARK_FIN),
        "check-outline" => Some(icons::CHECK_OUTLINE),
        "attachment-plus" => Some(icons::ATTACHMENT_PLUS),
        "comment-eye-outline" => Some(icons::COMMENT_EYE_OUTLINE),
        _ => None,
    }
}
