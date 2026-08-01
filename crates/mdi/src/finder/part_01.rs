// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_1(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "storefront-edit" => Some(icons::STOREFRONT_EDIT),
        "folder-lock" => Some(icons::FOLDER_LOCK),
        "details" => Some(icons::DETAILS),
        #[allow(deprecated)]
        "artstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'artstation' is deprecated.").print(py);
            }
            Some(icons::ARTSTATION)
        }
        "bag-personal-off-outline" => Some(icons::BAG_PERSONAL_OFF_OUTLINE),
        "car-info" => Some(icons::CAR_INFO),
        "pail-plus" => Some(icons::PAIL_PLUS),
        "water-outline" => Some(icons::WATER_OUTLINE),
        #[allow(deprecated)]
        "deviantart" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'deviantart' is deprecated.").print(py);
            }
            Some(icons::DEVIANTART)
        }
        "cellphone-dock" => Some(icons::CELLPHONE_DOCK),
        "close-octagon-outline" => Some(icons::CLOSE_OCTAGON_OUTLINE),
        "account-outline" => Some(icons::ACCOUNT_OUTLINE),
        "table" => Some(icons::TABLE),
        "lightbulb-on-80" => Some(icons::LIGHTBULB_ON_80),
        "satellite-variant" => Some(icons::SATELLITE_VARIANT),
        "puzzle-star" => Some(icons::PUZZLE_STAR),
        "one-up" => Some(icons::ONE_UP),
        "send-lock-outline" => Some(icons::SEND_LOCK_OUTLINE),
        "forest" => Some(icons::FOREST),
        "seat-legroom-reduced" => Some(icons::SEAT_LEGROOM_REDUCED),
        "star-remove" => Some(icons::STAR_REMOVE),
        "bike-fast" => Some(icons::BIKE_FAST),
        "printer-wireless" => Some(icons::PRINTER_WIRELESS),
        "wiper-wash-alert" => Some(icons::WIPER_WASH_ALERT),
        "bug-check-outline" => Some(icons::BUG_CHECK_OUTLINE),
        "file-edit" => Some(icons::FILE_EDIT),
        "bed-single-outline" => Some(icons::BED_SINGLE_OUTLINE),
        "transfer-right" => Some(icons::TRANSFER_RIGHT),
        "sawtooth-wave" => Some(icons::SAWTOOTH_WAVE),
        "laptop" => Some(icons::LAPTOP),
        "move-resize-variant" => Some(icons::MOVE_RESIZE_VARIANT),
        "wallet-outline" => Some(icons::WALLET_OUTLINE),
        "receipt-clock-outline" => Some(icons::RECEIPT_CLOCK_OUTLINE),
        "axe-battle" => Some(icons::AXE_BATTLE),
        "television-off" => Some(icons::TELEVISION_OFF),
        "plus-outline" => Some(icons::PLUS_OUTLINE),
        "rewind-outline" => Some(icons::REWIND_OUTLINE),
        "text-search-variant" => Some(icons::TEXT_SEARCH_VARIANT),
        "alpha-v-circle" => Some(icons::ALPHA_V_CIRCLE),
        "panorama-wide-angle-outline" => Some(icons::PANORAMA_WIDE_ANGLE_OUTLINE),
        "application-array-outline" => Some(icons::APPLICATION_ARRAY_OUTLINE),
        "information-off-outline" => Some(icons::INFORMATION_OFF_OUTLINE),
        "cards-playing-spade-multiple-outline" => Some(icons::CARDS_PLAYING_SPADE_MULTIPLE_OUTLINE),
        "beaker-minus" => Some(icons::BEAKER_MINUS),
        "cursor-default-click-outline" => Some(icons::CURSOR_DEFAULT_CLICK_OUTLINE),
        "coffee-outline" => Some(icons::COFFEE_OUTLINE),
        "axis-arrow-lock" => Some(icons::AXIS_ARROW_LOCK),
        "receipt-text-minus-outline" => Some(icons::RECEIPT_TEXT_MINUS_OUTLINE),
        "dns" => Some(icons::DNS),
        "blur-linear" => Some(icons::BLUR_LINEAR),
        "camera-enhance-outline" => Some(icons::CAMERA_ENHANCE_OUTLINE),
        "dishwasher-off" => Some(icons::DISHWASHER_OFF),
        "message-off" => Some(icons::MESSAGE_OFF),
        "card-account-details" => Some(icons::CARD_ACCOUNT_DETAILS),
        "database-lock" => Some(icons::DATABASE_LOCK),
        "egg-fried" => Some(icons::EGG_FRIED),
        "refresh-auto" => Some(icons::REFRESH_AUTO),
        "artboard" => Some(icons::ARTBOARD),
        "thumbs-up-down" => Some(icons::THUMBS_UP_DOWN),
        "database-settings-outline" => Some(icons::DATABASE_SETTINGS_OUTLINE),
        "pig" => Some(icons::PIG),
        "drag-vertical-variant" => Some(icons::DRAG_VERTICAL_VARIANT),
        "bunk-bed-outline" => Some(icons::BUNK_BED_OUTLINE),
        "close" => Some(icons::CLOSE),
        "file-image-plus" => Some(icons::FILE_IMAGE_PLUS),
        "email-variant" => Some(icons::EMAIL_VARIANT),
        "fridge-bottom" => Some(icons::FRIDGE_BOTTOM),
        "garage-variant-lock" => Some(icons::GARAGE_VARIANT_LOCK),
        "toggle-switch-outline" => Some(icons::TOGGLE_SWITCH_OUTLINE),
        "storefront-outline" => Some(icons::STOREFRONT_OUTLINE),
        "credit-card-check-outline" => Some(icons::CREDIT_CARD_CHECK_OUTLINE),
        "all-inclusive" => Some(icons::ALL_INCLUSIVE),
        "fire-extinguisher" => Some(icons::FIRE_EXTINGUISHER),
        "arrow-right-bottom-bold" => Some(icons::ARROW_RIGHT_BOTTOM_BOLD),
        "focus-field-vertical" => Some(icons::FOCUS_FIELD_VERTICAL),
        "eye-off-outline" => Some(icons::EYE_OFF_OUTLINE),
        "vector-square-plus" => Some(icons::VECTOR_SQUARE_PLUS),
        "format-text-wrapping-wrap" => Some(icons::FORMAT_TEXT_WRAPPING_WRAP),
        "skate-off" => Some(icons::SKATE_OFF),
        "lightbulb-cfl" => Some(icons::LIGHTBULB_CFL),
        "currency-fra" => Some(icons::CURRENCY_FRA),
        "comment-off-outline" => Some(icons::COMMENT_OFF_OUTLINE),
        "leaf-circle" => Some(icons::LEAF_CIRCLE),
        "video-3d-off" => Some(icons::VIDEO_3D_OFF),
        #[allow(deprecated)]
        "z-wave" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'z-wave' is deprecated.").print(py);
            }
            Some(icons::Z_WAVE)
        }
        "text-box-search-outline" => Some(icons::TEXT_BOX_SEARCH_OUTLINE),
        "account-hard-hat" => Some(icons::ACCOUNT_HARD_HAT),
        "format-list-bulleted-type" => Some(icons::FORMAT_LIST_BULLETED_TYPE),
        "tag-minus-outline" => Some(icons::TAG_MINUS_OUTLINE),
        "bell-ring" => Some(icons::BELL_RING),
        "image-filter-black-white" => Some(icons::IMAGE_FILTER_BLACK_WHITE),
        "backspace" => Some(icons::BACKSPACE),
        "image-filter-center-focus-weak" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_WEAK),
        "cloud-lock" => Some(icons::CLOUD_LOCK),
        "basketball-hoop" => Some(icons::BASKETBALL_HOOP),
        #[allow(deprecated)]
        "home-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'home-assistant' is deprecated.").print(py);
            }
            Some(icons::HOME_ASSISTANT)
        }
        "link-variant-plus" => Some(icons::LINK_VARIANT_PLUS),
        "wrench-check" => Some(icons::WRENCH_CHECK),
        "gesture-double-tap" => Some(icons::GESTURE_DOUBLE_TAP),
        "music-note-eighth-dotted" => Some(icons::MUSIC_NOTE_EIGHTH_DOTTED),
        "hubspot" => Some(icons::HUBSPOT),
        "file-powerpoint" => Some(icons::FILE_POWERPOINT),
        #[allow(deprecated)]
        "facebook-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-gaming' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_GAMING)
        }
        "network" => Some(icons::NETWORK),
        "message-lock-outline" => Some(icons::MESSAGE_LOCK_OUTLINE),
        "switch" => Some(icons::SWITCH),
        "signal-2g" => Some(icons::SIGNAL_2G),
        "arrow-decision-outline" => Some(icons::ARROW_DECISION_OUTLINE),
        "rugby" => Some(icons::RUGBY),
        "emoticon-remove" => Some(icons::EMOTICON_REMOVE),
        "swap-vertical-circle" => Some(icons::SWAP_VERTICAL_CIRCLE),
        "brightness-2" => Some(icons::BRIGHTNESS_2),
        "phone-incoming" => Some(icons::PHONE_INCOMING),
        "train-bus" => Some(icons::TRAIN_BUS),
        "calendar-lock" => Some(icons::CALENDAR_LOCK),
        "fire" => Some(icons::FIRE),
        "numeric-4-circle" => Some(icons::NUMERIC_4_CIRCLE),
        "cellphone-basic" => Some(icons::CELLPHONE_BASIC),
        "quality-high" => Some(icons::QUALITY_HIGH),
        "link-variant-minus" => Some(icons::LINK_VARIANT_MINUS),
        "patio-heater" => Some(icons::PATIO_HEATER),
        "cube-off" => Some(icons::CUBE_OFF),
        "crystal-ball" => Some(icons::CRYSTAL_BALL),
        "guitar-acoustic" => Some(icons::GUITAR_ACOUSTIC),
        "star-three-points" => Some(icons::STAR_THREE_POINTS),
        "finance" => Some(icons::FINANCE),
        "map" => Some(icons::MAP),
        #[allow(deprecated)]
        "semantic-web" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'semantic-web' is deprecated.").print(py);
            }
            Some(icons::SEMANTIC_WEB)
        }
        "code-block-braces" => Some(icons::CODE_BLOCK_BRACES),
        "robot-love" => Some(icons::ROBOT_LOVE),
        "folder-off" => Some(icons::FOLDER_OFF),
        "podium-gold" => Some(icons::PODIUM_GOLD),
        "wheelchair-accessibility" => Some(icons::WHEELCHAIR_ACCESSIBILITY),
        "treasure-chest" => Some(icons::TREASURE_CHEST),
        "table-chair" => Some(icons::TABLE_CHAIR),
        "format-color-highlight" => Some(icons::FORMAT_COLOR_HIGHLIGHT),
        "sign-real-estate" => Some(icons::SIGN_REAL_ESTATE),
        "arrow-u-left-top-bold" => Some(icons::ARROW_U_LEFT_TOP_BOLD),
        "play-box-multiple-outline" => Some(icons::PLAY_BOX_MULTIPLE_OUTLINE),
        "wall-sconce-flat" => Some(icons::WALL_SCONCE_FLAT),
        "curtains-closed" => Some(icons::CURTAINS_CLOSED),
        "dip-switch" => Some(icons::DIP_SWITCH),
        "plane-train" => Some(icons::PLANE_TRAIN),
        "battery-60-bluetooth" => Some(icons::BATTERY_60_BLUETOOTH),
        "attachment-lock" => Some(icons::ATTACHMENT_LOCK),
        "note-alert-outline" => Some(icons::NOTE_ALERT_OUTLINE),
        "train-car-hopper-covered" => Some(icons::TRAIN_CAR_HOPPER_COVERED),
        "flask-off-outline" => Some(icons::FLASK_OFF_OUTLINE),
        "glasses" => Some(icons::GLASSES),
        "step-forward-2" => Some(icons::STEP_FORWARD_2),
        "paper-cut-vertical" => Some(icons::PAPER_CUT_VERTICAL),
        "book" => Some(icons::BOOK),
        "bell" => Some(icons::BELL),
        "step-forward" => Some(icons::STEP_FORWARD),
        "video-switch-outline" => Some(icons::VIDEO_SWITCH_OUTLINE),
        "arm-flex" => Some(icons::ARM_FLEX),
        "wrench-check-outline" => Some(icons::WRENCH_CHECK_OUTLINE),
        "parking" => Some(icons::PARKING),
        "bag-carry-on-check" => Some(icons::BAG_CARRY_ON_CHECK),
        "book-minus-multiple" => Some(icons::BOOK_MINUS_MULTIPLE),
        "clipboard-text-multiple-outline" => Some(icons::CLIPBOARD_TEXT_MULTIPLE_OUTLINE),
        "octagram-plus" => Some(icons::OCTAGRAM_PLUS),
        "package-variant" => Some(icons::PACKAGE_VARIANT),
        "spray-bottle" => Some(icons::SPRAY_BOTTLE),
        "file-alert-outline" => Some(icons::FILE_ALERT_OUTLINE),
        "fountain-pen-tip" => Some(icons::FOUNTAIN_PEN_TIP),
        "mushroom-off" => Some(icons::MUSHROOM_OFF),
        "head-snowflake" => Some(icons::HEAD_SNOWFLAKE),
        "image-check-outline" => Some(icons::IMAGE_CHECK_OUTLINE),
        "format-text-rotation-none" => Some(icons::FORMAT_TEXT_ROTATION_NONE),
        "message-text-lock" => Some(icons::MESSAGE_TEXT_LOCK),
        "hamburger-off" => Some(icons::HAMBURGER_OFF),
        "cards-playing-heart-multiple-outline" => Some(icons::CARDS_PLAYING_HEART_MULTIPLE_OUTLINE),
        "file-excel-box" => Some(icons::FILE_EXCEL_BOX),
        #[allow(deprecated)]
        "sina-weibo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sina-weibo' is deprecated.").print(py);
            }
            Some(icons::SINA_WEIBO)
        }
        "antenna" => Some(icons::ANTENNA),
        "script-text" => Some(icons::SCRIPT_TEXT),
        "stairs-box" => Some(icons::STAIRS_BOX),
        "piston" => Some(icons::PISTON),
        "air-conditioner" => Some(icons::AIR_CONDITIONER),
        "alpha-u-box-outline" => Some(icons::ALPHA_U_BOX_OUTLINE),
        "hexagon-outline" => Some(icons::HEXAGON_OUTLINE),
        "robot-angry" => Some(icons::ROBOT_ANGRY),
        "file-cloud-outline" => Some(icons::FILE_CLOUD_OUTLINE),
        "shield-edit-outline" => Some(icons::SHIELD_EDIT_OUTLINE),
        "forest-outline" => Some(icons::FOREST_OUTLINE),
        "math-norm" => Some(icons::MATH_NORM),
        "lectern" => Some(icons::LECTERN),
        "arrow-down-circle-outline" => Some(icons::ARROW_DOWN_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "opera" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'opera' is deprecated.").print(py);
            }
            Some(icons::OPERA)
        }
        "toaster-off" => Some(icons::TOASTER_OFF),
        "cellphone-marker" => Some(icons::CELLPHONE_MARKER),
        "cookie-edit" => Some(icons::COOKIE_EDIT),
        "distribute-vertical-bottom" => Some(icons::DISTRIBUTE_VERTICAL_BOTTOM),
        "phone-log-outline" => Some(icons::PHONE_LOG_OUTLINE),
        "calendar-import" => Some(icons::CALENDAR_IMPORT),
        "server-outline" => Some(icons::SERVER_OUTLINE),
        "account-multiple-minus" => Some(icons::ACCOUNT_MULTIPLE_MINUS),
        "car-connected" => Some(icons::CAR_CONNECTED),
        "currency-ils" => Some(icons::CURRENCY_ILS),
        _ => None,
    }
}
