// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_2(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "printer-pos-wrench" => Some(icons::PRINTER_POS_WRENCH),
        "sofa-single" => Some(icons::SOFA_SINGLE),
        "chart-pie-outline" => Some(icons::CHART_PIE_OUTLINE),
        "download-box" => Some(icons::DOWNLOAD_BOX),
        "emoticon-remove" => Some(icons::EMOTICON_REMOVE),
        "compass-off-outline" => Some(icons::COMPASS_OFF_OUTLINE),
        "basket-remove-outline" => Some(icons::BASKET_REMOVE_OUTLINE),
        "human-male-board-poll" => Some(icons::HUMAN_MALE_BOARD_POLL),
        "view-compact-outline" => Some(icons::VIEW_COMPACT_OUTLINE),
        "curling" => Some(icons::CURLING),
        "chili-alert-outline" => Some(icons::CHILI_ALERT_OUTLINE),
        "download-multiple" => Some(icons::DOWNLOAD_MULTIPLE),
        "text-box-search" => Some(icons::TEXT_BOX_SEARCH),
        "format-wrap-inline" => Some(icons::FORMAT_WRAP_INLINE),
        "store-settings-outline" => Some(icons::STORE_SETTINGS_OUTLINE),
        "chess-bishop" => Some(icons::CHESS_BISHOP),
        "beaker-alert-outline" => Some(icons::BEAKER_ALERT_OUTLINE),
        "relation-one-to-only-one" => Some(icons::RELATION_ONE_TO_ONLY_ONE),
        "passport-check" => Some(icons::PASSPORT_CHECK),
        #[allow(deprecated)]
        "language-r" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-r' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_R)
        }
        "share-circle" => Some(icons::SHARE_CIRCLE),
        "basketball-hoop" => Some(icons::BASKETBALL_HOOP),
        "movie-open-play" => Some(icons::MOVIE_OPEN_PLAY),
        "content-save-move-outline" => Some(icons::CONTENT_SAVE_MOVE_OUTLINE),
        "code-less-than-or-equal" => Some(icons::CODE_LESS_THAN_OR_EQUAL),
        "circle-multiple-outline" => Some(icons::CIRCLE_MULTIPLE_OUTLINE),
        "ghost-off" => Some(icons::GHOST_OFF),
        "wallet-plus-outline" => Some(icons::WALLET_PLUS_OUTLINE),
        "size-xxs" => Some(icons::SIZE_XXS),
        "camera-wireless" => Some(icons::CAMERA_WIRELESS),
        "blur" => Some(icons::BLUR),
        "format-text-rotation-down" => Some(icons::FORMAT_TEXT_ROTATION_DOWN),
        "file-lock-open-outline" => Some(icons::FILE_LOCK_OPEN_OUTLINE),
        "heart-settings" => Some(icons::HEART_SETTINGS),
        "comment-bookmark" => Some(icons::COMMENT_BOOKMARK),
        "mouse-move-vertical" => Some(icons::MOUSE_MOVE_VERTICAL),
        "shore" => Some(icons::SHORE),
        #[allow(deprecated)]
        "gog" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gog' is deprecated.").print(py);
            }
            Some(icons::GOG)
        }
        "triangle-down" => Some(icons::TRIANGLE_DOWN),
        "bottle-wine" => Some(icons::BOTTLE_WINE),
        "briefcase-arrow-up-down-outline" => Some(icons::BRIEFCASE_ARROW_UP_DOWN_OUTLINE),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "car-search-outline" => Some(icons::CAR_SEARCH_OUTLINE),
        "clipboard-play-multiple" => Some(icons::CLIPBOARD_PLAY_MULTIPLE),
        "zodiac-libra" => Some(icons::ZODIAC_LIBRA),
        "dice-d20" => Some(icons::DICE_D20),
        "ruler-square-compass" => Some(icons::RULER_SQUARE_COMPASS),
        "pencil-remove-outline" => Some(icons::PENCIL_REMOVE_OUTLINE),
        "relation-zero-or-one-to-only-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONLY_ONE),
        "water-check-outline" => Some(icons::WATER_CHECK_OUTLINE),
        "gate-xor" => Some(icons::GATE_XOR),
        "numeric-0-circle-outline" => Some(icons::NUMERIC_0_CIRCLE_OUTLINE),
        "image-check" => Some(icons::IMAGE_CHECK),
        "numeric-positive-1" => Some(icons::NUMERIC_POSITIVE_1),
        "bag-personal-plus" => Some(icons::BAG_PERSONAL_PLUS),
        "account-multiple-remove" => Some(icons::ACCOUNT_MULTIPLE_REMOVE),
        #[allow(deprecated)]
        "apache-kafka" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apache-kafka' is deprecated.").print(py);
            }
            Some(icons::APACHE_KAFKA)
        }
        "book-minus-outline" => Some(icons::BOOK_MINUS_OUTLINE),
        "arrow-expand-right" => Some(icons::ARROW_EXPAND_RIGHT),
        "car-light-alert" => Some(icons::CAR_LIGHT_ALERT),
        "crop" => Some(icons::CROP),
        "check-underline-circle" => Some(icons::CHECK_UNDERLINE_CIRCLE),
        "account-cancel-outline" => Some(icons::ACCOUNT_CANCEL_OUTLINE),
        "hand-extended" => Some(icons::HAND_EXTENDED),
        "file-music" => Some(icons::FILE_MUSIC),
        "bottle-tonic-plus" => Some(icons::BOTTLE_TONIC_PLUS),
        "invoice-send-outline" => Some(icons::INVOICE_SEND_OUTLINE),
        "book-outline" => Some(icons::BOOK_OUTLINE),
        "cellphone-remove" => Some(icons::CELLPHONE_REMOVE),
        "calendar-heart-outline" => Some(icons::CALENDAR_HEART_OUTLINE),
        "arrow-u-right-top" => Some(icons::ARROW_U_RIGHT_TOP),
        "exit-to-app" => Some(icons::EXIT_TO_APP),
        "briefcase-minus" => Some(icons::BRIEFCASE_MINUS),
        "border-none" => Some(icons::BORDER_NONE),
        "keyboard-settings" => Some(icons::KEYBOARD_SETTINGS),
        "clock-time-three" => Some(icons::CLOCK_TIME_THREE),
        "percent-box" => Some(icons::PERCENT_BOX),
        "archive-check" => Some(icons::ARCHIVE_CHECK),
        "star-box" => Some(icons::STAR_BOX),
        "fit-to-page-outline" => Some(icons::FIT_TO_PAGE_OUTLINE),
        "bag-suitcase" => Some(icons::BAG_SUITCASE),
        "folder-arrow-down" => Some(icons::FOLDER_ARROW_DOWN),
        #[allow(deprecated)]
        "microsoft-windows" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-windows' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS)
        }
        "ocarina" => Some(icons::OCARINA),
        "asterisk-circle-outline" => Some(icons::ASTERISK_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "hulu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'hulu' is deprecated.").print(py);
            }
            Some(icons::HULU)
        }
        "file-clock-outline" => Some(icons::FILE_CLOCK_OUTLINE),
        "rhombus-medium-outline" => Some(icons::RHOMBUS_MEDIUM_OUTLINE),
        #[allow(deprecated)]
        "microsoft-edge" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-edge' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_EDGE)
        }
        "credit-card-wireless-off" => Some(icons::CREDIT_CARD_WIRELESS_OFF),
        "music-box-multiple" => Some(icons::MUSIC_BOX_MULTIPLE),
        "elevator" => Some(icons::ELEVATOR),
        "wifi-strength-1-alert" => Some(icons::WIFI_STRENGTH_1_ALERT),
        "clipboard-plus-outline" => Some(icons::CLIPBOARD_PLUS_OUTLINE),
        "donkey" => Some(icons::DONKEY),
        "sign-language" => Some(icons::SIGN_LANGUAGE),
        "fridge-variant-alert" => Some(icons::FRIDGE_VARIANT_ALERT),
        "roman-numeral-2" => Some(icons::ROMAN_NUMERAL_2),
        "cloud-arrow-left" => Some(icons::CLOUD_ARROW_LEFT),
        "earbuds-off" => Some(icons::EARBUDS_OFF),
        "check-circle-outline" => Some(icons::CHECK_CIRCLE_OUTLINE),
        "credit-card-settings" => Some(icons::CREDIT_CARD_SETTINGS),
        "bank-plus" => Some(icons::BANK_PLUS),
        "image-filter-vintage" => Some(icons::IMAGE_FILTER_VINTAGE),
        "star-cog-outline" => Some(icons::STAR_COG_OUTLINE),
        "priority-low" => Some(icons::PRIORITY_LOW),
        "pot-outline" => Some(icons::POT_OUTLINE),
        "application-cog" => Some(icons::APPLICATION_COG),
        "turbine" => Some(icons::TURBINE),
        "file-marker" => Some(icons::FILE_MARKER),
        "chat-sleep" => Some(icons::CHAT_SLEEP),
        "table-eye" => Some(icons::TABLE_EYE),
        "pillar" => Some(icons::PILLAR),
        "layers-off" => Some(icons::LAYERS_OFF),
        "cloud-lock" => Some(icons::CLOUD_LOCK),
        "folder-alert" => Some(icons::FOLDER_ALERT),
        #[allow(deprecated)]
        "language-xaml" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-xaml' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_XAML)
        }
        "flask-empty-minus" => Some(icons::FLASK_EMPTY_MINUS),
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        "shield-account-variant-outline" => Some(icons::SHIELD_ACCOUNT_VARIANT_OUTLINE),
        "flag-checkered" => Some(icons::FLAG_CHECKERED),
        "shield-lock-outline" => Some(icons::SHIELD_LOCK_OUTLINE),
        "tractor" => Some(icons::TRACTOR),
        "cellphone-nfc" => Some(icons::CELLPHONE_NFC),
        "bucket" => Some(icons::BUCKET),
        "rss" => Some(icons::RSS),
        "baseball-outline" => Some(icons::BASEBALL_OUTLINE),
        "car-turbocharger" => Some(icons::CAR_TURBOCHARGER),
        "numeric-5-box-multiple-outline" => Some(icons::NUMERIC_5_BOX_MULTIPLE_OUTLINE),
        "battery-remove-outline" => Some(icons::BATTERY_REMOVE_OUTLINE),
        "cards-playing" => Some(icons::CARDS_PLAYING),
        "mirror" => Some(icons::MIRROR),
        "map-clock-outline" => Some(icons::MAP_CLOCK_OUTLINE),
        "one-up" => Some(icons::ONE_UP),
        "sort-numeric-ascending" => Some(icons::SORT_NUMERIC_ASCENDING),
        "timer-stop-outline" => Some(icons::TIMER_STOP_OUTLINE),
        "surround-sound-3-1" => Some(icons::SURROUND_SOUND_3_1),
        "mirror-rectangle" => Some(icons::MIRROR_RECTANGLE),
        "poker-chip" => Some(icons::POKER_CHIP),
        "cast-audio" => Some(icons::CAST_AUDIO),
        "fuel" => Some(icons::FUEL),
        "ocr" => Some(icons::OCR),
        "arrow-up-down" => Some(icons::ARROW_UP_DOWN),
        "credit-card-off" => Some(icons::CREDIT_CARD_OFF),
        "shovel-off" => Some(icons::SHOVEL_OFF),
        "arrow-right-bold-circle-outline" => Some(icons::ARROW_RIGHT_BOLD_CIRCLE_OUTLINE),
        "smoke-detector-alert-outline" => Some(icons::SMOKE_DETECTOR_ALERT_OUTLINE),
        "book-plus-outline" => Some(icons::BOOK_PLUS_OUTLINE),
        "flag-variant-outline" => Some(icons::FLAG_VARIANT_OUTLINE),
        "alpha-o-box" => Some(icons::ALPHA_O_BOX),
        "sun-thermometer-outline" => Some(icons::SUN_THERMOMETER_OUTLINE),
        "snake" => Some(icons::SNAKE),
        "tally-mark-4" => Some(icons::TALLY_MARK_4),
        "robot-industrial-outline" => Some(icons::ROBOT_INDUSTRIAL_OUTLINE),
        "music-note" => Some(icons::MUSIC_NOTE),
        "thermometer-alert" => Some(icons::THERMOMETER_ALERT),
        "table-arrow-down" => Some(icons::TABLE_ARROW_DOWN),
        "receipt-send" => Some(icons::RECEIPT_SEND),
        "dice-3" => Some(icons::DICE_3),
        "folder-eye-outline" => Some(icons::FOLDER_EYE_OUTLINE),
        "engine" => Some(icons::ENGINE),
        "camera-wireless-outline" => Some(icons::CAMERA_WIRELESS_OUTLINE),
        "navigation-variant-outline" => Some(icons::NAVIGATION_VARIANT_OUTLINE),
        "library-shelves" => Some(icons::LIBRARY_SHELVES),
        "set-left-center" => Some(icons::SET_LEFT_CENTER),
        "inbox-arrow-down" => Some(icons::INBOX_ARROW_DOWN),
        "folder-eye" => Some(icons::FOLDER_EYE),
        #[allow(deprecated)]
        "midi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'midi' is deprecated.").print(py);
            }
            Some(icons::MIDI)
        }
        "archive-sync" => Some(icons::ARCHIVE_SYNC),
        "vector-intersection" => Some(icons::VECTOR_INTERSECTION),
        "routes" => Some(icons::ROUTES),
        "fishbowl" => Some(icons::FISHBOWL),
        "file-gif-box" => Some(icons::FILE_GIF_BOX),
        "invoice-text-clock-outline" => Some(icons::INVOICE_TEXT_CLOCK_OUTLINE),
        "progress-helper" => Some(icons::PROGRESS_HELPER),
        "relation-many-to-one-or-many" => Some(icons::RELATION_MANY_TO_ONE_OR_MANY),
        "cash-lock-open" => Some(icons::CASH_LOCK_OPEN),
        "overscan" => Some(icons::OVERSCAN),
        "umbrella-closed" => Some(icons::UMBRELLA_CLOSED),
        "wallet-outline" => Some(icons::WALLET_OUTLINE),
        "gesture" => Some(icons::GESTURE),
        "tooltip-cellphone" => Some(icons::TOOLTIP_CELLPHONE),
        "food-steak" => Some(icons::FOOD_STEAK),
        "cellphone-play" => Some(icons::CELLPHONE_PLAY),
        "alpha-g-box-outline" => Some(icons::ALPHA_G_BOX_OUTLINE),
        "head-flash-outline" => Some(icons::HEAD_FLASH_OUTLINE),
        "flask-round-bottom-outline" => Some(icons::FLASK_ROUND_BOTTOM_OUTLINE),
        "airplane-marker" => Some(icons::AIRPLANE_MARKER),
        "book-lock-open-outline" => Some(icons::BOOK_LOCK_OPEN_OUTLINE),
        "movie-cog-outline" => Some(icons::MOVIE_COG_OUTLINE),
        "triangle" => Some(icons::TRIANGLE),
        "arrow-right-circle-outline" => Some(icons::ARROW_RIGHT_CIRCLE_OUTLINE),
        "arrow-collapse-right" => Some(icons::ARROW_COLLAPSE_RIGHT),
        "needle" => Some(icons::NEEDLE),
        "go-kart-track" => Some(icons::GO_KART_TRACK),
        "tag-edit" => Some(icons::TAG_EDIT),
        "tooltip-image" => Some(icons::TOOLTIP_IMAGE),
        "chevron-up-circle" => Some(icons::CHEVRON_UP_CIRCLE),
        "account-clock-outline" => Some(icons::ACCOUNT_CLOCK_OUTLINE),
        "middleware" => Some(icons::MIDDLEWARE),
        _ => None,
    }
}
