// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_11(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "webcam-off" => Some(icons::WEBCAM_OFF),
        "molecule-co" => Some(icons::MOLECULE_CO),
        "pinwheel" => Some(icons::PINWHEEL),
        "map-clock-outline" => Some(icons::MAP_CLOCK_OUTLINE),
        "water-polo" => Some(icons::WATER_POLO),
        "smoke-detector-alert" => Some(icons::SMOKE_DETECTOR_ALERT),
        "image-off" => Some(icons::IMAGE_OFF),
        "cloud-check-variant-outline" => Some(icons::CLOUD_CHECK_VARIANT_OUTLINE),
        "beaker-question-outline" => Some(icons::BEAKER_QUESTION_OUTLINE),
        "file-swap-outline" => Some(icons::FILE_SWAP_OUTLINE),
        "home-edit-outline" => Some(icons::HOME_EDIT_OUTLINE),
        "power-settings" => Some(icons::POWER_SETTINGS),
        "cart-minus" => Some(icons::CART_MINUS),
        "map-search" => Some(icons::MAP_SEARCH),
        "cylinder-off" => Some(icons::CYLINDER_OFF),
        "percent-box-outline" => Some(icons::PERCENT_BOX_OUTLINE),
        "fruit-grapes-outline" => Some(icons::FRUIT_GRAPES_OUTLINE),
        "book-arrow-left" => Some(icons::BOOK_ARROW_LEFT),
        "mirror" => Some(icons::MIRROR),
        "truck-plus-outline" => Some(icons::TRUCK_PLUS_OUTLINE),
        "receipt-text-send-outline" => Some(icons::RECEIPT_TEXT_SEND_OUTLINE),
        "shopping" => Some(icons::SHOPPING),
        #[allow(deprecated)]
        "linux" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux' is deprecated.").print(py);
            }
            Some(icons::LINUX)
        }
        "water-remove-outline" => Some(icons::WATER_REMOVE_OUTLINE),
        "call-split" => Some(icons::CALL_SPLIT),
        "pan-left" => Some(icons::PAN_LEFT),
        "video-off" => Some(icons::VIDEO_OFF),
        "cloud-check-outline" => Some(icons::CLOUD_CHECK_OUTLINE),
        "apple-keyboard-command" => Some(icons::APPLE_KEYBOARD_COMMAND),
        "egg" => Some(icons::EGG),
        "eyedropper" => Some(icons::EYEDROPPER),
        "sim-alert-outline" => Some(icons::SIM_ALERT_OUTLINE),
        "door-sliding-open" => Some(icons::DOOR_SLIDING_OPEN),
        "bed-king" => Some(icons::BED_KING),
        "heart-cog" => Some(icons::HEART_COG),
        "ear-hearing-loop" => Some(icons::EAR_HEARING_LOOP),
        "bottle-tonic-skull" => Some(icons::BOTTLE_TONIC_SKULL),
        "billboard" => Some(icons::BILLBOARD),
        "minus-circle-multiple" => Some(icons::MINUS_CIRCLE_MULTIPLE),
        "puzzle" => Some(icons::PUZZLE),
        "watch-vibrate-off" => Some(icons::WATCH_VIBRATE_OFF),
        "download-off" => Some(icons::DOWNLOAD_OFF),
        "play-pause" => Some(icons::PLAY_PAUSE),
        "parachute-outline" => Some(icons::PARACHUTE_OUTLINE),
        "fridge-top" => Some(icons::FRIDGE_TOP),
        #[allow(deprecated)]
        "plex" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'plex' is deprecated.").print(py);
            }
            Some(icons::PLEX)
        }
        "router-network-wireless" => Some(icons::ROUTER_NETWORK_WIRELESS),
        "cookie-refresh" => Some(icons::COOKIE_REFRESH),
        "lock-open-alert-outline" => Some(icons::LOCK_OPEN_ALERT_OUTLINE),
        "rhombus-medium-outline" => Some(icons::RHOMBUS_MEDIUM_OUTLINE),
        "graph-outline" => Some(icons::GRAPH_OUTLINE),
        "home-alert" => Some(icons::HOME_ALERT),
        "rocket-launch" => Some(icons::ROCKET_LAUNCH),
        "book-heart" => Some(icons::BOOK_HEART),
        "dots-horizontal-circle-outline" => Some(icons::DOTS_HORIZONTAL_CIRCLE_OUTLINE),
        "box-shadow" => Some(icons::BOX_SHADOW),
        "clipboard-edit-outline" => Some(icons::CLIPBOARD_EDIT_OUTLINE),
        "briefcase-remove-outline" => Some(icons::BRIEFCASE_REMOVE_OUTLINE),
        "music-note-half-dotted" => Some(icons::MUSIC_NOTE_HALF_DOTTED),
        "message-off-outline" => Some(icons::MESSAGE_OFF_OUTLINE),
        #[allow(deprecated)]
        "disqus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'disqus' is deprecated.").print(py);
            }
            Some(icons::DISQUS)
        }
        "circle-slice-5" => Some(icons::CIRCLE_SLICE_5),
        #[allow(deprecated)]
        "angularjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'angularjs' is deprecated.").print(py);
            }
            Some(icons::ANGULARJS)
        }
        "format-text-variant-outline" => Some(icons::FORMAT_TEXT_VARIANT_OUTLINE),
        "content-save-alert-outline" => Some(icons::CONTENT_SAVE_ALERT_OUTLINE),
        "message-arrow-right" => Some(icons::MESSAGE_ARROW_RIGHT),
        "skull-crossbones-outline" => Some(icons::SKULL_CROSSBONES_OUTLINE),
        "kettle-outline" => Some(icons::KETTLE_OUTLINE),
        "bed-queen-outline" => Some(icons::BED_QUEEN_OUTLINE),
        "brush-off" => Some(icons::BRUSH_OFF),
        "border-none" => Some(icons::BORDER_NONE),
        "certificate" => Some(icons::CERTIFICATE),
        "table-headers-eye-off" => Some(icons::TABLE_HEADERS_EYE_OFF),
        "note-text-outline" => Some(icons::NOTE_TEXT_OUTLINE),
        "chemical-weapon" => Some(icons::CHEMICAL_WEAPON),
        "alpha-b-box-outline" => Some(icons::ALPHA_B_BOX_OUTLINE),
        "motion-play" => Some(icons::MOTION_PLAY),
        "coach-lamp" => Some(icons::COACH_LAMP),
        "alpha-h" => Some(icons::ALPHA_H),
        "broom" => Some(icons::BROOM),
        "alpha-p-box-outline" => Some(icons::ALPHA_P_BOX_OUTLINE),
        "dice-d6-outline" => Some(icons::DICE_D6_OUTLINE),
        "file-image-plus-outline" => Some(icons::FILE_IMAGE_PLUS_OUTLINE),
        "sim-off" => Some(icons::SIM_OFF),
        "panorama-wide-angle" => Some(icons::PANORAMA_WIDE_ANGLE),
        "thermometer" => Some(icons::THERMOMETER),
        "alpha-z-circle-outline" => Some(icons::ALPHA_Z_CIRCLE_OUTLINE),
        "firework-off" => Some(icons::FIREWORK_OFF),
        "information" => Some(icons::INFORMATION),
        "video-input-antenna" => Some(icons::VIDEO_INPUT_ANTENNA),
        "plus-circle-multiple-outline" => Some(icons::PLUS_CIRCLE_MULTIPLE_OUTLINE),
        "filter-off-outline" => Some(icons::FILTER_OFF_OUTLINE),
        "panorama-vertical-outline" => Some(icons::PANORAMA_VERTICAL_OUTLINE),
        #[allow(deprecated)]
        "font-awesome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'font-awesome' is deprecated.").print(py);
            }
            Some(icons::FONT_AWESOME)
        }
        "calendar-plus-outline" => Some(icons::CALENDAR_PLUS_OUTLINE),
        "link-circle" => Some(icons::LINK_CIRCLE),
        "book-remove-multiple" => Some(icons::BOOK_REMOVE_MULTIPLE),
        "text-box-minus" => Some(icons::TEXT_BOX_MINUS),
        "bed-single" => Some(icons::BED_SINGLE),
        "account-reactivate-outline" => Some(icons::ACCOUNT_REACTIVATE_OUTLINE),
        "airplane" => Some(icons::AIRPLANE),
        "chevron-down-circle" => Some(icons::CHEVRON_DOWN_CIRCLE),
        "sort-variant-lock" => Some(icons::SORT_VARIANT_LOCK),
        "battery-alert-bluetooth" => Some(icons::BATTERY_ALERT_BLUETOOTH),
        "filter-variant-plus" => Some(icons::FILTER_VARIANT_PLUS),
        "axis-y-rotate-clockwise" => Some(icons::AXIS_Y_ROTATE_CLOCKWISE),
        "star-four-points-circle" => Some(icons::STAR_FOUR_POINTS_CIRCLE),
        "lotion-plus-outline" => Some(icons::LOTION_PLUS_OUTLINE),
        "seat-legroom-normal" => Some(icons::SEAT_LEGROOM_NORMAL),
        "arrow-collapse-right" => Some(icons::ARROW_COLLAPSE_RIGHT),
        "hamburger-plus" => Some(icons::HAMBURGER_PLUS),
        "wifi-check" => Some(icons::WIFI_CHECK),
        "rhombus-split" => Some(icons::RHOMBUS_SPLIT),
        "mouse-move-up" => Some(icons::MOUSE_MOVE_UP),
        "airplane-remove" => Some(icons::AIRPLANE_REMOVE),
        "cookie-settings-outline" => Some(icons::COOKIE_SETTINGS_OUTLINE),
        "magnet" => Some(icons::MAGNET),
        "folder-refresh-outline" => Some(icons::FOLDER_REFRESH_OUTLINE),
        "beaker-alert" => Some(icons::BEAKER_ALERT),
        "package-down" => Some(icons::PACKAGE_DOWN),
        "cart-variant" => Some(icons::CART_VARIANT),
        "share-variant" => Some(icons::SHARE_VARIANT),
        "arrow-left-bold-circle-outline" => Some(icons::ARROW_LEFT_BOLD_CIRCLE_OUTLINE),
        "face-man" => Some(icons::FACE_MAN),
        "comment-minus-outline" => Some(icons::COMMENT_MINUS_OUTLINE),
        "coffee" => Some(icons::COFFEE),
        "robot-dead" => Some(icons::ROBOT_DEAD),
        "boom-gate-arrow-up" => Some(icons::BOOM_GATE_ARROW_UP),
        "currency-eth" => Some(icons::CURRENCY_ETH),
        "file-remove-outline" => Some(icons::FILE_REMOVE_OUTLINE),
        "prescription" => Some(icons::PRESCRIPTION),
        "bug" => Some(icons::BUG),
        "search-web" => Some(icons::SEARCH_WEB),
        "incognito-circle-off" => Some(icons::INCOGNITO_CIRCLE_OFF),
        "sticker-text-outline" => Some(icons::STICKER_TEXT_OUTLINE),
        "map-marker-star-outline" => Some(icons::MAP_MARKER_STAR_OUTLINE),
        "source-commit-start-next-local" => Some(icons::SOURCE_COMMIT_START_NEXT_LOCAL),
        "speaker-message" => Some(icons::SPEAKER_MESSAGE),
        "cog-transfer-outline" => Some(icons::COG_TRANSFER_OUTLINE),
        "cup-water" => Some(icons::CUP_WATER),
        #[allow(deprecated)]
        "soundcloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'soundcloud' is deprecated.").print(py);
            }
            Some(icons::SOUNDCLOUD)
        }
        "soldering-iron" => Some(icons::SOLDERING_IRON),
        "checkbox-blank" => Some(icons::CHECKBOX_BLANK),
        "account-check-outline" => Some(icons::ACCOUNT_CHECK_OUTLINE),
        "head-outline" => Some(icons::HEAD_OUTLINE),
        "folder-marker" => Some(icons::FOLDER_MARKER),
        "flask-off" => Some(icons::FLASK_OFF),
        "phone-settings" => Some(icons::PHONE_SETTINGS),
        "sack-percent" => Some(icons::SACK_PERCENT),
        #[allow(deprecated)]
        "microsoft-edge" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-edge' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_EDGE)
        }
        "zodiac-aries" => Some(icons::ZODIAC_ARIES),
        "airplane-plus" => Some(icons::AIRPLANE_PLUS),
        "key-variant" => Some(icons::KEY_VARIANT),
        "checkbox-blank-badge-outline" => Some(icons::CHECKBOX_BLANK_BADGE_OUTLINE),
        "panorama-variant-outline" => Some(icons::PANORAMA_VARIANT_OUTLINE),
        "account-switch-outline" => Some(icons::ACCOUNT_SWITCH_OUTLINE),
        "navigation-variant" => Some(icons::NAVIGATION_VARIANT),
        "human-greeting-proximity" => Some(icons::HUMAN_GREETING_PROXIMITY),
        "format-page-break" => Some(icons::FORMAT_PAGE_BREAK),
        "dice-d12" => Some(icons::DICE_D12),
        "racquetball" => Some(icons::RACQUETBALL),
        "card-bulleted-off" => Some(icons::CARD_BULLETED_OFF),
        "gamepad-circle-right" => Some(icons::GAMEPAD_CIRCLE_RIGHT),
        "sort-numeric-ascending" => Some(icons::SORT_NUMERIC_ASCENDING),
        "car-key" => Some(icons::CAR_KEY),
        "robot-outline" => Some(icons::ROBOT_OUTLINE),
        "briefcase-variant-off" => Some(icons::BRIEFCASE_VARIANT_OFF),
        "map-marker-alert" => Some(icons::MAP_MARKER_ALERT),
        "rotate-orbit" => Some(icons::ROTATE_ORBIT),
        "drag-horizontal" => Some(icons::DRAG_HORIZONTAL),
        "printer-pos-stop" => Some(icons::PRINTER_POS_STOP),
        "select-place" => Some(icons::SELECT_PLACE),
        "email-plus" => Some(icons::EMAIL_PLUS),
        "tag-check" => Some(icons::TAG_CHECK),
        "cloud-arrow-left" => Some(icons::CLOUD_ARROW_LEFT),
        "invoice-edit" => Some(icons::INVOICE_EDIT),
        "webcam" => Some(icons::WEBCAM),
        "skip-previous-outline" => Some(icons::SKIP_PREVIOUS_OUTLINE),
        "wind-power-outline" => Some(icons::WIND_POWER_OUTLINE),
        "printer-pos-pause-outline" => Some(icons::PRINTER_POS_PAUSE_OUTLINE),
        "controller" => Some(icons::CONTROLLER),
        "archive-arrow-down" => Some(icons::ARCHIVE_ARROW_DOWN),
        "pyramid" => Some(icons::PYRAMID),
        "triangle-outline" => Some(icons::TRIANGLE_OUTLINE),
        "video-plus-outline" => Some(icons::VIDEO_PLUS_OUTLINE),
        "printer-pos-check-outline" => Some(icons::PRINTER_POS_CHECK_OUTLINE),
        "arrow-down-bold-circle" => Some(icons::ARROW_DOWN_BOLD_CIRCLE),
        "newspaper-variant" => Some(icons::NEWSPAPER_VARIANT),
        #[allow(deprecated)]
        "rollupjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'rollupjs' is deprecated.").print(py);
            }
            Some(icons::ROLLUPJS)
        }
        "file-cancel-outline" => Some(icons::FILE_CANCEL_OUTLINE),
        "music-note-outline" => Some(icons::MUSIC_NOTE_OUTLINE),
        "food-off-outline" => Some(icons::FOOD_OFF_OUTLINE),
        "map-marker-remove-outline" => Some(icons::MAP_MARKER_REMOVE_OUTLINE),
        "kettle-alert-outline" => Some(icons::KETTLE_ALERT_OUTLINE),
        "deskphone" => Some(icons::DESKPHONE),
        "table-row-plus-after" => Some(icons::TABLE_ROW_PLUS_AFTER),
        "bookmark-box-outline" => Some(icons::BOOKMARK_BOX_OUTLINE),
        "table-settings" => Some(icons::TABLE_SETTINGS),
        "lock-open-minus" => Some(icons::LOCK_OPEN_MINUS),
        "publish" => Some(icons::PUBLISH),
        _ => None,
    }
}
