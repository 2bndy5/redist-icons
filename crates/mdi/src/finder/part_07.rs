// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_7(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "card-account-details-star" => Some(icons::CARD_ACCOUNT_DETAILS_STAR),
        "water-plus-outline" => Some(icons::WATER_PLUS_OUTLINE),
        "atm" => Some(icons::ATM),
        "clipboard-arrow-right" => Some(icons::CLIPBOARD_ARROW_RIGHT),
        "information-slab-circle" => Some(icons::INFORMATION_SLAB_CIRCLE),
        "clipboard-file-outline" => Some(icons::CLIPBOARD_FILE_OUTLINE),
        "bullhorn-outline" => Some(icons::BULLHORN_OUTLINE),
        "fax" => Some(icons::FAX),
        "wifi-strength-1-alert" => Some(icons::WIFI_STRENGTH_1_ALERT),
        "wallpaper" => Some(icons::WALLPAPER),
        "touch-text-outline" => Some(icons::TOUCH_TEXT_OUTLINE),
        "shield-lock-open-outline" => Some(icons::SHIELD_LOCK_OPEN_OUTLINE),
        "alpha-k-box-outline" => Some(icons::ALPHA_K_BOX_OUTLINE),
        "format-size" => Some(icons::FORMAT_SIZE),
        "robot-mower" => Some(icons::ROBOT_MOWER),
        "email-edit-outline" => Some(icons::EMAIL_EDIT_OUTLINE),
        "mail" => Some(icons::MAIL),
        "file-send-outline" => Some(icons::FILE_SEND_OUTLINE),
        "puzzle-outline" => Some(icons::PUZZLE_OUTLINE),
        "arrow-bottom-left-bold-box-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_BOX_OUTLINE),
        "flash-auto" => Some(icons::FLASH_AUTO),
        "train-car-flatbed-tank" => Some(icons::TRAIN_CAR_FLATBED_TANK),
        "relation-one-or-many-to-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONE),
        #[allow(deprecated)]
        "snapchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'snapchat' is deprecated.").print(py);
            }
            Some(icons::SNAPCHAT)
        }
        "cash-edit" => Some(icons::CASH_EDIT),
        "comment-arrow-left-outline" => Some(icons::COMMENT_ARROW_LEFT_OUTLINE),
        "pipe-leak" => Some(icons::PIPE_LEAK),
        "home-clock-outline" => Some(icons::HOME_CLOCK_OUTLINE),
        "camera-plus" => Some(icons::CAMERA_PLUS),
        "ski-water" => Some(icons::SKI_WATER),
        "hexagon-slice-3" => Some(icons::HEXAGON_SLICE_3),
        "bag-suitcase-off-outline" => Some(icons::BAG_SUITCASE_OFF_OUTLINE),
        "delete-alert-outline" => Some(icons::DELETE_ALERT_OUTLINE),
        "gate" => Some(icons::GATE),
        "comment-minus" => Some(icons::COMMENT_MINUS),
        "calendar-weekend-outline" => Some(icons::CALENDAR_WEEKEND_OUTLINE),
        "human-male-boy" => Some(icons::HUMAN_MALE_BOY),
        "read" => Some(icons::READ),
        "numeric-6-circle" => Some(icons::NUMERIC_6_CIRCLE),
        #[allow(deprecated)]
        "stack-overflow" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-overflow' is deprecated.").print(py);
            }
            Some(icons::STACK_OVERFLOW)
        }
        "alpha-f-box-outline" => Some(icons::ALPHA_F_BOX_OUTLINE),
        "note-check" => Some(icons::NOTE_CHECK),
        "triangle-wave" => Some(icons::TRIANGLE_WAVE),
        "thermometer-lines" => Some(icons::THERMOMETER_LINES),
        "microphone-message" => Some(icons::MICROPHONE_MESSAGE),
        "hydrogen-station" => Some(icons::HYDROGEN_STATION),
        "text-box-remove" => Some(icons::TEXT_BOX_REMOVE),
        "balcony" => Some(icons::BALCONY),
        "tag-arrow-right-outline" => Some(icons::TAG_ARROW_RIGHT_OUTLINE),
        "credit-card-clock" => Some(icons::CREDIT_CARD_CLOCK),
        "screen-rotation-lock" => Some(icons::SCREEN_ROTATION_LOCK),
        "wizard-hat" => Some(icons::WIZARD_HAT),
        "sprinkler-fire" => Some(icons::SPRINKLER_FIRE),
        "clock-time-eleven-outline" => Some(icons::CLOCK_TIME_ELEVEN_OUTLINE),
        "pier-crane" => Some(icons::PIER_CRANE),
        "skip-backward" => Some(icons::SKIP_BACKWARD),
        "car-parking-lights" => Some(icons::CAR_PARKING_LIGHTS),
        "account-network-off-outline" => Some(icons::ACCOUNT_NETWORK_OFF_OUTLINE),
        "fan-speed-2" => Some(icons::FAN_SPEED_2),
        "application-parentheses" => Some(icons::APPLICATION_PARENTHESES),
        "code-tags-check" => Some(icons::CODE_TAGS_CHECK),
        "email-sync" => Some(icons::EMAIL_SYNC),
        "battery-charging-10" => Some(icons::BATTERY_CHARGING_10),
        "battery-sync-outline" => Some(icons::BATTERY_SYNC_OUTLINE),
        "arrow-down-bold-hexagon-outline" => Some(icons::ARROW_DOWN_BOLD_HEXAGON_OUTLINE),
        "fridge-variant-alert-outline" => Some(icons::FRIDGE_VARIANT_ALERT_OUTLINE),
        "sack" => Some(icons::SACK),
        "home-analytics" => Some(icons::HOME_ANALYTICS),
        "card" => Some(icons::CARD),
        "folder-arrow-up-down" => Some(icons::FOLDER_ARROW_UP_DOWN),
        "camera-account" => Some(icons::CAMERA_ACCOUNT),
        "card-plus" => Some(icons::CARD_PLUS),
        "keyboard-backspace" => Some(icons::KEYBOARD_BACKSPACE),
        "clock-time-eleven" => Some(icons::CLOCK_TIME_ELEVEN),
        "arrow-left-bold-hexagon-outline" => Some(icons::ARROW_LEFT_BOLD_HEXAGON_OUTLINE),
        "printer-pos-minus-outline" => Some(icons::PRINTER_POS_MINUS_OUTLINE),
        "border-style" => Some(icons::BORDER_STYLE),
        "comment-text-multiple" => Some(icons::COMMENT_TEXT_MULTIPLE),
        "puzzle-heart-outline" => Some(icons::PUZZLE_HEART_OUTLINE),
        "application-edit" => Some(icons::APPLICATION_EDIT),
        "pipe" => Some(icons::PIPE),
        "television-stop" => Some(icons::TELEVISION_STOP),
        "sort-alphabetical-ascending-variant" => Some(icons::SORT_ALPHABETICAL_ASCENDING_VARIANT),
        "hammer-screwdriver" => Some(icons::HAMMER_SCREWDRIVER),
        "radioactive-circle-outline" => Some(icons::RADIOACTIVE_CIRCLE_OUTLINE),
        "emoticon-poop-outline" => Some(icons::EMOTICON_POOP_OUTLINE),
        "food-croissant" => Some(icons::FOOD_CROISSANT),
        "ice-pop" => Some(icons::ICE_POP),
        "currency-cny" => Some(icons::CURRENCY_CNY),
        "broadcast" => Some(icons::BROADCAST),
        "human-capacity-decrease" => Some(icons::HUMAN_CAPACITY_DECREASE),
        "seesaw" => Some(icons::SEESAW),
        "cookie-check-outline" => Some(icons::COOKIE_CHECK_OUTLINE),
        "rug" => Some(icons::RUG),
        "robot" => Some(icons::ROBOT),
        "dots-triangle" => Some(icons::DOTS_TRIANGLE),
        "network-outline" => Some(icons::NETWORK_OUTLINE),
        "radiator-disabled" => Some(icons::RADIATOR_DISABLED),
        "brightness-7" => Some(icons::BRIGHTNESS_7),
        "send-clock-outline" => Some(icons::SEND_CLOCK_OUTLINE),
        "account-filter" => Some(icons::ACCOUNT_FILTER),
        "chart-bell-curve-cumulative" => Some(icons::CHART_BELL_CURVE_CUMULATIVE),
        "selection-ellipse-remove" => Some(icons::SELECTION_ELLIPSE_REMOVE),
        "tray-plus" => Some(icons::TRAY_PLUS),
        "instrument-triangle" => Some(icons::INSTRUMENT_TRIANGLE),
        "mailbox-open-up" => Some(icons::MAILBOX_OPEN_UP),
        "storefront-plus-outline" => Some(icons::STOREFRONT_PLUS_OUTLINE),
        "home-automation" => Some(icons::HOME_AUTOMATION),
        "gamepad-round-right" => Some(icons::GAMEPAD_ROUND_RIGHT),
        "alpha-h-box-outline" => Some(icons::ALPHA_H_BOX_OUTLINE),
        "book-edit-outline" => Some(icons::BOOK_EDIT_OUTLINE),
        "wifi-arrow-right" => Some(icons::WIFI_ARROW_RIGHT),
        "battery-80-bluetooth" => Some(icons::BATTERY_80_BLUETOOTH),
        "home-minus-outline" => Some(icons::HOME_MINUS_OUTLINE),
        "vector-ellipse" => Some(icons::VECTOR_ELLIPSE),
        "car-cog" => Some(icons::CAR_COG),
        "wallet-bifold" => Some(icons::WALLET_BIFOLD),
        "sunglasses" => Some(icons::SUNGLASSES),
        "mouse-off" => Some(icons::MOUSE_OFF),
        "vector-difference-ba" => Some(icons::VECTOR_DIFFERENCE_BA),
        "signal-cellular-outline" => Some(icons::SIGNAL_CELLULAR_OUTLINE),
        "upload-box-outline" => Some(icons::UPLOAD_BOX_OUTLINE),
        "calendar-remove-outline" => Some(icons::CALENDAR_REMOVE_OUTLINE),
        "scale-unbalanced" => Some(icons::SCALE_UNBALANCED),
        "bell-cog-outline" => Some(icons::BELL_COG_OUTLINE),
        "cast-audio" => Some(icons::CAST_AUDIO),
        "heart-pulse" => Some(icons::HEART_PULSE),
        "subtitles-outline" => Some(icons::SUBTITLES_OUTLINE),
        "hospital-box-outline" => Some(icons::HOSPITAL_BOX_OUTLINE),
        "archive-arrow-up" => Some(icons::ARCHIVE_ARROW_UP),
        "file-table-box-outline" => Some(icons::FILE_TABLE_BOX_OUTLINE),
        "camera-plus-outline" => Some(icons::CAMERA_PLUS_OUTLINE),
        "book-marker-outline" => Some(icons::BOOK_MARKER_OUTLINE),
        #[allow(deprecated)]
        "npm" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'npm' is deprecated.").print(py);
            }
            Some(icons::NPM)
        }
        "campfire" => Some(icons::CAMPFIRE),
        "windsock" => Some(icons::WINDSOCK),
        "emoticon-sick" => Some(icons::EMOTICON_SICK),
        "sticker-check-outline" => Some(icons::STICKER_CHECK_OUTLINE),
        "file-document-plus-outline" => Some(icons::FILE_DOCUMENT_PLUS_OUTLINE),
        "filmstrip-box-multiple" => Some(icons::FILMSTRIP_BOX_MULTIPLE),
        "code-string" => Some(icons::CODE_STRING),
        "phone-message-outline" => Some(icons::PHONE_MESSAGE_OUTLINE),
        "tshirt-v" => Some(icons::TSHIRT_V),
        "cheese-off" => Some(icons::CHEESE_OFF),
        "circle-opacity" => Some(icons::CIRCLE_OPACITY),
        "table-split-cell" => Some(icons::TABLE_SPLIT_CELL),
        "email-check" => Some(icons::EMAIL_CHECK),
        "wifi-strength-2-lock-open" => Some(icons::WIFI_STRENGTH_2_LOCK_OPEN),
        "forklift" => Some(icons::FORKLIFT),
        #[allow(deprecated)]
        "spotify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'spotify' is deprecated.").print(py);
            }
            Some(icons::SPOTIFY)
        }
        #[allow(deprecated)]
        "fedora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'fedora' is deprecated.").print(py);
            }
            Some(icons::FEDORA)
        }
        "image-filter-vintage" => Some(icons::IMAGE_FILTER_VINTAGE),
        "currency-sign" => Some(icons::CURRENCY_SIGN),
        "comment-processing" => Some(icons::COMMENT_PROCESSING),
        "account-lock-open" => Some(icons::ACCOUNT_LOCK_OPEN),
        "image-marker" => Some(icons::IMAGE_MARKER),
        "ferris-wheel" => Some(icons::FERRIS_WHEEL),
        "weight-kilogram" => Some(icons::WEIGHT_KILOGRAM),
        "disc" => Some(icons::DISC),
        "stairs-up" => Some(icons::STAIRS_UP),
        "chevron-double-up" => Some(icons::CHEVRON_DOUBLE_UP),
        "police-station" => Some(icons::POLICE_STATION),
        "motion-sensor-off" => Some(icons::MOTION_SENSOR_OFF),
        "flower" => Some(icons::FLOWER),
        "track-light" => Some(icons::TRACK_LIGHT),
        "mouse" => Some(icons::MOUSE),
        "content-save-plus-outline" => Some(icons::CONTENT_SAVE_PLUS_OUTLINE),
        "calendar-clock" => Some(icons::CALENDAR_CLOCK),
        "arrow-vertical-lock" => Some(icons::ARROW_VERTICAL_LOCK),
        "bell-cog" => Some(icons::BELL_COG),
        "comment-flash-outline" => Some(icons::COMMENT_FLASH_OUTLINE),
        "zip-box" => Some(icons::ZIP_BOX),
        "close-box-outline" => Some(icons::CLOSE_BOX_OUTLINE),
        "database-minus" => Some(icons::DATABASE_MINUS),
        "close-box-multiple" => Some(icons::CLOSE_BOX_MULTIPLE),
        "ribbon" => Some(icons::RIBBON),
        "border-bottom-variant" => Some(icons::BORDER_BOTTOM_VARIANT),
        "image-broken" => Some(icons::IMAGE_BROKEN),
        "folder-settings" => Some(icons::FOLDER_SETTINGS),
        "glass-wine" => Some(icons::GLASS_WINE),
        "cloud-upload-outline" => Some(icons::CLOUD_UPLOAD_OUTLINE),
        "transcribe" => Some(icons::TRANSCRIBE),
        "vector-point-edit" => Some(icons::VECTOR_POINT_EDIT),
        "table-row" => Some(icons::TABLE_ROW),
        "office-building-cog" => Some(icons::OFFICE_BUILDING_COG),
        "map-marker-question" => Some(icons::MAP_MARKER_QUESTION),
        "storage-tank" => Some(icons::STORAGE_TANK),
        "key-alert-outline" => Some(icons::KEY_ALERT_OUTLINE),
        "shield-moon-outline" => Some(icons::SHIELD_MOON_OUTLINE),
        "check-network-outline" => Some(icons::CHECK_NETWORK_OUTLINE),
        "flag-off-outline" => Some(icons::FLAG_OFF_OUTLINE),
        "sun-angle" => Some(icons::SUN_ANGLE),
        "shield-edit" => Some(icons::SHIELD_EDIT),
        "radio-am" => Some(icons::RADIO_AM),
        "numeric-7-box-multiple" => Some(icons::NUMERIC_7_BOX_MULTIPLE),
        "horseshoe" => Some(icons::HORSESHOE),
        #[allow(deprecated)]
        "folder-google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'folder-google-drive' is deprecated.")
                    .print(py);
            }
            Some(icons::FOLDER_GOOGLE_DRIVE)
        }
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        "school" => Some(icons::SCHOOL),
        "blinds" => Some(icons::BLINDS),
        _ => None,
    }
}
