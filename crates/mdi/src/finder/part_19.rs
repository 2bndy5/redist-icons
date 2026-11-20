// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_19(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "treasure-chest-outline" => Some(icons::TREASURE_CHEST_OUTLINE),
        "file-code" => Some(icons::FILE_CODE),
        "book-settings" => Some(icons::BOOK_SETTINGS),
        "molecule-co2" => Some(icons::MOLECULE_CO2),
        "close-box-multiple" => Some(icons::CLOSE_BOX_MULTIPLE),
        "printer-pos-stop-outline" => Some(icons::PRINTER_POS_STOP_OUTLINE),
        "harddisk-plus" => Some(icons::HARDDISK_PLUS),
        "nature-outline" => Some(icons::NATURE_OUTLINE),
        "web-minus" => Some(icons::WEB_MINUS),
        #[allow(deprecated)]
        "steam" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'steam' is deprecated.").print(py);
            }
            Some(icons::STEAM)
        }
        "tent" => Some(icons::TENT),
        "fast-forward-45" => Some(icons::FAST_FORWARD_45),
        "signal-2g" => Some(icons::SIGNAL_2G),
        "sticker-minus" => Some(icons::STICKER_MINUS),
        "emoticon-minus-outline" => Some(icons::EMOTICON_MINUS_OUTLINE),
        "numeric-9-plus-circle" => Some(icons::NUMERIC_9_PLUS_CIRCLE),
        "movie" => Some(icons::MOVIE),
        "camera" => Some(icons::CAMERA),
        "toy-brick-remove-outline" => Some(icons::TOY_BRICK_REMOVE_OUTLINE),
        "numeric-8-box" => Some(icons::NUMERIC_8_BOX),
        "arrow-down-circle" => Some(icons::ARROW_DOWN_CIRCLE),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "power-socket-us" => Some(icons::POWER_SOCKET_US),
        "wind-power" => Some(icons::WIND_POWER),
        "spider-thread" => Some(icons::SPIDER_THREAD),
        "bomb" => Some(icons::BOMB),
        "cog-transfer-outline" => Some(icons::COG_TRANSFER_OUTLINE),
        "account-tie" => Some(icons::ACCOUNT_TIE),
        "solar-panel-large" => Some(icons::SOLAR_PANEL_LARGE),
        "bug-pause" => Some(icons::BUG_PAUSE),
        "layers-plus" => Some(icons::LAYERS_PLUS),
        "alpha-s-circle-outline" => Some(icons::ALPHA_S_CIRCLE_OUTLINE),
        "format-list-bulleted-triangle" => Some(icons::FORMAT_LIST_BULLETED_TRIANGLE),
        "locker" => Some(icons::LOCKER),
        "basketball" => Some(icons::BASKETBALL),
        "close-circle-multiple-outline" => Some(icons::CLOSE_CIRCLE_MULTIPLE_OUTLINE),
        "two-factor-authentication" => Some(icons::TWO_FACTOR_AUTHENTICATION),
        "alpha-i-circle-outline" => Some(icons::ALPHA_I_CIRCLE_OUTLINE),
        "bed-empty" => Some(icons::BED_EMPTY),
        "credit-card-scan-outline" => Some(icons::CREDIT_CARD_SCAN_OUTLINE),
        "minus-box-outline" => Some(icons::MINUS_BOX_OUTLINE),
        "audio-input-rca" => Some(icons::AUDIO_INPUT_RCA),
        "console-network-outline" => Some(icons::CONSOLE_NETWORK_OUTLINE),
        "numeric-7-box" => Some(icons::NUMERIC_7_BOX),
        "bottle-tonic-skull-outline" => Some(icons::BOTTLE_TONIC_SKULL_OUTLINE),
        "format-font" => Some(icons::FORMAT_FONT),
        "epsilon" => Some(icons::EPSILON),
        "menu-swap" => Some(icons::MENU_SWAP),
        "shield-lock" => Some(icons::SHIELD_LOCK),
        "bag-checked" => Some(icons::BAG_CHECKED),
        "bus-stop-uncovered" => Some(icons::BUS_STOP_UNCOVERED),
        "flash-triangle-outline" => Some(icons::FLASH_TRIANGLE_OUTLINE),
        "vacuum" => Some(icons::VACUUM),
        "page-previous" => Some(icons::PAGE_PREVIOUS),
        "selection-multiple-marker" => Some(icons::SELECTION_MULTIPLE_MARKER),
        "checkbox-blank-off-outline" => Some(icons::CHECKBOX_BLANK_OFF_OUTLINE),
        "radio-am" => Some(icons::RADIO_AM),
        #[allow(deprecated)]
        "language-haskell" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-haskell' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_HASKELL)
        }
        "truck-remove" => Some(icons::TRUCK_REMOVE),
        "chart-box" => Some(icons::CHART_BOX),
        "file-question" => Some(icons::FILE_QUESTION),
        #[allow(deprecated)]
        "pokemon-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pokemon-go' is deprecated.").print(py);
            }
            Some(icons::POKEMON_GO)
        }
        "monitor-account" => Some(icons::MONITOR_ACCOUNT),
        #[allow(deprecated)]
        "stack-overflow" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-overflow' is deprecated.").print(py);
            }
            Some(icons::STACK_OVERFLOW)
        }
        "view-module-outline" => Some(icons::VIEW_MODULE_OUTLINE),
        "basket-fill" => Some(icons::BASKET_FILL),
        "alpha-c-box-outline" => Some(icons::ALPHA_C_BOX_OUTLINE),
        "ruler-square" => Some(icons::RULER_SQUARE),
        "printer-pos-wrench" => Some(icons::PRINTER_POS_WRENCH),
        "star-minus-outline" => Some(icons::STAR_MINUS_OUTLINE),
        "numeric-7-circle" => Some(icons::NUMERIC_7_CIRCLE),
        "weather-dust" => Some(icons::WEATHER_DUST),
        "ninja" => Some(icons::NINJA),
        "looks" => Some(icons::LOOKS),
        "octagram-edit-outline" => Some(icons::OCTAGRAM_EDIT_OUTLINE),
        "account-box-multiple-outline" => Some(icons::ACCOUNT_BOX_MULTIPLE_OUTLINE),
        "rotate-360" => Some(icons::ROTATE_360),
        "map-check" => Some(icons::MAP_CHECK),
        "bugle" => Some(icons::BUGLE),
        "wifi-check" => Some(icons::WIFI_CHECK),
        "bluetooth-settings" => Some(icons::BLUETOOTH_SETTINGS),
        "thumb-down" => Some(icons::THUMB_DOWN),
        "arrow-right-drop-circle" => Some(icons::ARROW_RIGHT_DROP_CIRCLE),
        "archive-lock-open" => Some(icons::ARCHIVE_LOCK_OPEN),
        "brightness-7" => Some(icons::BRIGHTNESS_7),
        "factory" => Some(icons::FACTORY),
        "volume-equal" => Some(icons::VOLUME_EQUAL),
        "play" => Some(icons::PLAY),
        "cast-education" => Some(icons::CAST_EDUCATION),
        "volume-off" => Some(icons::VOLUME_OFF),
        "pipe-disconnected" => Some(icons::PIPE_DISCONNECTED),
        "chart-bar" => Some(icons::CHART_BAR),
        "dice-multiple-outline" => Some(icons::DICE_MULTIPLE_OUTLINE),
        "folder-search" => Some(icons::FOLDER_SEARCH),
        "math-cos" => Some(icons::MATH_COS),
        "balcony" => Some(icons::BALCONY),
        "account-convert" => Some(icons::ACCOUNT_CONVERT),
        "chart-line-variant" => Some(icons::CHART_LINE_VARIANT),
        "roller-skate" => Some(icons::ROLLER_SKATE),
        "signature-image" => Some(icons::SIGNATURE_IMAGE),
        "whistle-outline" => Some(icons::WHISTLE_OUTLINE),
        "storefront-remove" => Some(icons::STOREFRONT_REMOVE),
        "rewind-outline" => Some(icons::REWIND_OUTLINE),
        "newspaper-minus" => Some(icons::NEWSPAPER_MINUS),
        "lan-check" => Some(icons::LAN_CHECK),
        "chart-tree" => Some(icons::CHART_TREE),
        "movie-cog" => Some(icons::MOVIE_COG),
        "toothbrush-electric" => Some(icons::TOOTHBRUSH_ELECTRIC),
        "star-four-points-circle" => Some(icons::STAR_FOUR_POINTS_CIRCLE),
        "keyboard-f12" => Some(icons::KEYBOARD_F12),
        "timer-play" => Some(icons::TIMER_PLAY),
        "chevron-down-box" => Some(icons::CHEVRON_DOWN_BOX),
        "timetable" => Some(icons::TIMETABLE),
        "timer-cog" => Some(icons::TIMER_COG),
        "television-speaker-off" => Some(icons::TELEVISION_SPEAKER_OFF),
        "help" => Some(icons::HELP),
        "video-input-antenna" => Some(icons::VIDEO_INPUT_ANTENNA),
        "heart-half-outline" => Some(icons::HEART_HALF_OUTLINE),
        #[allow(deprecated)]
        "language-fortran" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-fortran' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_FORTRAN)
        }
        "feature-search-outline" => Some(icons::FEATURE_SEARCH_OUTLINE),
        "air-horn" => Some(icons::AIR_HORN),
        "shark-fin" => Some(icons::SHARK_FIN),
        "tray-arrow-down" => Some(icons::TRAY_ARROW_DOWN),
        "folder-minus" => Some(icons::FOLDER_MINUS),
        "sitemap" => Some(icons::SITEMAP),
        #[allow(deprecated)]
        "youtube" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE)
        }
        "folder-arrow-up-down-outline" => Some(icons::FOLDER_ARROW_UP_DOWN_OUTLINE),
        "invoice-text-fast" => Some(icons::INVOICE_TEXT_FAST),
        "invoice-arrow-left" => Some(icons::INVOICE_ARROW_LEFT),
        "plane-train" => Some(icons::PLANE_TRAIN),
        "filmstrip-box" => Some(icons::FILMSTRIP_BOX),
        "fan-alert" => Some(icons::FAN_ALERT),
        "medal-outline" => Some(icons::MEDAL_OUTLINE),
        "server-network-off" => Some(icons::SERVER_NETWORK_OFF),
        "alert-minus-outline" => Some(icons::ALERT_MINUS_OUTLINE),
        "cards-heart" => Some(icons::CARDS_HEART),
        "mushroom" => Some(icons::MUSHROOM),
        "door-sliding-open" => Some(icons::DOOR_SLIDING_OPEN),
        "shield-half" => Some(icons::SHIELD_HALF),
        "cookie-lock-outline" => Some(icons::COOKIE_LOCK_OUTLINE),
        "moon-first-quarter" => Some(icons::MOON_FIRST_QUARTER),
        "cart-variant" => Some(icons::CART_VARIANT),
        "file-question-outline" => Some(icons::FILE_QUESTION_OUTLINE),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "folder-move" => Some(icons::FOLDER_MOVE),
        "sun-clock" => Some(icons::SUN_CLOCK),
        "car-3-plus" => Some(icons::CAR_3_PLUS),
        "roman-numeral-8" => Some(icons::ROMAN_NUMERAL_8),
        "seat-passenger" => Some(icons::SEAT_PASSENGER),
        "passport-minus" => Some(icons::PASSPORT_MINUS),
        "water-alert" => Some(icons::WATER_ALERT),
        "av-timer" => Some(icons::AV_TIMER),
        #[allow(deprecated)]
        "google-circles" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CIRCLES)
        }
        "thumbs-up-down-outline" => Some(icons::THUMBS_UP_DOWN_OUTLINE),
        "application-array" => Some(icons::APPLICATION_ARRAY),
        "chart-box-plus-outline" => Some(icons::CHART_BOX_PLUS_OUTLINE),
        "mailbox-up-outline" => Some(icons::MAILBOX_UP_OUTLINE),
        #[allow(deprecated)]
        "microsoft-sharepoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-sharepoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_SHAREPOINT)
        }
        "ticket" => Some(icons::TICKET),
        "cards-playing-club-multiple" => Some(icons::CARDS_PLAYING_CLUB_MULTIPLE),
        "select-multiple" => Some(icons::SELECT_MULTIPLE),
        "card-multiple" => Some(icons::CARD_MULTIPLE),
        "check" => Some(icons::CHECK),
        "toslink" => Some(icons::TOSLINK),
        "bookmark-off-outline" => Some(icons::BOOKMARK_OFF_OUTLINE),
        #[allow(deprecated)]
        "gentoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gentoo' is deprecated.").print(py);
            }
            Some(icons::GENTOO)
        }
        "file-swap-outline" => Some(icons::FILE_SWAP_OUTLINE),
        "book-edit" => Some(icons::BOOK_EDIT),
        "clipboard-alert" => Some(icons::CLIPBOARD_ALERT),
        "bug-stop-outline" => Some(icons::BUG_STOP_OUTLINE),
        "script-outline" => Some(icons::SCRIPT_OUTLINE),
        "newspaper-check" => Some(icons::NEWSPAPER_CHECK),
        "book-arrow-down" => Some(icons::BOOK_ARROW_DOWN),
        "church" => Some(icons::CHURCH),
        "information-box" => Some(icons::INFORMATION_BOX),
        "arrow-up-drop-circle-outline" => Some(icons::ARROW_UP_DROP_CIRCLE_OUTLINE),
        "clock-time-two" => Some(icons::CLOCK_TIME_TWO),
        "shield-cross-outline" => Some(icons::SHIELD_CROSS_OUTLINE),
        "database-check" => Some(icons::DATABASE_CHECK),
        "message-arrow-right-outline" => Some(icons::MESSAGE_ARROW_RIGHT_OUTLINE),
        "numeric-10" => Some(icons::NUMERIC_10),
        "cast-audio" => Some(icons::CAST_AUDIO),
        "numeric-9-plus" => Some(icons::NUMERIC_9_PLUS),
        "tree" => Some(icons::TREE),
        "air-filter" => Some(icons::AIR_FILTER),
        "ev-plug-chademo" => Some(icons::EV_PLUG_CHADEMO),
        "cellphone-settings" => Some(icons::CELLPHONE_SETTINGS),
        "flag-variant-minus" => Some(icons::FLAG_VARIANT_MINUS),
        "broadcast-off" => Some(icons::BROADCAST_OFF),
        "boom-gate-up-outline" => Some(icons::BOOM_GATE_UP_OUTLINE),
        "cursor-default-outline" => Some(icons::CURSOR_DEFAULT_OUTLINE),
        "shape-outline" => Some(icons::SHAPE_OUTLINE),
        "abjad-hebrew" => Some(icons::ABJAD_HEBREW),
        "van-passenger" => Some(icons::VAN_PASSENGER),
        "face-man-shimmer-outline" => Some(icons::FACE_MAN_SHIMMER_OUTLINE),
        "receipt-text-check" => Some(icons::RECEIPT_TEXT_CHECK),
        #[allow(deprecated)]
        "microsoft-word" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-word' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_WORD)
        }
        "content-save-minus" => Some(icons::CONTENT_SAVE_MINUS),
        "necklace" => Some(icons::NECKLACE),
        "account-cog" => Some(icons::ACCOUNT_COG),
        _ => None,
    }
}
