// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_3(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "chili-hot" => Some(icons::CHILI_HOT),
        "gift-off-outline" => Some(icons::GIFT_OFF_OUTLINE),
        "invert-colors-off" => Some(icons::INVERT_COLORS_OFF),
        "car-side" => Some(icons::CAR_SIDE),
        "beaker-check-outline" => Some(icons::BEAKER_CHECK_OUTLINE),
        "horse-variant" => Some(icons::HORSE_VARIANT),
        "handshake" => Some(icons::HANDSHAKE),
        "water-remove" => Some(icons::WATER_REMOVE),
        "power-socket" => Some(icons::POWER_SOCKET),
        "fruit-pear" => Some(icons::FRUIT_PEAR),
        "fan-chevron-down" => Some(icons::FAN_CHEVRON_DOWN),
        "source-commit-start" => Some(icons::SOURCE_COMMIT_START),
        "crop-square" => Some(icons::CROP_SQUARE),
        "arrow-decision" => Some(icons::ARROW_DECISION),
        "phone-plus-outline" => Some(icons::PHONE_PLUS_OUTLINE),
        "newspaper-variant-multiple" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE),
        "stadium" => Some(icons::STADIUM),
        "email-minus-outline" => Some(icons::EMAIL_MINUS_OUTLINE),
        "television-play" => Some(icons::TELEVISION_PLAY),
        "triforce" => Some(icons::TRIFORCE),
        "publish-off" => Some(icons::PUBLISH_OFF),
        "cloud-percent" => Some(icons::CLOUD_PERCENT),
        "window-shutter-settings" => Some(icons::WINDOW_SHUTTER_SETTINGS),
        "cellphone-screenshot" => Some(icons::CELLPHONE_SCREENSHOT),
        "sort" => Some(icons::SORT),
        "food-steak-off" => Some(icons::FOOD_STEAK_OFF),
        "clipboard-minus-outline" => Some(icons::CLIPBOARD_MINUS_OUTLINE),
        "passport-minus" => Some(icons::PASSPORT_MINUS),
        "human-cane" => Some(icons::HUMAN_CANE),
        "bell-ring-outline" => Some(icons::BELL_RING_OUTLINE),
        "gift-open" => Some(icons::GIFT_OPEN),
        "format-text-rotation-down-vertical" => Some(icons::FORMAT_TEXT_ROTATION_DOWN_VERTICAL),
        "tag-text-outline" => Some(icons::TAG_TEXT_OUTLINE),
        "bag-carry-on" => Some(icons::BAG_CARRY_ON),
        "link-box-outline" => Some(icons::LINK_BOX_OUTLINE),
        "phone-voip" => Some(icons::PHONE_VOIP),
        "camera-wireless" => Some(icons::CAMERA_WIRELESS),
        "pretzel" => Some(icons::PRETZEL),
        "hospital-marker" => Some(icons::HOSPITAL_MARKER),
        "numeric-7-box-multiple-outline" => Some(icons::NUMERIC_7_BOX_MULTIPLE_OUTLINE),
        "home-flood" => Some(icons::HOME_FLOOD),
        "file-plus" => Some(icons::FILE_PLUS),
        "clipboard-search" => Some(icons::CLIPBOARD_SEARCH),
        "chart-pie" => Some(icons::CHART_PIE),
        "mouse-move-down" => Some(icons::MOUSE_MOVE_DOWN),
        "cards-heart" => Some(icons::CARDS_HEART),
        "smart-card-outline" => Some(icons::SMART_CARD_OUTLINE),
        "swap-vertical-circle-outline" => Some(icons::SWAP_VERTICAL_CIRCLE_OUTLINE),
        "zodiac-aquarius" => Some(icons::ZODIAC_AQUARIUS),
        #[allow(deprecated)]
        "whatsapp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'whatsapp' is deprecated.").print(py);
            }
            Some(icons::WHATSAPP)
        }
        "email-remove-outline" => Some(icons::EMAIL_REMOVE_OUTLINE),
        "format-text-rotation-up" => Some(icons::FORMAT_TEXT_ROTATION_UP),
        "inbox-full" => Some(icons::INBOX_FULL),
        "weather-moonset-up" => Some(icons::WEATHER_MOONSET_UP),
        "magnify-expand" => Some(icons::MAGNIFY_EXPAND),
        "text-box-search" => Some(icons::TEXT_BOX_SEARCH),
        "playlist-plus" => Some(icons::PLAYLIST_PLUS),
        "mailbox-open-up-outline" => Some(icons::MAILBOX_OPEN_UP_OUTLINE),
        "file-document-remove-outline" => Some(icons::FILE_DOCUMENT_REMOVE_OUTLINE),
        "death-star" => Some(icons::DEATH_STAR),
        "cloud-question" => Some(icons::CLOUD_QUESTION),
        "bookmark-outline" => Some(icons::BOOKMARK_OUTLINE),
        "information-variant" => Some(icons::INFORMATION_VARIANT),
        "checkbox-multiple-blank" => Some(icons::CHECKBOX_MULTIPLE_BLANK),
        "storefront-remove" => Some(icons::STOREFRONT_REMOVE),
        #[allow(deprecated)]
        "digital-ocean" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'digital-ocean' is deprecated.").print(py);
            }
            Some(icons::DIGITAL_OCEAN)
        }
        "synagogue" => Some(icons::SYNAGOGUE),
        "format-letter-starts-with" => Some(icons::FORMAT_LETTER_STARTS_WITH),
        "wrench-clock-outline" => Some(icons::WRENCH_CLOCK_OUTLINE),
        "seatbelt" => Some(icons::SEATBELT),
        "clipboard-arrow-left" => Some(icons::CLIPBOARD_ARROW_LEFT),
        "magnify-scan" => Some(icons::MAGNIFY_SCAN),
        "wifi-arrow-up-down" => Some(icons::WIFI_ARROW_UP_DOWN),
        "skull-outline" => Some(icons::SKULL_OUTLINE),
        "robot-happy-outline" => Some(icons::ROBOT_HAPPY_OUTLINE),
        "format-bold" => Some(icons::FORMAT_BOLD),
        "soundbar" => Some(icons::SOUNDBAR),
        "book-clock-outline" => Some(icons::BOOK_CLOCK_OUTLINE),
        "rolodex-outline" => Some(icons::ROLODEX_OUTLINE),
        "receipt-send-outline" => Some(icons::RECEIPT_SEND_OUTLINE),
        "image-lock-outline" => Some(icons::IMAGE_LOCK_OUTLINE),
        "cookie-outline" => Some(icons::COOKIE_OUTLINE),
        "flash-alert-outline" => Some(icons::FLASH_ALERT_OUTLINE),
        "store-search-outline" => Some(icons::STORE_SEARCH_OUTLINE),
        "credit-card-edit" => Some(icons::CREDIT_CARD_EDIT),
        "flower-tulip-outline" => Some(icons::FLOWER_TULIP_OUTLINE),
        "paperclip" => Some(icons::PAPERCLIP),
        "message-arrow-left-outline" => Some(icons::MESSAGE_ARROW_LEFT_OUTLINE),
        "fridge-industrial-off" => Some(icons::FRIDGE_INDUSTRIAL_OFF),
        "arrange-send-to-back" => Some(icons::ARRANGE_SEND_TO_BACK),
        #[allow(deprecated)]
        "microsoft-teams" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-teams' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_TEAMS)
        }
        "arrow-down-bold-circle-outline" => Some(icons::ARROW_DOWN_BOLD_CIRCLE_OUTLINE),
        "video-wireless-outline" => Some(icons::VIDEO_WIRELESS_OUTLINE),
        "snowmobile" => Some(icons::SNOWMOBILE),
        "car-windshield" => Some(icons::CAR_WINDSHIELD),
        "contain-start" => Some(icons::CONTAIN_START),
        "numeric-10-box" => Some(icons::NUMERIC_10_BOX),
        "vector-polyline-remove" => Some(icons::VECTOR_POLYLINE_REMOVE),
        "menu-up" => Some(icons::MENU_UP),
        "arrow-u-up-left-bold" => Some(icons::ARROW_U_UP_LEFT_BOLD),
        #[allow(deprecated)]
        "google" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google' is deprecated.").print(py);
            }
            Some(icons::GOOGLE)
        }
        "credit-card-chip" => Some(icons::CREDIT_CARD_CHIP),
        "tooth" => Some(icons::TOOTH),
        "shield-sync" => Some(icons::SHIELD_SYNC),
        "account-multiple-check-outline" => Some(icons::ACCOUNT_MULTIPLE_CHECK_OUTLINE),
        "calendar-account-outline" => Some(icons::CALENDAR_ACCOUNT_OUTLINE),
        "folder-table" => Some(icons::FOLDER_TABLE),
        "home-floor-g" => Some(icons::HOME_FLOOR_G),
        "weather-partly-snowy-rainy" => Some(icons::WEATHER_PARTLY_SNOWY_RAINY),
        "sign-caution" => Some(icons::SIGN_CAUTION),
        "cookie-lock" => Some(icons::COOKIE_LOCK),
        "human-female-dance" => Some(icons::HUMAN_FEMALE_DANCE),
        "vector-arrange-below" => Some(icons::VECTOR_ARRANGE_BELOW),
        "comma-circle-outline" => Some(icons::COMMA_CIRCLE_OUTLINE),
        "image-plus-outline" => Some(icons::IMAGE_PLUS_OUTLINE),
        "temperature-celsius" => Some(icons::TEMPERATURE_CELSIUS),
        "tab-search" => Some(icons::TAB_SEARCH),
        "attachment-plus" => Some(icons::ATTACHMENT_PLUS),
        "garage-open-variant" => Some(icons::GARAGE_OPEN_VARIANT),
        "chat-processing" => Some(icons::CHAT_PROCESSING),
        "dice-d8-outline" => Some(icons::DICE_D8_OUTLINE),
        "comment-text" => Some(icons::COMMENT_TEXT),
        "skull-scan" => Some(icons::SKULL_SCAN),
        "dresser" => Some(icons::DRESSER),
        "blinds-horizontal-closed" => Some(icons::BLINDS_HORIZONTAL_CLOSED),
        "relation-many-to-zero-or-many" => Some(icons::RELATION_MANY_TO_ZERO_OR_MANY),
        "mustache" => Some(icons::MUSTACHE),
        "bomb" => Some(icons::BOMB),
        "arrow-up-left" => Some(icons::ARROW_UP_LEFT),
        #[allow(deprecated)]
        "wikipedia" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wikipedia' is deprecated.").print(py);
            }
            Some(icons::WIKIPEDIA)
        }
        "radioactive-circle" => Some(icons::RADIOACTIVE_CIRCLE),
        "sphere-off" => Some(icons::SPHERE_OFF),
        "music-note-whole-dotted" => Some(icons::MUSIC_NOTE_WHOLE_DOTTED),
        "sticker-remove-outline" => Some(icons::STICKER_REMOVE_OUTLINE),
        "food-takeout-box" => Some(icons::FOOD_TAKEOUT_BOX),
        "invoice-text-check" => Some(icons::INVOICE_TEXT_CHECK),
        "factory" => Some(icons::FACTORY),
        "fan-plus" => Some(icons::FAN_PLUS),
        "gate-alert" => Some(icons::GATE_ALERT),
        "sort-calendar-descending" => Some(icons::SORT_CALENDAR_DESCENDING),
        "barcode" => Some(icons::BARCODE),
        "hat-fedora" => Some(icons::HAT_FEDORA),
        "rectangle" => Some(icons::RECTANGLE),
        "forum-outline" => Some(icons::FORUM_OUTLINE),
        "image-broken-variant" => Some(icons::IMAGE_BROKEN_VARIANT),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "selection-search" => Some(icons::SELECTION_SEARCH),
        "feature-search" => Some(icons::FEATURE_SEARCH),
        #[allow(deprecated)]
        "language-fortran" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-fortran' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_FORTRAN)
        }
        "briefcase-edit" => Some(icons::BRIEFCASE_EDIT),
        "border-all" => Some(icons::BORDER_ALL),
        "data-matrix-remove" => Some(icons::DATA_MATRIX_REMOVE),
        "earth-plus" => Some(icons::EARTH_PLUS),
        "arrow-bottom-right" => Some(icons::ARROW_BOTTOM_RIGHT),
        "food-apple" => Some(icons::FOOD_APPLE),
        "church" => Some(icons::CHURCH),
        "note-off" => Some(icons::NOTE_OFF),
        "microphone-off" => Some(icons::MICROPHONE_OFF),
        "arrow-down-drop-circle" => Some(icons::ARROW_DOWN_DROP_CIRCLE),
        "account-search-outline" => Some(icons::ACCOUNT_SEARCH_OUTLINE),
        "sigma" => Some(icons::SIGMA),
        "bell-check" => Some(icons::BELL_CHECK),
        "decimal-comma-increase" => Some(icons::DECIMAL_COMMA_INCREASE),
        "lock-open-check-outline" => Some(icons::LOCK_OPEN_CHECK_OUTLINE),
        "arrow-top-left-bold-box-outline" => Some(icons::ARROW_TOP_LEFT_BOLD_BOX_OUTLINE),
        "printer-settings" => Some(icons::PRINTER_SETTINGS),
        "credit-card-wireless" => Some(icons::CREDIT_CARD_WIRELESS),
        "wifi" => Some(icons::WIFI),
        "battery-charging-70" => Some(icons::BATTERY_CHARGING_70),
        "printer-pos-plus-outline" => Some(icons::PRINTER_POS_PLUS_OUTLINE),
        "music-note-bluetooth-off" => Some(icons::MUSIC_NOTE_BLUETOOTH_OFF),
        "help-network" => Some(icons::HELP_NETWORK),
        "book-cog-outline" => Some(icons::BOOK_COG_OUTLINE),
        "greenhouse" => Some(icons::GREENHOUSE),
        "selection-marker" => Some(icons::SELECTION_MARKER),
        "checkbox-blank-off" => Some(icons::CHECKBOX_BLANK_OFF),
        "dice-3" => Some(icons::DICE_3),
        "sun-compass" => Some(icons::SUN_COMPASS),
        "palette-swatch-variant" => Some(icons::PALETTE_SWATCH_VARIANT),
        "balloon" => Some(icons::BALLOON),
        "credit-card-multiple-outline" => Some(icons::CREDIT_CARD_MULTIPLE_OUTLINE),
        "clipboard-check-multiple-outline" => Some(icons::CLIPBOARD_CHECK_MULTIPLE_OUTLINE),
        "home-lightning-bolt" => Some(icons::HOME_LIGHTNING_BOLT),
        "folder-check" => Some(icons::FOLDER_CHECK),
        "timeline-minus" => Some(icons::TIMELINE_MINUS),
        "train-car-hopper" => Some(icons::TRAIN_CAR_HOPPER),
        "code-array" => Some(icons::CODE_ARRAY),
        "cast-connected" => Some(icons::CAST_CONNECTED),
        "reload-alert" => Some(icons::RELOAD_ALERT),
        "unfold-more-vertical" => Some(icons::UNFOLD_MORE_VERTICAL),
        "phone-outgoing-outline" => Some(icons::PHONE_OUTGOING_OUTLINE),
        "map-clock" => Some(icons::MAP_CLOCK),
        "invoice-fast" => Some(icons::INVOICE_FAST),
        "apple-keyboard-shift" => Some(icons::APPLE_KEYBOARD_SHIFT),
        "table-key" => Some(icons::TABLE_KEY),
        #[allow(deprecated)]
        "vuejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuejs' is deprecated.").print(py);
            }
            Some(icons::VUEJS)
        }
        "printer-3d-nozzle-heat-outline" => Some(icons::PRINTER_3D_NOZZLE_HEAT_OUTLINE),
        "volume-plus" => Some(icons::VOLUME_PLUS),
        "eye-arrow-left" => Some(icons::EYE_ARROW_LEFT),
        "lightbulb-question-outline" => Some(icons::LIGHTBULB_QUESTION_OUTLINE),
        _ => None,
    }
}
