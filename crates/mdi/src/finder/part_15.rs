// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_15(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "file-delimited" => Some(icons::FILE_DELIMITED),
        "ssh" => Some(icons::SSH),
        "video-minus" => Some(icons::VIDEO_MINUS),
        "zodiac-scorpio" => Some(icons::ZODIAC_SCORPIO),
        "storefront-plus-outline" => Some(icons::STOREFRONT_PLUS_OUTLINE),
        "arrow-right-drop-circle-outline" => Some(icons::ARROW_RIGHT_DROP_CIRCLE_OUTLINE),
        "gate-alert" => Some(icons::GATE_ALERT),
        "alphabet-cyrillic" => Some(icons::ALPHABET_CYRILLIC),
        "cannabis" => Some(icons::CANNABIS),
        "flask-empty-off-outline" => Some(icons::FLASK_EMPTY_OFF_OUTLINE),
        "code-parentheses-box" => Some(icons::CODE_PARENTHESES_BOX),
        "floor-lamp-torchiere" => Some(icons::FLOOR_LAMP_TORCHIERE),
        "card-bulleted-off-outline" => Some(icons::CARD_BULLETED_OFF_OUTLINE),
        "trademark" => Some(icons::TRADEMARK),
        "check-network-outline" => Some(icons::CHECK_NETWORK_OUTLINE),
        "wifi-check" => Some(icons::WIFI_CHECK),
        "panorama" => Some(icons::PANORAMA),
        "cone" => Some(icons::CONE),
        "reminder" => Some(icons::REMINDER),
        "mushroom" => Some(icons::MUSHROOM),
        #[allow(deprecated)]
        "apple-ios" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-ios' is deprecated.").print(py);
            }
            Some(icons::APPLE_IOS)
        }
        "pause-box" => Some(icons::PAUSE_BOX),
        "arrange-bring-forward" => Some(icons::ARRANGE_BRING_FORWARD),
        "bus-articulated-front" => Some(icons::BUS_ARTICULATED_FRONT),
        "robot-mower" => Some(icons::ROBOT_MOWER),
        "view-week" => Some(icons::VIEW_WEEK),
        "barrel-outline" => Some(icons::BARREL_OUTLINE),
        "silverware-fork-knife" => Some(icons::SILVERWARE_FORK_KNIFE),
        "lightbulb-night-outline" => Some(icons::LIGHTBULB_NIGHT_OUTLINE),
        "archive-lock" => Some(icons::ARCHIVE_LOCK),
        "relation-zero-or-many-to-one-or-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE_OR_MANY),
        "arrow-right-bottom-bold" => Some(icons::ARROW_RIGHT_BOTTOM_BOLD),
        "cog-stop" => Some(icons::COG_STOP),
        "play-box-lock-outline" => Some(icons::PLAY_BOX_LOCK_OUTLINE),
        "account-supervisor-outline" => Some(icons::ACCOUNT_SUPERVISOR_OUTLINE),
        "numeric-0-box" => Some(icons::NUMERIC_0_BOX),
        "plus-outline" => Some(icons::PLUS_OUTLINE),
        "silverware-clean" => Some(icons::SILVERWARE_CLEAN),
        "map-marker-remove-outline" => Some(icons::MAP_MARKER_REMOVE_OUTLINE),
        "hockey-puck" => Some(icons::HOCKEY_PUCK),
        "screw-machine-round-top" => Some(icons::SCREW_MACHINE_ROUND_TOP),
        "roman-numeral-6" => Some(icons::ROMAN_NUMERAL_6),
        "bank-check" => Some(icons::BANK_CHECK),
        "face-woman-outline" => Some(icons::FACE_WOMAN_OUTLINE),
        "relation-only-one-to-one-or-many" => Some(icons::RELATION_ONLY_ONE_TO_ONE_OR_MANY),
        "invoice-text-remove" => Some(icons::INVOICE_TEXT_REMOVE),
        "cloud-cancel-outline" => Some(icons::CLOUD_CANCEL_OUTLINE),
        "compare-vertical" => Some(icons::COMPARE_VERTICAL),
        "arrow-right-bold-hexagon-outline" => Some(icons::ARROW_RIGHT_BOLD_HEXAGON_OUTLINE),
        "phone-return-outline" => Some(icons::PHONE_RETURN_OUTLINE),
        "thought-bubble" => Some(icons::THOUGHT_BUBBLE),
        "dresser-outline" => Some(icons::DRESSER_OUTLINE),
        "emoticon-kiss-outline" => Some(icons::EMOTICON_KISS_OUTLINE),
        "ampersand" => Some(icons::AMPERSAND),
        "arrange-send-backward" => Some(icons::ARRANGE_SEND_BACKWARD),
        "basket-plus-outline" => Some(icons::BASKET_PLUS_OUTLINE),
        "alpha-p-circle-outline" => Some(icons::ALPHA_P_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "black-mesa" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'black-mesa' is deprecated.").print(py);
            }
            Some(icons::BLACK_MESA)
        }
        "scissors-cutting" => Some(icons::SCISSORS_CUTTING),
        "invoice-text" => Some(icons::INVOICE_TEXT),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-full" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-full' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_FULL)
        }
        "seat" => Some(icons::SEAT),
        "lock-plus-outline" => Some(icons::LOCK_PLUS_OUTLINE),
        "folder-marker-outline" => Some(icons::FOLDER_MARKER_OUTLINE),
        "credit-card-settings" => Some(icons::CREDIT_CARD_SETTINGS),
        "test-tube" => Some(icons::TEST_TUBE),
        "minus-box-outline" => Some(icons::MINUS_BOX_OUTLINE),
        "television-stop" => Some(icons::TELEVISION_STOP),
        "select-inverse" => Some(icons::SELECT_INVERSE),
        "glass-mug-variant-off" => Some(icons::GLASS_MUG_VARIANT_OFF),
        "phone-lock" => Some(icons::PHONE_LOCK),
        "bag-carry-on-off" => Some(icons::BAG_CARRY_ON_OFF),
        "cube-unfolded" => Some(icons::CUBE_UNFOLDED),
        "domain-switch" => Some(icons::DOMAIN_SWITCH),
        "grill-outline" => Some(icons::GRILL_OUTLINE),
        "movie-open-settings" => Some(icons::MOVIE_OPEN_SETTINGS),
        "saxophone" => Some(icons::SAXOPHONE),
        "tooltip-remove-outline" => Some(icons::TOOLTIP_REMOVE_OUTLINE),
        "folder-remove" => Some(icons::FOLDER_REMOVE),
        "circle-slice-3" => Some(icons::CIRCLE_SLICE_3),
        "file-table-box-outline" => Some(icons::FILE_TABLE_BOX_OUTLINE),
        "wall-sconce-flat-variant" => Some(icons::WALL_SCONCE_FLAT_VARIANT),
        "web-refresh" => Some(icons::WEB_REFRESH),
        "tanker-truck" => Some(icons::TANKER_TRUCK),
        "blender" => Some(icons::BLENDER),
        "signal-3g" => Some(icons::SIGNAL_3G),
        "map-search" => Some(icons::MAP_SEARCH),
        #[allow(deprecated)]
        "microsoft-onedrive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onedrive' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONEDRIVE)
        }
        "keyboard-esc" => Some(icons::KEYBOARD_ESC),
        "snowboard" => Some(icons::SNOWBOARD),
        "bottle-tonic-plus-outline" => Some(icons::BOTTLE_TONIC_PLUS_OUTLINE),
        "baseball" => Some(icons::BASEBALL),
        "folder-star" => Some(icons::FOLDER_STAR),
        "invoice-text-outline" => Some(icons::INVOICE_TEXT_OUTLINE),
        "kettle-steam-outline" => Some(icons::KETTLE_STEAM_OUTLINE),
        "book-arrow-up-outline" => Some(icons::BOOK_ARROW_UP_OUTLINE),
        "medical-cotton-swab" => Some(icons::MEDICAL_COTTON_SWAB),
        "notebook-edit-outline" => Some(icons::NOTEBOOK_EDIT_OUTLINE),
        "data-matrix-minus" => Some(icons::DATA_MATRIX_MINUS),
        "bus-sign" => Some(icons::BUS_SIGN),
        "dots-hexagon" => Some(icons::DOTS_HEXAGON),
        "send-circle" => Some(icons::SEND_CIRCLE),
        "tune-vertical" => Some(icons::TUNE_VERTICAL),
        "speaker-multiple" => Some(icons::SPEAKER_MULTIPLE),
        "emoticon-plus-outline" => Some(icons::EMOTICON_PLUS_OUTLINE),
        "printer-pos-minus-outline" => Some(icons::PRINTER_POS_MINUS_OUTLINE),
        "credit-card-refund-outline" => Some(icons::CREDIT_CARD_REFUND_OUTLINE),
        "message-plus-outline" => Some(icons::MESSAGE_PLUS_OUTLINE),
        "swap-vertical-circle" => Some(icons::SWAP_VERTICAL_CIRCLE),
        "bottle-tonic-skull" => Some(icons::BOTTLE_TONIC_SKULL),
        "account-sync-outline" => Some(icons::ACCOUNT_SYNC_OUTLINE),
        "numeric-0-circle-outline" => Some(icons::NUMERIC_0_CIRCLE_OUTLINE),
        "nail" => Some(icons::NAIL),
        #[allow(deprecated)]
        "git" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'git' is deprecated.").print(py);
            }
            Some(icons::GIT)
        }
        "transit-connection" => Some(icons::TRANSIT_CONNECTION),
        "microwave" => Some(icons::MICROWAVE),
        "vector-polyline-remove" => Some(icons::VECTOR_POLYLINE_REMOVE),
        "torch" => Some(icons::TORCH),
        "firework-off" => Some(icons::FIREWORK_OFF),
        "delete-variant" => Some(icons::DELETE_VARIANT),
        "puzzle-plus" => Some(icons::PUZZLE_PLUS),
        "printer-pos-wrench-outline" => Some(icons::PRINTER_POS_WRENCH_OUTLINE),
        "dresser" => Some(icons::DRESSER),
        "arrow-u-left-bottom-bold" => Some(icons::ARROW_U_LEFT_BOTTOM_BOLD),
        "map-marker-question-outline" => Some(icons::MAP_MARKER_QUESTION_OUTLINE),
        "movie-settings-outline" => Some(icons::MOVIE_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "whatsapp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'whatsapp' is deprecated.").print(py);
            }
            Some(icons::WHATSAPP)
        }
        "cloud-cancel" => Some(icons::CLOUD_CANCEL),
        "router-network-wireless" => Some(icons::ROUTER_NETWORK_WIRELESS),
        "receipt-text-check" => Some(icons::RECEIPT_TEXT_CHECK),
        "timeline-check" => Some(icons::TIMELINE_CHECK),
        "database-arrow-right" => Some(icons::DATABASE_ARROW_RIGHT),
        "wrench-cog" => Some(icons::WRENCH_COG),
        "file-excel-box-outline" => Some(icons::FILE_EXCEL_BOX_OUTLINE),
        "timer-music-outline" => Some(icons::TIMER_MUSIC_OUTLINE),
        "invoice-text-send" => Some(icons::INVOICE_TEXT_SEND),
        "minus-box-multiple" => Some(icons::MINUS_BOX_MULTIPLE),
        "loupe" => Some(icons::LOUPE),
        "alpha-n-circle" => Some(icons::ALPHA_N_CIRCLE),
        "hours-24" => Some(icons::HOURS_24),
        "account-plus" => Some(icons::ACCOUNT_PLUS),
        "format-page-break" => Some(icons::FORMAT_PAGE_BREAK),
        "arrow-right-top" => Some(icons::ARROW_RIGHT_TOP),
        #[allow(deprecated)]
        "google-keep" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-keep' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_KEEP)
        }
        "developer-board" => Some(icons::DEVELOPER_BOARD),
        "led-outline" => Some(icons::LED_OUTLINE),
        "view-column" => Some(icons::VIEW_COLUMN),
        "jellyfish-outline" => Some(icons::JELLYFISH_OUTLINE),
        "truck-delivery-outline" => Some(icons::TRUCK_DELIVERY_OUTLINE),
        "watch-vibrate-off" => Some(icons::WATCH_VIBRATE_OFF),
        "iron" => Some(icons::IRON),
        "order-bool-ascending-variant" => Some(icons::ORDER_BOOL_ASCENDING_VARIANT),
        "bag-suitcase-off" => Some(icons::BAG_SUITCASE_OFF),
        "view-compact" => Some(icons::VIEW_COMPACT),
        "railroad-light" => Some(icons::RAILROAD_LIGHT),
        "text-box-multiple-outline" => Some(icons::TEXT_BOX_MULTIPLE_OUTLINE),
        "upload-circle-outline" => Some(icons::UPLOAD_CIRCLE_OUTLINE),
        "clipboard-play-multiple-outline" => Some(icons::CLIPBOARD_PLAY_MULTIPLE_OUTLINE),
        "checkerboard" => Some(icons::CHECKERBOARD),
        "gas-station-off-outline" => Some(icons::GAS_STATION_OFF_OUTLINE),
        "arrow-u-left-top" => Some(icons::ARROW_U_LEFT_TOP),
        "qrcode" => Some(icons::QRCODE),
        "car-select" => Some(icons::CAR_SELECT),
        "forum-remove-outline" => Some(icons::FORUM_REMOVE_OUTLINE),
        "webhook" => Some(icons::WEBHOOK),
        "alert-circle" => Some(icons::ALERT_CIRCLE),
        "seed-plus-outline" => Some(icons::SEED_PLUS_OUTLINE),
        "alpha-s-box-outline" => Some(icons::ALPHA_S_BOX_OUTLINE),
        "delete-sweep" => Some(icons::DELETE_SWEEP),
        "store-settings" => Some(icons::STORE_SETTINGS),
        "format-letter-case-upper" => Some(icons::FORMAT_LETTER_CASE_UPPER),
        "play-circle-outline" => Some(icons::PLAY_CIRCLE_OUTLINE),
        "image-sync-outline" => Some(icons::IMAGE_SYNC_OUTLINE),
        "music-accidental-double-flat" => Some(icons::MUSIC_ACCIDENTAL_DOUBLE_FLAT),
        "football-australian" => Some(icons::FOOTBALL_AUSTRALIAN),
        "speedometer-medium" => Some(icons::SPEEDOMETER_MEDIUM),
        "shoe-print" => Some(icons::SHOE_PRINT),
        "account-arrow-left-outline" => Some(icons::ACCOUNT_ARROW_LEFT_OUTLINE),
        "projector-screen-variant-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OUTLINE),
        "briefcase-download" => Some(icons::BRIEFCASE_DOWNLOAD),
        "arrow-decision-outline" => Some(icons::ARROW_DECISION_OUTLINE),
        "home-floor-3" => Some(icons::HOME_FLOOR_3),
        "gauge-empty" => Some(icons::GAUGE_EMPTY),
        "format-pilcrow-arrow-right" => Some(icons::FORMAT_PILCROW_ARROW_RIGHT),
        "augmented-reality" => Some(icons::AUGMENTED_REALITY),
        "movie-settings" => Some(icons::MOVIE_SETTINGS),
        #[allow(deprecated)]
        "deviantart" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'deviantart' is deprecated.").print(py);
            }
            Some(icons::DEVIANTART)
        }
        "led-on" => Some(icons::LED_ON),
        "eye" => Some(icons::EYE),
        "folder-zip-outline" => Some(icons::FOLDER_ZIP_OUTLINE),
        "air-conditioner" => Some(icons::AIR_CONDITIONER),
        "timeline-text" => Some(icons::TIMELINE_TEXT),
        "antenna" => Some(icons::ANTENNA),
        "printer-3d-nozzle-outline" => Some(icons::PRINTER_3D_NOZZLE_OUTLINE),
        "mailbox-open" => Some(icons::MAILBOX_OPEN),
        "alpha-z-circle-outline" => Some(icons::ALPHA_Z_CIRCLE_OUTLINE),
        "sort-variant-lock-open" => Some(icons::SORT_VARIANT_LOCK_OPEN),
        "alpha-o-circle" => Some(icons::ALPHA_O_CIRCLE),
        "ocr" => Some(icons::OCR),
        "angle-acute" => Some(icons::ANGLE_ACUTE),
        _ => None,
    }
}
