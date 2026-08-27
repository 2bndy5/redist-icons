// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_15(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "timer-outline" => Some(icons::TIMER_OUTLINE),
        "video-wireless-outline" => Some(icons::VIDEO_WIRELESS_OUTLINE),
        "network-strength-1" => Some(icons::NETWORK_STRENGTH_1),
        "signal-cellular-1" => Some(icons::SIGNAL_CELLULAR_1),
        "landslide" => Some(icons::LANDSLIDE),
        "account-hard-hat-outline" => Some(icons::ACCOUNT_HARD_HAT_OUTLINE),
        "earth-plus" => Some(icons::EARTH_PLUS),
        "cloud-download" => Some(icons::CLOUD_DOWNLOAD),
        "shield-account-variant-outline" => Some(icons::SHIELD_ACCOUNT_VARIANT_OUTLINE),
        "head-check" => Some(icons::HEAD_CHECK),
        "upload-box-outline" => Some(icons::UPLOAD_BOX_OUTLINE),
        "tag-remove-outline" => Some(icons::TAG_REMOVE_OUTLINE),
        "sticker" => Some(icons::STICKER),
        #[allow(deprecated)]
        "box" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'box' is deprecated.").print(py);
            }
            Some(icons::BOX)
        }
        "basket-off-outline" => Some(icons::BASKET_OFF_OUTLINE),
        "arrow-top-right-bold-outline" => Some(icons::ARROW_TOP_RIGHT_BOLD_OUTLINE),
        "handcuffs" => Some(icons::HANDCUFFS),
        "marker" => Some(icons::MARKER),
        "download-off" => Some(icons::DOWNLOAD_OFF),
        "baseball-diamond-outline" => Some(icons::BASEBALL_DIAMOND_OUTLINE),
        "cart-arrow-down" => Some(icons::CART_ARROW_DOWN),
        "image-check-outline" => Some(icons::IMAGE_CHECK_OUTLINE),
        "plus" => Some(icons::PLUS),
        "tag-arrow-up-outline" => Some(icons::TAG_ARROW_UP_OUTLINE),
        #[allow(deprecated)]
        "tailwind" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'tailwind' is deprecated.").print(py);
            }
            Some(icons::TAILWIND)
        }
        "battery-low" => Some(icons::BATTERY_LOW),
        "candycane" => Some(icons::CANDYCANE),
        "alpha-q-box" => Some(icons::ALPHA_Q_BOX),
        "table-border" => Some(icons::TABLE_BORDER),
        #[allow(deprecated)]
        "webrtc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webrtc' is deprecated.").print(py);
            }
            Some(icons::WEBRTC)
        }
        "timeline-clock" => Some(icons::TIMELINE_CLOCK),
        #[allow(deprecated)]
        "microsoft-onenote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onenote' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONENOTE)
        }
        "folder-pound" => Some(icons::FOLDER_POUND),
        "account-arrow-right-outline" => Some(icons::ACCOUNT_ARROW_RIGHT_OUTLINE),
        "magnet-on" => Some(icons::MAGNET_ON),
        "heat-pump" => Some(icons::HEAT_PUMP),
        "axis-x-rotate-counterclockwise" => Some(icons::AXIS_X_ROTATE_COUNTERCLOCKWISE),
        "label-multiple" => Some(icons::LABEL_MULTIPLE),
        "sticker-alert-outline" => Some(icons::STICKER_ALERT_OUTLINE),
        "alpha-s-circle-outline" => Some(icons::ALPHA_S_CIRCLE_OUTLINE),
        "pen-remove" => Some(icons::PEN_REMOVE),
        "file-image-plus-outline" => Some(icons::FILE_IMAGE_PLUS_OUTLINE),
        "emoticon-frown-outline" => Some(icons::EMOTICON_FROWN_OUTLINE),
        "checkbook-arrow-left" => Some(icons::CHECKBOOK_ARROW_LEFT),
        "car-door-lock-open" => Some(icons::CAR_DOOR_LOCK_OPEN),
        "pill" => Some(icons::PILL),
        "border-none-variant" => Some(icons::BORDER_NONE_VARIANT),
        "door-sliding-open" => Some(icons::DOOR_SLIDING_OPEN),
        "account-cash" => Some(icons::ACCOUNT_CASH),
        "video-input-scart" => Some(icons::VIDEO_INPUT_SCART),
        "pig-variant" => Some(icons::PIG_VARIANT),
        "file-excel-box" => Some(icons::FILE_EXCEL_BOX),
        "account-eye-outline" => Some(icons::ACCOUNT_EYE_OUTLINE),
        "food-apple" => Some(icons::FOOD_APPLE),
        "view-list-outline" => Some(icons::VIEW_LIST_OUTLINE),
        "trophy-broken" => Some(icons::TROPHY_BROKEN),
        "source-branch-refresh" => Some(icons::SOURCE_BRANCH_REFRESH),
        "eye-check" => Some(icons::EYE_CHECK),
        "robot-vacuum-variant-alert" => Some(icons::ROBOT_VACUUM_VARIANT_ALERT),
        "elevator" => Some(icons::ELEVATOR),
        "information-outline" => Some(icons::INFORMATION_OUTLINE),
        "home-percent-outline" => Some(icons::HOME_PERCENT_OUTLINE),
        "phone-minus" => Some(icons::PHONE_MINUS),
        "spa-outline" => Some(icons::SPA_OUTLINE),
        "alarm-note" => Some(icons::ALARM_NOTE),
        "chart-histogram" => Some(icons::CHART_HISTOGRAM),
        "egg-off" => Some(icons::EGG_OFF),
        "link-variant-plus" => Some(icons::LINK_VARIANT_PLUS),
        "rewind-30" => Some(icons::REWIND_30),
        "battery-plus" => Some(icons::BATTERY_PLUS),
        "lightbulb-alert" => Some(icons::LIGHTBULB_ALERT),
        "gate-xor" => Some(icons::GATE_XOR),
        "lamps-outline" => Some(icons::LAMPS_OUTLINE),
        "car" => Some(icons::CAR),
        "car-parking-lights" => Some(icons::CAR_PARKING_LIGHTS),
        #[allow(deprecated)]
        "laravel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'laravel' is deprecated.").print(py);
            }
            Some(icons::LARAVEL)
        }
        "source-commit" => Some(icons::SOURCE_COMMIT),
        "fireplace" => Some(icons::FIREPLACE),
        "umbrella-beach-outline" => Some(icons::UMBRELLA_BEACH_OUTLINE),
        "car-lifted-pickup" => Some(icons::CAR_LIFTED_PICKUP),
        "format-quote-close-outline" => Some(icons::FORMAT_QUOTE_CLOSE_OUTLINE),
        "format-indent-decrease" => Some(icons::FORMAT_INDENT_DECREASE),
        "calendar-check" => Some(icons::CALENDAR_CHECK),
        "compare-remove" => Some(icons::COMPARE_REMOVE),
        "basket-check" => Some(icons::BASKET_CHECK),
        "hexagon-slice-5" => Some(icons::HEXAGON_SLICE_5),
        "swap-vertical-circle-outline" => Some(icons::SWAP_VERTICAL_CIRCLE_OUTLINE),
        "movie-play-outline" => Some(icons::MOVIE_PLAY_OUTLINE),
        "cup-water" => Some(icons::CUP_WATER),
        "battery" => Some(icons::BATTERY),
        "message-bulleted" => Some(icons::MESSAGE_BULLETED),
        "reload" => Some(icons::RELOAD),
        "home-group-remove" => Some(icons::HOME_GROUP_REMOVE),
        "home-analytics" => Some(icons::HOME_ANALYTICS),
        "cast-connected" => Some(icons::CAST_CONNECTED),
        "heart-half" => Some(icons::HEART_HALF),
        "timer-lock-open" => Some(icons::TIMER_LOCK_OPEN),
        "car-off" => Some(icons::CAR_OFF),
        "headphones-box" => Some(icons::HEADPHONES_BOX),
        "tag-arrow-up" => Some(icons::TAG_ARROW_UP),
        "comma-box" => Some(icons::COMMA_BOX),
        "brightness-1" => Some(icons::BRIGHTNESS_1),
        "gesture-tap-button" => Some(icons::GESTURE_TAP_BUTTON),
        "sticker-text-outline" => Some(icons::STICKER_TEXT_OUTLINE),
        "map-marker-plus-outline" => Some(icons::MAP_MARKER_PLUS_OUTLINE),
        "water-check" => Some(icons::WATER_CHECK),
        "car-brake-fluid-level" => Some(icons::CAR_BRAKE_FLUID_LEVEL),
        "text-box-search" => Some(icons::TEXT_BOX_SEARCH),
        "metronome" => Some(icons::METRONOME),
        "flag-variant-plus-outline" => Some(icons::FLAG_VARIANT_PLUS_OUTLINE),
        "star-plus" => Some(icons::STAR_PLUS),
        "file-image-marker" => Some(icons::FILE_IMAGE_MARKER),
        "tooltip-check-outline" => Some(icons::TOOLTIP_CHECK_OUTLINE),
        "stretch-to-page-outline" => Some(icons::STRETCH_TO_PAGE_OUTLINE),
        "circle-slice-7" => Some(icons::CIRCLE_SLICE_7),
        "post-outline" => Some(icons::POST_OUTLINE),
        "account-child" => Some(icons::ACCOUNT_CHILD),
        "printer-check" => Some(icons::PRINTER_CHECK),
        "eye-arrow-left-outline" => Some(icons::EYE_ARROW_LEFT_OUTLINE),
        "archive-music" => Some(icons::ARCHIVE_MUSIC),
        "phone-lock" => Some(icons::PHONE_LOCK),
        "flash-triangle" => Some(icons::FLASH_TRIANGLE),
        #[allow(deprecated)]
        "gitlab" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gitlab' is deprecated.").print(py);
            }
            Some(icons::GITLAB)
        }
        "format-line-spacing" => Some(icons::FORMAT_LINE_SPACING),
        "subway-variant" => Some(icons::SUBWAY_VARIANT),
        "dharmachakra" => Some(icons::DHARMACHAKRA),
        "folder-arrow-left-right-outline" => Some(icons::FOLDER_ARROW_LEFT_RIGHT_OUTLINE),
        "dolphin" => Some(icons::DOLPHIN),
        "chat-alert-outline" => Some(icons::CHAT_ALERT_OUTLINE),
        "clipboard-text-multiple-outline" => Some(icons::CLIPBOARD_TEXT_MULTIPLE_OUTLINE),
        "awning" => Some(icons::AWNING),
        "wifi-strength-2" => Some(icons::WIFI_STRENGTH_2),
        "eye-settings" => Some(icons::EYE_SETTINGS),
        "sticker-minus-outline" => Some(icons::STICKER_MINUS_OUTLINE),
        "folder-plus-outline" => Some(icons::FOLDER_PLUS_OUTLINE),
        "monitor-vertical" => Some(icons::MONITOR_VERTICAL),
        "alpha-s-box" => Some(icons::ALPHA_S_BOX),
        "chevron-left-box" => Some(icons::CHEVRON_LEFT_BOX),
        "highway" => Some(icons::HIGHWAY),
        "boom-gate-arrow-up-outline" => Some(icons::BOOM_GATE_ARROW_UP_OUTLINE),
        "bicycle-basket" => Some(icons::BICYCLE_BASKET),
        "mailbox-up-outline" => Some(icons::MAILBOX_UP_OUTLINE),
        "map-marker-plus" => Some(icons::MAP_MARKER_PLUS),
        "star-box-multiple-outline" => Some(icons::STAR_BOX_MULTIPLE_OUTLINE),
        "calendar-expand-horizontal-outline" => Some(icons::CALENDAR_EXPAND_HORIZONTAL_OUTLINE),
        "cookie-off" => Some(icons::COOKIE_OFF),
        "briefcase-upload-outline" => Some(icons::BRIEFCASE_UPLOAD_OUTLINE),
        "social-distance-2-meters" => Some(icons::SOCIAL_DISTANCE_2_METERS),
        "credit-card-lock" => Some(icons::CREDIT_CARD_LOCK),
        "pot" => Some(icons::POT),
        "dice-d10-outline" => Some(icons::DICE_D10_OUTLINE),
        "restore-alert" => Some(icons::RESTORE_ALERT),
        "image-lock-outline" => Some(icons::IMAGE_LOCK_OUTLINE),
        "usb-c-port" => Some(icons::USB_C_PORT),
        "content-save-cog-outline" => Some(icons::CONTENT_SAVE_COG_OUTLINE),
        "human-greeting-proximity" => Some(icons::HUMAN_GREETING_PROXIMITY),
        "magic-staff" => Some(icons::MAGIC_STAFF),
        "spotlight-beam" => Some(icons::SPOTLIGHT_BEAM),
        "nail" => Some(icons::NAIL),
        "format-list-numbered-rtl" => Some(icons::FORMAT_LIST_NUMBERED_RTL),
        "film" => Some(icons::FILM),
        "clipboard-arrow-left" => Some(icons::CLIPBOARD_ARROW_LEFT),
        "relation-one-to-zero-or-many" => Some(icons::RELATION_ONE_TO_ZERO_OR_MANY),
        "map-marker-outline" => Some(icons::MAP_MARKER_OUTLINE),
        "passport-check" => Some(icons::PASSPORT_CHECK),
        "battery-80" => Some(icons::BATTERY_80),
        "temperature-celsius" => Some(icons::TEMPERATURE_CELSIUS),
        "calculator-variant" => Some(icons::CALCULATOR_VARIANT),
        "phone-cancel" => Some(icons::PHONE_CANCEL),
        "text-account" => Some(icons::TEXT_ACCOUNT),
        "postage-stamp" => Some(icons::POSTAGE_STAMP),
        "database-outline" => Some(icons::DATABASE_OUTLINE),
        "roman-numeral-2" => Some(icons::ROMAN_NUMERAL_2),
        "land-plots-marker" => Some(icons::LAND_PLOTS_MARKER),
        "percent-circle" => Some(icons::PERCENT_CIRCLE),
        "shield-plus" => Some(icons::SHIELD_PLUS),
        "airplane-clock" => Some(icons::AIRPLANE_CLOCK),
        "axe" => Some(icons::AXE),
        "wifi-strength-4" => Some(icons::WIFI_STRENGTH_4),
        "cart-variant" => Some(icons::CART_VARIANT),
        "radiology-box-outline" => Some(icons::RADIOLOGY_BOX_OUTLINE),
        "printer-3d-nozzle-heat-outline" => Some(icons::PRINTER_3D_NOZZLE_HEAT_OUTLINE),
        "billiards" => Some(icons::BILLIARDS),
        "sprout-outline" => Some(icons::SPROUT_OUTLINE),
        "arrow-up-bold" => Some(icons::ARROW_UP_BOLD),
        "comment-text-multiple-outline" => Some(icons::COMMENT_TEXT_MULTIPLE_OUTLINE),
        "pan-up" => Some(icons::PAN_UP),
        "fast-forward" => Some(icons::FAST_FORWARD),
        "scatter-plot-outline" => Some(icons::SCATTER_PLOT_OUTLINE),
        "nfc-tap" => Some(icons::NFC_TAP),
        "file-document-arrow-right" => Some(icons::FILE_DOCUMENT_ARROW_RIGHT),
        "paperclip-check" => Some(icons::PAPERCLIP_CHECK),
        "robot-love-outline" => Some(icons::ROBOT_LOVE_OUTLINE),
        "wifi-arrow-down" => Some(icons::WIFI_ARROW_DOWN),
        "clipboard-account" => Some(icons::CLIPBOARD_ACCOUNT),
        "road-variant" => Some(icons::ROAD_VARIANT),
        "table-plus" => Some(icons::TABLE_PLUS),
        "dog-service" => Some(icons::DOG_SERVICE),
        "account-off-outline" => Some(icons::ACCOUNT_OFF_OUTLINE),
        "gamepad-round-left" => Some(icons::GAMEPAD_ROUND_LEFT),
        _ => None,
    }
}
