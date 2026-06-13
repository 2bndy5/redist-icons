// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_15(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "pencil" => Some(icons::PENCIL),
        "cross-bolnisi" => Some(icons::CROSS_BOLNISI),
        "relation-zero-or-many-to-zero-or-many" => {
            Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_MANY)
        }
        "calendar-clock" => Some(icons::CALENDAR_CLOCK),
        "percent-box" => Some(icons::PERCENT_BOX),
        "flower-tulip" => Some(icons::FLOWER_TULIP),
        "battery-60" => Some(icons::BATTERY_60),
        "text-box-plus" => Some(icons::TEXT_BOX_PLUS),
        "alphabet-piqad" => Some(icons::ALPHABET_PIQAD),
        "not-equal" => Some(icons::NOT_EQUAL),
        "timer-sand-empty" => Some(icons::TIMER_SAND_EMPTY),
        #[allow(deprecated)]
        "aws" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'aws' is deprecated.").print(py);
            }
            Some(icons::AWS)
        }
        "arrow-collapse" => Some(icons::ARROW_COLLAPSE),
        "cookie-off" => Some(icons::COOKIE_OFF),
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "delete-off" => Some(icons::DELETE_OFF),
        "timer-stop" => Some(icons::TIMER_STOP),
        "alert-decagram" => Some(icons::ALERT_DECAGRAM),
        "filter-outline" => Some(icons::FILTER_OUTLINE),
        "timeline" => Some(icons::TIMELINE),
        "power-settings" => Some(icons::POWER_SETTINGS),
        "timer-10" => Some(icons::TIMER_10),
        "hamburger" => Some(icons::HAMBURGER),
        "ceiling-light-multiple-outline" => Some(icons::CEILING_LIGHT_MULTIPLE_OUTLINE),
        "square-off-outline" => Some(icons::SQUARE_OFF_OUTLINE),
        "castle" => Some(icons::CASTLE),
        "label-variant" => Some(icons::LABEL_VARIANT),
        "archive-settings-outline" => Some(icons::ARCHIVE_SETTINGS_OUTLINE),
        "arrow-split-horizontal" => Some(icons::ARROW_SPLIT_HORIZONTAL),
        "invoice-outline" => Some(icons::INVOICE_OUTLINE),
        "drag-vertical" => Some(icons::DRAG_VERTICAL),
        "account-multiple" => Some(icons::ACCOUNT_MULTIPLE),
        "wifi-arrow-up" => Some(icons::WIFI_ARROW_UP),
        "sun-snowflake" => Some(icons::SUN_SNOWFLAKE),
        "ski-water" => Some(icons::SKI_WATER),
        "expansion-card" => Some(icons::EXPANSION_CARD),
        "arrow-down-circle-outline" => Some(icons::ARROW_DOWN_CIRCLE_OUTLINE),
        "airplane-edit" => Some(icons::AIRPLANE_EDIT),
        "label-percent" => Some(icons::LABEL_PERCENT),
        "flip-to-back" => Some(icons::FLIP_TO_BACK),
        "weather-moonset" => Some(icons::WEATHER_MOONSET),
        "file-move-outline" => Some(icons::FILE_MOVE_OUTLINE),
        "crown-circle-outline" => Some(icons::CROWN_CIRCLE_OUTLINE),
        "train-car-caboose" => Some(icons::TRAIN_CAR_CABOOSE),
        "vector-square-minus" => Some(icons::VECTOR_SQUARE_MINUS),
        "beaker-minus-outline" => Some(icons::BEAKER_MINUS_OUTLINE),
        "animation-play" => Some(icons::ANIMATION_PLAY),
        "account-badge" => Some(icons::ACCOUNT_BADGE),
        "harddisk" => Some(icons::HARDDISK),
        "bee" => Some(icons::BEE),
        "arrow-top-left-bottom-right" => Some(icons::ARROW_TOP_LEFT_BOTTOM_RIGHT),
        "shield-off-outline" => Some(icons::SHIELD_OFF_OUTLINE),
        "phone-remove-outline" => Some(icons::PHONE_REMOVE_OUTLINE),
        "forum-minus-outline" => Some(icons::FORUM_MINUS_OUTLINE),
        #[allow(deprecated)]
        "google-plus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-plus' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLUS)
        }
        "format-header-2" => Some(icons::FORMAT_HEADER_2),
        "temperature-fahrenheit" => Some(icons::TEMPERATURE_FAHRENHEIT),
        "image-sync" => Some(icons::IMAGE_SYNC),
        "network-pos" => Some(icons::NETWORK_POS),
        "weather-sunset" => Some(icons::WEATHER_SUNSET),
        "bridge" => Some(icons::BRIDGE),
        "less-than" => Some(icons::LESS_THAN),
        "home-export-outline" => Some(icons::HOME_EXPORT_OUTLINE),
        "wallet-bifold" => Some(icons::WALLET_BIFOLD),
        #[allow(deprecated)]
        "z-wave" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'z-wave' is deprecated.").print(py);
            }
            Some(icons::Z_WAVE)
        }
        "camera-retake-outline" => Some(icons::CAMERA_RETAKE_OUTLINE),
        "microwave-off" => Some(icons::MICROWAVE_OFF),
        "access-point-plus" => Some(icons::ACCESS_POINT_PLUS),
        "movie-open-star" => Some(icons::MOVIE_OPEN_STAR),
        "database-arrow-down" => Some(icons::DATABASE_ARROW_DOWN),
        "pill" => Some(icons::PILL),
        "inbox" => Some(icons::INBOX),
        "robot-industrial" => Some(icons::ROBOT_INDUSTRIAL),
        "file-send-outline" => Some(icons::FILE_SEND_OUTLINE),
        "delta" => Some(icons::DELTA),
        "account-settings" => Some(icons::ACCOUNT_SETTINGS),
        #[allow(deprecated)]
        "soundcloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'soundcloud' is deprecated.").print(py);
            }
            Some(icons::SOUNDCLOUD)
        }
        "cup-off" => Some(icons::CUP_OFF),
        "star-remove-outline" => Some(icons::STAR_REMOVE_OUTLINE),
        "wallet-giftcard" => Some(icons::WALLET_GIFTCARD),
        "note-off" => Some(icons::NOTE_OFF),
        "filter-settings-outline" => Some(icons::FILTER_SETTINGS_OUTLINE),
        "border-color" => Some(icons::BORDER_COLOR),
        "chart-box-multiple-outline" => Some(icons::CHART_BOX_MULTIPLE_OUTLINE),
        "folder-settings-outline" => Some(icons::FOLDER_SETTINGS_OUTLINE),
        "plus-network-outline" => Some(icons::PLUS_NETWORK_OUTLINE),
        "transit-transfer" => Some(icons::TRANSIT_TRANSFER),
        "water-percent-alert" => Some(icons::WATER_PERCENT_ALERT),
        "trophy-variant-outline" => Some(icons::TROPHY_VARIANT_OUTLINE),
        "slope-downhill" => Some(icons::SLOPE_DOWNHILL),
        "phone-ring" => Some(icons::PHONE_RING),
        "database-marker-outline" => Some(icons::DATABASE_MARKER_OUTLINE),
        "video-switch-outline" => Some(icons::VIDEO_SWITCH_OUTLINE),
        "file-word" => Some(icons::FILE_WORD),
        "calendar" => Some(icons::CALENDAR),
        "access-point-network" => Some(icons::ACCESS_POINT_NETWORK),
        "seed-outline" => Some(icons::SEED_OUTLINE),
        "drag-horizontal-variant" => Some(icons::DRAG_HORIZONTAL_VARIANT),
        "format-align-left" => Some(icons::FORMAT_ALIGN_LEFT),
        "filter-remove-outline" => Some(icons::FILTER_REMOVE_OUTLINE),
        "calendar-edit-outline" => Some(icons::CALENDAR_EDIT_OUTLINE),
        "align-vertical-top" => Some(icons::ALIGN_VERTICAL_TOP),
        "gift-open" => Some(icons::GIFT_OPEN),
        "face-mask" => Some(icons::FACE_MASK),
        "chart-pie-outline" => Some(icons::CHART_PIE_OUTLINE),
        "cup-off-outline" => Some(icons::CUP_OFF_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-medium" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-medium' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_MEDIUM)
        }
        "floor-lamp-torchiere" => Some(icons::FLOOR_LAMP_TORCHIERE),
        "tally-mark-5" => Some(icons::TALLY_MARK_5),
        "calendar-account-outline" => Some(icons::CALENDAR_ACCOUNT_OUTLINE),
        "help-rhombus" => Some(icons::HELP_RHOMBUS),
        "circle-half-full" => Some(icons::CIRCLE_HALF_FULL),
        "select-arrow-up" => Some(icons::SELECT_ARROW_UP),
        "hamburger-remove" => Some(icons::HAMBURGER_REMOVE),
        "exclamation" => Some(icons::EXCLAMATION),
        "keyboard-off-outline" => Some(icons::KEYBOARD_OFF_OUTLINE),
        "arrow-up-bold-circle-outline" => Some(icons::ARROW_UP_BOLD_CIRCLE_OUTLINE),
        "wind-power" => Some(icons::WIND_POWER),
        "lock-pattern" => Some(icons::LOCK_PATTERN),
        "card-bulleted-off" => Some(icons::CARD_BULLETED_OFF),
        "reorder-vertical" => Some(icons::REORDER_VERTICAL),
        "folder-heart-outline" => Some(icons::FOLDER_HEART_OUTLINE),
        "karate" => Some(icons::KARATE),
        #[allow(deprecated)]
        "language-ruby-on-rails" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby-on-rails' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_RUBY_ON_RAILS)
        }
        "numeric-6-circle-outline" => Some(icons::NUMERIC_6_CIRCLE_OUTLINE),
        "tablet" => Some(icons::TABLET),
        "furigana-horizontal" => Some(icons::FURIGANA_HORIZONTAL),
        "text-box-check" => Some(icons::TEXT_BOX_CHECK),
        "message-plus" => Some(icons::MESSAGE_PLUS),
        "cash-100" => Some(icons::CASH_100),
        "bone-off" => Some(icons::BONE_OFF),
        "folder-pound" => Some(icons::FOLDER_POUND),
        "chair-school" => Some(icons::CHAIR_SCHOOL),
        "credit-card-multiple-outline" => Some(icons::CREDIT_CARD_MULTIPLE_OUTLINE),
        "movie-open-remove" => Some(icons::MOVIE_OPEN_REMOVE),
        "high-definition" => Some(icons::HIGH_DEFINITION),
        "syllabary-katakana" => Some(icons::SYLLABARY_KATAKANA),
        "relation-only-one-to-only-one" => Some(icons::RELATION_ONLY_ONE_TO_ONLY_ONE),
        "lock-plus-outline" => Some(icons::LOCK_PLUS_OUTLINE),
        "alpha-a-circle" => Some(icons::ALPHA_A_CIRCLE),
        "reflect-vertical" => Some(icons::REFLECT_VERTICAL),
        "air-conditioner" => Some(icons::AIR_CONDITIONER),
        "train-car-intermodal" => Some(icons::TRAIN_CAR_INTERMODAL),
        "pan-bottom-left" => Some(icons::PAN_BOTTOM_LEFT),
        "shield-home-outline" => Some(icons::SHIELD_HOME_OUTLINE),
        "flask-round-bottom-outline" => Some(icons::FLASK_ROUND_BOTTOM_OUTLINE),
        "beaker-minus" => Some(icons::BEAKER_MINUS),
        "filmstrip-off" => Some(icons::FILMSTRIP_OFF),
        "bell-circle" => Some(icons::BELL_CIRCLE),
        "cookie-check" => Some(icons::COOKIE_CHECK),
        "bell-check-outline" => Some(icons::BELL_CHECK_OUTLINE),
        "movie-open-check-outline" => Some(icons::MOVIE_OPEN_CHECK_OUTLINE),
        "alpha-a-box-outline" => Some(icons::ALPHA_A_BOX_OUTLINE),
        "desk-lamp-off" => Some(icons::DESK_LAMP_OFF),
        "chili-medium" => Some(icons::CHILI_MEDIUM),
        "email-edit" => Some(icons::EMAIL_EDIT),
        "hamburger-minus" => Some(icons::HAMBURGER_MINUS),
        "upload-network" => Some(icons::UPLOAD_NETWORK),
        "search-web" => Some(icons::SEARCH_WEB),
        "broom" => Some(icons::BROOM),
        "bread-slice-outline" => Some(icons::BREAD_SLICE_OUTLINE),
        "radio-off" => Some(icons::RADIO_OFF),
        "cellphone-text" => Some(icons::CELLPHONE_TEXT),
        "menu-left-outline" => Some(icons::MENU_LEFT_OUTLINE),
        "factory" => Some(icons::FACTORY),
        "wallet" => Some(icons::WALLET),
        "scan-helper" => Some(icons::SCAN_HELPER),
        "numeric-4-box-outline" => Some(icons::NUMERIC_4_BOX_OUTLINE),
        "zip-box" => Some(icons::ZIP_BOX),
        "code-string" => Some(icons::CODE_STRING),
        "satellite-variant" => Some(icons::SATELLITE_VARIANT),
        "trash-can" => Some(icons::TRASH_CAN),
        "alphabet-greek" => Some(icons::ALPHABET_GREEK),
        "cannabis-off" => Some(icons::CANNABIS_OFF),
        #[allow(deprecated)]
        "linux-mint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux-mint' is deprecated.").print(py);
            }
            Some(icons::LINUX_MINT)
        }
        "airplane-cog" => Some(icons::AIRPLANE_COG),
        "cookie-clock" => Some(icons::COOKIE_CLOCK),
        "trending-neutral" => Some(icons::TRENDING_NEUTRAL),
        "account-multiple-outline" => Some(icons::ACCOUNT_MULTIPLE_OUTLINE),
        "selection" => Some(icons::SELECTION),
        "timer-plus-outline" => Some(icons::TIMER_PLUS_OUTLINE),
        "eraser" => Some(icons::ERASER),
        "menu-up" => Some(icons::MENU_UP),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "train-car-gondola" => Some(icons::TRAIN_CAR_GONDOLA),
        "video-box-off" => Some(icons::VIDEO_BOX_OFF),
        "progress-download" => Some(icons::PROGRESS_DOWNLOAD),
        "dog-side-off" => Some(icons::DOG_SIDE_OFF),
        "newspaper-variant-multiple-outline" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE_OUTLINE),
        "arrow-horizontal-lock" => Some(icons::ARROW_HORIZONTAL_LOCK),
        "tag" => Some(icons::TAG),
        "flask-outline" => Some(icons::FLASK_OUTLINE),
        "seal-variant" => Some(icons::SEAL_VARIANT),
        "calendar-start-outline" => Some(icons::CALENDAR_START_OUTLINE),
        "death-star-variant" => Some(icons::DEATH_STAR_VARIANT),
        "calendar-arrow-left" => Some(icons::CALENDAR_ARROW_LEFT),
        "valve-closed" => Some(icons::VALVE_CLOSED),
        "message-lock" => Some(icons::MESSAGE_LOCK),
        "glass-stange" => Some(icons::GLASS_STANGE),
        "roller-skate-off" => Some(icons::ROLLER_SKATE_OFF),
        _ => None,
    }
}
