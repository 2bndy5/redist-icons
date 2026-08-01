// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_17(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "baseball" => Some(icons::BASEBALL),
        "numeric-off" => Some(icons::NUMERIC_OFF),
        "invoice-send" => Some(icons::INVOICE_SEND),
        "bow-tie" => Some(icons::BOW_TIE),
        #[allow(deprecated)]
        "qi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qi' is deprecated.").print(py);
            }
            Some(icons::QI)
        }
        "wave" => Some(icons::WAVE),
        "receipt-text-remove" => Some(icons::RECEIPT_TEXT_REMOVE),
        #[allow(deprecated)]
        "language-cpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-cpp' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CPP)
        }
        "hexagon-slice-6" => Some(icons::HEXAGON_SLICE_6),
        "home-city" => Some(icons::HOME_CITY),
        "circle-outline" => Some(icons::CIRCLE_OUTLINE),
        "heart-circle" => Some(icons::HEART_CIRCLE),
        "lightbulb-cfl-spiral-off" => Some(icons::LIGHTBULB_CFL_SPIRAL_OFF),
        "graph" => Some(icons::GRAPH),
        "family-tree" => Some(icons::FAMILY_TREE),
        "lingerie" => Some(icons::LINGERIE),
        "calendar-import-outline" => Some(icons::CALENDAR_IMPORT_OUTLINE),
        "marker-cancel" => Some(icons::MARKER_CANCEL),
        "vacuum-outline" => Some(icons::VACUUM_OUTLINE),
        "caravan" => Some(icons::CARAVAN),
        "playlist-play" => Some(icons::PLAYLIST_PLAY),
        #[allow(deprecated)]
        "google-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-assistant' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ASSISTANT)
        }
        "archive-sync" => Some(icons::ARCHIVE_SYNC),
        "wifi-arrow-left-right" => Some(icons::WIFI_ARROW_LEFT_RIGHT),
        "invoice-check" => Some(icons::INVOICE_CHECK),
        "label-percent-outline" => Some(icons::LABEL_PERCENT_OUTLINE),
        "leaf-maple" => Some(icons::LEAF_MAPLE),
        "arm-flex-outline" => Some(icons::ARM_FLEX_OUTLINE),
        "thermometer-chevron-down" => Some(icons::THERMOMETER_CHEVRON_DOWN),
        "playlist-check" => Some(icons::PLAYLIST_CHECK),
        "content-save-alert" => Some(icons::CONTENT_SAVE_ALERT),
        "timer-3" => Some(icons::TIMER_3),
        "email-box" => Some(icons::EMAIL_BOX),
        "file-word-box-outline" => Some(icons::FILE_WORD_BOX_OUTLINE),
        "download-lock-outline" => Some(icons::DOWNLOAD_LOCK_OUTLINE),
        "domain-plus" => Some(icons::DOMAIN_PLUS),
        "note-outline" => Some(icons::NOTE_OUTLINE),
        "archive-marker" => Some(icons::ARCHIVE_MARKER),
        "chess-bishop" => Some(icons::CHESS_BISHOP),
        "ladder" => Some(icons::LADDER),
        "card-search" => Some(icons::CARD_SEARCH),
        "bag-personal-off" => Some(icons::BAG_PERSONAL_OFF),
        "arrow-oscillating" => Some(icons::ARROW_OSCILLATING),
        "amplifier" => Some(icons::AMPLIFIER),
        "chevron-up-box" => Some(icons::CHEVRON_UP_BOX),
        "account-card" => Some(icons::ACCOUNT_CARD),
        "printer-pos-cancel-outline" => Some(icons::PRINTER_POS_CANCEL_OUTLINE),
        "book-account" => Some(icons::BOOK_ACCOUNT),
        "white-balance-iridescent" => Some(icons::WHITE_BALANCE_IRIDESCENT),
        "door-sliding" => Some(icons::DOOR_SLIDING),
        "toy-brick-search-outline" => Some(icons::TOY_BRICK_SEARCH_OUTLINE),
        "bag-suitcase-outline" => Some(icons::BAG_SUITCASE_OUTLINE),
        "sprinkler" => Some(icons::SPRINKLER),
        "boom-gate-up-outline" => Some(icons::BOOM_GATE_UP_OUTLINE),
        "window-minimize" => Some(icons::WINDOW_MINIMIZE),
        "boom-gate-arrow-up-outline" => Some(icons::BOOM_GATE_ARROW_UP_OUTLINE),
        "car-convertible" => Some(icons::CAR_CONVERTIBLE),
        "ski-cross-country" => Some(icons::SKI_CROSS_COUNTRY),
        "sign-language" => Some(icons::SIGN_LANGUAGE),
        "signal-cellular-2" => Some(icons::SIGNAL_CELLULAR_2),
        "format-text-rotation-vertical" => Some(icons::FORMAT_TEXT_ROTATION_VERTICAL),
        "timer-lock-open-outline" => Some(icons::TIMER_LOCK_OPEN_OUTLINE),
        "power" => Some(icons::POWER),
        "camera-timer" => Some(icons::CAMERA_TIMER),
        "layers-search" => Some(icons::LAYERS_SEARCH),
        "calendar-multiselect" => Some(icons::CALENDAR_MULTISELECT),
        "map-marker-up" => Some(icons::MAP_MARKER_UP),
        #[allow(deprecated)]
        "microsoft-xbox-controller-menu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-menu' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_MENU)
        }
        "cellphone-link-off" => Some(icons::CELLPHONE_LINK_OFF),
        "compost" => Some(icons::COMPOST),
        "unfold-less-horizontal" => Some(icons::UNFOLD_LESS_HORIZONTAL),
        "cone-off" => Some(icons::CONE_OFF),
        "head-question" => Some(icons::HEAD_QUESTION),
        "account-convert-outline" => Some(icons::ACCOUNT_CONVERT_OUTLINE),
        "download-box" => Some(icons::DOWNLOAD_BOX),
        "content-save-move-outline" => Some(icons::CONTENT_SAVE_MOVE_OUTLINE),
        "image-remove" => Some(icons::IMAGE_REMOVE),
        "camera-party-mode" => Some(icons::CAMERA_PARTY_MODE),
        "car-speed-limiter" => Some(icons::CAR_SPEED_LIMITER),
        "wrench" => Some(icons::WRENCH),
        "arrow-right-circle-outline" => Some(icons::ARROW_RIGHT_CIRCLE_OUTLINE),
        "alpha-q" => Some(icons::ALPHA_Q),
        "flag-minus" => Some(icons::FLAG_MINUS),
        "flag-remove" => Some(icons::FLAG_REMOVE),
        "thermometer-auto" => Some(icons::THERMOMETER_AUTO),
        "rotate-right-variant" => Some(icons::ROTATE_RIGHT_VARIANT),
        "axis" => Some(icons::AXIS),
        "play-box-outline" => Some(icons::PLAY_BOX_OUTLINE),
        "keyboard-outline" => Some(icons::KEYBOARD_OUTLINE),
        "arrow-bottom-left-bold-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_OUTLINE),
        "gamepad-round-up" => Some(icons::GAMEPAD_ROUND_UP),
        "buffet" => Some(icons::BUFFET),
        "room-service-outline" => Some(icons::ROOM_SERVICE_OUTLINE),
        "call-merge" => Some(icons::CALL_MERGE),
        "music-rest-whole" => Some(icons::MUSIC_REST_WHOLE),
        "flask-outline" => Some(icons::FLASK_OUTLINE),
        "checkbox-blank-outline" => Some(icons::CHECKBOX_BLANK_OUTLINE),
        "battery-heart" => Some(icons::BATTERY_HEART),
        "subway-variant" => Some(icons::SUBWAY_VARIANT),
        "close-network-outline" => Some(icons::CLOSE_NETWORK_OUTLINE),
        "card-account-phone-outline" => Some(icons::CARD_ACCOUNT_PHONE_OUTLINE),
        #[allow(deprecated)]
        "codepen" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'codepen' is deprecated.").print(py);
            }
            Some(icons::CODEPEN)
        }
        "message-reply" => Some(icons::MESSAGE_REPLY),
        "two-factor-authentication" => Some(icons::TWO_FACTOR_AUTHENTICATION),
        "wave-undercurrent" => Some(icons::WAVE_UNDERCURRENT),
        "cards-diamond" => Some(icons::CARDS_DIAMOND),
        "map-marker-path" => Some(icons::MAP_MARKER_PATH),
        "cradle-outline" => Some(icons::CRADLE_OUTLINE),
        "archive-lock-open" => Some(icons::ARCHIVE_LOCK_OPEN),
        "ornament-variant" => Some(icons::ORNAMENT_VARIANT),
        "information-slab-box" => Some(icons::INFORMATION_SLAB_BOX),
        "alpha-t-circle-outline" => Some(icons::ALPHA_T_CIRCLE_OUTLINE),
        "vector-line" => Some(icons::VECTOR_LINE),
        "home-floor-0" => Some(icons::HOME_FLOOR_0),
        "percent-box" => Some(icons::PERCENT_BOX),
        "group" => Some(icons::GROUP),
        "flask-remove" => Some(icons::FLASK_REMOVE),
        #[allow(deprecated)]
        "language-swift" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-swift' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_SWIFT)
        }
        "monitor-off" => Some(icons::MONITOR_OFF),
        "cards-playing" => Some(icons::CARDS_PLAYING),
        "mother-nurse" => Some(icons::MOTHER_NURSE),
        "comment-question-outline" => Some(icons::COMMENT_QUESTION_OUTLINE),
        "image-filter-tilt-shift" => Some(icons::IMAGE_FILTER_TILT_SHIFT),
        #[allow(deprecated)]
        "gmail" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gmail' is deprecated.").print(py);
            }
            Some(icons::GMAIL)
        }
        "folder-heart-outline" => Some(icons::FOLDER_HEART_OUTLINE),
        "account-question" => Some(icons::ACCOUNT_QUESTION),
        "shield-account-variant-outline" => Some(icons::SHIELD_ACCOUNT_VARIANT_OUTLINE),
        "format-list-checks" => Some(icons::FORMAT_LIST_CHECKS),
        "home-outline" => Some(icons::HOME_OUTLINE),
        "alpha-r" => Some(icons::ALPHA_R),
        "human-male-height-variant" => Some(icons::HUMAN_MALE_HEIGHT_VARIANT),
        "spotlight-beam" => Some(icons::SPOTLIGHT_BEAM),
        "file-lock-open" => Some(icons::FILE_LOCK_OPEN),
        "progress-helper" => Some(icons::PROGRESS_HELPER),
        "order-numeric-ascending" => Some(icons::ORDER_NUMERIC_ASCENDING),
        "transit-connection-horizontal" => Some(icons::TRANSIT_CONNECTION_HORIZONTAL),
        "cloud-minus-outline" => Some(icons::CLOUD_MINUS_OUTLINE),
        "store-outline" => Some(icons::STORE_OUTLINE),
        "shoe-cleat" => Some(icons::SHOE_CLEAT),
        "fan-speed-3" => Some(icons::FAN_SPEED_3),
        "folder-hidden" => Some(icons::FOLDER_HIDDEN),
        "harddisk-remove" => Some(icons::HARDDISK_REMOVE),
        "file-question-outline" => Some(icons::FILE_QUESTION_OUTLINE),
        "induction" => Some(icons::INDUCTION),
        "timer-edit-outline" => Some(icons::TIMER_EDIT_OUTLINE),
        "hand-cycle" => Some(icons::HAND_CYCLE),
        "monitor-vertical" => Some(icons::MONITOR_VERTICAL),
        "new-box" => Some(icons::NEW_BOX),
        "octagram-plus-outline" => Some(icons::OCTAGRAM_PLUS_OUTLINE),
        "note-remove" => Some(icons::NOTE_REMOVE),
        "domain-remove" => Some(icons::DOMAIN_REMOVE),
        "email-fast" => Some(icons::EMAIL_FAST),
        "keyboard-f7" => Some(icons::KEYBOARD_F7),
        "ticket-outline" => Some(icons::TICKET_OUTLINE),
        "stop-circle-outline" => Some(icons::STOP_CIRCLE_OUTLINE),
        "image-edit" => Some(icons::IMAGE_EDIT),
        "signature-freehand" => Some(icons::SIGNATURE_FREEHAND),
        "note-remove-outline" => Some(icons::NOTE_REMOVE_OUTLINE),
        "server" => Some(icons::SERVER),
        "terrain" => Some(icons::TERRAIN),
        "thermometer-chevron-up" => Some(icons::THERMOMETER_CHEVRON_UP),
        "clock-time-seven" => Some(icons::CLOCK_TIME_SEVEN),
        #[allow(deprecated)]
        "google-plus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-plus' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLUS)
        }
        "file-key" => Some(icons::FILE_KEY),
        "file-marker-outline" => Some(icons::FILE_MARKER_OUTLINE),
        "arrow-split-horizontal" => Some(icons::ARROW_SPLIT_HORIZONTAL),
        "repeat" => Some(icons::REPEAT),
        "bluetooth" => Some(icons::BLUETOOTH),
        "eye-arrow-right" => Some(icons::EYE_ARROW_RIGHT),
        "cloud-circle-outline" => Some(icons::CLOUD_CIRCLE_OUTLINE),
        "ear-hearing" => Some(icons::EAR_HEARING),
        "emoticon-remove-outline" => Some(icons::EMOTICON_REMOVE_OUTLINE),
        "hand-saw" => Some(icons::HAND_SAW),
        "folder-remove" => Some(icons::FOLDER_REMOVE),
        "radiobox-marked" => Some(icons::RADIOBOX_MARKED),
        "thermometer-probe-off" => Some(icons::THERMOMETER_PROBE_OFF),
        "layers-remove" => Some(icons::LAYERS_REMOVE),
        "drag-horizontal-variant" => Some(icons::DRAG_HORIZONTAL_VARIANT),
        "file-link" => Some(icons::FILE_LINK),
        "water" => Some(icons::WATER),
        "pencil-box-multiple-outline" => Some(icons::PENCIL_BOX_MULTIPLE_OUTLINE),
        "border-radius" => Some(icons::BORDER_RADIUS),
        "chart-sankey" => Some(icons::CHART_SANKEY),
        "camera-document-off" => Some(icons::CAMERA_DOCUMENT_OFF),
        "cart-remove" => Some(icons::CART_REMOVE),
        "pencil-minus" => Some(icons::PENCIL_MINUS),
        "bed-queen" => Some(icons::BED_QUEEN),
        "timer-stop-outline" => Some(icons::TIMER_STOP_OUTLINE),
        "battery-30" => Some(icons::BATTERY_30),
        "clipboard-multiple-outline" => Some(icons::CLIPBOARD_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "docker" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'docker' is deprecated.").print(py);
            }
            Some(icons::DOCKER)
        }
        "basket-check-outline" => Some(icons::BASKET_CHECK_OUTLINE),
        "human-scooter" => Some(icons::HUMAN_SCOOTER),
        "cow" => Some(icons::COW),
        "file-import" => Some(icons::FILE_IMPORT),
        "phone-incoming-outgoing-outline" => Some(icons::PHONE_INCOMING_OUTGOING_OUTLINE),
        "view-grid" => Some(icons::VIEW_GRID),
        "upload-lock-outline" => Some(icons::UPLOAD_LOCK_OUTLINE),
        "video-standard-definition" => Some(icons::VIDEO_STANDARD_DEFINITION),
        "transit-connection-variant" => Some(icons::TRANSIT_CONNECTION_VARIANT),
        _ => None,
    }
}
