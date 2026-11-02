// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_2(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "puzzle-heart" => Some(icons::PUZZLE_HEART),
        "email-edit-outline" => Some(icons::EMAIL_EDIT_OUTLINE),
        "hexagon-multiple-outline" => Some(icons::HEXAGON_MULTIPLE_OUTLINE),
        "bag-suitcase-off-outline" => Some(icons::BAG_SUITCASE_OFF_OUTLINE),
        "car-door" => Some(icons::CAR_DOOR),
        "cursor-default-click" => Some(icons::CURSOR_DEFAULT_CLICK),
        "phone-voip" => Some(icons::PHONE_VOIP),
        "wrench-cog-outline" => Some(icons::WRENCH_COG_OUTLINE),
        "movie-open-settings-outline" => Some(icons::MOVIE_OPEN_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        "cards-playing-spade" => Some(icons::CARDS_PLAYING_SPADE),
        "devices" => Some(icons::DEVICES),
        "tunnel" => Some(icons::TUNNEL),
        "alpha-i-circle" => Some(icons::ALPHA_I_CIRCLE),
        "image-auto-adjust" => Some(icons::IMAGE_AUTO_ADJUST),
        "dice-3-outline" => Some(icons::DICE_3_OUTLINE),
        "clipboard-remove-outline" => Some(icons::CLIPBOARD_REMOVE_OUTLINE),
        "bug-check" => Some(icons::BUG_CHECK),
        "ribbon" => Some(icons::RIBBON),
        "home-thermometer" => Some(icons::HOME_THERMOMETER),
        "message-cog" => Some(icons::MESSAGE_COG),
        "credit-card-refresh" => Some(icons::CREDIT_CARD_REFRESH),
        "alpha-f-box" => Some(icons::ALPHA_F_BOX),
        "camera-switch-outline" => Some(icons::CAMERA_SWITCH_OUTLINE),
        "network-outline" => Some(icons::NETWORK_OUTLINE),
        "coffee-to-go-outline" => Some(icons::COFFEE_TO_GO_OUTLINE),
        "curtains-closed" => Some(icons::CURTAINS_CLOSED),
        "minus-circle-multiple-outline" => Some(icons::MINUS_CIRCLE_MULTIPLE_OUTLINE),
        "arrow-expand-down" => Some(icons::ARROW_EXPAND_DOWN),
        "upload-circle" => Some(icons::UPLOAD_CIRCLE),
        "tag-remove" => Some(icons::TAG_REMOVE),
        "relation-one-to-only-one" => Some(icons::RELATION_ONE_TO_ONLY_ONE),
        "format-quote-open" => Some(icons::FORMAT_QUOTE_OPEN),
        "decimal-comma-increase" => Some(icons::DECIMAL_COMMA_INCREASE),
        "truck-alert" => Some(icons::TRUCK_ALERT),
        "file-lock-outline" => Some(icons::FILE_LOCK_OUTLINE),
        "chevron-right-circle" => Some(icons::CHEVRON_RIGHT_CIRCLE),
        "comment-off-outline" => Some(icons::COMMENT_OFF_OUTLINE),
        "ice-cream-off" => Some(icons::ICE_CREAM_OFF),
        "sort-calendar-ascending" => Some(icons::SORT_CALENDAR_ASCENDING),
        "gamepad-circle" => Some(icons::GAMEPAD_CIRCLE),
        "fit-to-page" => Some(icons::FIT_TO_PAGE),
        "message-settings" => Some(icons::MESSAGE_SETTINGS),
        "cart-off" => Some(icons::CART_OFF),
        "alien" => Some(icons::ALIEN),
        "file-clock-outline" => Some(icons::FILE_CLOCK_OUTLINE),
        "drawing-box" => Some(icons::DRAWING_BOX),
        "file-video-outline" => Some(icons::FILE_VIDEO_OUTLINE),
        "tag-arrow-up" => Some(icons::TAG_ARROW_UP),
        "archive-search-outline" => Some(icons::ARCHIVE_SEARCH_OUTLINE),
        "diameter-variant" => Some(icons::DIAMETER_VARIANT),
        "redo-variant" => Some(icons::REDO_VARIANT),
        "theater" => Some(icons::THEATER),
        "washing-machine-off" => Some(icons::WASHING_MACHINE_OFF),
        "progress-wrench" => Some(icons::PROGRESS_WRENCH),
        "human-baby-changing-table" => Some(icons::HUMAN_BABY_CHANGING_TABLE),
        "alpha-b-box" => Some(icons::ALPHA_B_BOX),
        "hamburger-off" => Some(icons::HAMBURGER_OFF),
        "sitemap" => Some(icons::SITEMAP),
        "format-title" => Some(icons::FORMAT_TITLE),
        "account-alert" => Some(icons::ACCOUNT_ALERT),
        "chevron-double-down" => Some(icons::CHEVRON_DOUBLE_DOWN),
        "arrow-expand" => Some(icons::ARROW_EXPAND),
        "database-eye-off-outline" => Some(icons::DATABASE_EYE_OFF_OUTLINE),
        "emoticon-devil-outline" => Some(icons::EMOTICON_DEVIL_OUTLINE),
        "human-handsdown" => Some(icons::HUMAN_HANDSDOWN),
        "format-line-style" => Some(icons::FORMAT_LINE_STYLE),
        "sort-alphabetical-ascending-variant" => Some(icons::SORT_ALPHABETICAL_ASCENDING_VARIANT),
        "car-seat-cooler" => Some(icons::CAR_SEAT_COOLER),
        "align-vertical-bottom" => Some(icons::ALIGN_VERTICAL_BOTTOM),
        "battery-charging-wireless-80" => Some(icons::BATTERY_CHARGING_WIRELESS_80),
        "transition" => Some(icons::TRANSITION),
        "file-image-marker-outline" => Some(icons::FILE_IMAGE_MARKER_OUTLINE),
        "cards" => Some(icons::CARDS),
        "heart-cog" => Some(icons::HEART_COG),
        "cards-playing-club" => Some(icons::CARDS_PLAYING_CLUB),
        "printer-pos-play" => Some(icons::PRINTER_POS_PLAY),
        #[allow(deprecated)]
        "android" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android' is deprecated.").print(py);
            }
            Some(icons::ANDROID)
        }
        "map-marker-right-outline" => Some(icons::MAP_MARKER_RIGHT_OUTLINE),
        "garage-open-variant" => Some(icons::GARAGE_OPEN_VARIANT),
        "paperclip-plus" => Some(icons::PAPERCLIP_PLUS),
        "thumb-down-outline" => Some(icons::THUMB_DOWN_OUTLINE),
        "temperature-celsius" => Some(icons::TEMPERATURE_CELSIUS),
        "border-left" => Some(icons::BORDER_LEFT),
        "file-move-outline" => Some(icons::FILE_MOVE_OUTLINE),
        "checkbox-blank-outline" => Some(icons::CHECKBOX_BLANK_OUTLINE),
        "candy" => Some(icons::CANDY),
        "waves-arrow-left" => Some(icons::WAVES_ARROW_LEFT),
        "circle-off-outline" => Some(icons::CIRCLE_OFF_OUTLINE),
        "keyboard-off" => Some(icons::KEYBOARD_OFF),
        "book-open-variant" => Some(icons::BOOK_OPEN_VARIANT),
        "home-percent-outline" => Some(icons::HOME_PERCENT_OUTLINE),
        "gas-burner" => Some(icons::GAS_BURNER),
        "folder-alert-outline" => Some(icons::FOLDER_ALERT_OUTLINE),
        "elevation-decline" => Some(icons::ELEVATION_DECLINE),
        "tunnel-outline" => Some(icons::TUNNEL_OUTLINE),
        "stretch-to-page" => Some(icons::STRETCH_TO_PAGE),
        "cards-playing-spade-outline" => Some(icons::CARDS_PLAYING_SPADE_OUTLINE),
        "diameter-outline" => Some(icons::DIAMETER_OUTLINE),
        "new-box" => Some(icons::NEW_BOX),
        "image-broken" => Some(icons::IMAGE_BROKEN),
        "percent-outline" => Some(icons::PERCENT_OUTLINE),
        "hospital-box" => Some(icons::HOSPITAL_BOX),
        "all-inclusive" => Some(icons::ALL_INCLUSIVE),
        "label-off" => Some(icons::LABEL_OFF),
        "moped-outline" => Some(icons::MOPED_OUTLINE),
        "map-marker-radius" => Some(icons::MAP_MARKER_RADIUS),
        "file-document-remove" => Some(icons::FILE_DOCUMENT_REMOVE),
        "clipboard-multiple-outline" => Some(icons::CLIPBOARD_MULTIPLE_OUTLINE),
        "shield-lock-open" => Some(icons::SHIELD_LOCK_OPEN),
        "wallet-giftcard" => Some(icons::WALLET_GIFTCARD),
        "movie-open-plus-outline" => Some(icons::MOVIE_OPEN_PLUS_OUTLINE),
        "sim-outline" => Some(icons::SIM_OUTLINE),
        "file-word-outline" => Some(icons::FILE_WORD_OUTLINE),
        "progress-upload" => Some(icons::PROGRESS_UPLOAD),
        "mixed-martial-arts" => Some(icons::MIXED_MARTIAL_ARTS),
        "emoticon-remove" => Some(icons::EMOTICON_REMOVE),
        "thermometer-lines" => Some(icons::THERMOMETER_LINES),
        "calendar-multiple-check" => Some(icons::CALENDAR_MULTIPLE_CHECK),
        "folder-arrow-left-right-outline" => Some(icons::FOLDER_ARROW_LEFT_RIGHT_OUTLINE),
        "text-shadow" => Some(icons::TEXT_SHADOW),
        "format-text-variant-outline" => Some(icons::FORMAT_TEXT_VARIANT_OUTLINE),
        "yoga" => Some(icons::YOGA),
        "bowl-mix" => Some(icons::BOWL_MIX),
        "power-socket-us" => Some(icons::POWER_SOCKET_US),
        "pause-octagon" => Some(icons::PAUSE_OCTAGON),
        "projector" => Some(icons::PROJECTOR),
        "camera-wireless-outline" => Some(icons::CAMERA_WIRELESS_OUTLINE),
        "circle-slice-6" => Some(icons::CIRCLE_SLICE_6),
        #[allow(deprecated)]
        "twitter" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitter' is deprecated.").print(py);
            }
            Some(icons::TWITTER)
        }
        "baseball-bat" => Some(icons::BASEBALL_BAT),
        "information-variant-box" => Some(icons::INFORMATION_VARIANT_BOX),
        "toy-brick-minus-outline" => Some(icons::TOY_BRICK_MINUS_OUTLINE),
        "progress-question" => Some(icons::PROGRESS_QUESTION),
        "hexagon-slice-4" => Some(icons::HEXAGON_SLICE_4),
        "sort-variant-lock" => Some(icons::SORT_VARIANT_LOCK),
        "ray-start" => Some(icons::RAY_START),
        "monitor-vertical" => Some(icons::MONITOR_VERTICAL),
        "doorbell" => Some(icons::DOORBELL),
        "chili-medium" => Some(icons::CHILI_MEDIUM),
        "radiology-box" => Some(icons::RADIOLOGY_BOX),
        "assistant" => Some(icons::ASSISTANT),
        #[allow(deprecated)]
        "qqchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qqchat' is deprecated.").print(py);
            }
            Some(icons::QQCHAT)
        }
        "close-circle-outline" => Some(icons::CLOSE_CIRCLE_OUTLINE),
        "desk-lamp" => Some(icons::DESK_LAMP),
        "account-outline" => Some(icons::ACCOUNT_OUTLINE),
        "radius" => Some(icons::RADIUS),
        "battery-plus-variant" => Some(icons::BATTERY_PLUS_VARIANT),
        "briefcase-arrow-up-down-outline" => Some(icons::BRIEFCASE_ARROW_UP_DOWN_OUTLINE),
        "elevator-passenger-off-outline" => Some(icons::ELEVATOR_PASSENGER_OFF_OUTLINE),
        "head-cog" => Some(icons::HEAD_COG),
        "folder-multiple-outline" => Some(icons::FOLDER_MULTIPLE_OUTLINE),
        "card-account-phone" => Some(icons::CARD_ACCOUNT_PHONE),
        "format-wrap-square" => Some(icons::FORMAT_WRAP_SQUARE),
        "cursor-default-outline" => Some(icons::CURSOR_DEFAULT_OUTLINE),
        "printer-settings" => Some(icons::PRINTER_SETTINGS),
        "basket-fill" => Some(icons::BASKET_FILL),
        "shield-half-full" => Some(icons::SHIELD_HALF_FULL),
        "toy-brick-marker-outline" => Some(icons::TOY_BRICK_MARKER_OUTLINE),
        "shield-bug-outline" => Some(icons::SHIELD_BUG_OUTLINE),
        "screw-flat-top" => Some(icons::SCREW_FLAT_TOP),
        "truck-plus" => Some(icons::TRUCK_PLUS),
        "format-vertical-align-center" => Some(icons::FORMAT_VERTICAL_ALIGN_CENTER),
        "chart-box-plus-outline" => Some(icons::CHART_BOX_PLUS_OUTLINE),
        "link-variant-plus" => Some(icons::LINK_VARIANT_PLUS),
        "upload-box-outline" => Some(icons::UPLOAD_BOX_OUTLINE),
        "sort-clock-descending-outline" => Some(icons::SORT_CLOCK_DESCENDING_OUTLINE),
        "audio-input-stereo-minijack" => Some(icons::AUDIO_INPUT_STEREO_MINIJACK),
        "gesture" => Some(icons::GESTURE),
        "calendar-question-outline" => Some(icons::CALENDAR_QUESTION_OUTLINE),
        "robot-happy" => Some(icons::ROBOT_HAPPY),
        "texture-box" => Some(icons::TEXTURE_BOX),
        "timer-cancel" => Some(icons::TIMER_CANCEL),
        "chevron-up-box" => Some(icons::CHEVRON_UP_BOX),
        "fast-forward-outline" => Some(icons::FAST_FORWARD_OUTLINE),
        "file-rotate-right-outline" => Some(icons::FILE_ROTATE_RIGHT_OUTLINE),
        "head-flash-outline" => Some(icons::HEAD_FLASH_OUTLINE),
        "tumble-dryer" => Some(icons::TUMBLE_DRYER),
        #[allow(deprecated)]
        "language-r" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-r' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_R)
        }
        "test-tube-off" => Some(icons::TEST_TUBE_OFF),
        "robot-angry" => Some(icons::ROBOT_ANGRY),
        "format-text-rotation-angle-up" => Some(icons::FORMAT_TEXT_ROTATION_ANGLE_UP),
        "timer-lock" => Some(icons::TIMER_LOCK),
        "menu-right" => Some(icons::MENU_RIGHT),
        "alpha-w-box" => Some(icons::ALPHA_W_BOX),
        "slot-machine-outline" => Some(icons::SLOT_MACHINE_OUTLINE),
        "sort-descending" => Some(icons::SORT_DESCENDING),
        "bookmark-multiple" => Some(icons::BOOKMARK_MULTIPLE),
        "window-shutter-open" => Some(icons::WINDOW_SHUTTER_OPEN),
        "invoice-check-outline" => Some(icons::INVOICE_CHECK_OUTLINE),
        "flask-empty-remove-outline" => Some(icons::FLASK_EMPTY_REMOVE_OUTLINE),
        "ceiling-fan" => Some(icons::CEILING_FAN),
        "message-star" => Some(icons::MESSAGE_STAR),
        "harddisk" => Some(icons::HARDDISK),
        "label-percent" => Some(icons::LABEL_PERCENT),
        "newspaper-variant-outline" => Some(icons::NEWSPAPER_VARIANT_OUTLINE),
        "face-woman-shimmer-outline" => Some(icons::FACE_WOMAN_SHIMMER_OUTLINE),
        "signal-cellular-2" => Some(icons::SIGNAL_CELLULAR_2),
        #[allow(deprecated)]
        "webrtc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webrtc' is deprecated.").print(py);
            }
            Some(icons::WEBRTC)
        }
        "relation-many-to-one-or-many" => Some(icons::RELATION_MANY_TO_ONE_OR_MANY),
        _ => None,
    }
}
