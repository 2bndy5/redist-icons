// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_1(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "printer-pos-alert-outline" => Some(icons::PRINTER_POS_ALERT_OUTLINE),
        "arrow-bottom-left-bold-box-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_BOX_OUTLINE),
        "currency-rub" => Some(icons::CURRENCY_RUB),
        "football-helmet" => Some(icons::FOOTBALL_HELMET),
        "stove" => Some(icons::STOVE),
        "chart-box-plus-outline" => Some(icons::CHART_BOX_PLUS_OUTLINE),
        "comment-remove" => Some(icons::COMMENT_REMOVE),
        "city-variant" => Some(icons::CITY_VARIANT),
        "numeric-3" => Some(icons::NUMERIC_3),
        "trophy-variant" => Some(icons::TROPHY_VARIANT),
        "bag-checked" => Some(icons::BAG_CHECKED),
        "video-account" => Some(icons::VIDEO_ACCOUNT),
        "book-alert" => Some(icons::BOOK_ALERT),
        "folder-play-outline" => Some(icons::FOLDER_PLAY_OUTLINE),
        #[allow(deprecated)]
        "vuejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuejs' is deprecated.").print(py);
            }
            Some(icons::VUEJS)
        }
        "math-tan" => Some(icons::MATH_TAN),
        "code-not-equal-variant" => Some(icons::CODE_NOT_EQUAL_VARIANT),
        "clock-time-six-outline" => Some(icons::CLOCK_TIME_SIX_OUTLINE),
        "receipt-text-clock" => Some(icons::RECEIPT_TEXT_CLOCK),
        "bank-off" => Some(icons::BANK_OFF),
        "file-tree" => Some(icons::FILE_TREE),
        "download-network-outline" => Some(icons::DOWNLOAD_NETWORK_OUTLINE),
        "pan-vertical" => Some(icons::PAN_VERTICAL),
        "bookmark-music-outline" => Some(icons::BOOKMARK_MUSIC_OUTLINE),
        "select-off" => Some(icons::SELECT_OFF),
        #[allow(deprecated)]
        "firefox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firefox' is deprecated.").print(py);
            }
            Some(icons::FIREFOX)
        }
        "bat" => Some(icons::BAT),
        "lock-open-remove" => Some(icons::LOCK_OPEN_REMOVE),
        "flask-empty-plus-outline" => Some(icons::FLASK_EMPTY_PLUS_OUTLINE),
        "calendar-heart-outline" => Some(icons::CALENDAR_HEART_OUTLINE),
        "share-off-outline" => Some(icons::SHARE_OFF_OUTLINE),
        "database-arrow-left-outline" => Some(icons::DATABASE_ARROW_LEFT_OUTLINE),
        "clock-time-nine-outline" => Some(icons::CLOCK_TIME_NINE_OUTLINE),
        "note-edit" => Some(icons::NOTE_EDIT),
        "timer-plus" => Some(icons::TIMER_PLUS),
        "shape-plus-outline" => Some(icons::SHAPE_PLUS_OUTLINE),
        "mini-sd" => Some(icons::MINI_SD),
        #[allow(deprecated)]
        "vlc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vlc' is deprecated.").print(py);
            }
            Some(icons::VLC)
        }
        "printer-pos" => Some(icons::PRINTER_POS),
        "point-of-sale" => Some(icons::POINT_OF_SALE),
        "gamepad-round-right" => Some(icons::GAMEPAD_ROUND_RIGHT),
        "margin" => Some(icons::MARGIN),
        "panorama-horizontal" => Some(icons::PANORAMA_HORIZONTAL),
        "cookie-check-outline" => Some(icons::COOKIE_CHECK_OUTLINE),
        "calendar-weekend" => Some(icons::CALENDAR_WEEKEND),
        "lotion" => Some(icons::LOTION),
        "gesture-swipe-right" => Some(icons::GESTURE_SWIPE_RIGHT),
        "vector-selection" => Some(icons::VECTOR_SELECTION),
        "folder-home" => Some(icons::FOLDER_HOME),
        "clock-digital" => Some(icons::CLOCK_DIGITAL),
        "soccer-field" => Some(icons::SOCCER_FIELD),
        "message-bookmark" => Some(icons::MESSAGE_BOOKMARK),
        "smoke-detector-variant" => Some(icons::SMOKE_DETECTOR_VARIANT),
        "cog-transfer-outline" => Some(icons::COG_TRANSFER_OUTLINE),
        "movie-search" => Some(icons::MOVIE_SEARCH),
        "application-cog" => Some(icons::APPLICATION_COG),
        #[allow(deprecated)]
        "nuxt" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nuxt' is deprecated.").print(py);
            }
            Some(icons::NUXT)
        }
        "earth-plus" => Some(icons::EARTH_PLUS),
        "view-sequential-outline" => Some(icons::VIEW_SEQUENTIAL_OUTLINE),
        "forum-remove" => Some(icons::FORUM_REMOVE),
        "server-plus-outline" => Some(icons::SERVER_PLUS_OUTLINE),
        "receipt-clock" => Some(icons::RECEIPT_CLOCK),
        "numeric-1-box-outline" => Some(icons::NUMERIC_1_BOX_OUTLINE),
        "file-document-edit-outline" => Some(icons::FILE_DOCUMENT_EDIT_OUTLINE),
        "file-undo-outline" => Some(icons::FILE_UNDO_OUTLINE),
        "account-cog-outline" => Some(icons::ACCOUNT_COG_OUTLINE),
        "oil" => Some(icons::OIL),
        "download-outline" => Some(icons::DOWNLOAD_OUTLINE),
        "alpha-n" => Some(icons::ALPHA_N),
        "oil-lamp" => Some(icons::OIL_LAMP),
        "arrow-down-bold-box" => Some(icons::ARROW_DOWN_BOLD_BOX),
        "calendar-import" => Some(icons::CALENDAR_IMPORT),
        "archive-refresh" => Some(icons::ARCHIVE_REFRESH),
        "middleware" => Some(icons::MIDDLEWARE),
        "heart-half" => Some(icons::HEART_HALF),
        "water-pump" => Some(icons::WATER_PUMP),
        "bus-double-decker" => Some(icons::BUS_DOUBLE_DECKER),
        "betamax" => Some(icons::BETAMAX),
        "format-strikethrough" => Some(icons::FORMAT_STRIKETHROUGH),
        "box-cutter-off" => Some(icons::BOX_CUTTER_OFF),
        "notebook" => Some(icons::NOTEBOOK),
        "passport" => Some(icons::PASSPORT),
        "wrench-clock-outline" => Some(icons::WRENCH_CLOCK_OUTLINE),
        "arrow-down-bold-circle-outline" => Some(icons::ARROW_DOWN_BOLD_CIRCLE_OUTLINE),
        "id-card" => Some(icons::ID_CARD),
        "pencil-box-multiple" => Some(icons::PENCIL_BOX_MULTIPLE),
        "solid" => Some(icons::SOLID),
        "tablet-cellphone" => Some(icons::TABLET_CELLPHONE),
        #[allow(deprecated)]
        "docker" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'docker' is deprecated.").print(py);
            }
            Some(icons::DOCKER)
        }
        "clock-time-three" => Some(icons::CLOCK_TIME_THREE),
        "screw-round-top" => Some(icons::SCREW_ROUND_TOP),
        "source-commit" => Some(icons::SOURCE_COMMIT),
        "translate-off" => Some(icons::TRANSLATE_OFF),
        "archive-plus-outline" => Some(icons::ARCHIVE_PLUS_OUTLINE),
        "database-lock" => Some(icons::DATABASE_LOCK),
        "truck-plus" => Some(icons::TRUCK_PLUS),
        "download-multiple" => Some(icons::DOWNLOAD_MULTIPLE),
        "bookmark-minus-outline" => Some(icons::BOOKMARK_MINUS_OUTLINE),
        "bell-alert" => Some(icons::BELL_ALERT),
        "skip-backward-outline" => Some(icons::SKIP_BACKWARD_OUTLINE),
        "cloud-clock" => Some(icons::CLOUD_CLOCK),
        "pine-tree-variant-outline" => Some(icons::PINE_TREE_VARIANT_OUTLINE),
        "file-question-outline" => Some(icons::FILE_QUESTION_OUTLINE),
        "select-place" => Some(icons::SELECT_PLACE),
        "arrow-collapse-down" => Some(icons::ARROW_COLLAPSE_DOWN),
        "credit-card-check" => Some(icons::CREDIT_CARD_CHECK),
        "rewind-5" => Some(icons::REWIND_5),
        "bike-pedal-clipless" => Some(icons::BIKE_PEDAL_CLIPLESS),
        "cookie-cog-outline" => Some(icons::COOKIE_COG_OUTLINE),
        "cheese" => Some(icons::CHEESE),
        "iron" => Some(icons::IRON),
        "timer-pause-outline" => Some(icons::TIMER_PAUSE_OUTLINE),
        "lighthouse" => Some(icons::LIGHTHOUSE),
        "toggle-switch-off-outline" => Some(icons::TOGGLE_SWITCH_OFF_OUTLINE),
        "ray-vertex" => Some(icons::RAY_VERTEX),
        "progress-question" => Some(icons::PROGRESS_QUESTION),
        "cancel" => Some(icons::CANCEL),
        "train-car-passenger-door-open" => Some(icons::TRAIN_CAR_PASSENGER_DOOR_OPEN),
        #[allow(deprecated)]
        "language-swift" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-swift' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_SWIFT)
        }
        "window-closed" => Some(icons::WINDOW_CLOSED),
        "checkbox-marked-circle-plus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_PLUS_OUTLINE),
        "bio" => Some(icons::BIO),
        "emoticon" => Some(icons::EMOTICON),
        "cellphone-link-off" => Some(icons::CELLPHONE_LINK_OFF),
        "notebook-plus" => Some(icons::NOTEBOOK_PLUS),
        "raw" => Some(icons::RAW),
        "bank-transfer-in" => Some(icons::BANK_TRANSFER_IN),
        "paperclip-minus" => Some(icons::PAPERCLIP_MINUS),
        "earbuds-off-outline" => Some(icons::EARBUDS_OFF_OUTLINE),
        "comment-outline" => Some(icons::COMMENT_OUTLINE),
        "battery-70" => Some(icons::BATTERY_70),
        "paw-outline" => Some(icons::PAW_OUTLINE),
        "shield-check-outline" => Some(icons::SHIELD_CHECK_OUTLINE),
        "credit-card-edit" => Some(icons::CREDIT_CARD_EDIT),
        "oar" => Some(icons::OAR),
        "eyedropper-variant" => Some(icons::EYEDROPPER_VARIANT),
        #[allow(deprecated)]
        "fedora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'fedora' is deprecated.").print(py);
            }
            Some(icons::FEDORA)
        }
        "toggle-switch-variant" => Some(icons::TOGGLE_SWITCH_VARIANT),
        "human-male-female" => Some(icons::HUMAN_MALE_FEMALE),
        "home-lightbulb" => Some(icons::HOME_LIGHTBULB),
        "pretzel" => Some(icons::PRETZEL),
        "email-check-outline" => Some(icons::EMAIL_CHECK_OUTLINE),
        "file-find" => Some(icons::FILE_FIND),
        "map-marker" => Some(icons::MAP_MARKER),
        "fuel" => Some(icons::FUEL),
        "ultra-high-definition" => Some(icons::ULTRA_HIGH_DEFINITION),
        "movie-filter" => Some(icons::MOVIE_FILTER),
        "toy-brick-outline" => Some(icons::TOY_BRICK_OUTLINE),
        "circle-slice-3" => Some(icons::CIRCLE_SLICE_3),
        "wrench-cog-outline" => Some(icons::WRENCH_COG_OUTLINE),
        "book-minus-outline" => Some(icons::BOOK_MINUS_OUTLINE),
        "border-inside" => Some(icons::BORDER_INSIDE),
        "comment-remove-outline" => Some(icons::COMMENT_REMOVE_OUTLINE),
        "cards-playing-heart-outline" => Some(icons::CARDS_PLAYING_HEART_OUTLINE),
        "arrow-up-box" => Some(icons::ARROW_UP_BOX),
        "phone-incoming-outline" => Some(icons::PHONE_INCOMING_OUTLINE),
        "arrow-right-bold-circle" => Some(icons::ARROW_RIGHT_BOLD_CIRCLE),
        "speaker-play" => Some(icons::SPEAKER_PLAY),
        "file-excel-box" => Some(icons::FILE_EXCEL_BOX),
        "recycle-variant" => Some(icons::RECYCLE_VARIANT),
        "alphabetical-variant-off" => Some(icons::ALPHABETICAL_VARIANT_OFF),
        "file-key-outline" => Some(icons::FILE_KEY_OUTLINE),
        "cog-play-outline" => Some(icons::COG_PLAY_OUTLINE),
        "comma-circle" => Some(icons::COMMA_CIRCLE),
        "ellipse-outline" => Some(icons::ELLIPSE_OUTLINE),
        "elevator-passenger-outline" => Some(icons::ELEVATOR_PASSENGER_OUTLINE),
        "exit-run" => Some(icons::EXIT_RUN),
        "track-light" => Some(icons::TRACK_LIGHT),
        "vector-polygon-variant" => Some(icons::VECTOR_POLYGON_VARIANT),
        "sack" => Some(icons::SACK),
        "bed-empty" => Some(icons::BED_EMPTY),
        "radioactive-off" => Some(icons::RADIOACTIVE_OFF),
        "moon-last-quarter" => Some(icons::MOON_LAST_QUARTER),
        "cow" => Some(icons::COW),
        "file-document-check-outline" => Some(icons::FILE_DOCUMENT_CHECK_OUTLINE),
        "sun-thermometer-outline" => Some(icons::SUN_THERMOMETER_OUTLINE),
        "layers-minus" => Some(icons::LAYERS_MINUS),
        "signal-cellular-3" => Some(icons::SIGNAL_CELLULAR_3),
        "alpha-c-box" => Some(icons::ALPHA_C_BOX),
        "clipboard-remove" => Some(icons::CLIPBOARD_REMOVE),
        "close-outline" => Some(icons::CLOSE_OUTLINE),
        "note-outline" => Some(icons::NOTE_OUTLINE),
        "arrow-bottom-right-thin-circle-outline" => {
            Some(icons::ARROW_BOTTOM_RIGHT_THIN_CIRCLE_OUTLINE)
        }
        "application-brackets-outline" => Some(icons::APPLICATION_BRACKETS_OUTLINE),
        "border-style" => Some(icons::BORDER_STYLE),
        "queue-first-in-last-out" => Some(icons::QUEUE_FIRST_IN_LAST_OUT),
        "alpha-p-box-outline" => Some(icons::ALPHA_P_BOX_OUTLINE),
        "skate-off" => Some(icons::SKATE_OFF),
        "checkbook-arrow-left" => Some(icons::CHECKBOOK_ARROW_LEFT),
        "weather-moonset-down" => Some(icons::WEATHER_MOONSET_DOWN),
        "numeric-1-circle" => Some(icons::NUMERIC_1_CIRCLE),
        "alpha-t-box" => Some(icons::ALPHA_T_BOX),
        "truck-off-road" => Some(icons::TRUCK_OFF_ROAD),
        "gold" => Some(icons::GOLD),
        "message-star" => Some(icons::MESSAGE_STAR),
        "guy-fawkes-mask" => Some(icons::GUY_FAWKES_MASK),
        "developer-board" => Some(icons::DEVELOPER_BOARD),
        "archive-clock-outline" => Some(icons::ARCHIVE_CLOCK_OUTLINE),
        "chat-alert" => Some(icons::CHAT_ALERT),
        "clipboard-text-play" => Some(icons::CLIPBOARD_TEXT_PLAY),
        _ => None,
    }
}
