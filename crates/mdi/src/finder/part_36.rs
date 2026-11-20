// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_36(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "shield-alert" => Some(icons::SHIELD_ALERT),
        "wifi-strength-2-lock" => Some(icons::WIFI_STRENGTH_2_LOCK),
        "key-remove" => Some(icons::KEY_REMOVE),
        "window-maximize" => Some(icons::WINDOW_MAXIMIZE),
        "bookmark-minus" => Some(icons::BOOKMARK_MINUS),
        "credit-card-wireless-off-outline" => Some(icons::CREDIT_CARD_WIRELESS_OFF_OUTLINE),
        "tag-off-outline" => Some(icons::TAG_OFF_OUTLINE),
        "clipboard-search-outline" => Some(icons::CLIPBOARD_SEARCH_OUTLINE),
        "map-outline" => Some(icons::MAP_OUTLINE),
        "folder-multiple-plus" => Some(icons::FOLDER_MULTIPLE_PLUS),
        "chevron-triple-up" => Some(icons::CHEVRON_TRIPLE_UP),
        "invoice-clock-outline" => Some(icons::INVOICE_CLOCK_OUTLINE),
        "attachment" => Some(icons::ATTACHMENT),
        "bash" => Some(icons::BASH),
        "razor-double-edge" => Some(icons::RAZOR_DOUBLE_EDGE),
        "arrow-down-thin" => Some(icons::ARROW_DOWN_THIN),
        "camera-document-off" => Some(icons::CAMERA_DOCUMENT_OFF),
        #[allow(deprecated)]
        "minecraft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'minecraft' is deprecated.").print(py);
            }
            Some(icons::MINECRAFT)
        }
        "podium-gold" => Some(icons::PODIUM_GOLD),
        "vibrate-off" => Some(icons::VIBRATE_OFF),
        "toothbrush-paste" => Some(icons::TOOTHBRUSH_PASTE),
        "movie-open-minus-outline" => Some(icons::MOVIE_OPEN_MINUS_OUTLINE),
        "pin-off" => Some(icons::PIN_OFF),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-low" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-low' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_LOW)
        }
        "fruit-grapes-outline" => Some(icons::FRUIT_GRAPES_OUTLINE),
        "arrow-down-left" => Some(icons::ARROW_DOWN_LEFT),
        "sword-cross" => Some(icons::SWORD_CROSS),
        "format-float-left" => Some(icons::FORMAT_FLOAT_LEFT),
        "select-remove" => Some(icons::SELECT_REMOVE),
        "carabiner" => Some(icons::CARABINER),
        "hammer-sickle" => Some(icons::HAMMER_SICKLE),
        "wifi-cog" => Some(icons::WIFI_COG),
        "table-filter" => Some(icons::TABLE_FILTER),
        "earth-box-plus" => Some(icons::EARTH_BOX_PLUS),
        "bookmark-check-outline" => Some(icons::BOOKMARK_CHECK_OUTLINE),
        "hand-heart" => Some(icons::HAND_HEART),
        "account-convert-outline" => Some(icons::ACCOUNT_CONVERT_OUTLINE),
        "dice-d4-outline" => Some(icons::DICE_D4_OUTLINE),
        "timer-edit" => Some(icons::TIMER_EDIT),
        "snail" => Some(icons::SNAIL),
        "arrow-up-circle-outline" => Some(icons::ARROW_UP_CIRCLE_OUTLINE),
        "guitar-pick-outline" => Some(icons::GUITAR_PICK_OUTLINE),
        "credit-card-fast-outline" => Some(icons::CREDIT_CARD_FAST_OUTLINE),
        "notebook-remove-outline" => Some(icons::NOTEBOOK_REMOVE_OUTLINE),
        "hair-dryer-outline" => Some(icons::HAIR_DRYER_OUTLINE),
        "invoice-text-arrow-right" => Some(icons::INVOICE_TEXT_ARROW_RIGHT),
        "calendar-outline" => Some(icons::CALENDAR_OUTLINE),
        "human-female-female-child" => Some(icons::HUMAN_FEMALE_FEMALE_CHILD),
        "plus-box-multiple" => Some(icons::PLUS_BOX_MULTIPLE),
        "layers-search" => Some(icons::LAYERS_SEARCH),
        "square-rounded" => Some(icons::SQUARE_ROUNDED),
        "dishwasher" => Some(icons::DISHWASHER),
        "relation-one-to-zero-or-many" => Some(icons::RELATION_ONE_TO_ZERO_OR_MANY),
        "office-building-cog" => Some(icons::OFFICE_BUILDING_COG),
        "battery-20-bluetooth" => Some(icons::BATTERY_20_BLUETOOTH),
        "archive-settings" => Some(icons::ARCHIVE_SETTINGS),
        "message-question-outline" => Some(icons::MESSAGE_QUESTION_OUTLINE),
        "hand-pointing-right" => Some(icons::HAND_POINTING_RIGHT),
        "account-alert" => Some(icons::ACCOUNT_ALERT),
        "flag-variant" => Some(icons::FLAG_VARIANT),
        "soldering-iron" => Some(icons::SOLDERING_IRON),
        "mower-bag-on" => Some(icons::MOWER_BAG_ON),
        "file-clock-outline" => Some(icons::FILE_CLOCK_OUTLINE),
        "alarm-bell" => Some(icons::ALARM_BELL),
        "send-variant-outline" => Some(icons::SEND_VARIANT_OUTLINE),
        "home-floor-b" => Some(icons::HOME_FLOOR_B),
        "signal-hspa" => Some(icons::SIGNAL_HSPA),
        "watch" => Some(icons::WATCH),
        "book-account" => Some(icons::BOOK_ACCOUNT),
        "speaker-wireless" => Some(icons::SPEAKER_WIRELESS),
        "arrow-top-left-thin" => Some(icons::ARROW_TOP_LEFT_THIN),
        "movie-search-outline" => Some(icons::MOVIE_SEARCH_OUTLINE),
        "email-sync-outline" => Some(icons::EMAIL_SYNC_OUTLINE),
        "cross-outline" => Some(icons::CROSS_OUTLINE),
        "eye-lock-open-outline" => Some(icons::EYE_LOCK_OPEN_OUTLINE),
        "account-arrow-up-outline" => Some(icons::ACCOUNT_ARROW_UP_OUTLINE),
        "binoculars" => Some(icons::BINOCULARS),
        "shield-sun-outline" => Some(icons::SHIELD_SUN_OUTLINE),
        "camera-lock" => Some(icons::CAMERA_LOCK),
        "fire-extinguisher" => Some(icons::FIRE_EXTINGUISHER),
        "cash-100" => Some(icons::CASH_100),
        "bike" => Some(icons::BIKE),
        "battery-arrow-up-outline" => Some(icons::BATTERY_ARROW_UP_OUTLINE),
        "smoke-detector-variant-off" => Some(icons::SMOKE_DETECTOR_VARIANT_OFF),
        "fruit-pineapple" => Some(icons::FRUIT_PINEAPPLE),
        "axis-y-arrow-lock" => Some(icons::AXIS_Y_ARROW_LOCK),
        "battery-80-bluetooth" => Some(icons::BATTERY_80_BLUETOOTH),
        "car-back" => Some(icons::CAR_BACK),
        "email-multiple-outline" => Some(icons::EMAIL_MULTIPLE_OUTLINE),
        "dots-vertical-circle" => Some(icons::DOTS_VERTICAL_CIRCLE),
        "arrow-split-vertical" => Some(icons::ARROW_SPLIT_VERTICAL),
        "pipe-valve" => Some(icons::PIPE_VALVE),
        "folder-plus" => Some(icons::FOLDER_PLUS),
        "view-grid-outline" => Some(icons::VIEW_GRID_OUTLINE),
        "office-building-marker" => Some(icons::OFFICE_BUILDING_MARKER),
        "phone-log-outline" => Some(icons::PHONE_LOG_OUTLINE),
        "cellphone-message-off" => Some(icons::CELLPHONE_MESSAGE_OFF),
        "gesture-swipe-left" => Some(icons::GESTURE_SWIPE_LEFT),
        "database-arrow-right" => Some(icons::DATABASE_ARROW_RIGHT),
        "fruit-grapes" => Some(icons::FRUIT_GRAPES),
        "train-car-hopper-covered" => Some(icons::TRAIN_CAR_HOPPER_COVERED),
        "refresh" => Some(icons::REFRESH),
        "receipt-text-clock" => Some(icons::RECEIPT_TEXT_CLOCK),
        "unfold-more-vertical" => Some(icons::UNFOLD_MORE_VERTICAL),
        "sign-caution" => Some(icons::SIGN_CAUTION),
        "aspect-ratio" => Some(icons::ASPECT_RATIO),
        "decimal-comma" => Some(icons::DECIMAL_COMMA),
        "file-minus" => Some(icons::FILE_MINUS),
        "file-find-outline" => Some(icons::FILE_FIND_OUTLINE),
        "rocket-outline" => Some(icons::ROCKET_OUTLINE),
        "clock-time-eleven-outline" => Some(icons::CLOCK_TIME_ELEVEN_OUTLINE),
        "layers-edit" => Some(icons::LAYERS_EDIT),
        "numeric-3-circle-outline" => Some(icons::NUMERIC_3_CIRCLE_OUTLINE),
        "gas-cylinder" => Some(icons::GAS_CYLINDER),
        "distribute-horizontal-left" => Some(icons::DISTRIBUTE_HORIZONTAL_LEFT),
        "select-group" => Some(icons::SELECT_GROUP),
        "cog-box" => Some(icons::COG_BOX),
        "beaker-minus-outline" => Some(icons::BEAKER_MINUS_OUTLINE),
        "progress-download" => Some(icons::PROGRESS_DOWNLOAD),
        "water-check-outline" => Some(icons::WATER_CHECK_OUTLINE),
        "trackpad" => Some(icons::TRACKPAD),
        "truck-trailer" => Some(icons::TRUCK_TRAILER),
        "pipe-wrench" => Some(icons::PIPE_WRENCH),
        "comment-alert" => Some(icons::COMMENT_ALERT),
        "clipboard-clock-outline" => Some(icons::CLIPBOARD_CLOCK_OUTLINE),
        "book-arrow-right" => Some(icons::BOOK_ARROW_RIGHT),
        "movie-open-plus" => Some(icons::MOVIE_OPEN_PLUS),
        "water-boiler-off" => Some(icons::WATER_BOILER_OFF),
        "star-four-points-small" => Some(icons::STAR_FOUR_POINTS_SMALL),
        "eye-settings" => Some(icons::EYE_SETTINGS),
        "note-alert-outline" => Some(icons::NOTE_ALERT_OUTLINE),
        "view-sequential-outline" => Some(icons::VIEW_SEQUENTIAL_OUTLINE),
        "calendar-start" => Some(icons::CALENDAR_START),
        "pencil-box-outline" => Some(icons::PENCIL_BOX_OUTLINE),
        "ski" => Some(icons::SKI),
        "code-block-brackets" => Some(icons::CODE_BLOCK_BRACKETS),
        "tag-arrow-down" => Some(icons::TAG_ARROW_DOWN),
        "skate" => Some(icons::SKATE),
        "briefcase-search-outline" => Some(icons::BRIEFCASE_SEARCH_OUTLINE),
        "purse-outline" => Some(icons::PURSE_OUTLINE),
        "flag-remove" => Some(icons::FLAG_REMOVE),
        "notification-clear-all" => Some(icons::NOTIFICATION_CLEAR_ALL),
        #[allow(deprecated)]
        "symfony" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'symfony' is deprecated.").print(py);
            }
            Some(icons::SYMFONY)
        }
        "clock-alert-outline" => Some(icons::CLOCK_ALERT_OUTLINE),
        "tooltip-image" => Some(icons::TOOLTIP_IMAGE),
        "alpha-j" => Some(icons::ALPHA_J),
        "card-multiple-outline" => Some(icons::CARD_MULTIPLE_OUTLINE),
        "currency-eur" => Some(icons::CURRENCY_EUR),
        "network-strength-4-alert" => Some(icons::NETWORK_STRENGTH_4_ALERT),
        "virus" => Some(icons::VIRUS),
        "link-variant-remove" => Some(icons::LINK_VARIANT_REMOVE),
        "comment-flash" => Some(icons::COMMENT_FLASH),
        "storefront" => Some(icons::STOREFRONT),
        "invoice-minus-outline" => Some(icons::INVOICE_MINUS_OUTLINE),
        "slope-downhill" => Some(icons::SLOPE_DOWNHILL),
        "checkbox-blank-outline" => Some(icons::CHECKBOX_BLANK_OUTLINE),
        "arrow-u-down-right" => Some(icons::ARROW_U_DOWN_RIGHT),
        #[allow(deprecated)]
        "xamarin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xamarin' is deprecated.").print(py);
            }
            Some(icons::XAMARIN)
        }
        "paw-off-outline" => Some(icons::PAW_OFF_OUTLINE),
        "folder-arrow-left" => Some(icons::FOLDER_ARROW_LEFT),
        "comment-edit-outline" => Some(icons::COMMENT_EDIT_OUTLINE),
        "star-box-multiple-outline" => Some(icons::STAR_BOX_MULTIPLE_OUTLINE),
        "close-octagon" => Some(icons::CLOSE_OCTAGON),
        "flask-plus" => Some(icons::FLASK_PLUS),
        "church-outline" => Some(icons::CHURCH_OUTLINE),
        "numeric-9-box-multiple-outline" => Some(icons::NUMERIC_9_BOX_MULTIPLE_OUTLINE),
        "rv-truck" => Some(icons::RV_TRUCK),
        "size-xxl" => Some(icons::SIZE_XXL),
        "waves" => Some(icons::WAVES),
        "party-popper" => Some(icons::PARTY_POPPER),
        "cog-pause-outline" => Some(icons::COG_PAUSE_OUTLINE),
        "arrow-up-thick" => Some(icons::ARROW_UP_THICK),
        "truck-cargo-container" => Some(icons::TRUCK_CARGO_CONTAINER),
        "star-four-points-box-outline" => Some(icons::STAR_FOUR_POINTS_BOX_OUTLINE),
        "delete-off" => Some(icons::DELETE_OFF),
        "sticker-plus" => Some(icons::STICKER_PLUS),
        #[allow(deprecated)]
        "open-source-initiative" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'open-source-initiative' is deprecated.")
                    .print(py);
            }
            Some(icons::OPEN_SOURCE_INITIATIVE)
        }
        "numeric-5-box-multiple" => Some(icons::NUMERIC_5_BOX_MULTIPLE),
        "lightbulb-multiple" => Some(icons::LIGHTBULB_MULTIPLE),
        "wifi-arrow-up" => Some(icons::WIFI_ARROW_UP),
        "lead-pencil" => Some(icons::LEAD_PENCIL),
        "bookmark-box-multiple-outline" => Some(icons::BOOKMARK_BOX_MULTIPLE_OUTLINE),
        "keyboard-esc" => Some(icons::KEYBOARD_ESC),
        "currency-ils" => Some(icons::CURRENCY_ILS),
        "snowflake" => Some(icons::SNOWFLAKE),
        "greater-than" => Some(icons::GREATER_THAN),
        "allergy" => Some(icons::ALLERGY),
        "dice-d8-outline" => Some(icons::DICE_D8_OUTLINE),
        #[allow(deprecated)]
        "graphql" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'graphql' is deprecated.").print(py);
            }
            Some(icons::GRAPHQL)
        }
        "cash-check" => Some(icons::CASH_CHECK),
        "file-document-multiple-outline" => Some(icons::FILE_DOCUMENT_MULTIPLE_OUTLINE),
        "numeric-1" => Some(icons::NUMERIC_1),
        "home-floor-negative-1" => Some(icons::HOME_FLOOR_NEGATIVE_1),
        "music-note" => Some(icons::MUSIC_NOTE),
        "numeric-3" => Some(icons::NUMERIC_3),
        "arrow-all" => Some(icons::ARROW_ALL),
        "account-supervisor" => Some(icons::ACCOUNT_SUPERVISOR),
        "database-search" => Some(icons::DATABASE_SEARCH),
        "leaf-maple" => Some(icons::LEAF_MAPLE),
        "sign-yield" => Some(icons::SIGN_YIELD),
        _ => None,
    }
}
