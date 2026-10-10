// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_18(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "alarm-off" => Some(icons::ALARM_OFF),
        "chart-line-stacked" => Some(icons::CHART_LINE_STACKED),
        "delete-off" => Some(icons::DELETE_OFF),
        "key-chain-variant" => Some(icons::KEY_CHAIN_VARIANT),
        #[allow(deprecated)]
        "vimeo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vimeo' is deprecated.").print(py);
            }
            Some(icons::VIMEO)
        }
        "pipe" => Some(icons::PIPE),
        "clipboard-text-search-outline" => Some(icons::CLIPBOARD_TEXT_SEARCH_OUTLINE),
        "seat-outline" => Some(icons::SEAT_OUTLINE),
        "train-car" => Some(icons::TRAIN_CAR),
        "axis-y-arrow-lock" => Some(icons::AXIS_Y_ARROW_LOCK),
        "stadium-outline" => Some(icons::STADIUM_OUTLINE),
        "comment-search" => Some(icons::COMMENT_SEARCH),
        #[allow(deprecated)]
        "language-css3" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-css3' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CSS3)
        }
        "code-less-than" => Some(icons::CODE_LESS_THAN),
        "cow-off" => Some(icons::COW_OFF),
        "selection-ellipse-remove" => Some(icons::SELECTION_ELLIPSE_REMOVE),
        "credit-card-outline" => Some(icons::CREDIT_CARD_OUTLINE),
        "progress-upload" => Some(icons::PROGRESS_UPLOAD),
        "head-cog-outline" => Some(icons::HEAD_COG_OUTLINE),
        "weather-windy-variant" => Some(icons::WEATHER_WINDY_VARIANT),
        "receipt-text" => Some(icons::RECEIPT_TEXT),
        #[allow(deprecated)]
        "google-podcast" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-podcast' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PODCAST)
        }
        "heart-search" => Some(icons::HEART_SEARCH),
        "phone-log-outline" => Some(icons::PHONE_LOG_OUTLINE),
        "alarm-light-off" => Some(icons::ALARM_LIGHT_OFF),
        "email-plus" => Some(icons::EMAIL_PLUS),
        "fan-plus" => Some(icons::FAN_PLUS),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-full" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-full' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_FULL)
        }
        "briefcase-variant-off-outline" => Some(icons::BRIEFCASE_VARIANT_OFF_OUTLINE),
        "flask-empty-off" => Some(icons::FLASK_EMPTY_OFF),
        "emoticon-cry-outline" => Some(icons::EMOTICON_CRY_OUTLINE),
        "alert-minus-outline" => Some(icons::ALERT_MINUS_OUTLINE),
        "merge" => Some(icons::MERGE),
        "not-equal" => Some(icons::NOT_EQUAL),
        "undo-variant" => Some(icons::UNDO_VARIANT),
        "cookie-check" => Some(icons::COOKIE_CHECK),
        "gesture-two-double-tap" => Some(icons::GESTURE_TWO_DOUBLE_TAP),
        "file-clock" => Some(icons::FILE_CLOCK),
        "floppy" => Some(icons::FLOPPY),
        "microphone-minus" => Some(icons::MICROPHONE_MINUS),
        "menu-swap" => Some(icons::MENU_SWAP),
        "alert-decagram-outline" => Some(icons::ALERT_DECAGRAM_OUTLINE),
        "hazard-lights" => Some(icons::HAZARD_LIGHTS),
        "keyboard-variant" => Some(icons::KEYBOARD_VARIANT),
        "calendar-lock-outline" => Some(icons::CALENDAR_LOCK_OUTLINE),
        "lamps" => Some(icons::LAMPS),
        "comment-minus" => Some(icons::COMMENT_MINUS),
        "printer-pos" => Some(icons::PRINTER_POS),
        "table-star" => Some(icons::TABLE_STAR),
        "battery-low" => Some(icons::BATTERY_LOW),
        "ip-network" => Some(icons::IP_NETWORK),
        "silverware-variant" => Some(icons::SILVERWARE_VARIANT),
        "tree" => Some(icons::TREE),
        "weather-sunset-up" => Some(icons::WEATHER_SUNSET_UP),
        "selection-multiple-marker" => Some(icons::SELECTION_MULTIPLE_MARKER),
        "battery-60" => Some(icons::BATTERY_60),
        "format-header-equal" => Some(icons::FORMAT_HEADER_EQUAL),
        "power-off" => Some(icons::POWER_OFF),
        "rocket" => Some(icons::ROCKET),
        "size-m" => Some(icons::SIZE_M),
        "file-sync" => Some(icons::FILE_SYNC),
        "kabaddi" => Some(icons::KABADDI),
        "comment-arrow-left-outline" => Some(icons::COMMENT_ARROW_LEFT_OUTLINE),
        "collapse-all" => Some(icons::COLLAPSE_ALL),
        "puzzle-outline" => Some(icons::PUZZLE_OUTLINE),
        "chair-school" => Some(icons::CHAIR_SCHOOL),
        "timer-10" => Some(icons::TIMER_10),
        "invoice-text-send" => Some(icons::INVOICE_TEXT_SEND),
        "format-color-marker-cancel" => Some(icons::FORMAT_COLOR_MARKER_CANCEL),
        "newspaper-variant-outline" => Some(icons::NEWSPAPER_VARIANT_OUTLINE),
        "clipboard-account" => Some(icons::CLIPBOARD_ACCOUNT),
        "pencil-lock" => Some(icons::PENCIL_LOCK),
        "file-certificate" => Some(icons::FILE_CERTIFICATE),
        "folder-question-outline" => Some(icons::FOLDER_QUESTION_OUTLINE),
        "music-accidental-flat" => Some(icons::MUSIC_ACCIDENTAL_FLAT),
        "calendar-arrow-right" => Some(icons::CALENDAR_ARROW_RIGHT),
        "arrow-right-bottom-bold" => Some(icons::ARROW_RIGHT_BOTTOM_BOLD),
        "map-marker-alert-outline" => Some(icons::MAP_MARKER_ALERT_OUTLINE),
        "tune-variant" => Some(icons::TUNE_VARIANT),
        "store-edit-outline" => Some(icons::STORE_EDIT_OUTLINE),
        "chart-scatter-plot" => Some(icons::CHART_SCATTER_PLOT),
        "circle-slice-7" => Some(icons::CIRCLE_SLICE_7),
        "folder-open-outline" => Some(icons::FOLDER_OPEN_OUTLINE),
        "book-lock-outline" => Some(icons::BOOK_LOCK_OUTLINE),
        "alpha-g-box" => Some(icons::ALPHA_G_BOX),
        "content-save-edit" => Some(icons::CONTENT_SAVE_EDIT),
        "folder-cog-outline" => Some(icons::FOLDER_COG_OUTLINE),
        "pipe-wrench" => Some(icons::PIPE_WRENCH),
        "format-list-checks" => Some(icons::FORMAT_LIST_CHECKS),
        #[allow(deprecated)]
        "pandora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pandora' is deprecated.").print(py);
            }
            Some(icons::PANDORA)
        }
        "folder" => Some(icons::FOLDER),
        "clock-time-two-outline" => Some(icons::CLOCK_TIME_TWO_OUTLINE),
        "ballot-recount-outline" => Some(icons::BALLOT_RECOUNT_OUTLINE),
        "rounded-corner" => Some(icons::ROUNDED_CORNER),
        "panorama-outline" => Some(icons::PANORAMA_OUTLINE),
        #[allow(deprecated)]
        "language-ruby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUBY)
        }
        "square-rounded" => Some(icons::SQUARE_ROUNDED),
        "fuel-cell" => Some(icons::FUEL_CELL),
        "book-settings" => Some(icons::BOOK_SETTINGS),
        "office-building-plus-outline" => Some(icons::OFFICE_BUILDING_PLUS_OUTLINE),
        "registered-trademark" => Some(icons::REGISTERED_TRADEMARK),
        "heart-flash" => Some(icons::HEART_FLASH),
        "plus-minus" => Some(icons::PLUS_MINUS),
        "alpha-c-box-outline" => Some(icons::ALPHA_C_BOX_OUTLINE),
        "glass-cocktail-off" => Some(icons::GLASS_COCKTAIL_OFF),
        "cellphone-link" => Some(icons::CELLPHONE_LINK),
        "weather-moonset-up" => Some(icons::WEATHER_MOONSET_UP),
        "format-pilcrow" => Some(icons::FORMAT_PILCROW),
        "emoticon-poop-outline" => Some(icons::EMOTICON_POOP_OUTLINE),
        "robot-excited" => Some(icons::ROBOT_EXCITED),
        "sticker-emoji" => Some(icons::STICKER_EMOJI),
        "widgets" => Some(icons::WIDGETS),
        "upload-circle" => Some(icons::UPLOAD_CIRCLE),
        "printer-3d-nozzle-alert" => Some(icons::PRINTER_3D_NOZZLE_ALERT),
        "progress-alert" => Some(icons::PROGRESS_ALERT),
        "file-star-four-points" => Some(icons::FILE_STAR_FOUR_POINTS),
        "tshirt-crew-outline" => Some(icons::TSHIRT_CREW_OUTLINE),
        "alphabet-cyrillic" => Some(icons::ALPHABET_CYRILLIC),
        "shield-key-outline" => Some(icons::SHIELD_KEY_OUTLINE),
        "shield-lock-open" => Some(icons::SHIELD_LOCK_OPEN),
        "upload-off" => Some(icons::UPLOAD_OFF),
        "strategy" => Some(icons::STRATEGY),
        "roman-numeral-7" => Some(icons::ROMAN_NUMERAL_7),
        "emoticon-happy-outline" => Some(icons::EMOTICON_HAPPY_OUTLINE),
        "tag-multiple" => Some(icons::TAG_MULTIPLE),
        "chart-bell-curve-cumulative" => Some(icons::CHART_BELL_CURVE_CUMULATIVE),
        "battery-charging-wireless-50" => Some(icons::BATTERY_CHARGING_WIRELESS_50),
        "cube-off" => Some(icons::CUBE_OFF),
        "format-list-bulleted-triangle" => Some(icons::FORMAT_LIST_BULLETED_TRIANGLE),
        "projector-screen-variant-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OUTLINE),
        "sprout-outline" => Some(icons::SPROUT_OUTLINE),
        #[allow(deprecated)]
        "apple-finder" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-finder' is deprecated.").print(py);
            }
            Some(icons::APPLE_FINDER)
        }
        "filter-cog" => Some(icons::FILTER_COG),
        "saw-blade" => Some(icons::SAW_BLADE),
        "book-plus-multiple" => Some(icons::BOOK_PLUS_MULTIPLE),
        "account-convert" => Some(icons::ACCOUNT_CONVERT),
        "package-variant-closed-plus" => Some(icons::PACKAGE_VARIANT_CLOSED_PLUS),
        "certificate" => Some(icons::CERTIFICATE),
        "dock-right" => Some(icons::DOCK_RIGHT),
        "flask-minus-outline" => Some(icons::FLASK_MINUS_OUTLINE),
        "speaker-pause" => Some(icons::SPEAKER_PAUSE),
        "umbrella-beach-outline" => Some(icons::UMBRELLA_BEACH_OUTLINE),
        "screw-machine-flat-top" => Some(icons::SCREW_MACHINE_FLAT_TOP),
        "key-star" => Some(icons::KEY_STAR),
        "note" => Some(icons::NOTE),
        "archive-clock-outline" => Some(icons::ARCHIVE_CLOCK_OUTLINE),
        "volume-off" => Some(icons::VOLUME_OFF),
        "wifi-arrow-left-right" => Some(icons::WIFI_ARROW_LEFT_RIGHT),
        "gauge-low" => Some(icons::GAUGE_LOW),
        "mouse-move-up" => Some(icons::MOUSE_MOVE_UP),
        "battery" => Some(icons::BATTERY),
        "information-off-outline" => Some(icons::INFORMATION_OFF_OUTLINE),
        "weather-hazy" => Some(icons::WEATHER_HAZY),
        "file-arrow-up-down" => Some(icons::FILE_ARROW_UP_DOWN),
        "food-croissant" => Some(icons::FOOD_CROISSANT),
        "power-plug" => Some(icons::POWER_PLUG),
        "file-phone-outline" => Some(icons::FILE_PHONE_OUTLINE),
        "align-horizontal-left" => Some(icons::ALIGN_HORIZONTAL_LEFT),
        "filter-minus" => Some(icons::FILTER_MINUS),
        "draw" => Some(icons::DRAW),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "wifi-strength-off-outline" => Some(icons::WIFI_STRENGTH_OFF_OUTLINE),
        "alarm-light-off-outline" => Some(icons::ALARM_LIGHT_OFF_OUTLINE),
        "archive-lock-outline" => Some(icons::ARCHIVE_LOCK_OUTLINE),
        "cellphone-information" => Some(icons::CELLPHONE_INFORMATION),
        "bottle-tonic-plus-outline" => Some(icons::BOTTLE_TONIC_PLUS_OUTLINE),
        "text-long" => Some(icons::TEXT_LONG),
        "fire-hydrant-alert" => Some(icons::FIRE_HYDRANT_ALERT),
        "timer-play-outline" => Some(icons::TIMER_PLAY_OUTLINE),
        "fire-off" => Some(icons::FIRE_OFF),
        "message-arrow-right-outline" => Some(icons::MESSAGE_ARROW_RIGHT_OUTLINE),
        "set-square" => Some(icons::SET_SQUARE),
        "movie-open-plus" => Some(icons::MOVIE_OPEN_PLUS),
        "inbox-outline" => Some(icons::INBOX_OUTLINE),
        "tag-heart" => Some(icons::TAG_HEART),
        "harddisk" => Some(icons::HARDDISK),
        "cards-playing-spade-outline" => Some(icons::CARDS_PLAYING_SPADE_OUTLINE),
        "cogs" => Some(icons::COGS),
        "sticker-alert" => Some(icons::STICKER_ALERT),
        "movie-open-outline" => Some(icons::MOVIE_OPEN_OUTLINE),
        "phone-refresh" => Some(icons::PHONE_REFRESH),
        "storefront-edit-outline" => Some(icons::STOREFRONT_EDIT_OUTLINE),
        "file-alert" => Some(icons::FILE_ALERT),
        "forest-outline" => Some(icons::FOREST_OUTLINE),
        "sort-alphabetical-descending" => Some(icons::SORT_ALPHABETICAL_DESCENDING),
        "arrow-left-bold-outline" => Some(icons::ARROW_LEFT_BOLD_OUTLINE),
        "clock-check" => Some(icons::CLOCK_CHECK),
        "exclamation" => Some(icons::EXCLAMATION),
        "clipboard-flow" => Some(icons::CLIPBOARD_FLOW),
        "robot-vacuum-variant-off" => Some(icons::ROBOT_VACUUM_VARIANT_OFF),
        "fan-clock" => Some(icons::FAN_CLOCK),
        "card-account-details-outline" => Some(icons::CARD_ACCOUNT_DETAILS_OUTLINE),
        "robot-vacuum" => Some(icons::ROBOT_VACUUM),
        "radar" => Some(icons::RADAR),
        "propane-tank-outline" => Some(icons::PROPANE_TANK_OUTLINE),
        "star-four-points-circle-outline" => Some(icons::STAR_FOUR_POINTS_CIRCLE_OUTLINE),
        "format-text-variant-outline" => Some(icons::FORMAT_TEXT_VARIANT_OUTLINE),
        "account-network-off" => Some(icons::ACCOUNT_NETWORK_OFF),
        "database-check-outline" => Some(icons::DATABASE_CHECK_OUTLINE),
        "motorbike-electric" => Some(icons::MOTORBIKE_ELECTRIC),
        _ => None,
    }
}
