// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_26(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "lightbulb-on-20" => Some(icons::LIGHTBULB_ON_20),
        "battery-minus-variant" => Some(icons::BATTERY_MINUS_VARIANT),
        "heat-pump-outline" => Some(icons::HEAT_PUMP_OUTLINE),
        "file-plus-outline" => Some(icons::FILE_PLUS_OUTLINE),
        "printer-search" => Some(icons::PRINTER_SEARCH),
        "play-protected-content" => Some(icons::PLAY_PROTECTED_CONTENT),
        "meditation" => Some(icons::MEDITATION),
        "space-invaders" => Some(icons::SPACE_INVADERS),
        "gesture-pinch" => Some(icons::GESTURE_PINCH),
        "receipt-text-arrow-right" => Some(icons::RECEIPT_TEXT_ARROW_RIGHT),
        "replay" => Some(icons::REPLAY),
        "reminder" => Some(icons::REMINDER),
        "tag-faces" => Some(icons::TAG_FACES),
        "folder-information" => Some(icons::FOLDER_INFORMATION),
        "vibrate" => Some(icons::VIBRATE),
        "flask-empty-off-outline" => Some(icons::FLASK_EMPTY_OFF_OUTLINE),
        "zodiac-scorpio" => Some(icons::ZODIAC_SCORPIO),
        "signal-4g" => Some(icons::SIGNAL_4G),
        "lock-off" => Some(icons::LOCK_OFF),
        "briefcase-minus-outline" => Some(icons::BRIEFCASE_MINUS_OUTLINE),
        "home-sound-out" => Some(icons::HOME_SOUND_OUT),
        "braille" => Some(icons::BRAILLE),
        #[allow(deprecated)]
        "youtube-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-gaming' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_GAMING)
        }
        "step-backward-2" => Some(icons::STEP_BACKWARD_2),
        "book-music" => Some(icons::BOOK_MUSIC),
        "speaker-pause" => Some(icons::SPEAKER_PAUSE),
        "cake-variant-outline" => Some(icons::CAKE_VARIANT_OUTLINE),
        "rabbit-variant-outline" => Some(icons::RABBIT_VARIANT_OUTLINE),
        "fast-forward-5" => Some(icons::FAST_FORWARD_5),
        "seat-flat-angled" => Some(icons::SEAT_FLAT_ANGLED),
        "heat-pump" => Some(icons::HEAT_PUMP),
        "map-marker-circle" => Some(icons::MAP_MARKER_CIRCLE),
        "message-bulleted-off" => Some(icons::MESSAGE_BULLETED_OFF),
        "cart-check" => Some(icons::CART_CHECK),
        "format-align-justify" => Some(icons::FORMAT_ALIGN_JUSTIFY),
        "restart-alert" => Some(icons::RESTART_ALERT),
        "temperature-kelvin" => Some(icons::TEMPERATURE_KELVIN),
        #[allow(deprecated)]
        "youtube-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-studio' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_STUDIO)
        }
        "information-slab-symbol" => Some(icons::INFORMATION_SLAB_SYMBOL),
        #[allow(deprecated)]
        "dolby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dolby' is deprecated.").print(py);
            }
            Some(icons::DOLBY)
        }
        "content-save-outline" => Some(icons::CONTENT_SAVE_OUTLINE),
        "checkerboard" => Some(icons::CHECKERBOARD),
        "account-music" => Some(icons::ACCOUNT_MUSIC),
        "wifi-arrow-up" => Some(icons::WIFI_ARROW_UP),
        "arrow-left-bold-box-outline" => Some(icons::ARROW_LEFT_BOLD_BOX_OUTLINE),
        "dance-ballroom" => Some(icons::DANCE_BALLROOM),
        "sack-outline" => Some(icons::SACK_OUTLINE),
        "toy-brick-minus" => Some(icons::TOY_BRICK_MINUS),
        "arrow-bottom-left-thick" => Some(icons::ARROW_BOTTOM_LEFT_THICK),
        "account-file-outline" => Some(icons::ACCOUNT_FILE_OUTLINE),
        "cellphone-check" => Some(icons::CELLPHONE_CHECK),
        "cookie-plus-outline" => Some(icons::COOKIE_PLUS_OUTLINE),
        "silverware-spoon" => Some(icons::SILVERWARE_SPOON),
        "content-save-minus" => Some(icons::CONTENT_SAVE_MINUS),
        #[allow(deprecated)]
        "iobroker" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'iobroker' is deprecated.").print(py);
            }
            Some(icons::IOBROKER)
        }
        "wrap-disabled" => Some(icons::WRAP_DISABLED),
        "progress-star-four-points" => Some(icons::PROGRESS_STAR_FOUR_POINTS),
        "bus-alert" => Some(icons::BUS_ALERT),
        "circle-multiple-outline" => Some(icons::CIRCLE_MULTIPLE_OUTLINE),
        "timer-settings" => Some(icons::TIMER_SETTINGS),
        "qrcode-scan" => Some(icons::QRCODE_SCAN),
        "message-arrow-left" => Some(icons::MESSAGE_ARROW_LEFT),
        "water-boiler-alert" => Some(icons::WATER_BOILER_ALERT),
        "file-document-check" => Some(icons::FILE_DOCUMENT_CHECK),
        "table-border" => Some(icons::TABLE_BORDER),
        "application-outline" => Some(icons::APPLICATION_OUTLINE),
        "magic-staff" => Some(icons::MAGIC_STAFF),
        "reflect-vertical" => Some(icons::REFLECT_VERTICAL),
        "shield-sun" => Some(icons::SHIELD_SUN),
        "format-text-variant" => Some(icons::FORMAT_TEXT_VARIANT),
        "table-furniture" => Some(icons::TABLE_FURNITURE),
        "message-flash" => Some(icons::MESSAGE_FLASH),
        "lock-remove" => Some(icons::LOCK_REMOVE),
        "scanner-off" => Some(icons::SCANNER_OFF),
        "format-page-split" => Some(icons::FORMAT_PAGE_SPLIT),
        "message-badge-outline" => Some(icons::MESSAGE_BADGE_OUTLINE),
        "screw-machine-flat-top" => Some(icons::SCREW_MACHINE_FLAT_TOP),
        "file-check" => Some(icons::FILE_CHECK),
        "current-dc" => Some(icons::CURRENT_DC),
        "trash-can" => Some(icons::TRASH_CAN),
        "desktop-tower-monitor" => Some(icons::DESKTOP_TOWER_MONITOR),
        "folder-sync-outline" => Some(icons::FOLDER_SYNC_OUTLINE),
        "inbox-arrow-down" => Some(icons::INBOX_ARROW_DOWN),
        "mailbox-outline" => Some(icons::MAILBOX_OUTLINE),
        "invoice-remove" => Some(icons::INVOICE_REMOVE),
        "door-open" => Some(icons::DOOR_OPEN),
        "view-stream" => Some(icons::VIEW_STREAM),
        "filter-variant" => Some(icons::FILTER_VARIANT),
        "pin" => Some(icons::PIN),
        "hdr" => Some(icons::HDR),
        "virus-outline" => Some(icons::VIRUS_OUTLINE),
        "weather-partly-lightning" => Some(icons::WEATHER_PARTLY_LIGHTNING),
        "relation-zero-or-many-to-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_MANY),
        "usb-port" => Some(icons::USB_PORT),
        #[allow(deprecated)]
        "freebsd" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'freebsd' is deprecated.").print(py);
            }
            Some(icons::FREEBSD)
        }
        "note-minus" => Some(icons::NOTE_MINUS),
        "timer-remove" => Some(icons::TIMER_REMOVE),
        "clipboard-text-multiple" => Some(icons::CLIPBOARD_TEXT_MULTIPLE),
        "clock-time-four-outline" => Some(icons::CLOCK_TIME_FOUR_OUTLINE),
        "keg" => Some(icons::KEG),
        "clipboard-alert-outline" => Some(icons::CLIPBOARD_ALERT_OUTLINE),
        "shield-home" => Some(icons::SHIELD_HOME),
        "message-settings-outline" => Some(icons::MESSAGE_SETTINGS_OUTLINE),
        "paperclip-remove" => Some(icons::PAPERCLIP_REMOVE),
        "select-color" => Some(icons::SELECT_COLOR),
        "file-phone-outline" => Some(icons::FILE_PHONE_OUTLINE),
        "email-search" => Some(icons::EMAIL_SEARCH),
        "store-cog" => Some(icons::STORE_COG),
        "link-box" => Some(icons::LINK_BOX),
        "hand-front-right" => Some(icons::HAND_FRONT_RIGHT),
        "camera-switch" => Some(icons::CAMERA_SWITCH),
        "theater" => Some(icons::THEATER),
        "spa-outline" => Some(icons::SPA_OUTLINE),
        "folder-table-outline" => Some(icons::FOLDER_TABLE_OUTLINE),
        "select-compare" => Some(icons::SELECT_COMPARE),
        "caps-lock" => Some(icons::CAPS_LOCK),
        "beta" => Some(icons::BETA),
        "clipboard-play-multiple" => Some(icons::CLIPBOARD_PLAY_MULTIPLE),
        "shield-crown-outline" => Some(icons::SHIELD_CROWN_OUTLINE),
        "alpha-k" => Some(icons::ALPHA_K),
        "gesture-swipe-right" => Some(icons::GESTURE_SWIPE_RIGHT),
        "sun-angle-outline" => Some(icons::SUN_ANGLE_OUTLINE),
        "folder-swap" => Some(icons::FOLDER_SWAP),
        "chart-bubble" => Some(icons::CHART_BUBBLE),
        "email-heart-outline" => Some(icons::EMAIL_HEART_OUTLINE),
        "apple-keyboard-option" => Some(icons::APPLE_KEYBOARD_OPTION),
        "truck-check-outline" => Some(icons::TRUCK_CHECK_OUTLINE),
        "car-brake-temperature" => Some(icons::CAR_BRAKE_TEMPERATURE),
        "circle-small" => Some(icons::CIRCLE_SMALL),
        "numeric-2-box-multiple" => Some(icons::NUMERIC_2_BOX_MULTIPLE),
        "lightbulb-multiple-outline" => Some(icons::LIGHTBULB_MULTIPLE_OUTLINE),
        "water-minus-outline" => Some(icons::WATER_MINUS_OUTLINE),
        "application-cog" => Some(icons::APPLICATION_COG),
        "heart-outline" => Some(icons::HEART_OUTLINE),
        #[allow(deprecated)]
        "microsoft-windows" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-windows' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS)
        }
        "glass-mug" => Some(icons::GLASS_MUG),
        #[allow(deprecated)]
        "firebase" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firebase' is deprecated.").print(py);
            }
            Some(icons::FIREBASE)
        }
        "desk" => Some(icons::DESK),
        "resize" => Some(icons::RESIZE),
        "bio" => Some(icons::BIO),
        "calendar-expand-horizontal-outline" => Some(icons::CALENDAR_EXPAND_HORIZONTAL_OUTLINE),
        "fridge-industrial-alert-outline" => Some(icons::FRIDGE_INDUSTRIAL_ALERT_OUTLINE),
        "clipboard-text-clock-outline" => Some(icons::CLIPBOARD_TEXT_CLOCK_OUTLINE),
        "skateboarding" => Some(icons::SKATEBOARDING),
        #[allow(deprecated)]
        "umbraco" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'umbraco' is deprecated.").print(py);
            }
            Some(icons::UMBRACO)
        }
        "function" => Some(icons::FUNCTION),
        "minus-box-outline" => Some(icons::MINUS_BOX_OUTLINE),
        "alpha-w-circle-outline" => Some(icons::ALPHA_W_CIRCLE_OUTLINE),
        "arrow-u-right-bottom-bold" => Some(icons::ARROW_U_RIGHT_BOTTOM_BOLD),
        "wifi-strength-3" => Some(icons::WIFI_STRENGTH_3),
        #[allow(deprecated)]
        "steam" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'steam' is deprecated.").print(py);
            }
            Some(icons::STEAM)
        }
        "relation-zero-or-many-to-only-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONLY_ONE),
        "reply-outline" => Some(icons::REPLY_OUTLINE),
        "water-sync" => Some(icons::WATER_SYNC),
        "alien" => Some(icons::ALIEN),
        "dice-6" => Some(icons::DICE_6),
        "key-plus" => Some(icons::KEY_PLUS),
        "gamepad-up" => Some(icons::GAMEPAD_UP),
        "sign-direction-minus" => Some(icons::SIGN_DIRECTION_MINUS),
        "email-outline" => Some(icons::EMAIL_OUTLINE),
        "alpha-t-box" => Some(icons::ALPHA_T_BOX),
        "file-find" => Some(icons::FILE_FIND),
        #[allow(deprecated)]
        "litecoin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'litecoin' is deprecated.").print(py);
            }
            Some(icons::LITECOIN)
        }
        "dog-service" => Some(icons::DOG_SERVICE),
        "relation-zero-or-one-to-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE),
        "flip-horizontal" => Some(icons::FLIP_HORIZONTAL),
        "open-in-new" => Some(icons::OPEN_IN_NEW),
        "image-filter-frames" => Some(icons::IMAGE_FILTER_FRAMES),
        "wrench-clock" => Some(icons::WRENCH_CLOCK),
        "wifi-cancel" => Some(icons::WIFI_CANCEL),
        "layers-off" => Some(icons::LAYERS_OFF),
        "format-align-middle" => Some(icons::FORMAT_ALIGN_MIDDLE),
        "select" => Some(icons::SELECT),
        "alpha-e-box" => Some(icons::ALPHA_E_BOX),
        "cone" => Some(icons::CONE),
        "phone-paused-outline" => Some(icons::PHONE_PAUSED_OUTLINE),
        "calendar-collapse-horizontal" => Some(icons::CALENDAR_COLLAPSE_HORIZONTAL),
        "book-check" => Some(icons::BOOK_CHECK),
        "script" => Some(icons::SCRIPT),
        "account-clock-outline" => Some(icons::ACCOUNT_CLOCK_OUTLINE),
        "selection-drag" => Some(icons::SELECTION_DRAG),
        "account-tie-voice-outline" => Some(icons::ACCOUNT_TIE_VOICE_OUTLINE),
        "movie-edit-outline" => Some(icons::MOVIE_EDIT_OUTLINE),
        "archive-star-outline" => Some(icons::ARCHIVE_STAR_OUTLINE),
        "television-speaker" => Some(icons::TELEVISION_SPEAKER),
        "blood-bag" => Some(icons::BLOOD_BAG),
        "minus-circle-off-outline" => Some(icons::MINUS_CIRCLE_OFF_OUTLINE),
        "hexagon-slice-2" => Some(icons::HEXAGON_SLICE_2),
        "treasure-chest-outline" => Some(icons::TREASURE_CHEST_OUTLINE),
        "baguette" => Some(icons::BAGUETTE),
        "biathlon" => Some(icons::BIATHLON),
        "battery-charging" => Some(icons::BATTERY_CHARGING),
        "laser-pointer" => Some(icons::LASER_POINTER),
        "selection-multiple-marker" => Some(icons::SELECTION_MULTIPLE_MARKER),
        "star-settings" => Some(icons::STAR_SETTINGS),
        "forum-plus-outline" => Some(icons::FORUM_PLUS_OUTLINE),
        "lock-open" => Some(icons::LOCK_OPEN),
        "nas" => Some(icons::NAS),
        "atom-variant" => Some(icons::ATOM_VARIANT),
        "format-letter-case-lower" => Some(icons::FORMAT_LETTER_CASE_LOWER),
        _ => None,
    }
}
