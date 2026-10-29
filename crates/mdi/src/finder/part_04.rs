// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_4(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "heart-broken" => Some(icons::HEART_BROKEN),
        "cards-playing-heart" => Some(icons::CARDS_PLAYING_HEART),
        "phone-classic-off" => Some(icons::PHONE_CLASSIC_OFF),
        "clipboard-play" => Some(icons::CLIPBOARD_PLAY),
        "numeric-7-box-multiple-outline" => Some(icons::NUMERIC_7_BOX_MULTIPLE_OUTLINE),
        "directions" => Some(icons::DIRECTIONS),
        "view-sequential" => Some(icons::VIEW_SEQUENTIAL),
        "peanut-off-outline" => Some(icons::PEANUT_OFF_OUTLINE),
        "transfer-down" => Some(icons::TRANSFER_DOWN),
        "share-variant" => Some(icons::SHARE_VARIANT),
        "bank-transfer-in" => Some(icons::BANK_TRANSFER_IN),
        "file-arrow-up-down" => Some(icons::FILE_ARROW_UP_DOWN),
        "ticket-confirmation-outline" => Some(icons::TICKET_CONFIRMATION_OUTLINE),
        "alpha-u-box" => Some(icons::ALPHA_U_BOX),
        #[allow(deprecated)]
        "stackpath" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stackpath' is deprecated.").print(py);
            }
            Some(icons::STACKPATH)
        }
        "hdr-off" => Some(icons::HDR_OFF),
        "border-vertical" => Some(icons::BORDER_VERTICAL),
        "database-import-outline" => Some(icons::DATABASE_IMPORT_OUTLINE),
        #[allow(deprecated)]
        "ember" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ember' is deprecated.").print(py);
            }
            Some(icons::EMBER)
        }
        "nas" => Some(icons::NAS),
        "relation-one-or-many-to-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONE),
        "sticker-alert-outline" => Some(icons::STICKER_ALERT_OUTLINE),
        "book-remove-outline" => Some(icons::BOOK_REMOVE_OUTLINE),
        "relation-one-or-many-to-many" => Some(icons::RELATION_ONE_OR_MANY_TO_MANY),
        "clock-plus" => Some(icons::CLOCK_PLUS),
        "map-marker-remove" => Some(icons::MAP_MARKER_REMOVE),
        "clock-time-seven" => Some(icons::CLOCK_TIME_SEVEN),
        "comment-question-outline" => Some(icons::COMMENT_QUESTION_OUTLINE),
        "car-hatchback" => Some(icons::CAR_HATCHBACK),
        "heart-cog-outline" => Some(icons::HEART_COG_OUTLINE),
        "arrow-up-bold-box" => Some(icons::ARROW_UP_BOLD_BOX),
        "tag-edit" => Some(icons::TAG_EDIT),
        "view-sequential-outline" => Some(icons::VIEW_SEQUENTIAL_OUTLINE),
        "numeric-1-box-multiple-outline" => Some(icons::NUMERIC_1_BOX_MULTIPLE_OUTLINE),
        "watch-vibrate" => Some(icons::WATCH_VIBRATE),
        "inbox-multiple" => Some(icons::INBOX_MULTIPLE),
        "ufo-outline" => Some(icons::UFO_OUTLINE),
        "skip-next-outline" => Some(icons::SKIP_NEXT_OUTLINE),
        "bench-back" => Some(icons::BENCH_BACK),
        "filmstrip" => Some(icons::FILMSTRIP),
        "pipe-wrench" => Some(icons::PIPE_WRENCH),
        "thermostat-auto" => Some(icons::THERMOSTAT_AUTO),
        "view-array-outline" => Some(icons::VIEW_ARRAY_OUTLINE),
        "cookie-alert" => Some(icons::COOKIE_ALERT),
        "archive-arrow-up-outline" => Some(icons::ARCHIVE_ARROW_UP_OUTLINE),
        "relation-many-to-only-one" => Some(icons::RELATION_MANY_TO_ONLY_ONE),
        "fast-forward" => Some(icons::FAST_FORWARD),
        "account-badge-outline" => Some(icons::ACCOUNT_BADGE_OUTLINE),
        "sign-text" => Some(icons::SIGN_TEXT),
        "clipboard-arrow-right" => Some(icons::CLIPBOARD_ARROW_RIGHT),
        "view-agenda-outline" => Some(icons::VIEW_AGENDA_OUTLINE),
        "checkbox-blank-badge-outline" => Some(icons::CHECKBOX_BLANK_BADGE_OUTLINE),
        "lightbulb-multiple-outline" => Some(icons::LIGHTBULB_MULTIPLE_OUTLINE),
        "spa" => Some(icons::SPA),
        "calendar-edit" => Some(icons::CALENDAR_EDIT),
        "chili-hot" => Some(icons::CHILI_HOT),
        "bone" => Some(icons::BONE),
        "numeric-9-box-multiple" => Some(icons::NUMERIC_9_BOX_MULTIPLE),
        "nintendo-game-boy" => Some(icons::NINTENDO_GAME_BOY),
        "quality-high" => Some(icons::QUALITY_HIGH),
        "signature-freehand" => Some(icons::SIGNATURE_FREEHAND),
        "string-lights-off" => Some(icons::STRING_LIGHTS_OFF),
        "format-subscript" => Some(icons::FORMAT_SUBSCRIPT),
        #[allow(deprecated)]
        "kubernetes" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kubernetes' is deprecated.").print(py);
            }
            Some(icons::KUBERNETES)
        }
        "washing-machine" => Some(icons::WASHING_MACHINE),
        "upload-lock" => Some(icons::UPLOAD_LOCK),
        "format-text-rotation-none" => Some(icons::FORMAT_TEXT_ROTATION_NONE),
        "motion" => Some(icons::MOTION),
        "movie-remove-outline" => Some(icons::MOVIE_REMOVE_OUTLINE),
        "battery-remove-outline" => Some(icons::BATTERY_REMOVE_OUTLINE),
        "expansion-card" => Some(icons::EXPANSION_CARD),
        "gesture-swipe-right" => Some(icons::GESTURE_SWIPE_RIGHT),
        "wifi-arrow-up-down" => Some(icons::WIFI_ARROW_UP_DOWN),
        "alpha-y-box" => Some(icons::ALPHA_Y_BOX),
        "currency-jpy" => Some(icons::CURRENCY_JPY),
        "diving-scuba" => Some(icons::DIVING_SCUBA),
        "music-box" => Some(icons::MUSIC_BOX),
        "bell-remove" => Some(icons::BELL_REMOVE),
        "shopping" => Some(icons::SHOPPING),
        "star-four-points-circle" => Some(icons::STAR_FOUR_POINTS_CIRCLE),
        "pocket" => Some(icons::POCKET),
        "folder-open" => Some(icons::FOLDER_OPEN),
        "alarm-off" => Some(icons::ALARM_OFF),
        "update" => Some(icons::UPDATE),
        "book-cross" => Some(icons::BOOK_CROSS),
        "biohazard" => Some(icons::BIOHAZARD),
        "flower-pollen" => Some(icons::FLOWER_POLLEN),
        "tortoise" => Some(icons::TORTOISE),
        "square-medium" => Some(icons::SQUARE_MEDIUM),
        "roman-numeral-8" => Some(icons::ROMAN_NUMERAL_8),
        "church" => Some(icons::CHURCH),
        "invoice-text-edit" => Some(icons::INVOICE_TEXT_EDIT),
        "earbuds-off" => Some(icons::EARBUDS_OFF),
        "camera-enhance-outline" => Some(icons::CAMERA_ENHANCE_OUTLINE),
        "send-lock" => Some(icons::SEND_LOCK),
        "alarm-light-outline" => Some(icons::ALARM_LIGHT_OUTLINE),
        "clock-check" => Some(icons::CLOCK_CHECK),
        "alpha-n-circle-outline" => Some(icons::ALPHA_N_CIRCLE_OUTLINE),
        "shoe-heel" => Some(icons::SHOE_HEEL),
        "timeline-remove-outline" => Some(icons::TIMELINE_REMOVE_OUTLINE),
        "volume-vibrate" => Some(icons::VOLUME_VIBRATE),
        "clock-edit" => Some(icons::CLOCK_EDIT),
        "swap-horizontal-bold" => Some(icons::SWAP_HORIZONTAL_BOLD),
        "thumb-down" => Some(icons::THUMB_DOWN),
        "transition-masked" => Some(icons::TRANSITION_MASKED),
        "train-car-gondola" => Some(icons::TRAIN_CAR_GONDOLA),
        "clipboard-arrow-up" => Some(icons::CLIPBOARD_ARROW_UP),
        "toaster-oven" => Some(icons::TOASTER_OVEN),
        "handshake" => Some(icons::HANDSHAKE),
        "plus-circle" => Some(icons::PLUS_CIRCLE),
        "mouse-right-click-outline" => Some(icons::MOUSE_RIGHT_CLICK_OUTLINE),
        "cactus" => Some(icons::CACTUS),
        "network-strength-2-alert" => Some(icons::NETWORK_STRENGTH_2_ALERT),
        "vector-link" => Some(icons::VECTOR_LINK),
        "harddisk-plus" => Some(icons::HARDDISK_PLUS),
        "book-open-blank-variant-outline" => Some(icons::BOOK_OPEN_BLANK_VARIANT_OUTLINE),
        "note-search-outline" => Some(icons::NOTE_SEARCH_OUTLINE),
        "hand-wave-outline" => Some(icons::HAND_WAVE_OUTLINE),
        "badge-account-horizontal-outline" => Some(icons::BADGE_ACCOUNT_HORIZONTAL_OUTLINE),
        "image-filter-hdr" => Some(icons::IMAGE_FILTER_HDR),
        "biathlon" => Some(icons::BIATHLON),
        "remote-tv" => Some(icons::REMOTE_TV),
        "keyboard-settings" => Some(icons::KEYBOARD_SETTINGS),
        "vector-polygon-variant" => Some(icons::VECTOR_POLYGON_VARIANT),
        "fridge-industrial-off-outline" => Some(icons::FRIDGE_INDUSTRIAL_OFF_OUTLINE),
        "mailbox-up-outline" => Some(icons::MAILBOX_UP_OUTLINE),
        "minus-box" => Some(icons::MINUS_BOX),
        "layers-triple-outline" => Some(icons::LAYERS_TRIPLE_OUTLINE),
        "ticket" => Some(icons::TICKET),
        "lava-lamp" => Some(icons::LAVA_LAMP),
        "bed-single-outline" => Some(icons::BED_SINGLE_OUTLINE),
        "video-wireless-outline" => Some(icons::VIDEO_WIRELESS_OUTLINE),
        "lock-outline" => Some(icons::LOCK_OUTLINE),
        "chef-hat" => Some(icons::CHEF_HAT),
        "battery-20-bluetooth" => Some(icons::BATTERY_20_BLUETOOTH),
        "arrow-down-box" => Some(icons::ARROW_DOWN_BOX),
        "chart-arc" => Some(icons::CHART_ARC),
        "beach" => Some(icons::BEACH),
        "application-variable-outline" => Some(icons::APPLICATION_VARIABLE_OUTLINE),
        "iron-outline" => Some(icons::IRON_OUTLINE),
        "file-replace-outline" => Some(icons::FILE_REPLACE_OUTLINE),
        "dice-d10" => Some(icons::DICE_D10),
        "bus-school" => Some(icons::BUS_SCHOOL),
        "human-female" => Some(icons::HUMAN_FEMALE),
        "mirror" => Some(icons::MIRROR),
        "lock-clock" => Some(icons::LOCK_CLOCK),
        "file-cabinet" => Some(icons::FILE_CABINET),
        "movie-open-cog" => Some(icons::MOVIE_OPEN_COG),
        "tape-drive" => Some(icons::TAPE_DRIVE),
        "clock-time-three-outline" => Some(icons::CLOCK_TIME_THREE_OUTLINE),
        "code-less-than-or-equal" => Some(icons::CODE_LESS_THAN_OR_EQUAL),
        "heart-remove-outline" => Some(icons::HEART_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "onepassword" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'onepassword' is deprecated.").print(py);
            }
            Some(icons::ONEPASSWORD)
        }
        "axis-z-arrow" => Some(icons::AXIS_Z_ARROW),
        #[allow(deprecated)]
        "meteor" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'meteor' is deprecated.").print(py);
            }
            Some(icons::METEOR)
        }
        "file-upload-outline" => Some(icons::FILE_UPLOAD_OUTLINE),
        "calendar-text-outline" => Some(icons::CALENDAR_TEXT_OUTLINE),
        "menu-open" => Some(icons::MENU_OPEN),
        "bell-ring" => Some(icons::BELL_RING),
        "arrow-bottom-left-bold-box" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_BOX),
        "credit-card-off" => Some(icons::CREDIT_CARD_OFF),
        "harddisk-remove" => Some(icons::HARDDISK_REMOVE),
        "fast-forward-5" => Some(icons::FAST_FORWARD_5),
        "popcorn" => Some(icons::POPCORN),
        "alpha-o" => Some(icons::ALPHA_O),
        "chat-question-outline" => Some(icons::CHAT_QUESTION_OUTLINE),
        "watch" => Some(icons::WATCH),
        "logout-variant" => Some(icons::LOGOUT_VARIANT),
        "plus-box-multiple-outline" => Some(icons::PLUS_BOX_MULTIPLE_OUTLINE),
        "fence-electric" => Some(icons::FENCE_ELECTRIC),
        "stadium-outline" => Some(icons::STADIUM_OUTLINE),
        "alpha-k-circle" => Some(icons::ALPHA_K_CIRCLE),
        #[allow(deprecated)]
        "netflix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'netflix' is deprecated.").print(py);
            }
            Some(icons::NETFLIX)
        }
        "flask-minus" => Some(icons::FLASK_MINUS),
        "weather-sunset-up" => Some(icons::WEATHER_SUNSET_UP),
        "form-dropdown" => Some(icons::FORM_DROPDOWN),
        "sun-wireless-outline" => Some(icons::SUN_WIRELESS_OUTLINE),
        "video-input-antenna" => Some(icons::VIDEO_INPUT_ANTENNA),
        "gondola" => Some(icons::GONDOLA),
        "email-mark-as-unread" => Some(icons::EMAIL_MARK_AS_UNREAD),
        "ray-start-vertex-end" => Some(icons::RAY_START_VERTEX_END),
        "generator-portable" => Some(icons::GENERATOR_PORTABLE),
        "axis-x-rotate-clockwise" => Some(icons::AXIS_X_ROTATE_CLOCKWISE),
        "table-arrow-down" => Some(icons::TABLE_ARROW_DOWN),
        "lotion-outline" => Some(icons::LOTION_OUTLINE),
        "numeric-10-circle-outline" => Some(icons::NUMERIC_10_CIRCLE_OUTLINE),
        "thought-bubble-outline" => Some(icons::THOUGHT_BUBBLE_OUTLINE),
        "chili-alert" => Some(icons::CHILI_ALERT),
        "fleur-de-lis" => Some(icons::FLEUR_DE_LIS),
        "cigar-off" => Some(icons::CIGAR_OFF),
        "owl" => Some(icons::OWL),
        "button-cursor" => Some(icons::BUTTON_CURSOR),
        "heart-box" => Some(icons::HEART_BOX),
        "arrow-up-bold-circle-outline" => Some(icons::ARROW_UP_BOLD_CIRCLE_OUTLINE),
        "airplane-marker" => Some(icons::AIRPLANE_MARKER),
        "chevron-right-box-outline" => Some(icons::CHEVRON_RIGHT_BOX_OUTLINE),
        "robot-vacuum-alert" => Some(icons::ROBOT_VACUUM_ALERT),
        "set-split" => Some(icons::SET_SPLIT),
        "window-minimize" => Some(icons::WINDOW_MINIMIZE),
        "alpha-q" => Some(icons::ALPHA_Q),
        _ => None,
    }
}
