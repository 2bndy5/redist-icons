// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_26(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "gesture" => Some(icons::GESTURE),
        "store-check" => Some(icons::STORE_CHECK),
        "crop-landscape" => Some(icons::CROP_LANDSCAPE),
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        "set-all" => Some(icons::SET_ALL),
        "file-settings" => Some(icons::FILE_SETTINGS),
        "cash-check" => Some(icons::CASH_CHECK),
        "wifi-arrow-up" => Some(icons::WIFI_ARROW_UP),
        "heart-box" => Some(icons::HEART_BOX),
        "format-font-size-decrease" => Some(icons::FORMAT_FONT_SIZE_DECREASE),
        "boomerang" => Some(icons::BOOMERANG),
        "city-variant" => Some(icons::CITY_VARIANT),
        "hamburger-plus" => Some(icons::HAMBURGER_PLUS),
        "compost" => Some(icons::COMPOST),
        "order-bool-ascending-variant" => Some(icons::ORDER_BOOL_ASCENDING_VARIANT),
        "domain" => Some(icons::DOMAIN),
        "file-excel-outline" => Some(icons::FILE_EXCEL_OUTLINE),
        "arrow-bottom-left-bold-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_OUTLINE),
        "gesture-swipe-up" => Some(icons::GESTURE_SWIPE_UP),
        "power-standby" => Some(icons::POWER_STANDBY),
        "wall-sconce" => Some(icons::WALL_SCONCE),
        "harddisk-remove" => Some(icons::HARDDISK_REMOVE),
        "folder-swap-outline" => Some(icons::FOLDER_SWAP_OUTLINE),
        "comment-eye-outline" => Some(icons::COMMENT_EYE_OUTLINE),
        "desk-lamp" => Some(icons::DESK_LAMP),
        "align-vertical-distribute" => Some(icons::ALIGN_VERTICAL_DISTRIBUTE),
        "alpha-p-box" => Some(icons::ALPHA_P_BOX),
        "keyboard-variant" => Some(icons::KEYBOARD_VARIANT),
        "cake" => Some(icons::CAKE),
        "content-save-move" => Some(icons::CONTENT_SAVE_MOVE),
        "cog-pause-outline" => Some(icons::COG_PAUSE_OUTLINE),
        "octahedron-off" => Some(icons::OCTAHEDRON_OFF),
        "credit-card-marker" => Some(icons::CREDIT_CARD_MARKER),
        "arrow-decision-outline" => Some(icons::ARROW_DECISION_OUTLINE),
        "wallet-giftcard" => Some(icons::WALLET_GIFTCARD),
        "ocarina" => Some(icons::OCARINA),
        "puzzle-minus" => Some(icons::PUZZLE_MINUS),
        "dog-side" => Some(icons::DOG_SIDE),
        #[allow(deprecated)]
        "language-swift" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-swift' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_SWIFT)
        }
        "wifi-minus" => Some(icons::WIFI_MINUS),
        "format-align-middle" => Some(icons::FORMAT_ALIGN_MIDDLE),
        "circle-double" => Some(icons::CIRCLE_DOUBLE),
        "palette" => Some(icons::PALETTE),
        #[allow(deprecated)]
        "freebsd" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'freebsd' is deprecated.").print(py);
            }
            Some(icons::FREEBSD)
        }
        "magnify" => Some(icons::MAGNIFY),
        "file-tree-outline" => Some(icons::FILE_TREE_OUTLINE),
        "phone-ring" => Some(icons::PHONE_RING),
        "lock-alert-outline" => Some(icons::LOCK_ALERT_OUTLINE),
        "alpha-o-box-outline" => Some(icons::ALPHA_O_BOX_OUTLINE),
        "wrench-cog-outline" => Some(icons::WRENCH_COG_OUTLINE),
        "hexagon-multiple-outline" => Some(icons::HEXAGON_MULTIPLE_OUTLINE),
        "broadcast-off" => Some(icons::BROADCAST_OFF),
        "minus-circle-multiple" => Some(icons::MINUS_CIRCLE_MULTIPLE),
        "flask-minus" => Some(icons::FLASK_MINUS),
        #[allow(deprecated)]
        "aws" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'aws' is deprecated.").print(py);
            }
            Some(icons::AWS)
        }
        "car-esp" => Some(icons::CAR_ESP),
        "numeric-0" => Some(icons::NUMERIC_0),
        "format-paragraph-spacing" => Some(icons::FORMAT_PARAGRAPH_SPACING),
        "egg-easter" => Some(icons::EGG_EASTER),
        "progress-clock" => Some(icons::PROGRESS_CLOCK),
        "send-lock" => Some(icons::SEND_LOCK),
        "music-note-eighth-dotted" => Some(icons::MUSIC_NOTE_EIGHTH_DOTTED),
        "align-horizontal-right" => Some(icons::ALIGN_HORIZONTAL_RIGHT),
        "cog-counterclockwise" => Some(icons::COG_COUNTERCLOCKWISE),
        "usb-flash-drive-outline" => Some(icons::USB_FLASH_DRIVE_OUTLINE),
        "chart-gantt" => Some(icons::CHART_GANTT),
        #[allow(deprecated)]
        "google-podcast" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-podcast' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PODCAST)
        }
        "file-move-outline" => Some(icons::FILE_MOVE_OUTLINE),
        "close-thick" => Some(icons::CLOSE_THICK),
        "offer" => Some(icons::OFFER),
        "access-point-check" => Some(icons::ACCESS_POINT_CHECK),
        "chart-bar-stacked" => Some(icons::CHART_BAR_STACKED),
        "message-image" => Some(icons::MESSAGE_IMAGE),
        "battery-60-bluetooth" => Some(icons::BATTERY_60_BLUETOOTH),
        "cookie-check" => Some(icons::COOKIE_CHECK),
        "signal-variant" => Some(icons::SIGNAL_VARIANT),
        "application" => Some(icons::APPLICATION),
        "eye-lock-open-outline" => Some(icons::EYE_LOCK_OPEN_OUTLINE),
        "folder-check" => Some(icons::FOLDER_CHECK),
        #[allow(deprecated)]
        "bitcoin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bitcoin' is deprecated.").print(py);
            }
            Some(icons::BITCOIN)
        }
        "home-floor-b" => Some(icons::HOME_FLOOR_B),
        "folder-off-outline" => Some(icons::FOLDER_OFF_OUTLINE),
        "presentation-play" => Some(icons::PRESENTATION_PLAY),
        #[allow(deprecated)]
        "black-mesa" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'black-mesa' is deprecated.").print(py);
            }
            Some(icons::BLACK_MESA)
        }
        "timer-settings-outline" => Some(icons::TIMER_SETTINGS_OUTLINE),
        "microphone-message-off" => Some(icons::MICROPHONE_MESSAGE_OFF),
        "swap-horizontal-circle" => Some(icons::SWAP_HORIZONTAL_CIRCLE),
        "account-search-outline" => Some(icons::ACCOUNT_SEARCH_OUTLINE),
        "arrange-bring-forward" => Some(icons::ARRANGE_BRING_FORWARD),
        "bookmark-check-outline" => Some(icons::BOOKMARK_CHECK_OUTLINE),
        "candy-outline" => Some(icons::CANDY_OUTLINE),
        "shaker" => Some(icons::SHAKER),
        "cash-lock" => Some(icons::CASH_LOCK),
        "camera-iris" => Some(icons::CAMERA_IRIS),
        #[allow(deprecated)]
        "nintendo-wiiu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wiiu' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WIIU)
        }
        "abugida-thai" => Some(icons::ABUGIDA_THAI),
        "bed-empty" => Some(icons::BED_EMPTY),
        "camera-wireless-outline" => Some(icons::CAMERA_WIRELESS_OUTLINE),
        "ultra-high-definition" => Some(icons::ULTRA_HIGH_DEFINITION),
        "chat-plus" => Some(icons::CHAT_PLUS),
        "star-four-points-small" => Some(icons::STAR_FOUR_POINTS_SMALL),
        "square-rounded-badge-outline" => Some(icons::SQUARE_ROUNDED_BADGE_OUTLINE),
        "bug-pause" => Some(icons::BUG_PAUSE),
        "hand-wash" => Some(icons::HAND_WASH),
        "handball" => Some(icons::HANDBALL),
        "gamepad-left" => Some(icons::GAMEPAD_LEFT),
        "bell-circle-outline" => Some(icons::BELL_CIRCLE_OUTLINE),
        "tshirt-crew-outline" => Some(icons::TSHIRT_CREW_OUTLINE),
        "information-variant" => Some(icons::INFORMATION_VARIANT),
        "access-point-plus" => Some(icons::ACCESS_POINT_PLUS),
        "image-marker-outline" => Some(icons::IMAGE_MARKER_OUTLINE),
        "set-left-center" => Some(icons::SET_LEFT_CENTER),
        "dishwasher-off" => Some(icons::DISHWASHER_OFF),
        "select-compare" => Some(icons::SELECT_COMPARE),
        "speaker-bluetooth" => Some(icons::SPEAKER_BLUETOOTH),
        "calendar-clock" => Some(icons::CALENDAR_CLOCK),
        "attachment-lock" => Some(icons::ATTACHMENT_LOCK),
        "flask-remove" => Some(icons::FLASK_REMOVE),
        "clock-time-six" => Some(icons::CLOCK_TIME_SIX),
        "incognito-circle-off" => Some(icons::INCOGNITO_CIRCLE_OFF),
        #[allow(deprecated)]
        "google-glass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-glass' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_GLASS)
        }
        "emoticon-lol" => Some(icons::EMOTICON_LOL),
        "plus-box" => Some(icons::PLUS_BOX),
        "tag-arrow-down-outline" => Some(icons::TAG_ARROW_DOWN_OUTLINE),
        "transfer-right" => Some(icons::TRANSFER_RIGHT),
        "robot-love" => Some(icons::ROBOT_LOVE),
        "database-search" => Some(icons::DATABASE_SEARCH),
        "upload" => Some(icons::UPLOAD),
        "smart-card-outline" => Some(icons::SMART_CARD_OUTLINE),
        "thermometer-lines" => Some(icons::THERMOMETER_LINES),
        "vector-intersection" => Some(icons::VECTOR_INTERSECTION),
        "roller-shade-closed" => Some(icons::ROLLER_SHADE_CLOSED),
        "alert-minus" => Some(icons::ALERT_MINUS),
        "resize-bottom-right" => Some(icons::RESIZE_BOTTOM_RIGHT),
        "arrow-u-up-left-bold" => Some(icons::ARROW_U_UP_LEFT_BOLD),
        "creation" => Some(icons::CREATION),
        "smoke-detector" => Some(icons::SMOKE_DETECTOR),
        "alpha-o" => Some(icons::ALPHA_O),
        "circle-multiple-outline" => Some(icons::CIRCLE_MULTIPLE_OUTLINE),
        "alpha-j-box-outline" => Some(icons::ALPHA_J_BOX_OUTLINE),
        "projector-screen-variant-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OUTLINE),
        "account-tie-hat-outline" => Some(icons::ACCOUNT_TIE_HAT_OUTLINE),
        "weather-sunset-down" => Some(icons::WEATHER_SUNSET_DOWN),
        "decimal" => Some(icons::DECIMAL),
        "tag-heart-outline" => Some(icons::TAG_HEART_OUTLINE),
        "heart-circle-outline" => Some(icons::HEART_CIRCLE_OUTLINE),
        "square-opacity" => Some(icons::SQUARE_OPACITY),
        "go-kart-track" => Some(icons::GO_KART_TRACK),
        "radioactive-off" => Some(icons::RADIOACTIVE_OFF),
        "umbrella-closed-outline" => Some(icons::UMBRELLA_CLOSED_OUTLINE),
        "database-alert-outline" => Some(icons::DATABASE_ALERT_OUTLINE),
        "music-rest-half" => Some(icons::MUSIC_REST_HALF),
        "printer-pos" => Some(icons::PRINTER_POS),
        "battery-arrow-up-outline" => Some(icons::BATTERY_ARROW_UP_OUTLINE),
        "format-color-text" => Some(icons::FORMAT_COLOR_TEXT),
        "phone-incoming-outline" => Some(icons::PHONE_INCOMING_OUTLINE),
        "book-play-outline" => Some(icons::BOOK_PLAY_OUTLINE),
        "air-humidifier-off" => Some(icons::AIR_HUMIDIFIER_OFF),
        "clock-time-five-outline" => Some(icons::CLOCK_TIME_FIVE_OUTLINE),
        "train-car-flatbed-tank" => Some(icons::TRAIN_CAR_FLATBED_TANK),
        "battery-40" => Some(icons::BATTERY_40),
        "format-list-bulleted" => Some(icons::FORMAT_LIST_BULLETED),
        #[allow(deprecated)]
        "trello" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'trello' is deprecated.").print(py);
            }
            Some(icons::TRELLO)
        }
        "folder-off" => Some(icons::FOLDER_OFF),
        "table-eye" => Some(icons::TABLE_EYE),
        "alpha-t-circle" => Some(icons::ALPHA_T_CIRCLE),
        "camera-switch-outline" => Some(icons::CAMERA_SWITCH_OUTLINE),
        "battery-30" => Some(icons::BATTERY_30),
        #[allow(deprecated)]
        "google-circles" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CIRCLES)
        }
        "badge-account-horizontal" => Some(icons::BADGE_ACCOUNT_HORIZONTAL),
        "printer-pos-check-outline" => Some(icons::PRINTER_POS_CHECK_OUTLINE),
        "message-question-outline" => Some(icons::MESSAGE_QUESTION_OUTLINE),
        "currency-uah" => Some(icons::CURRENCY_UAH),
        "square-rounded-outline" => Some(icons::SQUARE_ROUNDED_OUTLINE),
        "coffee-maker-outline" => Some(icons::COFFEE_MAKER_OUTLINE),
        "cast-variant" => Some(icons::CAST_VARIANT),
        "basket-fill" => Some(icons::BASKET_FILL),
        "cloud-upload-outline" => Some(icons::CLOUD_UPLOAD_OUTLINE),
        "invoice-text-send-outline" => Some(icons::INVOICE_TEXT_SEND_OUTLINE),
        "ferry" => Some(icons::FERRY),
        "cursor-default-click-outline" => Some(icons::CURSOR_DEFAULT_CLICK_OUTLINE),
        "alarm-check" => Some(icons::ALARM_CHECK),
        "calendar-month-outline" => Some(icons::CALENDAR_MONTH_OUTLINE),
        "table-off" => Some(icons::TABLE_OFF),
        "filmstrip-off" => Some(icons::FILMSTRIP_OFF),
        "chart-scatter-plot" => Some(icons::CHART_SCATTER_PLOT),
        "arrow-left-top-bold" => Some(icons::ARROW_LEFT_TOP_BOLD),
        "folder-hidden" => Some(icons::FOLDER_HIDDEN),
        "bag-personal-outline" => Some(icons::BAG_PERSONAL_OUTLINE),
        "radio-off" => Some(icons::RADIO_OFF),
        "star-half" => Some(icons::STAR_HALF),
        "tally-mark-2" => Some(icons::TALLY_MARK_2),
        "receipt-text-clock-outline" => Some(icons::RECEIPT_TEXT_CLOCK_OUTLINE),
        "pause-box" => Some(icons::PAUSE_BOX),
        "pan-right" => Some(icons::PAN_RIGHT),
        "car-wireless" => Some(icons::CAR_WIRELESS),
        "bookmark-minus-outline" => Some(icons::BOOKMARK_MINUS_OUTLINE),
        "arrow-down-bold-outline" => Some(icons::ARROW_DOWN_BOLD_OUTLINE),
        "file-import-outline" => Some(icons::FILE_IMPORT_OUTLINE),
        #[allow(deprecated)]
        "firefox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firefox' is deprecated.").print(py);
            }
            Some(icons::FIREFOX)
        }
        _ => None,
    }
}
