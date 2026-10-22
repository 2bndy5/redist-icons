// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_0(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "mailbox-open" => Some(icons::MAILBOX_OPEN),
        "transfer-right" => Some(icons::TRANSFER_RIGHT),
        "battery-charging-wireless-10" => Some(icons::BATTERY_CHARGING_WIRELESS_10),
        "gate-alert" => Some(icons::GATE_ALERT),
        "panorama-vertical-outline" => Some(icons::PANORAMA_VERTICAL_OUTLINE),
        "label-multiple-outline" => Some(icons::LABEL_MULTIPLE_OUTLINE),
        "lifebuoy" => Some(icons::LIFEBUOY),
        "land-fields" => Some(icons::LAND_FIELDS),
        "tag-hidden" => Some(icons::TAG_HIDDEN),
        "clipboard-multiple" => Some(icons::CLIPBOARD_MULTIPLE),
        "incognito" => Some(icons::INCOGNITO),
        "id-card" => Some(icons::ID_CARD),
        "table-filter" => Some(icons::TABLE_FILTER),
        "invoice" => Some(icons::INVOICE),
        "arrow-down-bold-box-outline" => Some(icons::ARROW_DOWN_BOLD_BOX_OUTLINE),
        "email-fast-outline" => Some(icons::EMAIL_FAST_OUTLINE),
        "peanut-outline" => Some(icons::PEANUT_OUTLINE),
        "tennis-ball" => Some(icons::TENNIS_BALL),
        "resize-bottom-right" => Some(icons::RESIZE_BOTTOM_RIGHT),
        "perspective-more" => Some(icons::PERSPECTIVE_MORE),
        "receipt-text-arrow-right" => Some(icons::RECEIPT_TEXT_ARROW_RIGHT),
        "antenna" => Some(icons::ANTENNA),
        "border-left-variant" => Some(icons::BORDER_LEFT_VARIANT),
        "school-outline" => Some(icons::SCHOOL_OUTLINE),
        "axis-z-rotate-counterclockwise" => Some(icons::AXIS_Z_ROTATE_COUNTERCLOCKWISE),
        "watch-vibrate" => Some(icons::WATCH_VIBRATE),
        #[allow(deprecated)]
        "language-java" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-java' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_JAVA)
        }
        "violin" => Some(icons::VIOLIN),
        "ampersand" => Some(icons::AMPERSAND),
        "account-file-text" => Some(icons::ACCOUNT_FILE_TEXT),
        "information-variant-circle-outline" => Some(icons::INFORMATION_VARIANT_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "youtube" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE)
        }
        "clock-time-five" => Some(icons::CLOCK_TIME_FIVE),
        #[allow(deprecated)]
        "origin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'origin' is deprecated.").print(py);
            }
            Some(icons::ORIGIN)
        }
        "printer-outline" => Some(icons::PRINTER_OUTLINE),
        "cross-bolnisi" => Some(icons::CROSS_BOLNISI),
        "numeric-4-box" => Some(icons::NUMERIC_4_BOX),
        "arrow-collapse" => Some(icons::ARROW_COLLAPSE),
        "printer-3d-nozzle-heat" => Some(icons::PRINTER_3D_NOZZLE_HEAT),
        "timer-play" => Some(icons::TIMER_PLAY),
        "folder-upload" => Some(icons::FOLDER_UPLOAD),
        "clipboard-pulse" => Some(icons::CLIPBOARD_PULSE),
        "email-seal" => Some(icons::EMAIL_SEAL),
        "card-multiple" => Some(icons::CARD_MULTIPLE),
        "currency-rial" => Some(icons::CURRENCY_RIAL),
        #[allow(deprecated)]
        "google-fit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-fit' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_FIT)
        }
        "hand-pointing-down" => Some(icons::HAND_POINTING_DOWN),
        "archive-minus" => Some(icons::ARCHIVE_MINUS),
        "handcuffs" => Some(icons::HANDCUFFS),
        "tunnel" => Some(icons::TUNNEL),
        "cookie-edit" => Some(icons::COOKIE_EDIT),
        "wrench" => Some(icons::WRENCH),
        "heart-cog-outline" => Some(icons::HEART_COG_OUTLINE),
        "gamepad-round-up" => Some(icons::GAMEPAD_ROUND_UP),
        "audio-input-rca" => Some(icons::AUDIO_INPUT_RCA),
        "wifi-off" => Some(icons::WIFI_OFF),
        "check-network" => Some(icons::CHECK_NETWORK),
        "meter-gas-outline" => Some(icons::METER_GAS_OUTLINE),
        "pac-man" => Some(icons::PAC_MAN),
        "alpha-a-box" => Some(icons::ALPHA_A_BOX),
        "phone-return" => Some(icons::PHONE_RETURN),
        "television" => Some(icons::TELEVISION),
        "arrow-top-right-bold-outline" => Some(icons::ARROW_TOP_RIGHT_BOLD_OUTLINE),
        "perspective-less" => Some(icons::PERSPECTIVE_LESS),
        "movie" => Some(icons::MOVIE),
        "border-none-variant" => Some(icons::BORDER_NONE_VARIANT),
        "cards-playing-heart-multiple-outline" => Some(icons::CARDS_PLAYING_HEART_MULTIPLE_OUTLINE),
        "faucet" => Some(icons::FAUCET),
        "flash-off-outline" => Some(icons::FLASH_OFF_OUTLINE),
        "spider-web" => Some(icons::SPIDER_WEB),
        "tooltip-minus-outline" => Some(icons::TOOLTIP_MINUS_OUTLINE),
        "flash-triangle-outline" => Some(icons::FLASH_TRIANGLE_OUTLINE),
        "yin-yang" => Some(icons::YIN_YANG),
        "alert-rhombus-outline" => Some(icons::ALERT_RHOMBUS_OUTLINE),
        "rotate-360" => Some(icons::ROTATE_360),
        "brightness-7" => Some(icons::BRIGHTNESS_7),
        "egg-off" => Some(icons::EGG_OFF),
        "weather-snowy" => Some(icons::WEATHER_SNOWY),
        "numeric-6-box-multiple" => Some(icons::NUMERIC_6_BOX_MULTIPLE),
        "cards-playing-club-outline" => Some(icons::CARDS_PLAYING_CLUB_OUTLINE),
        "numeric-negative-1" => Some(icons::NUMERIC_NEGATIVE_1),
        "bread-slice" => Some(icons::BREAD_SLICE),
        "flag-remove" => Some(icons::FLAG_REMOVE),
        "toy-brick-minus" => Some(icons::TOY_BRICK_MINUS),
        "grill-outline" => Some(icons::GRILL_OUTLINE),
        "package-variant-remove" => Some(icons::PACKAGE_VARIANT_REMOVE),
        "camera-off-outline" => Some(icons::CAMERA_OFF_OUTLINE),
        "folder-cog" => Some(icons::FOLDER_COG),
        "headphones-settings" => Some(icons::HEADPHONES_SETTINGS),
        "unfold-less-vertical" => Some(icons::UNFOLD_LESS_VERTICAL),
        "clock-time-twelve-outline" => Some(icons::CLOCK_TIME_TWELVE_OUTLINE),
        "briefcase-variant-off" => Some(icons::BRIEFCASE_VARIANT_OFF),
        "progress-check" => Some(icons::PROGRESS_CHECK),
        "heart-remove-outline" => Some(icons::HEART_REMOVE_OUTLINE),
        "hamburger-remove" => Some(icons::HAMBURGER_REMOVE),
        "fence-electric" => Some(icons::FENCE_ELECTRIC),
        "map-clock" => Some(icons::MAP_CLOCK),
        "cloud-search-outline" => Some(icons::CLOUD_SEARCH_OUTLINE),
        "file-image-minus-outline" => Some(icons::FILE_IMAGE_MINUS_OUTLINE),
        "arrow-left-box" => Some(icons::ARROW_LEFT_BOX),
        "post-lamp" => Some(icons::POST_LAMP),
        "calendar-end-outline" => Some(icons::CALENDAR_END_OUTLINE),
        "octagon" => Some(icons::OCTAGON),
        "pail-plus" => Some(icons::PAIL_PLUS),
        "puzzle-remove-outline" => Some(icons::PUZZLE_REMOVE_OUTLINE),
        "loading" => Some(icons::LOADING),
        "stadium" => Some(icons::STADIUM),
        "help-network-outline" => Some(icons::HELP_NETWORK_OUTLINE),
        "book-account" => Some(icons::BOOK_ACCOUNT),
        "land-rows-vertical" => Some(icons::LAND_ROWS_VERTICAL),
        "palette-swatch-variant" => Some(icons::PALETTE_SWATCH_VARIANT),
        "certificate-outline" => Some(icons::CERTIFICATE_OUTLINE),
        "keyboard-outline" => Some(icons::KEYBOARD_OUTLINE),
        "cloud-print-outline" => Some(icons::CLOUD_PRINT_OUTLINE),
        "arrow-right-thin" => Some(icons::ARROW_RIGHT_THIN),
        "credit-card-check" => Some(icons::CREDIT_CARD_CHECK),
        "bell-cog-outline" => Some(icons::BELL_COG_OUTLINE),
        "escalator" => Some(icons::ESCALATOR),
        "image-multiple" => Some(icons::IMAGE_MULTIPLE),
        "alpha-f-circle" => Some(icons::ALPHA_F_CIRCLE),
        "gamepad-circle-down" => Some(icons::GAMEPAD_CIRCLE_DOWN),
        "projector-screen" => Some(icons::PROJECTOR_SCREEN),
        "keyboard-f1" => Some(icons::KEYBOARD_F1),
        "map-marker-minus" => Some(icons::MAP_MARKER_MINUS),
        "database-import" => Some(icons::DATABASE_IMPORT),
        "earbuds-off-outline" => Some(icons::EARBUDS_OFF_OUTLINE),
        "nature-outline" => Some(icons::NATURE_OUTLINE),
        "octagon-outline" => Some(icons::OCTAGON_OUTLINE),
        "desk-lamp-off" => Some(icons::DESK_LAMP_OFF),
        "note-text" => Some(icons::NOTE_TEXT),
        "web-check" => Some(icons::WEB_CHECK),
        "briefcase-search" => Some(icons::BRIEFCASE_SEARCH),
        "wallet-giftcard" => Some(icons::WALLET_GIFTCARD),
        "router" => Some(icons::ROUTER),
        "kettle-off" => Some(icons::KETTLE_OFF),
        "camera-rear-variant" => Some(icons::CAMERA_REAR_VARIANT),
        "sale" => Some(icons::SALE),
        "magnify-minus" => Some(icons::MAGNIFY_MINUS),
        "archive-arrow-up" => Some(icons::ARCHIVE_ARROW_UP),
        "order-numeric-descending" => Some(icons::ORDER_NUMERIC_DESCENDING),
        "pan-down" => Some(icons::PAN_DOWN),
        "sim-alert-outline" => Some(icons::SIM_ALERT_OUTLINE),
        "tortoise" => Some(icons::TORTOISE),
        "pencil-off" => Some(icons::PENCIL_OFF),
        "bell-alert-outline" => Some(icons::BELL_ALERT_OUTLINE),
        "creation" => Some(icons::CREATION),
        "import" => Some(icons::IMPORT),
        "handshake-outline" => Some(icons::HANDSHAKE_OUTLINE),
        "flask-minus" => Some(icons::FLASK_MINUS),
        "spirit-level" => Some(icons::SPIRIT_LEVEL),
        "lightbulb-on-30" => Some(icons::LIGHTBULB_ON_30),
        "minus-thick" => Some(icons::MINUS_THICK),
        "file-eye" => Some(icons::FILE_EYE),
        "wall" => Some(icons::WALL),
        "numeric-2-circle-outline" => Some(icons::NUMERIC_2_CIRCLE_OUTLINE),
        "alpha-m" => Some(icons::ALPHA_M),
        "phone-bluetooth" => Some(icons::PHONE_BLUETOOTH),
        "source-fork" => Some(icons::SOURCE_FORK),
        "lightbulb-cfl" => Some(icons::LIGHTBULB_CFL),
        "source-commit-local" => Some(icons::SOURCE_COMMIT_LOCAL),
        "chat-minus-outline" => Some(icons::CHAT_MINUS_OUTLINE),
        "ski-water" => Some(icons::SKI_WATER),
        "filter-check-outline" => Some(icons::FILTER_CHECK_OUTLINE),
        "cast-off" => Some(icons::CAST_OFF),
        "clipboard-alert-outline" => Some(icons::CLIPBOARD_ALERT_OUTLINE),
        "video-account" => Some(icons::VIDEO_ACCOUNT),
        "clipboard-arrow-down" => Some(icons::CLIPBOARD_ARROW_DOWN),
        "alphabetical-variant" => Some(icons::ALPHABETICAL_VARIANT),
        "calculator" => Some(icons::CALCULATOR),
        "open-in-new" => Some(icons::OPEN_IN_NEW),
        "phone-remove-outline" => Some(icons::PHONE_REMOVE_OUTLINE),
        "numeric-10-box" => Some(icons::NUMERIC_10_BOX),
        "notebook-heart" => Some(icons::NOTEBOOK_HEART),
        "alpha-o-circle-outline" => Some(icons::ALPHA_O_CIRCLE_OUTLINE),
        "storefront-plus" => Some(icons::STOREFRONT_PLUS),
        "lock-question" => Some(icons::LOCK_QUESTION),
        "chevron-triple-up" => Some(icons::CHEVRON_TRIPLE_UP),
        "map-marker" => Some(icons::MAP_MARKER),
        "dice-multiple" => Some(icons::DICE_MULTIPLE),
        "link-variant-minus" => Some(icons::LINK_VARIANT_MINUS),
        "et" => Some(icons::ET),
        "thumb-down-outline" => Some(icons::THUMB_DOWN_OUTLINE),
        "fridge-industrial-outline" => Some(icons::FRIDGE_INDUSTRIAL_OUTLINE),
        "printer-pos-check" => Some(icons::PRINTER_POS_CHECK),
        #[allow(deprecated)]
        "digital-ocean" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'digital-ocean' is deprecated.").print(py);
            }
            Some(icons::DIGITAL_OCEAN)
        }
        "arrange-send-backward" => Some(icons::ARRANGE_SEND_BACKWARD),
        "account-remove" => Some(icons::ACCOUNT_REMOVE),
        "timer-off-outline" => Some(icons::TIMER_OFF_OUTLINE),
        "script-outline" => Some(icons::SCRIPT_OUTLINE),
        "chef-hat" => Some(icons::CHEF_HAT),
        "alpha-p-circle-outline" => Some(icons::ALPHA_P_CIRCLE_OUTLINE),
        "laptop" => Some(icons::LAPTOP),
        "camera-retake" => Some(icons::CAMERA_RETAKE),
        "storefront-plus-outline" => Some(icons::STOREFRONT_PLUS_OUTLINE),
        "powershell" => Some(icons::POWERSHELL),
        "cards-diamond-outline" => Some(icons::CARDS_DIAMOND_OUTLINE),
        "credit-card-wireless-outline" => Some(icons::CREDIT_CARD_WIRELESS_OUTLINE),
        "shield-half-full" => Some(icons::SHIELD_HALF_FULL),
        "airplane-minus" => Some(icons::AIRPLANE_MINUS),
        "lungs" => Some(icons::LUNGS),
        _ => None,
    }
}
