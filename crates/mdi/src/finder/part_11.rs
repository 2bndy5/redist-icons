// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_11(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "console-line" => Some(icons::CONSOLE_LINE),
        "train-car-container" => Some(icons::TRAIN_CAR_CONTAINER),
        "valve-closed" => Some(icons::VALVE_CLOSED),
        "folder-move-outline" => Some(icons::FOLDER_MOVE_OUTLINE),
        "printer-pos-edit-outline" => Some(icons::PRINTER_POS_EDIT_OUTLINE),
        "microsoft-xbox-controller-battery-unknown" => {
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_UNKNOWN)
        }
        #[allow(deprecated)]
        "google-classroom" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-classroom' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CLASSROOM)
        }
        "phone-outgoing" => Some(icons::PHONE_OUTGOING),
        "link-box" => Some(icons::LINK_BOX),
        "printer-pos-check" => Some(icons::PRINTER_POS_CHECK),
        "pitchfork" => Some(icons::PITCHFORK),
        "delete-forever" => Some(icons::DELETE_FOREVER),
        "soundbar" => Some(icons::SOUNDBAR),
        "numeric-0-box-multiple-outline" => Some(icons::NUMERIC_0_BOX_MULTIPLE_OUTLINE),
        "tally-mark-1" => Some(icons::TALLY_MARK_1),
        #[allow(deprecated)]
        "vlc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vlc' is deprecated.").print(py);
            }
            Some(icons::VLC)
        }
        "account-multiple-minus" => Some(icons::ACCOUNT_MULTIPLE_MINUS),
        "alpha-g-circle" => Some(icons::ALPHA_G_CIRCLE),
        "hospital-marker" => Some(icons::HOSPITAL_MARKER),
        "volume-low" => Some(icons::VOLUME_LOW),
        "alpha-e-circle-outline" => Some(icons::ALPHA_E_CIRCLE_OUTLINE),
        "cookie-cog" => Some(icons::COOKIE_COG),
        "qrcode-plus" => Some(icons::QRCODE_PLUS),
        "dice-6-outline" => Some(icons::DICE_6_OUTLINE),
        "fuse-off" => Some(icons::FUSE_OFF),
        "bell" => Some(icons::BELL),
        "ghost-outline" => Some(icons::GHOST_OUTLINE),
        "code-string" => Some(icons::CODE_STRING),
        "database-minus-outline" => Some(icons::DATABASE_MINUS_OUTLINE),
        "cup-outline" => Some(icons::CUP_OUTLINE),
        "pail" => Some(icons::PAIL),
        "signal-hspa" => Some(icons::SIGNAL_HSPA),
        "reply" => Some(icons::REPLY),
        "star-circle" => Some(icons::STAR_CIRCLE),
        "grid-large" => Some(icons::GRID_LARGE),
        "server-plus-outline" => Some(icons::SERVER_PLUS_OUTLINE),
        "checkbox-blank-circle" => Some(icons::CHECKBOX_BLANK_CIRCLE),
        "tooltip-account" => Some(icons::TOOLTIP_ACCOUNT),
        "silverware-fork" => Some(icons::SILVERWARE_FORK),
        "table-row-plus-after" => Some(icons::TABLE_ROW_PLUS_AFTER),
        "source-branch-refresh" => Some(icons::SOURCE_BRANCH_REFRESH),
        "alpha-q-circle-outline" => Some(icons::ALPHA_Q_CIRCLE_OUTLINE),
        "gamepad-right" => Some(icons::GAMEPAD_RIGHT),
        "feature-search-outline" => Some(icons::FEATURE_SEARCH_OUTLINE),
        "view-carousel-outline" => Some(icons::VIEW_CAROUSEL_OUTLINE),
        "brain" => Some(icons::BRAIN),
        "arrow-right-bold-circle" => Some(icons::ARROW_RIGHT_BOLD_CIRCLE),
        "trash-can" => Some(icons::TRASH_CAN),
        "list-box" => Some(icons::LIST_BOX),
        #[allow(deprecated)]
        "material-ui" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-ui' is deprecated.").print(py);
            }
            Some(icons::MATERIAL_UI)
        }
        "briefcase-eye" => Some(icons::BRIEFCASE_EYE),
        "relation-zero-or-one-to-one" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE),
        "format-italic" => Some(icons::FORMAT_ITALIC),
        "rabbit-variant-outline" => Some(icons::RABBIT_VARIANT_OUTLINE),
        "head-heart" => Some(icons::HEAD_HEART),
        "cloud-check" => Some(icons::CLOUD_CHECK),
        "play-box-multiple" => Some(icons::PLAY_BOX_MULTIPLE),
        #[allow(deprecated)]
        "language-html5" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-html5' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_HTML5)
        }
        "archive-search" => Some(icons::ARCHIVE_SEARCH),
        "broadcast-off" => Some(icons::BROADCAST_OFF),
        "skull-scan" => Some(icons::SKULL_SCAN),
        "eye-lock-open-outline" => Some(icons::EYE_LOCK_OPEN_OUTLINE),
        "rivet" => Some(icons::RIVET),
        "bulldozer" => Some(icons::BULLDOZER),
        "gas-station-off" => Some(icons::GAS_STATION_OFF),
        "stomach" => Some(icons::STOMACH),
        "relation-zero-or-one-to-one-or-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE_OR_MANY),
        #[allow(deprecated)]
        "language-php" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-php' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_PHP)
        }
        "bank-off-outline" => Some(icons::BANK_OFF_OUTLINE),
        "temperature-fahrenheit" => Some(icons::TEMPERATURE_FAHRENHEIT),
        "counter" => Some(icons::COUNTER),
        "alert-remove-outline" => Some(icons::ALERT_REMOVE_OUTLINE),
        "hand-clap-off" => Some(icons::HAND_CLAP_OFF),
        "cart-heart" => Some(icons::CART_HEART),
        "movie-off" => Some(icons::MOVIE_OFF),
        "alpha-n-box-outline" => Some(icons::ALPHA_N_BOX_OUTLINE),
        "package-variant-remove" => Some(icons::PACKAGE_VARIANT_REMOVE),
        "id-card" => Some(icons::ID_CARD),
        "gesture-tap" => Some(icons::GESTURE_TAP),
        "numeric-7-circle-outline" => Some(icons::NUMERIC_7_CIRCLE_OUTLINE),
        "account-box-edit-outline" => Some(icons::ACCOUNT_BOX_EDIT_OUTLINE),
        "heart-off-outline" => Some(icons::HEART_OFF_OUTLINE),
        "star-four-points-box-outline" => Some(icons::STAR_FOUR_POINTS_BOX_OUTLINE),
        #[allow(deprecated)]
        "disqus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'disqus' is deprecated.").print(py);
            }
            Some(icons::DISQUS)
        }
        "file-alert-outline" => Some(icons::FILE_ALERT_OUTLINE),
        "emoticon-angry-outline" => Some(icons::EMOTICON_ANGRY_OUTLINE),
        "boom-gate-up-outline" => Some(icons::BOOM_GATE_UP_OUTLINE),
        "golf" => Some(icons::GOLF),
        "account-off-outline" => Some(icons::ACCOUNT_OFF_OUTLINE),
        "awning-outline" => Some(icons::AWNING_OUTLINE),
        "star-outline" => Some(icons::STAR_OUTLINE),
        "ninja" => Some(icons::NINJA),
        "chart-bar" => Some(icons::CHART_BAR),
        "book-open-page-variant-outline" => Some(icons::BOOK_OPEN_PAGE_VARIANT_OUTLINE),
        "pencil-ruler" => Some(icons::PENCIL_RULER),
        "email-search" => Some(icons::EMAIL_SEARCH),
        "ethernet-off" => Some(icons::ETHERNET_OFF),
        "cloud-lock-outline" => Some(icons::CLOUD_LOCK_OUTLINE),
        "account-box-multiple" => Some(icons::ACCOUNT_BOX_MULTIPLE),
        "message-bookmark-outline" => Some(icons::MESSAGE_BOOKMARK_OUTLINE),
        "filter-variant-plus" => Some(icons::FILTER_VARIANT_PLUS),
        "email-edit" => Some(icons::EMAIL_EDIT),
        "format-header-decrease" => Some(icons::FORMAT_HEADER_DECREASE),
        "flask-empty-off" => Some(icons::FLASK_EMPTY_OFF),
        "view-column-outline" => Some(icons::VIEW_COLUMN_OUTLINE),
        "incognito-off" => Some(icons::INCOGNITO_OFF),
        "book-open-blank-variant" => Some(icons::BOOK_OPEN_BLANK_VARIANT),
        "keyboard-variant" => Some(icons::KEYBOARD_VARIANT),
        "head-check-outline" => Some(icons::HEAD_CHECK_OUTLINE),
        "content-paste" => Some(icons::CONTENT_PASTE),
        "wifi-strength-3-lock" => Some(icons::WIFI_STRENGTH_3_LOCK),
        "synagogue" => Some(icons::SYNAGOGUE),
        "battery-charging-wireless-alert" => Some(icons::BATTERY_CHARGING_WIRELESS_ALERT),
        "one-up" => Some(icons::ONE_UP),
        "phone-minus-outline" => Some(icons::PHONE_MINUS_OUTLINE),
        "spellcheck" => Some(icons::SPELLCHECK),
        "air-purifier" => Some(icons::AIR_PURIFIER),
        "attachment-plus" => Some(icons::ATTACHMENT_PLUS),
        "border-style" => Some(icons::BORDER_STYLE),
        "timer-lock-outline" => Some(icons::TIMER_LOCK_OUTLINE),
        "emoticon-excited" => Some(icons::EMOTICON_EXCITED),
        "bottle-tonic-plus" => Some(icons::BOTTLE_TONIC_PLUS),
        "image-filter-none" => Some(icons::IMAGE_FILTER_NONE),
        "plus-circle-multiple" => Some(icons::PLUS_CIRCLE_MULTIPLE),
        "panorama-variant-outline" => Some(icons::PANORAMA_VARIANT_OUTLINE),
        "import" => Some(icons::IMPORT),
        "television-ambient-light" => Some(icons::TELEVISION_AMBIENT_LIGHT),
        "alpha-d-box-outline" => Some(icons::ALPHA_D_BOX_OUTLINE),
        "train-car-hopper-covered" => Some(icons::TRAIN_CAR_HOPPER_COVERED),
        "earbuds-outline" => Some(icons::EARBUDS_OUTLINE),
        "cursor-default" => Some(icons::CURSOR_DEFAULT),
        "speaker-message" => Some(icons::SPEAKER_MESSAGE),
        "movie-cog-outline" => Some(icons::MOVIE_COG_OUTLINE),
        "postage-stamp" => Some(icons::POSTAGE_STAMP),
        "keyboard-tab" => Some(icons::KEYBOARD_TAB),
        "greater-than-or-equal" => Some(icons::GREATER_THAN_OR_EQUAL),
        "archive-check-outline" => Some(icons::ARCHIVE_CHECK_OUTLINE),
        "mower-on" => Some(icons::MOWER_ON),
        #[allow(deprecated)]
        "teamviewer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'teamviewer' is deprecated.").print(py);
            }
            Some(icons::TEAMVIEWER)
        }
        "home-export-outline" => Some(icons::HOME_EXPORT_OUTLINE),
        "details" => Some(icons::DETAILS),
        "eye-arrow-right-outline" => Some(icons::EYE_ARROW_RIGHT_OUTLINE),
        "alert-octagram" => Some(icons::ALERT_OCTAGRAM),
        "text-box-outline" => Some(icons::TEXT_BOX_OUTLINE),
        "pen-lock" => Some(icons::PEN_LOCK),
        "map-marker-check" => Some(icons::MAP_MARKER_CHECK),
        "account-multiple-outline" => Some(icons::ACCOUNT_MULTIPLE_OUTLINE),
        "sort-numeric-variant" => Some(icons::SORT_NUMERIC_VARIANT),
        "bookmark-outline" => Some(icons::BOOKMARK_OUTLINE),
        "tag-outline" => Some(icons::TAG_OUTLINE),
        "alpha-r-box" => Some(icons::ALPHA_R_BOX),
        "cellphone" => Some(icons::CELLPHONE),
        "collapse-all" => Some(icons::COLLAPSE_ALL),
        "cloud-question-outline" => Some(icons::CLOUD_QUESTION_OUTLINE),
        "alpha-m" => Some(icons::ALPHA_M),
        "tooltip-minus" => Some(icons::TOOLTIP_MINUS),
        "sim-off" => Some(icons::SIM_OFF),
        "briefcase" => Some(icons::BRIEFCASE),
        "rhombus-medium-outline" => Some(icons::RHOMBUS_MEDIUM_OUTLINE),
        "printer-pos-star" => Some(icons::PRINTER_POS_STAR),
        "filter-settings-outline" => Some(icons::FILTER_SETTINGS_OUTLINE),
        "home-modern" => Some(icons::HOME_MODERN),
        "paper-roll" => Some(icons::PAPER_ROLL),
        "square-root-box" => Some(icons::SQUARE_ROOT_BOX),
        "virus" => Some(icons::VIRUS),
        "food-kosher" => Some(icons::FOOD_KOSHER),
        "tag" => Some(icons::TAG),
        "moon-waning-gibbous" => Some(icons::MOON_WANING_GIBBOUS),
        "folder-zip" => Some(icons::FOLDER_ZIP),
        "contacts" => Some(icons::CONTACTS),
        "alarm" => Some(icons::ALARM),
        "subdirectory-arrow-right" => Some(icons::SUBDIRECTORY_ARROW_RIGHT),
        "credit-card-lock" => Some(icons::CREDIT_CARD_LOCK),
        "plus-minus" => Some(icons::PLUS_MINUS),
        "selection-multiple" => Some(icons::SELECTION_MULTIPLE),
        "baby-face" => Some(icons::BABY_FACE),
        "pyramid-off" => Some(icons::PYRAMID_OFF),
        "hand-wave" => Some(icons::HAND_WAVE),
        "text-box-multiple" => Some(icons::TEXT_BOX_MULTIPLE),
        "numeric-9-circle" => Some(icons::NUMERIC_9_CIRCLE),
        "checkbox-intermediate" => Some(icons::CHECKBOX_INTERMEDIATE),
        "invoice-send" => Some(icons::INVOICE_SEND),
        "exclamation" => Some(icons::EXCLAMATION),
        "door-closed-cancel" => Some(icons::DOOR_CLOSED_CANCEL),
        "bird" => Some(icons::BIRD),
        "invoice-text-check" => Some(icons::INVOICE_TEXT_CHECK),
        "message-draw" => Some(icons::MESSAGE_DRAW),
        "sofa-outline" => Some(icons::SOFA_OUTLINE),
        "message-processing" => Some(icons::MESSAGE_PROCESSING),
        "image-size-select-large" => Some(icons::IMAGE_SIZE_SELECT_LARGE),
        "seal-variant" => Some(icons::SEAL_VARIANT),
        "phone-incoming-outgoing" => Some(icons::PHONE_INCOMING_OUTGOING),
        "security-network" => Some(icons::SECURITY_NETWORK),
        "account-heart-outline" => Some(icons::ACCOUNT_HEART_OUTLINE),
        "table-arrow-right" => Some(icons::TABLE_ARROW_RIGHT),
        "pencil-off" => Some(icons::PENCIL_OFF),
        "movie-open-plus" => Some(icons::MOVIE_OPEN_PLUS),
        "file-percent" => Some(icons::FILE_PERCENT),
        "account-arrow-right-outline" => Some(icons::ACCOUNT_ARROW_RIGHT_OUTLINE),
        "sunglasses" => Some(icons::SUNGLASSES),
        _ => None,
    }
}
