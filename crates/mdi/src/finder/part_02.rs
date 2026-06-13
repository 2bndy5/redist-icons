// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_2(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "car-electric-outline" => Some(icons::CAR_ELECTRIC_OUTLINE),
        "food-outline" => Some(icons::FOOD_OUTLINE),
        "menu-swap-outline" => Some(icons::MENU_SWAP_OUTLINE),
        "book-music" => Some(icons::BOOK_MUSIC),
        "alien" => Some(icons::ALIEN),
        "wifi-strength-1-lock" => Some(icons::WIFI_STRENGTH_1_LOCK),
        "calendar-star-four-points" => Some(icons::CALENDAR_STAR_FOUR_POINTS),
        "camera-front-variant" => Some(icons::CAMERA_FRONT_VARIANT),
        "cards-club" => Some(icons::CARDS_CLUB),
        "dots-vertical" => Some(icons::DOTS_VERTICAL),
        "transit-connection-horizontal" => Some(icons::TRANSIT_CONNECTION_HORIZONTAL),
        "emoticon-confused-outline" => Some(icons::EMOTICON_CONFUSED_OUTLINE),
        #[allow(deprecated)]
        "gentoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gentoo' is deprecated.").print(py);
            }
            Some(icons::GENTOO)
        }
        "tree" => Some(icons::TREE),
        "motion-sensor-off" => Some(icons::MOTION_SENSOR_OFF),
        "printer" => Some(icons::PRINTER),
        "folder-check" => Some(icons::FOLDER_CHECK),
        "battery-plus-outline" => Some(icons::BATTERY_PLUS_OUTLINE),
        "apple-keyboard-option" => Some(icons::APPLE_KEYBOARD_OPTION),
        "currency-inr" => Some(icons::CURRENCY_INR),
        "numeric-5-box" => Some(icons::NUMERIC_5_BOX),
        "moon-new" => Some(icons::MOON_NEW),
        "numeric-1-circle-outline" => Some(icons::NUMERIC_1_CIRCLE_OUTLINE),
        "airplane-alert" => Some(icons::AIRPLANE_ALERT),
        "alpha-m-box-outline" => Some(icons::ALPHA_M_BOX_OUTLINE),
        "content-save-edit-outline" => Some(icons::CONTENT_SAVE_EDIT_OUTLINE),
        "format-quote-open" => Some(icons::FORMAT_QUOTE_OPEN),
        "playlist-star" => Some(icons::PLAYLIST_STAR),
        "car-back" => Some(icons::CAR_BACK),
        "store-search" => Some(icons::STORE_SEARCH),
        "repeat" => Some(icons::REPEAT),
        "basket-remove-outline" => Some(icons::BASKET_REMOVE_OUTLINE),
        "looks" => Some(icons::LOOKS),
        "trending-up" => Some(icons::TRENDING_UP),
        "dots-horizontal-circle" => Some(icons::DOTS_HORIZONTAL_CIRCLE),
        "landslide" => Some(icons::LANDSLIDE),
        "bell-cog-outline" => Some(icons::BELL_COG_OUTLINE),
        #[allow(deprecated)]
        "material-design" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-design' is deprecated.")
                    .print(py);
            }
            Some(icons::MATERIAL_DESIGN)
        }
        "file-clock-outline" => Some(icons::FILE_CLOCK_OUTLINE),
        "account-multiple-check-outline" => Some(icons::ACCOUNT_MULTIPLE_CHECK_OUTLINE),
        "alpha-t-circle" => Some(icons::ALPHA_T_CIRCLE),
        "basket-off-outline" => Some(icons::BASKET_OFF_OUTLINE),
        "tab-minus" => Some(icons::TAB_MINUS),
        "auto-upload" => Some(icons::AUTO_UPLOAD),
        "eye-outline" => Some(icons::EYE_OUTLINE),
        "cursor-default-gesture-outline" => Some(icons::CURSOR_DEFAULT_GESTURE_OUTLINE),
        "cart-off" => Some(icons::CART_OFF),
        "map-check" => Some(icons::MAP_CHECK),
        "chili-off-outline" => Some(icons::CHILI_OFF_OUTLINE),
        "bag-personal-off" => Some(icons::BAG_PERSONAL_OFF),
        "ammunition" => Some(icons::AMMUNITION),
        "pencil-minus-outline" => Some(icons::PENCIL_MINUS_OUTLINE),
        "rhombus-outline" => Some(icons::RHOMBUS_OUTLINE),
        "distribute-vertical-top" => Some(icons::DISTRIBUTE_VERTICAL_TOP),
        "seat-legroom-extra" => Some(icons::SEAT_LEGROOM_EXTRA),
        "rhombus-medium" => Some(icons::RHOMBUS_MEDIUM),
        "home-floor-1" => Some(icons::HOME_FLOOR_1),
        "square-circle-outline" => Some(icons::SQUARE_CIRCLE_OUTLINE),
        "cast-variant" => Some(icons::CAST_VARIANT),
        "relation-one-to-one" => Some(icons::RELATION_ONE_TO_ONE),
        "timer-cog" => Some(icons::TIMER_COG),
        "rotate-right-variant" => Some(icons::ROTATE_RIGHT_VARIANT),
        "robot-mower" => Some(icons::ROBOT_MOWER),
        "syllabary-hiragana" => Some(icons::SYLLABARY_HIRAGANA),
        "store-minus-outline" => Some(icons::STORE_MINUS_OUTLINE),
        "horizontal-rotate-counterclockwise" => Some(icons::HORIZONTAL_ROTATE_COUNTERCLOCKWISE),
        "human-male-child" => Some(icons::HUMAN_MALE_CHILD),
        "view-module-outline" => Some(icons::VIEW_MODULE_OUTLINE),
        "baby-bottle-outline" => Some(icons::BABY_BOTTLE_OUTLINE),
        "ufo-outline" => Some(icons::UFO_OUTLINE),
        "lightbulb-on-90" => Some(icons::LIGHTBULB_ON_90),
        "handcuffs" => Some(icons::HANDCUFFS),
        "thermometer-probe" => Some(icons::THERMOMETER_PROBE),
        "circle-slice-6" => Some(icons::CIRCLE_SLICE_6),
        "rss-off" => Some(icons::RSS_OFF),
        "cog-pause" => Some(icons::COG_PAUSE),
        "format-size" => Some(icons::FORMAT_SIZE),
        "pause" => Some(icons::PAUSE),
        "image-area" => Some(icons::IMAGE_AREA),
        "table-sync" => Some(icons::TABLE_SYNC),
        "store-plus-outline" => Some(icons::STORE_PLUS_OUTLINE),
        "application-parentheses" => Some(icons::APPLICATION_PARENTHESES),
        "shredder" => Some(icons::SHREDDER),
        "car-brake-parking" => Some(icons::CAR_BRAKE_PARKING),
        "home-lock-open" => Some(icons::HOME_LOCK_OPEN),
        "calendar-filter" => Some(icons::CALENDAR_FILTER),
        "zip-disk" => Some(icons::ZIP_DISK),
        #[allow(deprecated)]
        "unity" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unity' is deprecated.").print(py);
            }
            Some(icons::UNITY)
        }
        "printer-3d-off" => Some(icons::PRINTER_3D_OFF),
        "keyboard-space" => Some(icons::KEYBOARD_SPACE),
        "printer-pos-pause" => Some(icons::PRINTER_POS_PAUSE),
        "order-alphabetical-ascending" => Some(icons::ORDER_ALPHABETICAL_ASCENDING),
        "arrow-bottom-left-thin" => Some(icons::ARROW_BOTTOM_LEFT_THIN),
        "account-search" => Some(icons::ACCOUNT_SEARCH),
        "multimedia" => Some(icons::MULTIMEDIA),
        "heart-box-outline" => Some(icons::HEART_BOX_OUTLINE),
        "card-outline" => Some(icons::CARD_OUTLINE),
        "train-car-container" => Some(icons::TRAIN_CAR_CONTAINER),
        "garage" => Some(icons::GARAGE),
        "car-sports" => Some(icons::CAR_SPORTS),
        "car-clock" => Some(icons::CAR_CLOCK),
        #[allow(deprecated)]
        "apple-ios" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-ios' is deprecated.").print(py);
            }
            Some(icons::APPLE_IOS)
        }
        "checkbox-multiple-outline" => Some(icons::CHECKBOX_MULTIPLE_OUTLINE),
        "filmstrip-box-multiple" => Some(icons::FILMSTRIP_BOX_MULTIPLE),
        "hand-back-left-off" => Some(icons::HAND_BACK_LEFT_OFF),
        "numeric-10-box-multiple-outline" => Some(icons::NUMERIC_10_BOX_MULTIPLE_OUTLINE),
        "dishwasher-alert" => Some(icons::DISHWASHER_ALERT),
        "arrow-bottom-left-bold-box" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_BOX),
        "skate" => Some(icons::SKATE),
        "camera-marker" => Some(icons::CAMERA_MARKER),
        "tray-arrow-down" => Some(icons::TRAY_ARROW_DOWN),
        "pentagon" => Some(icons::PENTAGON),
        "server" => Some(icons::SERVER),
        "taxi" => Some(icons::TAXI),
        "window-open" => Some(icons::WINDOW_OPEN),
        "apple-keyboard-command" => Some(icons::APPLE_KEYBOARD_COMMAND),
        "account-cash" => Some(icons::ACCOUNT_CASH),
        "file-lock-open-outline" => Some(icons::FILE_LOCK_OPEN_OUTLINE),
        "sticker-text" => Some(icons::STICKER_TEXT),
        "star-check-outline" => Some(icons::STAR_CHECK_OUTLINE),
        "headphones-bluetooth" => Some(icons::HEADPHONES_BLUETOOTH),
        #[allow(deprecated)]
        "git" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'git' is deprecated.").print(py);
            }
            Some(icons::GIT)
        }
        "clock-outline" => Some(icons::CLOCK_OUTLINE),
        "gender-male-female-variant" => Some(icons::GENDER_MALE_FEMALE_VARIANT),
        "file-table-outline" => Some(icons::FILE_TABLE_OUTLINE),
        "set-center-right" => Some(icons::SET_CENTER_RIGHT),
        "temperature-kelvin" => Some(icons::TEMPERATURE_KELVIN),
        "battery-positive" => Some(icons::BATTERY_POSITIVE),
        "hand-wash-outline" => Some(icons::HAND_WASH_OUTLINE),
        "focus-field-horizontal" => Some(icons::FOCUS_FIELD_HORIZONTAL),
        "star-cog-outline" => Some(icons::STAR_COG_OUTLINE),
        "alpha-r-box-outline" => Some(icons::ALPHA_R_BOX_OUTLINE),
        "sun-compass" => Some(icons::SUN_COMPASS),
        "plus-circle-multiple" => Some(icons::PLUS_CIRCLE_MULTIPLE),
        "receipt-text-plus-outline" => Some(icons::RECEIPT_TEXT_PLUS_OUTLINE),
        "zodiac-capricorn" => Some(icons::ZODIAC_CAPRICORN),
        "cylinder" => Some(icons::CYLINDER),
        "shield-airplane-outline" => Some(icons::SHIELD_AIRPLANE_OUTLINE),
        "fruit-citrus-off" => Some(icons::FRUIT_CITRUS_OFF),
        "alpha-q-box" => Some(icons::ALPHA_Q_BOX),
        "coffee-maker-check-outline" => Some(icons::COFFEE_MAKER_CHECK_OUTLINE),
        "drag-horizontal" => Some(icons::DRAG_HORIZONTAL),
        "view-week-outline" => Some(icons::VIEW_WEEK_OUTLINE),
        "hook" => Some(icons::HOOK),
        "tab" => Some(icons::TAB),
        "minus-circle-multiple-outline" => Some(icons::MINUS_CIRCLE_MULTIPLE_OUTLINE),
        "size-xl" => Some(icons::SIZE_XL),
        "awning-outline" => Some(icons::AWNING_OUTLINE),
        #[allow(deprecated)]
        "google-podcast" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-podcast' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PODCAST)
        }
        "store-outline" => Some(icons::STORE_OUTLINE),
        "pencil-lock" => Some(icons::PENCIL_LOCK),
        "database-arrow-left" => Some(icons::DATABASE_ARROW_LEFT),
        "fan-auto" => Some(icons::FAN_AUTO),
        "comment-alert-outline" => Some(icons::COMMENT_ALERT_OUTLINE),
        "cellphone-off" => Some(icons::CELLPHONE_OFF),
        "scent-off" => Some(icons::SCENT_OFF),
        #[allow(deprecated)]
        "pokemon-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pokemon-go' is deprecated.").print(py);
            }
            Some(icons::POKEMON_GO)
        }
        "poker-chip" => Some(icons::POKER_CHIP),
        "lock-percent-open-variant" => Some(icons::LOCK_PERCENT_OPEN_VARIANT),
        "camera-lock-open" => Some(icons::CAMERA_LOCK_OPEN),
        "wrench-clock" => Some(icons::WRENCH_CLOCK),
        #[allow(deprecated)]
        "bitcoin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bitcoin' is deprecated.").print(py);
            }
            Some(icons::BITCOIN)
        }
        "fan-minus" => Some(icons::FAN_MINUS),
        "folder-image" => Some(icons::FOLDER_IMAGE),
        "sticker-plus-outline" => Some(icons::STICKER_PLUS_OUTLINE),
        "chili-mild" => Some(icons::CHILI_MILD),
        "numeric-0-circle" => Some(icons::NUMERIC_0_CIRCLE),
        "email-open" => Some(icons::EMAIL_OPEN),
        "play-circle-outline" => Some(icons::PLAY_CIRCLE_OUTLINE),
        "numeric-7-circle-outline" => Some(icons::NUMERIC_7_CIRCLE_OUTLINE),
        "volume-low" => Some(icons::VOLUME_LOW),
        "knife-military" => Some(icons::KNIFE_MILITARY),
        "alert-octagram-outline" => Some(icons::ALERT_OCTAGRAM_OUTLINE),
        "signal-distance-variant" => Some(icons::SIGNAL_DISTANCE_VARIANT),
        "file-download" => Some(icons::FILE_DOWNLOAD),
        "mouse-variant-off" => Some(icons::MOUSE_VARIANT_OFF),
        "star-crescent" => Some(icons::STAR_CRESCENT),
        "progress-star-four-points" => Some(icons::PROGRESS_STAR_FOUR_POINTS),
        "view-sequential" => Some(icons::VIEW_SEQUENTIAL),
        "comment-plus" => Some(icons::COMMENT_PLUS),
        "folder-zip" => Some(icons::FOLDER_ZIP),
        "download-box-outline" => Some(icons::DOWNLOAD_BOX_OUTLINE),
        "ballot-recount" => Some(icons::BALLOT_RECOUNT),
        "timer-play-outline" => Some(icons::TIMER_PLAY_OUTLINE),
        "music-clef-treble" => Some(icons::MUSIC_CLEF_TREBLE),
        "home-battery-outline" => Some(icons::HOME_BATTERY_OUTLINE),
        "tag-minus" => Some(icons::TAG_MINUS),
        "bowling" => Some(icons::BOWLING),
        "carrot" => Some(icons::CARROT),
        "details" => Some(icons::DETAILS),
        "pan-down" => Some(icons::PAN_DOWN),
        "inbox-arrow-up-outline" => Some(icons::INBOX_ARROW_UP_OUTLINE),
        "currency-try" => Some(icons::CURRENCY_TRY),
        "comment-flash-outline" => Some(icons::COMMENT_FLASH_OUTLINE),
        "timeline-check-outline" => Some(icons::TIMELINE_CHECK_OUTLINE),
        "train-car-hopper-full" => Some(icons::TRAIN_CAR_HOPPER_FULL),
        "rewind-10" => Some(icons::REWIND_10),
        "crop-square" => Some(icons::CROP_SQUARE),
        "signal-3g" => Some(icons::SIGNAL_3G),
        "battery-80-bluetooth" => Some(icons::BATTERY_80_BLUETOOTH),
        _ => None,
    }
}
