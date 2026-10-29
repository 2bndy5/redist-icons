// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_21(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        #[allow(deprecated)]
        "nintendo-switch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-switch' is deprecated.")
                    .print(py);
            }
            Some(icons::NINTENDO_SWITCH)
        }
        "water-polo" => Some(icons::WATER_POLO),
        "database-search-outline" => Some(icons::DATABASE_SEARCH_OUTLINE),
        "mouse-off" => Some(icons::MOUSE_OFF),
        "screw-round-top" => Some(icons::SCREW_ROUND_TOP),
        "sticker-check" => Some(icons::STICKER_CHECK),
        "bookmark-plus-outline" => Some(icons::BOOKMARK_PLUS_OUTLINE),
        "home-off" => Some(icons::HOME_OFF),
        "invoice-text-clock" => Some(icons::INVOICE_TEXT_CLOCK),
        "car-brake-temperature" => Some(icons::CAR_BRAKE_TEMPERATURE),
        "file-lock-open-outline" => Some(icons::FILE_LOCK_OPEN_OUTLINE),
        "cards-playing-heart-outline" => Some(icons::CARDS_PLAYING_HEART_OUTLINE),
        "file-excel-outline" => Some(icons::FILE_EXCEL_OUTLINE),
        "source-fork" => Some(icons::SOURCE_FORK),
        "speaker" => Some(icons::SPEAKER),
        "boom-gate-arrow-down" => Some(icons::BOOM_GATE_ARROW_DOWN),
        "access-point-check" => Some(icons::ACCESS_POINT_CHECK),
        "camera-account" => Some(icons::CAMERA_ACCOUNT),
        "decagram" => Some(icons::DECAGRAM),
        "heart-box-outline" => Some(icons::HEART_BOX_OUTLINE),
        "car-door-lock-open" => Some(icons::CAR_DOOR_LOCK_OPEN),
        "account-music-outline" => Some(icons::ACCOUNT_MUSIC_OUTLINE),
        "vector-circle-variant" => Some(icons::VECTOR_CIRCLE_VARIANT),
        "lightbulb-alert" => Some(icons::LIGHTBULB_ALERT),
        "car-windshield" => Some(icons::CAR_WINDSHIELD),
        "skip-next-circle" => Some(icons::SKIP_NEXT_CIRCLE),
        "home-sound-in-outline" => Some(icons::HOME_SOUND_IN_OUTLINE),
        "hand-cycle" => Some(icons::HAND_CYCLE),
        "star-three-points" => Some(icons::STAR_THREE_POINTS),
        "credit-card-wireless-off" => Some(icons::CREDIT_CARD_WIRELESS_OFF),
        "map-marker-up" => Some(icons::MAP_MARKER_UP),
        "ideogram-cjk" => Some(icons::IDEOGRAM_CJK),
        "backspace-outline" => Some(icons::BACKSPACE_OUTLINE),
        "comment-check" => Some(icons::COMMENT_CHECK),
        "numeric-1-box-outline" => Some(icons::NUMERIC_1_BOX_OUTLINE),
        "invoice-arrow-right" => Some(icons::INVOICE_ARROW_RIGHT),
        "face-mask" => Some(icons::FACE_MASK),
        "motion-outline" => Some(icons::MOTION_OUTLINE),
        "home-lightbulb-outline" => Some(icons::HOME_LIGHTBULB_OUTLINE),
        "delete-circle-outline" => Some(icons::DELETE_CIRCLE_OUTLINE),
        "golf-cart" => Some(icons::GOLF_CART),
        "timer-stop" => Some(icons::TIMER_STOP),
        "printer-outline" => Some(icons::PRINTER_OUTLINE),
        "cart-arrow-down" => Some(icons::CART_ARROW_DOWN),
        "emoticon-kiss" => Some(icons::EMOTICON_KISS),
        "pill" => Some(icons::PILL),
        "pail-plus-outline" => Some(icons::PAIL_PLUS_OUTLINE),
        "battery-80" => Some(icons::BATTERY_80),
        "pentagon" => Some(icons::PENTAGON),
        "gamepad-variant" => Some(icons::GAMEPAD_VARIANT),
        "view-list" => Some(icons::VIEW_LIST),
        "bluetooth-off" => Some(icons::BLUETOOTH_OFF),
        "database-arrow-left" => Some(icons::DATABASE_ARROW_LEFT),
        "tab-plus" => Some(icons::TAB_PLUS),
        "window-shutter-settings" => Some(icons::WINDOW_SHUTTER_SETTINGS),
        "file-pdf-box" => Some(icons::FILE_PDF_BOX),
        "axis-x-arrow" => Some(icons::AXIS_X_ARROW),
        "water-circle" => Some(icons::WATER_CIRCLE),
        "certificate-outline" => Some(icons::CERTIFICATE_OUTLINE),
        "invoice-text-send-outline" => Some(icons::INVOICE_TEXT_SEND_OUTLINE),
        "led-strip-variant-off" => Some(icons::LED_STRIP_VARIANT_OFF),
        "boom-gate" => Some(icons::BOOM_GATE),
        "sort-bool-ascending-variant" => Some(icons::SORT_BOOL_ASCENDING_VARIANT),
        "currency-eur-off" => Some(icons::CURRENCY_EUR_OFF),
        "format-horizontal-align-left" => Some(icons::FORMAT_HORIZONTAL_ALIGN_LEFT),
        "bottle-wine-outline" => Some(icons::BOTTLE_WINE_OUTLINE),
        "timeline-minus-outline" => Some(icons::TIMELINE_MINUS_OUTLINE),
        "presentation-play" => Some(icons::PRESENTATION_PLAY),
        "magnify-close" => Some(icons::MAGNIFY_CLOSE),
        "arrow-bottom-left-bold-box-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_BOX_OUTLINE),
        "gesture-tap-box" => Some(icons::GESTURE_TAP_BOX),
        "book-plus-multiple-outline" => Some(icons::BOOK_PLUS_MULTIPLE_OUTLINE),
        "database-check-outline" => Some(icons::DATABASE_CHECK_OUTLINE),
        "email-fast-outline" => Some(icons::EMAIL_FAST_OUTLINE),
        "dna" => Some(icons::DNA),
        "table-row" => Some(icons::TABLE_ROW),
        "weather-partly-cloudy" => Some(icons::WEATHER_PARTLY_CLOUDY),
        "tooth" => Some(icons::TOOTH),
        "video-marker-outline" => Some(icons::VIDEO_MARKER_OUTLINE),
        "puzzle-heart-outline" => Some(icons::PUZZLE_HEART_OUTLINE),
        "gender-male-female-variant" => Some(icons::GENDER_MALE_FEMALE_VARIANT),
        "page-layout-footer" => Some(icons::PAGE_LAYOUT_FOOTER),
        "timeline-remove" => Some(icons::TIMELINE_REMOVE),
        "bookmark-music" => Some(icons::BOOKMARK_MUSIC),
        "ballot-recount-outline" => Some(icons::BALLOT_RECOUNT_OUTLINE),
        "hexagon-slice-2" => Some(icons::HEXAGON_SLICE_2),
        "battery-charging-30" => Some(icons::BATTERY_CHARGING_30),
        "chevron-triple-up" => Some(icons::CHEVRON_TRIPLE_UP),
        #[allow(deprecated)]
        "facebook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook' is deprecated.").print(py);
            }
            Some(icons::FACEBOOK)
        }
        "doorbell-video" => Some(icons::DOORBELL_VIDEO),
        "format-underline-wavy" => Some(icons::FORMAT_UNDERLINE_WAVY),
        "cloud-off-outline" => Some(icons::CLOUD_OFF_OUTLINE),
        "earth-box-off" => Some(icons::EARTH_BOX_OFF),
        "set-center-right" => Some(icons::SET_CENTER_RIGHT),
        #[allow(deprecated)]
        "drupal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'drupal' is deprecated.").print(py);
            }
            Some(icons::DRUPAL)
        }
        "movie-edit-outline" => Some(icons::MOVIE_EDIT_OUTLINE),
        "credit-card-marker" => Some(icons::CREDIT_CARD_MARKER),
        "chat-remove-outline" => Some(icons::CHAT_REMOVE_OUTLINE),
        "unicorn" => Some(icons::UNICORN),
        "folder-key-network" => Some(icons::FOLDER_KEY_NETWORK),
        "filter-off-outline" => Some(icons::FILTER_OFF_OUTLINE),
        "message-arrow-right-outline" => Some(icons::MESSAGE_ARROW_RIGHT_OUTLINE),
        "format-vertical-align-top" => Some(icons::FORMAT_VERTICAL_ALIGN_TOP),
        "receipt-outline" => Some(icons::RECEIPT_OUTLINE),
        "map-check" => Some(icons::MAP_CHECK),
        "candycane" => Some(icons::CANDYCANE),
        "pound-box" => Some(icons::POUND_BOX),
        "delete-restore" => Some(icons::DELETE_RESTORE),
        "beekeeper" => Some(icons::BEEKEEPER),
        #[allow(deprecated)]
        "reddit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'reddit' is deprecated.").print(py);
            }
            Some(icons::REDDIT)
        }
        "flag-variant-off-outline" => Some(icons::FLAG_VARIANT_OFF_OUTLINE),
        "phone-ring" => Some(icons::PHONE_RING),
        "weather-windy-variant" => Some(icons::WEATHER_WINDY_VARIANT),
        "skate-off" => Some(icons::SKATE_OFF),
        "arrow-top-left-bottom-right-bold" => Some(icons::ARROW_TOP_LEFT_BOTTOM_RIGHT_BOLD),
        "radiator-off" => Some(icons::RADIATOR_OFF),
        "cast-connected" => Some(icons::CAST_CONNECTED),
        "steering-off" => Some(icons::STEERING_OFF),
        "currency-mnt" => Some(icons::CURRENCY_MNT),
        "keyboard-f8" => Some(icons::KEYBOARD_F8),
        "bed-outline" => Some(icons::BED_OUTLINE),
        "message-check-outline" => Some(icons::MESSAGE_CHECK_OUTLINE),
        "cheese-off" => Some(icons::CHEESE_OFF),
        "layers" => Some(icons::LAYERS),
        "format-text-wrapping-wrap" => Some(icons::FORMAT_TEXT_WRAPPING_WRAP),
        "braille" => Some(icons::BRAILLE),
        "format-list-group-plus" => Some(icons::FORMAT_LIST_GROUP_PLUS),
        "airplane-minus" => Some(icons::AIRPLANE_MINUS),
        "format-text-wrapping-clip" => Some(icons::FORMAT_TEXT_WRAPPING_CLIP),
        "emoticon-dead" => Some(icons::EMOTICON_DEAD),
        "nuke" => Some(icons::NUKE),
        "car-coolant-level" => Some(icons::CAR_COOLANT_LEVEL),
        "syllabary-hiragana" => Some(icons::SYLLABARY_HIRAGANA),
        "email-open-multiple" => Some(icons::EMAIL_OPEN_MULTIPLE),
        "page-layout-header" => Some(icons::PAGE_LAYOUT_HEADER),
        "solar-power-variant-outline" => Some(icons::SOLAR_POWER_VARIANT_OUTLINE),
        "octagram-edit-outline" => Some(icons::OCTAGRAM_EDIT_OUTLINE),
        "format-align-left" => Some(icons::FORMAT_ALIGN_LEFT),
        "food-halal" => Some(icons::FOOD_HALAL),
        "comment-arrow-left" => Some(icons::COMMENT_ARROW_LEFT),
        "office-building-minus-outline" => Some(icons::OFFICE_BUILDING_MINUS_OUTLINE),
        "duck" => Some(icons::DUCK),
        "countertop-outline" => Some(icons::COUNTERTOP_OUTLINE),
        "comma-circle" => Some(icons::COMMA_CIRCLE),
        "dog-side" => Some(icons::DOG_SIDE),
        "note-alert-outline" => Some(icons::NOTE_ALERT_OUTLINE),
        "basketball-hoop" => Some(icons::BASKETBALL_HOOP),
        "arrow-top-right-thin-circle-outline" => Some(icons::ARROW_TOP_RIGHT_THIN_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "microsoft-dynamics-365" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-dynamics-365' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_DYNAMICS_365)
        }
        "emoticon-lol-outline" => Some(icons::EMOTICON_LOL_OUTLINE),
        "floor-plan" => Some(icons::FLOOR_PLAN),
        "navigation-variant" => Some(icons::NAVIGATION_VARIANT),
        "image-frame" => Some(icons::IMAGE_FRAME),
        "calendar-expand-horizontal" => Some(icons::CALENDAR_EXPAND_HORIZONTAL),
        "image-size-select-small" => Some(icons::IMAGE_SIZE_SELECT_SMALL),
        "file-gif-box" => Some(icons::FILE_GIF_BOX),
        "table-arrow-left" => Some(icons::TABLE_ARROW_LEFT),
        "help-circle" => Some(icons::HELP_CIRCLE),
        "cabin-a-frame" => Some(icons::CABIN_A_FRAME),
        "account-cowboy-hat" => Some(icons::ACCOUNT_COWBOY_HAT),
        "arrow-right" => Some(icons::ARROW_RIGHT),
        "select-all" => Some(icons::SELECT_ALL),
        "earth-minus" => Some(icons::EARTH_MINUS),
        "head-alert" => Some(icons::HEAD_ALERT),
        "microphone-variant-off" => Some(icons::MICROPHONE_VARIANT_OFF),
        "folder-off" => Some(icons::FOLDER_OFF),
        "panorama-vertical" => Some(icons::PANORAMA_VERTICAL),
        "send-lock-outline" => Some(icons::SEND_LOCK_OUTLINE),
        "account-question-outline" => Some(icons::ACCOUNT_QUESTION_OUTLINE),
        "vacuum" => Some(icons::VACUUM),
        "space-station" => Some(icons::SPACE_STATION),
        #[allow(deprecated)]
        "twitch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitch' is deprecated.").print(py);
            }
            Some(icons::TWITCH)
        }
        "checkbox-marked-outline" => Some(icons::CHECKBOX_MARKED_OUTLINE),
        "file-edit" => Some(icons::FILE_EDIT),
        "select-remove" => Some(icons::SELECT_REMOVE),
        "account-school" => Some(icons::ACCOUNT_SCHOOL),
        "archive-cog" => Some(icons::ARCHIVE_COG),
        "account-credit-card" => Some(icons::ACCOUNT_CREDIT_CARD),
        "format-text-rotation-vertical" => Some(icons::FORMAT_TEXT_ROTATION_VERTICAL),
        "movie-open-minus" => Some(icons::MOVIE_OPEN_MINUS),
        "key-chain-variant" => Some(icons::KEY_CHAIN_VARIANT),
        "hexagram" => Some(icons::HEXAGRAM),
        "flag-remove-outline" => Some(icons::FLAG_REMOVE_OUTLINE),
        "mailbox-open-outline" => Some(icons::MAILBOX_OPEN_OUTLINE),
        "diving-helmet" => Some(icons::DIVING_HELMET),
        "cup-off-outline" => Some(icons::CUP_OFF_OUTLINE),
        "garage-alert" => Some(icons::GARAGE_ALERT),
        "phone-dial-outline" => Some(icons::PHONE_DIAL_OUTLINE),
        #[allow(deprecated)]
        "ansible" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ansible' is deprecated.").print(py);
            }
            Some(icons::ANSIBLE)
        }
        "power-plug-outline" => Some(icons::POWER_PLUG_OUTLINE),
        "sack" => Some(icons::SACK),
        "file-find" => Some(icons::FILE_FIND),
        "format-header-5" => Some(icons::FORMAT_HEADER_5),
        "battery-check" => Some(icons::BATTERY_CHECK),
        "car-brake-alert" => Some(icons::CAR_BRAKE_ALERT),
        "fridge-off-outline" => Some(icons::FRIDGE_OFF_OUTLINE),
        "chart-timeline" => Some(icons::CHART_TIMELINE),
        "filter-check-outline" => Some(icons::FILTER_CHECK_OUTLINE),
        "lightbulb-question" => Some(icons::LIGHTBULB_QUESTION),
        "home-floor-g" => Some(icons::HOME_FLOOR_G),
        _ => None,
    }
}
