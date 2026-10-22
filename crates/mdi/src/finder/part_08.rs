// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_8(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "cellphone-basic" => Some(icons::CELLPHONE_BASIC),
        "menu-left" => Some(icons::MENU_LEFT),
        "go-kart-track" => Some(icons::GO_KART_TRACK),
        "account-heart-outline" => Some(icons::ACCOUNT_HEART_OUTLINE),
        "progress-download" => Some(icons::PROGRESS_DOWNLOAD),
        "source-branch-plus" => Some(icons::SOURCE_BRANCH_PLUS),
        "arrow-left" => Some(icons::ARROW_LEFT),
        "checkerboard-minus" => Some(icons::CHECKERBOARD_MINUS),
        "castle" => Some(icons::CASTLE),
        "speaker-wireless" => Some(icons::SPEAKER_WIRELESS),
        "newspaper-check" => Some(icons::NEWSPAPER_CHECK),
        #[allow(deprecated)]
        "pokemon-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pokemon-go' is deprecated.").print(py);
            }
            Some(icons::POKEMON_GO)
        }
        "numeric-6-box-outline" => Some(icons::NUMERIC_6_BOX_OUTLINE),
        "boomerang" => Some(icons::BOOMERANG),
        "crop-rotate" => Some(icons::CROP_ROTATE),
        "movie-minus-outline" => Some(icons::MOVIE_MINUS_OUTLINE),
        "auto-mode" => Some(icons::AUTO_MODE),
        "human-female-female-child" => Some(icons::HUMAN_FEMALE_FEMALE_CHILD),
        "account-circle-outline" => Some(icons::ACCOUNT_CIRCLE_OUTLINE),
        "flag-checkered" => Some(icons::FLAG_CHECKERED),
        "page-layout-header-footer" => Some(icons::PAGE_LAYOUT_HEADER_FOOTER),
        "pail-plus" => Some(icons::PAIL_PLUS),
        "thumb-down" => Some(icons::THUMB_DOWN),
        "select-marker" => Some(icons::SELECT_MARKER),
        "battery-charging-high" => Some(icons::BATTERY_CHARGING_HIGH),
        "battery-clock-outline" => Some(icons::BATTERY_CLOCK_OUTLINE),
        "asterisk" => Some(icons::ASTERISK),
        "currency-mnt" => Some(icons::CURRENCY_MNT),
        "table-column-plus-before" => Some(icons::TABLE_COLUMN_PLUS_BEFORE),
        "cards-playing-spade" => Some(icons::CARDS_PLAYING_SPADE),
        "lightbulb-off-outline" => Some(icons::LIGHTBULB_OFF_OUTLINE),
        "notebook-heart-outline" => Some(icons::NOTEBOOK_HEART_OUTLINE),
        "play" => Some(icons::PLAY),
        "lightbulb-spot" => Some(icons::LIGHTBULB_SPOT),
        "billboard" => Some(icons::BILLBOARD),
        "mace" => Some(icons::MACE),
        "office-building" => Some(icons::OFFICE_BUILDING),
        "kettle-alert-outline" => Some(icons::KETTLE_ALERT_OUTLINE),
        "blinds-horizontal" => Some(icons::BLINDS_HORIZONTAL),
        "arrow-u-left-bottom" => Some(icons::ARROW_U_LEFT_BOTTOM),
        "account-outline" => Some(icons::ACCOUNT_OUTLINE),
        "headset" => Some(icons::HEADSET),
        "cake-layered" => Some(icons::CAKE_LAYERED),
        "ring" => Some(icons::RING),
        "power-cycle" => Some(icons::POWER_CYCLE),
        "shield-check-outline" => Some(icons::SHIELD_CHECK_OUTLINE),
        "arrow-expand-up" => Some(icons::ARROW_EXPAND_UP),
        "decimal-comma-increase" => Some(icons::DECIMAL_COMMA_INCREASE),
        "train-car-hopper" => Some(icons::TRAIN_CAR_HOPPER),
        "alpha-r-box" => Some(icons::ALPHA_R_BOX),
        "playlist-play" => Some(icons::PLAYLIST_PLAY),
        "account-credit-card-outline" => Some(icons::ACCOUNT_CREDIT_CARD_OUTLINE),
        "flag-minus-outline" => Some(icons::FLAG_MINUS_OUTLINE),
        "movie-edit-outline" => Some(icons::MOVIE_EDIT_OUTLINE),
        "link-edit" => Some(icons::LINK_EDIT),
        "account-card" => Some(icons::ACCOUNT_CARD),
        "pause-box" => Some(icons::PAUSE_BOX),
        "comment-processing-outline" => Some(icons::COMMENT_PROCESSING_OUTLINE),
        "shield-link-variant" => Some(icons::SHIELD_LINK_VARIANT),
        "baby-face" => Some(icons::BABY_FACE),
        "cog-sync" => Some(icons::COG_SYNC),
        "arrow-oscillating" => Some(icons::ARROW_OSCILLATING),
        "food-croissant" => Some(icons::FOOD_CROISSANT),
        "camera-enhance" => Some(icons::CAMERA_ENHANCE),
        "incognito-off" => Some(icons::INCOGNITO_OFF),
        "decimal-decrease" => Some(icons::DECIMAL_DECREASE),
        "cog-sync-outline" => Some(icons::COG_SYNC_OUTLINE),
        "email-alert-outline" => Some(icons::EMAIL_ALERT_OUTLINE),
        "cake-variant-outline" => Some(icons::CAKE_VARIANT_OUTLINE),
        "lightbulb-question-outline" => Some(icons::LIGHTBULB_QUESTION_OUTLINE),
        "information-variant-circle-outline" => Some(icons::INFORMATION_VARIANT_CIRCLE_OUTLINE),
        "email-lock-outline" => Some(icons::EMAIL_LOCK_OUTLINE),
        "order-numeric-ascending" => Some(icons::ORDER_NUMERIC_ASCENDING),
        "hydraulic-oil-temperature" => Some(icons::HYDRAULIC_OIL_TEMPERATURE),
        "archive-search" => Some(icons::ARCHIVE_SEARCH),
        "train-car-caboose" => Some(icons::TRAIN_CAR_CABOOSE),
        "music-note-eighth" => Some(icons::MUSIC_NOTE_EIGHTH),
        "folder-upload" => Some(icons::FOLDER_UPLOAD),
        #[allow(deprecated)]
        "wikipedia" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wikipedia' is deprecated.").print(py);
            }
            Some(icons::WIKIPEDIA)
        }
        "hand-extended" => Some(icons::HAND_EXTENDED),
        "battery-alert-bluetooth" => Some(icons::BATTERY_ALERT_BLUETOOTH),
        "flashlight-off" => Some(icons::FLASHLIGHT_OFF),
        "kettlebell" => Some(icons::KETTLEBELL),
        "file-document-alert-outline" => Some(icons::FILE_DOCUMENT_ALERT_OUTLINE),
        "calendar-badge-outline" => Some(icons::CALENDAR_BADGE_OUTLINE),
        "glass-mug-variant" => Some(icons::GLASS_MUG_VARIANT),
        "mower-on" => Some(icons::MOWER_ON),
        "cloud-refresh-variant" => Some(icons::CLOUD_REFRESH_VARIANT),
        "account-arrow-down" => Some(icons::ACCOUNT_ARROW_DOWN),
        "printer-pos-pause" => Some(icons::PRINTER_POS_PAUSE),
        "music-rest-half" => Some(icons::MUSIC_REST_HALF),
        "alpha-m" => Some(icons::ALPHA_M),
        "file-marker" => Some(icons::FILE_MARKER),
        "camera-plus-outline" => Some(icons::CAMERA_PLUS_OUTLINE),
        "relation-zero-or-one-to-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE),
        "ip" => Some(icons::IP),
        "alpha-r-box-outline" => Some(icons::ALPHA_R_BOX_OUTLINE),
        #[allow(deprecated)]
        "whatsapp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'whatsapp' is deprecated.").print(py);
            }
            Some(icons::WHATSAPP)
        }
        "currency-ngn" => Some(icons::CURRENCY_NGN),
        "cloud-cancel-outline" => Some(icons::CLOUD_CANCEL_OUTLINE),
        "signature-text" => Some(icons::SIGNATURE_TEXT),
        "lightbulb-alert" => Some(icons::LIGHTBULB_ALERT),
        "led-strip-variant" => Some(icons::LED_STRIP_VARIANT),
        "eye-refresh" => Some(icons::EYE_REFRESH),
        "cloud" => Some(icons::CLOUD),
        "bag-carry-on" => Some(icons::BAG_CARRY_ON),
        "close-box" => Some(icons::CLOSE_BOX),
        "shield-outline" => Some(icons::SHIELD_OUTLINE),
        "message-lock-outline" => Some(icons::MESSAGE_LOCK_OUTLINE),
        "weight-gram" => Some(icons::WEIGHT_GRAM),
        "gesture-swipe-right" => Some(icons::GESTURE_SWIPE_RIGHT),
        "skip-forward-outline" => Some(icons::SKIP_FORWARD_OUTLINE),
        "phone-voip" => Some(icons::PHONE_VOIP),
        "bottle-soda-classic" => Some(icons::BOTTLE_SODA_CLASSIC),
        #[allow(deprecated)]
        "nintendo-switch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-switch' is deprecated.")
                    .print(py);
            }
            Some(icons::NINTENDO_SWITCH)
        }
        "train-car-intermodal" => Some(icons::TRAIN_CAR_INTERMODAL),
        "distribute-horizontal-right" => Some(icons::DISTRIBUTE_HORIZONTAL_RIGHT),
        "translate-variant" => Some(icons::TRANSLATE_VARIANT),
        "alpha-m-box" => Some(icons::ALPHA_M_BOX),
        "volume-medium" => Some(icons::VOLUME_MEDIUM),
        "map-clock" => Some(icons::MAP_CLOCK),
        #[allow(deprecated)]
        "google-my-business" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-my-business' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_MY_BUSINESS)
        }
        "clipboard-pulse" => Some(icons::CLIPBOARD_PULSE),
        "align-horizontal-center" => Some(icons::ALIGN_HORIZONTAL_CENTER),
        "label-variant" => Some(icons::LABEL_VARIANT),
        "dice-3" => Some(icons::DICE_3),
        "wrench-clock-outline" => Some(icons::WRENCH_CLOCK_OUTLINE),
        "airplane-search" => Some(icons::AIRPLANE_SEARCH),
        "file-question-outline" => Some(icons::FILE_QUESTION_OUTLINE),
        "car-outline" => Some(icons::CAR_OUTLINE),
        "rename-box-outline" => Some(icons::RENAME_BOX_OUTLINE),
        "folder-sync" => Some(icons::FOLDER_SYNC),
        "dice-d12" => Some(icons::DICE_D12),
        "hamburger-minus" => Some(icons::HAMBURGER_MINUS),
        "account-network-off-outline" => Some(icons::ACCOUNT_NETWORK_OFF_OUTLINE),
        "subdirectory-arrow-left" => Some(icons::SUBDIRECTORY_ARROW_LEFT),
        "video-marker-outline" => Some(icons::VIDEO_MARKER_OUTLINE),
        "music-note-sixteenth" => Some(icons::MUSIC_NOTE_SIXTEENTH),
        "dog-service" => Some(icons::DOG_SERVICE),
        "call-made" => Some(icons::CALL_MADE),
        "transcribe-close" => Some(icons::TRANSCRIBE_CLOSE),
        "speaker-bluetooth" => Some(icons::SPEAKER_BLUETOOTH),
        "chart-box-outline" => Some(icons::CHART_BOX_OUTLINE),
        "publish-off" => Some(icons::PUBLISH_OFF),
        "magazine-pistol" => Some(icons::MAGAZINE_PISTOL),
        "metronome" => Some(icons::METRONOME),
        "calendar-cursor-outline" => Some(icons::CALENDAR_CURSOR_OUTLINE),
        "gate-xnor" => Some(icons::GATE_XNOR),
        "food-steak" => Some(icons::FOOD_STEAK),
        "currency-brl" => Some(icons::CURRENCY_BRL),
        "weather-sunset" => Some(icons::WEATHER_SUNSET),
        "egg-easter" => Some(icons::EGG_EASTER),
        "video-input-svideo" => Some(icons::VIDEO_INPUT_SVIDEO),
        "eye-outline" => Some(icons::EYE_OUTLINE),
        "package-variant-closed-check" => Some(icons::PACKAGE_VARIANT_CLOSED_CHECK),
        "tag-arrow-up" => Some(icons::TAG_ARROW_UP),
        "folder-marker-outline" => Some(icons::FOLDER_MARKER_OUTLINE),
        "information-slab-box" => Some(icons::INFORMATION_SLAB_BOX),
        "receipt-outline" => Some(icons::RECEIPT_OUTLINE),
        "email-mark-as-unread" => Some(icons::EMAIL_MARK_AS_UNREAD),
        "file-replace-outline" => Some(icons::FILE_REPLACE_OUTLINE),
        "alpha-w-circle" => Some(icons::ALPHA_W_CIRCLE),
        "domain-switch" => Some(icons::DOMAIN_SWITCH),
        "flask-off" => Some(icons::FLASK_OFF),
        "arrow-up-right-bold" => Some(icons::ARROW_UP_RIGHT_BOLD),
        "robot-excited-outline" => Some(icons::ROBOT_EXCITED_OUTLINE),
        "movie-remove-outline" => Some(icons::MOVIE_REMOVE_OUTLINE),
        "division" => Some(icons::DIVISION),
        "upload-outline" => Some(icons::UPLOAD_OUTLINE),
        "bed-single" => Some(icons::BED_SINGLE),
        "file-outline" => Some(icons::FILE_OUTLINE),
        "page-next-outline" => Some(icons::PAGE_NEXT_OUTLINE),
        "account-credit-card" => Some(icons::ACCOUNT_CREDIT_CARD),
        "credit-card-lock" => Some(icons::CREDIT_CARD_LOCK),
        "sun-angle-outline" => Some(icons::SUN_ANGLE_OUTLINE),
        "khanda" => Some(icons::KHANDA),
        "receipt-text-check-outline" => Some(icons::RECEIPT_TEXT_CHECK_OUTLINE),
        "alert-decagram-outline" => Some(icons::ALERT_DECAGRAM_OUTLINE),
        "tank" => Some(icons::TANK),
        "account-box-outline" => Some(icons::ACCOUNT_BOX_OUTLINE),
        "home-lightbulb" => Some(icons::HOME_LIGHTBULB),
        "book-plus-multiple-outline" => Some(icons::BOOK_PLUS_MULTIPLE_OUTLINE),
        "gamepad-left" => Some(icons::GAMEPAD_LEFT),
        "flash-red-eye" => Some(icons::FLASH_RED_EYE),
        "format-align-left" => Some(icons::FORMAT_ALIGN_LEFT),
        "numeric-8-circle" => Some(icons::NUMERIC_8_CIRCLE),
        "credit-card-refund" => Some(icons::CREDIT_CARD_REFUND),
        "receipt-text-remove-outline" => Some(icons::RECEIPT_TEXT_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "unicode" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unicode' is deprecated.").print(py);
            }
            Some(icons::UNICODE)
        }
        "clock-time-two-outline" => Some(icons::CLOCK_TIME_TWO_OUTLINE),
        "cake-variant" => Some(icons::CAKE_VARIANT),
        "numeric-7-box" => Some(icons::NUMERIC_7_BOX),
        "form-dropdown" => Some(icons::FORM_DROPDOWN),
        "database-alert" => Some(icons::DATABASE_ALERT),
        "remote-tv" => Some(icons::REMOTE_TV),
        "postage-stamp" => Some(icons::POSTAGE_STAMP),
        "email-lock" => Some(icons::EMAIL_LOCK),
        "puzzle-edit" => Some(icons::PUZZLE_EDIT),
        "lightbulb-cfl-spiral" => Some(icons::LIGHTBULB_CFL_SPIRAL),
        "chat-plus-outline" => Some(icons::CHAT_PLUS_OUTLINE),
        _ => None,
    }
}
