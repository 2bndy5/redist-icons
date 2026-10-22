// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_18(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "sticker-remove" => Some(icons::STICKER_REMOVE),
        "printer-pos-plus-outline" => Some(icons::PRINTER_POS_PLUS_OUTLINE),
        "image-filter-drama" => Some(icons::IMAGE_FILTER_DRAMA),
        #[allow(deprecated)]
        "odnoklassniki" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'odnoklassniki' is deprecated.").print(py);
            }
            Some(icons::ODNOKLASSNIKI)
        }
        "gift-off" => Some(icons::GIFT_OFF),
        "map-marker-plus" => Some(icons::MAP_MARKER_PLUS),
        "triangle-small-down" => Some(icons::TRIANGLE_SMALL_DOWN),
        "timer-check-outline" => Some(icons::TIMER_CHECK_OUTLINE),
        "map-marker" => Some(icons::MAP_MARKER),
        #[allow(deprecated)]
        "gmail" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gmail' is deprecated.").print(py);
            }
            Some(icons::GMAIL)
        }
        "syllabary-hangul" => Some(icons::SYLLABARY_HANGUL),
        "home-sound-in-outline" => Some(icons::HOME_SOUND_IN_OUTLINE),
        "chevron-left-box-outline" => Some(icons::CHEVRON_LEFT_BOX_OUTLINE),
        "star-shooting-outline" => Some(icons::STAR_SHOOTING_OUTLINE),
        "account-file-text-outline" => Some(icons::ACCOUNT_FILE_TEXT_OUTLINE),
        "speaker-play" => Some(icons::SPEAKER_PLAY),
        "inbox-multiple-outline" => Some(icons::INBOX_MULTIPLE_OUTLINE),
        "calendar-weekend-outline" => Some(icons::CALENDAR_WEEKEND_OUTLINE),
        "abjad-arabic" => Some(icons::ABJAD_ARABIC),
        "bullseye-arrow" => Some(icons::BULLSEYE_ARROW),
        "closed-caption-outline" => Some(icons::CLOSED_CAPTION_OUTLINE),
        "book-arrow-down" => Some(icons::BOOK_ARROW_DOWN),
        "candy-outline" => Some(icons::CANDY_OUTLINE),
        "text-box-check-outline" => Some(icons::TEXT_BOX_CHECK_OUTLINE),
        "math-integral-box" => Some(icons::MATH_INTEGRAL_BOX),
        "home-floor-3" => Some(icons::HOME_FLOOR_3),
        "flower" => Some(icons::FLOWER),
        "bell-remove" => Some(icons::BELL_REMOVE),
        "palette-outline" => Some(icons::PALETTE_OUTLINE),
        "bookmark-minus" => Some(icons::BOOKMARK_MINUS),
        "led-strip-variant-off" => Some(icons::LED_STRIP_VARIANT_OFF),
        "brush-variant" => Some(icons::BRUSH_VARIANT),
        "logout" => Some(icons::LOGOUT),
        "selection-multiple-marker" => Some(icons::SELECTION_MULTIPLE_MARKER),
        "clipboard-arrow-up" => Some(icons::CLIPBOARD_ARROW_UP),
        "home-flood" => Some(icons::HOME_FLOOD),
        "upload-network" => Some(icons::UPLOAD_NETWORK),
        "server-network" => Some(icons::SERVER_NETWORK),
        "message-cog" => Some(icons::MESSAGE_COG),
        "code-block-parentheses" => Some(icons::CODE_BLOCK_PARENTHESES),
        "comment-eye" => Some(icons::COMMENT_EYE),
        "boom-gate-arrow-up-outline" => Some(icons::BOOM_GATE_ARROW_UP_OUTLINE),
        "sun-snowflake-variant" => Some(icons::SUN_SNOWFLAKE_VARIANT),
        "weather-snowy-rainy" => Some(icons::WEATHER_SNOWY_RAINY),
        "cards-playing" => Some(icons::CARDS_PLAYING),
        "code-equal" => Some(icons::CODE_EQUAL),
        "tools" => Some(icons::TOOLS),
        "cellphone-nfc" => Some(icons::CELLPHONE_NFC),
        "flip-horizontal" => Some(icons::FLIP_HORIZONTAL),
        "file-document-multiple" => Some(icons::FILE_DOCUMENT_MULTIPLE),
        "shield-crown" => Some(icons::SHIELD_CROWN),
        "wifi-strength-off-outline" => Some(icons::WIFI_STRENGTH_OFF_OUTLINE),
        "bike" => Some(icons::BIKE),
        "go-kart" => Some(icons::GO_KART),
        "file-certificate" => Some(icons::FILE_CERTIFICATE),
        "video-check-outline" => Some(icons::VIDEO_CHECK_OUTLINE),
        "power-socket-de" => Some(icons::POWER_SOCKET_DE),
        "string-lights-off" => Some(icons::STRING_LIGHTS_OFF),
        "car-brake-alert" => Some(icons::CAR_BRAKE_ALERT),
        "format-size" => Some(icons::FORMAT_SIZE),
        "phone-incoming-outline" => Some(icons::PHONE_INCOMING_OUTLINE),
        "calendar-collapse-horizontal" => Some(icons::CALENDAR_COLLAPSE_HORIZONTAL),
        "selection-ellipse" => Some(icons::SELECTION_ELLIPSE),
        "message-arrow-left" => Some(icons::MESSAGE_ARROW_LEFT),
        "web-check" => Some(icons::WEB_CHECK),
        "zodiac-cancer" => Some(icons::ZODIAC_CANCER),
        "submarine" => Some(icons::SUBMARINE),
        "camera-party-mode" => Some(icons::CAMERA_PARTY_MODE),
        "autorenew-off" => Some(icons::AUTORENEW_OFF),
        "shape-plus-outline" => Some(icons::SHAPE_PLUS_OUTLINE),
        "brightness-7" => Some(icons::BRIGHTNESS_7),
        "clipboard-multiple" => Some(icons::CLIPBOARD_MULTIPLE),
        "radiology-box" => Some(icons::RADIOLOGY_BOX),
        "arrow-right-bold-outline" => Some(icons::ARROW_RIGHT_BOLD_OUTLINE),
        "microphone-question-outline" => Some(icons::MICROPHONE_QUESTION_OUTLINE),
        "cloud-cancel" => Some(icons::CLOUD_CANCEL),
        "newspaper-variant-outline" => Some(icons::NEWSPAPER_VARIANT_OUTLINE),
        #[allow(deprecated)]
        "yahoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'yahoo' is deprecated.").print(py);
            }
            Some(icons::YAHOO)
        }
        "tree-outline" => Some(icons::TREE_OUTLINE),
        "account-hard-hat" => Some(icons::ACCOUNT_HARD_HAT),
        "roman-numeral-9" => Some(icons::ROMAN_NUMERAL_9),
        "skull-outline" => Some(icons::SKULL_OUTLINE),
        "dice-5-outline" => Some(icons::DICE_5_OUTLINE),
        "boom-gate-up" => Some(icons::BOOM_GATE_UP),
        "account-filter-outline" => Some(icons::ACCOUNT_FILTER_OUTLINE),
        "home-alert-outline" => Some(icons::HOME_ALERT_OUTLINE),
        "plus-box-outline" => Some(icons::PLUS_BOX_OUTLINE),
        "diving-scuba-flag" => Some(icons::DIVING_SCUBA_FLAG),
        "view-sequential" => Some(icons::VIEW_SEQUENTIAL),
        "numeric-9-plus-circle" => Some(icons::NUMERIC_9_PLUS_CIRCLE),
        "shield-key-outline" => Some(icons::SHIELD_KEY_OUTLINE),
        "format-float-left" => Some(icons::FORMAT_FLOAT_LEFT),
        "chart-bubble" => Some(icons::CHART_BUBBLE),
        "map-marker-outline" => Some(icons::MAP_MARKER_OUTLINE),
        #[allow(deprecated)]
        "google-earth" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-earth' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_EARTH)
        }
        "numeric-9-plus-box-multiple-outline" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE_OUTLINE),
        "numeric-0" => Some(icons::NUMERIC_0),
        "clipboard-text-clock" => Some(icons::CLIPBOARD_TEXT_CLOCK),
        "note-alert-outline" => Some(icons::NOTE_ALERT_OUTLINE),
        "file-remove-outline" => Some(icons::FILE_REMOVE_OUTLINE),
        "upload-off-outline" => Some(icons::UPLOAD_OFF_OUTLINE),
        "water-pump-off" => Some(icons::WATER_PUMP_OFF),
        "file-question" => Some(icons::FILE_QUESTION),
        "note-outline" => Some(icons::NOTE_OUTLINE),
        "wall" => Some(icons::WALL),
        "numeric-2-box-outline" => Some(icons::NUMERIC_2_BOX_OUTLINE),
        "movie-open-plus-outline" => Some(icons::MOVIE_OPEN_PLUS_OUTLINE),
        "dice-d20-outline" => Some(icons::DICE_D20_OUTLINE),
        "format-color-fill" => Some(icons::FORMAT_COLOR_FILL),
        "truck-plus-outline" => Some(icons::TRUCK_PLUS_OUTLINE),
        "brightness-2" => Some(icons::BRIGHTNESS_2),
        "bunk-bed-outline" => Some(icons::BUNK_BED_OUTLINE),
        "octagram-minus-outline" => Some(icons::OCTAGRAM_MINUS_OUTLINE),
        "lightbulb-off" => Some(icons::LIGHTBULB_OFF),
        "van-passenger" => Some(icons::VAN_PASSENGER),
        "help" => Some(icons::HELP),
        "ray-end" => Some(icons::RAY_END),
        "hvac-off" => Some(icons::HVAC_OFF),
        "spoon-sugar" => Some(icons::SPOON_SUGAR),
        "pencil-remove" => Some(icons::PENCIL_REMOVE),
        "keyboard-backspace" => Some(icons::KEYBOARD_BACKSPACE),
        "checkbox-blank-circle" => Some(icons::CHECKBOX_BLANK_CIRCLE),
        "text-short" => Some(icons::TEXT_SHORT),
        "check-decagram" => Some(icons::CHECK_DECAGRAM),
        "order-alphabetical-ascending" => Some(icons::ORDER_ALPHABETICAL_ASCENDING),
        "silverware-variant" => Some(icons::SILVERWARE_VARIANT),
        "signal-5g" => Some(icons::SIGNAL_5G),
        "clipboard-plus-outline" => Some(icons::CLIPBOARD_PLUS_OUTLINE),
        "monitor-star" => Some(icons::MONITOR_STAR),
        "lightbulb-on" => Some(icons::LIGHTBULB_ON),
        "notebook-edit-outline" => Some(icons::NOTEBOOK_EDIT_OUTLINE),
        "microphone-variant-off" => Some(icons::MICROPHONE_VARIANT_OFF),
        "gamepad-circle" => Some(icons::GAMEPAD_CIRCLE),
        "office-building-remove" => Some(icons::OFFICE_BUILDING_REMOVE),
        "shield-star" => Some(icons::SHIELD_STAR),
        "pulse" => Some(icons::PULSE),
        "import" => Some(icons::IMPORT),
        #[allow(deprecated)]
        "twitch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitch' is deprecated.").print(py);
            }
            Some(icons::TWITCH)
        }
        "lock-open-minus-outline" => Some(icons::LOCK_OPEN_MINUS_OUTLINE),
        "minus" => Some(icons::MINUS),
        "plus-network-outline" => Some(icons::PLUS_NETWORK_OUTLINE),
        "calendar-collapse-horizontal-outline" => Some(icons::CALENDAR_COLLAPSE_HORIZONTAL_OUTLINE),
        "heart-half" => Some(icons::HEART_HALF),
        "cellphone-arrow-down-variant" => Some(icons::CELLPHONE_ARROW_DOWN_VARIANT),
        "wallet" => Some(icons::WALLET),
        "cloud-print" => Some(icons::CLOUD_PRINT),
        "blur" => Some(icons::BLUR),
        "flip-to-front" => Some(icons::FLIP_TO_FRONT),
        "file-lock-open-outline" => Some(icons::FILE_LOCK_OPEN_OUTLINE),
        "head-alert" => Some(icons::HEAD_ALERT),
        "view-comfy-outline" => Some(icons::VIEW_COMFY_OUTLINE),
        "propane-tank-outline" => Some(icons::PROPANE_TANK_OUTLINE),
        "border-style" => Some(icons::BORDER_STYLE),
        "pump-off" => Some(icons::PUMP_OFF),
        "clock-time-four-outline" => Some(icons::CLOCK_TIME_FOUR_OUTLINE),
        "layers-off" => Some(icons::LAYERS_OFF),
        "clock-time-twelve-outline" => Some(icons::CLOCK_TIME_TWELVE_OUTLINE),
        "arrow-top-right-thin-circle-outline" => Some(icons::ARROW_TOP_RIGHT_THIN_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "jquery" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jquery' is deprecated.").print(py);
            }
            Some(icons::JQUERY)
        }
        "scale-unbalanced" => Some(icons::SCALE_UNBALANCED),
        "clock-fast" => Some(icons::CLOCK_FAST),
        "compare-vertical" => Some(icons::COMPARE_VERTICAL),
        "professional-hexagon" => Some(icons::PROFESSIONAL_HEXAGON),
        "hand-back-left-off-outline" => Some(icons::HAND_BACK_LEFT_OFF_OUTLINE),
        "credit-card" => Some(icons::CREDIT_CARD),
        "note-text" => Some(icons::NOTE_TEXT),
        "folder-marker" => Some(icons::FOLDER_MARKER),
        "timeline-alert-outline" => Some(icons::TIMELINE_ALERT_OUTLINE),
        "qrcode-plus" => Some(icons::QRCODE_PLUS),
        "store-plus-outline" => Some(icons::STORE_PLUS_OUTLINE),
        "plus-box-multiple" => Some(icons::PLUS_BOX_MULTIPLE),
        "numeric-2-circle-outline" => Some(icons::NUMERIC_2_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "language-cpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-cpp' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CPP)
        }
        "tablet-cellphone" => Some(icons::TABLET_CELLPHONE),
        "progress-alert" => Some(icons::PROGRESS_ALERT),
        "panorama-vertical" => Some(icons::PANORAMA_VERTICAL),
        "folder-edit-outline" => Some(icons::FOLDER_EDIT_OUTLINE),
        "rectangle-outline" => Some(icons::RECTANGLE_OUTLINE),
        "projector-screen-variant-off-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF_OUTLINE),
        "cellphone-cog" => Some(icons::CELLPHONE_COG),
        "send-variant-clock" => Some(icons::SEND_VARIANT_CLOCK),
        "chair-school" => Some(icons::CHAIR_SCHOOL),
        "newspaper-minus" => Some(icons::NEWSPAPER_MINUS),
        "bell-circle-outline" => Some(icons::BELL_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-xbox' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_XBOX)
        }
        "clock-time-eight-outline" => Some(icons::CLOCK_TIME_EIGHT_OUTLINE),
        "water-plus-outline" => Some(icons::WATER_PLUS_OUTLINE),
        "truck-plus" => Some(icons::TRUCK_PLUS),
        "cookie-edit" => Some(icons::COOKIE_EDIT),
        "fridge-variant-off-outline" => Some(icons::FRIDGE_VARIANT_OFF_OUTLINE),
        "lungs" => Some(icons::LUNGS),
        "dresser-outline" => Some(icons::DRESSER_OUTLINE),
        "application-brackets-outline" => Some(icons::APPLICATION_BRACKETS_OUTLINE),
        "sync-off" => Some(icons::SYNC_OFF),
        "movie-play-outline" => Some(icons::MOVIE_PLAY_OUTLINE),
        "timer-plus" => Some(icons::TIMER_PLUS),
        "clock-time-three" => Some(icons::CLOCK_TIME_THREE),
        "merge" => Some(icons::MERGE),
        "golf-cart" => Some(icons::GOLF_CART),
        "bus" => Some(icons::BUS),
        _ => None,
    }
}
