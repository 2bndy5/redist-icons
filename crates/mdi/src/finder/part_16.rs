// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_16(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "bullseye-arrow" => Some(icons::BULLSEYE_ARROW),
        "read" => Some(icons::READ),
        "battery-lock" => Some(icons::BATTERY_LOCK),
        "invoice-text-send" => Some(icons::INVOICE_TEXT_SEND),
        "map-marker-down" => Some(icons::MAP_MARKER_DOWN),
        "cloud-off" => Some(icons::CLOUD_OFF),
        "arrow-down-box" => Some(icons::ARROW_DOWN_BOX),
        "bucket" => Some(icons::BUCKET),
        "file-image-outline" => Some(icons::FILE_IMAGE_OUTLINE),
        "link-circle-outline" => Some(icons::LINK_CIRCLE_OUTLINE),
        "scale-off" => Some(icons::SCALE_OFF),
        "cookie-edit-outline" => Some(icons::COOKIE_EDIT_OUTLINE),
        "cloud-arrow-right" => Some(icons::CLOUD_ARROW_RIGHT),
        #[allow(deprecated)]
        "artstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'artstation' is deprecated.").print(py);
            }
            Some(icons::ARTSTATION)
        }
        "cart-arrow-up" => Some(icons::CART_ARROW_UP),
        "flip-to-front" => Some(icons::FLIP_TO_FRONT),
        "temple-buddhist" => Some(icons::TEMPLE_BUDDHIST),
        "case-sensitive-alt" => Some(icons::CASE_SENSITIVE_ALT),
        "duck" => Some(icons::DUCK),
        "book-open-blank-variant-outline" => Some(icons::BOOK_OPEN_BLANK_VARIANT_OUTLINE),
        "plus-circle-multiple-outline" => Some(icons::PLUS_CIRCLE_MULTIPLE_OUTLINE),
        "relation-one-to-only-one" => Some(icons::RELATION_ONE_TO_ONLY_ONE),
        "arrow-u-left-bottom" => Some(icons::ARROW_U_LEFT_BOTTOM),
        "human-greeting-proximity" => Some(icons::HUMAN_GREETING_PROXIMITY),
        "calendar-blank-multiple" => Some(icons::CALENDAR_BLANK_MULTIPLE),
        "calendar-alert" => Some(icons::CALENDAR_ALERT),
        "gate-arrow-right" => Some(icons::GATE_ARROW_RIGHT),
        "dots-grid" => Some(icons::DOTS_GRID),
        #[allow(deprecated)]
        "origin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'origin' is deprecated.").print(py);
            }
            Some(icons::ORIGIN)
        }
        "record-circle-outline" => Some(icons::RECORD_CIRCLE_OUTLINE),
        "file-image-plus" => Some(icons::FILE_IMAGE_PLUS),
        "wifi-strength-4-lock-open" => Some(icons::WIFI_STRENGTH_4_LOCK_OPEN),
        "ph" => Some(icons::PH),
        "post" => Some(icons::POST),
        "book-off-outline" => Some(icons::BOOK_OFF_OUTLINE),
        "archive-marker" => Some(icons::ARCHIVE_MARKER),
        "silverware-fork-knife" => Some(icons::SILVERWARE_FORK_KNIFE),
        "unfold-less-vertical" => Some(icons::UNFOLD_LESS_VERTICAL),
        #[allow(deprecated)]
        "openid" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'openid' is deprecated.").print(py);
            }
            Some(icons::OPENID)
        }
        "account-credit-card" => Some(icons::ACCOUNT_CREDIT_CARD),
        "ticket-percent-outline" => Some(icons::TICKET_PERCENT_OUTLINE),
        "arrow-left" => Some(icons::ARROW_LEFT),
        "cards-playing-diamond-multiple" => Some(icons::CARDS_PLAYING_DIAMOND_MULTIPLE),
        "arrow-horizontal-lock" => Some(icons::ARROW_HORIZONTAL_LOCK),
        "alert" => Some(icons::ALERT),
        "clipboard-plus" => Some(icons::CLIPBOARD_PLUS),
        "minus-box-multiple" => Some(icons::MINUS_BOX_MULTIPLE),
        "percent-box-outline" => Some(icons::PERCENT_BOX_OUTLINE),
        "checkbox-multiple-marked" => Some(icons::CHECKBOX_MULTIPLE_MARKED),
        "package-variant" => Some(icons::PACKAGE_VARIANT),
        "file-send" => Some(icons::FILE_SEND),
        "forum-outline" => Some(icons::FORUM_OUTLINE),
        "numeric-5-circle-outline" => Some(icons::NUMERIC_5_CIRCLE_OUTLINE),
        "lock-clock" => Some(icons::LOCK_CLOCK),
        "monitor-shimmer" => Some(icons::MONITOR_SHIMMER),
        "pistol" => Some(icons::PISTOL),
        "map-marker" => Some(icons::MAP_MARKER),
        "alpha-t-circle-outline" => Some(icons::ALPHA_T_CIRCLE_OUTLINE),
        "stairs-down" => Some(icons::STAIRS_DOWN),
        "api" => Some(icons::API),
        "desk-lamp-on" => Some(icons::DESK_LAMP_ON),
        "invert-colors-off" => Some(icons::INVERT_COLORS_OFF),
        "email-plus" => Some(icons::EMAIL_PLUS),
        "radiator-off" => Some(icons::RADIATOR_OFF),
        "axe" => Some(icons::AXE),
        "car-door" => Some(icons::CAR_DOOR),
        "thermometer-chevron-up" => Some(icons::THERMOMETER_CHEVRON_UP),
        "account-arrow-down-outline" => Some(icons::ACCOUNT_ARROW_DOWN_OUTLINE),
        "shimmer" => Some(icons::SHIMMER),
        "alpha-f" => Some(icons::ALPHA_F),
        "teddy-bear" => Some(icons::TEDDY_BEAR),
        "heart-pulse" => Some(icons::HEART_PULSE),
        "weight-kilogram" => Some(icons::WEIGHT_KILOGRAM),
        "comment-off-outline" => Some(icons::COMMENT_OFF_OUTLINE),
        "boom-gate-alert" => Some(icons::BOOM_GATE_ALERT),
        "bell-plus" => Some(icons::BELL_PLUS),
        "keyboard-caps" => Some(icons::KEYBOARD_CAPS),
        "file-move-outline" => Some(icons::FILE_MOVE_OUTLINE),
        "email-open-multiple" => Some(icons::EMAIL_OPEN_MULTIPLE),
        "fullscreen-exit" => Some(icons::FULLSCREEN_EXIT),
        "movie-outline" => Some(icons::MOVIE_OUTLINE),
        "send-variant" => Some(icons::SEND_VARIANT),
        "sleep" => Some(icons::SLEEP),
        "clock-time-six-outline" => Some(icons::CLOCK_TIME_SIX_OUTLINE),
        "propane-tank" => Some(icons::PROPANE_TANK),
        "school" => Some(icons::SCHOOL),
        "flip-to-back" => Some(icons::FLIP_TO_BACK),
        "shoe-cleat" => Some(icons::SHOE_CLEAT),
        "invoice-multiple" => Some(icons::INVOICE_MULTIPLE),
        "professional-hexagon" => Some(icons::PROFESSIONAL_HEXAGON),
        "web-cancel" => Some(icons::WEB_CANCEL),
        "wiper" => Some(icons::WIPER),
        "file-cog-outline" => Some(icons::FILE_COG_OUTLINE),
        "dolphin" => Some(icons::DOLPHIN),
        "color-helper" => Some(icons::COLOR_HELPER),
        "code-tags" => Some(icons::CODE_TAGS),
        "bell-cancel-outline" => Some(icons::BELL_CANCEL_OUTLINE),
        "briefcase-arrow-left-right" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT),
        #[allow(deprecated)]
        "emby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'emby' is deprecated.").print(py);
            }
            Some(icons::EMBY)
        }
        "fingerprint" => Some(icons::FINGERPRINT),
        "printer-3d-nozzle" => Some(icons::PRINTER_3D_NOZZLE),
        "comment-plus-outline" => Some(icons::COMMENT_PLUS_OUTLINE),
        "printer-pos-network" => Some(icons::PRINTER_POS_NETWORK),
        "turnstile-outline" => Some(icons::TURNSTILE_OUTLINE),
        "cross" => Some(icons::CROSS),
        "alarm-check" => Some(icons::ALARM_CHECK),
        "cog-play-outline" => Some(icons::COG_PLAY_OUTLINE),
        "chat" => Some(icons::CHAT),
        "comment-bookmark-outline" => Some(icons::COMMENT_BOOKMARK_OUTLINE),
        "emoticon-frown-outline" => Some(icons::EMOTICON_FROWN_OUTLINE),
        "fence" => Some(icons::FENCE),
        "map-marker-check" => Some(icons::MAP_MARKER_CHECK),
        "credit-card-multiple-outline" => Some(icons::CREDIT_CARD_MULTIPLE_OUTLINE),
        "wifi-settings" => Some(icons::WIFI_SETTINGS),
        "layers-remove" => Some(icons::LAYERS_REMOVE),
        "fountain" => Some(icons::FOUNTAIN),
        "shield-account" => Some(icons::SHIELD_ACCOUNT),
        "image-multiple-outline" => Some(icons::IMAGE_MULTIPLE_OUTLINE),
        "bowl-mix" => Some(icons::BOWL_MIX),
        "boxing-glove" => Some(icons::BOXING_GLOVE),
        "human-baby-changing-table" => Some(icons::HUMAN_BABY_CHANGING_TABLE),
        "close-octagon-outline" => Some(icons::CLOSE_OCTAGON_OUTLINE),
        "flag-variant-off" => Some(icons::FLAG_VARIANT_OFF),
        "baby-bottle" => Some(icons::BABY_BOTTLE),
        "cursor-default-gesture-outline" => Some(icons::CURSOR_DEFAULT_GESTURE_OUTLINE),
        "message-lock" => Some(icons::MESSAGE_LOCK),
        "numeric-5-box" => Some(icons::NUMERIC_5_BOX),
        "order-numeric-ascending" => Some(icons::ORDER_NUMERIC_ASCENDING),
        "meter-electric" => Some(icons::METER_ELECTRIC),
        "phone-missed" => Some(icons::PHONE_MISSED),
        "notebook-check" => Some(icons::NOTEBOOK_CHECK),
        "eye-remove-outline" => Some(icons::EYE_REMOVE_OUTLINE),
        "skull-crossbones" => Some(icons::SKULL_CROSSBONES),
        "notebook-remove" => Some(icons::NOTEBOOK_REMOVE),
        "image-filter-drama" => Some(icons::IMAGE_FILTER_DRAMA),
        "book-multiple-outline" => Some(icons::BOOK_MULTIPLE_OUTLINE),
        "kettle-alert" => Some(icons::KETTLE_ALERT),
        "movie-roll" => Some(icons::MOVIE_ROLL),
        "ethernet" => Some(icons::ETHERNET),
        "emoticon-outline" => Some(icons::EMOTICON_OUTLINE),
        "popcorn" => Some(icons::POPCORN),
        "whistle" => Some(icons::WHISTLE),
        "cash-marker" => Some(icons::CASH_MARKER),
        "alert-box-outline" => Some(icons::ALERT_BOX_OUTLINE),
        "panorama-vertical" => Some(icons::PANORAMA_VERTICAL),
        "qrcode" => Some(icons::QRCODE),
        "arrow-projectile-multiple" => Some(icons::ARROW_PROJECTILE_MULTIPLE),
        "upload-multiple" => Some(icons::UPLOAD_MULTIPLE),
        "link-variant-minus" => Some(icons::LINK_VARIANT_MINUS),
        "comment-text" => Some(icons::COMMENT_TEXT),
        "panorama-fisheye" => Some(icons::PANORAMA_FISHEYE),
        "kettle-steam" => Some(icons::KETTLE_STEAM),
        "toy-brick" => Some(icons::TOY_BRICK),
        "transmission-tower-import" => Some(icons::TRANSMISSION_TOWER_IMPORT),
        "phone-return" => Some(icons::PHONE_RETURN),
        "cards-playing-diamond-multiple-outline" => {
            Some(icons::CARDS_PLAYING_DIAMOND_MULTIPLE_OUTLINE)
        }
        "projector-screen-variant-off-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF_OUTLINE),
        "fan-chevron-down" => Some(icons::FAN_CHEVRON_DOWN),
        "package-variant-minus" => Some(icons::PACKAGE_VARIANT_MINUS),
        "pound" => Some(icons::POUND),
        "contacts" => Some(icons::CONTACTS),
        "tooltip-cellphone" => Some(icons::TOOLTIP_CELLPHONE),
        "message-fast" => Some(icons::MESSAGE_FAST),
        "cards-outline" => Some(icons::CARDS_OUTLINE),
        "horse-variant" => Some(icons::HORSE_VARIANT),
        "longitude" => Some(icons::LONGITUDE),
        "wifi-strength-4-lock" => Some(icons::WIFI_STRENGTH_4_LOCK),
        "scanner-off" => Some(icons::SCANNER_OFF),
        "keyboard-f5" => Some(icons::KEYBOARD_F5),
        "car-parking-lights" => Some(icons::CAR_PARKING_LIGHTS),
        "location-exit" => Some(icons::LOCATION_EXIT),
        "battery-medium" => Some(icons::BATTERY_MEDIUM),
        "star-remove" => Some(icons::STAR_REMOVE),
        "contain-start" => Some(icons::CONTAIN_START),
        "pause-octagon" => Some(icons::PAUSE_OCTAGON),
        "hammer-screwdriver" => Some(icons::HAMMER_SCREWDRIVER),
        "select-all" => Some(icons::SELECT_ALL),
        "library-outline" => Some(icons::LIBRARY_OUTLINE),
        "shape-square-rounded-plus" => Some(icons::SHAPE_SQUARE_ROUNDED_PLUS),
        "vector-point" => Some(icons::VECTOR_POINT),
        "transition" => Some(icons::TRANSITION),
        "storage-tank" => Some(icons::STORAGE_TANK),
        "clipboard-multiple-outline" => Some(icons::CLIPBOARD_MULTIPLE_OUTLINE),
        "bank-check" => Some(icons::BANK_CHECK),
        "chevron-double-down" => Some(icons::CHEVRON_DOUBLE_DOWN),
        "printer-pos-remove" => Some(icons::PRINTER_POS_REMOVE),
        "play-box-outline" => Some(icons::PLAY_BOX_OUTLINE),
        "account-clock" => Some(icons::ACCOUNT_CLOCK),
        "arrow-left-bottom-bold" => Some(icons::ARROW_LEFT_BOTTOM_BOLD),
        "disc-alert" => Some(icons::DISC_ALERT),
        "sync-off" => Some(icons::SYNC_OFF),
        "cloud-lock-outline" => Some(icons::CLOUD_LOCK_OUTLINE),
        "package-variant-plus" => Some(icons::PACKAGE_VARIANT_PLUS),
        "mirror" => Some(icons::MIRROR),
        "arrow-collapse-right" => Some(icons::ARROW_COLLAPSE_RIGHT),
        "store-plus" => Some(icons::STORE_PLUS),
        "file-download" => Some(icons::FILE_DOWNLOAD),
        "line-scan" => Some(icons::LINE_SCAN),
        "keyboard-settings-outline" => Some(icons::KEYBOARD_SETTINGS_OUTLINE),
        "monitor-arrow-down-variant" => Some(icons::MONITOR_ARROW_DOWN_VARIANT),
        _ => None,
    }
}
