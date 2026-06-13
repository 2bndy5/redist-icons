// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_7(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "book-open" => Some(icons::BOOK_OPEN),
        "puzzle-heart" => Some(icons::PUZZLE_HEART),
        "roman-numeral-9" => Some(icons::ROMAN_NUMERAL_9),
        "temperature-celsius" => Some(icons::TEMPERATURE_CELSIUS),
        "boomerang" => Some(icons::BOOMERANG),
        "calendar-refresh-outline" => Some(icons::CALENDAR_REFRESH_OUTLINE),
        "home-city" => Some(icons::HOME_CITY),
        "hand-peace-variant" => Some(icons::HAND_PEACE_VARIANT),
        "diving-flippers" => Some(icons::DIVING_FLIPPERS),
        "vector-point-plus" => Some(icons::VECTOR_POINT_PLUS),
        "emoticon-cool-outline" => Some(icons::EMOTICON_COOL_OUTLINE),
        "publish-off" => Some(icons::PUBLISH_OFF),
        "head-question" => Some(icons::HEAD_QUESTION),
        "pier" => Some(icons::PIER),
        "gamepad-round" => Some(icons::GAMEPAD_ROUND),
        "gesture-pinch" => Some(icons::GESTURE_PINCH),
        "account-tie-voice-outline" => Some(icons::ACCOUNT_TIE_VOICE_OUTLINE),
        "tooltip-cellphone" => Some(icons::TOOLTIP_CELLPHONE),
        "tune-vertical" => Some(icons::TUNE_VERTICAL),
        "emoticon-angry" => Some(icons::EMOTICON_ANGRY),
        "snowflake-alert" => Some(icons::SNOWFLAKE_ALERT),
        "information-slab-circle" => Some(icons::INFORMATION_SLAB_CIRCLE),
        "lock-open-alert-outline" => Some(icons::LOCK_OPEN_ALERT_OUTLINE),
        "nature-people" => Some(icons::NATURE_PEOPLE),
        "clock-time-seven-outline" => Some(icons::CLOCK_TIME_SEVEN_OUTLINE),
        "contain-start" => Some(icons::CONTAIN_START),
        "bookmark-multiple-outline" => Some(icons::BOOKMARK_MULTIPLE_OUTLINE),
        "email-open-multiple" => Some(icons::EMAIL_OPEN_MULTIPLE),
        "timer-remove-outline" => Some(icons::TIMER_REMOVE_OUTLINE),
        "license" => Some(icons::LICENSE),
        #[allow(deprecated)]
        "language-csharp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-csharp' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_CSHARP)
        }
        "format-text-variant" => Some(icons::FORMAT_TEXT_VARIANT),
        "bowl-mix" => Some(icons::BOWL_MIX),
        "size-s" => Some(icons::SIZE_S),
        "movie-check" => Some(icons::MOVIE_CHECK),
        "wifi-strength-lock-open-outline" => Some(icons::WIFI_STRENGTH_LOCK_OPEN_OUTLINE),
        #[allow(deprecated)]
        "google-circles-extended" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles-extended' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_EXTENDED)
        }
        "numeric-7" => Some(icons::NUMERIC_7),
        "relation-only-one-to-one" => Some(icons::RELATION_ONLY_ONE_TO_ONE),
        "file-powerpoint-box-outline" => Some(icons::FILE_POWERPOINT_BOX_OUTLINE),
        "shark-off" => Some(icons::SHARK_OFF),
        "arrow-split-vertical" => Some(icons::ARROW_SPLIT_VERTICAL),
        "lightbulb-group-off" => Some(icons::LIGHTBULB_GROUP_OFF),
        "upload-multiple" => Some(icons::UPLOAD_MULTIPLE),
        "music-note" => Some(icons::MUSIC_NOTE),
        "numeric-5-box-multiple-outline" => Some(icons::NUMERIC_5_BOX_MULTIPLE_OUTLINE),
        "filter-cog" => Some(icons::FILTER_COG),
        "glass-cocktail-off" => Some(icons::GLASS_COCKTAIL_OFF),
        "account-injury-outline" => Some(icons::ACCOUNT_INJURY_OUTLINE),
        "tie" => Some(icons::TIE),
        "keyboard-caps" => Some(icons::KEYBOARD_CAPS),
        "battery-heart-outline" => Some(icons::BATTERY_HEART_OUTLINE),
        "advertisements" => Some(icons::ADVERTISEMENTS),
        "kettle" => Some(icons::KETTLE),
        "om" => Some(icons::OM),
        "vector-square-remove" => Some(icons::VECTOR_SQUARE_REMOVE),
        "beta" => Some(icons::BETA),
        "robot-mower-outline" => Some(icons::ROBOT_MOWER_OUTLINE),
        "message-check" => Some(icons::MESSAGE_CHECK),
        "magnify-remove-outline" => Some(icons::MAGNIFY_REMOVE_OUTLINE),
        "dots-vertical-circle-outline" => Some(icons::DOTS_VERTICAL_CIRCLE_OUTLINE),
        "image-broken" => Some(icons::IMAGE_BROKEN),
        "home-roof" => Some(icons::HOME_ROOF),
        "format-text-rotation-up" => Some(icons::FORMAT_TEXT_ROTATION_UP),
        "page-last" => Some(icons::PAGE_LAST),
        "meditation" => Some(icons::MEDITATION),
        "numeric-3-circle" => Some(icons::NUMERIC_3_CIRCLE),
        "file-table-box-multiple" => Some(icons::FILE_TABLE_BOX_MULTIPLE),
        "chart-bubble" => Some(icons::CHART_BUBBLE),
        "calendar-import-outline" => Some(icons::CALENDAR_IMPORT_OUTLINE),
        "clock-minus" => Some(icons::CLOCK_MINUS),
        "bluetooth-off" => Some(icons::BLUETOOTH_OFF),
        "file-png-box" => Some(icons::FILE_PNG_BOX),
        "folder-arrow-up-down-outline" => Some(icons::FOLDER_ARROW_UP_DOWN_OUTLINE),
        "check-circle-outline" => Some(icons::CHECK_CIRCLE_OUTLINE),
        "rollerblade" => Some(icons::ROLLERBLADE),
        "file-image" => Some(icons::FILE_IMAGE),
        "timer-cancel" => Some(icons::TIMER_CANCEL),
        "numeric-9-plus-circle" => Some(icons::NUMERIC_9_PLUS_CIRCLE),
        "account-child-outline" => Some(icons::ACCOUNT_CHILD_OUTLINE),
        "message-arrow-left-outline" => Some(icons::MESSAGE_ARROW_LEFT_OUTLINE),
        "label-off" => Some(icons::LABEL_OFF),
        "clipboard-minus" => Some(icons::CLIPBOARD_MINUS),
        "page-layout-footer" => Some(icons::PAGE_LAYOUT_FOOTER),
        "file-edit" => Some(icons::FILE_EDIT),
        "battery-charging-wireless" => Some(icons::BATTERY_CHARGING_WIRELESS),
        "numeric-3-box-outline" => Some(icons::NUMERIC_3_BOX_OUTLINE),
        "bank-off-outline" => Some(icons::BANK_OFF_OUTLINE),
        "alpha-m-circle" => Some(icons::ALPHA_M_CIRCLE),
        "bookmark-box-multiple" => Some(icons::BOOKMARK_BOX_MULTIPLE),
        "chevron-double-up" => Some(icons::CHEVRON_DOUBLE_UP),
        "pause-box" => Some(icons::PAUSE_BOX),
        "tag-off" => Some(icons::TAG_OFF),
        "barcode" => Some(icons::BARCODE),
        "skip-next-circle" => Some(icons::SKIP_NEXT_CIRCLE),
        "code-less-than" => Some(icons::CODE_LESS_THAN),
        "currency-twd" => Some(icons::CURRENCY_TWD),
        "calendar-week" => Some(icons::CALENDAR_WEEK),
        #[allow(deprecated)]
        "dlna" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dlna' is deprecated.").print(py);
            }
            Some(icons::DLNA)
        }
        "whistle" => Some(icons::WHISTLE),
        "book-cog-outline" => Some(icons::BOOK_COG_OUTLINE),
        "account-file-outline" => Some(icons::ACCOUNT_FILE_OUTLINE),
        "comment-off-outline" => Some(icons::COMMENT_OFF_OUTLINE),
        "hvac-off" => Some(icons::HVAC_OFF),
        "robot-excited" => Some(icons::ROBOT_EXCITED),
        #[allow(deprecated)]
        "meteor" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'meteor' is deprecated.").print(py);
            }
            Some(icons::METEOR)
        }
        #[allow(deprecated)]
        "deviantart" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'deviantart' is deprecated.").print(py);
            }
            Some(icons::DEVIANTART)
        }
        "truck" => Some(icons::TRUCK),
        "music-note-eighth" => Some(icons::MUSIC_NOTE_EIGHTH),
        "data-matrix-remove" => Some(icons::DATA_MATRIX_REMOVE),
        "truck-trailer" => Some(icons::TRUCK_TRAILER),
        "folder-move" => Some(icons::FOLDER_MOVE),
        "keyboard-tab-reverse" => Some(icons::KEYBOARD_TAB_REVERSE),
        "view-carousel-outline" => Some(icons::VIEW_CAROUSEL_OUTLINE),
        #[allow(deprecated)]
        "debian" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'debian' is deprecated.").print(py);
            }
            Some(icons::DEBIAN)
        }
        "clipboard-text" => Some(icons::CLIPBOARD_TEXT),
        "mailbox-up-outline" => Some(icons::MAILBOX_UP_OUTLINE),
        "printer-pos-stop-outline" => Some(icons::PRINTER_POS_STOP_OUTLINE),
        "check-decagram-outline" => Some(icons::CHECK_DECAGRAM_OUTLINE),
        "camera-image" => Some(icons::CAMERA_IMAGE),
        "train-car-passenger-door" => Some(icons::TRAIN_CAR_PASSENGER_DOOR),
        "radioactive-circle-outline" => Some(icons::RADIOACTIVE_CIRCLE_OUTLINE),
        "selection-marker" => Some(icons::SELECTION_MARKER),
        #[allow(deprecated)]
        "ubisoft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ubisoft' is deprecated.").print(py);
            }
            Some(icons::UBISOFT)
        }
        "dots-hexagon" => Some(icons::DOTS_HEXAGON),
        "border-none-variant" => Some(icons::BORDER_NONE_VARIANT),
        "calendar-search" => Some(icons::CALENDAR_SEARCH),
        "flag-variant-minus" => Some(icons::FLAG_VARIANT_MINUS),
        "key-plus" => Some(icons::KEY_PLUS),
        "skip-previous-outline" => Some(icons::SKIP_PREVIOUS_OUTLINE),
        "thermometer-minus" => Some(icons::THERMOMETER_MINUS),
        "hazard-lights" => Some(icons::HAZARD_LIGHTS),
        "moon-waxing-gibbous" => Some(icons::MOON_WAXING_GIBBOUS),
        "distribute-vertical-center" => Some(icons::DISTRIBUTE_VERTICAL_CENTER),
        "arrow-up-circle-outline" => Some(icons::ARROW_UP_CIRCLE_OUTLINE),
        "bottle-wine" => Some(icons::BOTTLE_WINE),
        "music-circle" => Some(icons::MUSIC_CIRCLE),
        "content-duplicate" => Some(icons::CONTENT_DUPLICATE),
        "window-minimize" => Some(icons::WINDOW_MINIMIZE),
        "sort-alphabetical-descending" => Some(icons::SORT_ALPHABETICAL_DESCENDING),
        "van-passenger" => Some(icons::VAN_PASSENGER),
        "coolant-temperature" => Some(icons::COOLANT_TEMPERATURE),
        "calendar-export-outline" => Some(icons::CALENDAR_EXPORT_OUTLINE),
        "wallet-outline" => Some(icons::WALLET_OUTLINE),
        "account-network-off" => Some(icons::ACCOUNT_NETWORK_OFF),
        "tower-beach" => Some(icons::TOWER_BEACH),
        "exponent" => Some(icons::EXPONENT),
        "size-xxs" => Some(icons::SIZE_XXS),
        "file-delimited-outline" => Some(icons::FILE_DELIMITED_OUTLINE),
        "star-plus" => Some(icons::STAR_PLUS),
        "arrow-left-right-bold" => Some(icons::ARROW_LEFT_RIGHT_BOLD),
        "format-text-wrapping-clip" => Some(icons::FORMAT_TEXT_WRAPPING_CLIP),
        "home-percent" => Some(icons::HOME_PERCENT),
        "keyboard-settings" => Some(icons::KEYBOARD_SETTINGS),
        "delete-clock" => Some(icons::DELETE_CLOCK),
        "account-check-outline" => Some(icons::ACCOUNT_CHECK_OUTLINE),
        "store-clock" => Some(icons::STORE_CLOCK),
        "hand-wave" => Some(icons::HAND_WAVE),
        "eye-minus" => Some(icons::EYE_MINUS),
        "format-list-checks" => Some(icons::FORMAT_LIST_CHECKS),
        "fleur-de-lis" => Some(icons::FLEUR_DE_LIS),
        "tooltip-edit-outline" => Some(icons::TOOLTIP_EDIT_OUTLINE),
        "monitor-vertical" => Some(icons::MONITOR_VERTICAL),
        "police-badge-outline" => Some(icons::POLICE_BADGE_OUTLINE),
        "earth-arrow-right" => Some(icons::EARTH_ARROW_RIGHT),
        "cast-audio-variant" => Some(icons::CAST_AUDIO_VARIANT),
        "shield-lock-open-outline" => Some(icons::SHIELD_LOCK_OPEN_OUTLINE),
        "star-minus" => Some(icons::STAR_MINUS),
        "tag-edit" => Some(icons::TAG_EDIT),
        "invoice-multiple" => Some(icons::INVOICE_MULTIPLE),
        "swap-vertical-circle-outline" => Some(icons::SWAP_VERTICAL_CIRCLE_OUTLINE),
        "hvac" => Some(icons::HVAC),
        "magazine-pistol" => Some(icons::MAGAZINE_PISTOL),
        #[allow(deprecated)]
        "google-hangouts" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-hangouts' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_HANGOUTS)
        }
        "view-grid" => Some(icons::VIEW_GRID),
        "lightbulb-multiple-off" => Some(icons::LIGHTBULB_MULTIPLE_OFF),
        "map-marker-alert-outline" => Some(icons::MAP_MARKER_ALERT_OUTLINE),
        "wind-turbine-alert" => Some(icons::WIND_TURBINE_ALERT),
        "credit-card-sync-outline" => Some(icons::CREDIT_CARD_SYNC_OUTLINE),
        "television-classic" => Some(icons::TELEVISION_CLASSIC),
        "surround-sound-5-1" => Some(icons::SURROUND_SOUND_5_1),
        "numeric-9-circle" => Some(icons::NUMERIC_9_CIRCLE),
        "ethernet-off" => Some(icons::ETHERNET_OFF),
        "numeric-0-box" => Some(icons::NUMERIC_0_BOX),
        "tooltip-check-outline" => Some(icons::TOOLTIP_CHECK_OUTLINE),
        "desk-lamp-on" => Some(icons::DESK_LAMP_ON),
        "filter-minus-outline" => Some(icons::FILTER_MINUS_OUTLINE),
        "navigation-variant" => Some(icons::NAVIGATION_VARIANT),
        "cog-refresh-outline" => Some(icons::COG_REFRESH_OUTLINE),
        "bed-king" => Some(icons::BED_KING),
        "application-edit-outline" => Some(icons::APPLICATION_EDIT_OUTLINE),
        "arrow-top-left-bold-box" => Some(icons::ARROW_TOP_LEFT_BOLD_BOX),
        "table-network" => Some(icons::TABLE_NETWORK),
        "code-block-braces" => Some(icons::CODE_BLOCK_BRACES),
        "stadium" => Some(icons::STADIUM),
        "clipboard-pulse" => Some(icons::CLIPBOARD_PULSE),
        "toothbrush" => Some(icons::TOOTHBRUSH),
        #[allow(deprecated)]
        "language-php" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-php' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_PHP)
        }
        "image-search" => Some(icons::IMAGE_SEARCH),
        "cellphone-lock" => Some(icons::CELLPHONE_LOCK),
        _ => None,
    }
}
