// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_13(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "account-star-outline" => Some(icons::ACCOUNT_STAR_OUTLINE),
        "link-lock" => Some(icons::LINK_LOCK),
        "shape-circle-plus" => Some(icons::SHAPE_CIRCLE_PLUS),
        #[allow(deprecated)]
        "pandora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pandora' is deprecated.").print(py);
            }
            Some(icons::PANDORA)
        }
        "message-lock-outline" => Some(icons::MESSAGE_LOCK_OUTLINE),
        "cake-variant" => Some(icons::CAKE_VARIANT),
        "led-on" => Some(icons::LED_ON),
        "image-size-select-actual" => Some(icons::IMAGE_SIZE_SELECT_ACTUAL),
        "view-gallery" => Some(icons::VIEW_GALLERY),
        "network-strength-1-alert" => Some(icons::NETWORK_STRENGTH_1_ALERT),
        "fridge-variant" => Some(icons::FRIDGE_VARIANT),
        "rotate-3d" => Some(icons::ROTATE_3D),
        "thermometer-off" => Some(icons::THERMOMETER_OFF),
        "medication" => Some(icons::MEDICATION),
        "distribute-horizontal-right" => Some(icons::DISTRIBUTE_HORIZONTAL_RIGHT),
        "file-phone-outline" => Some(icons::FILE_PHONE_OUTLINE),
        "folder-settings" => Some(icons::FOLDER_SETTINGS),
        "weather-snowy-heavy" => Some(icons::WEATHER_SNOWY_HEAVY),
        "briefcase-variant-off" => Some(icons::BRIEFCASE_VARIANT_OFF),
        "skip-previous-outline" => Some(icons::SKIP_PREVIOUS_OUTLINE),
        "hexagon-slice-6" => Some(icons::HEXAGON_SLICE_6),
        "share-all" => Some(icons::SHARE_ALL),
        "folder-text" => Some(icons::FOLDER_TEXT),
        "heart-minus-outline" => Some(icons::HEART_MINUS_OUTLINE),
        "kettlebell" => Some(icons::KETTLEBELL),
        #[allow(deprecated)]
        "language-swift" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-swift' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_SWIFT)
        }
        "plus-minus" => Some(icons::PLUS_MINUS),
        "printer-3d-nozzle-heat" => Some(icons::PRINTER_3D_NOZZLE_HEAT),
        "nature-people-outline" => Some(icons::NATURE_PEOPLE_OUTLINE),
        "polaroid" => Some(icons::POLAROID),
        "alpha-y-circle-outline" => Some(icons::ALPHA_Y_CIRCLE_OUTLINE),
        "egg-outline" => Some(icons::EGG_OUTLINE),
        "tag-check-outline" => Some(icons::TAG_CHECK_OUTLINE),
        "stop-circle" => Some(icons::STOP_CIRCLE),
        "wind-power-outline" => Some(icons::WIND_POWER_OUTLINE),
        "fire-hydrant-off" => Some(icons::FIRE_HYDRANT_OFF),
        "replay" => Some(icons::REPLAY),
        "stairs-up" => Some(icons::STAIRS_UP),
        "image-remove" => Some(icons::IMAGE_REMOVE),
        "alpha-d-circle-outline" => Some(icons::ALPHA_D_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "jsfiddle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jsfiddle' is deprecated.").print(py);
            }
            Some(icons::JSFIDDLE)
        }
        "gate-arrow-left" => Some(icons::GATE_ARROW_LEFT),
        "close-circle-outline" => Some(icons::CLOSE_CIRCLE_OUTLINE),
        "robot-vacuum-variant-alert" => Some(icons::ROBOT_VACUUM_VARIANT_ALERT),
        "toolbox-outline" => Some(icons::TOOLBOX_OUTLINE),
        "desk-lamp-off" => Some(icons::DESK_LAMP_OFF),
        "send" => Some(icons::SEND),
        "find-replace" => Some(icons::FIND_REPLACE),
        "message-processing-outline" => Some(icons::MESSAGE_PROCESSING_OUTLINE),
        "car-child-seat" => Some(icons::CAR_CHILD_SEAT),
        "animation-outline" => Some(icons::ANIMATION_OUTLINE),
        "null" => Some(icons::NULL),
        "head-heart-outline" => Some(icons::HEAD_HEART_OUTLINE),
        "car-settings" => Some(icons::CAR_SETTINGS),
        "smog" => Some(icons::SMOG),
        "file-marker" => Some(icons::FILE_MARKER),
        "wrench" => Some(icons::WRENCH),
        "account-outline" => Some(icons::ACCOUNT_OUTLINE),
        "download-lock-outline" => Some(icons::DOWNLOAD_LOCK_OUTLINE),
        "email-box" => Some(icons::EMAIL_BOX),
        "clock-time-one" => Some(icons::CLOCK_TIME_ONE),
        "bag-personal-plus-outline" => Some(icons::BAG_PERSONAL_PLUS_OUTLINE),
        "corn-off" => Some(icons::CORN_OFF),
        "blinds-horizontal-closed" => Some(icons::BLINDS_HORIZONTAL_CLOSED),
        "delete-outline" => Some(icons::DELETE_OUTLINE),
        "checkbox-marked-outline" => Some(icons::CHECKBOX_MARKED_OUTLINE),
        "square-opacity" => Some(icons::SQUARE_OPACITY),
        "forum-remove" => Some(icons::FORUM_REMOVE),
        "office-building-minus" => Some(icons::OFFICE_BUILDING_MINUS),
        "radioactive" => Some(icons::RADIOACTIVE),
        "sort-numeric-descending-variant" => Some(icons::SORT_NUMERIC_DESCENDING_VARIANT),
        "text-box-remove" => Some(icons::TEXT_BOX_REMOVE),
        "message-video" => Some(icons::MESSAGE_VIDEO),
        "emoticon-tongue-outline" => Some(icons::EMOTICON_TONGUE_OUTLINE),
        "volcano-outline" => Some(icons::VOLCANO_OUTLINE),
        "billiards-rack" => Some(icons::BILLIARDS_RACK),
        "equalizer" => Some(icons::EQUALIZER),
        "contrast" => Some(icons::CONTRAST),
        "decimal-comma-decrease" => Some(icons::DECIMAL_COMMA_DECREASE),
        "screw-machine-round-top" => Some(icons::SCREW_MACHINE_ROUND_TOP),
        "format-quote-close-outline" => Some(icons::FORMAT_QUOTE_CLOSE_OUTLINE),
        "archive-eye" => Some(icons::ARCHIVE_EYE),
        "card-minus-outline" => Some(icons::CARD_MINUS_OUTLINE),
        "chevron-down-circle-outline" => Some(icons::CHEVRON_DOWN_CIRCLE_OUTLINE),
        "folder-refresh-outline" => Some(icons::FOLDER_REFRESH_OUTLINE),
        "arrow-right-thin" => Some(icons::ARROW_RIGHT_THIN),
        "traffic-light-outline" => Some(icons::TRAFFIC_LIGHT_OUTLINE),
        "battery-minus-variant" => Some(icons::BATTERY_MINUS_VARIANT),
        "focus-field-horizontal" => Some(icons::FOCUS_FIELD_HORIZONTAL),
        "tag-multiple-outline" => Some(icons::TAG_MULTIPLE_OUTLINE),
        "mouse-outline" => Some(icons::MOUSE_OUTLINE),
        "chili-off" => Some(icons::CHILI_OFF),
        #[allow(deprecated)]
        "stack-exchange" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-exchange' is deprecated.").print(py);
            }
            Some(icons::STACK_EXCHANGE)
        }
        "marker" => Some(icons::MARKER),
        "message-reply-text-outline" => Some(icons::MESSAGE_REPLY_TEXT_OUTLINE),
        "numeric-2-circle-outline" => Some(icons::NUMERIC_2_CIRCLE_OUTLINE),
        "baguette" => Some(icons::BAGUETTE),
        "decagram" => Some(icons::DECAGRAM),
        "train-car-gondola-full" => Some(icons::TRAIN_CAR_GONDOLA_FULL),
        "map-minus" => Some(icons::MAP_MINUS),
        "timer-edit-outline" => Some(icons::TIMER_EDIT_OUTLINE),
        "currency-usd-off" => Some(icons::CURRENCY_USD_OFF),
        "star-settings-outline" => Some(icons::STAR_SETTINGS_OUTLINE),
        "format-quote-close" => Some(icons::FORMAT_QUOTE_CLOSE),
        "home-silo-outline" => Some(icons::HOME_SILO_OUTLINE),
        "tray-full" => Some(icons::TRAY_FULL),
        "arrow-bottom-right-bold-box" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX),
        "ghost-off" => Some(icons::GHOST_OFF),
        "music-accidental-natural" => Some(icons::MUSIC_ACCIDENTAL_NATURAL),
        "robot-vacuum-variant" => Some(icons::ROBOT_VACUUM_VARIANT),
        "clipboard-flow" => Some(icons::CLIPBOARD_FLOW),
        "page-layout-footer" => Some(icons::PAGE_LAYOUT_FOOTER),
        "arrow-left-bold-hexagon-outline" => Some(icons::ARROW_LEFT_BOLD_HEXAGON_OUTLINE),
        "basketball-hoop-outline" => Some(icons::BASKETBALL_HOOP_OUTLINE),
        "grain" => Some(icons::GRAIN),
        #[allow(deprecated)]
        "google-lens" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-lens' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_LENS)
        }
        "pail-outline" => Some(icons::PAIL_OUTLINE),
        "chat-question" => Some(icons::CHAT_QUESTION),
        "content-save-edit" => Some(icons::CONTENT_SAVE_EDIT),
        "bell-cancel" => Some(icons::BELL_CANCEL),
        "gamepad-round-outline" => Some(icons::GAMEPAD_ROUND_OUTLINE),
        "wallet-giftcard" => Some(icons::WALLET_GIFTCARD),
        "smoking-pipe-off" => Some(icons::SMOKING_PIPE_OFF),
        "database-minus-outline" => Some(icons::DATABASE_MINUS_OUTLINE),
        "arrow-u-left-top" => Some(icons::ARROW_U_LEFT_TOP),
        "power-on" => Some(icons::POWER_ON),
        "card-bulleted-off" => Some(icons::CARD_BULLETED_OFF),
        "office-building-minus-outline" => Some(icons::OFFICE_BUILDING_MINUS_OUTLINE),
        "archive-cog-outline" => Some(icons::ARCHIVE_COG_OUTLINE),
        "bottle-tonic-outline" => Some(icons::BOTTLE_TONIC_OUTLINE),
        "arrow-u-right-bottom-bold" => Some(icons::ARROW_U_RIGHT_BOTTOM_BOLD),
        "database-edit-outline" => Some(icons::DATABASE_EDIT_OUTLINE),
        "smart-card-outline" => Some(icons::SMART_CARD_OUTLINE),
        "table-row-height" => Some(icons::TABLE_ROW_HEIGHT),
        "cloud-circle" => Some(icons::CLOUD_CIRCLE),
        "ray-start" => Some(icons::RAY_START),
        "progress-upload" => Some(icons::PROGRESS_UPLOAD),
        "close-network-outline" => Some(icons::CLOSE_NETWORK_OUTLINE),
        "protocol" => Some(icons::PROTOCOL),
        "file-cancel" => Some(icons::FILE_CANCEL),
        "calendar-lock-outline" => Some(icons::CALENDAR_LOCK_OUTLINE),
        "rotate-orbit" => Some(icons::ROTATE_ORBIT),
        "cog-play" => Some(icons::COG_PLAY),
        "monitor-star" => Some(icons::MONITOR_STAR),
        "monitor-speaker-off" => Some(icons::MONITOR_SPEAKER_OFF),
        "airplane-plus" => Some(icons::AIRPLANE_PLUS),
        "shipping-pallet" => Some(icons::SHIPPING_PALLET),
        "image-minus-outline" => Some(icons::IMAGE_MINUS_OUTLINE),
        "comment-multiple-outline" => Some(icons::COMMENT_MULTIPLE_OUTLINE),
        "lock" => Some(icons::LOCK),
        "watering-can-outline" => Some(icons::WATERING_CAN_OUTLINE),
        "battery-60" => Some(icons::BATTERY_60),
        "human-dolly" => Some(icons::HUMAN_DOLLY),
        "bell-badge-outline" => Some(icons::BELL_BADGE_OUTLINE),
        "repeat" => Some(icons::REPEAT),
        "mortar-pestle-plus" => Some(icons::MORTAR_PESTLE_PLUS),
        "tag-plus-outline" => Some(icons::TAG_PLUS_OUTLINE),
        "gamepad-circle-right" => Some(icons::GAMEPAD_CIRCLE_RIGHT),
        "brightness-5" => Some(icons::BRIGHTNESS_5),
        "credit-card-refresh" => Some(icons::CREDIT_CARD_REFRESH),
        "transfer-left" => Some(icons::TRANSFER_LEFT),
        "cogs" => Some(icons::COGS),
        "cloud-upload" => Some(icons::CLOUD_UPLOAD),
        "account-switch-outline" => Some(icons::ACCOUNT_SWITCH_OUTLINE),
        "file-move" => Some(icons::FILE_MOVE),
        "motion-play" => Some(icons::MOTION_PLAY),
        "waves-arrow-up" => Some(icons::WAVES_ARROW_UP),
        "folder-settings-outline" => Some(icons::FOLDER_SETTINGS_OUTLINE),
        "toggle-switch-off" => Some(icons::TOGGLE_SWITCH_OFF),
        "landslide-outline" => Some(icons::LANDSLIDE_OUTLINE),
        #[allow(deprecated)]
        "tailwind" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'tailwind' is deprecated.").print(py);
            }
            Some(icons::TAILWIND)
        }
        "butterfly-outline" => Some(icons::BUTTERFLY_OUTLINE),
        "coffee-maker-outline" => Some(icons::COFFEE_MAKER_OUTLINE),
        "video-marker" => Some(icons::VIDEO_MARKER),
        "store-marker" => Some(icons::STORE_MARKER),
        "timer-off" => Some(icons::TIMER_OFF),
        "fit-to-page" => Some(icons::FIT_TO_PAGE),
        "biathlon" => Some(icons::BIATHLON),
        "inbox-arrow-up-outline" => Some(icons::INBOX_ARROW_UP_OUTLINE),
        "page-layout-header-footer" => Some(icons::PAGE_LAYOUT_HEADER_FOOTER),
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        "select" => Some(icons::SELECT),
        "music" => Some(icons::MUSIC),
        "sign-language" => Some(icons::SIGN_LANGUAGE),
        "table-cog" => Some(icons::TABLE_COG),
        "tooltip-check-outline" => Some(icons::TOOLTIP_CHECK_OUTLINE),
        "certificate" => Some(icons::CERTIFICATE),
        "tune-variant" => Some(icons::TUNE_VARIANT),
        "archive-lock-outline" => Some(icons::ARCHIVE_LOCK_OUTLINE),
        "sledding" => Some(icons::SLEDDING),
        "bone" => Some(icons::BONE),
        "hours-24" => Some(icons::HOURS_24),
        #[allow(deprecated)]
        "netflix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'netflix' is deprecated.").print(py);
            }
            Some(icons::NETFLIX)
        }
        "account-plus" => Some(icons::ACCOUNT_PLUS),
        "credit-card-edit" => Some(icons::CREDIT_CARD_EDIT),
        "dice-4-outline" => Some(icons::DICE_4_OUTLINE),
        "paw-outline" => Some(icons::PAW_OUTLINE),
        "format-text-rotation-down-vertical" => Some(icons::FORMAT_TEXT_ROTATION_DOWN_VERTICAL),
        "comment-search-outline" => Some(icons::COMMENT_SEARCH_OUTLINE),
        "tune-vertical" => Some(icons::TUNE_VERTICAL),
        _ => None,
    }
}
