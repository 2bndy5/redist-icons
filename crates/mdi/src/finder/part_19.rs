// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_19(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "toolbox" => Some(icons::TOOLBOX),
        "airplane-plus" => Some(icons::AIRPLANE_PLUS),
        "alpha-h-box-outline" => Some(icons::ALPHA_H_BOX_OUTLINE),
        "exponent" => Some(icons::EXPONENT),
        "home-silo" => Some(icons::HOME_SILO),
        "ribbon" => Some(icons::RIBBON),
        "alpha-e-circle" => Some(icons::ALPHA_E_CIRCLE),
        "alpha-h" => Some(icons::ALPHA_H),
        "cellphone-arrow-down" => Some(icons::CELLPHONE_ARROW_DOWN),
        "food-variant" => Some(icons::FOOD_VARIANT),
        "emoticon-kiss-outline" => Some(icons::EMOTICON_KISS_OUTLINE),
        "leaf-maple" => Some(icons::LEAF_MAPLE),
        "home-variant" => Some(icons::HOME_VARIANT),
        "play-box-edit-outline" => Some(icons::PLAY_BOX_EDIT_OUTLINE),
        "wifi-strength-1" => Some(icons::WIFI_STRENGTH_1),
        #[allow(deprecated)]
        "android" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android' is deprecated.").print(py);
            }
            Some(icons::ANDROID)
        }
        "gate-arrow-right" => Some(icons::GATE_ARROW_RIGHT),
        "numeric-1-circle" => Some(icons::NUMERIC_1_CIRCLE),
        "border-right-variant" => Some(icons::BORDER_RIGHT_VARIANT),
        "alpha-c" => Some(icons::ALPHA_C),
        "book-remove-multiple" => Some(icons::BOOK_REMOVE_MULTIPLE),
        "emoticon-neutral-outline" => Some(icons::EMOTICON_NEUTRAL_OUTLINE),
        "data-matrix-minus" => Some(icons::DATA_MATRIX_MINUS),
        "bug" => Some(icons::BUG),
        "view-sequential-outline" => Some(icons::VIEW_SEQUENTIAL_OUTLINE),
        "data-matrix-plus" => Some(icons::DATA_MATRIX_PLUS),
        "seed-plus" => Some(icons::SEED_PLUS),
        "invoice-text-clock" => Some(icons::INVOICE_TEXT_CLOCK),
        "step-backward" => Some(icons::STEP_BACKWARD),
        "subway" => Some(icons::SUBWAY),
        "home-heart" => Some(icons::HOME_HEART),
        "keyboard-close" => Some(icons::KEYBOARD_CLOSE),
        "calendar-cursor" => Some(icons::CALENDAR_CURSOR),
        "middleware-outline" => Some(icons::MIDDLEWARE_OUTLINE),
        "ladybug" => Some(icons::LADYBUG),
        #[allow(deprecated)]
        "cryengine" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cryengine' is deprecated.").print(py);
            }
            Some(icons::CRYENGINE)
        }
        "folder-arrow-left-right-outline" => Some(icons::FOLDER_ARROW_LEFT_RIGHT_OUTLINE),
        "timer-star-outline" => Some(icons::TIMER_STAR_OUTLINE),
        "home-off-outline" => Some(icons::HOME_OFF_OUTLINE),
        "camera-gopro" => Some(icons::CAMERA_GOPRO),
        "power-socket-eu" => Some(icons::POWER_SOCKET_EU),
        "virtual-reality" => Some(icons::VIRTUAL_REALITY),
        #[allow(deprecated)]
        "spotify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'spotify' is deprecated.").print(py);
            }
            Some(icons::SPOTIFY)
        }
        "phone-check" => Some(icons::PHONE_CHECK),
        "chili-alert" => Some(icons::CHILI_ALERT),
        "inbox-remove" => Some(icons::INBOX_REMOVE),
        "lotion" => Some(icons::LOTION),
        "filter-variant-plus" => Some(icons::FILTER_VARIANT_PLUS),
        "city" => Some(icons::CITY),
        "wallet-bifold" => Some(icons::WALLET_BIFOLD),
        "pasta" => Some(icons::PASTA),
        "gamepad-circle-up" => Some(icons::GAMEPAD_CIRCLE_UP),
        "printer-search" => Some(icons::PRINTER_SEARCH),
        "engine-off-outline" => Some(icons::ENGINE_OFF_OUTLINE),
        "bed-double" => Some(icons::BED_DOUBLE),
        "format-float-none" => Some(icons::FORMAT_FLOAT_NONE),
        "view-comfy-outline" => Some(icons::VIEW_COMFY_OUTLINE),
        "clipboard-search-outline" => Some(icons::CLIPBOARD_SEARCH_OUTLINE),
        "car-sports" => Some(icons::CAR_SPORTS),
        "music-note-eighth" => Some(icons::MUSIC_NOTE_EIGHTH),
        "format-float-left" => Some(icons::FORMAT_FLOAT_LEFT),
        "head-snowflake" => Some(icons::HEAD_SNOWFLAKE),
        "snowboard" => Some(icons::SNOWBOARD),
        "map-marker-distance" => Some(icons::MAP_MARKER_DISTANCE),
        "midi-port" => Some(icons::MIDI_PORT),
        "clock-time-one" => Some(icons::CLOCK_TIME_ONE),
        "phone-hangup" => Some(icons::PHONE_HANGUP),
        "bed-king" => Some(icons::BED_KING),
        "application-variable-outline" => Some(icons::APPLICATION_VARIABLE_OUTLINE),
        "cookie-remove-outline" => Some(icons::COOKIE_REMOVE_OUTLINE),
        "wifi-strength-4-lock-open" => Some(icons::WIFI_STRENGTH_4_LOCK_OPEN),
        "arrow-collapse-vertical" => Some(icons::ARROW_COLLAPSE_VERTICAL),
        "numeric-3-box-outline" => Some(icons::NUMERIC_3_BOX_OUTLINE),
        "cloud-lock-open" => Some(icons::CLOUD_LOCK_OPEN),
        "identifier" => Some(icons::IDENTIFIER),
        "page-first" => Some(icons::PAGE_FIRST),
        "invoice-text-edit-outline" => Some(icons::INVOICE_TEXT_EDIT_OUTLINE),
        "omega" => Some(icons::OMEGA),
        "camera-party-mode" => Some(icons::CAMERA_PARTY_MODE),
        "image-refresh-outline" => Some(icons::IMAGE_REFRESH_OUTLINE),
        "qrcode-scan" => Some(icons::QRCODE_SCAN),
        "square-small" => Some(icons::SQUARE_SMALL),
        "hexagram" => Some(icons::HEXAGRAM),
        "chart-line-variant" => Some(icons::CHART_LINE_VARIANT),
        "sort-reverse-variant" => Some(icons::SORT_REVERSE_VARIANT),
        "consolidate" => Some(icons::CONSOLIDATE),
        "webcam" => Some(icons::WEBCAM),
        "tent" => Some(icons::TENT),
        "file-multiple" => Some(icons::FILE_MULTIPLE),
        "basket-fill" => Some(icons::BASKET_FILL),
        "phone-lock" => Some(icons::PHONE_LOCK),
        "wrench-clock-outline" => Some(icons::WRENCH_CLOCK_OUTLINE),
        "movie-settings" => Some(icons::MOVIE_SETTINGS),
        "folder-settings-outline" => Some(icons::FOLDER_SETTINGS_OUTLINE),
        "party-popper" => Some(icons::PARTY_POPPER),
        "doorbell-video" => Some(icons::DOORBELL_VIDEO),
        "map-marker-circle" => Some(icons::MAP_MARKER_CIRCLE),
        "invoice-text-send-outline" => Some(icons::INVOICE_TEXT_SEND_OUTLINE),
        "bag-personal-plus-outline" => Some(icons::BAG_PERSONAL_PLUS_OUTLINE),
        "folder-multiple" => Some(icons::FOLDER_MULTIPLE),
        "weather-cloudy-alert" => Some(icons::WEATHER_CLOUDY_ALERT),
        "airplane" => Some(icons::AIRPLANE),
        #[allow(deprecated)]
        "language-lua" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-lua' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_LUA)
        }
        "mushroom-outline" => Some(icons::MUSHROOM_OUTLINE),
        "human-male" => Some(icons::HUMAN_MALE),
        #[allow(deprecated)]
        "material-design" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-design' is deprecated.")
                    .print(py);
            }
            Some(icons::MATERIAL_DESIGN)
        }
        "image-minus-outline" => Some(icons::IMAGE_MINUS_OUTLINE),
        "airplane-takeoff" => Some(icons::AIRPLANE_TAKEOFF),
        "format-font-size-increase" => Some(icons::FORMAT_FONT_SIZE_INCREASE),
        "ceiling-light" => Some(icons::CEILING_LIGHT),
        "email-open-heart-outline" => Some(icons::EMAIL_OPEN_HEART_OUTLINE),
        "seat-recline-normal" => Some(icons::SEAT_RECLINE_NORMAL),
        "source-commit-end-local" => Some(icons::SOURCE_COMMIT_END_LOCAL),
        "file-download-outline" => Some(icons::FILE_DOWNLOAD_OUTLINE),
        "hair-dryer" => Some(icons::HAIR_DRYER),
        "hexagon-outline" => Some(icons::HEXAGON_OUTLINE),
        "currency-php" => Some(icons::CURRENCY_PHP),
        "database-settings" => Some(icons::DATABASE_SETTINGS),
        "calculator-variant" => Some(icons::CALCULATOR_VARIANT),
        "flash-alert" => Some(icons::FLASH_ALERT),
        "key-remove" => Some(icons::KEY_REMOVE),
        "message-reply-outline" => Some(icons::MESSAGE_REPLY_OUTLINE),
        "fountain-pen" => Some(icons::FOUNTAIN_PEN),
        "biathlon" => Some(icons::BIATHLON),
        "dots-horizontal" => Some(icons::DOTS_HORIZONTAL),
        "head-heart" => Some(icons::HEAD_HEART),
        "music" => Some(icons::MUSIC),
        "briefcase-edit" => Some(icons::BRIEFCASE_EDIT),
        "archive-remove-outline" => Some(icons::ARCHIVE_REMOVE_OUTLINE),
        "shape" => Some(icons::SHAPE),
        "flag-minus-outline" => Some(icons::FLAG_MINUS_OUTLINE),
        "skull-outline" => Some(icons::SKULL_OUTLINE),
        "snail" => Some(icons::SNAIL),
        "box-cutter-off" => Some(icons::BOX_CUTTER_OFF),
        "signal-hspa-plus" => Some(icons::SIGNAL_HSPA_PLUS),
        "podium-silver" => Some(icons::PODIUM_SILVER),
        #[allow(deprecated)]
        "kodi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kodi' is deprecated.").print(py);
            }
            Some(icons::KODI)
        }
        "folder-information-outline" => Some(icons::FOLDER_INFORMATION_OUTLINE),
        "star-remove-outline" => Some(icons::STAR_REMOVE_OUTLINE),
        "battery-20-bluetooth" => Some(icons::BATTERY_20_BLUETOOTH),
        "brightness-1" => Some(icons::BRIGHTNESS_1),
        #[allow(deprecated)]
        "google-cardboard" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cardboard' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CARDBOARD)
        }
        "package-variant" => Some(icons::PACKAGE_VARIANT),
        "clock-time-six" => Some(icons::CLOCK_TIME_SIX),
        "bus-alert" => Some(icons::BUS_ALERT),
        "smart-card-outline" => Some(icons::SMART_CARD_OUTLINE),
        #[allow(deprecated)]
        "microsoft-sharepoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-sharepoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_SHAREPOINT)
        }
        "fridge-alert" => Some(icons::FRIDGE_ALERT),
        "square-rounded-badge-outline" => Some(icons::SQUARE_ROUNDED_BADGE_OUTLINE),
        "car-estate" => Some(icons::CAR_ESTATE),
        "baby-bottle-outline" => Some(icons::BABY_BOTTLE_OUTLINE),
        "bell-ring-outline" => Some(icons::BELL_RING_OUTLINE),
        "arrow-right-bold-circle" => Some(icons::ARROW_RIGHT_BOLD_CIRCLE),
        "file-cog-outline" => Some(icons::FILE_COG_OUTLINE),
        "arrow-u-left-top-bold" => Some(icons::ARROW_U_LEFT_TOP_BOLD),
        #[allow(deprecated)]
        "nix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nix' is deprecated.").print(py);
            }
            Some(icons::NIX)
        }
        "format-text-rotation-down-vertical" => Some(icons::FORMAT_TEXT_ROTATION_DOWN_VERTICAL),
        "cloud-cog-outline" => Some(icons::CLOUD_COG_OUTLINE),
        "sim-outline" => Some(icons::SIM_OUTLINE),
        "network-strength-4-alert" => Some(icons::NETWORK_STRENGTH_4_ALERT),
        #[allow(deprecated)]
        "vuejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuejs' is deprecated.").print(py);
            }
            Some(icons::VUEJS)
        }
        "zodiac-sagittarius" => Some(icons::ZODIAC_SAGITTARIUS),
        "briefcase-upload" => Some(icons::BRIEFCASE_UPLOAD),
        "music-rest-half" => Some(icons::MUSIC_REST_HALF),
        "database-arrow-right-outline" => Some(icons::DATABASE_ARROW_RIGHT_OUTLINE),
        "tally-mark-5" => Some(icons::TALLY_MARK_5),
        "car-emergency" => Some(icons::CAR_EMERGENCY),
        "folder-minus" => Some(icons::FOLDER_MINUS),
        "chart-arc" => Some(icons::CHART_ARC),
        "brush-outline" => Some(icons::BRUSH_OUTLINE),
        "horse-variant-fast" => Some(icons::HORSE_VARIANT_FAST),
        "panda" => Some(icons::PANDA),
        "page-layout-sidebar-left" => Some(icons::PAGE_LAYOUT_SIDEBAR_LEFT),
        "train-car-flatbed-car" => Some(icons::TRAIN_CAR_FLATBED_CAR),
        "fruit-cherries" => Some(icons::FRUIT_CHERRIES),
        "account-multiple-minus" => Some(icons::ACCOUNT_MULTIPLE_MINUS),
        "hexagon-slice-6" => Some(icons::HEXAGON_SLICE_6),
        "pen-minus" => Some(icons::PEN_MINUS),
        "arrow-up-left-bold" => Some(icons::ARROW_UP_LEFT_BOLD),
        "office-building-remove-outline" => Some(icons::OFFICE_BUILDING_REMOVE_OUTLINE),
        "emoticon-angry-outline" => Some(icons::EMOTICON_ANGRY_OUTLINE),
        "content-save-settings" => Some(icons::CONTENT_SAVE_SETTINGS),
        "home-sound-out" => Some(icons::HOME_SOUND_OUT),
        "select-arrow-down" => Some(icons::SELECT_ARROW_DOWN),
        "lightbulb-question-outline" => Some(icons::LIGHTBULB_QUESTION_OUTLINE),
        "weather-hail" => Some(icons::WEATHER_HAIL),
        "account-details" => Some(icons::ACCOUNT_DETAILS),
        "note-minus-outline" => Some(icons::NOTE_MINUS_OUTLINE),
        "alpha-u-circle-outline" => Some(icons::ALPHA_U_CIRCLE_OUTLINE),
        "tow-truck" => Some(icons::TOW_TRUCK),
        "file-export-outline" => Some(icons::FILE_EXPORT_OUTLINE),
        "store-check-outline" => Some(icons::STORE_CHECK_OUTLINE),
        "switch" => Some(icons::SWITCH),
        "selection-off" => Some(icons::SELECTION_OFF),
        "lightbulb-night-outline" => Some(icons::LIGHTBULB_NIGHT_OUTLINE),
        "cash-register" => Some(icons::CASH_REGISTER),
        "battery-charging-wireless-90" => Some(icons::BATTERY_CHARGING_WIRELESS_90),
        "store-clock-outline" => Some(icons::STORE_CLOCK_OUTLINE),
        "file-arrow-left-right-outline" => Some(icons::FILE_ARROW_LEFT_RIGHT_OUTLINE),
        "cursor-move" => Some(icons::CURSOR_MOVE),
        _ => None,
    }
}
