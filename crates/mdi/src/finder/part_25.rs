// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_25(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        #[allow(deprecated)]
        "drupal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'drupal' is deprecated.").print(py);
            }
            Some(icons::DRUPAL)
        }
        "currency-inr" => Some(icons::CURRENCY_INR),
        "shoe-print" => Some(icons::SHOE_PRINT),
        "delete-variant" => Some(icons::DELETE_VARIANT),
        "book-plus" => Some(icons::BOOK_PLUS),
        "beaker-minus" => Some(icons::BEAKER_MINUS),
        "motorbike" => Some(icons::MOTORBIKE),
        "bed-single-outline" => Some(icons::BED_SINGLE_OUTLINE),
        "car-seat" => Some(icons::CAR_SEAT),
        "cookie-lock" => Some(icons::COOKIE_LOCK),
        "email-off" => Some(icons::EMAIL_OFF),
        "account-tag-outline" => Some(icons::ACCOUNT_TAG_OUTLINE),
        "bluetooth-audio" => Some(icons::BLUETOOTH_AUDIO),
        "key" => Some(icons::KEY),
        "account-multiple-outline" => Some(icons::ACCOUNT_MULTIPLE_OUTLINE),
        "card-bulleted-settings-outline" => Some(icons::CARD_BULLETED_SETTINGS_OUTLINE),
        "wall-sconce-flat-variant" => Some(icons::WALL_SCONCE_FLAT_VARIANT),
        "message-bulleted-off" => Some(icons::MESSAGE_BULLETED_OFF),
        "message-cog" => Some(icons::MESSAGE_COG),
        "playlist-minus" => Some(icons::PLAYLIST_MINUS),
        "download-box" => Some(icons::DOWNLOAD_BOX),
        "multicast" => Some(icons::MULTICAST),
        "format-paragraph" => Some(icons::FORMAT_PARAGRAPH),
        "food-variant" => Some(icons::FOOD_VARIANT),
        "puzzle-star" => Some(icons::PUZZLE_STAR),
        "numeric-5-box-outline" => Some(icons::NUMERIC_5_BOX_OUTLINE),
        #[allow(deprecated)]
        "onepassword" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'onepassword' is deprecated.").print(py);
            }
            Some(icons::ONEPASSWORD)
        }
        "hand-back-right" => Some(icons::HAND_BACK_RIGHT),
        "notebook-outline" => Some(icons::NOTEBOOK_OUTLINE),
        "clipboard-multiple" => Some(icons::CLIPBOARD_MULTIPLE),
        "filter-settings-outline" => Some(icons::FILTER_SETTINGS_OUTLINE),
        "dresser-outline" => Some(icons::DRESSER_OUTLINE),
        "hand-back-right-outline" => Some(icons::HAND_BACK_RIGHT_OUTLINE),
        "lightbulb-on-60" => Some(icons::LIGHTBULB_ON_60),
        #[allow(deprecated)]
        "iobroker" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'iobroker' is deprecated.").print(py);
            }
            Some(icons::IOBROKER)
        }
        "credit-card-wireless-off" => Some(icons::CREDIT_CARD_WIRELESS_OFF),
        "account-network-off" => Some(icons::ACCOUNT_NETWORK_OFF),
        "land-plots-marker" => Some(icons::LAND_PLOTS_MARKER),
        "progress-clock" => Some(icons::PROGRESS_CLOCK),
        "monitor-off" => Some(icons::MONITOR_OFF),
        "storage-tank-outline" => Some(icons::STORAGE_TANK_OUTLINE),
        "backspace" => Some(icons::BACKSPACE),
        "pig-variant" => Some(icons::PIG_VARIANT),
        "raspberry-pi" => Some(icons::RASPBERRY_PI),
        "chess-pawn" => Some(icons::CHESS_PAWN),
        "ideogram-cjk" => Some(icons::IDEOGRAM_CJK),
        "flash-triangle" => Some(icons::FLASH_TRIANGLE),
        "thermometer-lines" => Some(icons::THERMOMETER_LINES),
        "flask-plus-outline" => Some(icons::FLASK_PLUS_OUTLINE),
        "book-arrow-up-outline" => Some(icons::BOOK_ARROW_UP_OUTLINE),
        "gate" => Some(icons::GATE),
        "sort-alphabetical-descending-variant" => Some(icons::SORT_ALPHABETICAL_DESCENDING_VARIANT),
        "alpha-q-box" => Some(icons::ALPHA_Q_BOX),
        "soundbar" => Some(icons::SOUNDBAR),
        "package-check" => Some(icons::PACKAGE_CHECK),
        "tea" => Some(icons::TEA),
        "heart-broken-outline" => Some(icons::HEART_BROKEN_OUTLINE),
        "printer-3d-off" => Some(icons::PRINTER_3D_OFF),
        "hand-pointing-left" => Some(icons::HAND_POINTING_LEFT),
        "arrow-u-left-top-bold" => Some(icons::ARROW_U_LEFT_TOP_BOLD),
        "printer-pos-network-outline" => Some(icons::PRINTER_POS_NETWORK_OUTLINE),
        "briefcase-plus-outline" => Some(icons::BRIEFCASE_PLUS_OUTLINE),
        "head" => Some(icons::HEAD),
        "cash-register" => Some(icons::CASH_REGISTER),
        "content-save-minus-outline" => Some(icons::CONTENT_SAVE_MINUS_OUTLINE),
        "fan-speed-3" => Some(icons::FAN_SPEED_3),
        "calendar-month-outline" => Some(icons::CALENDAR_MONTH_OUTLINE),
        "clipboard-text-multiple" => Some(icons::CLIPBOARD_TEXT_MULTIPLE),
        "spa-outline" => Some(icons::SPA_OUTLINE),
        "view-split-vertical" => Some(icons::VIEW_SPLIT_VERTICAL),
        "phone-rotate-portrait" => Some(icons::PHONE_ROTATE_PORTRAIT),
        "image-filter-center-focus-weak" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_WEAK),
        "navigation-variant-outline" => Some(icons::NAVIGATION_VARIANT_OUTLINE),
        #[allow(deprecated)]
        "nintendo-switch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-switch' is deprecated.")
                    .print(py);
            }
            Some(icons::NINTENDO_SWITCH)
        }
        "timer-settings-outline" => Some(icons::TIMER_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "slack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'slack' is deprecated.").print(py);
            }
            Some(icons::SLACK)
        }
        "calendar-blank-outline" => Some(icons::CALENDAR_BLANK_OUTLINE),
        "magazine-pistol" => Some(icons::MAGAZINE_PISTOL),
        "table-arrow-up" => Some(icons::TABLE_ARROW_UP),
        "chart-areaspline-variant" => Some(icons::CHART_AREASPLINE_VARIANT),
        "storefront-minus" => Some(icons::STOREFRONT_MINUS),
        "battery-charging-wireless-40" => Some(icons::BATTERY_CHARGING_WIRELESS_40),
        "link-plus" => Some(icons::LINK_PLUS),
        "watch-export-variant" => Some(icons::WATCH_EXPORT_VARIANT),
        "email-arrow-left-outline" => Some(icons::EMAIL_ARROW_LEFT_OUTLINE),
        "email-sync" => Some(icons::EMAIL_SYNC),
        "text-box-edit" => Some(icons::TEXT_BOX_EDIT),
        "meter-electric-outline" => Some(icons::METER_ELECTRIC_OUTLINE),
        "police-badge-outline" => Some(icons::POLICE_BADGE_OUTLINE),
        "wrench-outline" => Some(icons::WRENCH_OUTLINE),
        "lightbulb-night-outline" => Some(icons::LIGHTBULB_NIGHT_OUTLINE),
        "warehouse" => Some(icons::WAREHOUSE),
        "garage" => Some(icons::GARAGE),
        "sticker-alert-outline" => Some(icons::STICKER_ALERT_OUTLINE),
        "wifi-star" => Some(icons::WIFI_STAR),
        "space-invaders" => Some(icons::SPACE_INVADERS),
        "file-cancel-outline" => Some(icons::FILE_CANCEL_OUTLINE),
        "lock-open-minus-outline" => Some(icons::LOCK_OPEN_MINUS_OUTLINE),
        "select-off" => Some(icons::SELECT_OFF),
        "hexagon-slice-5" => Some(icons::HEXAGON_SLICE_5),
        "phone-remove-outline" => Some(icons::PHONE_REMOVE_OUTLINE),
        "fruit-pear" => Some(icons::FRUIT_PEAR),
        "shield-sync" => Some(icons::SHIELD_SYNC),
        "sun-angle" => Some(icons::SUN_ANGLE),
        "account-multiple-remove-outline" => Some(icons::ACCOUNT_MULTIPLE_REMOVE_OUTLINE),
        "image-filter-tilt-shift" => Some(icons::IMAGE_FILTER_TILT_SHIFT),
        "shopping-music" => Some(icons::SHOPPING_MUSIC),
        "floor-lamp-torchiere-variant" => Some(icons::FLOOR_LAMP_TORCHIERE_VARIANT),
        "bank-transfer-out" => Some(icons::BANK_TRANSFER_OUT),
        "train-car-container" => Some(icons::TRAIN_CAR_CONTAINER),
        "folder-pound" => Some(icons::FOLDER_POUND),
        "arrow-collapse-horizontal" => Some(icons::ARROW_COLLAPSE_HORIZONTAL),
        "menu-right-outline" => Some(icons::MENU_RIGHT_OUTLINE),
        "grill-outline" => Some(icons::GRILL_OUTLINE),
        "code-braces" => Some(icons::CODE_BRACES),
        "table-network" => Some(icons::TABLE_NETWORK),
        #[allow(deprecated)]
        "twitch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitch' is deprecated.").print(py);
            }
            Some(icons::TWITCH)
        }
        "music-note-whole-dotted" => Some(icons::MUSIC_NOTE_WHOLE_DOTTED),
        "format-underline-wavy" => Some(icons::FORMAT_UNDERLINE_WAVY),
        "download-circle" => Some(icons::DOWNLOAD_CIRCLE),
        "rotate-3d-variant" => Some(icons::ROTATE_3D_VARIANT),
        "invoice-arrow-right-outline" => Some(icons::INVOICE_ARROW_RIGHT_OUTLINE),
        "lightbulb-group-off-outline" => Some(icons::LIGHTBULB_GROUP_OFF_OUTLINE),
        "abugida-thai" => Some(icons::ABUGIDA_THAI),
        "cosine-wave" => Some(icons::COSINE_WAVE),
        "view-stream-outline" => Some(icons::VIEW_STREAM_OUTLINE),
        "toy-brick-minus" => Some(icons::TOY_BRICK_MINUS),
        "bag-personal-tag-outline" => Some(icons::BAG_PERSONAL_TAG_OUTLINE),
        "alpha-b" => Some(icons::ALPHA_B),
        "puzzle-heart" => Some(icons::PUZZLE_HEART),
        "flag-minus" => Some(icons::FLAG_MINUS),
        #[allow(deprecated)]
        "twitter" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitter' is deprecated.").print(py);
            }
            Some(icons::TWITTER)
        }
        "battery-50" => Some(icons::BATTERY_50),
        "check-underline-circle-outline" => Some(icons::CHECK_UNDERLINE_CIRCLE_OUTLINE),
        "wall-sconce-round-variant-outline" => Some(icons::WALL_SCONCE_ROUND_VARIANT_OUTLINE),
        "arrow-u-right-top-bold" => Some(icons::ARROW_U_RIGHT_TOP_BOLD),
        "compass-off-outline" => Some(icons::COMPASS_OFF_OUTLINE),
        #[allow(deprecated)]
        "lastpass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'lastpass' is deprecated.").print(py);
            }
            Some(icons::LASTPASS)
        }
        "face-woman" => Some(icons::FACE_WOMAN),
        "airplane-remove" => Some(icons::AIRPLANE_REMOVE),
        "email-check" => Some(icons::EMAIL_CHECK),
        "lightbulb-night" => Some(icons::LIGHTBULB_NIGHT),
        "code-json" => Some(icons::CODE_JSON),
        #[allow(deprecated)]
        "codepen" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'codepen' is deprecated.").print(py);
            }
            Some(icons::CODEPEN)
        }
        "text-box-check" => Some(icons::TEXT_BOX_CHECK),
        "file-tree-outline" => Some(icons::FILE_TREE_OUTLINE),
        "food" => Some(icons::FOOD),
        "form-textbox-lock" => Some(icons::FORM_TEXTBOX_LOCK),
        "receipt-text-remove-outline" => Some(icons::RECEIPT_TEXT_REMOVE_OUTLINE),
        "moped-electric" => Some(icons::MOPED_ELECTRIC),
        "wifi-strength-2" => Some(icons::WIFI_STRENGTH_2),
        "lightbulb-on-90" => Some(icons::LIGHTBULB_ON_90),
        "wall" => Some(icons::WALL),
        "checkbox-marked-circle-auto-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_AUTO_OUTLINE),
        "qrcode-edit" => Some(icons::QRCODE_EDIT),
        "check-underline-circle" => Some(icons::CHECK_UNDERLINE_CIRCLE),
        "lock-question" => Some(icons::LOCK_QUESTION),
        "format-letter-spacing-variant" => Some(icons::FORMAT_LETTER_SPACING_VARIANT),
        "alpha-h-box" => Some(icons::ALPHA_H_BOX),
        #[allow(deprecated)]
        "google-spreadsheet" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-spreadsheet' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_SPREADSHEET)
        }
        "chevron-right-box" => Some(icons::CHEVRON_RIGHT_BOX),
        "chat-remove-outline" => Some(icons::CHAT_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "microsoft-azure-devops" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure-devops' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE_DEVOPS)
        }
        #[allow(deprecated)]
        "nintendo-wii" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wii' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WII)
        }
        "grid-off" => Some(icons::GRID_OFF),
        "rewind-10" => Some(icons::REWIND_10),
        #[allow(deprecated)]
        "angular" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'angular' is deprecated.").print(py);
            }
            Some(icons::ANGULAR)
        }
        "podcast" => Some(icons::PODCAST),
        "home-switch-outline" => Some(icons::HOME_SWITCH_OUTLINE),
        "pin-off-outline" => Some(icons::PIN_OFF_OUTLINE),
        "pail-remove" => Some(icons::PAIL_REMOVE),
        "ice-cream" => Some(icons::ICE_CREAM),
        "human" => Some(icons::HUMAN),
        "television-box" => Some(icons::TELEVISION_BOX),
        "mustache" => Some(icons::MUSTACHE),
        "feather" => Some(icons::FEATHER),
        "magnify-remove-cursor" => Some(icons::MAGNIFY_REMOVE_CURSOR),
        "panda" => Some(icons::PANDA),
        #[allow(deprecated)]
        "svg" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'svg' is deprecated.").print(py);
            }
            Some(icons::SVG)
        }
        "email-heart-outline" => Some(icons::EMAIL_HEART_OUTLINE),
        #[allow(deprecated)]
        "creative-commons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'creative-commons' is deprecated.")
                    .print(py);
            }
            Some(icons::CREATIVE_COMMONS)
        }
        "video-switch-outline" => Some(icons::VIDEO_SWITCH_OUTLINE),
        "view-parallel" => Some(icons::VIEW_PARALLEL),
        "flag-variant-off-outline" => Some(icons::FLAG_VARIANT_OFF_OUTLINE),
        "cards-playing-outline" => Some(icons::CARDS_PLAYING_OUTLINE),
        "door-sliding" => Some(icons::DOOR_SLIDING),
        "archive-remove-outline" => Some(icons::ARCHIVE_REMOVE_OUTLINE),
        "account-heart-outline" => Some(icons::ACCOUNT_HEART_OUTLINE),
        "usb-c-port" => Some(icons::USB_C_PORT),
        "nuke" => Some(icons::NUKE),
        "account-supervisor-outline" => Some(icons::ACCOUNT_SUPERVISOR_OUTLINE),
        "phone-off" => Some(icons::PHONE_OFF),
        "shield-plus" => Some(icons::SHIELD_PLUS),
        "printer-wireless" => Some(icons::PRINTER_WIRELESS),
        "picture-in-picture-bottom-right" => Some(icons::PICTURE_IN_PICTURE_BOTTOM_RIGHT),
        "clock-check-outline" => Some(icons::CLOCK_CHECK_OUTLINE),
        "bench" => Some(icons::BENCH),
        "emoticon-poop-outline" => Some(icons::EMOTICON_POOP_OUTLINE),
        "dice-2-outline" => Some(icons::DICE_2_OUTLINE),
        "picture-in-picture-bottom-right-outline" => {
            Some(icons::PICTURE_IN_PICTURE_BOTTOM_RIGHT_OUTLINE)
        }
        _ => None,
    }
}
