// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_32(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "multiplication" => Some(icons::MULTIPLICATION),
        "fridge-off-outline" => Some(icons::FRIDGE_OFF_OUTLINE),
        "truck-fast" => Some(icons::TRUCK_FAST),
        "hockey-sticks" => Some(icons::HOCKEY_STICKS),
        "shield-outline" => Some(icons::SHIELD_OUTLINE),
        "fish" => Some(icons::FISH),
        "timer-minus-outline" => Some(icons::TIMER_MINUS_OUTLINE),
        "page-last" => Some(icons::PAGE_LAST),
        "power-plug" => Some(icons::POWER_PLUG),
        "home-search" => Some(icons::HOME_SEARCH),
        "weather-partly-cloudy" => Some(icons::WEATHER_PARTLY_CLOUDY),
        "web-minus" => Some(icons::WEB_MINUS),
        "signal-distance-variant" => Some(icons::SIGNAL_DISTANCE_VARIANT),
        "minidisc" => Some(icons::MINIDISC),
        "folder-download" => Some(icons::FOLDER_DOWNLOAD),
        "layers-outline" => Some(icons::LAYERS_OUTLINE),
        "lightbulb-multiple-off" => Some(icons::LIGHTBULB_MULTIPLE_OFF),
        "sort-numeric-ascending-variant" => Some(icons::SORT_NUMERIC_ASCENDING_VARIANT),
        "playlist-star" => Some(icons::PLAYLIST_STAR),
        "cheese" => Some(icons::CHEESE),
        "doorbell-video" => Some(icons::DOORBELL_VIDEO),
        "printer-pos-stop-outline" => Some(icons::PRINTER_POS_STOP_OUTLINE),
        "gesture-swipe-left" => Some(icons::GESTURE_SWIPE_LEFT),
        "card-account-details-outline" => Some(icons::CARD_ACCOUNT_DETAILS_OUTLINE),
        "arrow-collapse-vertical" => Some(icons::ARROW_COLLAPSE_VERTICAL),
        "calendar-start" => Some(icons::CALENDAR_START),
        "routes-clock" => Some(icons::ROUTES_CLOCK),
        "alert-circle-check-outline" => Some(icons::ALERT_CIRCLE_CHECK_OUTLINE),
        "safety-goggles" => Some(icons::SAFETY_GOGGLES),
        "image-frame" => Some(icons::IMAGE_FRAME),
        "tooltip-cellphone" => Some(icons::TOOLTIP_CELLPHONE),
        "volume-low" => Some(icons::VOLUME_LOW),
        "ornament" => Some(icons::ORNAMENT),
        "ray-start-vertex-end" => Some(icons::RAY_START_VERTEX_END),
        "fan-speed-1" => Some(icons::FAN_SPEED_1),
        "star-four-points-outline" => Some(icons::STAR_FOUR_POINTS_OUTLINE),
        #[allow(deprecated)]
        "pokemon-go" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pokemon-go' is deprecated.").print(py);
            }
            Some(icons::POKEMON_GO)
        }
        #[allow(deprecated)]
        "language-java" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-java' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_JAVA)
        }
        "database-lock-outline" => Some(icons::DATABASE_LOCK_OUTLINE),
        "division-box" => Some(icons::DIVISION_BOX),
        "infinity" => Some(icons::INFINITY),
        "message-minus-outline" => Some(icons::MESSAGE_MINUS_OUTLINE),
        "lock-question" => Some(icons::LOCK_QUESTION),
        "headset-dock" => Some(icons::HEADSET_DOCK),
        "printer-outline" => Some(icons::PRINTER_OUTLINE),
        "crop-rotate" => Some(icons::CROP_ROTATE),
        "battery-20-bluetooth" => Some(icons::BATTERY_20_BLUETOOTH),
        "wifi-strength-3-alert" => Some(icons::WIFI_STRENGTH_3_ALERT),
        "gesture-swipe-vertical" => Some(icons::GESTURE_SWIPE_VERTICAL),
        "view-split-vertical" => Some(icons::VIEW_SPLIT_VERTICAL),
        "keyboard-tab-reverse" => Some(icons::KEYBOARD_TAB_REVERSE),
        "snowshoeing" => Some(icons::SNOWSHOEING),
        "wifi-strength-outline" => Some(icons::WIFI_STRENGTH_OUTLINE),
        "sort-numeric-variant" => Some(icons::SORT_NUMERIC_VARIANT),
        "weather-partly-rainy" => Some(icons::WEATHER_PARTLY_RAINY),
        "alpha-f-box" => Some(icons::ALPHA_F_BOX),
        "hand-back-left-off-outline" => Some(icons::HAND_BACK_LEFT_OFF_OUTLINE),
        "book-sync" => Some(icons::BOOK_SYNC),
        "tag-arrow-right" => Some(icons::TAG_ARROW_RIGHT),
        "dice-d8" => Some(icons::DICE_D8),
        "chevron-triple-down" => Some(icons::CHEVRON_TRIPLE_DOWN),
        "movie-outline" => Some(icons::MOVIE_OUTLINE),
        "table-column" => Some(icons::TABLE_COLUMN),
        "notebook-plus" => Some(icons::NOTEBOOK_PLUS),
        "rabbit-variant" => Some(icons::RABBIT_VARIANT),
        "axis-z-arrow" => Some(icons::AXIS_Z_ARROW),
        "format-wrap-top-bottom" => Some(icons::FORMAT_WRAP_TOP_BOTTOM),
        "music-note-off" => Some(icons::MUSIC_NOTE_OFF),
        "ufo" => Some(icons::UFO),
        "magnify-plus-outline" => Some(icons::MAGNIFY_PLUS_OUTLINE),
        "pentagon-outline" => Some(icons::PENTAGON_OUTLINE),
        "metronome-tick" => Some(icons::METRONOME_TICK),
        "message-outline" => Some(icons::MESSAGE_OUTLINE),
        "wifi-strength-1-lock-open" => Some(icons::WIFI_STRENGTH_1_LOCK_OPEN),
        "shield-crown" => Some(icons::SHIELD_CROWN),
        "alpha-c-box-outline" => Some(icons::ALPHA_C_BOX_OUTLINE),
        "movie-check-outline" => Some(icons::MOVIE_CHECK_OUTLINE),
        "meter-electric-outline" => Some(icons::METER_ELECTRIC_OUTLINE),
        "sync-off" => Some(icons::SYNC_OFF),
        "car-door-lock" => Some(icons::CAR_DOOR_LOCK),
        "alpha-u" => Some(icons::ALPHA_U),
        #[allow(deprecated)]
        "google-glass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-glass' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_GLASS)
        }
        "alpha-q-circle" => Some(icons::ALPHA_Q_CIRCLE),
        "cash-plus" => Some(icons::CASH_PLUS),
        "wifi-strength-3-lock" => Some(icons::WIFI_STRENGTH_3_LOCK),
        "tow-truck" => Some(icons::TOW_TRUCK),
        "folder-search" => Some(icons::FOLDER_SEARCH),
        "printer-pos-star-outline" => Some(icons::PRINTER_POS_STAR_OUTLINE),
        "alpha-j-circle-outline" => Some(icons::ALPHA_J_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "origin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'origin' is deprecated.").print(py);
            }
            Some(icons::ORIGIN)
        }
        "calendar-arrow-right" => Some(icons::CALENDAR_ARROW_RIGHT),
        "baby" => Some(icons::BABY),
        "battery-off-outline" => Some(icons::BATTERY_OFF_OUTLINE),
        "video-3d" => Some(icons::VIDEO_3D),
        "orbit" => Some(icons::ORBIT),
        "eye-lock-outline" => Some(icons::EYE_LOCK_OUTLINE),
        "headphones-box" => Some(icons::HEADPHONES_BOX),
        "horse-human" => Some(icons::HORSE_HUMAN),
        "format-clear" => Some(icons::FORMAT_CLEAR),
        "chair-rolling" => Some(icons::CHAIR_ROLLING),
        "puzzle-check-outline" => Some(icons::PUZZLE_CHECK_OUTLINE),
        "credit-card-refund" => Some(icons::CREDIT_CARD_REFUND),
        "bacteria" => Some(icons::BACTERIA),
        "phone-minus" => Some(icons::PHONE_MINUS),
        "border-outside" => Some(icons::BORDER_OUTSIDE),
        "laptop-off" => Some(icons::LAPTOP_OFF),
        "size-m" => Some(icons::SIZE_M),
        "virus-off-outline" => Some(icons::VIRUS_OFF_OUTLINE),
        "snowboard" => Some(icons::SNOWBOARD),
        "countertop-outline" => Some(icons::COUNTERTOP_OUTLINE),
        "printer-pos-alert" => Some(icons::PRINTER_POS_ALERT),
        "emoticon-devil-outline" => Some(icons::EMOTICON_DEVIL_OUTLINE),
        "glass-cocktail" => Some(icons::GLASS_COCKTAIL),
        "video-high-definition" => Some(icons::VIDEO_HIGH_DEFINITION),
        "pan-up" => Some(icons::PAN_UP),
        "account-cog-outline" => Some(icons::ACCOUNT_COG_OUTLINE),
        "train-car-container" => Some(icons::TRAIN_CAR_CONTAINER),
        "pan-bottom-right" => Some(icons::PAN_BOTTOM_RIGHT),
        "window-close" => Some(icons::WINDOW_CLOSE),
        "sickle" => Some(icons::SICKLE),
        "vector-polygon" => Some(icons::VECTOR_POLYGON),
        "table-headers-eye" => Some(icons::TABLE_HEADERS_EYE),
        "sheep" => Some(icons::SHEEP),
        "phone-log" => Some(icons::PHONE_LOG),
        "pool" => Some(icons::POOL),
        "truck-remove-outline" => Some(icons::TRUCK_REMOVE_OUTLINE),
        "progress-close" => Some(icons::PROGRESS_CLOSE),
        "image-edit-outline" => Some(icons::IMAGE_EDIT_OUTLINE),
        "flask-round-bottom-outline" => Some(icons::FLASK_ROUND_BOTTOM_OUTLINE),
        "flask-remove-outline" => Some(icons::FLASK_REMOVE_OUTLINE),
        "moon-new" => Some(icons::MOON_NEW),
        "format-list-text" => Some(icons::FORMAT_LIST_TEXT),
        "gender-male-female" => Some(icons::GENDER_MALE_FEMALE),
        "panorama-horizontal-outline" => Some(icons::PANORAMA_HORIZONTAL_OUTLINE),
        #[allow(deprecated)]
        "language-ruby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUBY)
        }
        "checkbox-outline" => Some(icons::CHECKBOX_OUTLINE),
        "clock-time-six-outline" => Some(icons::CLOCK_TIME_SIX_OUTLINE),
        "fireplace" => Some(icons::FIREPLACE),
        "shaker-outline" => Some(icons::SHAKER_OUTLINE),
        "checkbox-intermediate-variant" => Some(icons::CHECKBOX_INTERMEDIATE_VARIANT),
        "comment-bookmark" => Some(icons::COMMENT_BOOKMARK),
        "eight-track" => Some(icons::EIGHT_TRACK),
        #[allow(deprecated)]
        "gitlab" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gitlab' is deprecated.").print(py);
            }
            Some(icons::GITLAB)
        }
        "compass-rose" => Some(icons::COMPASS_ROSE),
        "arrow-right-bold-hexagon-outline" => Some(icons::ARROW_RIGHT_BOLD_HEXAGON_OUTLINE),
        "heart-off" => Some(icons::HEART_OFF),
        "phone-hangup" => Some(icons::PHONE_HANGUP),
        "emoticon-frown-outline" => Some(icons::EMOTICON_FROWN_OUTLINE),
        "projector" => Some(icons::PROJECTOR),
        "dice-2" => Some(icons::DICE_2),
        "home-remove-outline" => Some(icons::HOME_REMOVE_OUTLINE),
        "bus-stop" => Some(icons::BUS_STOP),
        "lock-clock" => Some(icons::LOCK_CLOCK),
        "format-line-weight" => Some(icons::FORMAT_LINE_WEIGHT),
        "phone-refresh" => Some(icons::PHONE_REFRESH),
        "axis-y-arrow-lock" => Some(icons::AXIS_Y_ARROW_LOCK),
        "briefcase-plus" => Some(icons::BRIEFCASE_PLUS),
        "flower-poppy" => Some(icons::FLOWER_POPPY),
        "pencil-lock" => Some(icons::PENCIL_LOCK),
        "bluetooth-audio" => Some(icons::BLUETOOTH_AUDIO),
        "flag-variant-remove-outline" => Some(icons::FLAG_VARIANT_REMOVE_OUTLINE),
        "hand-wave" => Some(icons::HAND_WAVE),
        "information-variant-circle" => Some(icons::INFORMATION_VARIANT_CIRCLE),
        "map-marker-alert-outline" => Some(icons::MAP_MARKER_ALERT_OUTLINE),
        "fountain" => Some(icons::FOUNTAIN),
        "bell-badge-outline" => Some(icons::BELL_BADGE_OUTLINE),
        "page-previous-outline" => Some(icons::PAGE_PREVIOUS_OUTLINE),
        "stadium-outline" => Some(icons::STADIUM_OUTLINE),
        "fence" => Some(icons::FENCE),
        "close-thick" => Some(icons::CLOSE_THICK),
        "car-cruise-control" => Some(icons::CAR_CRUISE_CONTROL),
        "content-save" => Some(icons::CONTENT_SAVE),
        "rotate-left-variant" => Some(icons::ROTATE_LEFT_VARIANT),
        "ice-cream-off" => Some(icons::ICE_CREAM_OFF),
        "home-off-outline" => Some(icons::HOME_OFF_OUTLINE),
        "human-male-male-child" => Some(icons::HUMAN_MALE_MALE_CHILD),
        "home-thermometer-outline" => Some(icons::HOME_THERMOMETER_OUTLINE),
        "history" => Some(icons::HISTORY),
        "gender-male-female-variant" => Some(icons::GENDER_MALE_FEMALE_VARIANT),
        "kettle-off-outline" => Some(icons::KETTLE_OFF_OUTLINE),
        "keyboard-tab" => Some(icons::KEYBOARD_TAB),
        "food-off" => Some(icons::FOOD_OFF),
        "bag-personal-plus-outline" => Some(icons::BAG_PERSONAL_PLUS_OUTLINE),
        "compass-off" => Some(icons::COMPASS_OFF),
        "filter-remove-outline" => Some(icons::FILTER_REMOVE_OUTLINE),
        "arrow-top-left-thick" => Some(icons::ARROW_TOP_LEFT_THICK),
        "view-day-outline" => Some(icons::VIEW_DAY_OUTLINE),
        "send" => Some(icons::SEND),
        "cloud-key" => Some(icons::CLOUD_KEY),
        "phone-classic-off" => Some(icons::PHONE_CLASSIC_OFF),
        "vhs" => Some(icons::VHS),
        "nfc-variant-off" => Some(icons::NFC_VARIANT_OFF),
        "roller-skate-off" => Some(icons::ROLLER_SKATE_OFF),
        "comment-processing-outline" => Some(icons::COMMENT_PROCESSING_OUTLINE),
        "baby-face" => Some(icons::BABY_FACE),
        "map-marker-minus" => Some(icons::MAP_MARKER_MINUS),
        "book-search" => Some(icons::BOOK_SEARCH),
        "music-box-outline" => Some(icons::MUSIC_BOX_OUTLINE),
        "weather-cloudy-alert" => Some(icons::WEATHER_CLOUDY_ALERT),
        "surround-sound-7-1" => Some(icons::SURROUND_SOUND_7_1),
        _ => None,
    }
}
