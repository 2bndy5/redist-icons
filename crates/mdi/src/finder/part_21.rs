// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_21(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "email-search-outline" => Some(icons::EMAIL_SEARCH_OUTLINE),
        "wheelchair" => Some(icons::WHEELCHAIR),
        "wallet-plus" => Some(icons::WALLET_PLUS),
        "calculator" => Some(icons::CALCULATOR),
        "star-three-points-outline" => Some(icons::STAR_THREE_POINTS_OUTLINE),
        #[allow(deprecated)]
        "laravel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'laravel' is deprecated.").print(py);
            }
            Some(icons::LARAVEL)
        }
        "square-medium" => Some(icons::SQUARE_MEDIUM),
        "cog-play" => Some(icons::COG_PLAY),
        "toothbrush-electric" => Some(icons::TOOTHBRUSH_ELECTRIC),
        "dice-d10-outline" => Some(icons::DICE_D10_OUTLINE),
        "weather-hurricane" => Some(icons::WEATHER_HURRICANE),
        "newspaper-variant-multiple-outline" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE_OUTLINE),
        "bullseye" => Some(icons::BULLSEYE),
        "select-group" => Some(icons::SELECT_GROUP),
        "chili-medium" => Some(icons::CHILI_MEDIUM),
        "account-credit-card-outline" => Some(icons::ACCOUNT_CREDIT_CARD_OUTLINE),
        "application-import" => Some(icons::APPLICATION_IMPORT),
        "notebook-remove-outline" => Some(icons::NOTEBOOK_REMOVE_OUTLINE),
        "email-open-heart-outline" => Some(icons::EMAIL_OPEN_HEART_OUTLINE),
        "size-l" => Some(icons::SIZE_L),
        "star-plus" => Some(icons::STAR_PLUS),
        "timeline-text" => Some(icons::TIMELINE_TEXT),
        "monitor-cellphone-star" => Some(icons::MONITOR_CELLPHONE_STAR),
        "comment-quote-outline" => Some(icons::COMMENT_QUOTE_OUTLINE),
        "file-arrow-up-down-outline" => Some(icons::FILE_ARROW_UP_DOWN_OUTLINE),
        "gas-station-in-use-outline" => Some(icons::GAS_STATION_IN_USE_OUTLINE),
        "pencil-circle" => Some(icons::PENCIL_CIRCLE),
        "camera-flip" => Some(icons::CAMERA_FLIP),
        "glass-mug-off" => Some(icons::GLASS_MUG_OFF),
        "bulkhead-light" => Some(icons::BULKHEAD_LIGHT),
        "crown-outline" => Some(icons::CROWN_OUTLINE),
        "cross-bolnisi" => Some(icons::CROSS_BOLNISI),
        "clipboard-check" => Some(icons::CLIPBOARD_CHECK),
        "window-restore" => Some(icons::WINDOW_RESTORE),
        "diving" => Some(icons::DIVING),
        "network-pos" => Some(icons::NETWORK_POS),
        #[allow(deprecated)]
        "microsoft-access" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-access' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ACCESS)
        }
        "lightbulb-spot-off" => Some(icons::LIGHTBULB_SPOT_OFF),
        "hand-front-left-outline" => Some(icons::HAND_FRONT_LEFT_OUTLINE),
        "qrcode" => Some(icons::QRCODE),
        "format-horizontal-align-center" => Some(icons::FORMAT_HORIZONTAL_ALIGN_CENTER),
        "text-account" => Some(icons::TEXT_ACCOUNT),
        "moon-full" => Some(icons::MOON_FULL),
        "rivet" => Some(icons::RIVET),
        "bank-transfer-out" => Some(icons::BANK_TRANSFER_OUT),
        "perspective-more" => Some(icons::PERSPECTIVE_MORE),
        "clock-time-twelve" => Some(icons::CLOCK_TIME_TWELVE),
        "inbox-remove-outline" => Some(icons::INBOX_REMOVE_OUTLINE),
        "access-point" => Some(icons::ACCESS_POINT),
        "printer-3d-nozzle-alert" => Some(icons::PRINTER_3D_NOZZLE_ALERT),
        #[allow(deprecated)]
        "symfony" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'symfony' is deprecated.").print(py);
            }
            Some(icons::SYMFONY)
        }
        "car-seat" => Some(icons::CAR_SEAT),
        "layers-search-outline" => Some(icons::LAYERS_SEARCH_OUTLINE),
        "hand-wash" => Some(icons::HAND_WASH),
        "star-half-full" => Some(icons::STAR_HALF_FULL),
        "surround-sound-2-1" => Some(icons::SURROUND_SOUND_2_1),
        "ninja" => Some(icons::NINJA),
        "file-restore-outline" => Some(icons::FILE_RESTORE_OUTLINE),
        "spider-thread" => Some(icons::SPIDER_THREAD),
        "wallet-plus-outline" => Some(icons::WALLET_PLUS_OUTLINE),
        "car-shift-pattern" => Some(icons::CAR_SHIFT_PATTERN),
        "emoticon-tongue" => Some(icons::EMOTICON_TONGUE),
        "road" => Some(icons::ROAD),
        "hand-peace" => Some(icons::HAND_PEACE),
        "car-battery" => Some(icons::CAR_BATTERY),
        "image-marker-outline" => Some(icons::IMAGE_MARKER_OUTLINE),
        "vote-outline" => Some(icons::VOTE_OUTLINE),
        "alpha-z-circle" => Some(icons::ALPHA_Z_CIRCLE),
        "motorbike-electric" => Some(icons::MOTORBIKE_ELECTRIC),
        "gamepad-circle" => Some(icons::GAMEPAD_CIRCLE),
        "rabbit" => Some(icons::RABBIT),
        "lightbulb-alert" => Some(icons::LIGHTBULB_ALERT),
        "head-alert" => Some(icons::HEAD_ALERT),
        "arrow-left-circle-outline" => Some(icons::ARROW_LEFT_CIRCLE_OUTLINE),
        "circle" => Some(icons::CIRCLE),
        "home-floor-3" => Some(icons::HOME_FLOOR_3),
        "archive-minus" => Some(icons::ARCHIVE_MINUS),
        "qrcode-minus" => Some(icons::QRCODE_MINUS),
        "credit-card-off" => Some(icons::CREDIT_CARD_OFF),
        "car" => Some(icons::CAR),
        "compass-outline" => Some(icons::COMPASS_OUTLINE),
        "text-box-multiple" => Some(icons::TEXT_BOX_MULTIPLE),
        "book-plus-multiple-outline" => Some(icons::BOOK_PLUS_MULTIPLE_OUTLINE),
        "calendar-start-outline" => Some(icons::CALENDAR_START_OUTLINE),
        "eye-refresh" => Some(icons::EYE_REFRESH),
        "tumble-dryer" => Some(icons::TUMBLE_DRYER),
        "car-seat-heater" => Some(icons::CAR_SEAT_HEATER),
        "hail" => Some(icons::HAIL),
        "arrow-top-right-bottom-left" => Some(icons::ARROW_TOP_RIGHT_BOTTOM_LEFT),
        "calendar-weekend" => Some(icons::CALENDAR_WEEKEND),
        "circle-slice-1" => Some(icons::CIRCLE_SLICE_1),
        "clock" => Some(icons::CLOCK),
        "earth-minus" => Some(icons::EARTH_MINUS),
        "robot-excited-outline" => Some(icons::ROBOT_EXCITED_OUTLINE),
        "folder-marker-outline" => Some(icons::FOLDER_MARKER_OUTLINE),
        "comment-question" => Some(icons::COMMENT_QUESTION),
        "align-vertical-center" => Some(icons::ALIGN_VERTICAL_CENTER),
        "flag-variant-remove" => Some(icons::FLAG_VARIANT_REMOVE),
        "head-question-outline" => Some(icons::HEAD_QUESTION_OUTLINE),
        "forum-minus-outline" => Some(icons::FORUM_MINUS_OUTLINE),
        "format-header-2" => Some(icons::FORMAT_HEADER_2),
        "relation-many-to-zero-or-one" => Some(icons::RELATION_MANY_TO_ZERO_OR_ONE),
        "eye-refresh-outline" => Some(icons::EYE_REFRESH_OUTLINE),
        "radiology-box" => Some(icons::RADIOLOGY_BOX),
        "format-font-size-increase" => Some(icons::FORMAT_FONT_SIZE_INCREASE),
        "donkey" => Some(icons::DONKEY),
        "application-settings" => Some(icons::APPLICATION_SETTINGS),
        "pizza" => Some(icons::PIZZA),
        "stop" => Some(icons::STOP),
        "file-question" => Some(icons::FILE_QUESTION),
        "relation-many-to-many" => Some(icons::RELATION_MANY_TO_MANY),
        "clipboard-check-outline" => Some(icons::CLIPBOARD_CHECK_OUTLINE),
        "image-sync-outline" => Some(icons::IMAGE_SYNC_OUTLINE),
        "numeric-1-box-outline" => Some(icons::NUMERIC_1_BOX_OUTLINE),
        "ceiling-fan" => Some(icons::CEILING_FAN),
        #[allow(deprecated)]
        "skype" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'skype' is deprecated.").print(py);
            }
            Some(icons::SKYPE)
        }
        "ghost-off" => Some(icons::GHOST_OFF),
        "account-box-plus-outline" => Some(icons::ACCOUNT_BOX_PLUS_OUTLINE),
        "calendar-badge-outline" => Some(icons::CALENDAR_BADGE_OUTLINE),
        "source-commit-local" => Some(icons::SOURCE_COMMIT_LOCAL),
        "timer-refresh" => Some(icons::TIMER_REFRESH),
        #[allow(deprecated)]
        "language-python" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-python' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_PYTHON)
        }
        "map-marker-plus" => Some(icons::MAP_MARKER_PLUS),
        "robot-industrial" => Some(icons::ROBOT_INDUSTRIAL),
        "printer-pos-network-outline" => Some(icons::PRINTER_POS_NETWORK_OUTLINE),
        "account-minus-outline" => Some(icons::ACCOUNT_MINUS_OUTLINE),
        "database-export-outline" => Some(icons::DATABASE_EXPORT_OUTLINE),
        "home-clock" => Some(icons::HOME_CLOCK),
        "taxi" => Some(icons::TAXI),
        "bottle-soda-classic-outline" => Some(icons::BOTTLE_SODA_CLASSIC_OUTLINE),
        "floor-lamp-torchiere" => Some(icons::FLOOR_LAMP_TORCHIERE),
        "weather-sunset-down" => Some(icons::WEATHER_SUNSET_DOWN),
        "battery-50" => Some(icons::BATTERY_50),
        "menu-down" => Some(icons::MENU_DOWN),
        "briefcase-check" => Some(icons::BRIEFCASE_CHECK),
        "chart-scatter-plot-hexbin" => Some(icons::CHART_SCATTER_PLOT_HEXBIN),
        "swap-horizontal-circle-outline" => Some(icons::SWAP_HORIZONTAL_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "youtube-tv" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-tv' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_TV)
        }
        "gesture-swipe-down" => Some(icons::GESTURE_SWIPE_DOWN),
        "file-cad" => Some(icons::FILE_CAD),
        #[allow(deprecated)]
        "sony-playstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sony-playstation' is deprecated.")
                    .print(py);
            }
            Some(icons::SONY_PLAYSTATION)
        }
        "star-outline" => Some(icons::STAR_OUTLINE),
        "passport-alert" => Some(icons::PASSPORT_ALERT),
        "account-credit-card" => Some(icons::ACCOUNT_CREDIT_CARD),
        "diving-scuba-tank-multiple" => Some(icons::DIVING_SCUBA_TANK_MULTIPLE),
        "seed-off" => Some(icons::SEED_OFF),
        "human-female-female-child" => Some(icons::HUMAN_FEMALE_FEMALE_CHILD),
        "trademark" => Some(icons::TRADEMARK),
        "code-block-tags" => Some(icons::CODE_BLOCK_TAGS),
        "yoga" => Some(icons::YOGA),
        "console-line" => Some(icons::CONSOLE_LINE),
        "bug-play" => Some(icons::BUG_PLAY),
        "umbrella-outline" => Some(icons::UMBRELLA_OUTLINE),
        "diving-flippers" => Some(icons::DIVING_FLIPPERS),
        "clipboard-remove-outline" => Some(icons::CLIPBOARD_REMOVE_OUTLINE),
        "mouse-right-click-outline" => Some(icons::MOUSE_RIGHT_CLICK_OUTLINE),
        "bluetooth-transfer" => Some(icons::BLUETOOTH_TRANSFER),
        "phone-ring-outline" => Some(icons::PHONE_RING_OUTLINE),
        "alpha-r-circle" => Some(icons::ALPHA_R_CIRCLE),
        "thermometer-minus" => Some(icons::THERMOMETER_MINUS),
        "pencil" => Some(icons::PENCIL),
        "plus-minus" => Some(icons::PLUS_MINUS),
        "escalator-box" => Some(icons::ESCALATOR_BOX),
        "database-search-outline" => Some(icons::DATABASE_SEARCH_OUTLINE),
        "tumble-dryer-off" => Some(icons::TUMBLE_DRYER_OFF),
        "robot-excited" => Some(icons::ROBOT_EXCITED),
        "alpha-j-box" => Some(icons::ALPHA_J_BOX),
        "projector-off" => Some(icons::PROJECTOR_OFF),
        "file-upload" => Some(icons::FILE_UPLOAD),
        "printer-pos-check" => Some(icons::PRINTER_POS_CHECK),
        "rename-outline" => Some(icons::RENAME_OUTLINE),
        "repeat-off" => Some(icons::REPEAT_OFF),
        "lock-percent-open-outline" => Some(icons::LOCK_PERCENT_OPEN_OUTLINE),
        "airplane-cog" => Some(icons::AIRPLANE_COG),
        "handshake-outline" => Some(icons::HANDSHAKE_OUTLINE),
        "account-cowboy-hat-outline" => Some(icons::ACCOUNT_COWBOY_HAT_OUTLINE),
        "police-badge" => Some(icons::POLICE_BADGE),
        "all-inclusive-box-outline" => Some(icons::ALL_INCLUSIVE_BOX_OUTLINE),
        "equal-box" => Some(icons::EQUAL_BOX),
        "tower-beach" => Some(icons::TOWER_BEACH),
        "shopping-music" => Some(icons::SHOPPING_MUSIC),
        "projector-screen-variant" => Some(icons::PROJECTOR_SCREEN_VARIANT),
        "weather-pouring" => Some(icons::WEATHER_POURING),
        "bell-check-outline" => Some(icons::BELL_CHECK_OUTLINE),
        "wifi-strength-2" => Some(icons::WIFI_STRENGTH_2),
        "train-car-flatbed-car" => Some(icons::TRAIN_CAR_FLATBED_CAR),
        "square-opacity" => Some(icons::SQUARE_OPACITY),
        "script-text-play-outline" => Some(icons::SCRIPT_TEXT_PLAY_OUTLINE),
        "lock-percent-open-variant-outline" => Some(icons::LOCK_PERCENT_OPEN_VARIANT_OUTLINE),
        "storage-tank-outline" => Some(icons::STORAGE_TANK_OUTLINE),
        "timer-star" => Some(icons::TIMER_STAR),
        "arrow-left-box" => Some(icons::ARROW_LEFT_BOX),
        "account-network-outline" => Some(icons::ACCOUNT_NETWORK_OUTLINE),
        "delete-clock" => Some(icons::DELETE_CLOCK),
        "road-variant" => Some(icons::ROAD_VARIANT),
        "alpha-n-circle-outline" => Some(icons::ALPHA_N_CIRCLE_OUTLINE),
        "cash-register" => Some(icons::CASH_REGISTER),
        "calendar-expand-horizontal" => Some(icons::CALENDAR_EXPAND_HORIZONTAL),
        "rhombus-outline" => Some(icons::RHOMBUS_OUTLINE),
        "currency-eur" => Some(icons::CURRENCY_EUR),
        _ => None,
    }
}
