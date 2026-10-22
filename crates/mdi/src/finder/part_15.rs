// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_15(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "cash-100" => Some(icons::CASH_100),
        "clock-time-three-outline" => Some(icons::CLOCK_TIME_THREE_OUTLINE),
        "glass-mug-variant" => Some(icons::GLASS_MUG_VARIANT),
        "bank-transfer" => Some(icons::BANK_TRANSFER),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-low" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-low' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_LOW)
        }
        "align-vertical-distribute" => Some(icons::ALIGN_VERTICAL_DISTRIBUTE),
        "dice-4" => Some(icons::DICE_4),
        "calendar-heart" => Some(icons::CALENDAR_HEART),
        "map-marker-plus-outline" => Some(icons::MAP_MARKER_PLUS_OUTLINE),
        "door-open" => Some(icons::DOOR_OPEN),
        "battery-minus-outline" => Some(icons::BATTERY_MINUS_OUTLINE),
        "book-cross" => Some(icons::BOOK_CROSS),
        "camera-flip" => Some(icons::CAMERA_FLIP),
        "mouse-variant" => Some(icons::MOUSE_VARIANT),
        "home-group-remove" => Some(icons::HOME_GROUP_REMOVE),
        "lightbulb-on-20" => Some(icons::LIGHTBULB_ON_20),
        "checkbook-arrow-right" => Some(icons::CHECKBOOK_ARROW_RIGHT),
        "debug-step-over" => Some(icons::DEBUG_STEP_OVER),
        #[allow(deprecated)]
        "netflix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'netflix' is deprecated.").print(py);
            }
            Some(icons::NETFLIX)
        }
        "cloud-download" => Some(icons::CLOUD_DOWNLOAD),
        "flash-alert-outline" => Some(icons::FLASH_ALERT_OUTLINE),
        "microphone-plus" => Some(icons::MICROPHONE_PLUS),
        "yurt" => Some(icons::YURT),
        "briefcase-minus-outline" => Some(icons::BRIEFCASE_MINUS_OUTLINE),
        "gamepad-square" => Some(icons::GAMEPAD_SQUARE),
        "cricket" => Some(icons::CRICKET),
        "charity" => Some(icons::CHARITY),
        "network-strength-2" => Some(icons::NETWORK_STRENGTH_2),
        "arrow-left-top" => Some(icons::ARROW_LEFT_TOP),
        "seat-legroom-reduced" => Some(icons::SEAT_LEGROOM_REDUCED),
        "account-lock-open" => Some(icons::ACCOUNT_LOCK_OPEN),
        "select-search" => Some(icons::SELECT_SEARCH),
        "audio-video-off" => Some(icons::AUDIO_VIDEO_OFF),
        "temple-hindu-outline" => Some(icons::TEMPLE_HINDU_OUTLINE),
        "arrow-up-bold-hexagon-outline" => Some(icons::ARROW_UP_BOLD_HEXAGON_OUTLINE),
        "account-reactivate" => Some(icons::ACCOUNT_REACTIVATE),
        "account-check-outline" => Some(icons::ACCOUNT_CHECK_OUTLINE),
        "picture-in-picture-top-right" => Some(icons::PICTURE_IN_PICTURE_TOP_RIGHT),
        "archive-off-outline" => Some(icons::ARCHIVE_OFF_OUTLINE),
        "numeric-2-box-outline" => Some(icons::NUMERIC_2_BOX_OUTLINE),
        "glass-pint-outline" => Some(icons::GLASS_PINT_OUTLINE),
        "emoticon-remove-outline" => Some(icons::EMOTICON_REMOVE_OUTLINE),
        "invoice-list-outline" => Some(icons::INVOICE_LIST_OUTLINE),
        "silverware-fork" => Some(icons::SILVERWARE_FORK),
        "hamburger-plus" => Some(icons::HAMBURGER_PLUS),
        "phone-ring-outline" => Some(icons::PHONE_RING_OUTLINE),
        "human-dolly" => Some(icons::HUMAN_DOLLY),
        "calendar-blank-outline" => Some(icons::CALENDAR_BLANK_OUTLINE),
        "moon-waxing-gibbous" => Some(icons::MOON_WAXING_GIBBOUS),
        "kangaroo" => Some(icons::KANGAROO),
        "stairs-down" => Some(icons::STAIRS_DOWN),
        "egg-fried" => Some(icons::EGG_FRIED),
        "candy-off" => Some(icons::CANDY_OFF),
        "printer-pos-network" => Some(icons::PRINTER_POS_NETWORK),
        "relation-only-one-to-only-one" => Some(icons::RELATION_ONLY_ONE_TO_ONLY_ONE),
        "math-sin" => Some(icons::MATH_SIN),
        "dice-d10" => Some(icons::DICE_D10),
        "calendar-blank" => Some(icons::CALENDAR_BLANK),
        "flag-variant-remove" => Some(icons::FLAG_VARIANT_REMOVE),
        "cloud-clock-outline" => Some(icons::CLOUD_CLOCK_OUTLINE),
        "briefcase-eye-outline" => Some(icons::BRIEFCASE_EYE_OUTLINE),
        "arrow-expand-all" => Some(icons::ARROW_EXPAND_ALL),
        "human-female-female-child" => Some(icons::HUMAN_FEMALE_FEMALE_CHILD),
        "drag-horizontal" => Some(icons::DRAG_HORIZONTAL),
        "domino-mask" => Some(icons::DOMINO_MASK),
        "slope-downhill" => Some(icons::SLOPE_DOWNHILL),
        "flag-variant-minus-outline" => Some(icons::FLAG_VARIANT_MINUS_OUTLINE),
        "washing-machine-off" => Some(icons::WASHING_MACHINE_OFF),
        "water-sync" => Some(icons::WATER_SYNC),
        "basket-check-outline" => Some(icons::BASKET_CHECK_OUTLINE),
        "earth-box" => Some(icons::EARTH_BOX),
        "briefcase" => Some(icons::BRIEFCASE),
        "plus-box-multiple" => Some(icons::PLUS_BOX_MULTIPLE),
        "archive-search" => Some(icons::ARCHIVE_SEARCH),
        "crystal-ball" => Some(icons::CRYSTAL_BALL),
        "smoking-pipe-off" => Some(icons::SMOKING_PIPE_OFF),
        "download-network" => Some(icons::DOWNLOAD_NETWORK),
        "play-box-lock-outline" => Some(icons::PLAY_BOX_LOCK_OUTLINE),
        "less-than" => Some(icons::LESS_THAN),
        "dolphin" => Some(icons::DOLPHIN),
        "zodiac-cancer" => Some(icons::ZODIAC_CANCER),
        "application-cog-outline" => Some(icons::APPLICATION_COG_OUTLINE),
        "email-arrow-left-outline" => Some(icons::EMAIL_ARROW_LEFT_OUTLINE),
        "star-half-full" => Some(icons::STAR_HALF_FULL),
        "layers-outline" => Some(icons::LAYERS_OUTLINE),
        "table-row-remove" => Some(icons::TABLE_ROW_REMOVE),
        "door-sliding" => Some(icons::DOOR_SLIDING),
        "numeric-1-box-outline" => Some(icons::NUMERIC_1_BOX_OUTLINE),
        "weather-rainy" => Some(icons::WEATHER_RAINY),
        "battery-remove" => Some(icons::BATTERY_REMOVE),
        "home-off" => Some(icons::HOME_OFF),
        "wifi-strength-2-lock-open" => Some(icons::WIFI_STRENGTH_2_LOCK_OPEN),
        "arrow-bottom-right-thin-circle-outline" => {
            Some(icons::ARROW_BOTTOM_RIGHT_THIN_CIRCLE_OUTLINE)
        }
        "book-lock" => Some(icons::BOOK_LOCK),
        "more" => Some(icons::MORE),
        "pen-remove" => Some(icons::PEN_REMOVE),
        "weather-lightning-rainy" => Some(icons::WEATHER_LIGHTNING_RAINY),
        "filter-plus" => Some(icons::FILTER_PLUS),
        #[allow(deprecated)]
        "arch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'arch' is deprecated.").print(py);
            }
            Some(icons::ARCH)
        }
        "folder-multiple-outline" => Some(icons::FOLDER_MULTIPLE_OUTLINE),
        "eyedropper-variant" => Some(icons::EYEDROPPER_VARIANT),
        "clipboard-arrow-right-outline" => Some(icons::CLIPBOARD_ARROW_RIGHT_OUTLINE),
        "alpha-v-box-outline" => Some(icons::ALPHA_V_BOX_OUTLINE),
        #[allow(deprecated)]
        "umbraco" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'umbraco' is deprecated.").print(py);
            }
            Some(icons::UMBRACO)
        }
        "application-braces" => Some(icons::APPLICATION_BRACES),
        "star-four-points-circle" => Some(icons::STAR_FOUR_POINTS_CIRCLE),
        "cloud-print" => Some(icons::CLOUD_PRINT),
        "music-rest-eighth" => Some(icons::MUSIC_REST_EIGHTH),
        "home-group-plus" => Some(icons::HOME_GROUP_PLUS),
        #[allow(deprecated)]
        "facebook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook' is deprecated.").print(py);
            }
            Some(icons::FACEBOOK)
        }
        "table-edit" => Some(icons::TABLE_EDIT),
        "battery-charging-high" => Some(icons::BATTERY_CHARGING_HIGH),
        "dots-triangle" => Some(icons::DOTS_TRIANGLE),
        "step-backward-2" => Some(icons::STEP_BACKWARD_2),
        "fire-truck" => Some(icons::FIRE_TRUCK),
        "camera-lock-open-outline" => Some(icons::CAMERA_LOCK_OPEN_OUTLINE),
        "weather-pouring" => Some(icons::WEATHER_POURING),
        "phone-minus" => Some(icons::PHONE_MINUS),
        "comment" => Some(icons::COMMENT),
        "credit-card-wireless" => Some(icons::CREDIT_CARD_WIRELESS),
        "sync-circle" => Some(icons::SYNC_CIRCLE),
        "picture-in-picture-bottom-right" => Some(icons::PICTURE_IN_PICTURE_BOTTOM_RIGHT),
        "weather-partly-lightning" => Some(icons::WEATHER_PARTLY_LIGHTNING),
        "movie-open-settings-outline" => Some(icons::MOVIE_OPEN_SETTINGS_OUTLINE),
        "clipboard-arrow-up" => Some(icons::CLIPBOARD_ARROW_UP),
        "thermometer-off" => Some(icons::THERMOMETER_OFF),
        "food-fork-drink" => Some(icons::FOOD_FORK_DRINK),
        "bookmark-minus-outline" => Some(icons::BOOKMARK_MINUS_OUTLINE),
        "arrow-down" => Some(icons::ARROW_DOWN),
        "arrow-bottom-left-bold-box-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_BOX_OUTLINE),
        "blender-outline" => Some(icons::BLENDER_OUTLINE),
        "invoice-text-remove" => Some(icons::INVOICE_TEXT_REMOVE),
        "garage-variant-lock" => Some(icons::GARAGE_VARIANT_LOCK),
        "cookie-minus-outline" => Some(icons::COOKIE_MINUS_OUTLINE),
        "dance-ballroom" => Some(icons::DANCE_BALLROOM),
        "send-clock" => Some(icons::SEND_CLOCK),
        "file-document-check" => Some(icons::FILE_DOCUMENT_CHECK),
        "invoice-import-outline" => Some(icons::INVOICE_IMPORT_OUTLINE),
        "clipboard-search" => Some(icons::CLIPBOARD_SEARCH),
        "star-minus" => Some(icons::STAR_MINUS),
        "copyright" => Some(icons::COPYRIGHT),
        "dumbbell" => Some(icons::DUMBBELL),
        "sign-pole" => Some(icons::SIGN_POLE),
        "blur-off" => Some(icons::BLUR_OFF),
        "camera-plus" => Some(icons::CAMERA_PLUS),
        "email-sync" => Some(icons::EMAIL_SYNC),
        "axe-battle" => Some(icons::AXE_BATTLE),
        #[allow(deprecated)]
        "ubisoft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ubisoft' is deprecated.").print(py);
            }
            Some(icons::UBISOFT)
        }
        "circle-small" => Some(icons::CIRCLE_SMALL),
        "reload-alert" => Some(icons::RELOAD_ALERT),
        "delete-variant" => Some(icons::DELETE_VARIANT),
        "alpha-f" => Some(icons::ALPHA_F),
        "text-box-plus-outline" => Some(icons::TEXT_BOX_PLUS_OUTLINE),
        "vector-point" => Some(icons::VECTOR_POINT),
        "bag-carry-on-off" => Some(icons::BAG_CARRY_ON_OFF),
        "credit-card-lock" => Some(icons::CREDIT_CARD_LOCK),
        "awning" => Some(icons::AWNING),
        "summit" => Some(icons::SUMMIT),
        "format-columns" => Some(icons::FORMAT_COLUMNS),
        "fan-chevron-down" => Some(icons::FAN_CHEVRON_DOWN),
        "blinds-open" => Some(icons::BLINDS_OPEN),
        "checkbox-marked-outline" => Some(icons::CHECKBOX_MARKED_OUTLINE),
        "reflect-vertical" => Some(icons::REFLECT_VERTICAL),
        "temperature-celsius" => Some(icons::TEMPERATURE_CELSIUS),
        "wardrobe-outline" => Some(icons::WARDROBE_OUTLINE),
        "chess-rook" => Some(icons::CHESS_ROOK),
        "thumbs-up-down-outline" => Some(icons::THUMBS_UP_DOWN_OUTLINE),
        "video-marker" => Some(icons::VIDEO_MARKER),
        "door-closed" => Some(icons::DOOR_CLOSED),
        "lock-pattern" => Some(icons::LOCK_PATTERN),
        "multiplication-box" => Some(icons::MULTIPLICATION_BOX),
        "code-json" => Some(icons::CODE_JSON),
        "code-block-parentheses" => Some(icons::CODE_BLOCK_PARENTHESES),
        "radioactive" => Some(icons::RADIOACTIVE),
        "alphabet-greek" => Some(icons::ALPHABET_GREEK),
        "nfc-search-variant" => Some(icons::NFC_SEARCH_VARIANT),
        "circle-slice-1" => Some(icons::CIRCLE_SLICE_1),
        "compass-off" => Some(icons::COMPASS_OFF),
        "distribute-vertical-bottom" => Some(icons::DISTRIBUTE_VERTICAL_BOTTOM),
        "store-remove" => Some(icons::STORE_REMOVE),
        #[allow(deprecated)]
        "apple-ios" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-ios' is deprecated.").print(py);
            }
            Some(icons::APPLE_IOS)
        }
        "wave-arrow-up" => Some(icons::WAVE_ARROW_UP),
        "mailbox-open-up" => Some(icons::MAILBOX_OPEN_UP),
        "arrow-right-bottom" => Some(icons::ARROW_RIGHT_BOTTOM),
        "emoticon-frown-outline" => Some(icons::EMOTICON_FROWN_OUTLINE),
        "key-wireless" => Some(icons::KEY_WIRELESS),
        "dog-service" => Some(icons::DOG_SERVICE),
        "image-filter-center-focus-strong-outline" => {
            Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG_OUTLINE)
        }
        "timer-alert-outline" => Some(icons::TIMER_ALERT_OUTLINE),
        "fingerprint-off" => Some(icons::FINGERPRINT_OFF),
        "file-rotate-right" => Some(icons::FILE_ROTATE_RIGHT),
        "teddy-bear" => Some(icons::TEDDY_BEAR),
        "comment-outline" => Some(icons::COMMENT_OUTLINE),
        "alpha-o-circle" => Some(icons::ALPHA_O_CIRCLE),
        "view-dashboard-edit-outline" => Some(icons::VIEW_DASHBOARD_EDIT_OUTLINE),
        "application-array" => Some(icons::APPLICATION_ARRAY),
        #[allow(deprecated)]
        "google" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google' is deprecated.").print(py);
            }
            Some(icons::GOOGLE)
        }
        "beaker-check" => Some(icons::BEAKER_CHECK),
        "car-brake-alert" => Some(icons::CAR_BRAKE_ALERT),
        "file-refresh-outline" => Some(icons::FILE_REFRESH_OUTLINE),
        _ => None,
    }
}
