// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_11(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "countertop" => Some(icons::COUNTERTOP),
        "layers-triple-outline" => Some(icons::LAYERS_TRIPLE_OUTLINE),
        "relation-one-to-zero-or-one" => Some(icons::RELATION_ONE_TO_ZERO_OR_ONE),
        "store-cog-outline" => Some(icons::STORE_COG_OUTLINE),
        "ocarina" => Some(icons::OCARINA),
        "robot-dead" => Some(icons::ROBOT_DEAD),
        "tally-mark-2" => Some(icons::TALLY_MARK_2),
        "application-variable-outline" => Some(icons::APPLICATION_VARIABLE_OUTLINE),
        "peanut" => Some(icons::PEANUT),
        "printer-pos-minus" => Some(icons::PRINTER_POS_MINUS),
        "car-outline" => Some(icons::CAR_OUTLINE),
        "camera-front-variant" => Some(icons::CAMERA_FRONT_VARIANT),
        "comment-text-multiple" => Some(icons::COMMENT_TEXT_MULTIPLE),
        "cookie-clock" => Some(icons::COOKIE_CLOCK),
        "baseball" => Some(icons::BASEBALL),
        #[allow(deprecated)]
        "microsoft-internet-explorer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-internet-explorer' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_INTERNET_EXPLORER)
        }
        "stocking" => Some(icons::STOCKING),
        "circle-multiple-outline" => Some(icons::CIRCLE_MULTIPLE_OUTLINE),
        "cloud-refresh-variant" => Some(icons::CLOUD_REFRESH_VARIANT),
        "fishbowl-outline" => Some(icons::FISHBOWL_OUTLINE),
        "archive-alert-outline" => Some(icons::ARCHIVE_ALERT_OUTLINE),
        "file-arrow-up-down" => Some(icons::FILE_ARROW_UP_DOWN),
        "calendar-heart" => Some(icons::CALENDAR_HEART),
        "sack-percent" => Some(icons::SACK_PERCENT),
        "alpha-q-circle" => Some(icons::ALPHA_Q_CIRCLE),
        "train-car-box-open" => Some(icons::TRAIN_CAR_BOX_OPEN),
        "filter-variant-minus" => Some(icons::FILTER_VARIANT_MINUS),
        "coffee-maker-check-outline" => Some(icons::COFFEE_MAKER_CHECK_OUTLINE),
        "account-cash" => Some(icons::ACCOUNT_CASH),
        "contactless-payment-circle" => Some(icons::CONTACTLESS_PAYMENT_CIRCLE),
        "file-document-alert" => Some(icons::FILE_DOCUMENT_ALERT),
        "email-open-heart-outline" => Some(icons::EMAIL_OPEN_HEART_OUTLINE),
        "calculator-variant" => Some(icons::CALCULATOR_VARIANT),
        "text-box-outline" => Some(icons::TEXT_BOX_OUTLINE),
        "alpha-a-circle" => Some(icons::ALPHA_A_CIRCLE),
        "home-flood" => Some(icons::HOME_FLOOD),
        "artboard" => Some(icons::ARTBOARD),
        "airport" => Some(icons::AIRPORT),
        "phone-outgoing" => Some(icons::PHONE_OUTGOING),
        "octagram-minus-outline" => Some(icons::OCTAGRAM_MINUS_OUTLINE),
        "tag-search-outline" => Some(icons::TAG_SEARCH_OUTLINE),
        "book-music-outline" => Some(icons::BOOK_MUSIC_OUTLINE),
        "skull-scan-outline" => Some(icons::SKULL_SCAN_OUTLINE),
        "gesture-swipe" => Some(icons::GESTURE_SWIPE),
        "gamepad-down" => Some(icons::GAMEPAD_DOWN),
        "wifi-cancel" => Some(icons::WIFI_CANCEL),
        "application-import" => Some(icons::APPLICATION_IMPORT),
        "fridge-variant-alert-outline" => Some(icons::FRIDGE_VARIANT_ALERT_OUTLINE),
        "car-tire-alert" => Some(icons::CAR_TIRE_ALERT),
        "car-key" => Some(icons::CAR_KEY),
        "menorah" => Some(icons::MENORAH),
        "puzzle-remove-outline" => Some(icons::PUZZLE_REMOVE_OUTLINE),
        "developer-board" => Some(icons::DEVELOPER_BOARD),
        "horizontal-rotate-counterclockwise" => Some(icons::HORIZONTAL_ROTATE_COUNTERCLOCKWISE),
        "robot-love-outline" => Some(icons::ROBOT_LOVE_OUTLINE),
        "content-copy" => Some(icons::CONTENT_COPY),
        "account-lock-outline" => Some(icons::ACCOUNT_LOCK_OUTLINE),
        "blood-bag" => Some(icons::BLOOD_BAG),
        "gesture-tap-hold" => Some(icons::GESTURE_TAP_HOLD),
        "email-arrow-right-outline" => Some(icons::EMAIL_ARROW_RIGHT_OUTLINE),
        "sim-off-outline" => Some(icons::SIM_OFF_OUTLINE),
        "bookmark-plus-outline" => Some(icons::BOOKMARK_PLUS_OUTLINE),
        "battery-off" => Some(icons::BATTERY_OFF),
        "align-horizontal-left" => Some(icons::ALIGN_HORIZONTAL_LEFT),
        "shield" => Some(icons::SHIELD),
        "emoticon-sick-outline" => Some(icons::EMOTICON_SICK_OUTLINE),
        "emoticon-lol" => Some(icons::EMOTICON_LOL),
        "knife" => Some(icons::KNIFE),
        "motorbike-electric" => Some(icons::MOTORBIKE_ELECTRIC),
        "crop-landscape" => Some(icons::CROP_LANDSCAPE),
        "keyboard-f3" => Some(icons::KEYBOARD_F3),
        "book-arrow-up" => Some(icons::BOOK_ARROW_UP),
        "human-female-dance" => Some(icons::HUMAN_FEMALE_DANCE),
        "knob" => Some(icons::KNOB),
        "mouse-bluetooth" => Some(icons::MOUSE_BLUETOOTH),
        "account-edit-outline" => Some(icons::ACCOUNT_EDIT_OUTLINE),
        "gamepad-circle-down" => Some(icons::GAMEPAD_CIRCLE_DOWN),
        "wall-sconce-flat" => Some(icons::WALL_SCONCE_FLAT),
        "audio-video-off" => Some(icons::AUDIO_VIDEO_OFF),
        "tag-arrow-down-outline" => Some(icons::TAG_ARROW_DOWN_OUTLINE),
        "forwardburger" => Some(icons::FORWARDBURGER),
        "roman-numeral-1" => Some(icons::ROMAN_NUMERAL_1),
        "doorbell" => Some(icons::DOORBELL),
        "archive-lock-open-outline" => Some(icons::ARCHIVE_LOCK_OPEN_OUTLINE),
        "image-text" => Some(icons::IMAGE_TEXT),
        "arrow-right-bold-circle-outline" => Some(icons::ARROW_RIGHT_BOLD_CIRCLE_OUTLINE),
        "folder-table-outline" => Some(icons::FOLDER_TABLE_OUTLINE),
        "application-brackets" => Some(icons::APPLICATION_BRACKETS),
        "folder-sync" => Some(icons::FOLDER_SYNC),
        "train-car-passenger-door" => Some(icons::TRAIN_CAR_PASSENGER_DOOR),
        "restore" => Some(icons::RESTORE),
        "projector-screen-off" => Some(icons::PROJECTOR_SCREEN_OFF),
        "alpha-h-circle" => Some(icons::ALPHA_H_CIRCLE),
        "image-filter-center-focus-strong-outline" => {
            Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG_OUTLINE)
        }
        "dock-window" => Some(icons::DOCK_WINDOW),
        "storefront-remove-outline" => Some(icons::STOREFRONT_REMOVE_OUTLINE),
        "shield-sword-outline" => Some(icons::SHIELD_SWORD_OUTLINE),
        "account-child-outline" => Some(icons::ACCOUNT_CHILD_OUTLINE),
        "thumb-up-outline" => Some(icons::THUMB_UP_OUTLINE),
        "cloud-cancel-outline" => Some(icons::CLOUD_CANCEL_OUTLINE),
        "kettle" => Some(icons::KETTLE),
        "train-car-hopper-full" => Some(icons::TRAIN_CAR_HOPPER_FULL),
        "ev-plug-ccs2" => Some(icons::EV_PLUG_CCS2),
        "air-purifier-off" => Some(icons::AIR_PURIFIER_OFF),
        "gesture-double-tap" => Some(icons::GESTURE_DOUBLE_TAP),
        "bookmark" => Some(icons::BOOKMARK),
        "sail-boat" => Some(icons::SAIL_BOAT),
        "cog-transfer" => Some(icons::COG_TRANSFER),
        "brightness-percent" => Some(icons::BRIGHTNESS_PERCENT),
        "store-edit-outline" => Some(icons::STORE_EDIT_OUTLINE),
        "mouse-off" => Some(icons::MOUSE_OFF),
        "dots-square" => Some(icons::DOTS_SQUARE),
        "calendar-range-outline" => Some(icons::CALENDAR_RANGE_OUTLINE),
        "microwave-off" => Some(icons::MICROWAVE_OFF),
        "vector-selection" => Some(icons::VECTOR_SELECTION),
        "currency-ngn" => Some(icons::CURRENCY_NGN),
        "upload-circle" => Some(icons::UPLOAD_CIRCLE),
        "lightning-bolt-circle" => Some(icons::LIGHTNING_BOLT_CIRCLE),
        "view-stream" => Some(icons::VIEW_STREAM),
        "source-commit-next-local" => Some(icons::SOURCE_COMMIT_NEXT_LOCAL),
        #[allow(deprecated)]
        "midi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'midi' is deprecated.").print(py);
            }
            Some(icons::MIDI)
        }
        "opacity" => Some(icons::OPACITY),
        "alarm-snooze" => Some(icons::ALARM_SNOOZE),
        "numeric-2-box-multiple" => Some(icons::NUMERIC_2_BOX_MULTIPLE),
        "arrow-down-thin-circle-outline" => Some(icons::ARROW_DOWN_THIN_CIRCLE_OUTLINE),
        "file-arrow-left-right-outline" => Some(icons::FILE_ARROW_LEFT_RIGHT_OUTLINE),
        #[allow(deprecated)]
        "youtube-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-gaming' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_GAMING)
        }
        "dice-d6-outline" => Some(icons::DICE_D6_OUTLINE),
        "speedometer-slow" => Some(icons::SPEEDOMETER_SLOW),
        "gesture-two-double-tap" => Some(icons::GESTURE_TWO_DOUBLE_TAP),
        "panorama-variant-outline" => Some(icons::PANORAMA_VARIANT_OUTLINE),
        "seat-recline-normal" => Some(icons::SEAT_RECLINE_NORMAL),
        "radioactive-circle-outline" => Some(icons::RADIOACTIVE_CIRCLE_OUTLINE),
        "human-greeting-variant" => Some(icons::HUMAN_GREETING_VARIANT),
        "cash-fast" => Some(icons::CASH_FAST),
        "blinds-vertical" => Some(icons::BLINDS_VERTICAL),
        "car-wireless" => Some(icons::CAR_WIRELESS),
        "history" => Some(icons::HISTORY),
        "panorama-wide-angle-outline" => Some(icons::PANORAMA_WIDE_ANGLE_OUTLINE),
        "train-car-tank" => Some(icons::TRAIN_CAR_TANK),
        "copyright" => Some(icons::COPYRIGHT),
        "package-up" => Some(icons::PACKAGE_UP),
        "timer-plus-outline" => Some(icons::TIMER_PLUS_OUTLINE),
        "information-slab-box-outline" => Some(icons::INFORMATION_SLAB_BOX_OUTLINE),
        "zodiac-sagittarius" => Some(icons::ZODIAC_SAGITTARIUS),
        "shield-sync-outline" => Some(icons::SHIELD_SYNC_OUTLINE),
        "tangram" => Some(icons::TANGRAM),
        "arrow-up-circle" => Some(icons::ARROW_UP_CIRCLE),
        "tray-minus" => Some(icons::TRAY_MINUS),
        "inbox-remove" => Some(icons::INBOX_REMOVE),
        #[allow(deprecated)]
        "apple-ios" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-ios' is deprecated.").print(py);
            }
            Some(icons::APPLE_IOS)
        }
        "relation-one-or-many-to-zero-or-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ZERO_OR_ONE),
        "select-place" => Some(icons::SELECT_PLACE),
        "death-star-variant" => Some(icons::DEATH_STAR_VARIANT),
        #[allow(deprecated)]
        "jira" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jira' is deprecated.").print(py);
            }
            Some(icons::JIRA)
        }
        "note-outline" => Some(icons::NOTE_OUTLINE),
        "alpha-w-circle" => Some(icons::ALPHA_W_CIRCLE),
        "toothbrush" => Some(icons::TOOTHBRUSH),
        "angle-right" => Some(icons::ANGLE_RIGHT),
        "water-remove" => Some(icons::WATER_REMOVE),
        "keyboard-space" => Some(icons::KEYBOARD_SPACE),
        "rhombus" => Some(icons::RHOMBUS),
        "cellphone-charging" => Some(icons::CELLPHONE_CHARGING),
        "head-flash" => Some(icons::HEAD_FLASH),
        "message-minus" => Some(icons::MESSAGE_MINUS),
        "format-header-1" => Some(icons::FORMAT_HEADER_1),
        "lock-open-outline" => Some(icons::LOCK_OPEN_OUTLINE),
        "filter-variant-plus" => Some(icons::FILTER_VARIANT_PLUS),
        "record-player" => Some(icons::RECORD_PLAYER),
        "credit-card-refund" => Some(icons::CREDIT_CARD_REFUND),
        "license" => Some(icons::LICENSE),
        "map-marker-remove" => Some(icons::MAP_MARKER_REMOVE),
        "reiterate" => Some(icons::REITERATE),
        "atom" => Some(icons::ATOM),
        "abacus" => Some(icons::ABACUS),
        "import" => Some(icons::IMPORT),
        "exclamation" => Some(icons::EXCLAMATION),
        "layers-off-outline" => Some(icons::LAYERS_OFF_OUTLINE),
        "library-shelves" => Some(icons::LIBRARY_SHELVES),
        "flask-off" => Some(icons::FLASK_OFF),
        "email-seal-outline" => Some(icons::EMAIL_SEAL_OUTLINE),
        "battery-alert-bluetooth" => Some(icons::BATTERY_ALERT_BLUETOOTH),
        "information-variant-box-outline" => Some(icons::INFORMATION_VARIANT_BOX_OUTLINE),
        "math-integral" => Some(icons::MATH_INTEGRAL),
        "relative-scale" => Some(icons::RELATIVE_SCALE),
        "star-three-points" => Some(icons::STAR_THREE_POINTS),
        #[allow(deprecated)]
        "qqchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qqchat' is deprecated.").print(py);
            }
            Some(icons::QQCHAT)
        }
        "calendar-filter" => Some(icons::CALENDAR_FILTER),
        "head-question" => Some(icons::HEAD_QUESTION),
        "folder-edit-outline" => Some(icons::FOLDER_EDIT_OUTLINE),
        "triangle-down-outline" => Some(icons::TRIANGLE_DOWN_OUTLINE),
        "filter" => Some(icons::FILTER),
        "keyboard-off-outline" => Some(icons::KEYBOARD_OFF_OUTLINE),
        "remote-off" => Some(icons::REMOTE_OFF),
        "kabaddi" => Some(icons::KABADDI),
        "image-filter-center-focus" => Some(icons::IMAGE_FILTER_CENTER_FOCUS),
        "clock-time-one-outline" => Some(icons::CLOCK_TIME_ONE_OUTLINE),
        "desktop-tower-monitor" => Some(icons::DESKTOP_TOWER_MONITOR),
        "radius-outline" => Some(icons::RADIUS_OUTLINE),
        "source-commit-end-local" => Some(icons::SOURCE_COMMIT_END_LOCAL),
        _ => None,
    }
}
