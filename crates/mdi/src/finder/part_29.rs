// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_29(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "car-battery" => Some(icons::CAR_BATTERY),
        "moped" => Some(icons::MOPED),
        "eject-circle" => Some(icons::EJECT_CIRCLE),
        "home-sound-in" => Some(icons::HOME_SOUND_IN),
        "arrow-oscillating" => Some(icons::ARROW_OSCILLATING),
        "headphones-bluetooth" => Some(icons::HEADPHONES_BLUETOOTH),
        #[allow(deprecated)]
        "dot-net" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dot-net' is deprecated.").print(py);
            }
            Some(icons::DOT_NET)
        }
        "shuffle" => Some(icons::SHUFFLE),
        "camera-flip" => Some(icons::CAMERA_FLIP),
        "battery-charging-high" => Some(icons::BATTERY_CHARGING_HIGH),
        "soldering-iron" => Some(icons::SOLDERING_IRON),
        "dots-vertical" => Some(icons::DOTS_VERTICAL),
        "spider-outline" => Some(icons::SPIDER_OUTLINE),
        #[allow(deprecated)]
        "gitlab" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gitlab' is deprecated.").print(py);
            }
            Some(icons::GITLAB)
        }
        "email-arrow-left-outline" => Some(icons::EMAIL_ARROW_LEFT_OUTLINE),
        "arrow-down-drop-circle" => Some(icons::ARROW_DOWN_DROP_CIRCLE),
        "pool" => Some(icons::POOL),
        "cookie-cog-outline" => Some(icons::COOKIE_COG_OUTLINE),
        "close-box-multiple" => Some(icons::CLOSE_BOX_MULTIPLE),
        "menu-left-outline" => Some(icons::MENU_LEFT_OUTLINE),
        "screen-rotation" => Some(icons::SCREEN_ROTATION),
        "card-minus-outline" => Some(icons::CARD_MINUS_OUTLINE),
        "plus-minus-box" => Some(icons::PLUS_MINUS_BOX),
        "puzzle-remove" => Some(icons::PUZZLE_REMOVE),
        "car-windshield-outline" => Some(icons::CAR_WINDSHIELD_OUTLINE),
        "ruler-square-compass" => Some(icons::RULER_SQUARE_COMPASS),
        #[allow(deprecated)]
        "goodreads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'goodreads' is deprecated.").print(py);
            }
            Some(icons::GOODREADS)
        }
        "voicemail" => Some(icons::VOICEMAIL),
        "weight-pound" => Some(icons::WEIGHT_POUND),
        "train-car-box" => Some(icons::TRAIN_CAR_BOX),
        "toy-brick" => Some(icons::TOY_BRICK),
        "archive-cancel-outline" => Some(icons::ARCHIVE_CANCEL_OUTLINE),
        "lock-off" => Some(icons::LOCK_OFF),
        "cloud-alert-outline" => Some(icons::CLOUD_ALERT_OUTLINE),
        "attachment-check" => Some(icons::ATTACHMENT_CHECK),
        "comment-alert-outline" => Some(icons::COMMENT_ALERT_OUTLINE),
        "home-automation" => Some(icons::HOME_AUTOMATION),
        "database-cog-outline" => Some(icons::DATABASE_COG_OUTLINE),
        "boom-gate-alert" => Some(icons::BOOM_GATE_ALERT),
        "food" => Some(icons::FOOD),
        "filter-minus" => Some(icons::FILTER_MINUS),
        "storefront-outline" => Some(icons::STOREFRONT_OUTLINE),
        "bus-stop" => Some(icons::BUS_STOP),
        "share-circle" => Some(icons::SHARE_CIRCLE),
        "chevron-triple-down" => Some(icons::CHEVRON_TRIPLE_DOWN),
        "transfer" => Some(icons::TRANSFER),
        "arrow-up-down-bold-outline" => Some(icons::ARROW_UP_DOWN_BOLD_OUTLINE),
        "content-save-move" => Some(icons::CONTENT_SAVE_MOVE),
        "baguette" => Some(icons::BAGUETTE),
        "chess-queen" => Some(icons::CHESS_QUEEN),
        "bluetooth-settings" => Some(icons::BLUETOOTH_SETTINGS),
        "home-lock" => Some(icons::HOME_LOCK),
        "chat-minus" => Some(icons::CHAT_MINUS),
        "dice-6" => Some(icons::DICE_6),
        "alarm-light-off-outline" => Some(icons::ALARM_LIGHT_OFF_OUTLINE),
        "account-credit-card-outline" => Some(icons::ACCOUNT_CREDIT_CARD_OUTLINE),
        "fit-to-screen-outline" => Some(icons::FIT_TO_SCREEN_OUTLINE),
        "database-refresh" => Some(icons::DATABASE_REFRESH),
        "alpha-x-box-outline" => Some(icons::ALPHA_X_BOX_OUTLINE),
        "cookie-clock-outline" => Some(icons::COOKIE_CLOCK_OUTLINE),
        "chevron-right" => Some(icons::CHEVRON_RIGHT),
        "logout" => Some(icons::LOGOUT),
        "trophy-variant-outline" => Some(icons::TROPHY_VARIANT_OUTLINE),
        "comment-remove" => Some(icons::COMMENT_REMOVE),
        "waves-arrow-right" => Some(icons::WAVES_ARROW_RIGHT),
        "ambulance" => Some(icons::AMBULANCE),
        #[allow(deprecated)]
        "google-street-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-street-view' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_STREET_VIEW)
        }
        "delete-empty" => Some(icons::DELETE_EMPTY),
        "handshake-outline" => Some(icons::HANDSHAKE_OUTLINE),
        "timeline-alert" => Some(icons::TIMELINE_ALERT),
        "comment-multiple-outline" => Some(icons::COMMENT_MULTIPLE_OUTLINE),
        "briefcase-minus" => Some(icons::BRIEFCASE_MINUS),
        "beehive-outline" => Some(icons::BEEHIVE_OUTLINE),
        "hand-back-right-outline" => Some(icons::HAND_BACK_RIGHT_OUTLINE),
        "head-check" => Some(icons::HEAD_CHECK),
        "clock-in" => Some(icons::CLOCK_IN),
        "basket-unfill" => Some(icons::BASKET_UNFILL),
        "database-off" => Some(icons::DATABASE_OFF),
        "cog-counterclockwise" => Some(icons::COG_COUNTERCLOCKWISE),
        "web-off" => Some(icons::WEB_OFF),
        "movie-search-outline" => Some(icons::MOVIE_SEARCH_OUTLINE),
        "human" => Some(icons::HUMAN),
        "thermometer-alert" => Some(icons::THERMOMETER_ALERT),
        "tag-arrow-left" => Some(icons::TAG_ARROW_LEFT),
        "battery" => Some(icons::BATTERY),
        "water-boiler-auto" => Some(icons::WATER_BOILER_AUTO),
        "spirit-level" => Some(icons::SPIRIT_LEVEL),
        "phone-plus" => Some(icons::PHONE_PLUS),
        "archive-refresh-outline" => Some(icons::ARCHIVE_REFRESH_OUTLINE),
        "phone-minus" => Some(icons::PHONE_MINUS),
        "filter-multiple" => Some(icons::FILTER_MULTIPLE),
        "key-outline" => Some(icons::KEY_OUTLINE),
        #[allow(deprecated)]
        "freebsd" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'freebsd' is deprecated.").print(py);
            }
            Some(icons::FREEBSD)
        }
        "account-lock" => Some(icons::ACCOUNT_LOCK),
        "phone-bluetooth" => Some(icons::PHONE_BLUETOOTH),
        "shape-rectangle-plus" => Some(icons::SHAPE_RECTANGLE_PLUS),
        "grain" => Some(icons::GRAIN),
        "integrated-circuit-chip" => Some(icons::INTEGRATED_CIRCUIT_CHIP),
        #[allow(deprecated)]
        "microsoft-xbox-controller-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-view' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_VIEW)
        }
        "delete-clock" => Some(icons::DELETE_CLOCK),
        "podium-bronze" => Some(icons::PODIUM_BRONZE),
        #[allow(deprecated)]
        "nintendo-wii" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wii' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WII)
        }
        "sickle" => Some(icons::SICKLE),
        "store-minus-outline" => Some(icons::STORE_MINUS_OUTLINE),
        "star-crescent" => Some(icons::STAR_CRESCENT),
        "gate-open" => Some(icons::GATE_OPEN),
        "dice-3" => Some(icons::DICE_3),
        "account-network-off" => Some(icons::ACCOUNT_NETWORK_OFF),
        #[allow(deprecated)]
        "microsoft-visual-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-visual-studio' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_VISUAL_STUDIO)
        }
        "text" => Some(icons::TEXT),
        "application-braces-outline" => Some(icons::APPLICATION_BRACES_OUTLINE),
        "emoticon-plus" => Some(icons::EMOTICON_PLUS),
        "format-list-group" => Some(icons::FORMAT_LIST_GROUP),
        "currency-gbp" => Some(icons::CURRENCY_GBP),
        "podcast" => Some(icons::PODCAST),
        "store-marker-outline" => Some(icons::STORE_MARKER_OUTLINE),
        "battery-80-bluetooth" => Some(icons::BATTERY_80_BLUETOOTH),
        "shield-sun" => Some(icons::SHIELD_SUN),
        "school-outline" => Some(icons::SCHOOL_OUTLINE),
        "coffee-off" => Some(icons::COFFEE_OFF),
        "smoke" => Some(icons::SMOKE),
        "human-non-binary" => Some(icons::HUMAN_NON_BINARY),
        "toolbox-outline" => Some(icons::TOOLBOX_OUTLINE),
        "content-save-cog-outline" => Some(icons::CONTENT_SAVE_COG_OUTLINE),
        "meter-electric" => Some(icons::METER_ELECTRIC),
        "book-open" => Some(icons::BOOK_OPEN),
        "home-edit-outline" => Some(icons::HOME_EDIT_OUTLINE),
        "bike" => Some(icons::BIKE),
        "bag-personal" => Some(icons::BAG_PERSONAL),
        "table-cancel" => Some(icons::TABLE_CANCEL),
        "paper-roll-outline" => Some(icons::PAPER_ROLL_OUTLINE),
        "shape-outline" => Some(icons::SHAPE_OUTLINE),
        "arrow-right-box" => Some(icons::ARROW_RIGHT_BOX),
        "shield-alert" => Some(icons::SHIELD_ALERT),
        "queue-first-in-last-out" => Some(icons::QUEUE_FIRST_IN_LAST_OUT),
        "roman-numeral-5" => Some(icons::ROMAN_NUMERAL_5),
        "spray-bottle" => Some(icons::SPRAY_BOTTLE),
        "car-search" => Some(icons::CAR_SEARCH),
        "format-align-top" => Some(icons::FORMAT_ALIGN_TOP),
        "card-search" => Some(icons::CARD_SEARCH),
        #[allow(deprecated)]
        "soundcloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'soundcloud' is deprecated.").print(py);
            }
            Some(icons::SOUNDCLOUD)
        }
        "video" => Some(icons::VIDEO),
        "door-sliding-open" => Some(icons::DOOR_SLIDING_OPEN),
        "file-image-plus" => Some(icons::FILE_IMAGE_PLUS),
        "magnify-remove-cursor" => Some(icons::MAGNIFY_REMOVE_CURSOR),
        "phone-classic" => Some(icons::PHONE_CLASSIC),
        "phone-paused-outline" => Some(icons::PHONE_PAUSED_OUTLINE),
        "basket-minus" => Some(icons::BASKET_MINUS),
        "hand-front-right-outline" => Some(icons::HAND_FRONT_RIGHT_OUTLINE),
        "battery-heart-variant" => Some(icons::BATTERY_HEART_VARIANT),
        "lightbulb-on" => Some(icons::LIGHTBULB_ON),
        "gesture-pinch" => Some(icons::GESTURE_PINCH),
        "book-cog-outline" => Some(icons::BOOK_COG_OUTLINE),
        "view-dashboard-outline" => Some(icons::VIEW_DASHBOARD_OUTLINE),
        "network-strength-4" => Some(icons::NETWORK_STRENGTH_4),
        "battery-charging-wireless-70" => Some(icons::BATTERY_CHARGING_WIRELESS_70),
        "bus" => Some(icons::BUS),
        "arrow-up" => Some(icons::ARROW_UP),
        "shield-alert-outline" => Some(icons::SHIELD_ALERT_OUTLINE),
        #[allow(deprecated)]
        "nfc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nfc' is deprecated.").print(py);
            }
            Some(icons::NFC)
        }
        "rewind-60" => Some(icons::REWIND_60),
        "file-move" => Some(icons::FILE_MOVE),
        "slide" => Some(icons::SLIDE),
        "arrow-collapse-all" => Some(icons::ARROW_COLLAPSE_ALL),
        "information-slab-box" => Some(icons::INFORMATION_SLAB_BOX),
        "creation-outline" => Some(icons::CREATION_OUTLINE),
        "lock-remove" => Some(icons::LOCK_REMOVE),
        "finance" => Some(icons::FINANCE),
        "file-certificate-outline" => Some(icons::FILE_CERTIFICATE_OUTLINE),
        "file-lock-open" => Some(icons::FILE_LOCK_OPEN),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "cast-audio" => Some(icons::CAST_AUDIO),
        "battery-unknown-bluetooth" => Some(icons::BATTERY_UNKNOWN_BLUETOOTH),
        "view-stream" => Some(icons::VIEW_STREAM),
        "eye-lock" => Some(icons::EYE_LOCK),
        "drawing" => Some(icons::DRAWING),
        "clipboard-text-play-outline" => Some(icons::CLIPBOARD_TEXT_PLAY_OUTLINE),
        "hvac-off" => Some(icons::HVAC_OFF),
        "page-first" => Some(icons::PAGE_FIRST),
        "stocking" => Some(icons::STOCKING),
        "yeast" => Some(icons::YEAST),
        "cursor-text" => Some(icons::CURSOR_TEXT),
        "office-building-cog" => Some(icons::OFFICE_BUILDING_COG),
        "beehive-off-outline" => Some(icons::BEEHIVE_OFF_OUTLINE),
        "menu-down" => Some(icons::MENU_DOWN),
        "sim-alert-outline" => Some(icons::SIM_ALERT_OUTLINE),
        "playlist-remove" => Some(icons::PLAYLIST_REMOVE),
        "hamburger-plus" => Some(icons::HAMBURGER_PLUS),
        "account-cancel-outline" => Some(icons::ACCOUNT_CANCEL_OUTLINE),
        "cube-outline" => Some(icons::CUBE_OUTLINE),
        "blinds-vertical" => Some(icons::BLINDS_VERTICAL),
        "car-info" => Some(icons::CAR_INFO),
        "tag-arrow-right" => Some(icons::TAG_ARROW_RIGHT),
        "human-greeting-proximity" => Some(icons::HUMAN_GREETING_PROXIMITY),
        "map-marker-alert" => Some(icons::MAP_MARKER_ALERT),
        "account-multiple-plus-outline" => Some(icons::ACCOUNT_MULTIPLE_PLUS_OUTLINE),
        "file-sign" => Some(icons::FILE_SIGN),
        "store-clock" => Some(icons::STORE_CLOCK),
        "flag-variant-plus" => Some(icons::FLAG_VARIANT_PLUS),
        "arrow-down-left-bold" => Some(icons::ARROW_DOWN_LEFT_BOLD),
        _ => None,
    }
}
