// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_13(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "account-cog" => Some(icons::ACCOUNT_COG),
        "card-account-phone-outline" => Some(icons::CARD_ACCOUNT_PHONE_OUTLINE),
        "gate-not" => Some(icons::GATE_NOT),
        "database-minus" => Some(icons::DATABASE_MINUS),
        "briefcase-search" => Some(icons::BRIEFCASE_SEARCH),
        "storefront-remove-outline" => Some(icons::STOREFRONT_REMOVE_OUTLINE),
        "car-lifted-pickup" => Some(icons::CAR_LIFTED_PICKUP),
        "flag" => Some(icons::FLAG),
        "oar" => Some(icons::OAR),
        "floor-lamp-torchiere-variant" => Some(icons::FLOOR_LAMP_TORCHIERE_VARIANT),
        "comment-arrow-right-outline" => Some(icons::COMMENT_ARROW_RIGHT_OUTLINE),
        "percent-box" => Some(icons::PERCENT_BOX),
        "router-network" => Some(icons::ROUTER_NETWORK),
        "stairs-down" => Some(icons::STAIRS_DOWN),
        "bullet" => Some(icons::BULLET),
        "audio-input-rca" => Some(icons::AUDIO_INPUT_RCA),
        "archive-cancel" => Some(icons::ARCHIVE_CANCEL),
        "human-female-girl" => Some(icons::HUMAN_FEMALE_GIRL),
        "menu-swap-outline" => Some(icons::MENU_SWAP_OUTLINE),
        "alpha-x-circle-outline" => Some(icons::ALPHA_X_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "bulma" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bulma' is deprecated.").print(py);
            }
            Some(icons::BULMA)
        }
        "swim" => Some(icons::SWIM),
        "bag-personal-off-outline" => Some(icons::BAG_PERSONAL_OFF_OUTLINE),
        "shield-half" => Some(icons::SHIELD_HALF),
        #[allow(deprecated)]
        "gentoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gentoo' is deprecated.").print(py);
            }
            Some(icons::GENTOO)
        }
        "domain-plus" => Some(icons::DOMAIN_PLUS),
        "decimal" => Some(icons::DECIMAL),
        "light-switch-off" => Some(icons::LIGHT_SWITCH_OFF),
        "flip-horizontal" => Some(icons::FLIP_HORIZONTAL),
        "content-duplicate" => Some(icons::CONTENT_DUPLICATE),
        "flashlight" => Some(icons::FLASHLIGHT),
        "boom-gate-outline" => Some(icons::BOOM_GATE_OUTLINE),
        "hand-back-right" => Some(icons::HAND_BACK_RIGHT),
        "shield-star" => Some(icons::SHIELD_STAR),
        "wrench-check" => Some(icons::WRENCH_CHECK),
        "database-search" => Some(icons::DATABASE_SEARCH),
        "comment-processing" => Some(icons::COMMENT_PROCESSING),
        "clock-time-three" => Some(icons::CLOCK_TIME_THREE),
        "alpha-y-circle-outline" => Some(icons::ALPHA_Y_CIRCLE_OUTLINE),
        "chart-line-variant" => Some(icons::CHART_LINE_VARIANT),
        "sort-reverse-variant" => Some(icons::SORT_REVERSE_VARIANT),
        "snowman" => Some(icons::SNOWMAN),
        "speaker-play" => Some(icons::SPEAKER_PLAY),
        "touch-text-outline" => Some(icons::TOUCH_TEXT_OUTLINE),
        "order-numeric-ascending" => Some(icons::ORDER_NUMERIC_ASCENDING),
        #[allow(deprecated)]
        "google-cloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cloud' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CLOUD)
        }
        "water" => Some(icons::WATER),
        "cellphone-message-off" => Some(icons::CELLPHONE_MESSAGE_OFF),
        "movie-open-minus-outline" => Some(icons::MOVIE_OPEN_MINUS_OUTLINE),
        "timer-check" => Some(icons::TIMER_CHECK),
        "wrap-disabled" => Some(icons::WRAP_DISABLED),
        "bell-circle" => Some(icons::BELL_CIRCLE),
        "wifi-strength-lock-outline" => Some(icons::WIFI_STRENGTH_LOCK_OUTLINE),
        "format-list-bulleted-square" => Some(icons::FORMAT_LIST_BULLETED_SQUARE),
        "factory" => Some(icons::FACTORY),
        "weather-tornado" => Some(icons::WEATHER_TORNADO),
        "download-outline" => Some(icons::DOWNLOAD_OUTLINE),
        "format-underline" => Some(icons::FORMAT_UNDERLINE),
        "text-recognition" => Some(icons::TEXT_RECOGNITION),
        "weather-hazy" => Some(icons::WEATHER_HAZY),
        "close-box-multiple-outline" => Some(icons::CLOSE_BOX_MULTIPLE_OUTLINE),
        "elevator-down" => Some(icons::ELEVATOR_DOWN),
        "food-outline" => Some(icons::FOOD_OUTLINE),
        "mirror-variant" => Some(icons::MIRROR_VARIANT),
        "numeric-10-circle" => Some(icons::NUMERIC_10_CIRCLE),
        "barcode-scan" => Some(icons::BARCODE_SCAN),
        "comment-bookmark" => Some(icons::COMMENT_BOOKMARK),
        "size-xxl" => Some(icons::SIZE_XXL),
        "star-box-multiple-outline" => Some(icons::STAR_BOX_MULTIPLE_OUTLINE),
        "clipboard-check-multiple" => Some(icons::CLIPBOARD_CHECK_MULTIPLE),
        "abacus" => Some(icons::ABACUS),
        "bag-personal-tag-outline" => Some(icons::BAG_PERSONAL_TAG_OUTLINE),
        "car-outline" => Some(icons::CAR_OUTLINE),
        "note-search" => Some(icons::NOTE_SEARCH),
        "robot-dead" => Some(icons::ROBOT_DEAD),
        "tray-alert" => Some(icons::TRAY_ALERT),
        "magazine-rifle" => Some(icons::MAGAZINE_RIFLE),
        "calendar-star-outline" => Some(icons::CALENDAR_STAR_OUTLINE),
        "clipboard-account" => Some(icons::CLIPBOARD_ACCOUNT),
        "timer-off" => Some(icons::TIMER_OFF),
        "link-variant" => Some(icons::LINK_VARIANT),
        "playlist-check" => Some(icons::PLAYLIST_CHECK),
        "head-cog-outline" => Some(icons::HEAD_COG_OUTLINE),
        "printer-pos" => Some(icons::PRINTER_POS),
        "volume-variant-off" => Some(icons::VOLUME_VARIANT_OFF),
        "guitar-acoustic" => Some(icons::GUITAR_ACOUSTIC),
        "note-text-outline" => Some(icons::NOTE_TEXT_OUTLINE),
        "chevron-left-box" => Some(icons::CHEVRON_LEFT_BOX),
        "ring" => Some(icons::RING),
        "plus-network" => Some(icons::PLUS_NETWORK),
        "sticker-circle-outline" => Some(icons::STICKER_CIRCLE_OUTLINE),
        "car-arrow-right" => Some(icons::CAR_ARROW_RIGHT),
        "chat-plus-outline" => Some(icons::CHAT_PLUS_OUTLINE),
        "book-arrow-left-outline" => Some(icons::BOOK_ARROW_LEFT_OUTLINE),
        "passport-plus" => Some(icons::PASSPORT_PLUS),
        "arrow-u-down-left" => Some(icons::ARROW_U_DOWN_LEFT),
        "crosshairs-question" => Some(icons::CROSSHAIRS_QUESTION),
        "jellyfish" => Some(icons::JELLYFISH),
        "robot-outline" => Some(icons::ROBOT_OUTLINE),
        "folder-heart-outline" => Some(icons::FOLDER_HEART_OUTLINE),
        "cloud-tags" => Some(icons::CLOUD_TAGS),
        "music-note" => Some(icons::MUSIC_NOTE),
        #[allow(deprecated)]
        "jira" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jira' is deprecated.").print(py);
            }
            Some(icons::JIRA)
        }
        "star-four-points-outline" => Some(icons::STAR_FOUR_POINTS_OUTLINE),
        "earth-arrow-up" => Some(icons::EARTH_ARROW_UP),
        "border-right" => Some(icons::BORDER_RIGHT),
        "train-car-box-open" => Some(icons::TRAIN_CAR_BOX_OPEN),
        "scale-bathroom" => Some(icons::SCALE_BATHROOM),
        "oil-lamp" => Some(icons::OIL_LAMP),
        "clock-remove-outline" => Some(icons::CLOCK_REMOVE_OUTLINE),
        "code-block-tags" => Some(icons::CODE_BLOCK_TAGS),
        "transit-detour" => Some(icons::TRANSIT_DETOUR),
        "medal-outline" => Some(icons::MEDAL_OUTLINE),
        "source-commit-local" => Some(icons::SOURCE_COMMIT_LOCAL),
        "arrow-up-circle" => Some(icons::ARROW_UP_CIRCLE),
        "sort-alphabetical-descending-variant" => Some(icons::SORT_ALPHABETICAL_DESCENDING_VARIANT),
        "clock-time-twelve" => Some(icons::CLOCK_TIME_TWELVE),
        "book-edit" => Some(icons::BOOK_EDIT),
        "dots-circle" => Some(icons::DOTS_CIRCLE),
        "run-fast" => Some(icons::RUN_FAST),
        "unfold-less-horizontal" => Some(icons::UNFOLD_LESS_HORIZONTAL),
        "battery-30-bluetooth" => Some(icons::BATTERY_30_BLUETOOTH),
        "email-alert-outline" => Some(icons::EMAIL_ALERT_OUTLINE),
        "pizza" => Some(icons::PIZZA),
        "kayaking" => Some(icons::KAYAKING),
        "alpha-v-box" => Some(icons::ALPHA_V_BOX),
        "timer-lock-open" => Some(icons::TIMER_LOCK_OPEN),
        "mirror-rectangle" => Some(icons::MIRROR_RECTANGLE),
        "car-door-lock" => Some(icons::CAR_DOOR_LOCK),
        "eyedropper-plus" => Some(icons::EYEDROPPER_PLUS),
        "octagram-edit" => Some(icons::OCTAGRAM_EDIT),
        "wifi-strength-2-alert" => Some(icons::WIFI_STRENGTH_2_ALERT),
        "car-clutch" => Some(icons::CAR_CLUTCH),
        "camera-switch" => Some(icons::CAMERA_SWITCH),
        "car-wireless" => Some(icons::CAR_WIRELESS),
        "currency-eth" => Some(icons::CURRENCY_ETH),
        "weather-snowy-heavy" => Some(icons::WEATHER_SNOWY_HEAVY),
        "numeric-9-box-multiple-outline" => Some(icons::NUMERIC_9_BOX_MULTIPLE_OUTLINE),
        "chart-box-multiple-outline" => Some(icons::CHART_BOX_MULTIPLE_OUTLINE),
        "subtitles-outline" => Some(icons::SUBTITLES_OUTLINE),
        "gift-outline" => Some(icons::GIFT_OUTLINE),
        "trending-up" => Some(icons::TRENDING_UP),
        "earth-plus" => Some(icons::EARTH_PLUS),
        #[allow(deprecated)]
        "plex" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'plex' is deprecated.").print(py);
            }
            Some(icons::PLEX)
        }
        "calendar-edit-outline" => Some(icons::CALENDAR_EDIT_OUTLINE),
        "image-filter-drama" => Some(icons::IMAGE_FILTER_DRAMA),
        "arrow-up-down" => Some(icons::ARROW_UP_DOWN),
        "movie-open-check-outline" => Some(icons::MOVIE_OPEN_CHECK_OUTLINE),
        "numeric-3-box-multiple-outline" => Some(icons::NUMERIC_3_BOX_MULTIPLE_OUTLINE),
        "home-battery" => Some(icons::HOME_BATTERY),
        "camera-metering-spot" => Some(icons::CAMERA_METERING_SPOT),
        "nfc-search-variant" => Some(icons::NFC_SEARCH_VARIANT),
        "star-minus-outline" => Some(icons::STAR_MINUS_OUTLINE),
        "skip-backward-outline" => Some(icons::SKIP_BACKWARD_OUTLINE),
        "weather-lightning" => Some(icons::WEATHER_LIGHTNING),
        "comment-plus-outline" => Some(icons::COMMENT_PLUS_OUTLINE),
        "vibrate" => Some(icons::VIBRATE),
        "rewind-15" => Some(icons::REWIND_15),
        "bell-remove-outline" => Some(icons::BELL_REMOVE_OUTLINE),
        "notebook-minus" => Some(icons::NOTEBOOK_MINUS),
        "bridge" => Some(icons::BRIDGE),
        "earbuds" => Some(icons::EARBUDS),
        "tennis" => Some(icons::TENNIS),
        "eye-plus" => Some(icons::EYE_PLUS),
        "cloud-cog-outline" => Some(icons::CLOUD_COG_OUTLINE),
        "database-check" => Some(icons::DATABASE_CHECK),
        "arrow-oscillating-off" => Some(icons::ARROW_OSCILLATING_OFF),
        "credit-card-multiple-outline" => Some(icons::CREDIT_CARD_MULTIPLE_OUTLINE),
        "controller-classic-outline" => Some(icons::CONTROLLER_CLASSIC_OUTLINE),
        "filter-minus-outline" => Some(icons::FILTER_MINUS_OUTLINE),
        "table-network" => Some(icons::TABLE_NETWORK),
        "email-seal" => Some(icons::EMAIL_SEAL),
        "chess-rook" => Some(icons::CHESS_ROOK),
        "bookmark-music-outline" => Some(icons::BOOKMARK_MUSIC_OUTLINE),
        "scatter-plot-outline" => Some(icons::SCATTER_PLOT_OUTLINE),
        "billiards" => Some(icons::BILLIARDS),
        "code-less-than" => Some(icons::CODE_LESS_THAN),
        "chandelier" => Some(icons::CHANDELIER),
        "signal-2g" => Some(icons::SIGNAL_2G),
        "khanda" => Some(icons::KHANDA),
        "zodiac-aquarius" => Some(icons::ZODIAC_AQUARIUS),
        "file-music" => Some(icons::FILE_MUSIC),
        "bookmark-check" => Some(icons::BOOKMARK_CHECK),
        "water-boiler-alert" => Some(icons::WATER_BOILER_ALERT),
        "snail" => Some(icons::SNAIL),
        "rotate-left-variant" => Some(icons::ROTATE_LEFT_VARIANT),
        "calendar-remove" => Some(icons::CALENDAR_REMOVE),
        "eyedropper" => Some(icons::EYEDROPPER),
        "timer-plus-outline" => Some(icons::TIMER_PLUS_OUTLINE),
        "table-plus" => Some(icons::TABLE_PLUS),
        "invoice-export-outline" => Some(icons::INVOICE_EXPORT_OUTLINE),
        "gamepad-circle-down" => Some(icons::GAMEPAD_CIRCLE_DOWN),
        "trumpet" => Some(icons::TRUMPET),
        "cloud-download-outline" => Some(icons::CLOUD_DOWNLOAD_OUTLINE),
        "boombox" => Some(icons::BOOMBOX),
        "lamp" => Some(icons::LAMP),
        "wiper" => Some(icons::WIPER),
        "home-search-outline" => Some(icons::HOME_SEARCH_OUTLINE),
        "pine-tree-variant-outline" => Some(icons::PINE_TREE_VARIANT_OUTLINE),
        "signal" => Some(icons::SIGNAL),
        _ => None,
    }
}
