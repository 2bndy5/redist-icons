// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_23(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "alpha-s-box" => Some(icons::ALPHA_S_BOX),
        "blinds-vertical" => Some(icons::BLINDS_VERTICAL),
        "grain" => Some(icons::GRAIN),
        "video-wireless-outline" => Some(icons::VIDEO_WIRELESS_OUTLINE),
        #[allow(deprecated)]
        "lumx" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'lumx' is deprecated.").print(py);
            }
            Some(icons::LUMX)
        }
        "cat" => Some(icons::CAT),
        "rolodex" => Some(icons::ROLODEX),
        "vector-point-plus" => Some(icons::VECTOR_POINT_PLUS),
        "gamepad-round-left" => Some(icons::GAMEPAD_ROUND_LEFT),
        #[allow(deprecated)]
        "litecoin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'litecoin' is deprecated.").print(py);
            }
            Some(icons::LITECOIN)
        }
        #[allow(deprecated)]
        "webpack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webpack' is deprecated.").print(py);
            }
            Some(icons::WEBPACK)
        }
        "mower-bag" => Some(icons::MOWER_BAG),
        "download-network-outline" => Some(icons::DOWNLOAD_NETWORK_OUTLINE),
        "eye-settings-outline" => Some(icons::EYE_SETTINGS_OUTLINE),
        "file" => Some(icons::FILE),
        "briefcase-clock" => Some(icons::BRIEFCASE_CLOCK),
        #[allow(deprecated)]
        "google-nearby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-nearby' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_NEARBY)
        }
        "protocol" => Some(icons::PROTOCOL),
        "comment-search" => Some(icons::COMMENT_SEARCH),
        "tag-edit" => Some(icons::TAG_EDIT),
        "octagram-outline" => Some(icons::OCTAGRAM_OUTLINE),
        "hexadecimal" => Some(icons::HEXADECIMAL),
        "all-inclusive-box" => Some(icons::ALL_INCLUSIVE_BOX),
        "timeline-check" => Some(icons::TIMELINE_CHECK),
        "bank-transfer" => Some(icons::BANK_TRANSFER),
        "thermometer-auto" => Some(icons::THERMOMETER_AUTO),
        "fit-to-page" => Some(icons::FIT_TO_PAGE),
        "layers-triple-outline" => Some(icons::LAYERS_TRIPLE_OUTLINE),
        "wizard-hat" => Some(icons::WIZARD_HAT),
        "lightbulb-cfl" => Some(icons::LIGHTBULB_CFL),
        "infinity" => Some(icons::INFINITY),
        "scent-off" => Some(icons::SCENT_OFF),
        "invoice-text" => Some(icons::INVOICE_TEXT),
        "arrow-left-bold-box" => Some(icons::ARROW_LEFT_BOLD_BOX),
        "table-edit" => Some(icons::TABLE_EDIT),
        "home-floor-1" => Some(icons::HOME_FLOOR_1),
        "hexagon-slice-2" => Some(icons::HEXAGON_SLICE_2),
        "projector-screen" => Some(icons::PROJECTOR_SCREEN),
        "book-multiple" => Some(icons::BOOK_MULTIPLE),
        "notebook-multiple" => Some(icons::NOTEBOOK_MULTIPLE),
        "flask-empty-minus-outline" => Some(icons::FLASK_EMPTY_MINUS_OUTLINE),
        "desk-lamp" => Some(icons::DESK_LAMP),
        "leaf-circle" => Some(icons::LEAF_CIRCLE),
        "video-3d-off" => Some(icons::VIDEO_3D_OFF),
        "movie-cog-outline" => Some(icons::MOVIE_COG_OUTLINE),
        "water-minus" => Some(icons::WATER_MINUS),
        "view-headline" => Some(icons::VIEW_HEADLINE),
        "account-box-edit-outline" => Some(icons::ACCOUNT_BOX_EDIT_OUTLINE),
        "kangaroo" => Some(icons::KANGAROO),
        "alarm" => Some(icons::ALARM),
        "calendar-heart" => Some(icons::CALENDAR_HEART),
        "bag-personal-plus" => Some(icons::BAG_PERSONAL_PLUS),
        "file-account" => Some(icons::FILE_ACCOUNT),
        "repeat" => Some(icons::REPEAT),
        "server-network-outline" => Some(icons::SERVER_NETWORK_OUTLINE),
        "phone-check-outline" => Some(icons::PHONE_CHECK_OUTLINE),
        "broadcast" => Some(icons::BROADCAST),
        "shield-cross" => Some(icons::SHIELD_CROSS),
        "delta" => Some(icons::DELTA),
        #[allow(deprecated)]
        "sina-weibo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sina-weibo' is deprecated.").print(py);
            }
            Some(icons::SINA_WEIBO)
        }
        "cash-multiple" => Some(icons::CASH_MULTIPLE),
        "horseshoe" => Some(icons::HORSESHOE),
        "water-percent" => Some(icons::WATER_PERCENT),
        "school-outline" => Some(icons::SCHOOL_OUTLINE),
        "knob" => Some(icons::KNOB),
        "printer-pos-edit" => Some(icons::PRINTER_POS_EDIT),
        "dice-d20" => Some(icons::DICE_D20),
        "receipt-text-plus" => Some(icons::RECEIPT_TEXT_PLUS),
        "camera-document" => Some(icons::CAMERA_DOCUMENT),
        "reiterate" => Some(icons::REITERATE),
        "store-settings" => Some(icons::STORE_SETTINGS),
        "file-move" => Some(icons::FILE_MOVE),
        "alpha-k" => Some(icons::ALPHA_K),
        "home-switch" => Some(icons::HOME_SWITCH),
        "book-multiple-outline" => Some(icons::BOOK_MULTIPLE_OUTLINE),
        "camera-metering-center" => Some(icons::CAMERA_METERING_CENTER),
        "email-search" => Some(icons::EMAIL_SEARCH),
        "safe" => Some(icons::SAFE),
        "drama-masks" => Some(icons::DRAMA_MASKS),
        "tooth" => Some(icons::TOOTH),
        "numeric-2-box" => Some(icons::NUMERIC_2_BOX),
        "airplane-check" => Some(icons::AIRPLANE_CHECK),
        "eye-arrow-right" => Some(icons::EYE_ARROW_RIGHT),
        "sun-snowflake" => Some(icons::SUN_SNOWFLAKE),
        "alpha-u-circle-outline" => Some(icons::ALPHA_U_CIRCLE_OUTLINE),
        "folder-text" => Some(icons::FOLDER_TEXT),
        "table-search" => Some(icons::TABLE_SEARCH),
        "television-play" => Some(icons::TELEVISION_PLAY),
        "message-off" => Some(icons::MESSAGE_OFF),
        "cellphone-settings" => Some(icons::CELLPHONE_SETTINGS),
        "timeline-plus-outline" => Some(icons::TIMELINE_PLUS_OUTLINE),
        "account-multiple-outline" => Some(icons::ACCOUNT_MULTIPLE_OUTLINE),
        "book-plus-multiple" => Some(icons::BOOK_PLUS_MULTIPLE),
        "calendar-refresh-outline" => Some(icons::CALENDAR_REFRESH_OUTLINE),
        "volume-source" => Some(icons::VOLUME_SOURCE),
        "play-box-multiple-outline" => Some(icons::PLAY_BOX_MULTIPLE_OUTLINE),
        "caps-lock" => Some(icons::CAPS_LOCK),
        "record-player" => Some(icons::RECORD_PLAYER),
        "hand-heart-outline" => Some(icons::HAND_HEART_OUTLINE),
        "store-minus-outline" => Some(icons::STORE_MINUS_OUTLINE),
        "panorama" => Some(icons::PANORAMA),
        "furigana-horizontal" => Some(icons::FURIGANA_HORIZONTAL),
        "shield-sun" => Some(icons::SHIELD_SUN),
        "tennis-ball" => Some(icons::TENNIS_BALL),
        #[allow(deprecated)]
        "language-xaml" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-xaml' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_XAML)
        }
        "battery-charging-wireless-40" => Some(icons::BATTERY_CHARGING_WIRELESS_40),
        "square-outline" => Some(icons::SQUARE_OUTLINE),
        "music-note-half-dotted" => Some(icons::MUSIC_NOTE_HALF_DOTTED),
        "sticker-alert" => Some(icons::STICKER_ALERT),
        "tournament" => Some(icons::TOURNAMENT),
        "numeric-2-box-multiple" => Some(icons::NUMERIC_2_BOX_MULTIPLE),
        "home-floor-0" => Some(icons::HOME_FLOOR_0),
        "api-off" => Some(icons::API_OFF),
        "creation-outline" => Some(icons::CREATION_OUTLINE),
        "resistor" => Some(icons::RESISTOR),
        "network-strength-off" => Some(icons::NETWORK_STRENGTH_OFF),
        "cloud-alert" => Some(icons::CLOUD_ALERT),
        "vanish" => Some(icons::VANISH),
        "arrow-bottom-left-bold-box-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_BOX_OUTLINE),
        "snowflake" => Some(icons::SNOWFLAKE),
        "human-female-dance" => Some(icons::HUMAN_FEMALE_DANCE),
        "pine-tree-fire" => Some(icons::PINE_TREE_FIRE),
        "smoking-off" => Some(icons::SMOKING_OFF),
        "arrow-down-circle" => Some(icons::ARROW_DOWN_CIRCLE),
        "battery-unknown" => Some(icons::BATTERY_UNKNOWN),
        "list-status" => Some(icons::LIST_STATUS),
        "message-text-clock" => Some(icons::MESSAGE_TEXT_CLOCK),
        "arrow-right-circle-outline" => Some(icons::ARROW_RIGHT_CIRCLE_OUTLINE),
        "vector-polyline-remove" => Some(icons::VECTOR_POLYLINE_REMOVE),
        "transit-connection-variant" => Some(icons::TRANSIT_CONNECTION_VARIANT),
        "signal-cellular-outline" => Some(icons::SIGNAL_CELLULAR_OUTLINE),
        "format-text-variant-outline" => Some(icons::FORMAT_TEXT_VARIANT_OUTLINE),
        "table-off" => Some(icons::TABLE_OFF),
        "dots-square" => Some(icons::DOTS_SQUARE),
        "television-classic-off" => Some(icons::TELEVISION_CLASSIC_OFF),
        "sigma" => Some(icons::SIGMA),
        "closed-caption" => Some(icons::CLOSED_CAPTION),
        "form-textbox-lock" => Some(icons::FORM_TEXTBOX_LOCK),
        "movie-remove" => Some(icons::MOVIE_REMOVE),
        "content-save-minus" => Some(icons::CONTENT_SAVE_MINUS),
        "cloud-arrow-left" => Some(icons::CLOUD_ARROW_LEFT),
        "clock-end" => Some(icons::CLOCK_END),
        "credit-card-multiple" => Some(icons::CREDIT_CARD_MULTIPLE),
        "magnet-on" => Some(icons::MAGNET_ON),
        "swap-vertical" => Some(icons::SWAP_VERTICAL),
        "filter-cog-outline" => Some(icons::FILTER_COG_OUTLINE),
        "battery-charging-wireless" => Some(icons::BATTERY_CHARGING_WIRELESS),
        "contactless-payment" => Some(icons::CONTACTLESS_PAYMENT),
        "eye-settings" => Some(icons::EYE_SETTINGS),
        "folder-search" => Some(icons::FOLDER_SEARCH),
        "cloud-off" => Some(icons::CLOUD_OFF),
        "motion-sensor" => Some(icons::MOTION_SENSOR),
        "usb-port" => Some(icons::USB_PORT),
        "slot-machine" => Some(icons::SLOT_MACHINE),
        "database-check" => Some(icons::DATABASE_CHECK),
        "gamepad-round-outline" => Some(icons::GAMEPAD_ROUND_OUTLINE),
        "emoticon-dead" => Some(icons::EMOTICON_DEAD),
        "power-plug" => Some(icons::POWER_PLUG),
        "badge-account-outline" => Some(icons::BADGE_ACCOUNT_OUTLINE),
        "star-half-full" => Some(icons::STAR_HALF_FULL),
        "transition-masked" => Some(icons::TRANSITION_MASKED),
        "smoking" => Some(icons::SMOKING),
        #[allow(deprecated)]
        "google-play" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-play' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLAY)
        }
        "gradient-vertical" => Some(icons::GRADIENT_VERTICAL),
        "traffic-light" => Some(icons::TRAFFIC_LIGHT),
        "coffee-maker-outline" => Some(icons::COFFEE_MAKER_OUTLINE),
        "bookmark-check" => Some(icons::BOOKMARK_CHECK),
        "kettle-pour-over" => Some(icons::KETTLE_POUR_OVER),
        "turnstile" => Some(icons::TURNSTILE),
        "food-steak-off" => Some(icons::FOOD_STEAK_OFF),
        "content-copy" => Some(icons::CONTENT_COPY),
        "mixed-reality" => Some(icons::MIXED_REALITY),
        "play-network-outline" => Some(icons::PLAY_NETWORK_OUTLINE),
        "pipe-leak" => Some(icons::PIPE_LEAK),
        "rv-truck" => Some(icons::RV_TRUCK),
        "ticket-account" => Some(icons::TICKET_ACCOUNT),
        "phone-remove-outline" => Some(icons::PHONE_REMOVE_OUTLINE),
        "numeric-2-circle" => Some(icons::NUMERIC_2_CIRCLE),
        "numeric-7" => Some(icons::NUMERIC_7),
        "music-note-whole" => Some(icons::MUSIC_NOTE_WHOLE),
        "weather-windy" => Some(icons::WEATHER_WINDY),
        "pin" => Some(icons::PIN),
        "image-filter-drama-outline" => Some(icons::IMAGE_FILTER_DRAMA_OUTLINE),
        "numeric-8-box-multiple" => Some(icons::NUMERIC_8_BOX_MULTIPLE),
        "expansion-card-variant" => Some(icons::EXPANSION_CARD_VARIANT),
        "file-refresh" => Some(icons::FILE_REFRESH),
        "nfc-tap" => Some(icons::NFC_TAP),
        "incognito-circle" => Some(icons::INCOGNITO_CIRCLE),
        "hand-wash" => Some(icons::HAND_WASH),
        "sleep-off" => Some(icons::SLEEP_OFF),
        "information-slab-box-outline" => Some(icons::INFORMATION_SLAB_BOX_OUTLINE),
        "trash-can-outline" => Some(icons::TRASH_CAN_OUTLINE),
        "gesture-pinch" => Some(icons::GESTURE_PINCH),
        "shield-edit-outline" => Some(icons::SHIELD_EDIT_OUTLINE),
        "bus-alert" => Some(icons::BUS_ALERT),
        "hydro-power" => Some(icons::HYDRO_POWER),
        #[allow(deprecated)]
        "steam" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'steam' is deprecated.").print(py);
            }
            Some(icons::STEAM)
        }
        "passport-plus" => Some(icons::PASSPORT_PLUS),
        "folder-multiple-plus-outline" => Some(icons::FOLDER_MULTIPLE_PLUS_OUTLINE),
        "sofa-single" => Some(icons::SOFA_SINGLE),
        _ => None,
    }
}
