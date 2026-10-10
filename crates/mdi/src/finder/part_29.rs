// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_29(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "content-save-off" => Some(icons::CONTENT_SAVE_OFF),
        "octagram-minus" => Some(icons::OCTAGRAM_MINUS),
        "plus-thick" => Some(icons::PLUS_THICK),
        #[allow(deprecated)]
        "z-wave" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'z-wave' is deprecated.").print(py);
            }
            Some(icons::Z_WAVE)
        }
        "electric-switch" => Some(icons::ELECTRIC_SWITCH),
        "video-input-hdmi" => Some(icons::VIDEO_INPUT_HDMI),
        "restore" => Some(icons::RESTORE),
        #[allow(deprecated)]
        "fedora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'fedora' is deprecated.").print(py);
            }
            Some(icons::FEDORA)
        }
        "basket-plus" => Some(icons::BASKET_PLUS),
        "math-norm-box" => Some(icons::MATH_NORM_BOX),
        "offer" => Some(icons::OFFER),
        "pine-tree-box" => Some(icons::PINE_TREE_BOX),
        "view-grid" => Some(icons::VIEW_GRID),
        "magnify-plus-outline" => Some(icons::MAGNIFY_PLUS_OUTLINE),
        "sun-angle" => Some(icons::SUN_ANGLE),
        "arrow-split-vertical" => Some(icons::ARROW_SPLIT_VERTICAL),
        "invoice-text-arrow-right-outline" => Some(icons::INVOICE_TEXT_ARROW_RIGHT_OUTLINE),
        "weather-partly-snowy-rainy" => Some(icons::WEATHER_PARTLY_SNOWY_RAINY),
        "train-car-box" => Some(icons::TRAIN_CAR_BOX),
        "calendar-sync" => Some(icons::CALENDAR_SYNC),
        "code-block-braces" => Some(icons::CODE_BLOCK_BRACES),
        "flower-pollen-outline" => Some(icons::FLOWER_POLLEN_OUTLINE),
        "deathly-hallows" => Some(icons::DEATHLY_HALLOWS),
        "cradle" => Some(icons::CRADLE),
        "database-check" => Some(icons::DATABASE_CHECK),
        "chili-hot" => Some(icons::CHILI_HOT),
        "printer-pos-play-outline" => Some(icons::PRINTER_POS_PLAY_OUTLINE),
        "brush" => Some(icons::BRUSH),
        "application-parentheses" => Some(icons::APPLICATION_PARENTHESES),
        "octagram-minus-outline" => Some(icons::OCTAGRAM_MINUS_OUTLINE),
        "file-document-check-outline" => Some(icons::FILE_DOCUMENT_CHECK_OUTLINE),
        "alpha-p-box-outline" => Some(icons::ALPHA_P_BOX_OUTLINE),
        "skip-forward" => Some(icons::SKIP_FORWARD),
        "email-seal-outline" => Some(icons::EMAIL_SEAL_OUTLINE),
        "file-import" => Some(icons::FILE_IMPORT),
        "security" => Some(icons::SECURITY),
        "account-remove-outline" => Some(icons::ACCOUNT_REMOVE_OUTLINE),
        "arrow-left-right-bold" => Some(icons::ARROW_LEFT_RIGHT_BOLD),
        "monitor-cellphone" => Some(icons::MONITOR_CELLPHONE),
        "image-text" => Some(icons::IMAGE_TEXT),
        "bolt" => Some(icons::BOLT),
        "high-definition-box" => Some(icons::HIGH_DEFINITION_BOX),
        "clock-time-ten" => Some(icons::CLOCK_TIME_TEN),
        "dice-d10-outline" => Some(icons::DICE_D10_OUTLINE),
        "reply-outline" => Some(icons::REPLY_OUTLINE),
        "numeric-9-box-outline" => Some(icons::NUMERIC_9_BOX_OUTLINE),
        "rugby" => Some(icons::RUGBY),
        "star-circle-outline" => Some(icons::STAR_CIRCLE_OUTLINE),
        "axis-arrow-lock" => Some(icons::AXIS_ARROW_LOCK),
        "bookmark-off" => Some(icons::BOOKMARK_OFF),
        "source-branch" => Some(icons::SOURCE_BRANCH),
        "phone-classic-off" => Some(icons::PHONE_CLASSIC_OFF),
        "bank-outline" => Some(icons::BANK_OUTLINE),
        "information-slab-symbol" => Some(icons::INFORMATION_SLAB_SYMBOL),
        #[allow(deprecated)]
        "twitter" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitter' is deprecated.").print(py);
            }
            Some(icons::TWITTER)
        }
        "calendar-lock-open-outline" => Some(icons::CALENDAR_LOCK_OPEN_OUTLINE),
        #[allow(deprecated)]
        "unreal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unreal' is deprecated.").print(py);
            }
            Some(icons::UNREAL)
        }
        "usb-flash-drive" => Some(icons::USB_FLASH_DRIVE),
        "greenhouse" => Some(icons::GREENHOUSE),
        "emoticon-confused" => Some(icons::EMOTICON_CONFUSED),
        "tray-minus" => Some(icons::TRAY_MINUS),
        "bluetooth-transfer" => Some(icons::BLUETOOTH_TRANSFER),
        "grass" => Some(icons::GRASS),
        "skull-crossbones-outline" => Some(icons::SKULL_CROSSBONES_OUTLINE),
        #[allow(deprecated)]
        "wikipedia" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wikipedia' is deprecated.").print(py);
            }
            Some(icons::WIKIPEDIA)
        }
        "toy-brick-remove" => Some(icons::TOY_BRICK_REMOVE),
        "relation-many-to-zero-or-one" => Some(icons::RELATION_MANY_TO_ZERO_OR_ONE),
        "percent" => Some(icons::PERCENT),
        #[allow(deprecated)]
        "gatsby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gatsby' is deprecated.").print(py);
            }
            Some(icons::GATSBY)
        }
        "folder-account" => Some(icons::FOLDER_ACCOUNT),
        "archive-star" => Some(icons::ARCHIVE_STAR),
        "printer-3d-nozzle-off" => Some(icons::PRINTER_3D_NOZZLE_OFF),
        "account-file" => Some(icons::ACCOUNT_FILE),
        "fleur-de-lis" => Some(icons::FLEUR_DE_LIS),
        "printer-pos-edit-outline" => Some(icons::PRINTER_POS_EDIT_OUTLINE),
        "numeric-9" => Some(icons::NUMERIC_9),
        "tumble-dryer-alert" => Some(icons::TUMBLE_DRYER_ALERT),
        "network-strength-off-outline" => Some(icons::NETWORK_STRENGTH_OFF_OUTLINE),
        "numeric-8-box-multiple-outline" => Some(icons::NUMERIC_8_BOX_MULTIPLE_OUTLINE),
        "bowl" => Some(icons::BOWL),
        "chip" => Some(icons::CHIP),
        "receipt-text-outline" => Some(icons::RECEIPT_TEXT_OUTLINE),
        "signal-2g" => Some(icons::SIGNAL_2G),
        "emoticon-sick" => Some(icons::EMOTICON_SICK),
        "space-station" => Some(icons::SPACE_STATION),
        "alert-remove-outline" => Some(icons::ALERT_REMOVE_OUTLINE),
        "firework-off" => Some(icons::FIREWORK_OFF),
        "file-compare" => Some(icons::FILE_COMPARE),
        "nfc-tap" => Some(icons::NFC_TAP),
        "message-text-fast-outline" => Some(icons::MESSAGE_TEXT_FAST_OUTLINE),
        "string-lights-off" => Some(icons::STRING_LIGHTS_OFF),
        "layers-minus" => Some(icons::LAYERS_MINUS),
        "trophy-award" => Some(icons::TROPHY_AWARD),
        #[allow(deprecated)]
        "google-nearby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-nearby' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_NEARBY)
        }
        "arrow-u-up-left-bold" => Some(icons::ARROW_U_UP_LEFT_BOLD),
        "form-textarea" => Some(icons::FORM_TEXTAREA),
        "alpha-d-box-outline" => Some(icons::ALPHA_D_BOX_OUTLINE),
        "popcorn" => Some(icons::POPCORN),
        "file-remove" => Some(icons::FILE_REMOVE),
        "basket-off" => Some(icons::BASKET_OFF),
        "plus-box-multiple-outline" => Some(icons::PLUS_BOX_MULTIPLE_OUTLINE),
        "alpha-l-box" => Some(icons::ALPHA_L_BOX),
        "format-header-6" => Some(icons::FORMAT_HEADER_6),
        "star-settings-outline" => Some(icons::STAR_SETTINGS_OUTLINE),
        "border-top" => Some(icons::BORDER_TOP),
        "book-search-outline" => Some(icons::BOOK_SEARCH_OUTLINE),
        "eject-circle" => Some(icons::EJECT_CIRCLE),
        "access-point-off" => Some(icons::ACCESS_POINT_OFF),
        "calendar-expand-horizontal-outline" => Some(icons::CALENDAR_EXPAND_HORIZONTAL_OUTLINE),
        "car-brake-retarder" => Some(icons::CAR_BRAKE_RETARDER),
        #[allow(deprecated)]
        "gnome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gnome' is deprecated.").print(py);
            }
            Some(icons::GNOME)
        }
        "printer" => Some(icons::PRINTER),
        "play-network-outline" => Some(icons::PLAY_NETWORK_OUTLINE),
        "cards-spade-outline" => Some(icons::CARDS_SPADE_OUTLINE),
        "rivet" => Some(icons::RIVET),
        "account-voice" => Some(icons::ACCOUNT_VOICE),
        "video-stabilization" => Some(icons::VIDEO_STABILIZATION),
        "timeline-minus-outline" => Some(icons::TIMELINE_MINUS_OUTLINE),
        "web-clock" => Some(icons::WEB_CLOCK),
        "flag-variant-plus-outline" => Some(icons::FLAG_VARIANT_PLUS_OUTLINE),
        "image-filter-tilt-shift" => Some(icons::IMAGE_FILTER_TILT_SHIFT),
        "boombox" => Some(icons::BOOMBOX),
        "minus-box-outline" => Some(icons::MINUS_BOX_OUTLINE),
        "human-walker" => Some(icons::HUMAN_WALKER),
        "folder-download" => Some(icons::FOLDER_DOWNLOAD),
        "transit-detour" => Some(icons::TRANSIT_DETOUR),
        "comment-off-outline" => Some(icons::COMMENT_OFF_OUTLINE),
        "reload" => Some(icons::RELOAD),
        "battery-plus-outline" => Some(icons::BATTERY_PLUS_OUTLINE),
        "dice-2-outline" => Some(icons::DICE_2_OUTLINE),
        "relative-scale" => Some(icons::RELATIVE_SCALE),
        "star-four-points-small" => Some(icons::STAR_FOUR_POINTS_SMALL),
        "set-none" => Some(icons::SET_NONE),
        "alpha-x" => Some(icons::ALPHA_X),
        "access-point-plus" => Some(icons::ACCESS_POINT_PLUS),
        "shield-refresh-outline" => Some(icons::SHIELD_REFRESH_OUTLINE),
        "forest" => Some(icons::FOREST),
        "dice-d6" => Some(icons::DICE_D6),
        "radius-outline" => Some(icons::RADIUS_OUTLINE),
        "bank-circle" => Some(icons::BANK_CIRCLE),
        "car" => Some(icons::CAR),
        "lock-open-check-outline" => Some(icons::LOCK_OPEN_CHECK_OUTLINE),
        "printer-3d-nozzle-outline" => Some(icons::PRINTER_3D_NOZZLE_OUTLINE),
        "account-injury-outline" => Some(icons::ACCOUNT_INJURY_OUTLINE),
        "food-off-outline" => Some(icons::FOOD_OFF_OUTLINE),
        "reply-all-outline" => Some(icons::REPLY_ALL_OUTLINE),
        "video-box-off" => Some(icons::VIDEO_BOX_OFF),
        "typewriter" => Some(icons::TYPEWRITER),
        "flip-vertical" => Some(icons::FLIP_VERTICAL),
        "face-man-profile" => Some(icons::FACE_MAN_PROFILE),
        "content-save-edit-outline" => Some(icons::CONTENT_SAVE_EDIT_OUTLINE),
        "play-box-lock-open" => Some(icons::PLAY_BOX_LOCK_OPEN),
        "book-cancel" => Some(icons::BOOK_CANCEL),
        "rename-box-outline" => Some(icons::RENAME_BOX_OUTLINE),
        "credit-card-fast-outline" => Some(icons::CREDIT_CARD_FAST_OUTLINE),
        "network-strength-3-alert" => Some(icons::NETWORK_STRENGTH_3_ALERT),
        "minus" => Some(icons::MINUS),
        "office-building-cog-outline" => Some(icons::OFFICE_BUILDING_COG_OUTLINE),
        "account-network-off-outline" => Some(icons::ACCOUNT_NETWORK_OFF_OUTLINE),
        "bookmark-box-multiple" => Some(icons::BOOKMARK_BOX_MULTIPLE),
        "trophy-broken" => Some(icons::TROPHY_BROKEN),
        "text-box-check-outline" => Some(icons::TEXT_BOX_CHECK_OUTLINE),
        "file-arrow-up-down-outline" => Some(icons::FILE_ARROW_UP_DOWN_OUTLINE),
        #[allow(deprecated)]
        "google-lens" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-lens' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_LENS)
        }
        "invoice-fast" => Some(icons::INVOICE_FAST),
        "ammunition" => Some(icons::AMMUNITION),
        "food-apple" => Some(icons::FOOD_APPLE),
        "heart-off" => Some(icons::HEART_OFF),
        "arrow-up-thin-circle-outline" => Some(icons::ARROW_UP_THIN_CIRCLE_OUTLINE),
        "head-minus" => Some(icons::HEAD_MINUS),
        "book-arrow-left" => Some(icons::BOOK_ARROW_LEFT),
        "highway" => Some(icons::HIGHWAY),
        "stomach" => Some(icons::STOMACH),
        "shield-cross-outline" => Some(icons::SHIELD_CROSS_OUTLINE),
        "cloud-key" => Some(icons::CLOUD_KEY),
        "collage" => Some(icons::COLLAGE),
        "fridge-top" => Some(icons::FRIDGE_TOP),
        "boom-gate" => Some(icons::BOOM_GATE),
        "information" => Some(icons::INFORMATION),
        "attachment-off" => Some(icons::ATTACHMENT_OFF),
        "play" => Some(icons::PLAY),
        "yoga" => Some(icons::YOGA),
        "layers-search-outline" => Some(icons::LAYERS_SEARCH_OUTLINE),
        "delete-circle-outline" => Some(icons::DELETE_CIRCLE_OUTLINE),
        "pail-off" => Some(icons::PAIL_OFF),
        "solar-power-variant-outline" => Some(icons::SOLAR_POWER_VARIANT_OUTLINE),
        "truck-remove" => Some(icons::TRUCK_REMOVE),
        "folder-arrow-left-right" => Some(icons::FOLDER_ARROW_LEFT_RIGHT),
        #[allow(deprecated)]
        "slack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'slack' is deprecated.").print(py);
            }
            Some(icons::SLACK)
        }
        "battery-alert-variant-outline" => Some(icons::BATTERY_ALERT_VARIANT_OUTLINE),
        "halloween" => Some(icons::HALLOWEEN),
        "image-filter-drama" => Some(icons::IMAGE_FILTER_DRAMA),
        "relation-only-one-to-one" => Some(icons::RELATION_ONLY_ONE_TO_ONE),
        "numeric-0-box-outline" => Some(icons::NUMERIC_0_BOX_OUTLINE),
        "cog-play-outline" => Some(icons::COG_PLAY_OUTLINE),
        "drag-vertical" => Some(icons::DRAG_VERTICAL),
        "invoice-clock-outline" => Some(icons::INVOICE_CLOCK_OUTLINE),
        "file-refresh" => Some(icons::FILE_REFRESH),
        "window-open" => Some(icons::WINDOW_OPEN),
        "eye-lock-open-outline" => Some(icons::EYE_LOCK_OPEN_OUTLINE),
        _ => None,
    }
}
