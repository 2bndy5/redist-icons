// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_14(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "home-import-outline" => Some(icons::HOME_IMPORT_OUTLINE),
        "smog" => Some(icons::SMOG),
        "arrow-collapse-up" => Some(icons::ARROW_COLLAPSE_UP),
        "satellite" => Some(icons::SATELLITE),
        "bottle-soda-classic-outline" => Some(icons::BOTTLE_SODA_CLASSIC_OUTLINE),
        "outdoor-lamp" => Some(icons::OUTDOOR_LAMP),
        "dishwasher" => Some(icons::DISHWASHER),
        "playlist-check" => Some(icons::PLAYLIST_CHECK),
        "receipt-text-edit-outline" => Some(icons::RECEIPT_TEXT_EDIT_OUTLINE),
        "shore" => Some(icons::SHORE),
        #[allow(deprecated)]
        "laravel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'laravel' is deprecated.").print(py);
            }
            Some(icons::LARAVEL)
        }
        "car-select" => Some(icons::CAR_SELECT),
        #[allow(deprecated)]
        "unreal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unreal' is deprecated.").print(py);
            }
            Some(icons::UNREAL)
        }
        "basket-remove-outline" => Some(icons::BASKET_REMOVE_OUTLINE),
        "arrow-left-thick" => Some(icons::ARROW_LEFT_THICK),
        "axis-y-arrow-lock" => Some(icons::AXIS_Y_ARROW_LOCK),
        "text-long" => Some(icons::TEXT_LONG),
        "panorama-sphere-outline" => Some(icons::PANORAMA_SPHERE_OUTLINE),
        "sticker-check" => Some(icons::STICKER_CHECK),
        "hydraulic-oil-level" => Some(icons::HYDRAULIC_OIL_LEVEL),
        "step-backward-2" => Some(icons::STEP_BACKWARD_2),
        "heating-coil" => Some(icons::HEATING_COIL),
        "server" => Some(icons::SERVER),
        "alpha-n" => Some(icons::ALPHA_N),
        "backup-restore" => Some(icons::BACKUP_RESTORE),
        "null" => Some(icons::NULL),
        "arrow-up-bold" => Some(icons::ARROW_UP_BOLD),
        "lock" => Some(icons::LOCK),
        "matrix" => Some(icons::MATRIX),
        "minus-network-outline" => Some(icons::MINUS_NETWORK_OUTLINE),
        "passport-plus" => Some(icons::PASSPORT_PLUS),
        "folder-question-outline" => Some(icons::FOLDER_QUESTION_OUTLINE),
        "earth-arrow-right" => Some(icons::EARTH_ARROW_RIGHT),
        "toy-brick-plus-outline" => Some(icons::TOY_BRICK_PLUS_OUTLINE),
        "truck-plus" => Some(icons::TRUCK_PLUS),
        "timeline-minus-outline" => Some(icons::TIMELINE_MINUS_OUTLINE),
        "form-textbox-password" => Some(icons::FORM_TEXTBOX_PASSWORD),
        "remote" => Some(icons::REMOTE),
        "reply-circle" => Some(icons::REPLY_CIRCLE),
        "wifi-arrow-left" => Some(icons::WIFI_ARROW_LEFT),
        "account-arrow-right" => Some(icons::ACCOUNT_ARROW_RIGHT),
        "home-automation" => Some(icons::HOME_AUTOMATION),
        "receipt-text-edit" => Some(icons::RECEIPT_TEXT_EDIT),
        "flare" => Some(icons::FLARE),
        "card-minus" => Some(icons::CARD_MINUS),
        "light-flood-up" => Some(icons::LIGHT_FLOOD_UP),
        "file-move-outline" => Some(icons::FILE_MOVE_OUTLINE),
        "pot-steam" => Some(icons::POT_STEAM),
        "knob" => Some(icons::KNOB),
        "image-off-outline" => Some(icons::IMAGE_OFF_OUTLINE),
        "message-video" => Some(icons::MESSAGE_VIDEO),
        "book-arrow-up" => Some(icons::BOOK_ARROW_UP),
        "web-box" => Some(icons::WEB_BOX),
        "forum-remove" => Some(icons::FORUM_REMOVE),
        "earth-plus" => Some(icons::EARTH_PLUS),
        "connection" => Some(icons::CONNECTION),
        "card-account-mail-outline" => Some(icons::CARD_ACCOUNT_MAIL_OUTLINE),
        "cart-outline" => Some(icons::CART_OUTLINE),
        "fuse-off" => Some(icons::FUSE_OFF),
        "cow" => Some(icons::COW),
        "cards-club-outline" => Some(icons::CARDS_CLUB_OUTLINE),
        "account-box" => Some(icons::ACCOUNT_BOX),
        "hail" => Some(icons::HAIL),
        "inbox-full" => Some(icons::INBOX_FULL),
        "volume-low" => Some(icons::VOLUME_LOW),
        "star-cog" => Some(icons::STAR_COG),
        "chess-knight" => Some(icons::CHESS_KNIGHT),
        "rss-box" => Some(icons::RSS_BOX),
        "directions" => Some(icons::DIRECTIONS),
        "pencil-outline" => Some(icons::PENCIL_OUTLINE),
        "note-alert-outline" => Some(icons::NOTE_ALERT_OUTLINE),
        "database-export-outline" => Some(icons::DATABASE_EXPORT_OUTLINE),
        "head-snowflake-outline" => Some(icons::HEAD_SNOWFLAKE_OUTLINE),
        "unfold-more-horizontal" => Some(icons::UNFOLD_MORE_HORIZONTAL),
        "battery-minus" => Some(icons::BATTERY_MINUS),
        "window-shutter" => Some(icons::WINDOW_SHUTTER),
        "video-input-antenna" => Some(icons::VIDEO_INPUT_ANTENNA),
        "invoice-arrow-left-outline" => Some(icons::INVOICE_ARROW_LEFT_OUTLINE),
        "alpha-h-box" => Some(icons::ALPHA_H_BOX),
        "arrow-up-left-bold" => Some(icons::ARROW_UP_LEFT_BOLD),
        "ev-plug-type2" => Some(icons::EV_PLUG_TYPE2),
        "contrast-circle" => Some(icons::CONTRAST_CIRCLE),
        "table-key" => Some(icons::TABLE_KEY),
        "folder-sync" => Some(icons::FOLDER_SYNC),
        "skip-backward-outline" => Some(icons::SKIP_BACKWARD_OUTLINE),
        "numeric-2" => Some(icons::NUMERIC_2),
        "alpha-f-circle-outline" => Some(icons::ALPHA_F_CIRCLE_OUTLINE),
        "barcode-scan" => Some(icons::BARCODE_SCAN),
        "numeric-1-box-multiple" => Some(icons::NUMERIC_1_BOX_MULTIPLE),
        "arrow-expand-up" => Some(icons::ARROW_EXPAND_UP),
        "alphabet-cyrillic" => Some(icons::ALPHABET_CYRILLIC),
        "gate-xor" => Some(icons::GATE_XOR),
        "alarm" => Some(icons::ALARM),
        "fullscreen-exit" => Some(icons::FULLSCREEN_EXIT),
        "radio" => Some(icons::RADIO),
        "blur-off" => Some(icons::BLUR_OFF),
        #[allow(deprecated)]
        "google-maps" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-maps' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_MAPS)
        }
        "hydrogen-station" => Some(icons::HYDROGEN_STATION),
        "clock-remove" => Some(icons::CLOCK_REMOVE),
        "account-supervisor-circle" => Some(icons::ACCOUNT_SUPERVISOR_CIRCLE),
        "image-edit-outline" => Some(icons::IMAGE_EDIT_OUTLINE),
        "store-check" => Some(icons::STORE_CHECK),
        "set-left" => Some(icons::SET_LEFT),
        "table-tennis" => Some(icons::TABLE_TENNIS),
        "bus-stop-uncovered" => Some(icons::BUS_STOP_UNCOVERED),
        "credit-card-fast-outline" => Some(icons::CREDIT_CARD_FAST_OUTLINE),
        "arrow-u-up-left-bold" => Some(icons::ARROW_U_UP_LEFT_BOLD),
        "emoticon-sick" => Some(icons::EMOTICON_SICK),
        "lightbulb-fluorescent-tube" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE),
        "arrow-left-drop-circle" => Some(icons::ARROW_LEFT_DROP_CIRCLE),
        "alpha-v-circle-outline" => Some(icons::ALPHA_V_CIRCLE_OUTLINE),
        "axe" => Some(icons::AXE),
        "battery-outline" => Some(icons::BATTERY_OUTLINE),
        "shield-link-variant-outline" => Some(icons::SHIELD_LINK_VARIANT_OUTLINE),
        "excavator" => Some(icons::EXCAVATOR),
        "alpha-j-circle-outline" => Some(icons::ALPHA_J_CIRCLE_OUTLINE),
        "book-outline" => Some(icons::BOOK_OUTLINE),
        "projector-screen-variant" => Some(icons::PROJECTOR_SCREEN_VARIANT),
        "angle-acute" => Some(icons::ANGLE_ACUTE),
        "calendar-question-outline" => Some(icons::CALENDAR_QUESTION_OUTLINE),
        "source-branch-sync" => Some(icons::SOURCE_BRANCH_SYNC),
        "horse" => Some(icons::HORSE),
        "car-estate" => Some(icons::CAR_ESTATE),
        "printer-pos-network-outline" => Some(icons::PRINTER_POS_NETWORK_OUTLINE),
        #[allow(deprecated)]
        "language-typescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-typescript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_TYPESCRIPT)
        }
        "alpha-l-box-outline" => Some(icons::ALPHA_L_BOX_OUTLINE),
        #[allow(deprecated)]
        "polymer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'polymer' is deprecated.").print(py);
            }
            Some(icons::POLYMER)
        }
        "sync-off" => Some(icons::SYNC_OFF),
        "timeline-question" => Some(icons::TIMELINE_QUESTION),
        "drag" => Some(icons::DRAG),
        "file-find" => Some(icons::FILE_FIND),
        "cloud-arrow-up" => Some(icons::CLOUD_ARROW_UP),
        "text-box-remove-outline" => Some(icons::TEXT_BOX_REMOVE_OUTLINE),
        "mailbox-open" => Some(icons::MAILBOX_OPEN),
        "menu-swap" => Some(icons::MENU_SWAP),
        "arrange-bring-forward" => Some(icons::ARRANGE_BRING_FORWARD),
        "phone-lock-outline" => Some(icons::PHONE_LOCK_OUTLINE),
        "archive-minus" => Some(icons::ARCHIVE_MINUS),
        "face-recognition" => Some(icons::FACE_RECOGNITION),
        #[allow(deprecated)]
        "dot-net" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dot-net' is deprecated.").print(py);
            }
            Some(icons::DOT_NET)
        }
        "human-cane" => Some(icons::HUMAN_CANE),
        "music-note-whole" => Some(icons::MUSIC_NOTE_WHOLE),
        "arrow-left-bottom" => Some(icons::ARROW_LEFT_BOTTOM),
        "presentation" => Some(icons::PRESENTATION),
        "movie-off" => Some(icons::MOVIE_OFF),
        #[allow(deprecated)]
        "odnoklassniki" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'odnoklassniki' is deprecated.").print(py);
            }
            Some(icons::ODNOKLASSNIKI)
        }
        "seed" => Some(icons::SEED),
        #[allow(deprecated)]
        "youtube" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE)
        }
        "qrcode-edit" => Some(icons::QRCODE_EDIT),
        "advertisements-off" => Some(icons::ADVERTISEMENTS_OFF),
        "palette" => Some(icons::PALETTE),
        "pail" => Some(icons::PAIL),
        "card-off-outline" => Some(icons::CARD_OFF_OUTLINE),
        "relation-zero-or-many-to-zero-or-many" => {
            Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_MANY)
        }
        "store-plus-outline" => Some(icons::STORE_PLUS_OUTLINE),
        "pier-crane" => Some(icons::PIER_CRANE),
        "timer-star" => Some(icons::TIMER_STAR),
        "earth-arrow-down" => Some(icons::EARTH_ARROW_DOWN),
        "bell-alert" => Some(icons::BELL_ALERT),
        "cloud-lock-open" => Some(icons::CLOUD_LOCK_OPEN),
        "city-switch" => Some(icons::CITY_SWITCH),
        "store-minus-outline" => Some(icons::STORE_MINUS_OUTLINE),
        "cloud-print" => Some(icons::CLOUD_PRINT),
        "receipt-text-arrow-right" => Some(icons::RECEIPT_TEXT_ARROW_RIGHT),
        "robot-confused-outline" => Some(icons::ROBOT_CONFUSED_OUTLINE),
        "comment-text" => Some(icons::COMMENT_TEXT),
        "sort-numeric-variant" => Some(icons::SORT_NUMERIC_VARIANT),
        "sun-wireless-outline" => Some(icons::SUN_WIRELESS_OUTLINE),
        "numeric-1-circle-outline" => Some(icons::NUMERIC_1_CIRCLE_OUTLINE),
        "view-parallel-outline" => Some(icons::VIEW_PARALLEL_OUTLINE),
        "focus-auto" => Some(icons::FOCUS_AUTO),
        "town-hall" => Some(icons::TOWN_HALL),
        "briefcase-remove" => Some(icons::BRIEFCASE_REMOVE),
        "magnify-plus-outline" => Some(icons::MAGNIFY_PLUS_OUTLINE),
        "clipboard-alert-outline" => Some(icons::CLIPBOARD_ALERT_OUTLINE),
        "lightbulb-multiple-off" => Some(icons::LIGHTBULB_MULTIPLE_OFF),
        "division" => Some(icons::DIVISION),
        #[allow(deprecated)]
        "origin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'origin' is deprecated.").print(py);
            }
            Some(icons::ORIGIN)
        }
        "countertop" => Some(icons::COUNTERTOP),
        "arrow-decision-outline" => Some(icons::ARROW_DECISION_OUTLINE),
        "briefcase-eye-outline" => Some(icons::BRIEFCASE_EYE_OUTLINE),
        "ip-network-outline" => Some(icons::IP_NETWORK_OUTLINE),
        "candelabra" => Some(icons::CANDELABRA),
        "beer" => Some(icons::BEER),
        "camera-iris" => Some(icons::CAMERA_IRIS),
        "phone-hangup" => Some(icons::PHONE_HANGUP),
        "lock-open-check-outline" => Some(icons::LOCK_OPEN_CHECK_OUTLINE),
        "delete-forever" => Some(icons::DELETE_FOREVER),
        "phone-ring" => Some(icons::PHONE_RING),
        "thermostat-box" => Some(icons::THERMOSTAT_BOX),
        "badge-account-alert" => Some(icons::BADGE_ACCOUNT_ALERT),
        "sticker-alert" => Some(icons::STICKER_ALERT),
        "newspaper-plus" => Some(icons::NEWSPAPER_PLUS),
        "weather-pouring" => Some(icons::WEATHER_POURING),
        "battery-plus" => Some(icons::BATTERY_PLUS),
        "dance-pole" => Some(icons::DANCE_POLE),
        "content-save-plus" => Some(icons::CONTENT_SAVE_PLUS),
        "clock-time-five-outline" => Some(icons::CLOCK_TIME_FIVE_OUTLINE),
        #[allow(deprecated)]
        "atlassian" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'atlassian' is deprecated.").print(py);
            }
            Some(icons::ATLASSIAN)
        }
        "web" => Some(icons::WEB),
        _ => None,
    }
}
