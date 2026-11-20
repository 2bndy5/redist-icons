// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_1(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "video-off" => Some(icons::VIDEO_OFF),
        "wall-sconce-flat-outline" => Some(icons::WALL_SCONCE_FLAT_OUTLINE),
        "arrow-down-bold-box" => Some(icons::ARROW_DOWN_BOLD_BOX),
        "speaker-off" => Some(icons::SPEAKER_OFF),
        "incognito-circle" => Some(icons::INCOGNITO_CIRCLE),
        "speedometer" => Some(icons::SPEEDOMETER),
        "karate" => Some(icons::KARATE),
        "egg" => Some(icons::EGG),
        "home-percent-outline" => Some(icons::HOME_PERCENT_OUTLINE),
        "format-indent-decrease" => Some(icons::FORMAT_INDENT_DECREASE),
        "bicycle-cargo" => Some(icons::BICYCLE_CARGO),
        "mixed-reality" => Some(icons::MIXED_REALITY),
        "shield-star-outline" => Some(icons::SHIELD_STAR_OUTLINE),
        "thermometer-chevron-down" => Some(icons::THERMOMETER_CHEVRON_DOWN),
        "pi" => Some(icons::PI),
        "power-plug-battery" => Some(icons::POWER_PLUG_BATTERY),
        "cash-lock-open" => Some(icons::CASH_LOCK_OPEN),
        "nut" => Some(icons::NUT),
        "vacuum-outline" => Some(icons::VACUUM_OUTLINE),
        "ceiling-light-multiple" => Some(icons::CEILING_LIGHT_MULTIPLE),
        "phone-refresh-outline" => Some(icons::PHONE_REFRESH_OUTLINE),
        "reload-alert" => Some(icons::RELOAD_ALERT),
        #[allow(deprecated)]
        "angularjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'angularjs' is deprecated.").print(py);
            }
            Some(icons::ANGULARJS)
        }
        "shoe-heel" => Some(icons::SHOE_HEEL),
        "tooltip-remove-outline" => Some(icons::TOOLTIP_REMOVE_OUTLINE),
        "food-drumstick" => Some(icons::FOOD_DRUMSTICK),
        "arrow-left-bold-outline" => Some(icons::ARROW_LEFT_BOLD_OUTLINE),
        "eye-arrow-right-outline" => Some(icons::EYE_ARROW_RIGHT_OUTLINE),
        "menu-left-outline" => Some(icons::MENU_LEFT_OUTLINE),
        "bookmark-music-outline" => Some(icons::BOOKMARK_MUSIC_OUTLINE),
        "valve-open" => Some(icons::VALVE_OPEN),
        "folder-off" => Some(icons::FOLDER_OFF),
        #[allow(deprecated)]
        "instagram" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'instagram' is deprecated.").print(py);
            }
            Some(icons::INSTAGRAM)
        }
        "khanda" => Some(icons::KHANDA),
        "sun-snowflake-variant" => Some(icons::SUN_SNOWFLAKE_VARIANT),
        "cloud-question-outline" => Some(icons::CLOUD_QUESTION_OUTLINE),
        "go-kart-track" => Some(icons::GO_KART_TRACK),
        "alpha-c-box" => Some(icons::ALPHA_C_BOX),
        "bee-flower" => Some(icons::BEE_FLOWER),
        "tab" => Some(icons::TAB),
        "horseshoe" => Some(icons::HORSESHOE),
        "television-classic-off" => Some(icons::TELEVISION_CLASSIC_OFF),
        "movie-open-play-outline" => Some(icons::MOVIE_OPEN_PLAY_OUTLINE),
        "receipt-text-edit" => Some(icons::RECEIPT_TEXT_EDIT),
        "ray-end" => Some(icons::RAY_END),
        "flower-pollen-outline" => Some(icons::FLOWER_POLLEN_OUTLINE),
        "cradle" => Some(icons::CRADLE),
        "information-variant-circle" => Some(icons::INFORMATION_VARIANT_CIRCLE),
        #[allow(deprecated)]
        "google-play" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-play' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLAY)
        }
        "select-drag" => Some(icons::SELECT_DRAG),
        "attachment-remove" => Some(icons::ATTACHMENT_REMOVE),
        "source-repository-multiple" => Some(icons::SOURCE_REPOSITORY_MULTIPLE),
        "camera-off" => Some(icons::CAMERA_OFF),
        "clock-time-twelve" => Some(icons::CLOCK_TIME_TWELVE),
        "diving-scuba-tank-multiple" => Some(icons::DIVING_SCUBA_TANK_MULTIPLE),
        "gas-station" => Some(icons::GAS_STATION),
        "camera-timer" => Some(icons::CAMERA_TIMER),
        "hydrogen-station" => Some(icons::HYDROGEN_STATION),
        "fan-off" => Some(icons::FAN_OFF),
        "handshake" => Some(icons::HANDSHAKE),
        "alpha-p-circle-outline" => Some(icons::ALPHA_P_CIRCLE_OUTLINE),
        "puzzle-outline" => Some(icons::PUZZLE_OUTLINE),
        "align-vertical-center" => Some(icons::ALIGN_VERTICAL_CENTER),
        "mouse-variant-off" => Some(icons::MOUSE_VARIANT_OFF),
        "tag-outline" => Some(icons::TAG_OUTLINE),
        "bookmark-outline" => Some(icons::BOOKMARK_OUTLINE),
        "bank-transfer-in" => Some(icons::BANK_TRANSFER_IN),
        "map-marker-path" => Some(icons::MAP_MARKER_PATH),
        "phone-in-talk-outline" => Some(icons::PHONE_IN_TALK_OUTLINE),
        "eject-outline" => Some(icons::EJECT_OUTLINE),
        "expansion-card" => Some(icons::EXPANSION_CARD),
        "pencil-off" => Some(icons::PENCIL_OFF),
        "weather-sunset-up" => Some(icons::WEATHER_SUNSET_UP),
        "account-box-plus-outline" => Some(icons::ACCOUNT_BOX_PLUS_OUTLINE),
        "beaker-question" => Some(icons::BEAKER_QUESTION),
        "door-closed-cancel" => Some(icons::DOOR_CLOSED_CANCEL),
        "relation-many-to-only-one" => Some(icons::RELATION_MANY_TO_ONLY_ONE),
        "rounded-corner" => Some(icons::ROUNDED_CORNER),
        "alpha-a-box-outline" => Some(icons::ALPHA_A_BOX_OUTLINE),
        "page-layout-sidebar-right" => Some(icons::PAGE_LAYOUT_SIDEBAR_RIGHT),
        "alpha-v-box-outline" => Some(icons::ALPHA_V_BOX_OUTLINE),
        "briefcase-download" => Some(icons::BRIEFCASE_DOWNLOAD),
        "content-save-settings" => Some(icons::CONTENT_SAVE_SETTINGS),
        "gamepad-circle-left" => Some(icons::GAMEPAD_CIRCLE_LEFT),
        "robot-confused-outline" => Some(icons::ROBOT_CONFUSED_OUTLINE),
        "alpha-k-circle" => Some(icons::ALPHA_K_CIRCLE),
        #[allow(deprecated)]
        "microsoft-office" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-office' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OFFICE)
        }
        "battery-low" => Some(icons::BATTERY_LOW),
        "cloud-key-outline" => Some(icons::CLOUD_KEY_OUTLINE),
        "home-lightning-bolt" => Some(icons::HOME_LIGHTNING_BOLT),
        "division" => Some(icons::DIVISION),
        "restore-alert" => Some(icons::RESTORE_ALERT),
        "speaker" => Some(icons::SPEAKER),
        "arrow-left-bold" => Some(icons::ARROW_LEFT_BOLD),
        "database-arrow-down" => Some(icons::DATABASE_ARROW_DOWN),
        "fan-speed-2" => Some(icons::FAN_SPEED_2),
        "volume-medium" => Some(icons::VOLUME_MEDIUM),
        "calendar-question-outline" => Some(icons::CALENDAR_QUESTION_OUTLINE),
        "align-vertical-top" => Some(icons::ALIGN_VERTICAL_TOP),
        "file-word" => Some(icons::FILE_WORD),
        "wifi-strength-1-lock-open" => Some(icons::WIFI_STRENGTH_1_LOCK_OPEN),
        "robot-industrial-outline" => Some(icons::ROBOT_INDUSTRIAL_OUTLINE),
        "playlist-star" => Some(icons::PLAYLIST_STAR),
        "order-alphabetical-ascending" => Some(icons::ORDER_ALPHABETICAL_ASCENDING),
        "bed" => Some(icons::BED),
        "car-brake-worn-linings" => Some(icons::CAR_BRAKE_WORN_LININGS),
        "heart-broken" => Some(icons::HEART_BROKEN),
        "basket-check" => Some(icons::BASKET_CHECK),
        "shield-home" => Some(icons::SHIELD_HOME),
        "garage-variant-lock" => Some(icons::GARAGE_VARIANT_LOCK),
        "transcribe-close" => Some(icons::TRANSCRIBE_CLOSE),
        "arrow-left-right" => Some(icons::ARROW_LEFT_RIGHT),
        "filmstrip" => Some(icons::FILMSTRIP),
        "chart-timeline" => Some(icons::CHART_TIMELINE),
        "betamax" => Some(icons::BETAMAX),
        "garage-open-variant" => Some(icons::GARAGE_OPEN_VARIANT),
        "foot-print" => Some(icons::FOOT_PRINT),
        "cards-heart-outline" => Some(icons::CARDS_HEART_OUTLINE),
        "water-thermometer" => Some(icons::WATER_THERMOMETER),
        "medical-bag" => Some(icons::MEDICAL_BAG),
        "home-circle-outline" => Some(icons::HOME_CIRCLE_OUTLINE),
        "battery-outline" => Some(icons::BATTERY_OUTLINE),
        "airplane-takeoff" => Some(icons::AIRPLANE_TAKEOFF),
        "bicycle" => Some(icons::BICYCLE),
        "dice-3" => Some(icons::DICE_3),
        "robot-confused" => Some(icons::ROBOT_CONFUSED),
        "wind-turbine-alert" => Some(icons::WIND_TURBINE_ALERT),
        "step-backward" => Some(icons::STEP_BACKWARD),
        "radius" => Some(icons::RADIUS),
        "furigana-vertical" => Some(icons::FURIGANA_VERTICAL),
        "generator-mobile" => Some(icons::GENERATOR_MOBILE),
        "car-select" => Some(icons::CAR_SELECT),
        "email-remove" => Some(icons::EMAIL_REMOVE),
        "image-multiple" => Some(icons::IMAGE_MULTIPLE),
        "drone" => Some(icons::DRONE),
        "chevron-left-box" => Some(icons::CHEVRON_LEFT_BOX),
        "gesture" => Some(icons::GESTURE),
        "pipe-leak" => Some(icons::PIPE_LEAK),
        "home-sound-in-outline" => Some(icons::HOME_SOUND_IN_OUTLINE),
        "message-flash" => Some(icons::MESSAGE_FLASH),
        "account-sync-outline" => Some(icons::ACCOUNT_SYNC_OUTLINE),
        "arrow-up-bold-hexagon-outline" => Some(icons::ARROW_UP_BOLD_HEXAGON_OUTLINE),
        "account-card-outline" => Some(icons::ACCOUNT_CARD_OUTLINE),
        "wallet-bifold" => Some(icons::WALLET_BIFOLD),
        "gamepad-round-left" => Some(icons::GAMEPAD_ROUND_LEFT),
        "file-document-edit" => Some(icons::FILE_DOCUMENT_EDIT),
        "alpha-m-circle-outline" => Some(icons::ALPHA_M_CIRCLE_OUTLINE),
        "book-clock-outline" => Some(icons::BOOK_CLOCK_OUTLINE),
        "barrel" => Some(icons::BARREL),
        "rice" => Some(icons::RICE),
        "music-note-bluetooth" => Some(icons::MUSIC_NOTE_BLUETOOTH),
        "chili-medium-outline" => Some(icons::CHILI_MEDIUM_OUTLINE),
        "login" => Some(icons::LOGIN),
        "drama-masks" => Some(icons::DRAMA_MASKS),
        "star-cog" => Some(icons::STAR_COG),
        "star-face" => Some(icons::STAR_FACE),
        "halloween" => Some(icons::HALLOWEEN),
        "theater" => Some(icons::THEATER),
        "comment-quote" => Some(icons::COMMENT_QUOTE),
        "zodiac-pisces" => Some(icons::ZODIAC_PISCES),
        "nutrition" => Some(icons::NUTRITION),
        "movie-plus-outline" => Some(icons::MOVIE_PLUS_OUTLINE),
        "integrated-circuit-chip" => Some(icons::INTEGRATED_CIRCUIT_CHIP),
        "database-cog-outline" => Some(icons::DATABASE_COG_OUTLINE),
        "rolodex-outline" => Some(icons::ROLODEX_OUTLINE),
        "car-search-outline" => Some(icons::CAR_SEARCH_OUTLINE),
        "archive-alert" => Some(icons::ARCHIVE_ALERT),
        #[allow(deprecated)]
        "language-css3" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-css3' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CSS3)
        }
        "slot-machine-outline" => Some(icons::SLOT_MACHINE_OUTLINE),
        "vector-triangle" => Some(icons::VECTOR_TRIANGLE),
        "emoticon-sad-outline" => Some(icons::EMOTICON_SAD_OUTLINE),
        "finance" => Some(icons::FINANCE),
        "clipboard-play-multiple-outline" => Some(icons::CLIPBOARD_PLAY_MULTIPLE_OUTLINE),
        "car-sports" => Some(icons::CAR_SPORTS),
        "ring" => Some(icons::RING),
        "airplane-minus" => Some(icons::AIRPLANE_MINUS),
        "relation-zero-or-one-to-only-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONLY_ONE),
        "oil-lamp" => Some(icons::OIL_LAMP),
        "speaker-play" => Some(icons::SPEAKER_PLAY),
        "tag-arrow-up-outline" => Some(icons::TAG_ARROW_UP_OUTLINE),
        "book-remove-multiple" => Some(icons::BOOK_REMOVE_MULTIPLE),
        "vector-square-minus" => Some(icons::VECTOR_SQUARE_MINUS),
        "truck-alert" => Some(icons::TRUCK_ALERT),
        "file-restore" => Some(icons::FILE_RESTORE),
        "merge" => Some(icons::MERGE),
        "format-text-variant" => Some(icons::FORMAT_TEXT_VARIANT),
        "spider-outline" => Some(icons::SPIDER_OUTLINE),
        "fridge-industrial-alert" => Some(icons::FRIDGE_INDUSTRIAL_ALERT),
        "printer-pos-sync" => Some(icons::PRINTER_POS_SYNC),
        "menu-right" => Some(icons::MENU_RIGHT),
        "share-outline" => Some(icons::SHARE_OUTLINE),
        "translate-variant" => Some(icons::TRANSLATE_VARIANT),
        "filter-variant" => Some(icons::FILTER_VARIANT),
        "badminton" => Some(icons::BADMINTON),
        "clipboard-text-off" => Some(icons::CLIPBOARD_TEXT_OFF),
        "snowflake-alert" => Some(icons::SNOWFLAKE_ALERT),
        "bank-off" => Some(icons::BANK_OFF),
        "remote-tv" => Some(icons::REMOTE_TV),
        "signal-cellular-2" => Some(icons::SIGNAL_CELLULAR_2),
        "tumble-dryer-alert" => Some(icons::TUMBLE_DRYER_ALERT),
        _ => None,
    }
}
