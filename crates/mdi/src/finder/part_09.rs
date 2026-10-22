// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_9(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "table-settings" => Some(icons::TABLE_SETTINGS),
        "barcode" => Some(icons::BARCODE),
        "bus-articulated-front" => Some(icons::BUS_ARTICULATED_FRONT),
        "calendar-export-outline" => Some(icons::CALENDAR_EXPORT_OUTLINE),
        "keyboard-return" => Some(icons::KEYBOARD_RETURN),
        "hand-back-left-off-outline" => Some(icons::HAND_BACK_LEFT_OFF_OUTLINE),
        "head-heart-outline" => Some(icons::HEAD_HEART_OUTLINE),
        "dns" => Some(icons::DNS),
        "lightbulb-on-outline" => Some(icons::LIGHTBULB_ON_OUTLINE),
        "domain-plus" => Some(icons::DOMAIN_PLUS),
        "application-settings-outline" => Some(icons::APPLICATION_SETTINGS_OUTLINE),
        "format-letter-case-lower" => Some(icons::FORMAT_LETTER_CASE_LOWER),
        "gas-station" => Some(icons::GAS_STATION),
        "account-circle-outline" => Some(icons::ACCOUNT_CIRCLE_OUTLINE),
        "contactless-payment-circle" => Some(icons::CONTACTLESS_PAYMENT_CIRCLE),
        "video-high-definition" => Some(icons::VIDEO_HIGH_DEFINITION),
        "account-arrow-left-outline" => Some(icons::ACCOUNT_ARROW_LEFT_OUTLINE),
        "arrow-expand-down" => Some(icons::ARROW_EXPAND_DOWN),
        "car-pickup" => Some(icons::CAR_PICKUP),
        #[allow(deprecated)]
        "skype-business" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'skype-business' is deprecated.").print(py);
            }
            Some(icons::SKYPE_BUSINESS)
        }
        "file-video-outline" => Some(icons::FILE_VIDEO_OUTLINE),
        "view-carousel" => Some(icons::VIEW_CAROUSEL),
        "weather-partly-cloudy" => Some(icons::WEATHER_PARTLY_CLOUDY),
        "invoice-arrow-left" => Some(icons::INVOICE_ARROW_LEFT),
        "calendar-search" => Some(icons::CALENDAR_SEARCH),
        "bag-personal-off-outline" => Some(icons::BAG_PERSONAL_OFF_OUTLINE),
        "alpha-m-box" => Some(icons::ALPHA_M_BOX),
        "bookmark-box-multiple-outline" => Some(icons::BOOKMARK_BOX_MULTIPLE_OUTLINE),
        "monitor-small" => Some(icons::MONITOR_SMALL),
        "credit-card-off-outline" => Some(icons::CREDIT_CARD_OFF_OUTLINE),
        "medical-cotton-swab" => Some(icons::MEDICAL_COTTON_SWAB),
        "exponent-box" => Some(icons::EXPONENT_BOX),
        "elevator-passenger-outline" => Some(icons::ELEVATOR_PASSENGER_OUTLINE),
        "thumb-up-outline" => Some(icons::THUMB_UP_OUTLINE),
        "diabetes" => Some(icons::DIABETES),
        "firework" => Some(icons::FIREWORK),
        "book-play-outline" => Some(icons::BOOK_PLAY_OUTLINE),
        "share-variant" => Some(icons::SHARE_VARIANT),
        "watch-variant" => Some(icons::WATCH_VARIANT),
        "image-off" => Some(icons::IMAGE_OFF),
        "attachment-plus" => Some(icons::ATTACHMENT_PLUS),
        "information-variant" => Some(icons::INFORMATION_VARIANT),
        "battery-90" => Some(icons::BATTERY_90),
        "image-remove" => Some(icons::IMAGE_REMOVE),
        "code-not-equal-variant" => Some(icons::CODE_NOT_EQUAL_VARIANT),
        "calendar-expand-horizontal" => Some(icons::CALENDAR_EXPAND_HORIZONTAL),
        "toilet" => Some(icons::TOILET),
        "animation" => Some(icons::ANIMATION),
        "medal" => Some(icons::MEDAL),
        "refresh-circle" => Some(icons::REFRESH_CIRCLE),
        "forum-outline" => Some(icons::FORUM_OUTLINE),
        "tooltip-text-outline" => Some(icons::TOOLTIP_TEXT_OUTLINE),
        "checkbox-multiple-marked" => Some(icons::CHECKBOX_MULTIPLE_MARKED),
        "fruit-grapes-outline" => Some(icons::FRUIT_GRAPES_OUTLINE),
        "printer-pos-plus" => Some(icons::PRINTER_POS_PLUS),
        "file-cancel" => Some(icons::FILE_CANCEL),
        "router-network" => Some(icons::ROUTER_NETWORK),
        "alpha-v-box" => Some(icons::ALPHA_V_BOX),
        "upload-network" => Some(icons::UPLOAD_NETWORK),
        "hammer-wrench" => Some(icons::HAMMER_WRENCH),
        "playlist-check" => Some(icons::PLAYLIST_CHECK),
        "speaker-multiple" => Some(icons::SPEAKER_MULTIPLE),
        "airport" => Some(icons::AIRPORT),
        "police-badge-outline" => Some(icons::POLICE_BADGE_OUTLINE),
        "select-group" => Some(icons::SELECT_GROUP),
        "cart-arrow-down" => Some(icons::CART_ARROW_DOWN),
        "account-badge-outline" => Some(icons::ACCOUNT_BADGE_OUTLINE),
        "chart-box-outline" => Some(icons::CHART_BOX_OUTLINE),
        "map-minus" => Some(icons::MAP_MINUS),
        "chevron-right-circle-outline" => Some(icons::CHEVRON_RIGHT_CIRCLE_OUTLINE),
        "projector-screen-variant-off" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF),
        "timeline-minus" => Some(icons::TIMELINE_MINUS),
        "skip-next-outline" => Some(icons::SKIP_NEXT_OUTLINE),
        "numeric-8-circle-outline" => Some(icons::NUMERIC_8_CIRCLE_OUTLINE),
        "arrow-left-bold-box" => Some(icons::ARROW_LEFT_BOLD_BOX),
        "cookie-clock" => Some(icons::COOKIE_CLOCK),
        "timeline-alert" => Some(icons::TIMELINE_ALERT),
        "location-exit" => Some(icons::LOCATION_EXIT),
        "numeric-9-plus-circle" => Some(icons::NUMERIC_9_PLUS_CIRCLE),
        "flower-tulip" => Some(icons::FLOWER_TULIP),
        "eye" => Some(icons::EYE),
        "badge-account-outline" => Some(icons::BADGE_ACCOUNT_OUTLINE),
        "folder-edit-outline" => Some(icons::FOLDER_EDIT_OUTLINE),
        "hand-clap" => Some(icons::HAND_CLAP),
        "package-up" => Some(icons::PACKAGE_UP),
        "storefront-check" => Some(icons::STOREFRONT_CHECK),
        "nail" => Some(icons::NAIL),
        "arrow-right-drop-circle" => Some(icons::ARROW_RIGHT_DROP_CIRCLE),
        "email-open-multiple" => Some(icons::EMAIL_OPEN_MULTIPLE),
        "scent" => Some(icons::SCENT),
        "usb-port" => Some(icons::USB_PORT),
        "comment-flash-outline" => Some(icons::COMMENT_FLASH_OUTLINE),
        "virus-off" => Some(icons::VIRUS_OFF),
        "sort-numeric-variant" => Some(icons::SORT_NUMERIC_VARIANT),
        "relation-zero-or-many-to-one-or-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE_OR_MANY),
        "currency-ils" => Some(icons::CURRENCY_ILS),
        "face-man-shimmer" => Some(icons::FACE_MAN_SHIMMER),
        #[allow(deprecated)]
        "microsoft-dynamics-365" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-dynamics-365' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_DYNAMICS_365)
        }
        "clipboard-text-clock" => Some(icons::CLIPBOARD_TEXT_CLOCK),
        "elevator-passenger-off" => Some(icons::ELEVATOR_PASSENGER_OFF),
        "invoice-text-plus" => Some(icons::INVOICE_TEXT_PLUS),
        "fridge-off-outline" => Some(icons::FRIDGE_OFF_OUTLINE),
        "cow" => Some(icons::COW),
        "cancel" => Some(icons::CANCEL),
        "phone-outgoing-outline" => Some(icons::PHONE_OUTGOING_OUTLINE),
        "check-decagram-outline" => Some(icons::CHECK_DECAGRAM_OUTLINE),
        "table-headers-eye-off" => Some(icons::TABLE_HEADERS_EYE_OFF),
        "car-multiple" => Some(icons::CAR_MULTIPLE),
        "zodiac-pisces" => Some(icons::ZODIAC_PISCES),
        "cash-multiple" => Some(icons::CASH_MULTIPLE),
        "battery-30-bluetooth" => Some(icons::BATTERY_30_BLUETOOTH),
        "lightbulb-multiple-outline" => Some(icons::LIGHTBULB_MULTIPLE_OUTLINE),
        "size-xs" => Some(icons::SIZE_XS),
        "format-text-wrapping-overflow" => Some(icons::FORMAT_TEXT_WRAPPING_OVERFLOW),
        "nature" => Some(icons::NATURE),
        "arrow-expand-up" => Some(icons::ARROW_EXPAND_UP),
        "toy-brick-search-outline" => Some(icons::TOY_BRICK_SEARCH_OUTLINE),
        "puzzle-check" => Some(icons::PUZZLE_CHECK),
        "wallet-membership" => Some(icons::WALLET_MEMBERSHIP),
        "monitor-dashboard" => Some(icons::MONITOR_DASHBOARD),
        "stairs-up" => Some(icons::STAIRS_UP),
        "briefcase-remove" => Some(icons::BRIEFCASE_REMOVE),
        "checkbox-multiple-blank-circle-outline" => {
            Some(icons::CHECKBOX_MULTIPLE_BLANK_CIRCLE_OUTLINE)
        }
        "file-cabinet" => Some(icons::FILE_CABINET),
        "pause-octagon" => Some(icons::PAUSE_OCTAGON),
        "human-male-girl" => Some(icons::HUMAN_MALE_GIRL),
        "stop" => Some(icons::STOP),
        "paragliding" => Some(icons::PARAGLIDING),
        "hand-coin-outline" => Some(icons::HAND_COIN_OUTLINE),
        "battery-charging-wireless-40" => Some(icons::BATTERY_CHARGING_WIRELESS_40),
        "fruit-citrus" => Some(icons::FRUIT_CITRUS),
        "ticket-account" => Some(icons::TICKET_ACCOUNT),
        "clipboard-clock-outline" => Some(icons::CLIPBOARD_CLOCK_OUTLINE),
        "chili-medium" => Some(icons::CHILI_MEDIUM),
        "chat-plus-outline" => Some(icons::CHAT_PLUS_OUTLINE),
        "sticker" => Some(icons::STICKER),
        "coffee-to-go" => Some(icons::COFFEE_TO_GO),
        "alpha-r-circle" => Some(icons::ALPHA_R_CIRCLE),
        "koala" => Some(icons::KOALA),
        "content-save-all-outline" => Some(icons::CONTENT_SAVE_ALL_OUTLINE),
        "hub-outline" => Some(icons::HUB_OUTLINE),
        #[allow(deprecated)]
        "language-php" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-php' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_PHP)
        }
        "arm-flex" => Some(icons::ARM_FLEX),
        "lightbulb-on-10" => Some(icons::LIGHTBULB_ON_10),
        "tower-beach" => Some(icons::TOWER_BEACH),
        "battery-off-outline" => Some(icons::BATTERY_OFF_OUTLINE),
        "pickaxe" => Some(icons::PICKAXE),
        "map-marker-minus-outline" => Some(icons::MAP_MARKER_MINUS_OUTLINE),
        "account-tie-voice-off-outline" => Some(icons::ACCOUNT_TIE_VOICE_OFF_OUTLINE),
        "iv-bag" => Some(icons::IV_BAG),
        "scent-off" => Some(icons::SCENT_OFF),
        "weather-dust" => Some(icons::WEATHER_DUST),
        "close-outline" => Some(icons::CLOSE_OUTLINE),
        "file-hidden" => Some(icons::FILE_HIDDEN),
        "attachment-minus" => Some(icons::ATTACHMENT_MINUS),
        "account-lock" => Some(icons::ACCOUNT_LOCK),
        "credit-card-settings-outline" => Some(icons::CREDIT_CARD_SETTINGS_OUTLINE),
        "track-light" => Some(icons::TRACK_LIGHT),
        "power-socket-de" => Some(icons::POWER_SOCKET_DE),
        "flask-empty" => Some(icons::FLASK_EMPTY),
        "train-car-tank" => Some(icons::TRAIN_CAR_TANK),
        #[allow(deprecated)]
        "iobroker" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'iobroker' is deprecated.").print(py);
            }
            Some(icons::IOBROKER)
        }
        #[allow(deprecated)]
        "nfc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nfc' is deprecated.").print(py);
            }
            Some(icons::NFC)
        }
        "lightbulb-off-outline" => Some(icons::LIGHTBULB_OFF_OUTLINE),
        "format-subscript" => Some(icons::FORMAT_SUBSCRIPT),
        "surround-sound-2-0" => Some(icons::SURROUND_SOUND_2_0),
        "abugida-devanagari" => Some(icons::ABUGIDA_DEVANAGARI),
        #[allow(deprecated)]
        "microsoft-access" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-access' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ACCESS)
        }
        "newspaper-variant" => Some(icons::NEWSPAPER_VARIANT),
        "integrated-circuit-chip" => Some(icons::INTEGRATED_CIRCUIT_CHIP),
        "book-marker-outline" => Some(icons::BOOK_MARKER_OUTLINE),
        "head-lightbulb" => Some(icons::HEAD_LIGHTBULB),
        "history" => Some(icons::HISTORY),
        "filter-menu" => Some(icons::FILTER_MENU),
        "microphone-variant" => Some(icons::MICROPHONE_VARIANT),
        "link-circle" => Some(icons::LINK_CIRCLE),
        "format-quote-close" => Some(icons::FORMAT_QUOTE_CLOSE),
        "valve-closed" => Some(icons::VALVE_CLOSED),
        "video-switch" => Some(icons::VIDEO_SWITCH),
        "receipt-text-check-outline" => Some(icons::RECEIPT_TEXT_CHECK_OUTLINE),
        "smoking-off" => Some(icons::SMOKING_OFF),
        "wifi-remove" => Some(icons::WIFI_REMOVE),
        "script-text-outline" => Some(icons::SCRIPT_TEXT_OUTLINE),
        "chevron-up-box-outline" => Some(icons::CHEVRON_UP_BOX_OUTLINE),
        "rename-box" => Some(icons::RENAME_BOX),
        "folder-plus-outline" => Some(icons::FOLDER_PLUS_OUTLINE),
        "radio-tower" => Some(icons::RADIO_TOWER),
        "dock-left" => Some(icons::DOCK_LEFT),
        "phone-missed" => Some(icons::PHONE_MISSED),
        "card-plus-outline" => Some(icons::CARD_PLUS_OUTLINE),
        "propane-tank" => Some(icons::PROPANE_TANK),
        "content-save-cog" => Some(icons::CONTENT_SAVE_COG),
        "alpha-x-box-outline" => Some(icons::ALPHA_X_BOX_OUTLINE),
        "calendar-month-outline" => Some(icons::CALENDAR_MONTH_OUTLINE),
        "chat-sleep-outline" => Some(icons::CHAT_SLEEP_OUTLINE),
        "comment-edit" => Some(icons::COMMENT_EDIT),
        "floppy-variant" => Some(icons::FLOPPY_VARIANT),
        "clock-time-eight" => Some(icons::CLOCK_TIME_EIGHT),
        "chemical-weapon" => Some(icons::CHEMICAL_WEAPON),
        #[allow(deprecated)]
        "language-markdown" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-markdown' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_MARKDOWN)
        }
        _ => None,
    }
}
