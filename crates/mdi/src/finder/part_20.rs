// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_20(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "tally-mark-1" => Some(icons::TALLY_MARK_1),
        #[allow(deprecated)]
        "box" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'box' is deprecated.").print(py);
            }
            Some(icons::BOX)
        }
        "tape-drive" => Some(icons::TAPE_DRIVE),
        "comment-question" => Some(icons::COMMENT_QUESTION),
        "square-small" => Some(icons::SQUARE_SMALL),
        "server-security" => Some(icons::SERVER_SECURITY),
        "car-multiple" => Some(icons::CAR_MULTIPLE),
        "chat-processing" => Some(icons::CHAT_PROCESSING),
        "gas-station" => Some(icons::GAS_STATION),
        "cash-check" => Some(icons::CASH_CHECK),
        "fit-to-screen-outline" => Some(icons::FIT_TO_SCREEN_OUTLINE),
        "timeline-remove" => Some(icons::TIMELINE_REMOVE),
        "shoe-ballet" => Some(icons::SHOE_BALLET),
        "cloud-refresh-variant" => Some(icons::CLOUD_REFRESH_VARIANT),
        "phone-classic" => Some(icons::PHONE_CLASSIC),
        "gesture-tap-button" => Some(icons::GESTURE_TAP_BUTTON),
        "prescription" => Some(icons::PRESCRIPTION),
        "fan-remove" => Some(icons::FAN_REMOVE),
        "printer-pos-wrench" => Some(icons::PRINTER_POS_WRENCH),
        "arrow-u-up-left-bold" => Some(icons::ARROW_U_UP_LEFT_BOLD),
        "comment-multiple" => Some(icons::COMMENT_MULTIPLE),
        "grave-stone" => Some(icons::GRAVE_STONE),
        "medication" => Some(icons::MEDICATION),
        "palette-swatch" => Some(icons::PALETTE_SWATCH),
        "shield-sun" => Some(icons::SHIELD_SUN),
        "human-handsdown" => Some(icons::HUMAN_HANDSDOWN),
        "thermometer-water" => Some(icons::THERMOMETER_WATER),
        "equal-box" => Some(icons::EQUAL_BOX),
        "format-underline-wavy" => Some(icons::FORMAT_UNDERLINE_WAVY),
        "school" => Some(icons::SCHOOL),
        "ray-start-vertex-end" => Some(icons::RAY_START_VERTEX_END),
        "key-change" => Some(icons::KEY_CHANGE),
        "printer-pos-outline" => Some(icons::PRINTER_POS_OUTLINE),
        "arrow-bottom-left-thin-circle-outline" => {
            Some(icons::ARROW_BOTTOM_LEFT_THIN_CIRCLE_OUTLINE)
        }
        "puzzle-star" => Some(icons::PUZZLE_STAR),
        "head-question-outline" => Some(icons::HEAD_QUESTION_OUTLINE),
        #[allow(deprecated)]
        "google-circles-communities" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'google-circles-communities' is deprecated.",
                )
                .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_COMMUNITIES)
        }
        "car-battery" => Some(icons::CAR_BATTERY),
        "toilet" => Some(icons::TOILET),
        "set-split" => Some(icons::SET_SPLIT),
        "autorenew-off" => Some(icons::AUTORENEW_OFF),
        "cookie-alert-outline" => Some(icons::COOKIE_ALERT_OUTLINE),
        "door-closed-cancel" => Some(icons::DOOR_CLOSED_CANCEL),
        "arrow-top-left-thin" => Some(icons::ARROW_TOP_LEFT_THIN),
        "volume-medium" => Some(icons::VOLUME_MEDIUM),
        "watch-export" => Some(icons::WATCH_EXPORT),
        "data-matrix-scan" => Some(icons::DATA_MATRIX_SCAN),
        "web-off" => Some(icons::WEB_OFF),
        "virus-off" => Some(icons::VIRUS_OFF),
        "molecule-co2" => Some(icons::MOLECULE_CO2),
        "remote-tv" => Some(icons::REMOTE_TV),
        "arrange-bring-to-front" => Some(icons::ARRANGE_BRING_TO_FRONT),
        "webcam-off" => Some(icons::WEBCAM_OFF),
        "billboard" => Some(icons::BILLBOARD),
        "battery-50" => Some(icons::BATTERY_50),
        "hexagon-slice-1" => Some(icons::HEXAGON_SLICE_1),
        "information-box-outline" => Some(icons::INFORMATION_BOX_OUTLINE),
        "briefcase-arrow-left-right" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT),
        "cellphone-information" => Some(icons::CELLPHONE_INFORMATION),
        "arrow-left-bold" => Some(icons::ARROW_LEFT_BOLD),
        "arrow-top-right-bottom-left" => Some(icons::ARROW_TOP_RIGHT_BOTTOM_LEFT),
        "bookmark-box" => Some(icons::BOOKMARK_BOX),
        "transfer-down" => Some(icons::TRANSFER_DOWN),
        "shape" => Some(icons::SHAPE),
        "emoticon-devil" => Some(icons::EMOTICON_DEVIL),
        "movie-settings-outline" => Some(icons::MOVIE_SETTINGS_OUTLINE),
        "filter-multiple" => Some(icons::FILTER_MULTIPLE),
        "bag-personal" => Some(icons::BAG_PERSONAL),
        "state-machine" => Some(icons::STATE_MACHINE),
        "arrow-right-drop-circle-outline" => Some(icons::ARROW_RIGHT_DROP_CIRCLE_OUTLINE),
        "backspace-outline" => Some(icons::BACKSPACE_OUTLINE),
        "skull-crossbones-outline" => Some(icons::SKULL_CROSSBONES_OUTLINE),
        "hub-outline" => Some(icons::HUB_OUTLINE),
        "radioactive-circle" => Some(icons::RADIOACTIVE_CIRCLE),
        "email-open-multiple-outline" => Some(icons::EMAIL_OPEN_MULTIPLE_OUTLINE),
        "close" => Some(icons::CLOSE),
        "light-flood-down" => Some(icons::LIGHT_FLOOD_DOWN),
        "vector-arrange-above" => Some(icons::VECTOR_ARRANGE_ABOVE),
        "face-man-outline" => Some(icons::FACE_MAN_OUTLINE),
        #[allow(deprecated)]
        "semantic-web" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'semantic-web' is deprecated.").print(py);
            }
            Some(icons::SEMANTIC_WEB)
        }
        "ring" => Some(icons::RING),
        "printer-wireless" => Some(icons::PRINTER_WIRELESS),
        "nature-outline" => Some(icons::NATURE_OUTLINE),
        "gamepad-right" => Some(icons::GAMEPAD_RIGHT),
        "cloud-sync-outline" => Some(icons::CLOUD_SYNC_OUTLINE),
        "dots-triangle" => Some(icons::DOTS_TRIANGLE),
        "radio-fm" => Some(icons::RADIO_FM),
        "invoice-send" => Some(icons::INVOICE_SEND),
        "bottle-tonic" => Some(icons::BOTTLE_TONIC),
        "delete-circle-outline" => Some(icons::DELETE_CIRCLE_OUTLINE),
        "shopping-outline" => Some(icons::SHOPPING_OUTLINE),
        "play-box-outline" => Some(icons::PLAY_BOX_OUTLINE),
        "cards-playing-heart" => Some(icons::CARDS_PLAYING_HEART),
        "api" => Some(icons::API),
        "upload-box-outline" => Some(icons::UPLOAD_BOX_OUTLINE),
        "timeline-minus" => Some(icons::TIMELINE_MINUS),
        "credit-card-marker-outline" => Some(icons::CREDIT_CARD_MARKER_OUTLINE),
        "table-column-plus-after" => Some(icons::TABLE_COLUMN_PLUS_AFTER),
        "disc-player" => Some(icons::DISC_PLAYER),
        "image-filter-none" => Some(icons::IMAGE_FILTER_NONE),
        "file-restore" => Some(icons::FILE_RESTORE),
        "rocket-outline" => Some(icons::ROCKET_OUTLINE),
        "hand-back-left-outline" => Some(icons::HAND_BACK_LEFT_OUTLINE),
        "tank" => Some(icons::TANK),
        "pen-remove" => Some(icons::PEN_REMOVE),
        "image-size-select-large" => Some(icons::IMAGE_SIZE_SELECT_LARGE),
        "wrench" => Some(icons::WRENCH),
        "kite" => Some(icons::KITE),
        "vibrate" => Some(icons::VIBRATE),
        "keyboard-close" => Some(icons::KEYBOARD_CLOSE),
        "image-outline" => Some(icons::IMAGE_OUTLINE),
        "rectangle-outline" => Some(icons::RECTANGLE_OUTLINE),
        "engine-off-outline" => Some(icons::ENGINE_OFF_OUTLINE),
        "share-all" => Some(icons::SHARE_ALL),
        "home-outline" => Some(icons::HOME_OUTLINE),
        "bell-circle-outline" => Some(icons::BELL_CIRCLE_OUTLINE),
        "power-plug-off-outline" => Some(icons::POWER_PLUG_OFF_OUTLINE),
        "fire-hydrant" => Some(icons::FIRE_HYDRANT),
        "file-refresh-outline" => Some(icons::FILE_REFRESH_OUTLINE),
        "car-light-high" => Some(icons::CAR_LIGHT_HIGH),
        "sim-outline" => Some(icons::SIM_OUTLINE),
        "file-minus" => Some(icons::FILE_MINUS),
        "hexagon-multiple" => Some(icons::HEXAGON_MULTIPLE),
        "vector-link" => Some(icons::VECTOR_LINK),
        "egg-off" => Some(icons::EGG_OFF),
        "polaroid" => Some(icons::POLAROID),
        "shield-account" => Some(icons::SHIELD_ACCOUNT),
        "trackpad" => Some(icons::TRACKPAD),
        "map-marker-outline" => Some(icons::MAP_MARKER_OUTLINE),
        "microphone-minus" => Some(icons::MICROPHONE_MINUS),
        "fan-off" => Some(icons::FAN_OFF),
        "home-remove-outline" => Some(icons::HOME_REMOVE_OUTLINE),
        "bone" => Some(icons::BONE),
        "database-edit" => Some(icons::DATABASE_EDIT),
        "medication-outline" => Some(icons::MEDICATION_OUTLINE),
        "signal-5g" => Some(icons::SIGNAL_5G),
        "printer-pos-cog-outline" => Some(icons::PRINTER_POS_COG_OUTLINE),
        "pasta" => Some(icons::PASTA),
        "phone-incoming-outgoing" => Some(icons::PHONE_INCOMING_OUTGOING),
        "timer-lock-open-outline" => Some(icons::TIMER_LOCK_OPEN_OUTLINE),
        "controller-classic-outline" => Some(icons::CONTROLLER_CLASSIC_OUTLINE),
        "lock-open-minus" => Some(icons::LOCK_OPEN_MINUS),
        "chat-question-outline" => Some(icons::CHAT_QUESTION_OUTLINE),
        "alert-rhombus" => Some(icons::ALERT_RHOMBUS),
        "aurora" => Some(icons::AURORA),
        "thermometer-auto" => Some(icons::THERMOMETER_AUTO),
        "pumpkin" => Some(icons::PUMPKIN),
        "movie-filter-outline" => Some(icons::MOVIE_FILTER_OUTLINE),
        "puzzle-edit-outline" => Some(icons::PUZZLE_EDIT_OUTLINE),
        "account-cog" => Some(icons::ACCOUNT_COG),
        "bookmark-check" => Some(icons::BOOKMARK_CHECK),
        "phone-paused" => Some(icons::PHONE_PAUSED),
        "circle-box" => Some(icons::CIRCLE_BOX),
        "arrow-down-left" => Some(icons::ARROW_DOWN_LEFT),
        "view-grid-outline" => Some(icons::VIEW_GRID_OUTLINE),
        "coach-lamp" => Some(icons::COACH_LAMP),
        "desk" => Some(icons::DESK),
        "arrow-all" => Some(icons::ARROW_ALL),
        "video-input-svideo" => Some(icons::VIDEO_INPUT_SVIDEO),
        "ticket-percent" => Some(icons::TICKET_PERCENT),
        "battery-70-bluetooth" => Some(icons::BATTERY_70_BLUETOOTH),
        "book-remove-multiple" => Some(icons::BOOK_REMOVE_MULTIPLE),
        "link-box-variant" => Some(icons::LINK_BOX_VARIANT),
        "gas-station-in-use" => Some(icons::GAS_STATION_IN_USE),
        "power-plug-off" => Some(icons::POWER_PLUG_OFF),
        "focus-auto" => Some(icons::FOCUS_AUTO),
        "equalizer-outline" => Some(icons::EQUALIZER_OUTLINE),
        "archive-remove-outline" => Some(icons::ARCHIVE_REMOVE_OUTLINE),
        "zip-box-outline" => Some(icons::ZIP_BOX_OUTLINE),
        "head-snowflake" => Some(icons::HEAD_SNOWFLAKE),
        "rotate-orbit" => Some(icons::ROTATE_ORBIT),
        "table-row-height" => Some(icons::TABLE_ROW_HEIGHT),
        "currency-fra" => Some(icons::CURRENCY_FRA),
        "package-check" => Some(icons::PACKAGE_CHECK),
        "skip-previous-circle-outline" => Some(icons::SKIP_PREVIOUS_CIRCLE_OUTLINE),
        "source-commit-start-next-local" => Some(icons::SOURCE_COMMIT_START_NEXT_LOCAL),
        "package-variant-closed" => Some(icons::PACKAGE_VARIANT_CLOSED),
        "format-text" => Some(icons::FORMAT_TEXT),
        "numeric-negative-1" => Some(icons::NUMERIC_NEGATIVE_1),
        "all-inclusive" => Some(icons::ALL_INCLUSIVE),
        "size-xxxl" => Some(icons::SIZE_XXXL),
        "office-building-remove-outline" => Some(icons::OFFICE_BUILDING_REMOVE_OUTLINE),
        "book-remove-multiple-outline" => Some(icons::BOOK_REMOVE_MULTIPLE_OUTLINE),
        "test-tube" => Some(icons::TEST_TUBE),
        "smoking-pipe" => Some(icons::SMOKING_PIPE),
        "chart-multiple" => Some(icons::CHART_MULTIPLE),
        #[allow(deprecated)]
        "microsoft-access" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-access' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ACCESS)
        }
        "broadcast" => Some(icons::BROADCAST),
        "folder-marker" => Some(icons::FOLDER_MARKER),
        "message-arrow-left" => Some(icons::MESSAGE_ARROW_LEFT),
        "gamepad-round-outline" => Some(icons::GAMEPAD_ROUND_OUTLINE),
        "battery-negative" => Some(icons::BATTERY_NEGATIVE),
        "folder-search" => Some(icons::FOLDER_SEARCH),
        "car-convertible" => Some(icons::CAR_CONVERTIBLE),
        "moped" => Some(icons::MOPED),
        "information-slab-box-outline" => Some(icons::INFORMATION_SLAB_BOX_OUTLINE),
        "fan-chevron-down" => Some(icons::FAN_CHEVRON_DOWN),
        "format-rotate-90" => Some(icons::FORMAT_ROTATE_90),
        "printer-pos-minus" => Some(icons::PRINTER_POS_MINUS),
        "clipboard-text-off-outline" => Some(icons::CLIPBOARD_TEXT_OFF_OUTLINE),
        _ => None,
    }
}
