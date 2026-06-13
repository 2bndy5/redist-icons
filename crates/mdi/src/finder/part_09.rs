// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_9(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "arrow-down-left-bold" => Some(icons::ARROW_DOWN_LEFT_BOLD),
        "arrange-send-backward" => Some(icons::ARRANGE_SEND_BACKWARD),
        "dialpad" => Some(icons::DIALPAD),
        "periodic-table" => Some(icons::PERIODIC_TABLE),
        "axis" => Some(icons::AXIS),
        "arrow-left-box" => Some(icons::ARROW_LEFT_BOX),
        "quadcopter" => Some(icons::QUADCOPTER),
        "timer-music" => Some(icons::TIMER_MUSIC),
        "clock-time-one" => Some(icons::CLOCK_TIME_ONE),
        "alpha-k-circle" => Some(icons::ALPHA_K_CIRCLE),
        "flash" => Some(icons::FLASH),
        #[allow(deprecated)]
        "jsfiddle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jsfiddle' is deprecated.").print(py);
            }
            Some(icons::JSFIDDLE)
        }
        "receipt-text-arrow-left-outline" => Some(icons::RECEIPT_TEXT_ARROW_LEFT_OUTLINE),
        "battery-charging-high" => Some(icons::BATTERY_CHARGING_HIGH),
        "door-sliding-lock" => Some(icons::DOOR_SLIDING_LOCK),
        "cassette" => Some(icons::CASSETTE),
        "hdmi-port" => Some(icons::HDMI_PORT),
        "minus-circle-multiple" => Some(icons::MINUS_CIRCLE_MULTIPLE),
        "menu-right-outline" => Some(icons::MENU_RIGHT_OUTLINE),
        "signal-cellular-2" => Some(icons::SIGNAL_CELLULAR_2),
        "skateboarding" => Some(icons::SKATEBOARDING),
        "liquor" => Some(icons::LIQUOR),
        "tag-search-outline" => Some(icons::TAG_SEARCH_OUTLINE),
        "rabbit-variant" => Some(icons::RABBIT_VARIANT),
        "music-note-plus" => Some(icons::MUSIC_NOTE_PLUS),
        "smart-card-off-outline" => Some(icons::SMART_CARD_OFF_OUTLINE),
        "image-size-select-actual" => Some(icons::IMAGE_SIZE_SELECT_ACTUAL),
        "badge-account-alert-outline" => Some(icons::BADGE_ACCOUNT_ALERT_OUTLINE),
        "help-network" => Some(icons::HELP_NETWORK),
        "calendar-heart" => Some(icons::CALENDAR_HEART),
        "projector-screen-outline" => Some(icons::PROJECTOR_SCREEN_OUTLINE),
        "bag-personal-outline" => Some(icons::BAG_PERSONAL_OUTLINE),
        "violin" => Some(icons::VIOLIN),
        "flag-variant-remove" => Some(icons::FLAG_VARIANT_REMOVE),
        "video-outline" => Some(icons::VIDEO_OUTLINE),
        "keyboard-f3" => Some(icons::KEYBOARD_F3),
        #[allow(deprecated)]
        "nodejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nodejs' is deprecated.").print(py);
            }
            Some(icons::NODEJS)
        }
        "file-cancel" => Some(icons::FILE_CANCEL),
        "food-hot-dog" => Some(icons::FOOD_HOT_DOG),
        "flask-remove-outline" => Some(icons::FLASK_REMOVE_OUTLINE),
        "jellyfish-outline" => Some(icons::JELLYFISH_OUTLINE),
        "help-rhombus-outline" => Some(icons::HELP_RHOMBUS_OUTLINE),
        "format-letter-case-upper" => Some(icons::FORMAT_LETTER_CASE_UPPER),
        "qrcode-minus" => Some(icons::QRCODE_MINUS),
        "chef-hat" => Some(icons::CHEF_HAT),
        "lamp-outline" => Some(icons::LAMP_OUTLINE),
        "content-save-minus-outline" => Some(icons::CONTENT_SAVE_MINUS_OUTLINE),
        "map-plus" => Some(icons::MAP_PLUS),
        "window-shutter" => Some(icons::WINDOW_SHUTTER),
        "wan" => Some(icons::WAN),
        "alert-octagon-outline" => Some(icons::ALERT_OCTAGON_OUTLINE),
        "alphabet-latin" => Some(icons::ALPHABET_LATIN),
        "piano" => Some(icons::PIANO),
        "printer-pos-star-outline" => Some(icons::PRINTER_POS_STAR_OUTLINE),
        "checkbox-multiple-blank-outline" => Some(icons::CHECKBOX_MULTIPLE_BLANK_OUTLINE),
        #[allow(deprecated)]
        "simple-icons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'simple-icons' is deprecated.").print(py);
            }
            Some(icons::SIMPLE_ICONS)
        }
        "bottle-soda-classic" => Some(icons::BOTTLE_SODA_CLASSIC),
        "weather-hazy" => Some(icons::WEATHER_HAZY),
        "music-accidental-double-sharp" => Some(icons::MUSIC_ACCIDENTAL_DOUBLE_SHARP),
        "timer-sync-outline" => Some(icons::TIMER_SYNC_OUTLINE),
        "battery-30" => Some(icons::BATTERY_30),
        "train-car" => Some(icons::TRAIN_CAR),
        "close-box" => Some(icons::CLOSE_BOX),
        "washing-machine-off" => Some(icons::WASHING_MACHINE_OFF),
        "collapse-all-outline" => Some(icons::COLLAPSE_ALL_OUTLINE),
        "beaker-check-outline" => Some(icons::BEAKER_CHECK_OUTLINE),
        "tshirt-v" => Some(icons::TSHIRT_V),
        "application" => Some(icons::APPLICATION),
        "printer-pos-cancel" => Some(icons::PRINTER_POS_CANCEL),
        "gesture-swipe-vertical" => Some(icons::GESTURE_SWIPE_VERTICAL),
        "keyboard-variant" => Some(icons::KEYBOARD_VARIANT),
        "card-plus-outline" => Some(icons::CARD_PLUS_OUTLINE),
        "boom-gate-alert-outline" => Some(icons::BOOM_GATE_ALERT_OUTLINE),
        "account-heart" => Some(icons::ACCOUNT_HEART),
        #[allow(deprecated)]
        "eslint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'eslint' is deprecated.").print(py);
            }
            Some(icons::ESLINT)
        }
        "horseshoe" => Some(icons::HORSESHOE),
        "unicorn" => Some(icons::UNICORN),
        "chess-bishop" => Some(icons::CHESS_BISHOP),
        "content-save-alert-outline" => Some(icons::CONTENT_SAVE_ALERT_OUTLINE),
        "format-text-rotation-vertical" => Some(icons::FORMAT_TEXT_ROTATION_VERTICAL),
        "water-boiler" => Some(icons::WATER_BOILER),
        "grass" => Some(icons::GRASS),
        "view-dashboard-variant-outline" => Some(icons::VIEW_DASHBOARD_VARIANT_OUTLINE),
        "tag-arrow-left-outline" => Some(icons::TAG_ARROW_LEFT_OUTLINE),
        "flask-empty" => Some(icons::FLASK_EMPTY),
        "wall-sconce-round-variant" => Some(icons::WALL_SCONCE_ROUND_VARIANT),
        "chevron-up-box-outline" => Some(icons::CHEVRON_UP_BOX_OUTLINE),
        #[allow(deprecated)]
        "angular" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'angular' is deprecated.").print(py);
            }
            Some(icons::ANGULAR)
        }
        "account-star-outline" => Some(icons::ACCOUNT_STAR_OUTLINE),
        "microphone-outline" => Some(icons::MICROPHONE_OUTLINE),
        "cookie-minus-outline" => Some(icons::COOKIE_MINUS_OUTLINE),
        "format-list-bulleted" => Some(icons::FORMAT_LIST_BULLETED),
        #[allow(deprecated)]
        "apple-icloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-icloud' is deprecated.").print(py);
            }
            Some(icons::APPLE_ICLOUD)
        }
        "home-lock" => Some(icons::HOME_LOCK),
        "car-light-dimmed" => Some(icons::CAR_LIGHT_DIMMED),
        "store-minus" => Some(icons::STORE_MINUS),
        "lightbulb-fluorescent-tube" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE),
        "square-root" => Some(icons::SQUARE_ROOT),
        "send-clock-outline" => Some(icons::SEND_CLOCK_OUTLINE),
        "clippy" => Some(icons::CLIPPY),
        "alpha-w-box-outline" => Some(icons::ALPHA_W_BOX_OUTLINE),
        "alpha-o-box" => Some(icons::ALPHA_O_BOX),
        "fraction-one-half" => Some(icons::FRACTION_ONE_HALF),
        "battery-50-bluetooth" => Some(icons::BATTERY_50_BLUETOOTH),
        "comment-arrow-left-outline" => Some(icons::COMMENT_ARROW_LEFT_OUTLINE),
        "circular-saw" => Some(icons::CIRCULAR_SAW),
        "comment-account" => Some(icons::COMMENT_ACCOUNT),
        "account-circle" => Some(icons::ACCOUNT_CIRCLE),
        "content-save-all" => Some(icons::CONTENT_SAVE_ALL),
        "signature" => Some(icons::SIGNATURE),
        "star-shooting-outline" => Some(icons::STAR_SHOOTING_OUTLINE),
        "beaker-remove" => Some(icons::BEAKER_REMOVE),
        "paw-off-outline" => Some(icons::PAW_OFF_OUTLINE),
        "high-definition-box" => Some(icons::HIGH_DEFINITION_BOX),
        "account-reactivate-outline" => Some(icons::ACCOUNT_REACTIVATE_OUTLINE),
        "cube-off" => Some(icons::CUBE_OFF),
        "hub" => Some(icons::HUB),
        "card-off" => Some(icons::CARD_OFF),
        "checkerboard-plus" => Some(icons::CHECKERBOARD_PLUS),
        "source-branch" => Some(icons::SOURCE_BRANCH),
        "view-week" => Some(icons::VIEW_WEEK),
        "invoice-text" => Some(icons::INVOICE_TEXT),
        "feature-search" => Some(icons::FEATURE_SEARCH),
        "delete-outline" => Some(icons::DELETE_OUTLINE),
        "sigma" => Some(icons::SIGMA),
        "pill-off" => Some(icons::PILL_OFF),
        "ceiling-fan-light" => Some(icons::CEILING_FAN_LIGHT),
        "emoticon-lol" => Some(icons::EMOTICON_LOL),
        "book-search-outline" => Some(icons::BOOK_SEARCH_OUTLINE),
        "road" => Some(icons::ROAD),
        "set-none" => Some(icons::SET_NONE),
        "checkbox-blank-circle-outline" => Some(icons::CHECKBOX_BLANK_CIRCLE_OUTLINE),
        "turnstile" => Some(icons::TURNSTILE),
        #[allow(deprecated)]
        "google-my-business" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-my-business' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_MY_BUSINESS)
        }
        "power-plug-outline" => Some(icons::POWER_PLUG_OUTLINE),
        "format-header-pound" => Some(icons::FORMAT_HEADER_POUND),
        "roman-numeral-2" => Some(icons::ROMAN_NUMERAL_2),
        "eyedropper" => Some(icons::EYEDROPPER),
        "clock-check-outline" => Some(icons::CLOCK_CHECK_OUTLINE),
        "relation-only-one-to-one-or-many" => Some(icons::RELATION_ONLY_ONE_TO_ONE_OR_MANY),
        "image-filter-hdr-outline" => Some(icons::IMAGE_FILTER_HDR_OUTLINE),
        "calendar-start" => Some(icons::CALENDAR_START),
        "beach" => Some(icons::BEACH),
        "flag" => Some(icons::FLAG),
        "cog-counterclockwise" => Some(icons::COG_COUNTERCLOCKWISE),
        "network-strength-2-alert" => Some(icons::NETWORK_STRENGTH_2_ALERT),
        "calendar-today" => Some(icons::CALENDAR_TODAY),
        "necklace" => Some(icons::NECKLACE),
        "watering-can-outline" => Some(icons::WATERING_CAN_OUTLINE),
        "contacts-outline" => Some(icons::CONTACTS_OUTLINE),
        "panorama-vertical-outline" => Some(icons::PANORAMA_VERTICAL_OUTLINE),
        "diving-scuba-tank-multiple" => Some(icons::DIVING_SCUBA_TANK_MULTIPLE),
        "car-brake-fluid-level" => Some(icons::CAR_BRAKE_FLUID_LEVEL),
        "bell-badge" => Some(icons::BELL_BADGE),
        "nature-people-outline" => Some(icons::NATURE_PEOPLE_OUTLINE),
        "head-sync" => Some(icons::HEAD_SYNC),
        "file-plus" => Some(icons::FILE_PLUS),
        "diameter-variant" => Some(icons::DIAMETER_VARIANT),
        "clock-time-ten" => Some(icons::CLOCK_TIME_TEN),
        "folder-edit-outline" => Some(icons::FOLDER_EDIT_OUTLINE),
        "podium-bronze" => Some(icons::PODIUM_BRONZE),
        "toy-brick-plus" => Some(icons::TOY_BRICK_PLUS),
        "arrow-down-right" => Some(icons::ARROW_DOWN_RIGHT),
        "table-edit" => Some(icons::TABLE_EDIT),
        "keyboard-backspace" => Some(icons::KEYBOARD_BACKSPACE),
        "pulse" => Some(icons::PULSE),
        "axis-y-arrow" => Some(icons::AXIS_Y_ARROW),
        "table-check" => Some(icons::TABLE_CHECK),
        "snowflake-melt" => Some(icons::SNOWFLAKE_MELT),
        "headphones-settings" => Some(icons::HEADPHONES_SETTINGS),
        "tablet-dashboard" => Some(icons::TABLET_DASHBOARD),
        "cards-variant" => Some(icons::CARDS_VARIANT),
        "tshirt-crew-outline" => Some(icons::TSHIRT_CREW_OUTLINE),
        "video-input-hdmi" => Some(icons::VIDEO_INPUT_HDMI),
        "brightness-5" => Some(icons::BRIGHTNESS_5),
        "abjad-arabic" => Some(icons::ABJAD_ARABIC),
        "reorder-horizontal" => Some(icons::REORDER_HORIZONTAL),
        "sale-outline" => Some(icons::SALE_OUTLINE),
        "flask-empty-plus" => Some(icons::FLASK_EMPTY_PLUS),
        "comment-edit" => Some(icons::COMMENT_EDIT),
        "view-compact" => Some(icons::VIEW_COMPACT),
        "phone-settings-outline" => Some(icons::PHONE_SETTINGS_OUTLINE),
        "door-closed" => Some(icons::DOOR_CLOSED),
        "seat-recline-extra" => Some(icons::SEAT_RECLINE_EXTRA),
        "roller-shade-closed" => Some(icons::ROLLER_SHADE_CLOSED),
        "file-refresh" => Some(icons::FILE_REFRESH),
        #[allow(deprecated)]
        "microsoft-xbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-xbox' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_XBOX)
        }
        "chili-medium-outline" => Some(icons::CHILI_MEDIUM_OUTLINE),
        "shield-outline" => Some(icons::SHIELD_OUTLINE),
        "clock-remove" => Some(icons::CLOCK_REMOVE),
        "script-text-play" => Some(icons::SCRIPT_TEXT_PLAY),
        "calendar-star-outline" => Some(icons::CALENDAR_STAR_OUTLINE),
        "arrow-u-down-right" => Some(icons::ARROW_U_DOWN_RIGHT),
        "selection-drag" => Some(icons::SELECTION_DRAG),
        "magnify-plus-outline" => Some(icons::MAGNIFY_PLUS_OUTLINE),
        "numeric-1" => Some(icons::NUMERIC_1),
        "cannabis" => Some(icons::CANNABIS),
        "lock-open-variant" => Some(icons::LOCK_OPEN_VARIANT),
        "border-bottom-variant" => Some(icons::BORDER_BOTTOM_VARIANT),
        "car-arrow-left" => Some(icons::CAR_ARROW_LEFT),
        _ => None,
    }
}
