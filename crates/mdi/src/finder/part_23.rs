// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_23(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "egg-easter" => Some(icons::EGG_EASTER),
        "cookie-remove-outline" => Some(icons::COOKIE_REMOVE_OUTLINE),
        "table-heart" => Some(icons::TABLE_HEART),
        "watch-export" => Some(icons::WATCH_EXPORT),
        "minus-circle-outline" => Some(icons::MINUS_CIRCLE_OUTLINE),
        "alpha-y-box-outline" => Some(icons::ALPHA_Y_BOX_OUTLINE),
        "format-text-rotation-down" => Some(icons::FORMAT_TEXT_ROTATION_DOWN),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-alert" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-alert' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_ALERT)
        }
        "tree" => Some(icons::TREE),
        "globe-light-outline" => Some(icons::GLOBE_LIGHT_OUTLINE),
        "page-next" => Some(icons::PAGE_NEXT),
        "cookie-plus-outline" => Some(icons::COOKIE_PLUS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-xbox' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_XBOX)
        }
        "gas-station-outline" => Some(icons::GAS_STATION_OUTLINE),
        "sort-variant" => Some(icons::SORT_VARIANT),
        "fan" => Some(icons::FAN),
        "comma-circle-outline" => Some(icons::COMMA_CIRCLE_OUTLINE),
        "contrast-circle" => Some(icons::CONTRAST_CIRCLE),
        "car-speed-limiter" => Some(icons::CAR_SPEED_LIMITER),
        "receipt-text-plus-outline" => Some(icons::RECEIPT_TEXT_PLUS_OUTLINE),
        "alpha-z-box" => Some(icons::ALPHA_Z_BOX),
        "lightbulb-on-60" => Some(icons::LIGHTBULB_ON_60),
        "format-size" => Some(icons::FORMAT_SIZE),
        "checkbox-marked" => Some(icons::CHECKBOX_MARKED),
        "chess-king" => Some(icons::CHESS_KING),
        "pencil-circle-outline" => Some(icons::PENCIL_CIRCLE_OUTLINE),
        "weather-hail" => Some(icons::WEATHER_HAIL),
        "numeric-4-box" => Some(icons::NUMERIC_4_BOX),
        "sort-numeric-ascending" => Some(icons::SORT_NUMERIC_ASCENDING),
        "car-brake-hold" => Some(icons::CAR_BRAKE_HOLD),
        "folder-wrench-outline" => Some(icons::FOLDER_WRENCH_OUTLINE),
        "home-clock-outline" => Some(icons::HOME_CLOCK_OUTLINE),
        "view-compact-outline" => Some(icons::VIEW_COMPACT_OUTLINE),
        "checkbox-multiple-marked" => Some(icons::CHECKBOX_MULTIPLE_MARKED),
        "arrow-vertical-lock" => Some(icons::ARROW_VERTICAL_LOCK),
        "play-box-edit-outline" => Some(icons::PLAY_BOX_EDIT_OUTLINE),
        "shield-moon-outline" => Some(icons::SHIELD_MOON_OUTLINE),
        "numeric-7" => Some(icons::NUMERIC_7),
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
        "monitor-star" => Some(icons::MONITOR_STAR),
        "order-bool-ascending" => Some(icons::ORDER_BOOL_ASCENDING),
        "lightbulb-cfl-spiral" => Some(icons::LIGHTBULB_CFL_SPIRAL),
        "video-high-definition" => Some(icons::VIDEO_HIGH_DEFINITION),
        "receipt-text-arrow-left" => Some(icons::RECEIPT_TEXT_ARROW_LEFT),
        "rss-off" => Some(icons::RSS_OFF),
        #[allow(deprecated)]
        "hulu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'hulu' is deprecated.").print(py);
            }
            Some(icons::HULU)
        }
        "marker" => Some(icons::MARKER),
        "remote-tv-off" => Some(icons::REMOTE_TV_OFF),
        "monitor-multiple" => Some(icons::MONITOR_MULTIPLE),
        "hexagon" => Some(icons::HEXAGON),
        "palm-tree" => Some(icons::PALM_TREE),
        "fridge-top" => Some(icons::FRIDGE_TOP),
        "head" => Some(icons::HEAD),
        "car-seat-heater" => Some(icons::CAR_SEAT_HEATER),
        "video-wireless" => Some(icons::VIDEO_WIRELESS),
        "application-array-outline" => Some(icons::APPLICATION_ARRAY_OUTLINE),
        "sort-numeric-descending" => Some(icons::SORT_NUMERIC_DESCENDING),
        "shovel" => Some(icons::SHOVEL),
        "account-multiple" => Some(icons::ACCOUNT_MULTIPLE),
        "lock-alert" => Some(icons::LOCK_ALERT),
        "bullseye" => Some(icons::BULLSEYE),
        "glass-cocktail-off" => Some(icons::GLASS_COCKTAIL_OFF),
        "bus-side" => Some(icons::BUS_SIDE),
        "dice-d12" => Some(icons::DICE_D12),
        "paperclip-off" => Some(icons::PAPERCLIP_OFF),
        "merge" => Some(icons::MERGE),
        "inbox-full" => Some(icons::INBOX_FULL),
        "table-minus" => Some(icons::TABLE_MINUS),
        "book-minus-multiple" => Some(icons::BOOK_MINUS_MULTIPLE),
        "store-cog" => Some(icons::STORE_COG),
        "sofa-single-outline" => Some(icons::SOFA_SINGLE_OUTLINE),
        "newspaper-variant" => Some(icons::NEWSPAPER_VARIANT),
        "email-newsletter" => Some(icons::EMAIL_NEWSLETTER),
        "battery-10" => Some(icons::BATTERY_10),
        "medication" => Some(icons::MEDICATION),
        "timeline-alert-outline" => Some(icons::TIMELINE_ALERT_OUTLINE),
        "projector-screen-variant-off" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF),
        "arrow-up-thick" => Some(icons::ARROW_UP_THICK),
        "numeric-3-box-outline" => Some(icons::NUMERIC_3_BOX_OUTLINE),
        "battery-low" => Some(icons::BATTERY_LOW),
        "book-outline" => Some(icons::BOOK_OUTLINE),
        "gas-station-in-use-outline" => Some(icons::GAS_STATION_IN_USE_OUTLINE),
        "seed-plus" => Some(icons::SEED_PLUS),
        "form-textbox-password" => Some(icons::FORM_TEXTBOX_PASSWORD),
        "sign-pole" => Some(icons::SIGN_POLE),
        "pot-outline" => Some(icons::POT_OUTLINE),
        "account-voice" => Some(icons::ACCOUNT_VOICE),
        "file-check-outline" => Some(icons::FILE_CHECK_OUTLINE),
        "folder-lock-open" => Some(icons::FOLDER_LOCK_OPEN),
        "zodiac-taurus" => Some(icons::ZODIAC_TAURUS),
        "star-cog" => Some(icons::STAR_COG),
        "alert-rhombus-outline" => Some(icons::ALERT_RHOMBUS_OUTLINE),
        "fuse" => Some(icons::FUSE),
        "folder-minus" => Some(icons::FOLDER_MINUS),
        "train-car-centerbeam" => Some(icons::TRAIN_CAR_CENTERBEAM),
        "home-remove" => Some(icons::HOME_REMOVE),
        "arrow-down-right" => Some(icons::ARROW_DOWN_RIGHT),
        "alpha-d-box" => Some(icons::ALPHA_D_BOX),
        "account-lock-outline" => Some(icons::ACCOUNT_LOCK_OUTLINE),
        "heart-off" => Some(icons::HEART_OFF),
        "remote" => Some(icons::REMOTE),
        "alpha-k-box-outline" => Some(icons::ALPHA_K_BOX_OUTLINE),
        "note-off" => Some(icons::NOTE_OFF),
        "palette" => Some(icons::PALETTE),
        "watch-export-variant" => Some(icons::WATCH_EXPORT_VARIANT),
        "account-cowboy-hat-outline" => Some(icons::ACCOUNT_COWBOY_HAT_OUTLINE),
        "vote-outline" => Some(icons::VOTE_OUTLINE),
        "toy-brick-plus" => Some(icons::TOY_BRICK_PLUS),
        "database-eye-off" => Some(icons::DATABASE_EYE_OFF),
        "cards-variant" => Some(icons::CARDS_VARIANT),
        "head-alert-outline" => Some(icons::HEAD_ALERT_OUTLINE),
        #[allow(deprecated)]
        "qi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qi' is deprecated.").print(py);
            }
            Some(icons::QI)
        }
        "checkbox-multiple-marked-circle" => Some(icons::CHECKBOX_MULTIPLE_MARKED_CIRCLE),
        "file-table" => Some(icons::FILE_TABLE),
        "phone-paused" => Some(icons::PHONE_PAUSED),
        "projector-screen-variant" => Some(icons::PROJECTOR_SCREEN_VARIANT),
        "timer-sand-paused" => Some(icons::TIMER_SAND_PAUSED),
        "rewind-outline" => Some(icons::REWIND_OUTLINE),
        "vector-circle" => Some(icons::VECTOR_CIRCLE),
        "select-marker" => Some(icons::SELECT_MARKER),
        "upload-network" => Some(icons::UPLOAD_NETWORK),
        "book-check-outline" => Some(icons::BOOK_CHECK_OUTLINE),
        "cellphone-basic" => Some(icons::CELLPHONE_BASIC),
        "text-box" => Some(icons::TEXT_BOX),
        "chess-knight" => Some(icons::CHESS_KNIGHT),
        "fast-forward-30" => Some(icons::FAST_FORWARD_30),
        "robot-confused-outline" => Some(icons::ROBOT_CONFUSED_OUTLINE),
        "line-scan" => Some(icons::LINE_SCAN),
        "progress-helper" => Some(icons::PROGRESS_HELPER),
        "heart-outline" => Some(icons::HEART_OUTLINE),
        "archive-sync" => Some(icons::ARCHIVE_SYNC),
        "email-variant" => Some(icons::EMAIL_VARIANT),
        #[allow(deprecated)]
        "skype" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'skype' is deprecated.").print(py);
            }
            Some(icons::SKYPE)
        }
        "human-male-height-variant" => Some(icons::HUMAN_MALE_HEIGHT_VARIANT),
        "play-protected-content" => Some(icons::PLAY_PROTECTED_CONTENT),
        "skull-outline" => Some(icons::SKULL_OUTLINE),
        "shark" => Some(icons::SHARK),
        "tune" => Some(icons::TUNE),
        "video-switch" => Some(icons::VIDEO_SWITCH),
        "trash-can-outline" => Some(icons::TRASH_CAN_OUTLINE),
        "restore-alert" => Some(icons::RESTORE_ALERT),
        "format-indent-decrease" => Some(icons::FORMAT_INDENT_DECREASE),
        "close-octagon" => Some(icons::CLOSE_OCTAGON),
        "hexagram-outline" => Some(icons::HEXAGRAM_OUTLINE),
        "virus-outline" => Some(icons::VIRUS_OUTLINE),
        "social-distance-6-feet" => Some(icons::SOCIAL_DISTANCE_6_FEET),
        "safe-square-outline" => Some(icons::SAFE_SQUARE_OUTLINE),
        "pentagon-outline" => Some(icons::PENTAGON_OUTLINE),
        "phone-outgoing-outline" => Some(icons::PHONE_OUTGOING_OUTLINE),
        "lasso" => Some(icons::LASSO),
        "alpha-u-box-outline" => Some(icons::ALPHA_U_BOX_OUTLINE),
        "store-plus-outline" => Some(icons::STORE_PLUS_OUTLINE),
        "inbox-multiple-outline" => Some(icons::INBOX_MULTIPLE_OUTLINE),
        "star-minus" => Some(icons::STAR_MINUS),
        "ceiling-light" => Some(icons::CEILING_LIGHT),
        "account-cash-outline" => Some(icons::ACCOUNT_CASH_OUTLINE),
        "currency-cny" => Some(icons::CURRENCY_CNY),
        "tshirt-crew-outline" => Some(icons::TSHIRT_CREW_OUTLINE),
        "dump-truck" => Some(icons::DUMP_TRUCK),
        "table-eye" => Some(icons::TABLE_EYE),
        "sprinkler" => Some(icons::SPRINKLER),
        "hook" => Some(icons::HOOK),
        "surround-sound" => Some(icons::SURROUND_SOUND),
        "human-male" => Some(icons::HUMAN_MALE),
        "home-floor-l" => Some(icons::HOME_FLOOR_L),
        "cookie-lock-outline" => Some(icons::COOKIE_LOCK_OUTLINE),
        "weather-moonset-up" => Some(icons::WEATHER_MOONSET_UP),
        "surround-sound-7-1" => Some(icons::SURROUND_SOUND_7_1),
        "high-definition" => Some(icons::HIGH_DEFINITION),
        "bathtub" => Some(icons::BATHTUB),
        "face-man-shimmer-outline" => Some(icons::FACE_MAN_SHIMMER_OUTLINE),
        "relation-many-to-zero-or-one" => Some(icons::RELATION_MANY_TO_ZERO_OR_ONE),
        "receipt-clock-outline" => Some(icons::RECEIPT_CLOCK_OUTLINE),
        "video-3d-variant" => Some(icons::VIDEO_3D_VARIANT),
        "music-off" => Some(icons::MUSIC_OFF),
        "sun-clock" => Some(icons::SUN_CLOCK),
        "alpha-o-box" => Some(icons::ALPHA_O_BOX),
        "movie" => Some(icons::MOVIE),
        "auto-fix" => Some(icons::AUTO_FIX),
        "diabetes" => Some(icons::DIABETES),
        "vote" => Some(icons::VOTE),
        "dice-5-outline" => Some(icons::DICE_5_OUTLINE),
        "lightbulb-multiple" => Some(icons::LIGHTBULB_MULTIPLE),
        "clipboard-arrow-left" => Some(icons::CLIPBOARD_ARROW_LEFT),
        "calendar-expand-horizontal-outline" => Some(icons::CALENDAR_EXPAND_HORIZONTAL_OUTLINE),
        "information-box-outline" => Some(icons::INFORMATION_BOX_OUTLINE),
        "timeline-minus" => Some(icons::TIMELINE_MINUS),
        "emoticon-sick-outline" => Some(icons::EMOTICON_SICK_OUTLINE),
        "alpha-x" => Some(icons::ALPHA_X),
        "water-plus" => Some(icons::WATER_PLUS),
        "message-bulleted-off" => Some(icons::MESSAGE_BULLETED_OFF),
        #[allow(deprecated)]
        "mastodon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mastodon' is deprecated.").print(py);
            }
            Some(icons::MASTODON)
        }
        "pail-plus" => Some(icons::PAIL_PLUS),
        "swap-vertical-circle-outline" => Some(icons::SWAP_VERTICAL_CIRCLE_OUTLINE),
        "checkbox-multiple-blank-circle" => Some(icons::CHECKBOX_MULTIPLE_BLANK_CIRCLE),
        "mine" => Some(icons::MINE),
        "hops" => Some(icons::HOPS),
        "drama-masks" => Some(icons::DRAMA_MASKS),
        "usb-flash-drive" => Some(icons::USB_FLASH_DRIVE),
        "download-network" => Some(icons::DOWNLOAD_NETWORK),
        _ => None,
    }
}
