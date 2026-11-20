// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_26(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "phone-lock" => Some(icons::PHONE_LOCK),
        "wifi-arrow-down" => Some(icons::WIFI_ARROW_DOWN),
        "quality-medium" => Some(icons::QUALITY_MEDIUM),
        "beehive-off-outline" => Some(icons::BEEHIVE_OFF_OUTLINE),
        "ghost" => Some(icons::GHOST),
        "vector-difference" => Some(icons::VECTOR_DIFFERENCE),
        "invoice-text-arrow-left" => Some(icons::INVOICE_TEXT_ARROW_LEFT),
        "barcode" => Some(icons::BARCODE),
        "cellphone-message" => Some(icons::CELLPHONE_MESSAGE),
        "cloud-check" => Some(icons::CLOUD_CHECK),
        "invoice-text-arrow-right-outline" => Some(icons::INVOICE_TEXT_ARROW_RIGHT_OUTLINE),
        "border-inside" => Some(icons::BORDER_INSIDE),
        "play-outline" => Some(icons::PLAY_OUTLINE),
        "view-quilt" => Some(icons::VIEW_QUILT),
        "checkbox-multiple-marked-circle-outline" => {
            Some(icons::CHECKBOX_MULTIPLE_MARKED_CIRCLE_OUTLINE)
        }
        "bell-ring" => Some(icons::BELL_RING),
        "head-alert" => Some(icons::HEAD_ALERT),
        "triangle-outline" => Some(icons::TRIANGLE_OUTLINE),
        "movie-filter-outline" => Some(icons::MOVIE_FILTER_OUTLINE),
        "account-check-outline" => Some(icons::ACCOUNT_CHECK_OUTLINE),
        "phone-cancel" => Some(icons::PHONE_CANCEL),
        "pencil-outline" => Some(icons::PENCIL_OUTLINE),
        "message-settings" => Some(icons::MESSAGE_SETTINGS),
        "lock-open-check-outline" => Some(icons::LOCK_OPEN_CHECK_OUTLINE),
        "train-car-flatbed" => Some(icons::TRAIN_CAR_FLATBED),
        "fridge-bottom" => Some(icons::FRIDGE_BOTTOM),
        "code-tags-check" => Some(icons::CODE_TAGS_CHECK),
        "relation-zero-or-many-to-one-or-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE_OR_MANY),
        #[allow(deprecated)]
        "microsoft-windows-classic" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-windows-classic' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS_CLASSIC)
        }
        "earth-box-remove" => Some(icons::EARTH_BOX_REMOVE),
        "numeric-negative-1" => Some(icons::NUMERIC_NEGATIVE_1),
        "file-image-remove" => Some(icons::FILE_IMAGE_REMOVE),
        "ip-network-outline" => Some(icons::IP_NETWORK_OUTLINE),
        "xml" => Some(icons::XML),
        "function" => Some(icons::FUNCTION),
        "bank" => Some(icons::BANK),
        "shape-square-plus" => Some(icons::SHAPE_SQUARE_PLUS),
        "minus-circle-multiple-outline" => Some(icons::MINUS_CIRCLE_MULTIPLE_OUTLINE),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "resistor" => Some(icons::RESISTOR),
        "kangaroo" => Some(icons::KANGAROO),
        "image-remove-outline" => Some(icons::IMAGE_REMOVE_OUTLINE),
        "barrel-outline" => Some(icons::BARREL_OUTLINE),
        "head-flash-outline" => Some(icons::HEAD_FLASH_OUTLINE),
        "lock-alert-outline" => Some(icons::LOCK_ALERT_OUTLINE),
        "store-settings-outline" => Some(icons::STORE_SETTINGS_OUTLINE),
        "pencil-ruler-outline" => Some(icons::PENCIL_RULER_OUTLINE),
        #[allow(deprecated)]
        "skype-business" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'skype-business' is deprecated.").print(py);
            }
            Some(icons::SKYPE_BUSINESS)
        }
        "sim" => Some(icons::SIM),
        "clock-time-twelve-outline" => Some(icons::CLOCK_TIME_TWELVE_OUTLINE),
        "silo-outline" => Some(icons::SILO_OUTLINE),
        "clock-time-nine" => Some(icons::CLOCK_TIME_NINE),
        "gamepad-round-up" => Some(icons::GAMEPAD_ROUND_UP),
        "format-header-equal" => Some(icons::FORMAT_HEADER_EQUAL),
        "faucet-variant" => Some(icons::FAUCET_VARIANT),
        "folder-key-network" => Some(icons::FOLDER_KEY_NETWORK),
        "symbol" => Some(icons::SYMBOL),
        "bus-alert" => Some(icons::BUS_ALERT),
        "weather-rainy" => Some(icons::WEATHER_RAINY),
        "checkbox-blank" => Some(icons::CHECKBOX_BLANK),
        "printer-3d-nozzle-alert-outline" => Some(icons::PRINTER_3D_NOZZLE_ALERT_OUTLINE),
        "image-check" => Some(icons::IMAGE_CHECK),
        "emoticon-sick" => Some(icons::EMOTICON_SICK),
        "playlist-music-outline" => Some(icons::PLAYLIST_MUSIC_OUTLINE),
        "file-music-outline" => Some(icons::FILE_MUSIC_OUTLINE),
        "cards-playing-diamond-outline" => Some(icons::CARDS_PLAYING_DIAMOND_OUTLINE),
        "package-variant-closed-plus" => Some(icons::PACKAGE_VARIANT_CLOSED_PLUS),
        "thought-bubble" => Some(icons::THOUGHT_BUBBLE),
        "toy-brick-plus" => Some(icons::TOY_BRICK_PLUS),
        "subdirectory-arrow-left" => Some(icons::SUBDIRECTORY_ARROW_LEFT),
        "refresh-auto" => Some(icons::REFRESH_AUTO),
        "folder" => Some(icons::FOLDER),
        "storefront-check" => Some(icons::STOREFRONT_CHECK),
        "clock-time-five-outline" => Some(icons::CLOCK_TIME_FIVE_OUTLINE),
        "hand-water" => Some(icons::HAND_WATER),
        "lightbulb-group-outline" => Some(icons::LIGHTBULB_GROUP_OUTLINE),
        "dots-hexagon" => Some(icons::DOTS_HEXAGON),
        "checkbox-multiple-outline" => Some(icons::CHECKBOX_MULTIPLE_OUTLINE),
        "format-rotate-90" => Some(icons::FORMAT_ROTATE_90),
        "turkey" => Some(icons::TURKEY),
        "calendar-alert-outline" => Some(icons::CALENDAR_ALERT_OUTLINE),
        "file-table-outline" => Some(icons::FILE_TABLE_OUTLINE),
        "image-lock-outline" => Some(icons::IMAGE_LOCK_OUTLINE),
        "camera-metering-spot" => Some(icons::CAMERA_METERING_SPOT),
        "stairs-box" => Some(icons::STAIRS_BOX),
        "basket-remove" => Some(icons::BASKET_REMOVE),
        "printer-pos-play" => Some(icons::PRINTER_POS_PLAY),
        "briefcase-check" => Some(icons::BRIEFCASE_CHECK),
        #[allow(deprecated)]
        "gnome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gnome' is deprecated.").print(py);
            }
            Some(icons::GNOME)
        }
        "earth-box" => Some(icons::EARTH_BOX),
        "train-car-box" => Some(icons::TRAIN_CAR_BOX),
        "coach-lamp" => Some(icons::COACH_LAMP),
        "alpha-f-circle-outline" => Some(icons::ALPHA_F_CIRCLE_OUTLINE),
        "human-queue" => Some(icons::HUMAN_QUEUE),
        "invoice-outline" => Some(icons::INVOICE_OUTLINE),
        "battery-charging-wireless-90" => Some(icons::BATTERY_CHARGING_WIRELESS_90),
        "account-multiple" => Some(icons::ACCOUNT_MULTIPLE),
        "car-hatchback" => Some(icons::CAR_HATCHBACK),
        "motion-sensor" => Some(icons::MOTION_SENSOR),
        "play-box" => Some(icons::PLAY_BOX),
        "string-lights" => Some(icons::STRING_LIGHTS),
        "printer-pos-check-outline" => Some(icons::PRINTER_POS_CHECK_OUTLINE),
        "inbox-full-outline" => Some(icons::INBOX_FULL_OUTLINE),
        "watch-import-variant" => Some(icons::WATCH_IMPORT_VARIANT),
        "clipboard-arrow-right-outline" => Some(icons::CLIPBOARD_ARROW_RIGHT_OUTLINE),
        "home-analytics" => Some(icons::HOME_ANALYTICS),
        "chevron-up-circle" => Some(icons::CHEVRON_UP_CIRCLE),
        "glass-mug-variant" => Some(icons::GLASS_MUG_VARIANT),
        #[allow(deprecated)]
        "freebsd" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'freebsd' is deprecated.").print(py);
            }
            Some(icons::FREEBSD)
        }
        "briefcase-clock" => Some(icons::BRIEFCASE_CLOCK),
        "file-excel-box-outline" => Some(icons::FILE_EXCEL_BOX_OUTLINE),
        "tape-drive" => Some(icons::TAPE_DRIVE),
        "account-tie-voice-off-outline" => Some(icons::ACCOUNT_TIE_VOICE_OFF_OUTLINE),
        "bathtub" => Some(icons::BATHTUB),
        "tablet" => Some(icons::TABLET),
        "distribute-vertical-bottom" => Some(icons::DISTRIBUTE_VERTICAL_BOTTOM),
        "shoe-ballet" => Some(icons::SHOE_BALLET),
        "cloud-lock" => Some(icons::CLOUD_LOCK),
        "arrow-up-bold-box-outline" => Some(icons::ARROW_UP_BOLD_BOX_OUTLINE),
        "trackpad-lock" => Some(icons::TRACKPAD_LOCK),
        "sign-language-outline" => Some(icons::SIGN_LANGUAGE_OUTLINE),
        "circular-saw" => Some(icons::CIRCULAR_SAW),
        "camera-lock-open-outline" => Some(icons::CAMERA_LOCK_OPEN_OUTLINE),
        "flask-round-bottom-outline" => Some(icons::FLASK_ROUND_BOTTOM_OUTLINE),
        "water-off" => Some(icons::WATER_OFF),
        "table-of-contents" => Some(icons::TABLE_OF_CONTENTS),
        "alpha-y" => Some(icons::ALPHA_Y),
        "tag-remove" => Some(icons::TAG_REMOVE),
        "alpha-d-circle" => Some(icons::ALPHA_D_CIRCLE),
        "account-card" => Some(icons::ACCOUNT_CARD),
        "select-compare" => Some(icons::SELECT_COMPARE),
        "rabbit-variant" => Some(icons::RABBIT_VARIANT),
        "file-document-minus" => Some(icons::FILE_DOCUMENT_MINUS),
        "battery-plus-outline" => Some(icons::BATTERY_PLUS_OUTLINE),
        "web-clock" => Some(icons::WEB_CLOCK),
        "chili-medium" => Some(icons::CHILI_MEDIUM),
        "hdmi-port" => Some(icons::HDMI_PORT),
        "magnet-on" => Some(icons::MAGNET_ON),
        "thermometer-probe-off" => Some(icons::THERMOMETER_PROBE_OFF),
        "bag-suitcase" => Some(icons::BAG_SUITCASE),
        "keyboard-variant" => Some(icons::KEYBOARD_VARIANT),
        "pail" => Some(icons::PAIL),
        "alphabet-greek" => Some(icons::ALPHABET_GREEK),
        "square-rounded-badge" => Some(icons::SQUARE_ROUNDED_BADGE),
        "face-man-profile" => Some(icons::FACE_MAN_PROFILE),
        "hook" => Some(icons::HOOK),
        "file-cloud-outline" => Some(icons::FILE_CLOUD_OUTLINE),
        "cpu-64-bit" => Some(icons::CPU_64_BIT),
        "apps-box" => Some(icons::APPS_BOX),
        "calendar-end" => Some(icons::CALENDAR_END),
        "bag-personal-off-outline" => Some(icons::BAG_PERSONAL_OFF_OUTLINE),
        "water-polo" => Some(icons::WATER_POLO),
        "pan-right" => Some(icons::PAN_RIGHT),
        "fan" => Some(icons::FAN),
        "arrow-left-top" => Some(icons::ARROW_LEFT_TOP),
        "clock-star-four-points" => Some(icons::CLOCK_STAR_FOUR_POINTS),
        "eye-settings-outline" => Some(icons::EYE_SETTINGS_OUTLINE),
        "printer-3d-nozzle-off-outline" => Some(icons::PRINTER_3D_NOZZLE_OFF_OUTLINE),
        "book-lock-open-outline" => Some(icons::BOOK_LOCK_OPEN_OUTLINE),
        "tray-arrow-up" => Some(icons::TRAY_ARROW_UP),
        "format-text" => Some(icons::FORMAT_TEXT),
        "vector-bezier" => Some(icons::VECTOR_BEZIER),
        "alpha-n-circle-outline" => Some(icons::ALPHA_N_CIRCLE_OUTLINE),
        "clipboard-text-outline" => Some(icons::CLIPBOARD_TEXT_OUTLINE),
        "printer-pos-cancel" => Some(icons::PRINTER_POS_CANCEL),
        "label-multiple-outline" => Some(icons::LABEL_MULTIPLE_OUTLINE),
        "chart-box-multiple-outline" => Some(icons::CHART_BOX_MULTIPLE_OUTLINE),
        "dialpad" => Some(icons::DIALPAD),
        "archive-settings-outline" => Some(icons::ARCHIVE_SETTINGS_OUTLINE),
        "lightbulb-alert-outline" => Some(icons::LIGHTBULB_ALERT_OUTLINE),
        "keyboard-off" => Some(icons::KEYBOARD_OFF),
        "folder-multiple-image" => Some(icons::FOLDER_MULTIPLE_IMAGE),
        "bell-alert-outline" => Some(icons::BELL_ALERT_OUTLINE),
        "drawing" => Some(icons::DRAWING),
        "book-open-blank-variant" => Some(icons::BOOK_OPEN_BLANK_VARIANT),
        "headphones-off" => Some(icons::HEADPHONES_OFF),
        "hand-front-left" => Some(icons::HAND_FRONT_LEFT),
        "car-light-high" => Some(icons::CAR_LIGHT_HIGH),
        "video-3d-variant" => Some(icons::VIDEO_3D_VARIANT),
        "swap-horizontal-hidden" => Some(icons::SWAP_HORIZONTAL_HIDDEN),
        "white-balance-iridescent" => Some(icons::WHITE_BALANCE_IRIDESCENT),
        "folder-marker-outline" => Some(icons::FOLDER_MARKER_OUTLINE),
        "phone-check-outline" => Some(icons::PHONE_CHECK_OUTLINE),
        "forum-plus" => Some(icons::FORUM_PLUS),
        "food-variant-off" => Some(icons::FOOD_VARIANT_OFF),
        "file-rotate-left" => Some(icons::FILE_ROTATE_LEFT),
        "flower-outline" => Some(icons::FLOWER_OUTLINE),
        "alpha-l-circle" => Some(icons::ALPHA_L_CIRCLE),
        "phone-sync-outline" => Some(icons::PHONE_SYNC_OUTLINE),
        "location-enter" => Some(icons::LOCATION_ENTER),
        "email-edit-outline" => Some(icons::EMAIL_EDIT_OUTLINE),
        "gesture-pinch" => Some(icons::GESTURE_PINCH),
        "metronome-tick" => Some(icons::METRONOME_TICK),
        "email-variant" => Some(icons::EMAIL_VARIANT),
        "heart-flash" => Some(icons::HEART_FLASH),
        "boom-gate-outline" => Some(icons::BOOM_GATE_OUTLINE),
        "circle-slice-5" => Some(icons::CIRCLE_SLICE_5),
        "yurt" => Some(icons::YURT),
        "checkbox-multiple-blank-circle" => Some(icons::CHECKBOX_MULTIPLE_BLANK_CIRCLE),
        "transit-connection-horizontal" => Some(icons::TRANSIT_CONNECTION_HORIZONTAL),
        _ => None,
    }
}
