// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_10(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "spa" => Some(icons::SPA),
        "cog-outline" => Some(icons::COG_OUTLINE),
        "flask-minus-outline" => Some(icons::FLASK_MINUS_OUTLINE),
        "cloud-remove" => Some(icons::CLOUD_REMOVE),
        #[allow(deprecated)]
        "nintendo-wiiu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wiiu' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WIIU)
        }
        "medal-outline" => Some(icons::MEDAL_OUTLINE),
        "camera-off-outline" => Some(icons::CAMERA_OFF_OUTLINE),
        "alarm-multiple" => Some(icons::ALARM_MULTIPLE),
        "timer-sync" => Some(icons::TIMER_SYNC),
        "water-remove" => Some(icons::WATER_REMOVE),
        "table-cog" => Some(icons::TABLE_COG),
        "lightbulb-alert-outline" => Some(icons::LIGHTBULB_ALERT_OUTLINE),
        "gesture-swipe-up" => Some(icons::GESTURE_SWIPE_UP),
        "folder-arrow-left-right-outline" => Some(icons::FOLDER_ARROW_LEFT_RIGHT_OUTLINE),
        "shopping-search-outline" => Some(icons::SHOPPING_SEARCH_OUTLINE),
        "zodiac-leo" => Some(icons::ZODIAC_LEO),
        "video-plus-outline" => Some(icons::VIDEO_PLUS_OUTLINE),
        "download-box" => Some(icons::DOWNLOAD_BOX),
        "map-marker-minus-outline" => Some(icons::MAP_MARKER_MINUS_OUTLINE),
        "kite-outline" => Some(icons::KITE_OUTLINE),
        "network-strength-1" => Some(icons::NETWORK_STRENGTH_1),
        "archive-outline" => Some(icons::ARCHIVE_OUTLINE),
        "dishwasher-alert" => Some(icons::DISHWASHER_ALERT),
        "comment-text-outline" => Some(icons::COMMENT_TEXT_OUTLINE),
        "calendar-star" => Some(icons::CALENDAR_STAR),
        "crown-circle-outline" => Some(icons::CROWN_CIRCLE_OUTLINE),
        "information-outline" => Some(icons::INFORMATION_OUTLINE),
        "shaker" => Some(icons::SHAKER),
        "screwdriver" => Some(icons::SCREWDRIVER),
        "credit-card-multiple-outline" => Some(icons::CREDIT_CARD_MULTIPLE_OUTLINE),
        "volcano-outline" => Some(icons::VOLCANO_OUTLINE),
        "credit-card-lock-outline" => Some(icons::CREDIT_CARD_LOCK_OUTLINE),
        "hail" => Some(icons::HAIL),
        "gondola" => Some(icons::GONDOLA),
        #[allow(deprecated)]
        "zigbee" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'zigbee' is deprecated.").print(py);
            }
            Some(icons::ZIGBEE)
        }
        "thumb-up" => Some(icons::THUMB_UP),
        "wifi-strength-1-lock" => Some(icons::WIFI_STRENGTH_1_LOCK),
        "dolly" => Some(icons::DOLLY),
        "mailbox-outline" => Some(icons::MAILBOX_OUTLINE),
        "butterfly-outline" => Some(icons::BUTTERFLY_OUTLINE),
        "pencil-lock-outline" => Some(icons::PENCIL_LOCK_OUTLINE),
        "clipboard-outline" => Some(icons::CLIPBOARD_OUTLINE),
        "alpha-v-circle-outline" => Some(icons::ALPHA_V_CIRCLE_OUTLINE),
        "alpha-y-box-outline" => Some(icons::ALPHA_Y_BOX_OUTLINE),
        "rugby" => Some(icons::RUGBY),
        "watch-import-variant" => Some(icons::WATCH_IMPORT_VARIANT),
        "text-box-edit-outline" => Some(icons::TEXT_BOX_EDIT_OUTLINE),
        "bus-marker" => Some(icons::BUS_MARKER),
        "snowboard" => Some(icons::SNOWBOARD),
        "card-minus" => Some(icons::CARD_MINUS),
        "undo" => Some(icons::UNDO),
        "image-edit-outline" => Some(icons::IMAGE_EDIT_OUTLINE),
        "hand-back-right-off-outline" => Some(icons::HAND_BACK_RIGHT_OFF_OUTLINE),
        "bell-minus-outline" => Some(icons::BELL_MINUS_OUTLINE),
        "alphabetical-off" => Some(icons::ALPHABETICAL_OFF),
        "waveform" => Some(icons::WAVEFORM),
        "rollerblade-off" => Some(icons::ROLLERBLADE_OFF),
        "vector-intersection" => Some(icons::VECTOR_INTERSECTION),
        "ceiling-light-multiple" => Some(icons::CEILING_LIGHT_MULTIPLE),
        "order-alphabetical-descending" => Some(icons::ORDER_ALPHABETICAL_DESCENDING),
        "tumble-dryer" => Some(icons::TUMBLE_DRYER),
        "format-vertical-align-bottom" => Some(icons::FORMAT_VERTICAL_ALIGN_BOTTOM),
        "slope-uphill" => Some(icons::SLOPE_UPHILL),
        "format-pilcrow-arrow-right" => Some(icons::FORMAT_PILCROW_ARROW_RIGHT),
        "numeric-9-plus-circle-outline" => Some(icons::NUMERIC_9_PLUS_CIRCLE_OUTLINE),
        "image-filter-tilt-shift" => Some(icons::IMAGE_FILTER_TILT_SHIFT),
        "battery-charging-90" => Some(icons::BATTERY_CHARGING_90),
        "pencil-off" => Some(icons::PENCIL_OFF),
        "ninja" => Some(icons::NINJA),
        "clipboard-arrow-right" => Some(icons::CLIPBOARD_ARROW_RIGHT),
        "clock-time-seven-outline" => Some(icons::CLOCK_TIME_SEVEN_OUTLINE),
        "leaf-circle-outline" => Some(icons::LEAF_CIRCLE_OUTLINE),
        "receipt-text-outline" => Some(icons::RECEIPT_TEXT_OUTLINE),
        "signal-distance-variant" => Some(icons::SIGNAL_DISTANCE_VARIANT),
        "circle-expand" => Some(icons::CIRCLE_EXPAND),
        "share-off" => Some(icons::SHARE_OFF),
        "pan-right" => Some(icons::PAN_RIGHT),
        "format-letter-case-lower" => Some(icons::FORMAT_LETTER_CASE_LOWER),
        "map-search" => Some(icons::MAP_SEARCH),
        "skip-next-circle-outline" => Some(icons::SKIP_NEXT_CIRCLE_OUTLINE),
        "wardrobe-outline" => Some(icons::WARDROBE_OUTLINE),
        "scoreboard-outline" => Some(icons::SCOREBOARD_OUTLINE),
        "speaker-multiple" => Some(icons::SPEAKER_MULTIPLE),
        "gamma" => Some(icons::GAMMA),
        "diving" => Some(icons::DIVING),
        "book-lock-open-outline" => Some(icons::BOOK_LOCK_OPEN_OUTLINE),
        "account-key" => Some(icons::ACCOUNT_KEY),
        #[allow(deprecated)]
        "black-mesa" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'black-mesa' is deprecated.").print(py);
            }
            Some(icons::BLACK_MESA)
        }
        "forest-outline" => Some(icons::FOREST_OUTLINE),
        "file-hidden" => Some(icons::FILE_HIDDEN),
        "face-man" => Some(icons::FACE_MAN),
        "cart-arrow-down" => Some(icons::CART_ARROW_DOWN),
        "label" => Some(icons::LABEL),
        "cloud-clock-outline" => Some(icons::CLOUD_CLOCK_OUTLINE),
        "clipboard-play" => Some(icons::CLIPBOARD_PLAY),
        "tea-outline" => Some(icons::TEA_OUTLINE),
        "sticker-alert-outline" => Some(icons::STICKER_ALERT_OUTLINE),
        "pen-lock" => Some(icons::PEN_LOCK),
        #[allow(deprecated)]
        "google-cardboard" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cardboard' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CARDBOARD)
        }
        "calendar-text-outline" => Some(icons::CALENDAR_TEXT_OUTLINE),
        "numeric-10-box" => Some(icons::NUMERIC_10_BOX),
        "eye-off" => Some(icons::EYE_OFF),
        "id-card" => Some(icons::ID_CARD),
        "dots-circle" => Some(icons::DOTS_CIRCLE),
        "flask-empty" => Some(icons::FLASK_EMPTY),
        "clock-time-three-outline" => Some(icons::CLOCK_TIME_THREE_OUTLINE),
        "mosque" => Some(icons::MOSQUE),
        "clipboard" => Some(icons::CLIPBOARD),
        "gate-not" => Some(icons::GATE_NOT),
        "toggle-switch-off" => Some(icons::TOGGLE_SWITCH_OFF),
        "mixed-martial-arts" => Some(icons::MIXED_MARTIAL_ARTS),
        "racing-helmet" => Some(icons::RACING_HELMET),
        "filter-plus-outline" => Some(icons::FILTER_PLUS_OUTLINE),
        "source-branch-check" => Some(icons::SOURCE_BRANCH_CHECK),
        "transit-connection" => Some(icons::TRANSIT_CONNECTION),
        "arrow-bottom-right-bold-box" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX),
        "arrow-expand-right" => Some(icons::ARROW_EXPAND_RIGHT),
        "office-building-marker" => Some(icons::OFFICE_BUILDING_MARKER),
        "popcorn" => Some(icons::POPCORN),
        "decimal-increase" => Some(icons::DECIMAL_INCREASE),
        "controller-classic-outline" => Some(icons::CONTROLLER_CLASSIC_OUTLINE),
        "pan-top-right" => Some(icons::PAN_TOP_RIGHT),
        #[allow(deprecated)]
        "folder-google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'folder-google-drive' is deprecated.")
                    .print(py);
            }
            Some(icons::FOLDER_GOOGLE_DRIVE)
        }
        "folder-zip" => Some(icons::FOLDER_ZIP),
        "forum-plus" => Some(icons::FORUM_PLUS),
        "align-horizontal-distribute" => Some(icons::ALIGN_HORIZONTAL_DISTRIBUTE),
        "clipboard-play-outline" => Some(icons::CLIPBOARD_PLAY_OUTLINE),
        "chevron-down-circle-outline" => Some(icons::CHEVRON_DOWN_CIRCLE_OUTLINE),
        "close" => Some(icons::CLOSE),
        "lightbulb-on-10" => Some(icons::LIGHTBULB_ON_10),
        "grass" => Some(icons::GRASS),
        "attachment-check" => Some(icons::ATTACHMENT_CHECK),
        "latitude" => Some(icons::LATITUDE),
        "cloud-refresh-variant-outline" => Some(icons::CLOUD_REFRESH_VARIANT_OUTLINE),
        "minus-box" => Some(icons::MINUS_BOX),
        "file-delimited-outline" => Some(icons::FILE_DELIMITED_OUTLINE),
        "file-minus-outline" => Some(icons::FILE_MINUS_OUTLINE),
        "hand-back-left" => Some(icons::HAND_BACK_LEFT),
        "file-document-outline" => Some(icons::FILE_DOCUMENT_OUTLINE),
        "identifier" => Some(icons::IDENTIFIER),
        "dock-top" => Some(icons::DOCK_TOP),
        "variable-box" => Some(icons::VARIABLE_BOX),
        "file-document-multiple-outline" => Some(icons::FILE_DOCUMENT_MULTIPLE_OUTLINE),
        "vector-point-edit" => Some(icons::VECTOR_POINT_EDIT),
        "clover" => Some(icons::CLOVER),
        "ev-plug-chademo" => Some(icons::EV_PLUG_CHADEMO),
        "human-greeting-proximity" => Some(icons::HUMAN_GREETING_PROXIMITY),
        "phone-settings" => Some(icons::PHONE_SETTINGS),
        "bacteria" => Some(icons::BACTERIA),
        "email-fast" => Some(icons::EMAIL_FAST),
        "file-table-box" => Some(icons::FILE_TABLE_BOX),
        "email-open" => Some(icons::EMAIL_OPEN),
        "angle-right" => Some(icons::ANGLE_RIGHT),
        "trophy-variant" => Some(icons::TROPHY_VARIANT),
        "fan-chevron-down" => Some(icons::FAN_CHEVRON_DOWN),
        "message-off-outline" => Some(icons::MESSAGE_OFF_OUTLINE),
        "movie-open-check-outline" => Some(icons::MOVIE_OPEN_CHECK_OUTLINE),
        "hand-saw" => Some(icons::HAND_SAW),
        "shopping-outline" => Some(icons::SHOPPING_OUTLINE),
        "sort-calendar-descending" => Some(icons::SORT_CALENDAR_DESCENDING),
        "crosshairs" => Some(icons::CROSSHAIRS),
        "arrow-up-box" => Some(icons::ARROW_UP_BOX),
        "email-open-multiple-outline" => Some(icons::EMAIL_OPEN_MULTIPLE_OUTLINE),
        "emoticon-cool-outline" => Some(icons::EMOTICON_COOL_OUTLINE),
        "printer-pos-off-outline" => Some(icons::PRINTER_POS_OFF_OUTLINE),
        "basket-plus" => Some(icons::BASKET_PLUS),
        "chevron-up" => Some(icons::CHEVRON_UP),
        "arrow-horizontal-lock" => Some(icons::ARROW_HORIZONTAL_LOCK),
        "water-pump" => Some(icons::WATER_PUMP),
        "delete" => Some(icons::DELETE),
        "file-phone" => Some(icons::FILE_PHONE),
        "movie-open-settings-outline" => Some(icons::MOVIE_OPEN_SETTINGS_OUTLINE),
        "margin" => Some(icons::MARGIN),
        "script-text-key" => Some(icons::SCRIPT_TEXT_KEY),
        "application" => Some(icons::APPLICATION),
        "book-open" => Some(icons::BOOK_OPEN),
        "image-area-close" => Some(icons::IMAGE_AREA_CLOSE),
        "hvac" => Some(icons::HVAC),
        "bag-personal-plus-outline" => Some(icons::BAG_PERSONAL_PLUS_OUTLINE),
        "relation-one-or-many-to-only-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONLY_ONE),
        "credit-card-minus-outline" => Some(icons::CREDIT_CARD_MINUS_OUTLINE),
        "lifebuoy" => Some(icons::LIFEBUOY),
        "briefcase-remove" => Some(icons::BRIEFCASE_REMOVE),
        "file-chart-check" => Some(icons::FILE_CHART_CHECK),
        "flask-empty-plus" => Some(icons::FLASK_EMPTY_PLUS),
        "message-badge" => Some(icons::MESSAGE_BADGE),
        "radius" => Some(icons::RADIUS),
        "water-well" => Some(icons::WATER_WELL),
        "parking" => Some(icons::PARKING),
        "folder-heart-outline" => Some(icons::FOLDER_HEART_OUTLINE),
        "cloud-lock" => Some(icons::CLOUD_LOCK),
        "guitar-electric" => Some(icons::GUITAR_ELECTRIC),
        "sine-wave" => Some(icons::SINE_WAVE),
        "file-presentation-box" => Some(icons::FILE_PRESENTATION_BOX),
        "checkerboard" => Some(icons::CHECKERBOARD),
        "surround-sound-7-1" => Some(icons::SURROUND_SOUND_7_1),
        "receipt-text-minus-outline" => Some(icons::RECEIPT_TEXT_MINUS_OUTLINE),
        "printer-pos-alert-outline" => Some(icons::PRINTER_POS_ALERT_OUTLINE),
        "cards-playing-heart-multiple" => Some(icons::CARDS_PLAYING_HEART_MULTIPLE),
        "numeric-off" => Some(icons::NUMERIC_OFF),
        _ => None,
    }
}
