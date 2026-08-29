// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_1(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "receipt-text-arrow-right" => Some(icons::RECEIPT_TEXT_ARROW_RIGHT),
        "cake-variant-outline" => Some(icons::CAKE_VARIANT_OUTLINE),
        "home-lightning-bolt-outline" => Some(icons::HOME_LIGHTNING_BOLT_OUTLINE),
        "briefcase-edit-outline" => Some(icons::BRIEFCASE_EDIT_OUTLINE),
        "content-save-all" => Some(icons::CONTENT_SAVE_ALL),
        "record-player" => Some(icons::RECORD_PLAYER),
        "water-minus" => Some(icons::WATER_MINUS),
        "beaker-minus-outline" => Some(icons::BEAKER_MINUS_OUTLINE),
        "relation-zero-or-one-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ZERO_OR_ONE),
        "gamepad-variant-outline" => Some(icons::GAMEPAD_VARIANT_OUTLINE),
        "magnify-expand" => Some(icons::MAGNIFY_EXPAND),
        "perspective-more" => Some(icons::PERSPECTIVE_MORE),
        "chat-sleep" => Some(icons::CHAT_SLEEP),
        "invoice-text-fast" => Some(icons::INVOICE_TEXT_FAST),
        #[allow(deprecated)]
        "redhat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'redhat' is deprecated.").print(py);
            }
            Some(icons::REDHAT)
        }
        "grass" => Some(icons::GRASS),
        "battery-negative" => Some(icons::BATTERY_NEGATIVE),
        "shield-off" => Some(icons::SHIELD_OFF),
        "gamepad-variant" => Some(icons::GAMEPAD_VARIANT),
        "music-note-sixteenth-dotted" => Some(icons::MUSIC_NOTE_SIXTEENTH_DOTTED),
        "rollerblade" => Some(icons::ROLLERBLADE),
        "source-fork" => Some(icons::SOURCE_FORK),
        "skip-previous-circle-outline" => Some(icons::SKIP_PREVIOUS_CIRCLE_OUTLINE),
        "transit-connection-variant" => Some(icons::TRANSIT_CONNECTION_VARIANT),
        "cursor-default-click" => Some(icons::CURSOR_DEFAULT_CLICK),
        "high-definition" => Some(icons::HIGH_DEFINITION),
        "format-page-split" => Some(icons::FORMAT_PAGE_SPLIT),
        "home-export-outline" => Some(icons::HOME_EXPORT_OUTLINE),
        "cake-variant" => Some(icons::CAKE_VARIANT),
        "wifi-strength-4-lock-open" => Some(icons::WIFI_STRENGTH_4_LOCK_OPEN),
        "invoice-remove-outline" => Some(icons::INVOICE_REMOVE_OUTLINE),
        "desktop-tower" => Some(icons::DESKTOP_TOWER),
        "smoking-pipe-off" => Some(icons::SMOKING_PIPE_OFF),
        "home-silo" => Some(icons::HOME_SILO),
        "shape-square-plus" => Some(icons::SHAPE_SQUARE_PLUS),
        "map" => Some(icons::MAP),
        "contactless-payment-circle-outline" => Some(icons::CONTACTLESS_PAYMENT_CIRCLE_OUTLINE),
        "map-outline" => Some(icons::MAP_OUTLINE),
        "cylinder-off" => Some(icons::CYLINDER_OFF),
        "information-off" => Some(icons::INFORMATION_OFF),
        "playlist-minus" => Some(icons::PLAYLIST_MINUS),
        "arrow-expand-right" => Some(icons::ARROW_EXPAND_RIGHT),
        "emoticon-excited-outline" => Some(icons::EMOTICON_EXCITED_OUTLINE),
        "domain-off" => Some(icons::DOMAIN_OFF),
        "signature-image" => Some(icons::SIGNATURE_IMAGE),
        "chart-line-stacked" => Some(icons::CHART_LINE_STACKED),
        "receipt-text-edit-outline" => Some(icons::RECEIPT_TEXT_EDIT_OUTLINE),
        "account-search" => Some(icons::ACCOUNT_SEARCH),
        "timer-play" => Some(icons::TIMER_PLAY),
        "cloud-circle-outline" => Some(icons::CLOUD_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "lastpass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'lastpass' is deprecated.").print(py);
            }
            Some(icons::LASTPASS)
        }
        "car-tire-alert" => Some(icons::CAR_TIRE_ALERT),
        "video-high-definition" => Some(icons::VIDEO_HIGH_DEFINITION),
        "bus-articulated-end" => Some(icons::BUS_ARTICULATED_END),
        "battery-check-outline" => Some(icons::BATTERY_CHECK_OUTLINE),
        "truck-check" => Some(icons::TRUCK_CHECK),
        "clock-time-three" => Some(icons::CLOCK_TIME_THREE),
        "file-chart" => Some(icons::FILE_CHART),
        "finance" => Some(icons::FINANCE),
        "flag-variant-plus" => Some(icons::FLAG_VARIANT_PLUS),
        "function" => Some(icons::FUNCTION),
        "numeric-7-box-outline" => Some(icons::NUMERIC_7_BOX_OUTLINE),
        "database-minus-outline" => Some(icons::DATABASE_MINUS_OUTLINE),
        "tire" => Some(icons::TIRE),
        "ideogram-cjk" => Some(icons::IDEOGRAM_CJK),
        "folder-lock-open-outline" => Some(icons::FOLDER_LOCK_OPEN_OUTLINE),
        "tooltip-outline" => Some(icons::TOOLTIP_OUTLINE),
        "movie-edit" => Some(icons::MOVIE_EDIT),
        "dns-outline" => Some(icons::DNS_OUTLINE),
        #[allow(deprecated)]
        "language-ruby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUBY)
        }
        "pier-crane" => Some(icons::PIER_CRANE),
        "cards-spade-outline" => Some(icons::CARDS_SPADE_OUTLINE),
        "horse-variant-fast" => Some(icons::HORSE_VARIANT_FAST),
        "folder-network" => Some(icons::FOLDER_NETWORK),
        "battery-outline" => Some(icons::BATTERY_OUTLINE),
        "hand-back-left-outline" => Some(icons::HAND_BACK_LEFT_OUTLINE),
        "powershell" => Some(icons::POWERSHELL),
        "numeric-10-box-outline" => Some(icons::NUMERIC_10_BOX_OUTLINE),
        "fingerprint" => Some(icons::FINGERPRINT),
        "popcorn" => Some(icons::POPCORN),
        "calendar-collapse-horizontal-outline" => Some(icons::CALENDAR_COLLAPSE_HORIZONTAL_OUTLINE),
        "battery-charging-40" => Some(icons::BATTERY_CHARGING_40),
        "invoice-text-arrow-left-outline" => Some(icons::INVOICE_TEXT_ARROW_LEFT_OUTLINE),
        "cookie-cog" => Some(icons::COOKIE_COG),
        "bookmark-check" => Some(icons::BOOKMARK_CHECK),
        "folder" => Some(icons::FOLDER),
        "network-off" => Some(icons::NETWORK_OFF),
        "bed-clock" => Some(icons::BED_CLOCK),
        "math-integral-box" => Some(icons::MATH_INTEGRAL_BOX),
        "asterisk" => Some(icons::ASTERISK),
        "train-variant" => Some(icons::TRAIN_VARIANT),
        "deathly-hallows" => Some(icons::DEATHLY_HALLOWS),
        "ruler-square" => Some(icons::RULER_SQUARE),
        "application-brackets-outline" => Some(icons::APPLICATION_BRACKETS_OUTLINE),
        "database-arrow-up" => Some(icons::DATABASE_ARROW_UP),
        "database-plus" => Some(icons::DATABASE_PLUS),
        "file-find" => Some(icons::FILE_FIND),
        "octagram" => Some(icons::OCTAGRAM),
        "cellphone-arrow-down" => Some(icons::CELLPHONE_ARROW_DOWN),
        #[allow(deprecated)]
        "hulu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'hulu' is deprecated.").print(py);
            }
            Some(icons::HULU)
        }
        "alpha-i-box" => Some(icons::ALPHA_I_BOX),
        "calendar-heart-outline" => Some(icons::CALENDAR_HEART_OUTLINE),
        "sort-bool-descending" => Some(icons::SORT_BOOL_DESCENDING),
        "toy-brick-search-outline" => Some(icons::TOY_BRICK_SEARCH_OUTLINE),
        "lock-open" => Some(icons::LOCK_OPEN),
        "dns" => Some(icons::DNS),
        "ticket" => Some(icons::TICKET),
        "battery-80-bluetooth" => Some(icons::BATTERY_80_BLUETOOTH),
        "fireplace-off" => Some(icons::FIREPLACE_OFF),
        "comment-bookmark-outline" => Some(icons::COMMENT_BOOKMARK_OUTLINE),
        "lock-minus-outline" => Some(icons::LOCK_MINUS_OUTLINE),
        "vector-bezier" => Some(icons::VECTOR_BEZIER),
        "printer-pos-off-outline" => Some(icons::PRINTER_POS_OFF_OUTLINE),
        "lock-outline" => Some(icons::LOCK_OUTLINE),
        "timeline-minus" => Some(icons::TIMELINE_MINUS),
        "egg-outline" => Some(icons::EGG_OUTLINE),
        "chart-multiple" => Some(icons::CHART_MULTIPLE),
        "wrench-clock-outline" => Some(icons::WRENCH_CLOCK_OUTLINE),
        "file-image" => Some(icons::FILE_IMAGE),
        "boom-gate-up-outline" => Some(icons::BOOM_GATE_UP_OUTLINE),
        "alert-rhombus" => Some(icons::ALERT_RHOMBUS),
        "cannabis" => Some(icons::CANNABIS),
        "table-search" => Some(icons::TABLE_SEARCH),
        "clock-time-nine" => Some(icons::CLOCK_TIME_NINE),
        "table-column-remove" => Some(icons::TABLE_COLUMN_REMOVE),
        #[allow(deprecated)]
        "apple-finder" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-finder' is deprecated.").print(py);
            }
            Some(icons::APPLE_FINDER)
        }
        "shield-sync" => Some(icons::SHIELD_SYNC),
        "diamond" => Some(icons::DIAMOND),
        "collapse-all-outline" => Some(icons::COLLAPSE_ALL_OUTLINE),
        "ambulance" => Some(icons::AMBULANCE),
        "power-sleep" => Some(icons::POWER_SLEEP),
        "weather-lightning" => Some(icons::WEATHER_LIGHTNING),
        "checkbox-blank-outline" => Some(icons::CHECKBOX_BLANK_OUTLINE),
        "purse-outline" => Some(icons::PURSE_OUTLINE),
        "recycle" => Some(icons::RECYCLE),
        "microwave-off" => Some(icons::MICROWAVE_OFF),
        #[allow(deprecated)]
        "home-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'home-assistant' is deprecated.").print(py);
            }
            Some(icons::HOME_ASSISTANT)
        }
        "filter-settings" => Some(icons::FILTER_SETTINGS),
        "brush" => Some(icons::BRUSH),
        "boom-gate-arrow-down" => Some(icons::BOOM_GATE_ARROW_DOWN),
        "alpha-u-box-outline" => Some(icons::ALPHA_U_BOX_OUTLINE),
        "calendar-blank" => Some(icons::CALENDAR_BLANK),
        "post-lamp" => Some(icons::POST_LAMP),
        "file-table-outline" => Some(icons::FILE_TABLE_OUTLINE),
        "printer-pos-pause-outline" => Some(icons::PRINTER_POS_PAUSE_OUTLINE),
        "key-variant" => Some(icons::KEY_VARIANT),
        "invoice-edit" => Some(icons::INVOICE_EDIT),
        "clover" => Some(icons::CLOVER),
        "printer-pos-network" => Some(icons::PRINTER_POS_NETWORK),
        "clipboard-search-outline" => Some(icons::CLIPBOARD_SEARCH_OUTLINE),
        "robot-dead" => Some(icons::ROBOT_DEAD),
        "bridge" => Some(icons::BRIDGE),
        "format-annotation-minus" => Some(icons::FORMAT_ANNOTATION_MINUS),
        "format-text" => Some(icons::FORMAT_TEXT),
        "cards-outline" => Some(icons::CARDS_OUTLINE),
        "battery-10" => Some(icons::BATTERY_10),
        "plus-network" => Some(icons::PLUS_NETWORK),
        "generator-portable" => Some(icons::GENERATOR_PORTABLE),
        "contactless-payment-circle" => Some(icons::CONTACTLESS_PAYMENT_CIRCLE),
        "alpha-m-box-outline" => Some(icons::ALPHA_M_BOX_OUTLINE),
        "format-clear" => Some(icons::FORMAT_CLEAR),
        "ceiling-fan" => Some(icons::CEILING_FAN),
        "gender-male" => Some(icons::GENDER_MALE),
        "format-size" => Some(icons::FORMAT_SIZE),
        "human-male" => Some(icons::HUMAN_MALE),
        "message-arrow-right-outline" => Some(icons::MESSAGE_ARROW_RIGHT_OUTLINE),
        "view-grid" => Some(icons::VIEW_GRID),
        "basket-check-outline" => Some(icons::BASKET_CHECK_OUTLINE),
        "keyboard-f8" => Some(icons::KEYBOARD_F8),
        "arrow-up-thin-circle-outline" => Some(icons::ARROW_UP_THIN_CIRCLE_OUTLINE),
        "charity-search" => Some(icons::CHARITY_SEARCH),
        "car-wash" => Some(icons::CAR_WASH),
        "dice-d4" => Some(icons::DICE_D4),
        "rivet" => Some(icons::RIVET),
        "map-marker-distance" => Some(icons::MAP_MARKER_DISTANCE),
        "sort-numeric-descending-variant" => Some(icons::SORT_NUMERIC_DESCENDING_VARIANT),
        "comment-check" => Some(icons::COMMENT_CHECK),
        "thermometer-high" => Some(icons::THERMOMETER_HIGH),
        "rewind-60" => Some(icons::REWIND_60),
        "pan-top-right" => Some(icons::PAN_TOP_RIGHT),
        "video-input-hdmi" => Some(icons::VIDEO_INPUT_HDMI),
        "calendar-end-outline" => Some(icons::CALENDAR_END_OUTLINE),
        "garage" => Some(icons::GARAGE),
        "virus-off" => Some(icons::VIRUS_OFF),
        "briefcase-account-outline" => Some(icons::BRIEFCASE_ACCOUNT_OUTLINE),
        "flask-round-bottom-empty" => Some(icons::FLASK_ROUND_BOTTOM_EMPTY),
        "caravan" => Some(icons::CARAVAN),
        #[allow(deprecated)]
        "ubisoft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ubisoft' is deprecated.").print(py);
            }
            Some(icons::UBISOFT)
        }
        "filter-minus" => Some(icons::FILTER_MINUS),
        "fuse-off" => Some(icons::FUSE_OFF),
        "lightbulb" => Some(icons::LIGHTBULB),
        "camera-lock-outline" => Some(icons::CAMERA_LOCK_OUTLINE),
        "database-edit-outline" => Some(icons::DATABASE_EDIT_OUTLINE),
        "shield-off-outline" => Some(icons::SHIELD_OFF_OUTLINE),
        "speaker-off" => Some(icons::SPEAKER_OFF),
        "store-outline" => Some(icons::STORE_OUTLINE),
        "book-information-variant" => Some(icons::BOOK_INFORMATION_VARIANT),
        "mouse-move-down" => Some(icons::MOUSE_MOVE_DOWN),
        "copyright" => Some(icons::COPYRIGHT),
        "home-alert-outline" => Some(icons::HOME_ALERT_OUTLINE),
        _ => None,
    }
}
