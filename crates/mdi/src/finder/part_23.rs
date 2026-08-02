// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_23(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "thermostat-auto" => Some(icons::THERMOSTAT_AUTO),
        "folder-account-outline" => Some(icons::FOLDER_ACCOUNT_OUTLINE),
        "seat-recline-normal" => Some(icons::SEAT_RECLINE_NORMAL),
        "camera-lock-open-outline" => Some(icons::CAMERA_LOCK_OPEN_OUTLINE),
        "image-size-select-small" => Some(icons::IMAGE_SIZE_SELECT_SMALL),
        "quadcopter" => Some(icons::QUADCOPTER),
        "book-cross" => Some(icons::BOOK_CROSS),
        "cellphone-nfc" => Some(icons::CELLPHONE_NFC),
        "ethernet-cable-off" => Some(icons::ETHERNET_CABLE_OFF),
        "circle-slice-4" => Some(icons::CIRCLE_SLICE_4),
        "cash-off" => Some(icons::CASH_OFF),
        "checkbox-marked-circle-minus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_MINUS_OUTLINE),
        "head-lightbulb" => Some(icons::HEAD_LIGHTBULB),
        "phone-plus" => Some(icons::PHONE_PLUS),
        "tooltip-image-outline" => Some(icons::TOOLTIP_IMAGE_OUTLINE),
        "folder-network-outline" => Some(icons::FOLDER_NETWORK_OUTLINE),
        "view-quilt-outline" => Some(icons::VIEW_QUILT_OUTLINE),
        "beaker-alert-outline" => Some(icons::BEAKER_ALERT_OUTLINE),
        "toggle-switch-variant-off" => Some(icons::TOGGLE_SWITCH_VARIANT_OFF),
        "headphones-off" => Some(icons::HEADPHONES_OFF),
        "numeric-5" => Some(icons::NUMERIC_5),
        "map-marker-outline" => Some(icons::MAP_MARKER_OUTLINE),
        "sticker-alert-outline" => Some(icons::STICKER_ALERT_OUTLINE),
        #[allow(deprecated)]
        "microsoft-sharepoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-sharepoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_SHAREPOINT)
        }
        #[allow(deprecated)]
        "cordova" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cordova' is deprecated.").print(py);
            }
            Some(icons::CORDOVA)
        }
        "book-off-outline" => Some(icons::BOOK_OFF_OUTLINE),
        "apps-box" => Some(icons::APPS_BOX),
        "card-search-outline" => Some(icons::CARD_SEARCH_OUTLINE),
        "heart-settings" => Some(icons::HEART_SETTINGS),
        "chart-donut" => Some(icons::CHART_DONUT),
        "engine-off" => Some(icons::ENGINE_OFF),
        "cloud-key-outline" => Some(icons::CLOUD_KEY_OUTLINE),
        "timer-play" => Some(icons::TIMER_PLAY),
        "label-off" => Some(icons::LABEL_OFF),
        "playlist-minus" => Some(icons::PLAYLIST_MINUS),
        "calendar-lock-outline" => Some(icons::CALENDAR_LOCK_OUTLINE),
        "clock-edit-outline" => Some(icons::CLOCK_EDIT_OUTLINE),
        "star-off-outline" => Some(icons::STAR_OFF_OUTLINE),
        "surround-sound" => Some(icons::SURROUND_SOUND),
        "mushroom-off-outline" => Some(icons::MUSHROOM_OFF_OUTLINE),
        "message-badge" => Some(icons::MESSAGE_BADGE),
        "printer-pos-off" => Some(icons::PRINTER_POS_OFF),
        "alpha-z-box-outline" => Some(icons::ALPHA_Z_BOX_OUTLINE),
        "surround-sound-5-1" => Some(icons::SURROUND_SOUND_5_1),
        "account-badge" => Some(icons::ACCOUNT_BADGE),
        "currency-usd" => Some(icons::CURRENCY_USD),
        "movie-minus-outline" => Some(icons::MOVIE_MINUS_OUTLINE),
        #[allow(deprecated)]
        "jabber" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jabber' is deprecated.").print(py);
            }
            Some(icons::JABBER)
        }
        "folder-music-outline" => Some(icons::FOLDER_MUSIC_OUTLINE),
        "content-save-edit" => Some(icons::CONTENT_SAVE_EDIT),
        "atom" => Some(icons::ATOM),
        "auto-upload" => Some(icons::AUTO_UPLOAD),
        "script-text-outline" => Some(icons::SCRIPT_TEXT_OUTLINE),
        "airplane-landing" => Some(icons::AIRPLANE_LANDING),
        "account-arrow-up-outline" => Some(icons::ACCOUNT_ARROW_UP_OUTLINE),
        "fire-circle" => Some(icons::FIRE_CIRCLE),
        "contactless-payment-circle" => Some(icons::CONTACTLESS_PAYMENT_CIRCLE),
        "sticker-outline" => Some(icons::STICKER_OUTLINE),
        "car-defrost-front" => Some(icons::CAR_DEFROST_FRONT),
        "tag-remove" => Some(icons::TAG_REMOVE),
        "file-upload-outline" => Some(icons::FILE_UPLOAD_OUTLINE),
        "fan-off" => Some(icons::FAN_OFF),
        "format-vertical-align-center" => Some(icons::FORMAT_VERTICAL_ALIGN_CENTER),
        "calendar-remove" => Some(icons::CALENDAR_REMOVE),
        "bike-pedal" => Some(icons::BIKE_PEDAL),
        "stool-outline" => Some(icons::STOOL_OUTLINE),
        "bookmark-minus-outline" => Some(icons::BOOKMARK_MINUS_OUTLINE),
        "distribute-vertical-top" => Some(icons::DISTRIBUTE_VERTICAL_TOP),
        "shield-sync-outline" => Some(icons::SHIELD_SYNC_OUTLINE),
        "battery-check-outline" => Some(icons::BATTERY_CHECK_OUTLINE),
        "rotate-3d" => Some(icons::ROTATE_3D),
        "clipboard-text-search-outline" => Some(icons::CLIPBOARD_TEXT_SEARCH_OUTLINE),
        "fingerprint" => Some(icons::FINGERPRINT),
        #[allow(deprecated)]
        "babel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'babel' is deprecated.").print(py);
            }
            Some(icons::BABEL)
        }
        "flash-off" => Some(icons::FLASH_OFF),
        "tag-multiple-outline" => Some(icons::TAG_MULTIPLE_OUTLINE),
        "headphones" => Some(icons::HEADPHONES),
        "desk-lamp" => Some(icons::DESK_LAMP),
        "auto-mode" => Some(icons::AUTO_MODE),
        "weather-moonset" => Some(icons::WEATHER_MOONSET),
        "debug-step-into" => Some(icons::DEBUG_STEP_INTO),
        "decagram" => Some(icons::DECAGRAM),
        "cloud-braces" => Some(icons::CLOUD_BRACES),
        #[allow(deprecated)]
        "humble-bundle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'humble-bundle' is deprecated.").print(py);
            }
            Some(icons::HUMBLE_BUNDLE)
        }
        "math-cos" => Some(icons::MATH_COS),
        "battery-90-bluetooth" => Some(icons::BATTERY_90_BLUETOOTH),
        "shopping-search-outline" => Some(icons::SHOPPING_SEARCH_OUTLINE),
        "phone-rotate-portrait" => Some(icons::PHONE_ROTATE_PORTRAIT),
        "aspect-ratio" => Some(icons::ASPECT_RATIO),
        "filter" => Some(icons::FILTER),
        "numeric-0-box-multiple-outline" => Some(icons::NUMERIC_0_BOX_MULTIPLE_OUTLINE),
        "card-remove-outline" => Some(icons::CARD_REMOVE_OUTLINE),
        "robot-confused" => Some(icons::ROBOT_CONFUSED),
        "file-powerpoint-box-outline" => Some(icons::FILE_POWERPOINT_BOX_OUTLINE),
        "database-edit-outline" => Some(icons::DATABASE_EDIT_OUTLINE),
        "sort-alphabetical-descending-variant" => Some(icons::SORT_ALPHABETICAL_DESCENDING_VARIANT),
        "phone-missed-outline" => Some(icons::PHONE_MISSED_OUTLINE),
        "magnify-remove-outline" => Some(icons::MAGNIFY_REMOVE_OUTLINE),
        "file-compare" => Some(icons::FILE_COMPARE),
        "camera-retake-outline" => Some(icons::CAMERA_RETAKE_OUTLINE),
        "shape-square-rounded-plus" => Some(icons::SHAPE_SQUARE_ROUNDED_PLUS),
        "bird" => Some(icons::BIRD),
        "account-box" => Some(icons::ACCOUNT_BOX),
        "mini-sd" => Some(icons::MINI_SD),
        "flower-pollen-outline" => Some(icons::FLOWER_POLLEN_OUTLINE),
        "sort-numeric-descending" => Some(icons::SORT_NUMERIC_DESCENDING),
        "tooltip-plus-outline" => Some(icons::TOOLTIP_PLUS_OUTLINE),
        "smart-card-off" => Some(icons::SMART_CARD_OFF),
        "passport-check" => Some(icons::PASSPORT_CHECK),
        "arrow-left-circle" => Some(icons::ARROW_LEFT_CIRCLE),
        "social-distance-2-meters" => Some(icons::SOCIAL_DISTANCE_2_METERS),
        "invoice-list-outline" => Some(icons::INVOICE_LIST_OUTLINE),
        "movie-filter-outline" => Some(icons::MOVIE_FILTER_OUTLINE),
        "movie-open-minus-outline" => Some(icons::MOVIE_OPEN_MINUS_OUTLINE),
        "comment-flash" => Some(icons::COMMENT_FLASH),
        "chili-off" => Some(icons::CHILI_OFF),
        "earth" => Some(icons::EARTH),
        "microphone-plus" => Some(icons::MICROPHONE_PLUS),
        "clipboard-arrow-down-outline" => Some(icons::CLIPBOARD_ARROW_DOWN_OUTLINE),
        "arrow-top-right-bold-outline" => Some(icons::ARROW_TOP_RIGHT_BOLD_OUTLINE),
        "file-export-outline" => Some(icons::FILE_EXPORT_OUTLINE),
        #[allow(deprecated)]
        "apple-finder" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-finder' is deprecated.").print(py);
            }
            Some(icons::APPLE_FINDER)
        }
        "battery-charging-low" => Some(icons::BATTERY_CHARGING_LOW),
        "clock-time-six" => Some(icons::CLOCK_TIME_SIX),
        "invoice-plus" => Some(icons::INVOICE_PLUS),
        "cloud-refresh" => Some(icons::CLOUD_REFRESH),
        "video-4k-box" => Some(icons::VIDEO_4K_BOX),
        "battery-charging-high" => Some(icons::BATTERY_CHARGING_HIGH),
        "account-multiple-plus-outline" => Some(icons::ACCOUNT_MULTIPLE_PLUS_OUTLINE),
        "airplane-search" => Some(icons::AIRPLANE_SEARCH),
        "tag-plus" => Some(icons::TAG_PLUS),
        "eject-circle" => Some(icons::EJECT_CIRCLE),
        "egg-off-outline" => Some(icons::EGG_OFF_OUTLINE),
        "null" => Some(icons::NULL),
        "arrow-left-bottom" => Some(icons::ARROW_LEFT_BOTTOM),
        "arrow-bottom-right-bold-box-outline" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX_OUTLINE),
        "clock-star-four-points" => Some(icons::CLOCK_STAR_FOUR_POINTS),
        "sail-boat-sink" => Some(icons::SAIL_BOAT_SINK),
        "bee-flower" => Some(icons::BEE_FLOWER),
        "cash-lock-open" => Some(icons::CASH_LOCK_OPEN),
        "earbuds-off" => Some(icons::EARBUDS_OFF),
        #[allow(deprecated)]
        "google-circles-communities" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'google-circles-communities' is deprecated.",
                )
                .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_COMMUNITIES)
        }
        "text-box-check" => Some(icons::TEXT_BOX_CHECK),
        "reiterate" => Some(icons::REITERATE),
        "email-seal-outline" => Some(icons::EMAIL_SEAL_OUTLINE),
        "battery-lock" => Some(icons::BATTERY_LOCK),
        "hammer" => Some(icons::HAMMER),
        "checkbox-marked-circle" => Some(icons::CHECKBOX_MARKED_CIRCLE),
        "arrow-right-box" => Some(icons::ARROW_RIGHT_BOX),
        "movie-open-off" => Some(icons::MOVIE_OPEN_OFF),
        "sticker-check" => Some(icons::STICKER_CHECK),
        "ip-network-outline" => Some(icons::IP_NETWORK_OUTLINE),
        "image-remove-outline" => Some(icons::IMAGE_REMOVE_OUTLINE),
        "trophy-award" => Some(icons::TROPHY_AWARD),
        "application-variable" => Some(icons::APPLICATION_VARIABLE),
        "file-powerpoint-box" => Some(icons::FILE_POWERPOINT_BOX),
        "heart-multiple" => Some(icons::HEART_MULTIPLE),
        "head-sync-outline" => Some(icons::HEAD_SYNC_OUTLINE),
        "align-vertical-distribute" => Some(icons::ALIGN_VERTICAL_DISTRIBUTE),
        "fleur-de-lis" => Some(icons::FLEUR_DE_LIS),
        "clipboard-text-play" => Some(icons::CLIPBOARD_TEXT_PLAY),
        "flask-minus-outline" => Some(icons::FLASK_MINUS_OUTLINE),
        "white-balance-sunny" => Some(icons::WHITE_BALANCE_SUNNY),
        "folder-open" => Some(icons::FOLDER_OPEN),
        "truck-flatbed" => Some(icons::TRUCK_FLATBED),
        "map-legend" => Some(icons::MAP_LEGEND),
        "image-search-outline" => Some(icons::IMAGE_SEARCH_OUTLINE),
        "khanda" => Some(icons::KHANDA),
        "bug-play-outline" => Some(icons::BUG_PLAY_OUTLINE),
        "bus-articulated-front" => Some(icons::BUS_ARTICULATED_FRONT),
        "file-code-outline" => Some(icons::FILE_CODE_OUTLINE),
        "battery-arrow-up" => Some(icons::BATTERY_ARROW_UP),
        "thought-bubble" => Some(icons::THOUGHT_BUBBLE),
        "hand-extended" => Some(icons::HAND_EXTENDED),
        "view-column-outline" => Some(icons::VIEW_COLUMN_OUTLINE),
        "projector-screen-variant-off-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF_OUTLINE),
        "audio-input-xlr" => Some(icons::AUDIO_INPUT_XLR),
        "alpha-h-box" => Some(icons::ALPHA_H_BOX),
        "propane-tank" => Some(icons::PROPANE_TANK),
        "movie" => Some(icons::MOVIE),
        "vector-square" => Some(icons::VECTOR_SQUARE),
        "unicorn-variant" => Some(icons::UNICORN_VARIANT),
        "select-remove" => Some(icons::SELECT_REMOVE),
        "vector-square-close" => Some(icons::VECTOR_SQUARE_CLOSE),
        "border-right-variant" => Some(icons::BORDER_RIGHT_VARIANT),
        "chandelier" => Some(icons::CHANDELIER),
        "folder-multiple" => Some(icons::FOLDER_MULTIPLE),
        "face-woman" => Some(icons::FACE_WOMAN),
        "music-box-multiple" => Some(icons::MUSIC_BOX_MULTIPLE),
        "movie-edit" => Some(icons::MOVIE_EDIT),
        "stocking" => Some(icons::STOCKING),
        "radar" => Some(icons::RADAR),
        "file-certificate-outline" => Some(icons::FILE_CERTIFICATE_OUTLINE),
        "numeric-7" => Some(icons::NUMERIC_7),
        "sync-circle" => Some(icons::SYNC_CIRCLE),
        "alarm-note-off" => Some(icons::ALARM_NOTE_OFF),
        "camera-off" => Some(icons::CAMERA_OFF),
        "dice-multiple-outline" => Some(icons::DICE_MULTIPLE_OUTLINE),
        "controller-off" => Some(icons::CONTROLLER_OFF),
        "file-marker" => Some(icons::FILE_MARKER),
        _ => None,
    }
}
