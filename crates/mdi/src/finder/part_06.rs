// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_6(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "beaker-question-outline" => Some(icons::BEAKER_QUESTION_OUTLINE),
        "shield-crown" => Some(icons::SHIELD_CROWN),
        "steering-off" => Some(icons::STEERING_OFF),
        "human-capacity-decrease" => Some(icons::HUMAN_CAPACITY_DECREASE),
        "access-point-check" => Some(icons::ACCESS_POINT_CHECK),
        "guitar-acoustic" => Some(icons::GUITAR_ACOUSTIC),
        "brightness-auto" => Some(icons::BRIGHTNESS_AUTO),
        "flask-empty-off-outline" => Some(icons::FLASK_EMPTY_OFF_OUTLINE),
        "memory" => Some(icons::MEMORY),
        #[allow(deprecated)]
        "jabber" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jabber' is deprecated.").print(py);
            }
            Some(icons::JABBER)
        }
        "domino-mask" => Some(icons::DOMINO_MASK),
        "invoice-text-arrow-left-outline" => Some(icons::INVOICE_TEXT_ARROW_LEFT_OUTLINE),
        "invoice-list-outline" => Some(icons::INVOICE_LIST_OUTLINE),
        "pot" => Some(icons::POT),
        "sticker-alert-outline" => Some(icons::STICKER_ALERT_OUTLINE),
        "file-cog-outline" => Some(icons::FILE_COG_OUTLINE),
        "minus-box" => Some(icons::MINUS_BOX),
        "clipboard-clock" => Some(icons::CLIPBOARD_CLOCK),
        "bank-minus" => Some(icons::BANK_MINUS),
        "bunk-bed-outline" => Some(icons::BUNK_BED_OUTLINE),
        "view-quilt" => Some(icons::VIEW_QUILT),
        "invoice-check-outline" => Some(icons::INVOICE_CHECK_OUTLINE),
        "eye-plus-outline" => Some(icons::EYE_PLUS_OUTLINE),
        "microphone-off" => Some(icons::MICROPHONE_OFF),
        "alpha-j-box-outline" => Some(icons::ALPHA_J_BOX_OUTLINE),
        "dice-5" => Some(icons::DICE_5),
        "numeric-9" => Some(icons::NUMERIC_9),
        "timer-outline" => Some(icons::TIMER_OUTLINE),
        "folder-network" => Some(icons::FOLDER_NETWORK),
        "order-bool-ascending-variant" => Some(icons::ORDER_BOOL_ASCENDING_VARIANT),
        "fire" => Some(icons::FIRE),
        "book-off-outline" => Some(icons::BOOK_OFF_OUTLINE),
        "home-clock" => Some(icons::HOME_CLOCK),
        "alpha-z-circle-outline" => Some(icons::ALPHA_Z_CIRCLE_OUTLINE),
        "escalator-up" => Some(icons::ESCALATOR_UP),
        "file-plus-outline" => Some(icons::FILE_PLUS_OUTLINE),
        "alpha-i-circle" => Some(icons::ALPHA_I_CIRCLE),
        "progress-check" => Some(icons::PROGRESS_CHECK),
        "star-off" => Some(icons::STAR_OFF),
        "filmstrip-box" => Some(icons::FILMSTRIP_BOX),
        "shovel-off" => Some(icons::SHOVEL_OFF),
        "oil-level" => Some(icons::OIL_LEVEL),
        "video-3d-off" => Some(icons::VIDEO_3D_OFF),
        "ellipse" => Some(icons::ELLIPSE),
        "bomb" => Some(icons::BOMB),
        "basketball" => Some(icons::BASKETBALL),
        "television-off" => Some(icons::TELEVISION_OFF),
        "balcony" => Some(icons::BALCONY),
        "archive-eye-outline" => Some(icons::ARCHIVE_EYE_OUTLINE),
        "coffee-off-outline" => Some(icons::COFFEE_OFF_OUTLINE),
        "wall-sconce-round-variant-outline" => Some(icons::WALL_SCONCE_ROUND_VARIANT_OUTLINE),
        "boom-gate-arrow-up" => Some(icons::BOOM_GATE_ARROW_UP),
        "floppy-variant" => Some(icons::FLOPPY_VARIANT),
        "clipboard-search" => Some(icons::CLIPBOARD_SEARCH),
        "axis-z-arrow" => Some(icons::AXIS_Z_ARROW),
        "file-word-box" => Some(icons::FILE_WORD_BOX),
        "dip-switch" => Some(icons::DIP_SWITCH),
        "currency-gbp" => Some(icons::CURRENCY_GBP),
        "projector" => Some(icons::PROJECTOR),
        "door-closed-lock" => Some(icons::DOOR_CLOSED_LOCK),
        "select-arrow-down" => Some(icons::SELECT_ARROW_DOWN),
        "sun-angle" => Some(icons::SUN_ANGLE),
        "truck-fast" => Some(icons::TRUCK_FAST),
        "ceiling-light" => Some(icons::CEILING_LIGHT),
        "checkbox-outline" => Some(icons::CHECKBOX_OUTLINE),
        "file-arrow-left-right" => Some(icons::FILE_ARROW_LEFT_RIGHT),
        "hand-water" => Some(icons::HAND_WATER),
        "cog-refresh" => Some(icons::COG_REFRESH),
        "file-remove-outline" => Some(icons::FILE_REMOVE_OUTLINE),
        "debug-step-over" => Some(icons::DEBUG_STEP_OVER),
        "archive-arrow-up-outline" => Some(icons::ARCHIVE_ARROW_UP_OUTLINE),
        "flip-to-front" => Some(icons::FLIP_TO_FRONT),
        "printer-pos-remove" => Some(icons::PRINTER_POS_REMOVE),
        "playlist-remove" => Some(icons::PLAYLIST_REMOVE),
        "wallet-plus" => Some(icons::WALLET_PLUS),
        "tag-heart-outline" => Some(icons::TAG_HEART_OUTLINE),
        "ticket-outline" => Some(icons::TICKET_OUTLINE),
        "arrow-up-thin" => Some(icons::ARROW_UP_THIN),
        "basket-check" => Some(icons::BASKET_CHECK),
        "credit-card-off" => Some(icons::CREDIT_CARD_OFF),
        "tumble-dryer" => Some(icons::TUMBLE_DRYER),
        "credit-card-plus" => Some(icons::CREDIT_CARD_PLUS),
        "robot-vacuum-off" => Some(icons::ROBOT_VACUUM_OFF),
        "bathtub" => Some(icons::BATHTUB),
        "play-box-multiple-outline" => Some(icons::PLAY_BOX_MULTIPLE_OUTLINE),
        "pot-mix" => Some(icons::POT_MIX),
        "wallet-plus-outline" => Some(icons::WALLET_PLUS_OUTLINE),
        "circle-small" => Some(icons::CIRCLE_SMALL),
        "clock-edit-outline" => Some(icons::CLOCK_EDIT_OUTLINE),
        "folder-network-outline" => Some(icons::FOLDER_NETWORK_OUTLINE),
        "bottle-tonic-skull" => Some(icons::BOTTLE_TONIC_SKULL),
        "land-plots-circle" => Some(icons::LAND_PLOTS_CIRCLE),
        "home-sound-in" => Some(icons::HOME_SOUND_IN),
        "baseball-diamond-outline" => Some(icons::BASEBALL_DIAMOND_OUTLINE),
        "monitor-dashboard" => Some(icons::MONITOR_DASHBOARD),
        "storefront-remove" => Some(icons::STOREFRONT_REMOVE),
        "video-wireless-outline" => Some(icons::VIDEO_WIRELESS_OUTLINE),
        "cash-lock-open" => Some(icons::CASH_LOCK_OPEN),
        "scoreboard" => Some(icons::SCOREBOARD),
        "bookmark-outline" => Some(icons::BOOKMARK_OUTLINE),
        "account-tie-hat" => Some(icons::ACCOUNT_TIE_HAT),
        "head-dots-horizontal-outline" => Some(icons::HEAD_DOTS_HORIZONTAL_OUTLINE),
        "book-multiple-outline" => Some(icons::BOOK_MULTIPLE_OUTLINE),
        "stairs-down" => Some(icons::STAIRS_DOWN),
        "signal-cellular-1" => Some(icons::SIGNAL_CELLULAR_1),
        "fridge-outline" => Some(icons::FRIDGE_OUTLINE),
        "generator-portable" => Some(icons::GENERATOR_PORTABLE),
        "alpha-e-box-outline" => Some(icons::ALPHA_E_BOX_OUTLINE),
        "sort-bool-ascending" => Some(icons::SORT_BOOL_ASCENDING),
        "map-marker-remove" => Some(icons::MAP_MARKER_REMOVE),
        "roman-numeral-4" => Some(icons::ROMAN_NUMERAL_4),
        "emoticon-sick" => Some(icons::EMOTICON_SICK),
        "label" => Some(icons::LABEL),
        "alpha-e-circle" => Some(icons::ALPHA_E_CIRCLE),
        "robot-excited-outline" => Some(icons::ROBOT_EXCITED_OUTLINE),
        "note-alert" => Some(icons::NOTE_ALERT),
        "alpha-b-box" => Some(icons::ALPHA_B_BOX),
        "filter-variant-minus" => Some(icons::FILTER_VARIANT_MINUS),
        "arrow-bottom-right-bold-outline" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_OUTLINE),
        "speaker-message" => Some(icons::SPEAKER_MESSAGE),
        "arrow-down" => Some(icons::ARROW_DOWN),
        "numeric-3-box-multiple-outline" => Some(icons::NUMERIC_3_BOX_MULTIPLE_OUTLINE),
        "arrow-left-circle" => Some(icons::ARROW_LEFT_CIRCLE),
        "thought-bubble" => Some(icons::THOUGHT_BUBBLE),
        "send-clock" => Some(icons::SEND_CLOCK),
        #[allow(deprecated)]
        "language-haskell" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-haskell' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_HASKELL)
        }
        "bullhorn-outline" => Some(icons::BULLHORN_OUTLINE),
        "sticker-remove-outline" => Some(icons::STICKER_REMOVE_OUTLINE),
        "timer-settings-outline" => Some(icons::TIMER_SETTINGS_OUTLINE),
        "archive-search" => Some(icons::ARCHIVE_SEARCH),
        "hand-back-right-outline" => Some(icons::HAND_BACK_RIGHT_OUTLINE),
        "pause-circle-outline" => Some(icons::PAUSE_CIRCLE_OUTLINE),
        "video-off-outline" => Some(icons::VIDEO_OFF_OUTLINE),
        "folder-clock" => Some(icons::FOLDER_CLOCK),
        #[allow(deprecated)]
        "pi-hole" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pi-hole' is deprecated.").print(py);
            }
            Some(icons::PI_HOLE)
        }
        "safety-goggles" => Some(icons::SAFETY_GOGGLES),
        "numeric-7-box-outline" => Some(icons::NUMERIC_7_BOX_OUTLINE),
        "alpha-y" => Some(icons::ALPHA_Y),
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
        "book-edit" => Some(icons::BOOK_EDIT),
        "antenna" => Some(icons::ANTENNA),
        "alert-remove-outline" => Some(icons::ALERT_REMOVE_OUTLINE),
        "boom-gate-arrow-down-outline" => Some(icons::BOOM_GATE_ARROW_DOWN_OUTLINE),
        "hockey-puck" => Some(icons::HOCKEY_PUCK),
        "mouse-scroll-wheel" => Some(icons::MOUSE_SCROLL_WHEEL),
        "fridge-industrial-alert" => Some(icons::FRIDGE_INDUSTRIAL_ALERT),
        "rowing" => Some(icons::ROWING),
        "piano-off" => Some(icons::PIANO_OFF),
        "comment-off" => Some(icons::COMMENT_OFF),
        "shaker" => Some(icons::SHAKER),
        "numeric-9-box" => Some(icons::NUMERIC_9_BOX),
        "charity" => Some(icons::CHARITY),
        #[allow(deprecated)]
        "google-fit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-fit' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_FIT)
        }
        "arrow-right-bold" => Some(icons::ARROW_RIGHT_BOLD),
        "eye-closed" => Some(icons::EYE_CLOSED),
        "label-outline" => Some(icons::LABEL_OUTLINE),
        "newspaper-minus" => Some(icons::NEWSPAPER_MINUS),
        "panorama-sphere" => Some(icons::PANORAMA_SPHERE),
        "middleware-outline" => Some(icons::MIDDLEWARE_OUTLINE),
        "sofa" => Some(icons::SOFA),
        "water-opacity" => Some(icons::WATER_OPACITY),
        "image-marker" => Some(icons::IMAGE_MARKER),
        "axis-arrow-lock" => Some(icons::AXIS_ARROW_LOCK),
        "fence" => Some(icons::FENCE),
        "clipboard-plus-outline" => Some(icons::CLIPBOARD_PLUS_OUTLINE),
        "audio-video" => Some(icons::AUDIO_VIDEO),
        "silverware" => Some(icons::SILVERWARE),
        "folder-lock" => Some(icons::FOLDER_LOCK),
        "format-list-text" => Some(icons::FORMAT_LIST_TEXT),
        #[allow(deprecated)]
        "freebsd" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'freebsd' is deprecated.").print(py);
            }
            Some(icons::FREEBSD)
        }
        "map-marker-down" => Some(icons::MAP_MARKER_DOWN),
        "floor-lamp-outline" => Some(icons::FLOOR_LAMP_OUTLINE),
        "format-header-equal" => Some(icons::FORMAT_HEADER_EQUAL),
        "power-socket-us" => Some(icons::POWER_SOCKET_US),
        "bug-play" => Some(icons::BUG_PLAY),
        "home-plus" => Some(icons::HOME_PLUS),
        "view-list" => Some(icons::VIEW_LIST),
        "clock-alert-outline" => Some(icons::CLOCK_ALERT_OUTLINE),
        "office-building-remove" => Some(icons::OFFICE_BUILDING_REMOVE),
        "cursor-default" => Some(icons::CURSOR_DEFAULT),
        "account-arrow-down" => Some(icons::ACCOUNT_ARROW_DOWN),
        "wifi-sync" => Some(icons::WIFI_SYNC),
        "weather-rainy" => Some(icons::WEATHER_RAINY),
        "gantry-crane" => Some(icons::GANTRY_CRANE),
        "perspective-more" => Some(icons::PERSPECTIVE_MORE),
        "book-settings-outline" => Some(icons::BOOK_SETTINGS_OUTLINE),
        "lock-open-variant-outline" => Some(icons::LOCK_OPEN_VARIANT_OUTLINE),
        "gas-station-off-outline" => Some(icons::GAS_STATION_OFF_OUTLINE),
        "sticker-check" => Some(icons::STICKER_CHECK),
        "code-brackets" => Some(icons::CODE_BRACKETS),
        "tilde" => Some(icons::TILDE),
        "table-eye" => Some(icons::TABLE_EYE),
        "alert-minus-outline" => Some(icons::ALERT_MINUS_OUTLINE),
        "checkbox-blank-badge-outline" => Some(icons::CHECKBOX_BLANK_BADGE_OUTLINE),
        "key-link" => Some(icons::KEY_LINK),
        "sail-boat-sink" => Some(icons::SAIL_BOAT_SINK),
        "numeric-9-box-multiple-outline" => Some(icons::NUMERIC_9_BOX_MULTIPLE_OUTLINE),
        "bluetooth-settings" => Some(icons::BLUETOOTH_SETTINGS),
        "function-variant" => Some(icons::FUNCTION_VARIANT),
        "eye-circle-outline" => Some(icons::EYE_CIRCLE_OUTLINE),
        _ => None,
    }
}
