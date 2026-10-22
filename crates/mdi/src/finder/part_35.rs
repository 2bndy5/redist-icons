// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_35(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "tooltip-image" => Some(icons::TOOLTIP_IMAGE),
        "timer-check" => Some(icons::TIMER_CHECK),
        "table-alert" => Some(icons::TABLE_ALERT),
        "cloud-remove-outline" => Some(icons::CLOUD_REMOVE_OUTLINE),
        "lock" => Some(icons::LOCK),
        "email-open-multiple" => Some(icons::EMAIL_OPEN_MULTIPLE),
        "face-man-shimmer-outline" => Some(icons::FACE_MAN_SHIMMER_OUTLINE),
        "clock-check-outline" => Some(icons::CLOCK_CHECK_OUTLINE),
        "pirate" => Some(icons::PIRATE),
        "movie-check" => Some(icons::MOVIE_CHECK),
        "cash-clock" => Some(icons::CASH_CLOCK),
        "human-male-board" => Some(icons::HUMAN_MALE_BOARD),
        "trophy-award" => Some(icons::TROPHY_AWARD),
        "printer-pos-remove" => Some(icons::PRINTER_POS_REMOVE),
        "chess-rook" => Some(icons::CHESS_ROOK),
        "clock-start" => Some(icons::CLOCK_START),
        "book-settings" => Some(icons::BOOK_SETTINGS),
        "domain-plus" => Some(icons::DOMAIN_PLUS),
        "battery-20-bluetooth" => Some(icons::BATTERY_20_BLUETOOTH),
        "cards-club-outline" => Some(icons::CARDS_CLUB_OUTLINE),
        "lightbulb-cfl-off" => Some(icons::LIGHTBULB_CFL_OFF),
        "corn" => Some(icons::CORN),
        "badge-account-horizontal" => Some(icons::BADGE_ACCOUNT_HORIZONTAL),
        "turnstile-outline" => Some(icons::TURNSTILE_OUTLINE),
        "chevron-right-circle-outline" => Some(icons::CHEVRON_RIGHT_CIRCLE_OUTLINE),
        "square-rounded-badge-outline" => Some(icons::SQUARE_ROUNDED_BADGE_OUTLINE),
        "blinds" => Some(icons::BLINDS),
        "file-rotate-right-outline" => Some(icons::FILE_ROTATE_RIGHT_OUTLINE),
        "relation-zero-or-many-to-one-or-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE_OR_MANY),
        "play-circle-outline" => Some(icons::PLAY_CIRCLE_OUTLINE),
        "alpha-g-circle" => Some(icons::ALPHA_G_CIRCLE),
        "eye-plus-outline" => Some(icons::EYE_PLUS_OUTLINE),
        "arrow-u-right-bottom" => Some(icons::ARROW_U_RIGHT_BOTTOM),
        "ev-plug-ccs2" => Some(icons::EV_PLUG_CCS2),
        "bandage" => Some(icons::BANDAGE),
        "smart-card-reader-outline" => Some(icons::SMART_CARD_READER_OUTLINE),
        "cloud-minus" => Some(icons::CLOUD_MINUS),
        #[allow(deprecated)]
        "microsoft-azure" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE)
        }
        "motion" => Some(icons::MOTION),
        "timer-sand-full" => Some(icons::TIMER_SAND_FULL),
        "clock-time-two" => Some(icons::CLOCK_TIME_TWO),
        "peanut-outline" => Some(icons::PEANUT_OUTLINE),
        "table-star" => Some(icons::TABLE_STAR),
        "transit-transfer" => Some(icons::TRANSIT_TRANSFER),
        "format-quote-close-outline" => Some(icons::FORMAT_QUOTE_CLOSE_OUTLINE),
        "compass-off" => Some(icons::COMPASS_OFF),
        "store" => Some(icons::STORE),
        "comma-box-outline" => Some(icons::COMMA_BOX_OUTLINE),
        "water-sync" => Some(icons::WATER_SYNC),
        "room-service-outline" => Some(icons::ROOM_SERVICE_OUTLINE),
        "tag-heart" => Some(icons::TAG_HEART),
        "airplane-remove" => Some(icons::AIRPLANE_REMOVE),
        "bell-sleep" => Some(icons::BELL_SLEEP),
        "alert-octagram" => Some(icons::ALERT_OCTAGRAM),
        "lightbulb-auto-outline" => Some(icons::LIGHTBULB_AUTO_OUTLINE),
        "chart-box-multiple" => Some(icons::CHART_BOX_MULTIPLE),
        "check-underline-circle" => Some(icons::CHECK_UNDERLINE_CIRCLE),
        "border-bottom" => Some(icons::BORDER_BOTTOM),
        "store-check" => Some(icons::STORE_CHECK),
        "cigar-off" => Some(icons::CIGAR_OFF),
        "battery-check-outline" => Some(icons::BATTERY_CHECK_OUTLINE),
        "panda" => Some(icons::PANDA),
        "chart-bar-stacked" => Some(icons::CHART_BAR_STACKED),
        "briefcase-outline" => Some(icons::BRIEFCASE_OUTLINE),
        "table-network" => Some(icons::TABLE_NETWORK),
        "watch-variant" => Some(icons::WATCH_VARIANT),
        "fencing" => Some(icons::FENCING),
        "code-braces" => Some(icons::CODE_BRACES),
        "usb-c-port" => Some(icons::USB_C_PORT),
        "dance-pole" => Some(icons::DANCE_POLE),
        "tooltip-minus" => Some(icons::TOOLTIP_MINUS),
        "details" => Some(icons::DETAILS),
        "format-header-1" => Some(icons::FORMAT_HEADER_1),
        "mortar-pestle" => Some(icons::MORTAR_PESTLE),
        "hot-tub" => Some(icons::HOT_TUB),
        "book-minus-multiple-outline" => Some(icons::BOOK_MINUS_MULTIPLE_OUTLINE),
        "reload-alert" => Some(icons::RELOAD_ALERT),
        "image-filter-hdr" => Some(icons::IMAGE_FILTER_HDR),
        "lock-percent-open-outline" => Some(icons::LOCK_PERCENT_OPEN_OUTLINE),
        "phone-check" => Some(icons::PHONE_CHECK),
        "numeric-9-circle-outline" => Some(icons::NUMERIC_9_CIRCLE_OUTLINE),
        "clock-time-one-outline" => Some(icons::CLOCK_TIME_ONE_OUTLINE),
        "music-note-sixteenth-dotted" => Some(icons::MUSIC_NOTE_SIXTEENTH_DOTTED),
        "clipboard-minus-outline" => Some(icons::CLIPBOARD_MINUS_OUTLINE),
        "gamepad-circle-right" => Some(icons::GAMEPAD_CIRCLE_RIGHT),
        "wallpaper" => Some(icons::WALLPAPER),
        "arrow-down-bold-box" => Some(icons::ARROW_DOWN_BOLD_BOX),
        "axis-x-arrow" => Some(icons::AXIS_X_ARROW),
        "watch-vibrate" => Some(icons::WATCH_VIBRATE),
        "axis-lock" => Some(icons::AXIS_LOCK),
        "rewind-30" => Some(icons::REWIND_30),
        "clipboard-search-outline" => Some(icons::CLIPBOARD_SEARCH_OUTLINE),
        #[allow(deprecated)]
        "language-markdown-outline" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'language-markdown-outline' is deprecated.",
                )
                .print(py);
            }
            Some(icons::LANGUAGE_MARKDOWN_OUTLINE)
        }
        "wrench-cog" => Some(icons::WRENCH_COG),
        "table-border" => Some(icons::TABLE_BORDER),
        "regex" => Some(icons::REGEX),
        "book-cross" => Some(icons::BOOK_CROSS),
        "test-tube" => Some(icons::TEST_TUBE),
        "cable-data" => Some(icons::CABLE_DATA),
        "zodiac-libra" => Some(icons::ZODIAC_LIBRA),
        "wifi-star" => Some(icons::WIFI_STAR),
        "notebook-remove" => Some(icons::NOTEBOOK_REMOVE),
        "file-gif-box" => Some(icons::FILE_GIF_BOX),
        "folder-multiple-plus" => Some(icons::FOLDER_MULTIPLE_PLUS),
        "ghost-off-outline" => Some(icons::GHOST_OFF_OUTLINE),
        "key-chain-variant" => Some(icons::KEY_CHAIN_VARIANT),
        "cart-arrow-right" => Some(icons::CART_ARROW_RIGHT),
        "shield-plus-outline" => Some(icons::SHIELD_PLUS_OUTLINE),
        "drawing" => Some(icons::DRAWING),
        "numeric-1" => Some(icons::NUMERIC_1),
        "image-sync" => Some(icons::IMAGE_SYNC),
        "water-outline" => Some(icons::WATER_OUTLINE),
        "spray" => Some(icons::SPRAY),
        "file-table-box-outline" => Some(icons::FILE_TABLE_BOX_OUTLINE),
        "folder-minus-outline" => Some(icons::FOLDER_MINUS_OUTLINE),
        "cow-off" => Some(icons::COW_OFF),
        "information-box-outline" => Some(icons::INFORMATION_BOX_OUTLINE),
        "alpha-k-box" => Some(icons::ALPHA_K_BOX),
        "doctor" => Some(icons::DOCTOR),
        "monitor-screenshot" => Some(icons::MONITOR_SCREENSHOT),
        "circle-slice-7" => Some(icons::CIRCLE_SLICE_7),
        "table-of-contents" => Some(icons::TABLE_OF_CONTENTS),
        "call-received" => Some(icons::CALL_RECEIVED),
        "dice-d6" => Some(icons::DICE_D6),
        "delete-empty" => Some(icons::DELETE_EMPTY),
        "format-page-split" => Some(icons::FORMAT_PAGE_SPLIT),
        #[allow(deprecated)]
        "mapbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mapbox' is deprecated.").print(py);
            }
            Some(icons::MAPBOX)
        }
        "contacts" => Some(icons::CONTACTS),
        "solar-power" => Some(icons::SOLAR_POWER),
        "head-lightbulb-outline" => Some(icons::HEAD_LIGHTBULB_OUTLINE),
        "crop" => Some(icons::CROP),
        "train-car-autorack" => Some(icons::TRAIN_CAR_AUTORACK),
        "temperature-fahrenheit" => Some(icons::TEMPERATURE_FAHRENHEIT),
        "dots-horizontal-circle-outline" => Some(icons::DOTS_HORIZONTAL_CIRCLE_OUTLINE),
        "table-arrow-left" => Some(icons::TABLE_ARROW_LEFT),
        "camera-switch" => Some(icons::CAMERA_SWITCH),
        "death-star-variant" => Some(icons::DEATH_STAR_VARIANT),
        "heating-coil" => Some(icons::HEATING_COIL),
        "bottle-soda" => Some(icons::BOTTLE_SODA),
        "spade" => Some(icons::SPADE),
        "head-sync-outline" => Some(icons::HEAD_SYNC_OUTLINE),
        "mirror" => Some(icons::MIRROR),
        "timeline-minus-outline" => Some(icons::TIMELINE_MINUS_OUTLINE),
        "wifi" => Some(icons::WIFI),
        "safety-goggles" => Some(icons::SAFETY_GOGGLES),
        "download-off-outline" => Some(icons::DOWNLOAD_OFF_OUTLINE),
        #[allow(deprecated)]
        "google-keep" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-keep' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_KEEP)
        }
        "file-xml-box" => Some(icons::FILE_XML_BOX),
        "scooter-electric" => Some(icons::SCOOTER_ELECTRIC),
        "web-sync" => Some(icons::WEB_SYNC),
        "checkbox-blank-outline" => Some(icons::CHECKBOX_BLANK_OUTLINE),
        "airballoon-outline" => Some(icons::AIRBALLOON_OUTLINE),
        "form-select" => Some(icons::FORM_SELECT),
        "size-xxxl" => Some(icons::SIZE_XXXL),
        "numeric-4-circle-outline" => Some(icons::NUMERIC_4_CIRCLE_OUTLINE),
        "format-list-bulleted-square" => Some(icons::FORMAT_LIST_BULLETED_SQUARE),
        "skull-crossbones" => Some(icons::SKULL_CROSSBONES),
        "cards-spade" => Some(icons::CARDS_SPADE),
        "text-shadow" => Some(icons::TEXT_SHADOW),
        "podium-gold" => Some(icons::PODIUM_GOLD),
        "cloud-lock-open" => Some(icons::CLOUD_LOCK_OPEN),
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "medal" => Some(icons::MEDAL),
        "wrap-disabled" => Some(icons::WRAP_DISABLED),
        "wifi-lock" => Some(icons::WIFI_LOCK),
        "download-multiple" => Some(icons::DOWNLOAD_MULTIPLE),
        "square-edit-outline" => Some(icons::SQUARE_EDIT_OUTLINE),
        "store-remove-outline" => Some(icons::STORE_REMOVE_OUTLINE),
        "bookshelf" => Some(icons::BOOKSHELF),
        "heart-settings" => Some(icons::HEART_SETTINGS),
        "pill" => Some(icons::PILL),
        "leaf-maple-off" => Some(icons::LEAF_MAPLE_OFF),
        "flash-alert" => Some(icons::FLASH_ALERT),
        "car-key" => Some(icons::CAR_KEY),
        "book-plus-outline" => Some(icons::BOOK_PLUS_OUTLINE),
        "augmented-reality" => Some(icons::AUGMENTED_REALITY),
        "package-down" => Some(icons::PACKAGE_DOWN),
        "alien-outline" => Some(icons::ALIEN_OUTLINE),
        "transcribe" => Some(icons::TRANSCRIBE),
        "plus" => Some(icons::PLUS),
        "account-injury" => Some(icons::ACCOUNT_INJURY),
        "movie-open-star-outline" => Some(icons::MOVIE_OPEN_STAR_OUTLINE),
        "pokeball" => Some(icons::POKEBALL),
        "clipboard-off-outline" => Some(icons::CLIPBOARD_OFF_OUTLINE),
        "credit-card-wireless" => Some(icons::CREDIT_CARD_WIRELESS),
        #[allow(deprecated)]
        "jabber" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jabber' is deprecated.").print(py);
            }
            Some(icons::JABBER)
        }
        "math-cos" => Some(icons::MATH_COS),
        "podium-bronze" => Some(icons::PODIUM_BRONZE),
        "shield-account-variant-outline" => Some(icons::SHIELD_ACCOUNT_VARIANT_OUTLINE),
        "movie-open-off-outline" => Some(icons::MOVIE_OPEN_OFF_OUTLINE),
        "shark-off" => Some(icons::SHARK_OFF),
        "battery-arrow-down-outline" => Some(icons::BATTERY_ARROW_DOWN_OUTLINE),
        "account-cash" => Some(icons::ACCOUNT_CASH),
        "apple-keyboard-command" => Some(icons::APPLE_KEYBOARD_COMMAND),
        "bookmark-box-multiple-outline" => Some(icons::BOOKMARK_BOX_MULTIPLE_OUTLINE),
        "folder-arrow-up-outline" => Some(icons::FOLDER_ARROW_UP_OUTLINE),
        "copyleft" => Some(icons::COPYLEFT),
        "wallet-membership" => Some(icons::WALLET_MEMBERSHIP),
        "text-box-multiple" => Some(icons::TEXT_BOX_MULTIPLE),
        "source-commit-start" => Some(icons::SOURCE_COMMIT_START),
        _ => None,
    }
}
