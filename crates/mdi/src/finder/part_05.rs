// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_5(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "vector-ellipse" => Some(icons::VECTOR_ELLIPSE),
        "tooth-outline" => Some(icons::TOOTH_OUTLINE),
        "arrow-expand-down" => Some(icons::ARROW_EXPAND_DOWN),
        "window-shutter-open" => Some(icons::WINDOW_SHUTTER_OPEN),
        "map-marker-left" => Some(icons::MAP_MARKER_LEFT),
        "credit-card-remove" => Some(icons::CREDIT_CARD_REMOVE),
        "calendar-cursor" => Some(icons::CALENDAR_CURSOR),
        "mouse-variant" => Some(icons::MOUSE_VARIANT),
        "email-open-outline" => Some(icons::EMAIL_OPEN_OUTLINE),
        "database-marker-outline" => Some(icons::DATABASE_MARKER_OUTLINE),
        "comment-search" => Some(icons::COMMENT_SEARCH),
        "magnify-remove-outline" => Some(icons::MAGNIFY_REMOVE_OUTLINE),
        "play-network-outline" => Some(icons::PLAY_NETWORK_OUTLINE),
        "qrcode-plus" => Some(icons::QRCODE_PLUS),
        "email" => Some(icons::EMAIL),
        #[allow(deprecated)]
        "patreon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'patreon' is deprecated.").print(py);
            }
            Some(icons::PATREON)
        }
        "spade" => Some(icons::SPADE),
        #[allow(deprecated)]
        "aws" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'aws' is deprecated.").print(py);
            }
            Some(icons::AWS)
        }
        "application-settings-outline" => Some(icons::APPLICATION_SETTINGS_OUTLINE),
        "seat-individual-suite" => Some(icons::SEAT_INDIVIDUAL_SUITE),
        "record" => Some(icons::RECORD),
        "table-check" => Some(icons::TABLE_CHECK),
        "weather-moonset" => Some(icons::WEATHER_MOONSET),
        "relation-one-or-many-to-many" => Some(icons::RELATION_ONE_OR_MANY_TO_MANY),
        #[allow(deprecated)]
        "linkedin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linkedin' is deprecated.").print(py);
            }
            Some(icons::LINKEDIN)
        }
        "dumbbell" => Some(icons::DUMBBELL),
        "dice-5-outline" => Some(icons::DICE_5_OUTLINE),
        "eraser-variant" => Some(icons::ERASER_VARIANT),
        "bowl-outline" => Some(icons::BOWL_OUTLINE),
        "book-open-variant-outline" => Some(icons::BOOK_OPEN_VARIANT_OUTLINE),
        "move-resize" => Some(icons::MOVE_RESIZE),
        "football-helmet" => Some(icons::FOOTBALL_HELMET),
        "group" => Some(icons::GROUP),
        "map" => Some(icons::MAP),
        "smoke-detector-alert" => Some(icons::SMOKE_DETECTOR_ALERT),
        "pail-minus-outline" => Some(icons::PAIL_MINUS_OUTLINE),
        "window-open-variant" => Some(icons::WINDOW_OPEN_VARIANT),
        "smart-card-reader" => Some(icons::SMART_CARD_READER),
        "magic-staff" => Some(icons::MAGIC_STAFF),
        #[allow(deprecated)]
        "github" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'github' is deprecated.").print(py);
            }
            Some(icons::GITHUB)
        }
        "brush-off" => Some(icons::BRUSH_OFF),
        "ev-plug-ccs1" => Some(icons::EV_PLUG_CCS1),
        "delete-forever" => Some(icons::DELETE_FOREVER),
        "vector-polyline-edit" => Some(icons::VECTOR_POLYLINE_EDIT),
        "triforce" => Some(icons::TRIFORCE),
        "lock-open-alert" => Some(icons::LOCK_OPEN_ALERT),
        "grave-stone" => Some(icons::GRAVE_STONE),
        "flask-empty-plus-outline" => Some(icons::FLASK_EMPTY_PLUS_OUTLINE),
        "bench-back" => Some(icons::BENCH_BACK),
        "dice-d10-outline" => Some(icons::DICE_D10_OUTLINE),
        "account-filter-outline" => Some(icons::ACCOUNT_FILTER_OUTLINE),
        "account-settings" => Some(icons::ACCOUNT_SETTINGS),
        "arrange-send-to-back" => Some(icons::ARRANGE_SEND_TO_BACK),
        "information-off" => Some(icons::INFORMATION_OFF),
        "tram-side" => Some(icons::TRAM_SIDE),
        "filter-off" => Some(icons::FILTER_OFF),
        "book-search" => Some(icons::BOOK_SEARCH),
        "alpha-h-box-outline" => Some(icons::ALPHA_H_BOX_OUTLINE),
        "video-input-svideo" => Some(icons::VIDEO_INPUT_SVIDEO),
        "file-eye" => Some(icons::FILE_EYE),
        "gender-male" => Some(icons::GENDER_MALE),
        "gamepad-left" => Some(icons::GAMEPAD_LEFT),
        "numeric-1-circle-outline" => Some(icons::NUMERIC_1_CIRCLE_OUTLINE),
        "cog-sync-outline" => Some(icons::COG_SYNC_OUTLINE),
        "flower-poppy" => Some(icons::FLOWER_POPPY),
        "book-play-outline" => Some(icons::BOOK_PLAY_OUTLINE),
        "led-variant-off" => Some(icons::LED_VARIANT_OFF),
        "video-plus-outline" => Some(icons::VIDEO_PLUS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-access" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-access' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ACCESS)
        }
        "paperclip-remove" => Some(icons::PAPERCLIP_REMOVE),
        "basket-off-outline" => Some(icons::BASKET_OFF_OUTLINE),
        "backspace-reverse-outline" => Some(icons::BACKSPACE_REVERSE_OUTLINE),
        "text-box-minus-outline" => Some(icons::TEXT_BOX_MINUS_OUTLINE),
        "racquetball" => Some(icons::RACQUETBALL),
        "car-wrench" => Some(icons::CAR_WRENCH),
        "folder-arrow-down-outline" => Some(icons::FOLDER_ARROW_DOWN_OUTLINE),
        "account-school" => Some(icons::ACCOUNT_SCHOOL),
        "comment-edit" => Some(icons::COMMENT_EDIT),
        "vote" => Some(icons::VOTE),
        "vector-rectangle" => Some(icons::VECTOR_RECTANGLE),
        "cloud-clock-outline" => Some(icons::CLOUD_CLOCK_OUTLINE),
        "check-all" => Some(icons::CHECK_ALL),
        "timeline-check" => Some(icons::TIMELINE_CHECK),
        "rhombus-medium" => Some(icons::RHOMBUS_MEDIUM),
        "fire-station" => Some(icons::FIRE_STATION),
        "emoticon-frown" => Some(icons::EMOTICON_FROWN),
        "gamepad-round-right" => Some(icons::GAMEPAD_ROUND_RIGHT),
        "car-brake-fluid-level" => Some(icons::CAR_BRAKE_FLUID_LEVEL),
        "source-branch-minus" => Some(icons::SOURCE_BRANCH_MINUS),
        "code-brackets" => Some(icons::CODE_BRACKETS),
        "state-machine" => Some(icons::STATE_MACHINE),
        "water-pump" => Some(icons::WATER_PUMP),
        "battery-30-bluetooth" => Some(icons::BATTERY_30_BLUETOOTH),
        "car" => Some(icons::CAR),
        "movie-open" => Some(icons::MOVIE_OPEN),
        "hoop-house" => Some(icons::HOOP_HOUSE),
        "pen-minus" => Some(icons::PEN_MINUS),
        "screw-flat-top" => Some(icons::SCREW_FLAT_TOP),
        "search-web" => Some(icons::SEARCH_WEB),
        "table-arrow-right" => Some(icons::TABLE_ARROW_RIGHT),
        "arrow-left-drop-circle-outline" => Some(icons::ARROW_LEFT_DROP_CIRCLE_OUTLINE),
        "fence-electric" => Some(icons::FENCE_ELECTRIC),
        "perspective-less" => Some(icons::PERSPECTIVE_LESS),
        "numeric-3-box-outline" => Some(icons::NUMERIC_3_BOX_OUTLINE),
        #[allow(deprecated)]
        "language-python" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-python' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_PYTHON)
        }
        "format-section" => Some(icons::FORMAT_SECTION),
        "cash-remove" => Some(icons::CASH_REMOVE),
        "clock-minus-outline" => Some(icons::CLOCK_MINUS_OUTLINE),
        "earth-box-off" => Some(icons::EARTH_BOX_OFF),
        "arrow-left-bottom" => Some(icons::ARROW_LEFT_BOTTOM),
        "home-lock" => Some(icons::HOME_LOCK),
        "chart-donut" => Some(icons::CHART_DONUT),
        "music-box-outline" => Some(icons::MUSIC_BOX_OUTLINE),
        "plus-thick" => Some(icons::PLUS_THICK),
        "passport" => Some(icons::PASSPORT),
        "trophy" => Some(icons::TROPHY),
        "database-search-outline" => Some(icons::DATABASE_SEARCH_OUTLINE),
        "gesture-swipe-vertical" => Some(icons::GESTURE_SWIPE_VERTICAL),
        "numeric-9-box-outline" => Some(icons::NUMERIC_9_BOX_OUTLINE),
        "margin" => Some(icons::MARGIN),
        "tennis-ball-outline" => Some(icons::TENNIS_BALL_OUTLINE),
        "keyboard-f7" => Some(icons::KEYBOARD_F7),
        "progress-close" => Some(icons::PROGRESS_CLOSE),
        "battery-arrow-down-outline" => Some(icons::BATTERY_ARROW_DOWN_OUTLINE),
        #[allow(deprecated)]
        "facebook-workplace" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-workplace' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_WORKPLACE)
        }
        "database-remove" => Some(icons::DATABASE_REMOVE),
        "battery-charging-40" => Some(icons::BATTERY_CHARGING_40),
        "order-alphabetical-descending" => Some(icons::ORDER_ALPHABETICAL_DESCENDING),
        "check-circle-outline" => Some(icons::CHECK_CIRCLE_OUTLINE),
        "archive-arrow-down-outline" => Some(icons::ARCHIVE_ARROW_DOWN_OUTLINE),
        "train" => Some(icons::TRAIN),
        "weather-partly-snowy-rainy" => Some(icons::WEATHER_PARTLY_SNOWY_RAINY),
        "format-wrap-square" => Some(icons::FORMAT_WRAP_SQUARE),
        "skull-scan" => Some(icons::SKULL_SCAN),
        "briefcase-edit-outline" => Some(icons::BRIEFCASE_EDIT_OUTLINE),
        "rewind-15" => Some(icons::REWIND_15),
        "help-box-multiple" => Some(icons::HELP_BOX_MULTIPLE),
        "mortar-pestle" => Some(icons::MORTAR_PESTLE),
        "archive-off" => Some(icons::ARCHIVE_OFF),
        "check-network-outline" => Some(icons::CHECK_NETWORK_OUTLINE),
        "pause-circle-outline" => Some(icons::PAUSE_CIRCLE_OUTLINE),
        "arrow-up-thin-circle-outline" => Some(icons::ARROW_UP_THIN_CIRCLE_OUTLINE),
        "chart-box-multiple" => Some(icons::CHART_BOX_MULTIPLE),
        "chili-alert" => Some(icons::CHILI_ALERT),
        "numeric-8" => Some(icons::NUMERIC_8),
        "fire-off" => Some(icons::FIRE_OFF),
        "oar" => Some(icons::OAR),
        "timer-sand" => Some(icons::TIMER_SAND),
        "hat-fedora" => Some(icons::HAT_FEDORA),
        "clipboard-minus-outline" => Some(icons::CLIPBOARD_MINUS_OUTLINE),
        "folder-sync-outline" => Some(icons::FOLDER_SYNC_OUTLINE),
        "cart-minus" => Some(icons::CART_MINUS),
        "gamepad-right" => Some(icons::GAMEPAD_RIGHT),
        "jump-rope" => Some(icons::JUMP_ROPE),
        "thermometer-check" => Some(icons::THERMOMETER_CHECK),
        "account-search" => Some(icons::ACCOUNT_SEARCH),
        "octagram" => Some(icons::OCTAGRAM),
        "store-clock-outline" => Some(icons::STORE_CLOCK_OUTLINE),
        "database-eye" => Some(icons::DATABASE_EYE),
        "poll" => Some(icons::POLL),
        "sale-outline" => Some(icons::SALE_OUTLINE),
        "motorbike-off" => Some(icons::MOTORBIKE_OFF),
        "alarm-note" => Some(icons::ALARM_NOTE),
        "message-off-outline" => Some(icons::MESSAGE_OFF_OUTLINE),
        "archive-minus-outline" => Some(icons::ARCHIVE_MINUS_OUTLINE),
        "emoticon-confused-outline" => Some(icons::EMOTICON_CONFUSED_OUTLINE),
        "folder-file" => Some(icons::FOLDER_FILE),
        "tab-minus" => Some(icons::TAB_MINUS),
        #[allow(deprecated)]
        "pi-hole" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pi-hole' is deprecated.").print(py);
            }
            Some(icons::PI_HOLE)
        }
        "high-definition" => Some(icons::HIGH_DEFINITION),
        "car-shift-pattern" => Some(icons::CAR_SHIFT_PATTERN),
        "movie-open-play" => Some(icons::MOVIE_OPEN_PLAY),
        "hexagon-slice-2" => Some(icons::HEXAGON_SLICE_2),
        "snowshoeing" => Some(icons::SNOWSHOEING),
        "seat-recline-extra" => Some(icons::SEAT_RECLINE_EXTRA),
        "selection-drag" => Some(icons::SELECTION_DRAG),
        "download-box-outline" => Some(icons::DOWNLOAD_BOX_OUTLINE),
        "flower" => Some(icons::FLOWER),
        "boom-gate-alert-outline" => Some(icons::BOOM_GATE_ALERT_OUTLINE),
        "saxophone" => Some(icons::SAXOPHONE),
        "music-note-quarter-dotted" => Some(icons::MUSIC_NOTE_QUARTER_DOTTED),
        "baby-carriage-off" => Some(icons::BABY_CARRIAGE_OFF),
        "target-variant" => Some(icons::TARGET_VARIANT),
        "panorama-outline" => Some(icons::PANORAMA_OUTLINE),
        "keyboard-f8" => Some(icons::KEYBOARD_F8),
        "source-branch-check" => Some(icons::SOURCE_BRANCH_CHECK),
        "arrow-expand-right" => Some(icons::ARROW_EXPAND_RIGHT),
        "book-open-page-variant-outline" => Some(icons::BOOK_OPEN_PAGE_VARIANT_OUTLINE),
        "table-account" => Some(icons::TABLE_ACCOUNT),
        "helicopter" => Some(icons::HELICOPTER),
        "list-status" => Some(icons::LIST_STATUS),
        "view-column" => Some(icons::VIEW_COLUMN),
        "pound-box-outline" => Some(icons::POUND_BOX_OUTLINE),
        "vhs" => Some(icons::VHS),
        "peanut-off-outline" => Some(icons::PEANUT_OFF_OUTLINE),
        "escalator-down" => Some(icons::ESCALATOR_DOWN),
        "piggy-bank-outline" => Some(icons::PIGGY_BANK_OUTLINE),
        "alpha-r-box-outline" => Some(icons::ALPHA_R_BOX_OUTLINE),
        "clover-outline" => Some(icons::CLOVER_OUTLINE),
        "note-minus" => Some(icons::NOTE_MINUS),
        _ => None,
    }
}
