// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_12(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "boom-gate-arrow-up" => Some(icons::BOOM_GATE_ARROW_UP),
        "file-jpg-box" => Some(icons::FILE_JPG_BOX),
        "blur-linear" => Some(icons::BLUR_LINEAR),
        "consolidate" => Some(icons::CONSOLIDATE),
        "pencil-lock-outline" => Some(icons::PENCIL_LOCK_OUTLINE),
        "lightbulb-group-outline" => Some(icons::LIGHTBULB_GROUP_OUTLINE),
        "alpha-a-circle" => Some(icons::ALPHA_A_CIRCLE),
        "greater-than" => Some(icons::GREATER_THAN),
        "battery-50" => Some(icons::BATTERY_50),
        "cosine-wave" => Some(icons::COSINE_WAVE),
        "file-code" => Some(icons::FILE_CODE),
        "filter-menu-outline" => Some(icons::FILTER_MENU_OUTLINE),
        "bookmark-minus-outline" => Some(icons::BOOKMARK_MINUS_OUTLINE),
        "briefcase-plus-outline" => Some(icons::BRIEFCASE_PLUS_OUTLINE),
        "ballot" => Some(icons::BALLOT),
        "note-check" => Some(icons::NOTE_CHECK),
        "apps" => Some(icons::APPS),
        "home-lock-open" => Some(icons::HOME_LOCK_OPEN),
        "book-lock-open-outline" => Some(icons::BOOK_LOCK_OPEN_OUTLINE),
        "gesture-tap-button" => Some(icons::GESTURE_TAP_BUTTON),
        "noodles" => Some(icons::NOODLES),
        "train" => Some(icons::TRAIN),
        "progress-close" => Some(icons::PROGRESS_CLOSE),
        "image-area-close" => Some(icons::IMAGE_AREA_CLOSE),
        "vector-point-minus" => Some(icons::VECTOR_POINT_MINUS),
        "distribute-vertical-center" => Some(icons::DISTRIBUTE_VERTICAL_CENTER),
        "home-variant" => Some(icons::HOME_VARIANT),
        "flask-off-outline" => Some(icons::FLASK_OFF_OUTLINE),
        #[allow(deprecated)]
        "home-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'home-assistant' is deprecated.").print(py);
            }
            Some(icons::HOME_ASSISTANT)
        }
        "book-music" => Some(icons::BOOK_MUSIC),
        #[allow(deprecated)]
        "language-csharp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-csharp' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_CSHARP)
        }
        "metronome" => Some(icons::METRONOME),
        "debug-step-out" => Some(icons::DEBUG_STEP_OUT),
        "wifi-strength-1" => Some(icons::WIFI_STRENGTH_1),
        "database-edit-outline" => Some(icons::DATABASE_EDIT_OUTLINE),
        "camera-gopro" => Some(icons::CAMERA_GOPRO),
        "expansion-card-variant" => Some(icons::EXPANSION_CARD_VARIANT),
        "content-save-minus-outline" => Some(icons::CONTENT_SAVE_MINUS_OUTLINE),
        "map-marker-alert-outline" => Some(icons::MAP_MARKER_ALERT_OUTLINE),
        "chart-tree" => Some(icons::CHART_TREE),
        "lock-open-remove-outline" => Some(icons::LOCK_OPEN_REMOVE_OUTLINE),
        "movie-filter-outline" => Some(icons::MOVIE_FILTER_OUTLINE),
        "account-tie-hat" => Some(icons::ACCOUNT_TIE_HAT),
        "candelabra" => Some(icons::CANDELABRA),
        "camera-timer" => Some(icons::CAMERA_TIMER),
        "store-marker" => Some(icons::STORE_MARKER),
        "volume-high" => Some(icons::VOLUME_HIGH),
        "passport-check" => Some(icons::PASSPORT_CHECK),
        "heart-remove" => Some(icons::HEART_REMOVE),
        "timer-refresh-outline" => Some(icons::TIMER_REFRESH_OUTLINE),
        "tire" => Some(icons::TIRE),
        "printer-pos-plus" => Some(icons::PRINTER_POS_PLUS),
        "star-plus" => Some(icons::STAR_PLUS),
        "account-hard-hat-outline" => Some(icons::ACCOUNT_HARD_HAT_OUTLINE),
        "badge-account-alert" => Some(icons::BADGE_ACCOUNT_ALERT),
        "format-header-1" => Some(icons::FORMAT_HEADER_1),
        "puzzle-check-outline" => Some(icons::PUZZLE_CHECK_OUTLINE),
        "window-closed-variant" => Some(icons::WINDOW_CLOSED_VARIANT),
        "calendar-blank-multiple" => Some(icons::CALENDAR_BLANK_MULTIPLE),
        "microphone-plus" => Some(icons::MICROPHONE_PLUS),
        "checkerboard-remove" => Some(icons::CHECKERBOARD_REMOVE),
        "caravan" => Some(icons::CARAVAN),
        "close-network" => Some(icons::CLOSE_NETWORK),
        "airbag" => Some(icons::AIRBAG),
        "crown-outline" => Some(icons::CROWN_OUTLINE),
        "cloud-sync-outline" => Some(icons::CLOUD_SYNC_OUTLINE),
        "timeline-check-outline" => Some(icons::TIMELINE_CHECK_OUTLINE),
        "music-note-off-outline" => Some(icons::MUSIC_NOTE_OFF_OUTLINE),
        "translate-variant" => Some(icons::TRANSLATE_VARIANT),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-charging" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-charging' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_CHARGING)
        }
        "moon-waning-crescent" => Some(icons::MOON_WANING_CRESCENT),
        "book-education" => Some(icons::BOOK_EDUCATION),
        "autorenew-off" => Some(icons::AUTORENEW_OFF),
        "folder-file-outline" => Some(icons::FOLDER_FILE_OUTLINE),
        "shield-link-variant" => Some(icons::SHIELD_LINK_VARIANT),
        "circle-slice-2" => Some(icons::CIRCLE_SLICE_2),
        "filter-menu" => Some(icons::FILTER_MENU),
        "script-text-key" => Some(icons::SCRIPT_TEXT_KEY),
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "account-filter-outline" => Some(icons::ACCOUNT_FILTER_OUTLINE),
        "cookie-edit" => Some(icons::COOKIE_EDIT),
        "debug-step-over" => Some(icons::DEBUG_STEP_OVER),
        "file-export" => Some(icons::FILE_EXPORT),
        "fast-forward-45" => Some(icons::FAST_FORWARD_45),
        "transfer-right" => Some(icons::TRANSFER_RIGHT),
        "beaker-question" => Some(icons::BEAKER_QUESTION),
        "toaster" => Some(icons::TOASTER),
        "sort-calendar-descending" => Some(icons::SORT_CALENDAR_DESCENDING),
        "content-save-all-outline" => Some(icons::CONTENT_SAVE_ALL_OUTLINE),
        "numeric" => Some(icons::NUMERIC),
        "map-marker-remove-variant" => Some(icons::MAP_MARKER_REMOVE_VARIANT),
        "handball" => Some(icons::HANDBALL),
        "application-brackets-outline" => Some(icons::APPLICATION_BRACKETS_OUTLINE),
        "axe-battle" => Some(icons::AXE_BATTLE),
        "alpha-u-circle-outline" => Some(icons::ALPHA_U_CIRCLE_OUTLINE),
        "balcony" => Some(icons::BALCONY),
        "nutrition" => Some(icons::NUTRITION),
        "image-album" => Some(icons::IMAGE_ALBUM),
        "briefcase-check-outline" => Some(icons::BRIEFCASE_CHECK_OUTLINE),
        "store-edit-outline" => Some(icons::STORE_EDIT_OUTLINE),
        "checkerboard-minus" => Some(icons::CHECKERBOARD_MINUS),
        "music-note-bluetooth" => Some(icons::MUSIC_NOTE_BLUETOOTH),
        "backburger" => Some(icons::BACKBURGER),
        "window-restore" => Some(icons::WINDOW_RESTORE),
        "virus-off" => Some(icons::VIRUS_OFF),
        "size-xs" => Some(icons::SIZE_XS),
        "set-square" => Some(icons::SET_SQUARE),
        "home-switch" => Some(icons::HOME_SWITCH),
        "basket-remove-outline" => Some(icons::BASKET_REMOVE_OUTLINE),
        "bucket-outline" => Some(icons::BUCKET_OUTLINE),
        "panorama-horizontal-outline" => Some(icons::PANORAMA_HORIZONTAL_OUTLINE),
        "credit-card-refund" => Some(icons::CREDIT_CARD_REFUND),
        "tablet-cellphone" => Some(icons::TABLET_CELLPHONE),
        "border-horizontal" => Some(icons::BORDER_HORIZONTAL),
        "eye-minus" => Some(icons::EYE_MINUS),
        "comment-search" => Some(icons::COMMENT_SEARCH),
        "pump" => Some(icons::PUMP),
        "kettle-outline" => Some(icons::KETTLE_OUTLINE),
        "radiator-disabled" => Some(icons::RADIATOR_DISABLED),
        "database-alert" => Some(icons::DATABASE_ALERT),
        "gamepad-round-left" => Some(icons::GAMEPAD_ROUND_LEFT),
        "order-bool-descending-variant" => Some(icons::ORDER_BOOL_DESCENDING_VARIANT),
        "toy-brick-remove-outline" => Some(icons::TOY_BRICK_REMOVE_OUTLINE),
        "sort-bool-ascending" => Some(icons::SORT_BOOL_ASCENDING),
        "silo" => Some(icons::SILO),
        "file-document-edit" => Some(icons::FILE_DOCUMENT_EDIT),
        "tag-off-outline" => Some(icons::TAG_OFF_OUTLINE),
        "arrow-all" => Some(icons::ARROW_ALL),
        "play-pause" => Some(icons::PLAY_PAUSE),
        "database-sync" => Some(icons::DATABASE_SYNC),
        "glass-stange" => Some(icons::GLASS_STANGE),
        "share-off-outline" => Some(icons::SHARE_OFF_OUTLINE),
        "alert-octagram-outline" => Some(icons::ALERT_OCTAGRAM_OUTLINE),
        "grass" => Some(icons::GRASS),
        "account-wrench-outline" => Some(icons::ACCOUNT_WRENCH_OUTLINE),
        "comment-quote" => Some(icons::COMMENT_QUOTE),
        "alphabet-aurebesh" => Some(icons::ALPHABET_AUREBESH),
        "lock-check" => Some(icons::LOCK_CHECK),
        "play-network" => Some(icons::PLAY_NETWORK),
        "sticker-remove" => Some(icons::STICKER_REMOVE),
        "menu-down-outline" => Some(icons::MENU_DOWN_OUTLINE),
        "memory-arrow-down" => Some(icons::MEMORY_ARROW_DOWN),
        "format-letter-spacing-variant" => Some(icons::FORMAT_LETTER_SPACING_VARIANT),
        "hamburger-remove" => Some(icons::HAMBURGER_REMOVE),
        "clipboard-text-clock-outline" => Some(icons::CLIPBOARD_TEXT_CLOCK_OUTLINE),
        "radio-fm" => Some(icons::RADIO_FM),
        "podium-gold" => Some(icons::PODIUM_GOLD),
        #[allow(deprecated)]
        "apple" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple' is deprecated.").print(py);
            }
            Some(icons::APPLE)
        }
        "arrow-top-right-thin" => Some(icons::ARROW_TOP_RIGHT_THIN),
        "lock-percent-outline" => Some(icons::LOCK_PERCENT_OUTLINE),
        "clock-time-seven-outline" => Some(icons::CLOCK_TIME_SEVEN_OUTLINE),
        "auto-upload" => Some(icons::AUTO_UPLOAD),
        "lightning-bolt-outline" => Some(icons::LIGHTNING_BOLT_OUTLINE),
        "car-limousine" => Some(icons::CAR_LIMOUSINE),
        "timer-pause" => Some(icons::TIMER_PAUSE),
        "arrow-up-bold-outline" => Some(icons::ARROW_UP_BOLD_OUTLINE),
        "shield-refresh-outline" => Some(icons::SHIELD_REFRESH_OUTLINE),
        "shark-fin" => Some(icons::SHARK_FIN),
        "arrow-top-left-thick" => Some(icons::ARROW_TOP_LEFT_THICK),
        "check-underline-circle-outline" => Some(icons::CHECK_UNDERLINE_CIRCLE_OUTLINE),
        "ray-start-arrow" => Some(icons::RAY_START_ARROW),
        #[allow(deprecated)]
        "fedora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'fedora' is deprecated.").print(py);
            }
            Some(icons::FEDORA)
        }
        "pail-off" => Some(icons::PAIL_OFF),
        "download-multiple" => Some(icons::DOWNLOAD_MULTIPLE),
        "arrow-top-left-bold-outline" => Some(icons::ARROW_TOP_LEFT_BOLD_OUTLINE),
        "camera-front-variant" => Some(icons::CAMERA_FRONT_VARIANT),
        "delete-alert-outline" => Some(icons::DELETE_ALERT_OUTLINE),
        "relation-one-or-many-to-zero-or-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ZERO_OR_ONE),
        "pound" => Some(icons::POUND),
        "folder-settings-outline" => Some(icons::FOLDER_SETTINGS_OUTLINE),
        "bell-ring-outline" => Some(icons::BELL_RING_OUTLINE),
        "shield-check-outline" => Some(icons::SHIELD_CHECK_OUTLINE),
        "cart-arrow-right" => Some(icons::CART_ARROW_RIGHT),
        "view-grid-outline" => Some(icons::VIEW_GRID_OUTLINE),
        "account-tie" => Some(icons::ACCOUNT_TIE),
        "safety-goggles" => Some(icons::SAFETY_GOGGLES),
        "lock-minus" => Some(icons::LOCK_MINUS),
        "comment-eye-outline" => Some(icons::COMMENT_EYE_OUTLINE),
        "network" => Some(icons::NETWORK),
        "table-tennis" => Some(icons::TABLE_TENNIS),
        #[allow(deprecated)]
        "digital-ocean" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'digital-ocean' is deprecated.").print(py);
            }
            Some(icons::DIGITAL_OCEAN)
        }
        "math-tan" => Some(icons::MATH_TAN),
        "palette-swatch-variant" => Some(icons::PALETTE_SWATCH_VARIANT),
        "wave-arrow-up" => Some(icons::WAVE_ARROW_UP),
        "head-heart-outline" => Some(icons::HEAD_HEART_OUTLINE),
        "airport" => Some(icons::AIRPORT),
        "battery-minus-outline" => Some(icons::BATTERY_MINUS_OUTLINE),
        "earth-box-plus" => Some(icons::EARTH_BOX_PLUS),
        "bus-marker" => Some(icons::BUS_MARKER),
        "chart-bell-curve-cumulative" => Some(icons::CHART_BELL_CURVE_CUMULATIVE),
        "shape-plus" => Some(icons::SHAPE_PLUS),
        "content-save" => Some(icons::CONTENT_SAVE),
        "sun-angle-outline" => Some(icons::SUN_ANGLE_OUTLINE),
        "key-remove" => Some(icons::KEY_REMOVE),
        "receipt-text-check-outline" => Some(icons::RECEIPT_TEXT_CHECK_OUTLINE),
        "flag-checkered" => Some(icons::FLAG_CHECKERED),
        "send-variant-outline" => Some(icons::SEND_VARIANT_OUTLINE),
        "arrow-down-circle-outline" => Some(icons::ARROW_DOWN_CIRCLE_OUTLINE),
        "credit-card-wireless" => Some(icons::CREDIT_CARD_WIRELESS),
        "polo" => Some(icons::POLO),
        _ => None,
    }
}
