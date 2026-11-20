// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_36(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "domain" => Some(icons::DOMAIN),
        "volume-vibrate" => Some(icons::VOLUME_VIBRATE),
        "numeric-off" => Some(icons::NUMERIC_OFF),
        "cross-celtic" => Some(icons::CROSS_CELTIC),
        "calendar-account" => Some(icons::CALENDAR_ACCOUNT),
        "glass-wine" => Some(icons::GLASS_WINE),
        "sync" => Some(icons::SYNC),
        "call-merge" => Some(icons::CALL_MERGE),
        "image-filter-black-white" => Some(icons::IMAGE_FILTER_BLACK_WHITE),
        "folder-alert" => Some(icons::FOLDER_ALERT),
        "format-header-equal" => Some(icons::FORMAT_HEADER_EQUAL),
        #[allow(deprecated)]
        "google-lens" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-lens' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_LENS)
        }
        "image-remove" => Some(icons::IMAGE_REMOVE),
        "storefront-outline" => Some(icons::STOREFRONT_OUTLINE),
        "bank-transfer-in" => Some(icons::BANK_TRANSFER_IN),
        "food-takeout-box" => Some(icons::FOOD_TAKEOUT_BOX),
        "arrow-left-bold-circle-outline" => Some(icons::ARROW_LEFT_BOLD_CIRCLE_OUTLINE),
        "power-socket-au" => Some(icons::POWER_SOCKET_AU),
        "home-floor-1" => Some(icons::HOME_FLOOR_1),
        "bicycle-penny-farthing" => Some(icons::BICYCLE_PENNY_FARTHING),
        "battery-80" => Some(icons::BATTERY_80),
        "view-day-outline" => Some(icons::VIEW_DAY_OUTLINE),
        "led-outline" => Some(icons::LED_OUTLINE),
        "cube-scan" => Some(icons::CUBE_SCAN),
        #[allow(deprecated)]
        "blender-software" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'blender-software' is deprecated.")
                    .print(py);
            }
            Some(icons::BLENDER_SOFTWARE)
        }
        "cash-remove" => Some(icons::CASH_REMOVE),
        "text-recognition" => Some(icons::TEXT_RECOGNITION),
        "bookmark-box-outline" => Some(icons::BOOKMARK_BOX_OUTLINE),
        "sign-caution" => Some(icons::SIGN_CAUTION),
        "beaker-minus" => Some(icons::BEAKER_MINUS),
        #[allow(deprecated)]
        "steam" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'steam' is deprecated.").print(py);
            }
            Some(icons::STEAM)
        }
        "screen-rotation-lock" => Some(icons::SCREEN_ROTATION_LOCK),
        "heat-wave" => Some(icons::HEAT_WAVE),
        "hand-water" => Some(icons::HAND_WATER),
        "tunnel" => Some(icons::TUNNEL),
        "shape-square-rounded-plus" => Some(icons::SHAPE_SQUARE_ROUNDED_PLUS),
        "inbox-arrow-up" => Some(icons::INBOX_ARROW_UP),
        "menu-close" => Some(icons::MENU_CLOSE),
        "note-off-outline" => Some(icons::NOTE_OFF_OUTLINE),
        "arrow-up-bold-box" => Some(icons::ARROW_UP_BOLD_BOX),
        "phone-paused-outline" => Some(icons::PHONE_PAUSED_OUTLINE),
        "wizard-hat" => Some(icons::WIZARD_HAT),
        "format-rotate-90" => Some(icons::FORMAT_ROTATE_90),
        "minus-network" => Some(icons::MINUS_NETWORK),
        "book-alert-outline" => Some(icons::BOOK_ALERT_OUTLINE),
        "format-text-rotation-up" => Some(icons::FORMAT_TEXT_ROTATION_UP),
        "storefront" => Some(icons::STOREFRONT),
        "briefcase-clock" => Some(icons::BRIEFCASE_CLOCK),
        "database" => Some(icons::DATABASE),
        "decimal-comma" => Some(icons::DECIMAL_COMMA),
        "message-fast" => Some(icons::MESSAGE_FAST),
        "view-headline" => Some(icons::VIEW_HEADLINE),
        "cash-marker" => Some(icons::CASH_MARKER),
        "timer-minus-outline" => Some(icons::TIMER_MINUS_OUTLINE),
        "gate-open" => Some(icons::GATE_OPEN),
        "arrow-down-right" => Some(icons::ARROW_DOWN_RIGHT),
        "home-floor-3" => Some(icons::HOME_FLOOR_3),
        "close-circle-outline" => Some(icons::CLOSE_CIRCLE_OUTLINE),
        "checkbox-blank-badge" => Some(icons::CHECKBOX_BLANK_BADGE),
        "chili-off-outline" => Some(icons::CHILI_OFF_OUTLINE),
        "arrow-up-bold-box-outline" => Some(icons::ARROW_UP_BOLD_BOX_OUTLINE),
        "download-multiple-outline" => Some(icons::DOWNLOAD_MULTIPLE_OUTLINE),
        "egg-off-outline" => Some(icons::EGG_OFF_OUTLINE),
        "arrow-left-right-bold-outline" => Some(icons::ARROW_LEFT_RIGHT_BOLD_OUTLINE),
        "database-marker" => Some(icons::DATABASE_MARKER),
        "timer-cancel" => Some(icons::TIMER_CANCEL),
        "diaper-outline" => Some(icons::DIAPER_OUTLINE),
        "window-open-variant" => Some(icons::WINDOW_OPEN_VARIANT),
        "code-block-tags" => Some(icons::CODE_BLOCK_TAGS),
        "heart-flash" => Some(icons::HEART_FLASH),
        "lungs" => Some(icons::LUNGS),
        "water-well" => Some(icons::WATER_WELL),
        "diversify" => Some(icons::DIVERSIFY),
        "flask-remove-outline" => Some(icons::FLASK_REMOVE_OUTLINE),
        "peanut-off-outline" => Some(icons::PEANUT_OFF_OUTLINE),
        #[allow(deprecated)]
        "hulu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'hulu' is deprecated.").print(py);
            }
            Some(icons::HULU)
        }
        "filter-multiple" => Some(icons::FILTER_MULTIPLE),
        "cellphone-settings" => Some(icons::CELLPHONE_SETTINGS),
        "home-sound-out" => Some(icons::HOME_SOUND_OUT),
        "cloud-plus" => Some(icons::CLOUD_PLUS),
        "card-bulleted-settings" => Some(icons::CARD_BULLETED_SETTINGS),
        "weather-partly-cloudy" => Some(icons::WEATHER_PARTLY_CLOUDY),
        "battery-remove" => Some(icons::BATTERY_REMOVE),
        "television-ambient-light" => Some(icons::TELEVISION_AMBIENT_LIGHT),
        "volume-high" => Some(icons::VOLUME_HIGH),
        #[allow(deprecated)]
        "language-css3" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-css3' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CSS3)
        }
        "book-cross" => Some(icons::BOOK_CROSS),
        "cloud-alert" => Some(icons::CLOUD_ALERT),
        "arrow-left-thin" => Some(icons::ARROW_LEFT_THIN),
        "radiobox-indeterminate-variant" => Some(icons::RADIOBOX_INDETERMINATE_VARIANT),
        "auto-upload" => Some(icons::AUTO_UPLOAD),
        "book-open-outline" => Some(icons::BOOK_OPEN_OUTLINE),
        "book-minus-multiple-outline" => Some(icons::BOOK_MINUS_MULTIPLE_OUTLINE),
        "movie-open-cog-outline" => Some(icons::MOVIE_OPEN_COG_OUTLINE),
        "check-circle" => Some(icons::CHECK_CIRCLE),
        "car-defrost-rear" => Some(icons::CAR_DEFROST_REAR),
        "briefcase-arrow-left-right" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT),
        "dog-service" => Some(icons::DOG_SERVICE),
        "umbrella-beach" => Some(icons::UMBRELLA_BEACH),
        "bell-check" => Some(icons::BELL_CHECK),
        "keyboard-f11" => Some(icons::KEYBOARD_F11),
        "keyboard-f4" => Some(icons::KEYBOARD_F4),
        "vhs" => Some(icons::VHS),
        "ruler-square" => Some(icons::RULER_SQUARE),
        "comment-alert" => Some(icons::COMMENT_ALERT),
        "lock-open-check" => Some(icons::LOCK_OPEN_CHECK),
        "camera-metering-center" => Some(icons::CAMERA_METERING_CENTER),
        "forum-minus-outline" => Some(icons::FORUM_MINUS_OUTLINE),
        "note" => Some(icons::NOTE),
        "phone-classic-off" => Some(icons::PHONE_CLASSIC_OFF),
        "speaker-play" => Some(icons::SPEAKER_PLAY),
        "checkbox-marked-circle-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_OUTLINE),
        "movie-settings" => Some(icons::MOVIE_SETTINGS),
        "timer-off-outline" => Some(icons::TIMER_OFF_OUTLINE),
        "archive-lock-open" => Some(icons::ARCHIVE_LOCK_OPEN),
        "ski-water" => Some(icons::SKI_WATER),
        "egg-off" => Some(icons::EGG_OFF),
        "chart-box-outline" => Some(icons::CHART_BOX_OUTLINE),
        "card-bulleted-off" => Some(icons::CARD_BULLETED_OFF),
        "invoice-text-remove-outline" => Some(icons::INVOICE_TEXT_REMOVE_OUTLINE),
        "printer-3d-nozzle-heat-outline" => Some(icons::PRINTER_3D_NOZZLE_HEAT_OUTLINE),
        "access-point-minus" => Some(icons::ACCESS_POINT_MINUS),
        "head-sync-outline" => Some(icons::HEAD_SYNC_OUTLINE),
        "message-reply" => Some(icons::MESSAGE_REPLY),
        "space-station" => Some(icons::SPACE_STATION),
        "record-circle" => Some(icons::RECORD_CIRCLE),
        "pause-box-outline" => Some(icons::PAUSE_BOX_OUTLINE),
        "puzzle-outline" => Some(icons::PUZZLE_OUTLINE),
        "phone-sync" => Some(icons::PHONE_SYNC),
        "arrow-right-bold-circle" => Some(icons::ARROW_RIGHT_BOLD_CIRCLE),
        "plus-box" => Some(icons::PLUS_BOX),
        #[allow(deprecated)]
        "microsoft-windows" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-windows' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS)
        }
        "debug-step-over" => Some(icons::DEBUG_STEP_OVER),
        "credit-card-remove" => Some(icons::CREDIT_CARD_REMOVE),
        "clipboard-off-outline" => Some(icons::CLIPBOARD_OFF_OUTLINE),
        "card-account-details" => Some(icons::CARD_ACCOUNT_DETAILS),
        "all-inclusive" => Some(icons::ALL_INCLUSIVE),
        "view-sequential-outline" => Some(icons::VIEW_SEQUENTIAL_OUTLINE),
        "script-text-play-outline" => Some(icons::SCRIPT_TEXT_PLAY_OUTLINE),
        "help-box-outline" => Some(icons::HELP_BOX_OUTLINE),
        "rewind" => Some(icons::REWIND),
        "robot-industrial-outline" => Some(icons::ROBOT_INDUSTRIAL_OUTLINE),
        "robot-happy" => Some(icons::ROBOT_HAPPY),
        "bag-checked" => Some(icons::BAG_CHECKED),
        "earbuds-off" => Some(icons::EARBUDS_OFF),
        "alpha-i-circle" => Some(icons::ALPHA_I_CIRCLE),
        "file-image" => Some(icons::FILE_IMAGE),
        "disc" => Some(icons::DISC),
        "television-classic" => Some(icons::TELEVISION_CLASSIC),
        "cloud-circle" => Some(icons::CLOUD_CIRCLE),
        "taco" => Some(icons::TACO),
        "calendar-collapse-horizontal-outline" => Some(icons::CALENDAR_COLLAPSE_HORIZONTAL_OUTLINE),
        "human-white-cane" => Some(icons::HUMAN_WHITE_CANE),
        "head-dots-horizontal-outline" => Some(icons::HEAD_DOTS_HORIZONTAL_OUTLINE),
        "firework" => Some(icons::FIREWORK),
        "alpha-g-circle" => Some(icons::ALPHA_G_CIRCLE),
        "cards-playing-spade-outline" => Some(icons::CARDS_PLAYING_SPADE_OUTLINE),
        #[allow(deprecated)]
        "vimeo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vimeo' is deprecated.").print(py);
            }
            Some(icons::VIMEO)
        }
        "cash-clock" => Some(icons::CASH_CLOCK),
        "pliers" => Some(icons::PLIERS),
        "hops" => Some(icons::HOPS),
        "car-multiple" => Some(icons::CAR_MULTIPLE),
        "airplane-marker" => Some(icons::AIRPLANE_MARKER),
        "text-box-multiple" => Some(icons::TEXT_BOX_MULTIPLE),
        "image-refresh" => Some(icons::IMAGE_REFRESH),
        "map-search-outline" => Some(icons::MAP_SEARCH_OUTLINE),
        "select-marker" => Some(icons::SELECT_MARKER),
        "synagogue-outline" => Some(icons::SYNAGOGUE_OUTLINE),
        "home-lock" => Some(icons::HOME_LOCK),
        "table-arrow-up" => Some(icons::TABLE_ARROW_UP),
        "archive-settings" => Some(icons::ARCHIVE_SETTINGS),
        "charity-search" => Some(icons::CHARITY_SEARCH),
        "approximately-equal-box" => Some(icons::APPROXIMATELY_EQUAL_BOX),
        "code-tags-check" => Some(icons::CODE_TAGS_CHECK),
        "weather-lightning" => Some(icons::WEATHER_LIGHTNING),
        #[allow(deprecated)]
        "android-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android-studio' is deprecated.").print(py);
            }
            Some(icons::ANDROID_STUDIO)
        }
        "message-plus" => Some(icons::MESSAGE_PLUS),
        "tilde-off" => Some(icons::TILDE_OFF),
        "radio-handheld" => Some(icons::RADIO_HANDHELD),
        "carabiner" => Some(icons::CARABINER),
        "format-align-justify" => Some(icons::FORMAT_ALIGN_JUSTIFY),
        "human-female-dance" => Some(icons::HUMAN_FEMALE_DANCE),
        "shield-account-variant" => Some(icons::SHIELD_ACCOUNT_VARIANT),
        "cookie-alert-outline" => Some(icons::COOKIE_ALERT_OUTLINE),
        "star-remove" => Some(icons::STAR_REMOVE),
        "lan" => Some(icons::LAN),
        "arrow-top-right-bottom-left" => Some(icons::ARROW_TOP_RIGHT_BOTTOM_LEFT),
        "golf" => Some(icons::GOLF),
        "floppy" => Some(icons::FLOPPY),
        "square-off-outline" => Some(icons::SQUARE_OFF_OUTLINE),
        "filter-menu" => Some(icons::FILTER_MENU),
        "movie-check-outline" => Some(icons::MOVIE_CHECK_OUTLINE),
        "camera-front-variant" => Some(icons::CAMERA_FRONT_VARIANT),
        "format-header-pound" => Some(icons::FORMAT_HEADER_POUND),
        "zodiac-cancer" => Some(icons::ZODIAC_CANCER),
        "step-forward-2" => Some(icons::STEP_FORWARD_2),
        "console" => Some(icons::CONSOLE),
        "bell-ring-outline" => Some(icons::BELL_RING_OUTLINE),
        "image-filter-center-focus-strong" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG),
        "truck-trailer" => Some(icons::TRUCK_TRAILER),
        _ => None,
    }
}
