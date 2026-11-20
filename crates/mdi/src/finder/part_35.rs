// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_35(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "cursor-default-gesture-outline" => Some(icons::CURSOR_DEFAULT_GESTURE_OUTLINE),
        "disc-player" => Some(icons::DISC_PLAYER),
        "human-male-male" => Some(icons::HUMAN_MALE_MALE),
        "soldering-iron" => Some(icons::SOLDERING_IRON),
        "cloud-key" => Some(icons::CLOUD_KEY),
        #[allow(deprecated)]
        "minecraft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'minecraft' is deprecated.").print(py);
            }
            Some(icons::MINECRAFT)
        }
        "apple-keyboard-shift" => Some(icons::APPLE_KEYBOARD_SHIFT),
        "delete-empty-outline" => Some(icons::DELETE_EMPTY_OUTLINE),
        "format-italic" => Some(icons::FORMAT_ITALIC),
        "music-note-off" => Some(icons::MUSIC_NOTE_OFF),
        "page-layout-footer" => Some(icons::PAGE_LAYOUT_FOOTER),
        "cellphone-check" => Some(icons::CELLPHONE_CHECK),
        "home-silo" => Some(icons::HOME_SILO),
        "thermometer-probe" => Some(icons::THERMOMETER_PROBE),
        "movie-check" => Some(icons::MOVIE_CHECK),
        "border-inside" => Some(icons::BORDER_INSIDE),
        "bell-cog-outline" => Some(icons::BELL_COG_OUTLINE),
        "cloud-minus" => Some(icons::CLOUD_MINUS),
        "select-compare" => Some(icons::SELECT_COMPARE),
        "email-heart-outline" => Some(icons::EMAIL_HEART_OUTLINE),
        "decagram" => Some(icons::DECAGRAM),
        "death-star-variant" => Some(icons::DEATH_STAR_VARIANT),
        "credit-card-wireless" => Some(icons::CREDIT_CARD_WIRELESS),
        "not-equal-variant" => Some(icons::NOT_EQUAL_VARIANT),
        "bell-badge-outline" => Some(icons::BELL_BADGE_OUTLINE),
        "badge-account-horizontal-outline" => Some(icons::BADGE_ACCOUNT_HORIZONTAL_OUTLINE),
        "comma-circle-outline" => Some(icons::COMMA_CIRCLE_OUTLINE),
        "application-brackets-outline" => Some(icons::APPLICATION_BRACKETS_OUTLINE),
        "checkerboard-remove" => Some(icons::CHECKERBOARD_REMOVE),
        "diving" => Some(icons::DIVING),
        "message-alert-outline" => Some(icons::MESSAGE_ALERT_OUTLINE),
        "heart-broken-outline" => Some(icons::HEART_BROKEN_OUTLINE),
        "lock-minus-outline" => Some(icons::LOCK_MINUS_OUTLINE),
        "truck-check-outline" => Some(icons::TRUCK_CHECK_OUTLINE),
        "printer-3d-nozzle-off-outline" => Some(icons::PRINTER_3D_NOZZLE_OFF_OUTLINE),
        "escalator-up" => Some(icons::ESCALATOR_UP),
        "weight-kilogram" => Some(icons::WEIGHT_KILOGRAM),
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
        "map-clock-outline" => Some(icons::MAP_CLOCK_OUTLINE),
        "gender-male-female-variant" => Some(icons::GENDER_MALE_FEMALE_VARIANT),
        "fan-plus" => Some(icons::FAN_PLUS),
        "numeric-4-box" => Some(icons::NUMERIC_4_BOX),
        "arrow-bottom-left" => Some(icons::ARROW_BOTTOM_LEFT),
        #[allow(deprecated)]
        "zend" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'zend' is deprecated.").print(py);
            }
            Some(icons::ZEND)
        }
        "brush-off" => Some(icons::BRUSH_OFF),
        "music-note-sixteenth-dotted" => Some(icons::MUSIC_NOTE_SIXTEENTH_DOTTED),
        "toy-brick-remove-outline" => Some(icons::TOY_BRICK_REMOVE_OUTLINE),
        "file-question" => Some(icons::FILE_QUESTION),
        "clipboard-arrow-left-outline" => Some(icons::CLIPBOARD_ARROW_LEFT_OUTLINE),
        "zodiac-gemini" => Some(icons::ZODIAC_GEMINI),
        "archive-off-outline" => Some(icons::ARCHIVE_OFF_OUTLINE),
        "data-matrix" => Some(icons::DATA_MATRIX),
        "timer-outline" => Some(icons::TIMER_OUTLINE),
        "shovel" => Some(icons::SHOVEL),
        "dots-vertical-circle-outline" => Some(icons::DOTS_VERTICAL_CIRCLE_OUTLINE),
        "screw-flat-top" => Some(icons::SCREW_FLAT_TOP),
        "bicycle-electric" => Some(icons::BICYCLE_ELECTRIC),
        "ufo-outline" => Some(icons::UFO_OUTLINE),
        "submarine" => Some(icons::SUBMARINE),
        "magnify-plus-cursor" => Some(icons::MAGNIFY_PLUS_CURSOR),
        "panorama-wide-angle" => Some(icons::PANORAMA_WIDE_ANGLE),
        "code-greater-than" => Some(icons::CODE_GREATER_THAN),
        "elevator-up" => Some(icons::ELEVATOR_UP),
        "lightning-bolt-outline" => Some(icons::LIGHTNING_BOLT_OUTLINE),
        "pan" => Some(icons::PAN),
        "cart" => Some(icons::CART),
        "cup" => Some(icons::CUP),
        "video-input-component" => Some(icons::VIDEO_INPUT_COMPONENT),
        "crown-outline" => Some(icons::CROWN_OUTLINE),
        "briefcase-check-outline" => Some(icons::BRIEFCASE_CHECK_OUTLINE),
        "note-edit" => Some(icons::NOTE_EDIT),
        "alpha-t-circle" => Some(icons::ALPHA_T_CIRCLE),
        "phone-log-outline" => Some(icons::PHONE_LOG_OUTLINE),
        "wrench" => Some(icons::WRENCH),
        "table-column-width" => Some(icons::TABLE_COLUMN_WIDTH),
        "skull-scan" => Some(icons::SKULL_SCAN),
        "clipboard-play-multiple-outline" => Some(icons::CLIPBOARD_PLAY_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "microsoft-office" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-office' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OFFICE)
        }
        "format-annotation-plus" => Some(icons::FORMAT_ANNOTATION_PLUS),
        "led-strip-variant" => Some(icons::LED_STRIP_VARIANT),
        "book-lock-outline" => Some(icons::BOOK_LOCK_OUTLINE),
        "vector-point-edit" => Some(icons::VECTOR_POINT_EDIT),
        "pencil-lock-outline" => Some(icons::PENCIL_LOCK_OUTLINE),
        "chess-king" => Some(icons::CHESS_KING),
        "wheelchair-accessibility" => Some(icons::WHEELCHAIR_ACCESSIBILITY),
        "file-document-refresh-outline" => Some(icons::FILE_DOCUMENT_REFRESH_OUTLINE),
        "home-alert-outline" => Some(icons::HOME_ALERT_OUTLINE),
        "account-multiple-remove" => Some(icons::ACCOUNT_MULTIPLE_REMOVE),
        "leak" => Some(icons::LEAK),
        "view-compact" => Some(icons::VIEW_COMPACT),
        "alpha-e-box-outline" => Some(icons::ALPHA_E_BOX_OUTLINE),
        "cogs" => Some(icons::COGS),
        "folder-check" => Some(icons::FOLDER_CHECK),
        "star-plus" => Some(icons::STAR_PLUS),
        "compare-vertical" => Some(icons::COMPARE_VERTICAL),
        "wrench-cog-outline" => Some(icons::WRENCH_COG_OUTLINE),
        "hand-back-right" => Some(icons::HAND_BACK_RIGHT),
        "volume-mute" => Some(icons::VOLUME_MUTE),
        #[allow(deprecated)]
        "codepen" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'codepen' is deprecated.").print(py);
            }
            Some(icons::CODEPEN)
        }
        "gate-nand" => Some(icons::GATE_NAND),
        "stomach" => Some(icons::STOMACH),
        "paw-outline" => Some(icons::PAW_OUTLINE),
        "gesture-swipe" => Some(icons::GESTURE_SWIPE),
        "water-pump" => Some(icons::WATER_PUMP),
        "book-arrow-right" => Some(icons::BOOK_ARROW_RIGHT),
        "star-outline" => Some(icons::STAR_OUTLINE),
        "folder-file-outline" => Some(icons::FOLDER_FILE_OUTLINE),
        "scan-helper" => Some(icons::SCAN_HELPER),
        "flower-tulip-outline" => Some(icons::FLOWER_TULIP_OUTLINE),
        "cards-playing" => Some(icons::CARDS_PLAYING),
        "table-refresh" => Some(icons::TABLE_REFRESH),
        "pan-up" => Some(icons::PAN_UP),
        "file-replace" => Some(icons::FILE_REPLACE),
        "timetable" => Some(icons::TIMETABLE),
        "map-marker-remove" => Some(icons::MAP_MARKER_REMOVE),
        "content-save-minus-outline" => Some(icons::CONTENT_SAVE_MINUS_OUTLINE),
        "file-document-remove" => Some(icons::FILE_DOCUMENT_REMOVE),
        "application-settings-outline" => Some(icons::APPLICATION_SETTINGS_OUTLINE),
        "egg" => Some(icons::EGG),
        "comment-edit" => Some(icons::COMMENT_EDIT),
        "mortar-pestle" => Some(icons::MORTAR_PESTLE),
        #[allow(deprecated)]
        "unity" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unity' is deprecated.").print(py);
            }
            Some(icons::UNITY)
        }
        #[allow(deprecated)]
        "microsoft-outlook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-outlook' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OUTLOOK)
        }
        "account-network-outline" => Some(icons::ACCOUNT_NETWORK_OUTLINE),
        "file-image-minus-outline" => Some(icons::FILE_IMAGE_MINUS_OUTLINE),
        "book-edit" => Some(icons::BOOK_EDIT),
        "arrow-top-right-thick" => Some(icons::ARROW_TOP_RIGHT_THICK),
        "typewriter" => Some(icons::TYPEWRITER),
        "camera-plus-outline" => Some(icons::CAMERA_PLUS_OUTLINE),
        "human-male-male-child" => Some(icons::HUMAN_MALE_MALE_CHILD),
        "vector-square-remove" => Some(icons::VECTOR_SQUARE_REMOVE),
        "octagram-minus" => Some(icons::OCTAGRAM_MINUS),
        "blinds-vertical" => Some(icons::BLINDS_VERTICAL),
        "flag-checkered" => Some(icons::FLAG_CHECKERED),
        "vanish-quarter" => Some(icons::VANISH_QUARTER),
        "square-rounded-badge" => Some(icons::SQUARE_ROUNDED_BADGE),
        "syllabary-katakana" => Some(icons::SYLLABARY_KATAKANA),
        "home-search" => Some(icons::HOME_SEARCH),
        "cloud-minus-outline" => Some(icons::CLOUD_MINUS_OUTLINE),
        "wall-sconce-round-variant-outline" => Some(icons::WALL_SCONCE_ROUND_VARIANT_OUTLINE),
        "pencil-circle-outline" => Some(icons::PENCIL_CIRCLE_OUTLINE),
        "beaker-remove-outline" => Some(icons::BEAKER_REMOVE_OUTLINE),
        "transit-transfer" => Some(icons::TRANSIT_TRANSFER),
        "water-boiler" => Some(icons::WATER_BOILER),
        "link-variant-off" => Some(icons::LINK_VARIANT_OFF),
        "keyboard-space" => Some(icons::KEYBOARD_SPACE),
        "email-newsletter" => Some(icons::EMAIL_NEWSLETTER),
        "file-export-outline" => Some(icons::FILE_EXPORT_OUTLINE),
        "archive-music" => Some(icons::ARCHIVE_MUSIC),
        "floor-lamp-dual-outline" => Some(icons::FLOOR_LAMP_DUAL_OUTLINE),
        "arrow-bottom-right" => Some(icons::ARROW_BOTTOM_RIGHT),
        "airplane-alert" => Some(icons::AIRPLANE_ALERT),
        "account-wrench" => Some(icons::ACCOUNT_WRENCH),
        "roman-numeral-1" => Some(icons::ROMAN_NUMERAL_1),
        #[allow(deprecated)]
        "qi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qi' is deprecated.").print(py);
            }
            Some(icons::QI)
        }
        "transfer-up" => Some(icons::TRANSFER_UP),
        "download-lock" => Some(icons::DOWNLOAD_LOCK),
        "emoticon-confused-outline" => Some(icons::EMOTICON_CONFUSED_OUTLINE),
        "dance-ballroom" => Some(icons::DANCE_BALLROOM),
        "account-arrow-up" => Some(icons::ACCOUNT_ARROW_UP),
        "package-check" => Some(icons::PACKAGE_CHECK),
        "clock-time-five" => Some(icons::CLOCK_TIME_FIVE),
        "bookmark-check" => Some(icons::BOOKMARK_CHECK),
        "download-circle-outline" => Some(icons::DOWNLOAD_CIRCLE_OUTLINE),
        "bank" => Some(icons::BANK),
        "medication" => Some(icons::MEDICATION),
        "file-xml-box" => Some(icons::FILE_XML_BOX),
        "audio-input-stereo-minijack" => Some(icons::AUDIO_INPUT_STEREO_MINIJACK),
        "office-building-plus" => Some(icons::OFFICE_BUILDING_PLUS),
        "file-tree-outline" => Some(icons::FILE_TREE_OUTLINE),
        "send-variant" => Some(icons::SEND_VARIANT),
        "upload-box-outline" => Some(icons::UPLOAD_BOX_OUTLINE),
        "upload" => Some(icons::UPLOAD),
        "chevron-triple-left" => Some(icons::CHEVRON_TRIPLE_LEFT),
        "car-settings" => Some(icons::CAR_SETTINGS),
        "stocking" => Some(icons::STOCKING),
        "truck-remove" => Some(icons::TRUCK_REMOVE),
        "alpha-s-circle" => Some(icons::ALPHA_S_CIRCLE),
        "beaker-plus-outline" => Some(icons::BEAKER_PLUS_OUTLINE),
        "menorah" => Some(icons::MENORAH),
        "vector-union" => Some(icons::VECTOR_UNION),
        "dice-multiple" => Some(icons::DICE_MULTIPLE),
        #[allow(deprecated)]
        "linux-mint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux-mint' is deprecated.").print(py);
            }
            Some(icons::LINUX_MINT)
        }
        "van-utility" => Some(icons::VAN_UTILITY),
        "sort-bool-descending" => Some(icons::SORT_BOOL_DESCENDING),
        "home-variant" => Some(icons::HOME_VARIANT),
        "form-textbox" => Some(icons::FORM_TEXTBOX),
        "alpha-c-circle" => Some(icons::ALPHA_C_CIRCLE),
        "omega" => Some(icons::OMEGA),
        "restore-alert" => Some(icons::RESTORE_ALERT),
        "thermometer-low" => Some(icons::THERMOMETER_LOW),
        "file-gif-box" => Some(icons::FILE_GIF_BOX),
        "quality-low" => Some(icons::QUALITY_LOW),
        "border-left" => Some(icons::BORDER_LEFT),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "alpha-z-circle-outline" => Some(icons::ALPHA_Z_CIRCLE_OUTLINE),
        "clipboard-flow-outline" => Some(icons::CLIPBOARD_FLOW_OUTLINE),
        "car-child-seat" => Some(icons::CAR_CHILD_SEAT),
        "relation-one-to-one" => Some(icons::RELATION_ONE_TO_ONE),
        "relation-zero-or-one-to-one-or-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE_OR_MANY),
        _ => None,
    }
}
